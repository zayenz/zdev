# Shadow record mode

Status: implemented (2026-11-12). The user reference is `docs/cli/shadow.md`;
see "Implementation notes" below for where the implementation differs from
this design.

## Goal

Use zdev with Git history for its records in a repository where `.zdev` must
not enter the main repository. A fourth record policy, `shadow`, keeps `.zdev`
out of the main repository and versions it in a separate *shadow repository*.
That repository is local-only or has a real remote.

## Agreed decisions

| Fork | Decision |
| --- | --- |
| Layout | `.zdev/` is its own nested Git repository (`.zdev/.git`). One shadow repository per project. |
| Commits | Automatic and linked: zdev commits the shadow repo at the same points where a `project` record would be committed, with cross-references. |
| Push | Opt-in auto-push. Default off. |
| Branches | Mirror: the shadow repo's checked-out branch follows the main repo's branch. zdev manages shadow branches whenever zdev itself drives Git; otherwise it follows on a best-effort basis. |
| Creation | Lazy: a shadow branch appears when the main repo switches to its branch, not at `area create`. |
| Conversion | `zdev shadow adopt` (personal → shadow) is in v1. |

## Model

Toward the main repository, `shadow` behaves exactly like `personal`: `.zdev`
is untracked and excluded, trunk areas are allowed, and linked worktrees lack
`.zdev`. Workers already read the authoritative checkout's record paths, so
no change is needed there. Every main-repo check that special-cases `personal`
must treat `shadow` the same way.

All Git operations on records go through a record-repository abstraction:

- `project` / `pull-request`: Git dir = main repo, record paths `.zdev/<p>`.
- `shadow`: Git dir = `.zdev`, record paths `<p>` (no `.zdev/` prefix).
- `personal`: no record repository.

Shadow Git commands run as `git -C <root>/.zdev`, with `GIT_DIR` and
`GIT_WORK_TREE` removed, as the existing helpers already do.

`list_areas` already skips entries without `area.toml`, so `.zdev/.git` is
harmless. Research boundaries already exclude `.zdev`.

## Setup

- `zdev init --record shadow`
  - Refuse if any `.zdev` path is tracked in the main repo.
  - `git init` in `.zdev`, using the main repo's current branch name.
  - Write `config.toml` with `record = "shadow"` and make the initial shadow
    commit.
  - Append `/.zdev/` to the main repo's `.git/info/exclude`. Unlike
    `personal`, do this automatically, since an unexcluded nested repo would
    show up as an embedded repository.
- `zdev init --record shadow --shadow-remote <url>`
  - If the remote already holds a shadow record (`config.toml` with
    `record = "shadow"`), clone it into `.zdev` and check out the branch that
    mirrors the main repo's current branch. Use this for a fresh clone of the
    main repo.
  - Otherwise initialize as above and add the URL as `origin`.
- `zdev shadow adopt [--shadow-remote <url>]` converts an existing
  `personal` record in place: refuse if `.zdev` is tracked in the main repo
  or `.zdev/.git` already exists. Then `git init`, set `record = "shadow"`,
  ensure the exclude rule, and commit the existing records as the first shadow
  commit on the branch matching the main repo's branch. Converting a `project`
  record is out of scope for v1.

## Branch mirroring

The shadow branch name equals the main repo's branch name. Alignment happens
under `ZdevStateLock` in every command that reads or writes records:

1. Read the main HEAD branch `B` from the authoritative checkout.
2. If the shadow HEAD is already `B`, do nothing.
3. Otherwise run `git -C .zdev switch B`, or `switch -c B` when the branch is
   missing. Git semantics then apply directly. A new branch forks from the
   current shadow HEAD, and uncommitted record changes carry over. Area
   creation therefore works the same way as with `project` records: create the
   area on trunk, then switch the main repo to the area branch. When the
   switch would conflict, fail and name both branches.
4. On a detached main HEAD, leave the shadow repo where it is. Reads are
   allowed. Automatic shadow commits are skipped with a warning.

### When zdev drives Git (managed)

Each Git operation zdev performs on the main repo gets a matching shadow
operation in the same command, under the same lock:

- `zdev commit`: linked shadow commit (see below).
- `zdev area rebase <tag>`: after the main rebase succeeds, rebase the area's
  shadow branch onto the parent's shadow branch. Its `--continue` and
  `--abort` paths do the same on the shadow side. A shadow conflict leaves the
  shadow rebase in progress and reports both repositories.
- `zdev area bind <tag> <branch>`: no shadow operation. Bind only records a
  branch name; the shadow branch appears when the main repository switches to
  it.
- Any future zdev command that switches, merges, or deletes main branches
  must do the same to the shadow repo. Treat this as a rule in the
  record-repository abstraction, not a per-command afterthought.

### When the user drives Git (best effort)

zdev cannot see plain `git switch`, `git merge`, or `git branch -d`. At the
next zdev command, alignment repairs what it safely can:

- Switches: follow them, as described above.
- Merges: if the shadow branch of a main branch contained in the current main
  branch is not yet contained in the current shadow branch, and
  `merge --ff-only` (or a clean rebase followed by a fast-forward) succeeds,
  integrate it automatically and say so in the output. Otherwise leave the
  shadow repo unchanged and report the branch as awaiting integration.
  `zdev shadow integrate <branch>` does the same thing on request. Never
  create merge commits.
- Deleted main branches: report the orphaned shadow branch. Never delete it
  automatically.
- `zdev status` and `zdev check` list misalignment, branches awaiting
  integration, and orphans.

## Commit coupling

- `zdev commit`: commit the main index unchanged. Then, if the shadow repo has
  changes, run `add -A` and commit them in the shadow repo with:
  - the same subject,
  - `Zdev-Change-Id: <same id>`,
  - `Zdev-Source-Commit: <main sha>`,
  - `Zdev-Source-Branch: <B>`.
  The JSON output gains `"shadow": {"commit", "branch", "pushed"}`. If the
  shadow commit fails after the main commit succeeded, report the partial
  result plus the recovery command `zdev shadow commit`. Do not undo the main
  commit.
- `zdev tasks import --commit`: commit only in the shadow repo. The
  `require_committable_*` checks move onto the record-repository abstraction
  (`ls-files`, `diff`, and `diff --cached` against the shadow repo with
  unprefixed paths).
- `zdev shadow commit -m <subject>`: manual snapshot, for example after area
  creation or notes. It carries a change ID and no source trailers.
- `zdev change inspect` also searches the shadow log, so a change ID resolves
  to both commits.
- `zdev cleanup squash` is not applicable. Refuse it, as for `personal`.

Harness contract: for `shadow`, stage like `personal` (never stage `.zdev` in
the main repo); `zdev commit` handles the shadow side. Templates, skills, and
docs need a `shadow` row wherever `personal | project | pull-request` appear.

## Push

- Setting `shadow.push = "never" | "after-commit"` (default `never`) lives in
  layered configuration. It is a per-clone preference, so it can be set
  locally without committing it to the shared record.
- With `after-commit`, every automatic or manual shadow commit pushes the
  current shadow branch to its upstream (`push -u origin <B>` the first time).
- A push failure is a warning, never an error. The commit stands, and status
  shows how far the branch is ahead of its upstream.
- `zdev shadow push` exists for explicit pushes regardless of the setting.

## Status and check

`zdev status` and `check` show:

- the shadow branch versus the main branch (aligned or not),
- uncommitted shadow changes,
- ahead/behind against the upstream,
- branches awaiting integration and orphaned branches,
- warnings when `.zdev` is not excluded or is tracked in the main repo.

## Likely touch points

- `src/project.rs`: `RecordPolicy::Shadow`, init, the trunk-area and
  pull-request guards (treat `Shadow` like `Personal`).
- `src/tasks.rs`: task-import commit checks and commit, through the
  record-repository abstraction.
- `src/lib.rs`: `commit`, change lookup (`git log --all`), alignment hook after
  root discovery, the new `shadow` subcommand.
- `src/config.rs`: the `shadow.push` key and the read-only `project.record`
  message.
- `templates/zdev/*` and the generated skill copies, docs (`docs/cli/init.md`,
  `user-guide.md`, new `docs/cli/shadow.md`), CHANGELOG.
- `tests/lean.rs`: init (local and remote with a bare repo fixture), linked
  commit, alignment and conflict refusal, detached HEAD, integrate, push
  opt-in and push failure.

## Resolved questions

1. Integration: managed when zdev drives Git; best-effort automatic
   fast-forward when the user drives it, otherwise reported.
2. Branch creation follows switches lazily.
3. `shadow adopt` is in v1.

## Implementation notes

- `zdev commit` commits only the staged shadow index when it has staged
  changes; otherwise it stages every record change with `add -A`. With nothing
  staged in the main repository and pending record changes, it makes a
  records-only shadow commit.
- `zdev area rebase` rebases the shadow branch best effort after the main
  rebase completes. A dirty shadow record or a shadow conflict leaves the
  shadow branch unchanged (the shadow rebase is aborted) and prints the manual
  command; the main rebase stands.
- Explicit `zdev shadow integrate` never rewrites trunk: on trunk it rebases
  the other branch onto trunk, elsewhere it rebases the current branch.
- Init and clone copy the main repository's local `user.name` and
  `user.email` into the shadow repository. A failed init removes the `.zdev`
  it created.
- Merge detection: a merged main branch must have moved on since its shadow
  branch began, shown by a `Zdev-Source-Commit` trailer or by the main tip
  differing from the fork point zdev records in the shadow repository's
  `branch.<name>.zdevForkPoint` when it creates the branch. The fork point is
  the main HEAD at the first zdev command after the switch, so commits made
  before that command and merged without later commits are not detected.
- When the main commit of a two-repository record commit fails, the shadow
  commit is removed with a soft reset and only its paths are unstaged. A
  failed source-trailer amendment leaves both commits and exits with status 1.
- `shadow commit` and `shadow push` run despite a failed alignment, acting on
  the current shadow branch, so they can clear a blocked switch.
- Shadow Git commands drop `GIT_INDEX_FILE`; main-repository commands keep it.
- Alignment recreates the empty `tasks` directory of areas without tasks,
  since Git does not track empty directories. Project records cloned fresh
  have the same gap, which is not addressed here.
- Status, check, change, and shadow status downgrade an alignment failure to a
  warning; every other command stops on it.

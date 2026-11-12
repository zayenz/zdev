# Pull-request areas on the current branch: field feedback

1 November 2026. Claude Code (Opus 5.5) inside T3 Code, macOS, zdev 2.1.0.
Gecode repository, on `feature/breaking-7`. That branch is itself a long-lived
branch that will become one PR to `main`.

The user asked to "organise the work using pull-request scoped zdev". The
coordinator created six new `breaking-7/<area>` branches, one area on each.
The user wanted all areas on the checked-out branch and had not asked for any
branches. The areas were moved back by hand. The guidance made the wrong
reading easy, and three CLI gaps made the repair slow.

## 1. The guidance reads as "create a branch per area"

- `references/shape-work.md` step 1 says: "Pull-request records need an area
  branch for review under either setting. With pull-request records, or under
  `isolated`, select the area's branch with the shared workspace-choice rules,
  check it out, and create the isolated area there." Read alone, "need an area
  branch" and "check it out" sound like an instruction to make a branch.
- `references/setup.md` says "Pull-request records use isolated areas", and
  its `general` example passes `--branch general`. That reinforces the same
  reading.
- The root skill's workspace rules ("create a branch or worktree only when the
  user asks for one") say the opposite, but they sit far from the step that
  runs.

Proposed wording for shape-work step 1:

> Pull-request records bind each area to a non-trunk branch. Use the
> checked-out branch unless the user names a different one. Never create a
> branch for an area unless the user explicitly asks for it.

Add the same sentence next to "Pull-request records use isolated areas" in
setup.md.

## 2. `zdev init` on a PR branch records the PR branch as trunk

`zdev init --record pull-request` on `feature/breaking-7` recorded
`feature/breaking-7` as trunk. Under pull-request records an area cannot be
isolated on trunk, so the only way forward seemed to be new branches. That was
the root of the wrong move above. The right setup was trunk = `main` (the PR's
base) with every area bound to `feature/breaking-7`.

Ideas:

- When `--record pull-request` runs on a branch other than the repository's
  default branch, have `init` ask for or suggest `--trunk <default>` (or
  `zdev config trunk main`). It could also say that areas will bind to the
  checked-out branch.
- Setup guidance: "With pull-request records, trunk is the branch the PR
  merges into, not the branch under review."

## 3. Finalizing base anchors after `bind` needs a rebase-and-amend loop

After `zdev config trunk main` and `zdev area bind <area> feature/breaking-7`,
every area reported `base-anchor-needs-finalization`.
`task_work.safe` was false. The branch was already fresh on `main`.

- The fix is `zdev area rebase <area>`, a no-op rebase that rewrites
  `area.toml`. That leaves the tree dirty, so the next area's rebase fails:
  "Cannot rebase with local changes". Six areas needed rebase, `git add -f`,
  and `git commit --amend`, one at a time.
- The diagnostic does not name the command that clears it. I found it in
  `references/recovery.md`.

Ideas: have `bind` set the anchor directly when the branch already contains
its base. Add `zdev area rebase --all` for the areas on one branch, or allow
the finalization when only `.zdev` records are dirty. Put the fix command in
the diagnostic text.

## 4. An ignored `.zdev` is not reported

The repository's shared `.git/info/exclude` contains `/.zdev/`, left by
personal records in another worktree. `zdev init --record pull-request`
succeeded and `git status` showed nothing, so the records looked absent. Other
pull-request worktrees of the same repo had force-added their `.zdev`.

Idea: `init`, `check`, and `status` warn when `.zdev` is ignored under project
or pull-request records. Name the ignoring file and line (from
`git check-ignore -v`), and say that the records need `git add -f`, or have
`zdev` stage them with `-f` when it commits records.

## 5. No command to retitle an area

When the user narrowed an area's scope, the title and objective in
`area.toml` had to be edited by hand. `zdev tasks reindex` does not exist
(it is `zdev tasks index`). Idea: add
`zdev area edit <area> --title ... --objective ...`, which validates the
values and regenerates `TASKS.md`.

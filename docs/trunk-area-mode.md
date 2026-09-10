# Trunk-based area work

Use trunk mode to work on several areas on the configured project trunk.
Areas use isolated branches by default. This reference covers trunk records,
branch checks, and transitions between the two modes.

## Durable representation

`area.toml` accepts an optional mode:

```toml
mode = "isolated" # or "trunk"
```

The parser continues to deny unknown fields. The mode-dependent schema is:

| Mode | Stored `branch` | `parent` | `base_commit` |
| --- | --- | --- | --- |
| field absent, or `mode = "isolated"` | required | optional | optional |
| `mode = "trunk"` | forbidden | forbidden | forbidden |

The absent value means `isolated`. Writers omit `mode` for isolated areas, so
default records keep the legacy format. Writers emit
`mode = "trunk"` for trunk areas. A trunk area's operating branch is always the
current `project.trunk`; it is resolved, not copied into the area record.

`schema_version` remains `1`. The change adds a tagged variant while preserving
the full meaning of every existing v1 area record; it needs no record rewrite
or migration. An old binary has a strict reader and therefore rejects the new
`mode` field; it cannot silently manage a trunk area as an isolated one. The
compatibility direction is deliberate: the new binary accepts all old records,
while old binaries fail closed on the new record.

Examples:

```toml
# Existing and new default form: isolated
schema_version = 1
tag = "payments"
title = "Payments"
objective = "Improve payment handling."
branch = "payments"
base_commit = "0123456789abcdef0123456789abcdef01234567"
```

```toml
# Explicit trunk form
schema_version = 1
tag = "docs"
title = "Documentation"
objective = "Keep the documentation accurate."
mode = "trunk"
```

`zdev check`, every area loader, and every write path enforce the conditional
schema. A trunk record containing `branch`, `parent`, or `base_commit`, or an
isolated record without `branch`, is invalid. No loader repairs it.

### Legacy records

Every record without `mode` remains isolated. This includes an area whose
stored branch happens to equal the current trunk and an area whose branch was a
former trunk. It keeps the existing single-owner rule, same-branch shortcut,
anchor behavior, and stored branch. Reconfiguring trunk never adds `mode`,
removes its branch, or moves it. Once the configured trunk changes, a legacy
area on the former trunk is assessed as an ordinary isolated area against the
new base and may be fresh, stale-but-safe, or unsafe from its existing facts.

## Commands and transitions

The grammar is:

```text
zdev area create <tag> --title <title> --objective <objective> [--branch <branch> | --trunk]
zdev area bind <area> [<branch> | --trunk]
```

`--trunk` is a flag. It conflicts with `--branch` on create and with the
positional branch on bind. A repeated flag or extra positional value is a CLI
error.

Create without either option uses the default: create an isolated area on
the checked-out branch. `--branch` creates an isolated area on the canonical
requested branch. `--trunk` creates a trunk area following the configured
trunk. It requires a configured, locally existing trunk, a record policy that
supports trunk areas, no active Git operation, and no isolated owner of that
branch. It does not create or check out the branch. Like explicit
`--branch`, it can record the explicit target while HEAD is detached or on
another branch; its task-work status is then unsafe until trunk is checked out.
An inferred isolated branch fails on detached HEAD with the existing
`No branch is checked out; pass an explicit branch name` error.

Bind without a target uses the default: bind as isolated to the checked-out
branch. A positional branch binds as isolated to that canonical branch.
`--trunk` changes the area to trunk mode. All bind forms take the state lock,
reread config and area records under the lock, validate the complete candidate
area graph, and atomically replace only that area's `area.toml`.

Mode changes have these additional checks:

- Isolated to trunk requires configured and locally existing trunk, no active
  Git operation, no parent, a supported record policy, and no isolated claimant
  of trunk. The old isolated branch must exist and its tip must be an ancestor
  of the trunk tip. This proves that the mode change does not abandon
  unintegrated area commits. The successful write removes `branch` and
  `base_commit` by publishing the trunk form.
- Trunk to isolated requires the named or inferred target branch to exist, no
  active Git operation, and its tip to contain the current trunk tip. The
  successful write removes `mode`, stores the target branch, leaves `parent`
  absent, and records the current trunk tip as `base_commit`.
- Isolated to isolated retains the current anchor-containment and anchor
  computation behavior. It does not gain a whole-worktree cleanliness check.
- Repeating the current binding is a successful `unchanged` result. A failure
  preserves the prior bytes and mode.

An explicit branch makes isolated create/bind usable on detached HEAD. An
omitted branch still fails there. `--trunk` does not infer from HEAD, so
detached HEAD is not a parse or configuration error; status reports it and
task work remains blocked. No transition switches, creates, renames, deletes,
merges, rebases, or pushes a branch.

Stable mode-transition failures are:

| Condition | Error |
| --- | --- |
| Trunk is unconfigured | ``Project trunk is not configured; set it with `zdev config trunk <branch>` `` |
| Configured trunk is missing | `Cannot use trunk mode because configured trunk <branch> is missing locally` |
| Inferred branch on detached HEAD | existing `No branch is checked out; pass an explicit branch name` |
| Isolated owner conflicts | `Branch <branch> is already owned by isolated area <tag>` |
| Isolated source has commits outside trunk | `Cannot bind area <tag> to trunk: branch <branch> has commits not contained in <trunk>` |
| Isolated target does not contain trunk | `Cannot bind trunk area <tag> to <branch>: the branch does not contain configured trunk <trunk>` |
| Parent remains | `Cannot bind area <tag> to trunk while it has parent <parent>; remove the parent first` |
| Pull-request record | `Trunk areas are not supported with pull-request records; use an isolated area branch` |

CLI conflict errors remain Clap errors. Active-operation errors retain the
existing `while a <operation> is in progress` form and preserve Git state.

Human results are:

```text
Created trunk area docs on configured trunk main
Bound area docs to configured trunk main
Bound area docs to branch docs-work
Area docs is already bound to configured trunk main
```

The corresponding JSON objects contain exactly the existing
`schema_version`, `status`, `area`, and `path` where create already returns it,
plus these stable fields:

```json
{"mode":"trunk","branch":"main","base_commit":null}
```

or:

```json
{"mode":"isolated","branch":"docs-work","base_commit":"<40-hex-commit>"}
```

`branch` is the resolved configured branch in a trunk result. `base_commit` is
always null there. Isolated create keeps status `created`; a changed bind keeps
status `updated`; an exact repeat uses `unchanged`.

## Ownership and several trunk areas

Several explicit trunk areas may coexist. Branch ownership validation resolves
the candidate configured trunk and applies one rule:

- any number of `mode = "trunk"` areas may share that resolved branch;
- an isolated area remains the sole owner of its stored branch; and
- a branch may not be shared between an isolated area and any other area,
  including a trunk area.

Thus a legacy isolated area stored on `main` blocks the first explicit trunk
area until the user binds it elsewhere or deliberately converts it. Zdev never
reclassifies the record because its branch name matches configuration.

On checked-out trunk, all structurally safe trunk areas have
`branch_matches = true`. `next --any` retains its current comparison: matching
candidates first, then lexical area tag; each area's existing numeric task
order chooses its candidate. With ready `docs-002` and `quality-001` on trunk,
`docs-002` wins. Mixed-mode candidates on the checked-out isolated branch win
before off-branch trunk candidates. No area is hidden and no branch is switched.

## Reconfiguring project trunk

Both trunk-setting forms call one resolver and writer:

```text
zdev config trunk [<branch>] [--allow-divergent]
zdev config set [--local] [--allow-divergent] project.trunk <branch>
```

Omission in the convenience form still means the checked-out branch.
`--allow-divergent` is valid only for these two local trunk writes and only when
explicit trunk areas exist. The generic form rejects it with any other key;
both forms reject it with `--global`.

When at least one explicit trunk area exists, the command:

1. acquires the zdev state lock and rereads config and all areas;
2. rejects an active Git operation;
3. canonicalizes the requested branch and requires it to exist locally and be
   inspectable;
4. validates the complete area graph against the candidate branch, including
   isolated/trunk ownership collisions and record policy; and
5. when changing an existing configured trunk, resolves both tips and requires
   the old tip to be an ancestor of the candidate tip; and
6. atomically replaces only `.zdev/config.toml`.

The ancestry rule accepts the same tip, a second branch name at that tip, and
a fast-forward descendant. If the old configured branch is missing or its tip
or ancestry cannot be inspected, reconfiguration fails even with the override.
A candidate on divergent or older history fails by default. Both failures name
every affected trunk area in lexical order and leave config unchanged:

```text
Cannot reconfigure trunk from main to stable for trunk areas docs, quality: main is not an ancestor of stable. Re-run with --allow-divergent only after deciding to move these areas without ancestry continuity
```

```text
Cannot reconfigure trunk from main to stable for trunk areas docs, quality: previous trunk main is missing or cannot be inspected. Restore it before reconfiguring trunk
```

An actual external rename normally removes the old ref, so the second failure
applies even when the user knows both names refer to the same former tip. A
same-tip name change is automatic only while both refs remain inspectable.

`--allow-divergent` overrides only a confirmed ancestry failure. The old and
candidate tips must both remain inspectable. Candidate existence, ownership,
record policy, active-operation, schema, and locking checks still apply. It
performs no Git operation. Success reports the decision:

```text
Configured project trunk stable (previous: main; affected trunk areas: docs, quality)
Ancestry override: previous trunk main is not contained in stable
```

JSON is exact apart from the commit values:

```json
{
  "schema_version": 1,
  "status": "updated",
  "previous_trunk": "main",
  "trunk": "stable",
  "affected_areas": ["docs", "quality"],
  "ancestry": {
    "old_tip": "<40-hex-commit-or-null>",
    "new_tip": "<40-hex-commit>",
    "old_is_ancestor": false,
    "override": true
  }
}
```

Successful reconfiguration always reports resolved old and new tips. A normal
contained move returns the same object with `override = false` and
`old_is_ancestor = true`, and human output omits the ancestry-override line. An
exact repeat returns `status = "unchanged"`, identical old/new branch and tip,
`old_is_ancestor = true`, and performs no write.

Because trunk areas store no branch copy, the config write moves every open and
closed trunk area's resolved operating branch together. A filesystem or
validation failure leaves the old config bytes and every area record intact;
recovery is to fix the reported condition and retry. There is no multi-file
partial state to roll back.

Reconfiguration does not require the new trunk to be checked out. If the old
trunk is checked out, open trunk areas immediately report
`branch_matches = false` and `task_work.safe = false`; named task work cannot
start until the user checks out the new trunk. `next --any` may still project a
structurally safe off-branch candidate under its existing discovery contract,
but reports the required new branch and `branch_matches = false`. There is no
stored task selection to migrate. A harness-native prompt or goal is live
harness state, not zdev state, and must be re-oriented against fresh status.
The command never switches branches.

`zdev config unset project.trunk` is rejected while a trunk area exists:

```text
Cannot unset project.trunk while trunk areas exist: docs, quality
```

The tags are lexical. Reconfigure those areas to isolated first. With no trunk
areas, set/unset and the convenience command retain their existing behavior,
including the ability to name a branch not present locally. Existing isolated
projects therefore keep their configuration behavior.

If the configured branch is later renamed or deleted directly through Git,
trunk records remain unchanged. Status becomes unsafe and names the missing
configured branch. The user restores that branch or atomically reconfigures
`project.trunk` to an existing branch. Zdev does not discover a rename or alter
Git refs.

## Status and task-work projection

Area status adds `mode` to both the `area` and `branch_status` objects. For an
isolated area, `area.branch` remains the stored branch. For a trunk area,
`area.branch` is the resolved `project.trunk`, or null when unconfigured; this
is a view and is not written to `area.toml`. Optional `parent` remains omitted
and `base_commit` is projected as null.

A healthy trunk area on checked-out `main` has exactly these mode-sensitive
fields:

```json
{
  "mode": "trunk",
  "branch": "main",
  "checked_out_branch": "main",
  "branch_matches": true,
  "parent_area": null,
  "base_commit": null,
  "effective_base": {"kind": "trunk", "area": null, "branch": "main"},
  "fresh": true,
  "anchor_valid": null,
  "finalized": null,
  "linear_history": null,
  "task_work": {
    "safe": true,
    "structurally_safe": true,
    "stale_advisory": false,
    "git_operation": null
  },
  "diagnostics": ["fresh"]
}
```

`effective_base.kind` remains `trunk` for output compatibility, but in trunk
mode its branch is also the operating branch. `fresh = true` means the resolved
branch is necessarily current with itself. Anchor validity, finalization, and
child linearity are null because trunk mode has no child/base boundary. They
are not silently treated as passed checks. `stale_advisory` is always false.

The remaining cases are exact:

| Condition | `branch_matches` | `fresh` | structural / task safe | Added diagnostics |
| --- | --- | --- | --- | --- |
| Another attached branch is checked out | false | true | true / false | `wrong-branch` |
| Detached HEAD | null | true | true / false | `detached-head` |
| Configured trunk absent | null | null | false / false | `project-trunk-unbound` |
| Configured branch missing locally | comparison with attached HEAD, normally false | null | false / false | `trunk-branch-missing` |
| Active named Git operation | normal comparison | true when branch exists | false / false | `git-operation-in-progress`; `task_work.git_operation` is its name |
| Git operation cannot be inspected | normal comparison | true when branch exists | false / false | `git-state-unavailable` |

Diagnostic order is Git inspection/operation, checkout mismatch, configuration
or missing-branch error, then `fresh` when applicable. Status is read-only and
succeeds for these unsafe states. Task-work commands return the same
`branch_status` in structured error details.

The human branch line is:

```text
docs: trunk mode on main [fresh]
```

and, for example:

```text
docs: trunk mode on main [wrong-branch, fresh]
```

The existing title, lifecycle, queue, counts, slices, advisory placement, and
project-wide summary stay unchanged. Trunk mode never prints a rebase advisory.
Selected `next` JSON retains its existing task fields and includes the resolved
`branch`, `mode = "trunk"`, `branch_matches`, and complete `branch_status`.
Off-branch human output keeps `Required branch: main (not checked out; current
branch: <branch>)`.

`zdev goal <area>` remains branch-independent. Its human and JSON output stay
the deterministic one-task projection of area lifecycle, queue, and task
context; neither form gains `mode`, `branch`, or `branch_status`. Harness
workflows obtain branch facts from the nested status projection in fresh
`zdev work-context <area> --format json` output.

## Base, parents, and rebase

`base_commit` has no meaning in trunk mode. Create does not compute it,
isolated-to-trunk bind removes it, status returns null, and all trunk writers
reject a stored value. Trunk-to-isolated bind creates a new boundary at the
current configured trunk tip as described above.

The parent matrix is deliberately small:

| Child | Parent | Result |
| --- | --- | --- |
| isolated | isolated | Existing behavior. |
| isolated | trunk | Reject: bind the child directly to configured trunk instead of naming one of several equivalent trunk areas. |
| trunk | isolated | Reject: trunk mode cannot have a parent. |
| trunk | trunk | Reject: trunk mode cannot have a parent. |

`zdev area parent` reports the mode conflict before writing. An isolated area
must remove its parent before `area bind --trunk`.

For `zdev area rebase <trunk-area>`, zdev first rejects an active Git operation,
an unconfigured or missing trunk, detached HEAD, or a wrong checked-out branch.
On healthy checked-out trunk it returns without requiring a clean worktree,
running Git, or writing metadata:

```text
Area docs runs on configured trunk main; no rebase is needed
```

```json
{"schema_version":1,"status":"unchanged","area":"docs","mode":"trunk","branch":"main","effective_base":"main","base_commit":null,"fresh":true}
```

`--continue` and `--abort` reject trunk mode because zdev never starts a
managed rebase for it. They do not touch an unrelated Git recovery operation.
The error tells the user to inspect and finish or abort that operation through
its owning workflow. Isolated rebase behavior is unchanged.

## Task records and Git attribution

Task and area lifecycle formats, task order, slices, and bundle approvals do
not change. The mode affects only the branch facts used by existing gates:

| Operation in trunk mode | Rule |
| --- | --- |
| Ordinary import | Unchanged: valid open area, approved bundle when supplied, slices and whole graph, state lock, transactional task/index publication. It remains possible off-branch and does not commit. |
| Committed import | Requires configured trunk checked out, local and inspectable, with no active Git operation. The existing owning-area preflight and rollback apply. A tracked valid worktree-modified owning `brief.md` may join the task files and `TASKS.md`; staged, deleted, untracked, symlinked, conflicted, or partially staged brief state is rejected. |
| Named selection and completion | Require trunk `task_work.safe`: configured trunk exists, is checked out, Git state is inspectable, and no operation is active. Completion still atomically writes only the task and `TASKS.md`. |
| Task reopen | Keeps the branch-independent rule and atomic task/index write. A closed area and completed dependents still block it. |
| Area close/reopen | Require `task_work.safe`, evaluated from trunk mode. |
| Final commit | Unchanged `zdev commit`: commit exactly the caller-prepared index and add the stable change ID. Mode does not make every change on trunk belong to the selected area. |

Non-overlapping unrelated staged, unstaged, and untracked trunk changes remain
in place. They do not become owned merely because several areas use trunk. The
coordinator captures all three baseline components, attributes every delta,
stops for unexplained or overlapping paths, and stages only the accepted task's
source/test and durable task paths. Pre-existing unrelated staged changes are
not automatically unstaged; they block a task-specific final commit until the
user resolves ownership. Unrelated worktree or untracked changes may remain if
their ownership is understood and paths do not overlap.

Committed import keeps its narrower transaction. It commits only newly created
tasks, regenerated index, and the eligible owning brief with `git commit
--only`; it preserves unrelated index and worktree bytes. A change in another
trunk area's `.zdev` directory is unrelated, not part of the import. Any
failure restores the owning brief's prior index/worktree state, removes new
task files, restores the prior index file, and leaves unrelated state as it
was. Successful import leaves only its managed paths clean and reports the
same deterministic path order: eligible brief first, numeric task files, then
`TASKS.md`.

An overlap with the selected task's claimed source/test paths, its task file,
the owning brief when included, `TASKS.md`, or the area/config record being
changed is not “unrelated.” The command or coordinator stops before overwrite
or commit. There is no workspace-wide lock, active-area state, or broad clean
worktree requirement.

## Record policy and cleanup

| Record policy | Trunk mode |
| --- | --- |
| `personal` | Supported. The clone-local exclusion behavior is unchanged. |
| `project` | Supported. Trunk area records remain lasting shared project state. |
| `pull-request` | Unsupported. Create, bind-to-trunk, relationship check, and trunk-aware config validation reject it. |

Pull-request records are designed to live on a review branch and be removed by
`zdev cleanup squash` before squash merge. A trunk area has neither that
feature-branch boundary nor a safe place to make the cleanup commit; cleanup
already refuses configured trunk. Rejecting the combination preserves that
record policy rather than inventing a second cleanup lifecycle.

`cleanup squash` itself is unchanged: it remains pull-request-only, requires a
clean attached non-trunk branch and no active Git operation, removes tracked
`.zdev` paths, and makes one plain commit. Personal/project projects still
reject cleanup. Since valid pull-request projects cannot contain explicit
trunk areas, no mode-specific deletion rule is needed. Legacy isolated areas
stored on trunk still reject cleanup.

## Scenario matrix

“No mutation” includes no Git ref, index, or record change.
Each row uses the exact human line and JSON fields defined in **Status and
task-work projection**, with the listed condition supplying its diagnostic and
null/boolean changes; mode-transition failures use the exact errors above.

| Case | Metadata and status | Command result | Mutation boundary |
| --- | --- | --- | --- |
| Default isolated create on `feature` | no `mode`; `branch = "feature"`; normal anchor fields | Existing success and isolated status | New area directory only |
| One explicit trunk area on checked-out `main` | `mode = "trunk"`; no branch/parent/base; healthy projection above | Named next/completion allowed | Existing task-specific files only |
| Two ready trunk areas | both store only trunk mode; both match `main` | `next --any` chooses lexical area tag, then AFK suitability, priority, and numeric task ID within that area | Read-only |
| Checked-out isolated area plus ready trunk areas | mixed records; isolated matches, trunk areas are structurally safe but off-branch | `next --any` chooses matching isolated candidate first | Read-only |
| Legacy area stored on current trunk | no mode; stored branch remains exclusive; existing same-branch shortcut | A trunk create/bind colliding with it fails | No mutation |
| Legacy area on former trunk | stored old branch; effective base is newly configured trunk | Existing isolated fresh/stale/unsafe result | Reconfiguration changes config only |
| Reconfigure `main` to descendant `stable` with open/closed trunk areas | old tip is ancestor; area files unchanged; all resolve `stable`; old checkout makes open work unsafe | Config command succeeds with old/new branches, tips, lexical areas, and no override; named work waits for checkout; `next --any` may report required `stable` | Atomic config write only |
| Reconfigure `main` to divergent `stable` by default | ancestry is false; all areas still resolve `main` | Reject with lexical affected areas and `--allow-divergent` recovery | No mutation |
| Reconfigure divergent history with `--allow-divergent` | area files unchanged; all open/closed areas resolve `stable`; old checkout is off-branch | Success records `old_is_ancestor = false`, `override = true`, old/new branches and lexical affected areas | Atomic config write only; no Git ref operation |
| Previous configured trunk is missing | old tip/ancestry are null; areas still resolve the old name | Reject even with `--allow-divergent`; restore the old branch first | No mutation |
| Reconfigure to missing branch while trunk areas exist | candidate cannot produce inspectable operating branch | Reject `configured trunk <name> is missing locally` | No mutation |
| Reconfigure onto isolated-owned branch | candidate graph has mixed ownership | Reject naming isolated owner and trunk areas | No mutation |
| Unset trunk while trunk areas exist | trunk projections would become unbound | Reject with lexical area list | No mutation |
| Configured trunk deleted outside zdev | trunk records unchanged; fresh null; structural/task safe false; missing diagnostic | Status succeeds; task work fails with details | Read-only |
| Wrong branch or detached HEAD | fresh true if trunk exists; branch match false/null; task safe false | Status succeeds; named work fails; `next --any` may project off-branch work | Read-only |
| Goal on wrong branch, detached HEAD, or unsafe trunk facts | task graph and lifecycle remain valid | `zdev goal` returns its unchanged branch-independent task projection; workflow work-context still blocks execution | Read-only |
| Active rebase/merge/cherry-pick | operation diagnostic; both safety flags false | Create/bind mode change, reconfigure, committed import, selection, completion, lifecycle change, and managed rebase fail | No zdev mutation; Git recovery state preserved |
| Ordinary import off trunk | branch status need not be safe | Existing transactional import succeeds if area/bundle/graph are valid | New tasks and owning index only |
| Committed import with eligible brief and unrelated changes | trunk checked out; exact owning brief state valid | Import commits brief, new tasks, index; preserves unrelated state | Managed paths and one commit only |
| Committed import with staged/overlapping owning path | ownership is ambiguous | Existing precise error and rollback | Prior bytes/index restored |
| Completion with attributed unrelated worktree files | trunk safe; baseline separates paths | Completion succeeds; later exact staging excludes unrelated files | Task and index only |
| Final commit with unexplained or unrelated staged path | mode grants no ownership | Coordinator stops before `zdev commit` | No mutation |
| Trunk area given any parent | invalid conditional schema/relationship | Parent/bind/check rejects | No mutation |
| Rebase healthy trunk area with dirty unrelated files | no anchor/history relationship | Successful unchanged result; no cleanliness requirement | Read-only |
| Pull-request project requests trunk mode | unsupported record/mode pair | Create/bind/check rejects and points to isolated work | No mutation |
| Cleanup on configured trunk or legacy isolated-on-trunk | existing cleanup guard | Reject | No mutation |

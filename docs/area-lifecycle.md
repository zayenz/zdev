# Area lifecycle and task-queue state

An area objective and its task queue answer different questions. The area is
`open` until someone explicitly closes it. The queue is `empty`, `ready`, or
`exhausted`; task validation rejects a blocked-only graph. Finishing the current
task bundle therefore exhausts the queue but does not close the objective.

This contract adds one lifecycle bit. It does not add abandonment, execution,
integration, branch deletion, switching, or rebasing behavior. Task status
remains `open` or `done`, and slice progress remains derived from tasks.

## Current behavior at the design baseline

At commit `2a8c451e48b9bcf22252a6111551f16f3f5fdbe1`, `AreaMetadata` has no
lifecycle and rejects unknown TOML fields. Goal calls a zero-task area `empty`
and an all-done area `complete`. Area-specific and project-wide next also use
`complete` for exhausted queues. Status reports task counts without an area
state. Task import always creates open tasks, and task reopen can change a done
task back to open. General areas and parent links are conventions and ordinary
area metadata, respectively. The installed implement workflow treats goal
`empty` and `complete` as a successful no-work result. Explicit verify instead
requires the selected ready task and returns its blocker envelope for no work.

The contract below changes that vocabulary because `complete` currently
describes a task count while sounding like an objective decision. It preserves
the existing dependency validation, selection order, branch-safety facts, and
parent-branch mechanics.

## Durable record

`area.toml` gains one field:

```toml
lifecycle = "open"
```

The only valid values are `open` and `closed`. `zdev area create` writes
`lifecycle = "open"`. A record without the field reads as `open`, so every
existing area remains usable without migration. The next successful close or
reopen writes the field. Unknown values and unknown fields remain validation
errors.

The common area validator enforces `closed => no open tasks`. `zdev check`,
status, goal, and selection all use that validator, so a hand-edited closed
record with open tasks fails rather than hiding or selecting the tasks.

`lifecycle` is the field name in TOML and JSON. Human output labels it
`Lifecycle`. Queue projections use the JSON field `queue` and the human label
`Queue`, with these exact values:

- `empty`: the area has no task records;
- `ready`: at least one open task is ready; blocked tasks may also exist;
- `blocked`: open tasks exist but none is ready;
- `exhausted`: at least one task exists and every task is done.

`blocked` is diagnostic vocabulary, not a reachable successful projection.
The current dependency rules reject missing blockers and cycles. Every valid
acyclic graph with an open task has a ready task. Commands fail task validation
instead of returning a successful `blocked` projection.

## State matrix

| Lifecycle | Zero tasks (`empty`) | Ready work (`ready`) | Blocked-only work (`blocked`) | All tasks done (`exhausted`) |
| --- | --- | --- | --- | --- |
| `open` | Valid. The objective remains open for a first bundle. | Valid. Selection may return the first ready task. | Invalid task graph; validation fails without output or mutation. | Valid. The objective remains open for another reviewed bundle or an explicit close. |
| `closed` | Valid. Explicitly closing an empty objective is allowed. | Invalid record combination; close and task creation prevent it, and validation rejects it. | Invalid task graph and invalid closed-area combination. | Valid. The objective is explicitly closed. |

Task counts never change `lifecycle`. The semantic invariant is only that a
closed area has no open tasks. Closing an empty area records an explicit
decision that the objective needs no queued implementation; it does not create
an abandoned task or a third lifecycle value.

## Commands and mutation gates

The public grammar is exactly:

```text
zdev area close <area>
zdev area reopen <area>
```

There are no flags, reason fields, summaries, or implicit task changes. Both
commands acquire the zdev state lock, validate the project relationships and
the target area's brief, slices, tasks, and generated index, and require
`branch_status.task_work.safe = true`. A stale-but-safe link is allowed and
returns the existing rebase advisory. A wrong branch, detached HEAD, active Git
operation, missing branch or anchor, invalid ancestry, or nonlinear child
history blocks the mutation. Ordinary staged, unstaged, or untracked files do
not block it. Publication replaces only the target `area.toml` atomically.

`area close` accepts `empty` and `exhausted` queues. It rejects every open task,
whether ready or blocked, before writing:

```text
Cannot close area <area>: <count> tasks are open. Complete them or keep the area open
```

On a change, human output is `Closed area <area>`. Its JSON contains
`schema_version: 1`, `status: "closed"`, `area`, `lifecycle: "closed"`, the
complete `branch_status`, and `advisory` (a string or `null`). Closing an already
closed area succeeds without writing: human output is
`Area <area> is already closed`, and JSON changes only `status` to `"unchanged"`.

`area reopen` changes only the lifecycle. It does not reopen tasks. On a
change, human output is `Reopened area <area>` and the same JSON shape uses
`status: "open"` and `lifecycle: "open"`. Reopening an open area succeeds
without writing: human output is `Area <area> is already open`, and JSON uses
`status: "unchanged"`. A stale advisory follows the human status on a new line
and appears once in JSON.

## Read and selection vocabulary

Successful read output always reports lifecycle and queue separately. Existing
task, count, slice, branch-status, advisory, and path objects keep their current
shapes.

### `zdev goal <area>`

The human header is exactly:

```text
Area: <tag> — <title>
Lifecycle: <open|closed>
Queue: <empty|ready|exhausted>
Objective:
<objective>
Counts: <total> total; <open> open; <ready> ready; <blocked> blocked; <done> done
```

Ready output then keeps the current task, slice, done-when, validation, and
native-goal sections. The three no-work messages are:

```text
The open area has no tasks. Create and approve a task, or close the area.
The open area's task queue is exhausted. Add approved work, reopen a task, or close the area.
The area is closed. Reopen it before adding or selecting work.
```

They correspond to open/empty, open/exhausted, and either closed queue. Closed
output still reports whether its queue is `empty` or `exhausted` in the header.
Every human result ends with one newline.

Goal JSON removes the ambiguous top-level `state` field. In deterministic key
order it contains `schema_version`, `area`, `lifecycle`, `queue`, `counts`, and
`task`; `native_goal` follows `task` only for `ready`. The `area` object keeps
`tag`, `title`, `objective`, and `path`. `task` is exactly `null` for empty,
exhausted, and closed projections. `native_goal` is omitted for all three.
Thus neither `complete` nor a done count claims that the area objective closed.

### `zdev next <area>`

Area-specific next JSON replaces `status` with the two fields `lifecycle` and
`queue`. Ready and open no-work output retain the existing `branch_status` and
`advisory` fields. Ready output uses `open` and `ready`. Open no-work output
uses `open` with `empty` or `exhausted`. Closed output uses `closed` with its
actual queue and contains exactly `schema_version`, `area`, `lifecycle`,
`queue`, and `task`; `task` is `null`, and `branch_status` and `advisory` are
omitted.
Ready human output prefixes the existing task text with these exact lines:

```text
Area: <area>
Lifecycle: open
Queue: ready
```

One blank line separates this header from the existing task text.

The exact no-work human messages are:

```text
No tasks are recorded in open area <area>. Add approved tasks or run `zdev area close <area>`
The task queue is exhausted in open area <area>. Add approved tasks, reopen a task, or run `zdev area close <area>`
Area <area> is closed. Run `zdev area reopen <area>` before adding or selecting work
```

An open area still passes the existing task-work branch gate before selection,
including an empty or exhausted result. For a closed area, next loads and
validates project and area metadata, relationships, the brief, slices, task
records, and the generated index, then returns the closed result before any Git
or task-work branch check. Observing closure therefore requires neither
`task_work.safe` nor `task_work.structurally_safe`, and works on another branch,
detached HEAD, or during an unrelated Git operation. This read rule does not
weaken mutation gates: `zdev area reopen <area>` still requires
`task_work.safe`, so an off-branch caller must switch to the recorded area
branch first. Invalid records or task graphs fail before projection.

When `<area>` is omitted, an explicit project default still selects that area,
including a closed default, and returns its area-specific result. Without a
default, inference considers only open areas. One open area is selected even
when its queue is empty or exhausted; multiple open areas still require an
explicit area. If none is open, inference fails with
`No area is open. Run \`zdev area reopen <area>\` before selecting work`.

### `zdev next --any`

Project-wide JSON uses `selection: "ready"`, `"none"`, or `"unsafe"` instead
of `status: "complete"`. A ready result includes the selected open area's
`lifecycle: "open"` and `queue: "ready"`. Every result includes
`closed_areas`, exactly a tag-sorted JSON array of area-tag strings. It is `[]`
when no area is closed and, for example, `["alpha", "zeta"]` when those areas
are closed. It never contains objects, branches, or diagnostics. Closed areas
are excluded before branch diagnostics, candidate ranking, and
unsafe-open-work calculation.

A ready human result prefixes the existing task, area, branch, and skipped-area
text with `Selection: ready` and one blank line. A `none` result includes
`reason` in JSON: `"no-ready-open-area"` when open areas exist and
`"no-open-area"` when every area is closed. `reason` is omitted from ready and
unsafe results.

When there are open areas but none has ready work, the successful result uses
`selection: "none"`, `task: null`, and:

```text
No open area has ready work. Open task queues are empty or exhausted
```

When every area is closed, it instead prints:

```text
No area is open. Run `zdev area reopen <area>` before selecting work
```

Structurally unsafe open work keeps `selection: "unsafe"`, the existing
`skipped` evidence, and the existing `No safe task is ready` human wording.
This is distinct from queue exhaustion. A ready area still wins even when
other open areas are unsafe or exhausted.

For every human result with a nonempty `closed_areas`, append one final line:

```text
Excluded closed areas: <comma-separated tags in tag order>
```

Omit the line when `closed_areas` is empty. Thus an all-closed example is
exactly:

```text
No area is open. Run `zdev area reopen <area>` before selecting work
Excluded closed areas: alpha, zeta
```

### `zdev status [<area>]`

Selected-area human output begins with these exact lines before slice and
branch diagnostics:

```text
<title>
Lifecycle: <open|closed>
Queue: <empty|ready|exhausted>
Counts: <total> total; <ready> ready; <blocked> blocked; <done> done
```

Selected JSON keeps the current object and adds top-level `lifecycle` and
`queue`; the embedded area metadata also contains `lifecycle`. Project-wide
human output keeps the project header and lists each area as
`<tag>: <open|closed>, <empty|ready|exhausted>; <branch> -> <relationship> [<diagnostics>]`.
Each project-wide JSON area summary contains `tag`, `title`, `lifecycle`,
`queue`, `total`, `ready`, `blocked`, `done`, and the existing `branch_status`.
Status includes closed areas and diagnoses their branches normally.

## Other operations

- `zdev tasks review` remains read-only and may review a bundle for a closed
  area. `zdev tasks import`, with or without `--commit`, rejects a closed area
  before publication: `Cannot add tasks to closed area <area>. Run \`zdev area
  reopen <area>\` first`. It never reopens the area implicitly.
- `zdev task reopen` rejects a closed area before changing the task:
  `Cannot reopen task <task> in closed area <area>. Run \`zdev area reopen
  <area>\` first`. Task show and list remain available. Task completion cannot
  occur in a valid closed area because closure requires every task to be done.
- `general` remains an ordinary area. A standing general area normally stays
  open across exhausted queues. It closes and reopens only through the same
  explicit commands and gates.
- Parent and child lifecycle values are independent. Closing a parent neither
  closes its children nor removes its branch as their effective base. An open
  child of a closed parent remains selectable when its existing branch link is
  safe. Closing a child does not affect its parent. Parent assignment and
  managed rebase continue to use records and branches, not lifecycle.
- Slices gain no lifecycle and do not affect area closure.
- After valid preflight evidence, the implement entrypoint may return a
  successful no-work result for open/empty, open/exhausted, or closed and starts
  no worker. Explicit verify always requires an open/ready goal matching its
  requested task ID. For every no-work goal it starts no verifier and returns
  `BLOCKER zdev-verify`, never a successful verification. Both paths preserve
  their complete status, goal, and Git-baseline evidence requirements.

## Narrow implementation task

Outcome: implement the two-value area lifecycle and make every queue projection
distinguish exhaustion from explicit closure.

Boundaries: add no state beyond `open` and `closed`; do not alter task or slice
status; do not integrate, delete, create, switch, or rebase branches; retain the
existing task ordering, graph validation, branch diagnostics, and atomic-write
helpers.

Implementation seam:

1. Add a defaulted lifecycle enum to `AreaMetadata` in `project.rs`, plus locked
   close and reopen operations using the existing area validator, task summary,
   task-work branch gate, and atomic metadata publisher.
2. Route the two CLI subcommands in `lib.rs` and add one shared queue classifier
   over the existing task summary. Use it from status, `goal.rs`, and
   area-specific and project-wide selection rather than duplicating dependency
   logic.
3. Gate task import and task reopen on the area's lifecycle. Exclude closed
   areas from project-wide selection before Git diagnostics.
4. Update the canonical shared workflow guidance, harness-specific strict
   no-work parsers, user documentation, and generated fixtures through the
   established renderer.

Acceptance criteria:

- Missing `lifecycle` reads as open; new records write it; invalid values fail;
  close and reopen are atomic, locked, idempotent, and match the command,
  branch-gate, message, JSON, and advisory contract above.
- Focused black-box coverage exercises every matrix cell, including empty
  closure, an open exhausted area, rejection with open tasks, closed-record
  validation, and backward-compatible records without the field.
- Goal, next, next-any, and both status modes use only `Lifecycle`/`lifecycle`
  and `Queue`/`queue` for these concepts. No successful output calls an
  exhausted open area `complete` or implies that its objective closed.
- Import and task reopen reject closed areas without mutation. Reopening an
  area changes no task. General and parent/child cases follow the rules above.
- Closed areas never produce ready work or start an orchestration worker;
  project-wide selection excludes them before branch safety analysis.
- Area-specific next returns a validated closed result without Git or task-work
  branch checks and omits `branch_status` and `advisory`; close and reopen still
  require `task_work.safe`.
- After valid preflight evidence, implement may return successful no-work for
  open/empty, open/exhausted, and closed goals. Explicit verify returns
  `BLOCKER zdev-verify` without a worker for those same goals and succeeds only
  through an open/ready selection.
- Canonical templates render deterministically, install/check succeeds for all
  five harnesses, and the lean and full validation suites pass.

This design is based on the repository at
`2a8c451e48b9bcf22252a6111551f16f3f5fdbe1`. It does not test how third-party
harnesses display installed entrypoints; it defines the generated artifact
contract that zdev can validate locally.

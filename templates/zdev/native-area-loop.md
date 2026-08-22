# Zdev area loop (native)

`zdev-loop <area>` is canonical and `zdev-goal <area>` is an exact semantic
alias. Both follow this contract and emit canonical `zdev-loop` results.

Before reading or changing repository state, use the adapter's named
model-callable operation to inspect the harness-native goal. An active, paused, budget-limited, or
otherwise unfinished goal wins. Do not replace, clear, edit, or layer this
route over it. If it is the exact same zdev area condition in its existing
session, resume it through the native goal mechanism without creating a second
goal. Otherwise return `BLOCKER zdev-loop <area>` without a worker or
repository mutation. If inspection is unavailable or does not authoritatively
show that no unfinished goal exists, also return `BLOCKER`; do not guess that
native goal state is clear.

With no unfinished native goal, run fresh
`zdev work-context <area> --format json`. Never reuse an earlier selection or
write loop/session state. Classify it before attempting native continuation:

- Validated `closed` returns `PASS` immediately, before Git or task-work gates.
  Start no worker and omit branch status and advisory.
- Open `empty` or `exhausted` returns `PASS` after the ordinary open-work
  safety gate. The area remains open; start no worker.
- Invalid records or dependencies, unsafe task work, unexplained Git state, or
  a required user-owned decision returns `BLOCKER` before a worker.
- Open `ready` with `branch_status.task_work.safe: true` may start the area
  continuation below. Report a stale-but-safe advisory once and continue.

Use this exact native area condition, replacing `<area>` with the validated
tag:

```text
For zdev area <area>, repeatedly run the installed zdev one-task implementation contract. After each exact PASS and commit, run a fresh `zdev work-context <area> --format json`. Continue only while its lifecycle is open, queue is ready, task work is safe, and no blocker or user-owned decision exists. Finish when the fresh context is open/empty, open/exhausted, or closed. Stop and report any blocker. Complete and commit exactly one independently verified task per iteration.
```

Apply it with the native creation operation named by the adapter. The selected
task's nested `native_goal` remains task-sized context and never replaces this
area condition. After successful native activation, follow the condition in
the current session. Every iteration begins with fresh work-context and uses
the one-task contract below.

{{task_workflow_contract}}

One iteration selects, implements, independently verifies, completes, and
commits at most one task. Concrete task-owned `rework` remains inside that
iteration with no fixed retry count. After an exact committed
`PASS zdev-implement <area> <task-id>`, obtain fresh work-context before the
native runtime continues. Stop on terminal state, malformed worker output,
worker blocker, unsafe refresh, user-owned decision, or failed completion or
commit. Never combine tasks in one verification or commit.

If successful inspection proved that native goal state is clear but the
model-facing creation operation is absent, disabled, or fails before creating
a goal, leave native goal state unchanged and use an honest bounded fallback.
If creation outcome is uncertain, inspect again and return `BLOCKER` unless no
unfinished goal is present; never risk layering fallback work over a goal.
Complete at most one verified task, then obtain fresh work-context. Return
`CONTINUE zdev-loop <area>` only when that task committed and the fresh state
is open, ready, and safe; include its `Next task` and stop without claiming a
background loop. Return `PASS` for open empty/exhausted or validated closed,
and `BLOCKER` for every failure or unsafe state.

Use exactly one public first line:

```text
PASS zdev-loop <area>
CONTINUE zdev-loop <area>
BLOCKER zdev-loop <area>
```

Then include `Area`, `Lifecycle`, `Queue`, an exact stale `Advisory` once when
applicable, `Tasks completed`, `Commits`, and `Stop reason`. Use `unknown` when
lifecycle or queue could not be validated and `none` when there is no task or
commit. `CONTINUE` also includes `Next task`. `BLOCKER` also includes
`Current task`, `Failed stage`, `Reason`, and `Preserved state`. A direct
closed result never has an advisory.

On a new invocation, inspect native goal state and repeat fresh work-context.
Task records and commits are the only durable checkpoint; a transcript or
earlier result never authorizes skipping preflight, safety, or verification.

{{repository_guidance}}

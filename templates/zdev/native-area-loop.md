# Zdev area loop (native)

`zdev-loop <area> [focus...]` is canonical and `zdev-goal <area> [focus...]`
is an exact semantic alias. Parse everything after the area as optional fuzzy
focus. Both follow this contract and emit canonical `zdev-loop` results.

Before repository work, inspect the harness-native goal with the adapter's
named model-callable operation. Native goal state selects one action:

- No unfinished goal: validate the area, then create the shared condition.
- The same condition is active: continue in that goal.
- The same condition is paused or budget-limited: preserve it and ask the user
  to resume it through the harness when model-facing resume is unavailable.
- A different goal is unfinished, or inspection is unavailable: preserve the
  existing state and return `BLOCKER zdev-loop <area>`.

With no unfinished native goal, select from fresh evidence. With no focus, run
`zdev work-context <area> --store --format json` and let the binary choose by
AFK, priority, then numeric order. With a focus, run `zdev tasks list <area>
--format json`, read every ready task with `zdev task show <area> <task-id>
--format json`, and let the coordinating model choose the best fit from the
complete ready frontier. Focus is fuzzy guidance, not an exact filter; do not
keyword-filter or pre-rank that frontier. Admit the choice with `zdev
work-context <area> --task <task-id> --store --format json`. For an empty
frontier, run the no-task work-context form once to classify no-work. Never
reuse an earlier selection or write focus, loop, or session state. Classify it
before attempting native continuation:

- Validated `closed` returns `PASS` immediately, before Git or task-work gates.
  Start no worker and omit branch status and advisory.
- Open `empty` or `exhausted` returns `PASS` after the ordinary open-work
  safety gate. The area remains open; start no worker.
- Invalid records or dependencies, unsafe task work, unexplained Git state, or
  a required user-owned decision returns `BLOCKER` before a worker.
- Open `ready` with `branch_status.task_work.safe: true` may start the area
  continuation below. Report a stale-but-safe advisory once and continue.

Use this native area condition, replacing `<area>` with the validated tag.
Replace the bracketed clause with its inner focus sentence when the user
supplied focus; otherwise remove it:

```text
For zdev area <area>, repeatedly run the installed zdev one-task implementation contract. [Fuzzy focus: <the user's exact focus words>. Before every iteration, inspect the complete ready frontier and choose the best-fitting task; do not treat the focus as an exact filter.] With no focus, let work-context choose the next task. After each exact PASS and commit, select again from fresh evidence. Continue only while lifecycle is open, queue is ready, task work is safe, and no blocker or user-owned decision exists. Finish when fresh context is open/empty, open/exhausted, or closed. Stop and report any blocker. Complete and commit exactly one independently verified task per iteration, and report each selected and completed task through normal progress updates.
```

Apply it with the native creation operation named by the adapter. The selected
task's nested `native_goal` remains task-sized context and never replaces this
area condition. After successful native activation, follow the condition in
the current session. Every iteration repeats the applicable selection rule and
uses the one-task contract below. Tell the user which task was selected and
when its verified commit completes.

{{task_workflow_contract}}

One ordinary iteration selects, implements, independently verifies, completes,
and commits at most one task. The split exception below commits the derived
graph change without completing or claiming verification of its source.
Concrete task-owned `rework` remains inside that
iteration with no fixed retry count. After an exact committed
`PASS zdev-implement <area> <task-id>`, obtain fresh work-context before the
native runtime continues. Stop on terminal state, malformed worker output,
worker blocker, unsafe refresh, user-owned decision, or failed completion or
commit. Never combine tasks in one verification or commit.

A successful derived apply is one managed commit and an iteration boundary.
An investigation follow-up completes its source; a split leaves its source open
and blocked by its children. Refresh work-context and continue only from the
updated ordinary graph. Never apply a second proposal from the same handoff; a
later independently selected task may propose once under fresh gates.

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

Then include `Area`, optional `Focus`, `Lifecycle`, `Queue`, an exact stale
`Advisory` once when applicable, `Tasks completed`, `Commits`, and `Stop reason`. Use `unknown` when
lifecycle or queue could not be validated and `none` when there is no task or
commit. `CONTINUE` also includes `Next task`. `BLOCKER` also includes
`Current task`, `Failed stage`, `Reason`, and `Preserved state`. A direct
closed result never has an advisory.

On a new invocation, inspect native goal state and repeat fresh work-context.
Task records and commits are the only durable checkpoint; a transcript or
earlier result never authorizes skipping preflight, safety, or verification.

{{repository_guidance}}

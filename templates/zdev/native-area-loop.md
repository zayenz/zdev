# Zdev area loop (native)

{% if research_supported %}An explicitly authorized research run uses the separate installed research
reference. This ordinary loop does not generate work or dispatch hypotheses.
{% endif %}

Resolve the run's concrete role settings once before the first iteration and
retain them in the goal conversation. Every continued iteration uses those
values; do not re-resolve profile names between tasks.

When the user authorizes parallel work for this run, read
`references/parallel.md`. Keep four independent task slots, refill each from
fresh compatible work, and serialize destination acceptance. Retain the run
authority and concrete role settings in the conversation. The ordinary one-task
instructions below apply when parallel continuation was not authorized. Keep this choice in
the native goal condition throughout the run.

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
complete ready frontier. When any ready task has `afk = true`, choose only among
those unattended tasks and use the focus to rank that eligible set. Choose an
`afk = false` attended task only when no unattended task is ready. Focus is
fuzzy guidance, not an exact filter; do not keyword-filter or pre-rank that
frontier. Admit the choice with
`zdev work-context <area> --task <task-id> --store --format json`. For an empty
frontier, run the no-task work-context form once to classify no-work. Never
reuse an earlier selection or write focus, loop, or session state. Classify it
before attempting native continuation:

- Validated `closed` returns `PASS` immediately, before Git or task-work gates.
  Start no worker and omit branch status and advisory.
- Open `empty` or `exhausted` returns `PASS` after the ordinary open-work
  safety gate. The area remains open; start no worker.
- Open `blocked`, where every open task waits on a task in another area, also
  returns `PASS` after the ordinary open-work safety gate. The area remains
  open; start no worker or native continuation. Report each wait in the goal's
  `cross_area_waits` as `<task> blocked by <id> (<area>: <status>)`. Do not
  select or start work in the blocking area.
- Invalid records or dependencies, unsafe task work, unexplained Git state, or
  a required user-owned decision returns `BLOCKER` before a worker.
- Open `ready` with `branch_status.task_work.safe: true` may start the area
  continuation below. Report a stale-but-safe advisory once and continue.

Run `zdev goal <area> --format json` and pass its `native_goal` unchanged to
the adapter's native creation operation. Pass the user's exact focus as
`--focus <text>` when supplied, and `--parallel` when parallel continuation is
authorized. The CLI owns the continuation and completion wording. Keep selected
tasks, snapshots, progress, and concrete worker settings in the conversation;
refresh them as work advances. Retain the same goal across task completions.

After activation, follow the selection rule above and the applicable execution
contract. Report each selected task and its verified commit.

Reporting a terminal `BLOCKER` satisfies this run's explicit stop condition.
End the matching active native goal through the adapter's completion operation
in that same turn. Keep the public result `BLOCKER` and preserve the exact task,
area, and Git state; ending the run does not complete unfinished tasks or close
the area. Do not repeat workers or wait for more goal turns to reconfirm an
established impasse. Partial progress and actionable in-scope rework still
continue under the one-task contract. Never complete a different, paused, or
budget-limited native goal.

The exact installed task-workflows contract path is
{{ task_workflows_contract_path_json }}. Decode it. Resolve a relative path
against the directory containing the loaded root zdev SKILL.md and include the
resulting absolute path in every worker payload.

{{task_workflow_contract}}

One ordinary iteration selects, implements, independently verifies, completes,
and commits at most one task. The split exception below commits the derived
graph change without completing or claiming verification of its source.
Concrete task-owned `rework` follows the bounded correction cycle in the
supplied coding guidance's **Review** reference. Exhausted corrections stop the
loop as a blocker; continuation preserves the review stage. A worker result that reports only partial
progress, remaining in-scope work, or an additional attributable path continues
inside the iteration; it is not a terminal blocker. After an exact committed
`PASS zdev-implement <area> <task-id>`, obtain fresh work-context before the
native runtime continues. Stop on terminal state, malformed worker output,
genuine worker blocker, unsafe refresh, user-owned decision, or failed completion or
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
background loop. Return `PASS` for open empty/exhausted/blocked or validated
closed, and `BLOCKER` for every failure or unsafe state.

Use exactly one public first line:

```text
PASS zdev-loop <area>
CONTINUE zdev-loop <area>
BLOCKER zdev-loop <area>
```

Then include `Area`, optional `Focus`, `Lifecycle`, `Queue`, `Cross-area
waits` for an open `blocked` queue, an exact stale
`Advisory` once when applicable, `Tasks completed`, `Commits`, and `Stop reason`. Use `unknown` when
lifecycle or queue could not be validated and `none` when there is no task or
commit. `CONTINUE` also includes `Next task`. `BLOCKER` also includes
`Current task`, `Failed stage`, `Reason`, and `Preserved state`. A direct
closed result never has an advisory.

On a new invocation, inspect native goal state and repeat fresh work-context.
Task records and commits are the only durable checkpoint; a transcript or
earlier result never authorizes skipping preflight, safety, or verification.

{{repository_guidance}}

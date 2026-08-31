---
description: Continue a zdev area, completing at most one independently verified task
---

# Zdev area loop (bounded)

Parse the first token of `$ARGUMENTS` as the area and the remaining text as
optional fuzzy focus. `zdev-loop` is the canonical name; `zdev-goal` is an
exact alias. Both commands follow this same contract and always emit the
canonical `zdev-loop` result.

With no focus, start by running `zdev work-context <area> --store --format
json`; the binary selects one task by AFK, priority, then numeric order. With a
focus, run `zdev tasks list <area> --format json`, then `zdev task show <area>
<task-id> --format json` for every task whose state is `ready`. Give the
coordinating model the complete ready frontier and the user's words, and let it
choose the best fit. The focus is guidance, not an exact filter: do not discard
or pre-rank ready tasks by keywords. Admit the chosen task with `zdev
work-context <area> --task <task-id> --store --format json`. For an empty
frontier, run the no-task work-context form once to classify ordinary no-work.
Do not reuse context from an earlier invocation or write focus, loop, or
session state.
Classify the result as follows:

- `closed` returns `PASS` immediately, before Git or task-work gates. Start no
  worker and omit branch status and advisory.
- Open `empty` or `exhausted` returns `PASS` after the ordinary open-work
  safety gate. The area remains open; start no worker.
- Open `ready` with `branch_status.task_work.safe: true` runs the one-task
  contract below for exactly the selected task. Report a stale-but-safe
  advisory once and continue.
- Invalid records, missing blockers, dependency cycles, unsafe task work,
  unexplained Git state, or a required user-owned decision returns `BLOCKER`
  before a worker or further mutation.

For open `ready`, the validated entry context is the one-task contract's
initial work-context. Reuse it for the first dispatch, and tell the user which
task was selected before implementation begins.

The exact installed task-workflows contract path is
{{ task_workflows_contract_path_json }}. Decode it and include the resulting
path in every worker payload.

{{task_workflow_contract}}

Concrete task-owned `rework` remains inside that one task cycle with fresh
independent verification and no fixed retry count. Partial progress, remaining
in-scope work, or an additional attributable path also remains inside the
cycle. A malformed worker result, genuine worker blocker, unsafe refresh,
failed completion, or failed commit returns
`BLOCKER` and stops. Each ordinary successful invocation completes and commits
at most one selected task through exactly one independently accepted
verification. The split exception below commits the derived graph change
without completing or claiming verification of its source.

A successful derived apply is also an iteration boundary. An investigation
follow-up completes its source; a split leaves its source open and blocked by
the new children. In either case, collect fresh work-context from the updated
ordinary graph before deciding whether to continue. Do not apply a second
proposal from the same handoff. A later independently selected task may propose
again under fresh authority checks.

After an exact committed `PASS zdev-implement <area> <task-id>` or a successful
derived apply, tell the user that task finished, then make one fresh selection
using the same no-focus or focused rule before deciding the public result. If
it reports open `ready` and safe task work, return `CONTINUE`, name that fresh
next task, and stop. Do not start it or claim a background loop. If it reports
open `empty`, open `exhausted`, or validated `closed`, return `PASS` and stop.
Any validation, safety, or refresh failure returns `BLOCKER`; do not count or
start another task.

Use exactly one of these first lines:

```text
PASS zdev-loop <area>
CONTINUE zdev-loop <area>
BLOCKER zdev-loop <area>
```

Then include `Area`, optional `Focus`, `Lifecycle`, `Queue`, an exact stale
`Advisory` once when applicable, `Tasks completed`, `Commits`, and `Stop reason`. Use `unknown` for
lifecycle or queue when validation failed before classification, and `none`
when there is no task or commit. `CONTINUE` also includes `Next task` with the
fresh ready task ID. `BLOCKER` also includes `Current task`, `Failed stage`,
`Reason`, and `Preserved state`. A direct closed result never has an advisory.

On restart, repeat this contract from fresh repository evidence. Task records
and commits are the only checkpoint. A previous `CONTINUE`, transcript, or
session resume is context, never authority to skip preflight or verification.

{{repository_guidance}}

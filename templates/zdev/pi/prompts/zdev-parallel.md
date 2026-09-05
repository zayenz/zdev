---
description: Run one approved bounded parallel zdev task batch
argument-hint: <area> <task-id> <task-id> [...]
---

Load the Parallel route from the installed zdev-pi skill and follow its shared
admission, isolation, serial integration, completion, stopping, and recovery
contract. Use `$ARGUMENTS` only as the requested area and finite task list; all
required user choices must already be explicit before source mutation.

The current Pi session is the coordinator. Use batch operations only for
read-only advanced planners and initial implementation workers in assigned
source worktrees. Call `start` once with an opaque run ID, the complete finite
task list, and the agreed worker limit. It returns the first settled task while
other admitted children may remain active. Integrate that candidate and run its
verifier or rework through the legacy single-worker form. Only after that serial
gate may the coordinator call `continue`; that call returns the next settled
task. It drains every already-settled result through this one-result and serial
gate sequence before it may fill available slots from pending tasks. Repeat the
single gate then `continue` sequence until the handle expires with its final
result. Count every active planner, implementer, and verifier against the agreed
total limit when choosing the batch worker limit.

Consume cumulative updates and the final task-attributed results in completion
order. Call `cancel` when stopping the run; it propagates cancellation, accounts
for active and pending tasks, and removes the handle. Preserve failed,
cancelled, not-dispatched, and unconfirmed-stop entries with their source work.
The handle exists only in this Pi session. Recovery uses retained Git work and
fresh admission and never depends on a prior handle or run ID.

{{repository_guidance}}

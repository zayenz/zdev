# Run one approved parallel task batch

Parallel execution is an explicit, finite route. A discussion, ready queue,
goal, loop, or ordinary Implement request does not select it.

{% if parallel_supported %}
## Native harness support

This harness supports this route through its native parallel runtime. Before any source
mutation, collect or reuse explicit authorization for all of these choices:

- two or more existing task IDs from one area;
- creation of one source worktree and branch per admitted task;
- the authoritative destination branch and checkout;
- a total delegated-role limit, counting planners, implementers, and verifiers;
- whether run-owned worktrees and temporary branches may be cleaned up.

Explain that overlap can reduce elapsed time while consuming usage faster and
may add integration work. Propose a limit of two when none was supplied.
Completing this batch authorizes no additional task or batch.

## Admission

Inspect the destination, applicable repository instructions, visible agents
and competing work, available collaboration capacity, and every requested
task. Require at least two ready tasks whose likely source changes, shared
interfaces, validation resources, generated outputs, services, ownership, and
local resource needs are mutually compatible. Dependency readiness alone is
insufficient. Reject duplicate task IDs. If fewer than two candidates remain,
or the harness cannot provide isolated workers within the agreed limit, explain the
reason and offer ordinary sequential Implement before creating a source tree.

Reuse the **Optional assigned source worktree** contract in the installed
task-workflows reference unchanged for every admitted task. Keep briefs, task
records, lifecycle, snapshots, completion, staging, and final commits in the
authoritative destination. Create one run-owned source worktree and branch
from each task's admitted destination baseline. Give every source worker the
absolute source cwd, authoritative root and record paths, baseline, and
snapshot locator. Source edits, validation, and ordinary transport commits
run in that cwd. Never copy authoritative records into a source tree.

## Dispatch and role capacity

Maintain an in-memory set of dispatched task IDs and never dispatch one twice.
Inspect available capacity before every dispatch. Run the
required advanced planner before edits, then spawn the configured implementation
profile with `fork_turns="none"` and the explicit source cwd. Count every live
planner, implementer, and verifier against the agreed and harness limits.
Accept results as workers finish. When a candidate is ready,
give the next available role slot to its destination integration verifier
before starting another implementation.

## Serial integration and completion

Workers may overlap only in their assigned source trees. Process finished
candidates one at a time in the destination:

1. Refresh admission for that explicit task ID and inspect its complete source
   delta from the recorded baseline, including additions, deletions, binary
   files, and modes.
2. Create and inspect an ordinary source commit, then integrate it with the
   assigned-worktree no-commit transport into the authoritative destination.
3. Store fresh explicit-task context, run a fresh independent verifier against
   the integrated destination, route any rework to the same source tree, and
   preserve the snapshot comparison and completion gates.
4. Complete only that task, stage only its attributable source and generated
   task records, inspect the staged diff, and make exactly one zdev commit.

A destination commit made by an earlier task in this batch is expected drift.
Reconcile each sibling delta against the new destination before integration.
If the commit changes an affected task's requirements or shared interface,
send the changed requirements to its active worker before accepting its result.
Unexpected destination or record changes still require an ownership check.
Once the destination contains an unresolved integration, verification,
completion, staging, or commit checkpoint, pause further integration; workers
may finish safely in isolation.

## Blockers, stopping, and resume

A task-local blocker preserves its source tree, result, and findings while
unaffected tasks continue. A shared decision pauses every affected task. A
derived split or follow-up proposal remains with its source work and is not
reviewed, applied, or used to complete the source during this batch; report it
for a later explicitly authorized ordinary derived-work interaction.

Cancellation or loss of capacity stops new dispatch. Use supported agent
interruption, account for every active or unconfirmed worker, and retain any
tree whose result or termination is ambiguous. Do not clean up until workers
are confirmed stopped. Resume only when explicitly requested, following the
parallel recovery section in `recovery.md`; rerun admission and never rely on
old agent IDs.

The final report separates tasks with successful destination commits from
unfinished preserved tasks and their worktree, branch, baseline, commits, and
last result. It also states cleanup performed or withheld and the stop reason.
{% else %}
## Unsupported adapter

This installed harness adapter does not support zdev's parallel execution
contract with its current native worker controls. Do not dispatch concurrent
task implementations or create batch worktrees. Explain the missing control
and offer ordinary sequential **Implement** for the requested tasks.
{% endif %}

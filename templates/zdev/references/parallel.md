# Run parallel tasks

An explicit parallel request selects this route. A finite task list bounds the
work; parallel authorization for an ongoing goal permits fresh task admission.

Inspect every requested or resumed task for `hypothesis` before creating source
worktrees or dispatching workers. This ordinary parallel route admits ordinary
tasks only. The installed task-format reference defines the available research
record operations.

{% if parallel_supported %}
## Native harness support

Use four independent task slots by default, with one source worktree per task.
A slot carries its task through planning, implementation, verification, and
commit. Reuse explicit worker limits and workspace choices. Available harness
capacity bounds live delegated roles; the coordinator is separate.

An explicit parallel request authorizes the default worktrees and slots. Reuse
the named area, eligible work and authoritative destination. Ask only for an
unresolved destination or scope decision. Preserve worktrees unless cleanup is
authorized. Report that parallel work consumes usage faster.

A finite task list ends after those tasks. Parallel authorization for an ongoing
goal covers fresh compatible tasks in that area: refill each free slot as soon
as its previous task commits, without waiting for the other slots. Retain scope,
worker settings and workspace choices across continuation and recovery.

## Admission

Inspect the destination, applicable repository instructions, visible agents
and competing work, available collaboration capacity, and every requested
task. Start parallel work with at least two ready tasks whose likely source changes, shared
interfaces, validation resources, generated outputs, services, ownership, and
local resource needs are mutually compatible. Dependency readiness alone is
insufficient. Reject duplicate task IDs. Refill one slot at a time while peers
remain active. When only one eligible task remains, finish it through the same
worktree and acceptance contract. If the harness cannot isolate workers within
its capacity, report that limit before creating a source tree.

Reuse the **Optional assigned source worktree** contract in the installed
task-workflows reference unchanged for every admitted task. Keep briefs, task
records, lifecycle, snapshots, completion, staging, and final commits in the
authoritative destination. Create one run-owned source worktree and branch
from each task's admitted destination baseline. Give every source worker the
absolute source cwd, authoritative root and record paths, baseline, and
snapshot locator. Source edits, validation, and ordinary transport commits
run in that cwd. Never copy authoritative records into a source tree.

## Dispatch and role capacity

Freeze the run's concrete role settings at admission and include them in
every assigned-task handoff. Each lane uses those values for planning,
implementation, verification, and rework even if shared profile preferences
change while the batch runs.

Track each slot's task, phase, worker, and source root in the conversation.
Admit replacement work against every occupied slot and the current destination.
Dispatch each task once; an existing assignment resumes its recorded phase.
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
   the integrated destination, and route accepted corrections to the same source
   tree under **Bounded corrections** in [review.md](review.md). Carry the initial
   review into the focused recheck. Preserve review stage, snapshot comparison,
   and completion gates; another batch or recovery does not reset the allowance.
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
For an ongoing goal, select replacement work at each completed slot. A runtime
that only accepts a finite task list drains that invocation before the next;
report this adapter limit instead of promising rolling admission. Keep the
run's scope and settings across invocations.
{% else %}
## Unsupported adapter

This installed harness adapter does not support zdev's parallel execution
contract with its current native worker controls. Do not dispatch concurrent
task implementations or create batch worktrees. Explain the missing control
and offer ordinary sequential **Implement** for the requested tasks.
{% endif %}

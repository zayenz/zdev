# Plan one ready task without implementing it

This route is selected by the exact request “plan next task with advanced
planner.” It is read-only planning even when the selected task is routine or
standard. Codex supports this route now. Claude Code, OpenCode, Pi, and Oh My
Pi must report that scoped plan-only dispatch is not yet supported by their
current adapter; they must not approximate it with implementation.

Use normal area selection. If an area was named, run `zdev work-context <area>
--store --format json`. If the user explicitly chose a ready task, add `--task
<task-id>`. Preserve the compact snapshot and show it to validate the complete
open, ready, safe context. Keep the selected task ID explicit in every later
refresh. Closed, empty, exhausted, unsafe, ambiguous, or mismatched context
stops without changing it.

Resolve the run's concrete role settings once under **Scoped execution
profiles**. For this step only, resolve `planner` with one-off profile
`advanced`; retain that concrete result for a same-step retry. The task's
authored complexity remains unchanged, and the planner choice does not become
the run's implementation, advanced-implementation, or verifier choice.

Start exactly one fresh read-only planner. Give it the immutable work-context
snapshot, brief, optional slice, complete task, repository guidance, and the
task-workflow contract. Validate its four-field semantic result and construct
the public planner envelope exactly as required by that contract. Return the
selected area and task, the plan's approach, paths, validation, findings, the
snapshot baseline, and the concrete planner profile, model, and effort.

Stop after returning the plan. Do not start an implementer or verifier, mutate
source or `.zdev`, complete a task, draft or import tasks, stage or commit,
switch or create a branch, or create a worktree.

Keep the task identity, task requirements, baseline snapshot, semantic plan,
and concrete planner settings in the conversation or native workflow handoff.
A later explicit implementation is a new authorized interaction: collect fresh
explicit-task context and compare the task identity, requirements, baseline,
and relevant checkout state. Reuse the plan when those inputs still make it
applicable. If it is absent or materially stale, say why and follow the normal
planning rule for the task's authored complexity. Never carry the one-off
advanced planner setting into that implementation or its verifier.

---
name: zdev
description: "Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use when the user invokes zdev or $zdev, names an existing .zdev area, task, or note, or asks to continue stored zdev work."
---

# Zdev for Claude Code

{{t3_code_routing}}

{{shared_contract}}

## Claude Code orchestration

This section is the native route. A run that selected T3 orchestration follows
[references/t3-code.md](references/t3-code.md) and never uses these subagents or
packaged workflows.

Route the goal's authored complexity through `zdev:zdev-routine-implementer`,
`zdev:zdev-implementer`, or `zdev:zdev-advanced-implementer`. Advanced work
first uses one read-only `zdev:zdev-planner`. Always verify with a fresh
`zdev:zdev-verifier`. Ordinary rework stays on the selected profile; one valid
standard-work escalation uses an advanced replacement without replanning.
Include rendered repository guidance in every prompt. If named agents are
unavailable, use ordinary Claude Code subagents with the same profiles and
boundaries.

When an implementer will author human-facing prose, include the shared `Write
human-facing prose plainly` section in its prompt. Other workers do not need
that editorial guidance.

The root zdev skill selects the route and loads its contract from `references/`.
On the native route, when packaged workflows are available, the root zdev skill
uses them internally for a full task cycle, explicit verification, audit, or
continuing area work. “Goal” and “loop” select the same continuation workflow.
The ordinary subagent loop also works.

On the native route, an explicit research-run request uses
[references/research.md](references/research.md) and the packaged
`zdev-research` workflow. Pass structured start/resume arguments, a fresh
invocation identity, execution serial (default) or rolling, and retained
concrete profiles. Rolling start accepts separate positive worker_limit and
measurement_limit values; resume carries execution, cleanup, retained_plans and
retained_candidates from its prior report. After stop/failure,
use a new resume invocation of the same durable run. Ordinary Loop keeps its
existing continuation behavior.

On the native route, an explicit parallel request uses the packaged
`zdev-parallel` workflow when it is installed. Resolve the finite task list and
destination, use four workers and source worktrees by default, and retain
explicit overrides and cleanup authority. The parallel request supplies
consent. If the workflow runtime is unavailable, report that parallel execution
is unavailable and offer the unchanged sequential Implement route.

On the native route, an active-zdev goal or loop request uses the packaged
continuation workflow when it is available. It repeats the ordinary one-task
route, refreshes work context after every verified commit, and applies the
shared stop states. It does not inspect or invoke Claude Code's separate
`/goal` command. If the packaged workflows are unavailable, continue under
coordinator control or stop after one task and report the fresh next state.

Before launching a packaged workflow, tell the user which area and fuzzy focus
will be used. Workflow labels report selection, the chosen task ID, planning,
implementation, verification, rework, and commit stages; do not replace them
with a generic “waiting for dynamic workflow” update. Report completed task IDs
and commits from the final envelope.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

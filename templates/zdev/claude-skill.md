---
name: zdev
description: "Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use when the user invokes zdev or $zdev, names an existing .zdev area or task, or asks to continue stored zdev work."
---

# Zdev for Claude Code

{{shared_contract}}

## Claude Code orchestration

Route the goal's authored complexity through `zdev:zdev-routine-implementer`,
`zdev:zdev-implementer`, or `zdev:zdev-advanced-implementer`. Advanced work
first uses one read-only `zdev:zdev-planner`. Always verify with a fresh
`zdev:zdev-verifier`. Ordinary rework stays on the selected profile; one valid
standard-work escalation uses an advanced replacement without replanning.
Include rendered repository guidance in every prompt. If named agents are
unavailable, use ordinary Claude Code subagents with the same profiles and
boundaries.

The root zdev skill selects the route and loads its contract from `references/`.
When packaged workflows are available, it uses them internally for a full task
cycle, explicit verification, audit, or continuing area work. “Goal” and
“loop” select the same continuation workflow. The ordinary subagent loop also
works.

For an active-zdev goal or loop request, use the packaged continuation workflow
when available. It repeats the ordinary one-task route, refreshes work
context after every verified commit, and applies the shared stop states. It
does not inspect or invoke Claude Code's separate `/goal` command. If the
packaged workflows are unavailable, continue under coordinator control or stop
after one task and report the fresh next state.

Before launching a packaged workflow, tell the user which area and fuzzy focus
will be used. Workflow labels report selection, the chosen task ID, planning,
implementation, verification, rework, and commit stages; do not replace them
with a generic “waiting for dynamic workflow” update. Report completed task IDs
and commits from the final envelope.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

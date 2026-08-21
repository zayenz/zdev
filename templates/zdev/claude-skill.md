---
name: zdev
description: "Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use only when the user invokes zdev or $zdev; asks to work through an existing .zdev area; or unmistakably refers to zdev's stored areas or tasks."
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

When the packaged workflows are available, `/zdev:zdev-implement` runs a full
task cycle, `/zdev:zdev-verify` verifies an explicit current ready task without
mutation, and `/zdev:zdev-audit` runs a read-only audit. The ordinary subagent
loop also works.

For an active-zdev goal or loop request, repeat the ordinary one-task route in
the current interaction, refreshing work context after every verified commit
and applying the shared stop states. This route does not inspect or apply
Claude Code's separate `/goal` command. If continuation cannot remain under
coordinator control, stop after one task and report the fresh next state.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

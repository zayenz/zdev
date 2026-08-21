---
name: zdev
description: "Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use only when the user invokes zdev or $zdev; asks to work through an existing .zdev area; or unmistakably refers to zdev's stored areas or tasks."
---

# Zdev for Claude Code

{{shared_contract}}

## Claude Code orchestration

For an ordinary task, delegate source changes to `zdev:zdev-implementer`, then
ask a different `zdev:zdev-verifier` to check the task requirements, touched
code, and validation. Return each task-owned `REWORK` finding to an implementer
and verify the correction with a different agent. Continue until `PASS` or
`BLOCKER`. Include the rendered repository guidance in every prompt. If the
named agents are unavailable, use ordinary Claude Code subagents with the same
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

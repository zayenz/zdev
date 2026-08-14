---
name: zdev
description: Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use only when the user invokes zdev, zd, or $zdev; asks to work through an existing .zd area; or unmistakably refers to zdev's stored areas or tasks.
---

# Zdev for Oh My Pi

{{shared_contract}}

## Oh My Pi orchestration

For an ordinary task, use the built-in `task` tool to give the selected brief,
task, repository guidance, and relevant source to `zdev-implementer`. Inspect
the resulting diff, then ask a different `zdev-verifier` to check the task
requirements, touched code, and validation.

Return concrete failures to the existing implementer with `hub` when possible,
then verify the correction with a different agent. Continue until `PASS` or
`BLOCKER`. The coordinating agent runs `zd task done` and `zd commit`.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

---
name: zdev
description: Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use only when the user invokes zdev or $zdev; asks to work through an existing .zdev area; or unmistakably refers to zdev's stored areas or tasks.
---

# Zdev for Oh My Pi

{{shared_contract}}

## Oh My Pi orchestration

Use `/zdev-implement` for one complete task cycle, `/zdev-verify` for explicit
read-only task verification, and `/zdev-audit` for a read-only audit.

For an ordinary task, use the built-in `task` tool to give the selected brief,
task, repository guidance, and relevant source to `zdev-implementer`. Inspect
the resulting diff, then ask a different `zdev-verifier` to check the task
requirements, touched code, and validation.

Return concrete failures to the existing implementer with `hub` when possible,
then verify the correction with a different agent. Continue until `PASS` or
`BLOCKER`. The coordinating agent runs `zdev task done` and `zdev commit`.

For an active-zdev goal or loop request, inspect `/goal show` first. If no
unfinished goal exists, use the shared area continuation condition as the
native goal; the selected task's `native_goal` remains task context and is not
the area goal. If native continuation is unavailable, complete at most one task
and report the fresh next state. Never drop, replace, or layer work over an
unfinished Oh My Pi goal.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

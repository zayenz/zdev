---
name: zdev
description: Shape, discuss, audit, investigate, task, implement, verify, and commit durable work with zdev. Use only when the user invokes zdev, zd, or $zdev; asks to work through an existing .zd area; or unmistakably refers to zdev's stored areas or tasks.
---

# Zdev for Oh My Pi

{{shared_contract}}

## Oh My Pi orchestration

For an ordinary task, use the built-in `task` tool to delegate the selected
brief, task, repository guidance, and relevant source context to
`zdev-implementer`. Inspect the resulting uncommitted diff, then delegate
separate Spec and Standards passes to a fresh `zdev-verifier`.

Return concrete failures to the existing implementer with `hub` when possible,
then verify the corrected diff again with a fresh verifier. Repeat for every
task-owned `REWORK` until `PASS` or a real `BLOCKER`. Neither task agent can
delegate or run `zd task done` or `zd commit`.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

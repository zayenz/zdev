---
name: zdev-pi
description: Shape, discuss, audit, investigate, task, implement, verify, and commit durable work with zdev. Use only when the user invokes zdev, zd, or $zdev; asks to work through an existing .zd area; or unmistakably refers to zdev's stored areas or tasks.
---

# Zdev for Pi

{{shared_contract}}

## Pi orchestration

For an ordinary task, call `zdev_subagent` with role `implementer` and the
selected brief, task, repository guidance, and relevant source context. Inspect
the resulting uncommitted diff, then call a fresh `zdev_subagent` with role
`verifier` for separate Spec and Standards passes. Send every task-owned
`REWORK` to a fresh implementer and then a fresh verifier; repeat until `PASS`
or a real `BLOCKER`.

Use `/zdev-task` for one bounded task cycle and `/zdev-audit` for a bounded
read-only audit. Child Pi processes are ephemeral and cannot load extensions,
so they cannot delegate. Child processes do not run `zd task done` or
`zd commit`.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

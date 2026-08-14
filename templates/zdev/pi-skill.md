---
name: zdev-pi
description: Shape, discuss, audit, investigate, task, implement, verify, and commit durable work with zdev. Use only when the user invokes zdev, zd, or $zdev; asks to work through an existing .zd area; or unmistakably refers to zdev's stored areas or tasks.
---

# Zdev for Pi

{{shared_contract}}

## Pi orchestration

For an ordinary task, call `zdev_subagent` with role `implementer` and the
selected brief, task, repository guidance, and relevant source context. Inspect
the resulting diff, then call a different `zdev_subagent` with role `verifier`
to check the task requirements, touched code, and validation. Return each
task-owned `REWORK` finding to an implementer and verify the correction with a
different agent. Continue until `PASS` or `BLOCKER`.

Use `/zdev-task` for one task cycle and `/zdev-audit` for a read-only audit.
Child Pi processes cannot load extensions or delegate. The main conversation
runs `zd task done` and `zd commit`.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

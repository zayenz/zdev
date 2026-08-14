---
name: zdev-opencode
description: "Shape, discuss, audit, investigate, task, implement, verify, and commit durable work with zdev. Use only when the user invokes zdev, zd, or $zdev; asks to work through an existing .zd area; or unmistakably refers to zdev's stored areas or tasks."
compatibility: opencode
---

# Zdev for OpenCode

{{shared_contract}}

## OpenCode orchestration

Delegate one selected task to `@zdev-implementer`, then ask a different
`@zdev-verifier` to check the task requirements, touched code, and validation.
Return each task-owned `REWORK` finding to an implementer and verify the
correction with a different agent. Continue until `PASS` or `BLOCKER`. Include
the rendered repository guidance and applicable instructions in every prompt.

Use `/zdev-task` for one implementation and verification cycle. Use
`/zdev-audit` for a read-only audit. The main conversation completes tasks and
commits.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

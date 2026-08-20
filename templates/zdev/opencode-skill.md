---
name: zdev-opencode
description: "Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use only when the user invokes zdev or $zdev; asks to work through an existing .zdev area; or unmistakably refers to zdev's stored areas or tasks."
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
`/zdev-audit` for a read-only audit. The coordinating agent completes tasks and
commits.

OpenCode has no required native goal surface. Use the rendered zdev goal as an
ordinary prompt, including when a native feature was requested but is
unavailable, and state that no native continuation was started.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

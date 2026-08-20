---
name: zdev-pi
description: Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use only when the user invokes zdev or $zdev; asks to work through an existing .zdev area; or unmistakably refers to zdev's stored areas or tasks.
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

Use `/zdev-implement` for one complete task cycle, `/zdev-verify` for explicit
read-only task verification, and `/zdev-audit` for a read-only audit. Child Pi
processes cannot load extensions or delegate.

Stock Pi has no native goal surface. Use the rendered zdev goal as an ordinary
prompt, including when a native feature was requested but is unavailable, and
state that no native continuation was started.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

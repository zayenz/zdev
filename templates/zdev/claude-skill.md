---
name: zdev
description: "Shape, discuss, audit, investigate, task, implement, verify, and commit durable work with zdev. Use only when the user invokes zdev, zd, or $zdev; asks to work through an existing .zd area; or unmistakably refers to zdev's stored areas or tasks."
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

On Claude Code v2.1.154 or later, `/zdev:zdev-task` runs this task cycle and
`/zdev:zdev-audit` runs a read-only audit. The ordinary subagent loop also
works. The main conversation runs `zd task done` and `zd commit`.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

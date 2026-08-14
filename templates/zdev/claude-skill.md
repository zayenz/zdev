---
name: zdev
description: "Shape, discuss, audit, investigate, task, implement, verify, and commit durable work with zdev. Use only when the user invokes zdev, zd, or $zdev; asks to work through an existing .zd area; or unmistakably refers to zdev's stored areas or tasks."
---

# Zdev for Claude Code

{{shared_contract}}

## Claude Code orchestration

For an ordinary task, delegate source changes to
`zdev:zdev-implementer`, then use a fresh `zdev:zdev-verifier` for separate
Spec and Standards passes. Send every task-owned `REWORK` back through
implementation and then use another fresh verifier; repeat until `PASS` or a
real `BLOCKER`. Include the rendered repository guidance in every prompt. If
named agents are unavailable, use ordinary Claude Code subagents with the same
boundaries.

On Claude Code v2.1.154 or later, `/zdev:zdev-task` can structure a bounded
task cycle and `/zdev:zdev-audit` can structure a bounded read-only audit.
Dynamic workflows are optional; the ordinary subagent loop remains the
compatibility path. These agents do not run `zd task done` or `zd commit`.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

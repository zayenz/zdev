---
name: zdev-opencode
description: "Shape, discuss, audit, investigate, task, implement, verify, and commit durable work with zdev. Use only when the user invokes zdev, zd, or $zdev; asks to work through an existing .zd area; or unmistakably refers to zdev's stored areas or tasks."
compatibility: opencode
---

# Zdev for OpenCode

{{shared_contract}}

## OpenCode orchestration

Delegate one selected task to `@zdev-implementer`, then give the resulting diff
to a fresh `@zdev-verifier`. Send every task-owned `REWORK` back through
implementation and then use another fresh verifier; repeat until `PASS` or a
real `BLOCKER`. Include the rendered repository guidance and any narrower
applicable instructions in every prompt.

Use `/zdev-task` to start a bounded implementation and verification cycle.
Use `/zdev-audit` for a read-only audit that returns candidate work for human
selection. Agents and commands do not complete tasks or commit.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

---
name: zdev
description: "Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use only when the user invokes zdev or $zdev; asks to work through an existing .zdev area; or unmistakably refers to zdev's stored areas or tasks."
---

# Zdev for Codex

{{shared_contract}}

## Codex orchestration

Use one Codex collaboration agent to implement a task and a different agent to
verify it. The coordinating agent owns zdev state, user decisions, task
completion, and commits. Give each agent the rendered repository guidance and
applicable `AGENTS.md` instructions.

For longer work, use a Codex goal only when the user explicitly requests one.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

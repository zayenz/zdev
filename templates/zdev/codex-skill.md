---
name: zdev
description: "Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use only when the user invokes zdev or $zdev; asks to work through an existing .zdev area; or unmistakably refers to zdev's stored areas or tasks."
---

# Zdev for Codex

{{shared_contract}}

## Codex orchestration

Use `$zdev-implement` for one complete task cycle, `$zdev-verify` for explicit
read-only task verification, and `$zdev-audit` for a read-only audit.

Use one Codex collaboration agent to implement a task and a different agent to
verify it. The coordinating agent owns zdev state, user decisions, task
completion, and commits. Give each agent the rendered repository guidance and
applicable `AGENTS.md` instructions.

{% if implementer_has_model %}For the implementer, pass `model={{ implementer_model }}`{% if implementer_has_effort %} and
`reasoning_effort={{ implementer_effort }}`{% endif %} to the spawned agent.{% else %}Leave the implementer's model and reasoning effort unset so it inherits them.{% endif %}
{% if verifier_has_model %}For the verifier, pass `model={{ verifier_model }}`{% if verifier_has_effort %} and
`reasoning_effort={{ verifier_effort }}`{% endif %} to the spawned agent.{% else %}Leave the verifier's model and reasoning effort unset so it inherits them.{% endif %}

For an active-zdev goal or loop request, inspect `/goal` first. If no unfinished
goal exists, use the shared area continuation condition as the native goal; the
selected task's `native_goal` remains task context and is not the area goal. If
native continuation is unavailable, complete at most one task and report the
fresh next state. Never replace or layer work over an unfinished goal.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

---
name: zdev
description: "Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use only when the user invokes zdev or $zdev; asks to work through an existing .zdev area; or unmistakably refers to zdev's stored areas or tasks."
---

# Zdev for Codex

{{shared_contract}}

## Codex orchestration

Use `$zdev-audit` for the installed read-only audit workflow.

Use one Codex collaboration agent to implement a task and a different agent to
verify it. The coordinating agent owns zdev state, user decisions, task
completion, and commits. Give each agent the rendered repository guidance and
applicable `AGENTS.md` instructions.

{% if implementer_has_model %}For the implementer, pass `model={{ implementer_model }}`{% if implementer_has_effort %} and
`reasoning_effort={{ implementer_effort }}`{% endif %} to the spawned agent.{% else %}Leave the implementer's model and reasoning effort unset so it inherits them.{% endif %}
{% if verifier_has_model %}For the verifier, pass `model={{ verifier_model }}`{% if verifier_has_effort %} and
`reasoning_effort={{ verifier_effort }}`{% endif %} to the spawned agent.{% else %}Leave the verifier's model and reasoning effort unset so it inherits them.{% endif %}

Inspect `/goal` before explicit native-goal use, then apply `/goal <native_goal>`
only when no unfinished goal exists. Otherwise follow the shared ordinary-prompt
or unavailable-feature fallback.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

---
name: zdev-verify
description: "Independently verify the explicit current ready zdev task without lifecycle changes. Use when the user invokes $zdev-verify with an area and task ID."
---

# Zdev verify for Codex

{{task_workflow_contract}}

Use one fresh read-only Codex collaboration agent with the configured verifier
profile. {% if verifier_has_model %}Pass `model={{ verifier_model }}`{% if verifier_has_effort %} and
`reasoning_effort={{ verifier_effort }}`{% endif %} when spawning it.{% else %}Leave its model and reasoning effort unset so it inherits them.{% endif %}
The current Codex session performs preflight, checks the explicit task ID, and
validates the returned envelope without changing task or Git state.

{{repository_guidance}}

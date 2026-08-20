---
name: zdev-implement
description: "Implement, independently verify, complete, and commit one ready zdev task. Use when the user invokes $zdev-implement for an area."
---

# Zdev implement for Codex

{{task_workflow_contract}}

Use one fresh Codex collaboration agent with the configured implementer profile
for implementation. {% if implementer_has_model %}Pass `model={{ implementer_model }}`{% if implementer_has_effort %} and
`reasoning_effort={{ implementer_effort }}`{% endif %} when spawning it.{% else %}Leave its model and reasoning effort unset so it inherits them.{% endif %}
Use a different fresh agent for every verification verdict. {% if verifier_has_model %}Pass
`model={{ verifier_model }}`{% if verifier_has_effort %} and
`reasoning_effort={{ verifier_effort }}`{% endif %}.{% else %}Leave its model and reasoning effort unset so it inherits them.{% endif %}
The current Codex session remains the coordinator and validates every envelope.

{{repository_guidance}}

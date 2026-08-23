---
name: zdev-implement
description: "Completes one ready zdev task with independent verification and a commit. Use when the user invokes $zdev-implement, asks active zdev to implement the next task in a named area, or asks to continue one stored task."
---

# Zdev implement for Codex

{{task_workflow_contract}}

Use one fresh Codex collaboration agent with the configured role profile.
Normal work uses `implementer`. Authored routine work alone may use
`routine-implementer`; that worker may edit only the supplied task-owned
implementation paths and may collect narrow evidence, but never performs final
verification, lifecycle changes, staging, commits, or product decisions.
Advanced work and explicit advanced rework use `advanced-implementer`; read-only
planning reuses that same profile rather than defining another role.

Spawn every role with `fork_turns="none"`. Give it a compact
filesystem-backed message containing the role, exact area and task ID, baseline
commit, task-owned paths, and exact paths to the installed task contract,
applicable repository guidance, `AGENTS.md` files, area brief, slice when
present, and task. Include the opaque work-context snapshot ID when the task
contract supplies one. The agent reads those files and returns only the
contract envelope.

For `routine-implementer`, {% if routine_implementer_has_model %}pass `model={{ routine_implementer_model }}`{% if routine_implementer_has_effort %} and
`reasoning_effort={{ routine_implementer_effort }}`{% endif %} together with `fork_turns="none"`.{% else %}leave its model and reasoning effort unset so it inherits them.{% endif %}
For `implementer`, {% if implementer_has_model %}pass `model={{ implementer_model }}`{% if implementer_has_effort %} and
`reasoning_effort={{ implementer_effort }}`{% endif %} together with `fork_turns="none"`.{% else %}Leave its model and reasoning effort unset so it inherits them.{% endif %}
For `advanced-implementer`, {% if advanced_implementer_has_model %}pass `model={{ advanced_implementer_model }}`{% if advanced_implementer_has_effort %} and
`reasoning_effort={{ advanced_implementer_effort }}`{% endif %} together with `fork_turns="none"`.{% else %}leave its model and reasoning effort unset so it inherits them.{% endif %}
Use a different fresh agent for every verification verdict. {% if verifier_has_model %}Pass
`model={{ verifier_model }}`{% if verifier_has_effort %} and
`reasoning_effort={{ verifier_effort }}`{% endif %} together with `fork_turns="none"`.{% else %}Leave its model and reasoning effort unset so it inherits them.{% endif %}
The current Codex session remains the coordinator and validates every envelope.

{{repository_guidance}}

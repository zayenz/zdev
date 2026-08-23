---
name: zdev-verify
description: "Independently verifies the explicit current ready zdev task while preserving lifecycle and Git state. Use when the user invokes $zdev-verify or asks active zdev to verify a named ready task."
---

# Zdev verify for Codex

{{verify_workflow_contract}}

Use one fresh read-only Codex collaboration agent with the configured verifier
profile and set `fork_turns="none"`. Give it a compact filesystem-backed
message containing the verifier role, exact area and task ID, expected HEAD,
and exact paths to the installed verification contract, applicable repository
guidance, `AGENTS.md` files, area brief, slice when present, and task. It creates
and returns the opaque work-context snapshot required by the contract.
{% if verifier_has_model %}Pass `model={{ verifier_model }}`{% if verifier_has_effort %} and
`reasoning_effort={{ verifier_effort }}`{% endif %} together with
`fork_turns="none"` when spawning it.{% else %}Leave its model and reasoning effort unset so it inherits them.{% endif %}
The current Codex session performs preflight, checks the explicit task ID, and
validates the returned envelope without changing task or Git state.

{{repository_guidance}}

---
name: zdev-audit
description: "Run a read-only zdev codebase audit with independently checked findings. Use when the user invokes $zdev-audit or asks active zdev to audit a named boundary."
---

# Zdev audit for Codex

{{audit_contract}}

Use a fresh Codex collaboration agent as the verifier. {% if verifier_has_model %}Pass
`model={{ verifier_model }}`{% if verifier_has_effort %} and
`reasoning_effort={{ verifier_effort }}`{% endif %} when spawning it.{% else %}Leave its model and reasoning effort unset so it inherits them.{% endif %}
Give it the boundary, repository guidance, applicable `AGENTS.md` instructions,
and the audit contract above. If fan-out is warranted, use fresh verifier
agents for the lenses and a different fresh verifier for final evidence
vetting. Validate the returned first line and required body before reporting
it. Do not create tasks automatically; the user decides whether findings
become durable work.

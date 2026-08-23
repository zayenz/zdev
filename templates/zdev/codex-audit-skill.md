---
name: zdev-audit
description: "Runs a read-only zdev codebase audit with independently checked findings. Use when the user invokes $zdev-audit or asks active zdev to audit or review a named repository boundary."
---

# Zdev audit for Codex

{{audit_contract}}

Use a fresh Codex collaboration agent as the verifier and set
`fork_turns="none"`. {% if verifier_has_model %}Pass
`model={{ verifier_model }}`{% if verifier_has_effort %} and
`reasoning_effort={{ verifier_effort }}`{% endif %} together with
`fork_turns="none"` when spawning it.{% else %}Leave its model and reasoning effort unset so it inherits them.{% endif %}
Give it a compact filesystem-backed message containing its role, the exact
boundary or lens, and exact paths to the installed audit contract, repository
guidance, and applicable `AGENTS.md` files. The agent reads those files and
returns the contract's public result. With no explicit lenses, start exactly that one
verifier. With one to four explicit lenses, use fresh verifier agents for the
lenses and a different fresh verifier for final evidence vetting. Reject more
than four before starting an agent. Validate the returned first line and
required body before reporting it. Do not create tasks automatically; the user
decides whether findings become durable work.

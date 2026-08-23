+++
schema_version = 1
id = "improvements-056"
key = "trim-claude-handoffs"
area = "improvements"
status = "done"
complexity = "standard"
blocked_by = []
+++
# Trim duplicate Claude task handoffs

## Outcome

Claude task workflows stop copying redundant implementation and verification payloads between workers while retaining the evidence required for safe verification and completion.

## Context

The current Claude workflow accumulates every implementer envelope and sends overlapping raw and parsed verifier data into later prompts. Change the canonical Claude workflow template and its executable coverage before introducing filesystem-backed Git evidence, so the remaining transport boundary is explicit.

## Boundaries

- Retain only the latest accepted implementer or rework envelope as a locator for verification; do not accumulate implementationHistory.
- Do not send the implementer envelope to completion, and do not send both raw and parsed verifier representations when one suffices.
- Keep the current inline Git evidence and every fresh work-context, identity, validation, ownership, completion, and commit gate until the later snapshot task replaces only its transport.
- Change canonical templates and regenerate installed artifacts; do not add persistent state in this task.

## Done when

- [x] Claude verification receives only the latest accepted implementer or rework envelope needed to locate changes.
- [x] Claude completion receives one sufficient verifier representation and no implementation history or duplicate verifier copy.
- [x] Executable workflow coverage exercises initial pass, rework, escalation, completion, and invalid envelopes without relying on string-count-only assertions.
- [x] Round-trip documentation describes the reduced handoff accurately.

## Validation

- Run the focused Claude workflow and generated-artifact tests.
- Run the repository standard full validation.

## Result

Trimmed Claude task handoffs to the latest implementation locator and one verifier evidence representation.

Validation:

- Focused Claude workflow and generated-artifact tests passed.
- Format, clippy, all 130 tests, build, and git diff checks passed.

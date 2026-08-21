+++
schema_version = 1
id = "improvements-035"
key = "small-audit-economy"
area = "improvements"
status = "done"
blocked_by = []
+++
# Make small audits use one verifier

## Outcome

The ordinary small-boundary audit path uses one checking verifier while explicit larger audits remain bounded.

## Context

Claude currently always dispatches a reviewer and then a vetter, unlike the shared contract. Align it with the common small-audit path and bound explicit lens fan-out.

## Boundaries

- Use one fresh verifier when no explicit lenses are requested.
- Retain reviewer fan-out plus a fresh vetter only for explicitly larger audits.
- Set a small documented maximum lens count and reject excess rather than silently spawning more agents.
- Do not weaken evidence requirements or allow audit workers to mutate.

## Done when

- [x] A default Claude audit dispatches exactly one checking verifier.
- [x] Explicit multi-lens audit is bounded and still receives one independent final vetting pass.
- [x] Canonical and generated audit guidance agrees across harnesses.

## Validation

- Add focused workflow tests for default, bounded multi-lens, and excessive-lens cases.
- Regenerate and check affected artifacts.
- Run the area-wide validation from brief.md.

## Result

Made the default audit use one checking verifier and bounded explicit lens fan-out at four plus one final vetter.

Validation:

- Executable default/bounded/excessive workflow tests, all-harness generation checks, full 106-test suite, formatting, strict Clippy, build, diff check, and fresh independent verification passed.

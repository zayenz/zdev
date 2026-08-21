+++
schema_version = 1
id = "improvements-039"
key = "review-fingerprint-handoff"
area = "improvements"
status = "done"
blocked_by = []
+++
# Make the review fingerprint an opaque machine handoff

## Outcome

Exact task-bundle drift remains detected without making the fingerprint another human decision or transcription step.

## Context

The current approval value usefully binds review to unchanged canonical bundle content, but it is not authentication. Keep the check and describe it accurately as an opaque review fingerprint carried automatically by the coordinator.

## Boundaries

- The user approves the rendered bundle once and is never asked to read, copy, compare, or reason about the fingerprint.
- Preserve the existing approval JSON field and --approval compatibility.
- Do not add provenance, identities, signatures, durable approval state, or a new approval turn.
- Keep direct import behavior where current contracts allow it.

## Done when

- [x] Review and import documentation consistently call the value a review fingerprint rather than security authorization.
- [x] The coordinator passes it automatically for an unchanged reviewed bundle.
- [x] A changed bundle is still rejected when the supplied fingerprint no longer matches.

## Validation

- Use focused existing fingerprint tests for unchanged, changed, and omitted cases; do not test hash implementation details.
- Regenerate affected guidance and run the area-wide validation from brief.md.

## Result

Reframed the approval value as an opaque machine-carried review fingerprint while preserving bundle-drift rejection and direct-import compatibility.

Validation:

- Existing unchanged, changed, and omitted-fingerprint tests, generation checks, full 106-test suite, formatting, strict Clippy, build, diff check, and fresh independent verification passed.

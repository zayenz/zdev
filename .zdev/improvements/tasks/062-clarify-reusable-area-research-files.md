+++
schema_version = 1
id = "improvements-062"
key = "clarify-reusable-area-research"
area = "improvements"
status = "done"
complexity = "routine"
blocked_by = []
+++
# Clarify reusable area research files

## Outcome

Zdev guidance consistently permits stable, source-backed research to live as indexed area background files when later tasks will reuse it, without turning transcripts or standalone investigations into durable state.

## Context

Shape guidance already supports .zdev/<area>/background, while the root state rule omits it and investigation normally leaves detail in conversation. Align the state model and routes without changing lifecycle or introducing another report type.

## Boundaries

- Background files may be retained during approved area shaping or an authorized investigation task.
- Standalone investigations remain report-only unless the user asks to preserve their result.
- Every retained file is readable, stable, source-backed, indexed from the brief, and linked selectively from relevant tasks.
- The brief remains the authoritative synthesis.
- Exclude transcripts, raw tool or search dumps, repository source copies, temporary prototypes, and lifecycle metadata.

## Done when

- [x] The root state contract explicitly includes indexed area background files with their narrow retention criteria.
- [x] Shape, Investigate, Create tasks, workflow, and user guidance describe the same write authority and source-of-truth relationship.
- [x] Generated harness artifacts remain synchronized with canonical guidance.

## Validation

- Run canonical generation and documentation-contract checks.
- Run the repository standard full validation.

## Result

Zdev guidance now consistently permits narrowly retained, indexed area background research while keeping the brief authoritative and standalone investigations report-only by default.

Validation:

- Independent verification passed; generated-fixture and documentation contracts, formatting, strict clippy, all 138 tests, build, diff check, and fresh work-context comparison passed.

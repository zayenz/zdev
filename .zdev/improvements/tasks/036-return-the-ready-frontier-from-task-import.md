+++
schema_version = 1
id = "improvements-036"
key = "import-ready-frontier"
area = "improvements"
status = "done"
blocked_by = []
+++
# Return the ready frontier from task import

## Outcome

Successful import reports the complete validated ready frontier so the coordinator need not run tasks list immediately afterward.

## Context

The importer already has the hypothetical validated graph before publication. Expose its stable numeric ready frontier in the committed and uncommitted success result, and update guidance to retain post-import check but remove the redundant list call.

## Boundaries

- Do not remove the post-import check or change locking, rollback, approval, or commit behavior.
- Return the complete area frontier, including existing ready tasks and excluding newly blocked tasks.
- Preserve existing result fields and human output unless a small additive clarification is needed.

## Done when

- [x] Import JSON includes ready task IDs in stable numeric order after the published graph is validated.
- [x] The result is correct when existing tasks remain ready and newly imported tasks are blocked.
- [x] Task-creation guidance uses the returned frontier and no longer calls tasks list after successful import.

## Validation

- Add focused committed and uncommitted import tests for mixed existing/new readiness and rollback compatibility.
- Regenerate and check affected guidance artifacts.
- Run the area-wide validation from brief.md.

## Result

Added the complete validated ready frontier to committed and uncommitted import results and removed the redundant post-import list call from guidance.

Validation:

- Focused mixed-readiness and rollback tests, generation checks, full 106-test suite, formatting, strict Clippy, build, diff check, and fresh independent verification passed.

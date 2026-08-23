+++
schema_version = 1
id = "improvements-061"
key = "challenge-stored-task-reviews"
area = "improvements"
status = "done"
complexity = "routine"
blocked_by = []
+++
# Challenge task drafts through stored reviews

## Outcome

Independent task-draft review reads the exact stored review document, avoiding another complete bundle copy before the user approval checkpoint.

## Context

The current guidance challenges an in-memory bundle before zdev stores its canonical Markdown. Reorder only the guidance flow: store the candidate first, let the reviewer read the returned review.md path, and present only the final challenged artifact.

## Boundaries

- Storage and independent challenge never imply user approval.
- The reviewer reads the exact stored Markdown path and does not need access to adjacent internal bundle metadata.
- Suggested revisions update the coordinator draft and rerun review, atomically replacing the stored candidate.
- Only the final challenged stored artifact is shown for approval.
- This is canonical and generated guidance work; do not invent new runtime state or testing machinery.

## Done when

- [x] The Create tasks route stores a valid candidate before independent challenge and passes its Markdown path to the reviewer.
- [x] An unchanged candidate proceeds directly to presentation, while a revised candidate is replaced and challenged again before presentation.
- [x] Canonical references, generated harness guidance, and user documentation agree that review storage is not approval.

## Validation

- Run canonical generation and documentation-contract checks.
- Run the repository standard full validation.

## Result

Task creation now stores a candidate before independent challenge, passes the stored Markdown path to the reviewer, and presents only the final challenged artifact for approval.

Validation:

- Independent verification passed; canonical/generated consistency, documentation contracts, formatting, strict clippy, all tests, build, diff check, and fresh work-context comparison passed.

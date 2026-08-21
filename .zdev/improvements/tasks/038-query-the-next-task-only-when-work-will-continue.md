+++
schema_version = 1
id = "improvements-038"
key = "conditional-next-query"
area = "improvements"
status = "done"
blocked_by = []
+++
# Query the next task only when work will continue

## Outcome

Single-task completion stops after reporting its verified commit, while loop and explicit continuation routes refresh the next task before dispatch.

## Context

Current guidance may run zdev next after every commit even when the user requested one task. Make the query conditional so it remains load-bearing for continuation but disappears from terminal one-task flows.

## Boundaries

- Do not cache or reuse pre-commit selection.
- Loops and explicit continuation still refresh before dispatching another worker.
- Do not combine commit and next selection into one mutation command.

## Done when

- [x] Canonical one-task routes report the commit and stop without an unused next query.
- [x] Goal/loop and explicit continue routes run a fresh next/work-context boundary before another task.
- [x] All harness guidance agrees on the distinction.

## Validation

- Add focused contract assertions for terminal one-task and continuing routes.
- Regenerate affected artifacts and run the area-wide validation from brief.md.

## Result

Made one-task routes stop after the verified commit while reserving fresh post-commit work-context selection for explicit continuation and future goal/loop routes.

Validation:

- Focused terminal/continuation call-order, count, all-harness, generation, full 106-test, formatting, strict Clippy, build, diff-check, and fresh independent verification passed.

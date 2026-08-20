+++
schema_version = 1
id = "improvements-006"
key = "slice-tasks"
area = "improvements"
status = "done"
blocked_by = ["improvements-005"]
+++
# Associate tasks with slices and derive slice progress

## Outcome

Tasks can optionally belong to a slice, and every task-selection and reporting surface provides the applicable slice context and derived progress.

## Context

After lightweight slice briefs exist, extend the task bundle and durable header in `src/tasks.rs` with an optional slice key. Imports and `zdev check` must validate references without weakening transactional publication. Expose membership through task commands, generated `TASKS.md`, and area status in `src/lib.rs`. Update canonical implement and verify workflows to read the area brief first, then the referenced slice brief, then the task. Area Testing remains authoritative; slice boundaries may narrow work but cannot override area decisions.

## Boundaries

- Keep unsliced tasks and version 1 areas valid.
- Do not add stored slice lifecycle state, cross-area task dependencies, or dependencies between slices.
- Derive slice ready, blocked, and done counts from task state, including zero counts for slices with no tasks.
- Do not let a slice brief override the area brief's testing level or settled decisions.

## Done when

- [x] Reviewed bundles, imported task files, and manual task validation accept an optional slice key and reject references to missing slices before publishing state.
- [x] `zdev tasks list`, `zdev task show`, `zdev next`, and generated `TASKS.md` expose task slice membership.
- [x] `zdev task show` and `zdev next` include the slice brief path in useful human and JSON output.
- [x] Area status reports derived ready, blocked, and done counts for every slice while keeping unsliced tasks only in area totals.
- [x] Canonical implement and verify workflows load slice context in the agreed order, and checked-in integrations match generated sources.
- [x] Focused tests cover bundle review/import, invalid manual references, index rendering, status counts, zero-task slices, and selected-task context.

## Validation

- Run `cargo test --locked --test lean`.
- Run the repository's standard full validation from the area brief.

## Result

Added optional task-to-slice membership with validated references, slice-aware selection output, and derived per-slice progress.

Validation:

- Independent verification passed, including upgrade compatibility for existing unsliced task indexes.
- cargo test --locked --test lean (71 passed)
- cargo fmt --all -- --check
- cargo clippy --locked --all-targets --all-features -- -D warnings
- cargo test --locked (73 passed)
- cargo build --locked
- git diff --check

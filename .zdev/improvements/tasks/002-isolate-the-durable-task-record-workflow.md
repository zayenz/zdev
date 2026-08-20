+++
schema_version = 1
id = "improvements-002"
key = "tasks-module"
area = "improvements"
status = "done"
blocked_by = []
+++
# Isolate the durable task-record workflow

## Outcome

Task bundle review and import, task-file parsing and validation, dependency selection, completion, and index generation can be understood and changed within one internal module.

## Context

The task models and workflow currently occupy a large central region of `src/lib.rs`. Existing black-box tests in `tests/lean.rs` cover bundle review and approval, transactional import, dependency validation, selection, completion, reopening, and generated indexes.

## Boundaries

- Move task-specific models and operations into `src/tasks.rs`: bundle validation and rendering, import, task parsing, graph validation, list, show, next selection, completion, reopening, and index rendering.
- Leave project configuration, area branch lifecycle, stable change IDs, and genuinely shared Git or filesystem primitives outside the task module.
- Keep cross-domain status and check orchestration in the shell if moving it would create circular ownership between task and area modules.
- Preserve task formats, ordering, errors, text and JSON output, locking, rollback, and generated `TASKS.md` behavior.
- Add no tests that merely assert the new module boundary.

## Done when

- [x] The durable task-record lifecycle has one clear home in `src/tasks.rs`.
- [x] Its interface to area and project state and shared repository operations is narrow and explicit.
- [x] Existing task behavior and file formats remain unchanged.
- [x] Existing black-box task tests pass without coverage expansion for the move itself.

## Validation

- Run `cargo test --locked --test lean`.
- Run the full validation set in `brief.md`.

## Result

Isolated the durable task-record workflow in a cohesive internal module and independently verified behavioral preservation.

Validation:

- Focused lean integration suite passed: 65 tests.
- Full repository validation passed: format, Clippy, 66 tests, build, and diff check.

+++
schema_version = 1
id = "improvements-003"
key = "areas-module"
area = "improvements"
status = "done"
blocked_by = []
+++
# Isolate project and area lifecycle behavior

## Outcome

Repository-record configuration and area branch and rebase lifecycle can be changed without navigating task or harness implementation.

## Context

Configuration, initialization, cleanup, area metadata, branch relationships, managed rebase, and branch-health logic currently span large regions of `src/lib.rs`. Existing black-box tests in `tests/lean.rs` cover record policies, cleanup, branch binding, parents, status diagnostics, and successful, conflicting, and rejected rebases.

## Boundaries

- Move project configuration, record initialization and cleanup, area metadata, branch binding and parent relationships, base anchors, managed rebase, and branch-health calculation into one cohesive internal module.
- Leave task-file behavior, harness integrations, stable change-ID commands, and generic helpers used by several domains outside it.
- Preserve all on-disk schemas, Git operations, recovery guidance, text and JSON output, and CLI behavior.
- Keep cross-domain status and check rendering in the shell where appropriate.
- Add no tests solely for moved code.

## Done when

- [x] Project and area lifecycle logic has one clear internal home.
- [x] Task and harness modules consume only the project and area facts or operations they need.
- [x] Existing branch, rebase, initialization, cleanup, status, and validation behavior remains unchanged.
- [x] Existing black-box coverage passes unchanged.

## Validation

- Run `cargo test --locked --test lean`.
- Run the full validation set in `brief.md`.

## Result

Isolated project and area lifecycle behavior in a cohesive internal module and independently verified behavioral preservation.

Validation:

- Focused lean integration suite passed: 65 tests.
- Full repository validation passed: format, Clippy, 66 tests, build, and diff check.

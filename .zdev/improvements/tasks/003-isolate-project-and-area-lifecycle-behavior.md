+++
schema_version = 1
id = "improvements-003"
key = "areas-module"
area = "improvements"
status = "open"
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

- [ ] Project and area lifecycle logic has one clear internal home.
- [ ] Task and harness modules consume only the project and area facts or operations they need.
- [ ] Existing branch, rebase, initialization, cleanup, status, and validation behavior remains unchanged.
- [ ] Existing black-box coverage passes unchanged.

## Validation

- Run `cargo test --locked --test lean`.
- Run the full validation set in `brief.md`.

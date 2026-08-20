+++
schema_version = 1
id = "improvements-017"
key = "deterministic-goal-command"
area = "improvements"
status = "done"
blocked_by = []
+++
# Add the deterministic area goal command

## Outcome

zdev goal <area> returns a stable read-only projection of the next ready task and teaches every installed harness how to use it as ordinary context or an explicitly requested native goal.

## Context

Implement docs/harness-goals.md. Add a small goal projection module routed from src/lib.rs. Reuse narrow task and slice read views from src/tasks.rs and src/project.rs rather than parsing Markdown or dependencies again. Update shared canonical integration guidance with the settled native-goal conflict and fallback rules, then regenerate checked-in integrations.

## Boundaries

- Read only existing area, slice, and task records; add no goal storage or lifecycle.
- Do not enforce task-work branch gates, inspect Git, acquire a write lock, repair indexes, or call a harness.
- Keep ready, empty, and complete as the only successful states; malformed dependency graphs remain validation errors.
- Native goal application is harness guidance, not a binary API.
- Do not duplicate the existing task parser, graph validator, readiness rule, or numeric ordering.

## Done when

- [x] Human and JSON output match the documented fields, ordering, omissions, paths, counts, fixed native condition, and final newline for ready, empty, and complete states.
- [x] Goal selection is identical to ordinary next-task ordering for the same valid graph without requiring the area branch.
- [x] Sliced and unsliced tasks expose exactly their recorded objective, context, boundaries, proof conditions, and validation.
- [x] Repeated runs over unchanged records are byte-identical.
- [x] Malformed records, missing slices, missing blockers, and cycles use the existing error envelope and leave files and Git unchanged.
- [x] All five realized integrations describe ordinary-prompt use, explicit native-goal intent, unfinished-goal precedence, and unavailable-feature fallback consistently.

## Validation

- Run focused black-box tests for sliced ready output, unsliced omission, empty, complete, deterministic reruns, and one non-mutating malformed graph.
- Run cargo test --locked --test lean.
- Run the repository's standard full validation from the area brief.

## Result

Added deterministic read-only area goal projection with exact ready, empty, and complete outputs plus consistent native-goal guidance for every harness.

Validation:

- Independent verification confirmed exact byte contracts, shared task ordering and validation, off-branch behavior, deterministic generation, and non-mutation.
- cargo test --locked --test lean (81 passed)
- cargo fmt --all -- --check
- cargo clippy --locked --all-targets --all-features -- -D warnings
- cargo test --locked
- cargo build --locked
- all-harness install/check release smoke
- cargo package --locked --allow-dirty
- git diff --check

+++
schema_version = 1
id = "improvements-008"
key = "project-wide-next"
area = "improvements"
status = "done"
blocked_by = ["improvements-004"]
+++
# Select any ready task across areas explicitly

## Outcome

An explicit `zdev next --any` request deterministically selects safe ready work across the project and reports the owning area and branch without changing the checkout.

## Context

Bare `zdev next` currently uses an explicit area, `default_area`, or an unambiguous open area. Add an opt-in project-wide mode in `src/tasks.rs` and the CLI routing in `src/lib.rs`, reusing the stale-safe branch classification introduced by the advisory-base task. Update canonical skills so only explicit requests to work on any ready or unblocked task use this mode. Preserve the existing area-specific behavior.

## Boundaries

- `--any` conflicts with positional AREA and ignores `default_area`.
- Never switch branches, rebase, mutate task state, or hide excluded areas.
- Prefer safe ready tasks whose area branch is checked out, then order remaining safe candidates by area tag and existing numeric task order.
- An off-branch task may be selected, but output must report its branch and that the branch does not match.
- Exclude structurally unsafe areas using the shared branch classification rather than duplicating safety rules.

## Done when

- [x] `zdev next --any` returns the selected task, area, branch, branch-match flag, and task path in stable human and JSON output.
- [x] JSON includes a structured skipped list with area diagnostics, and human output names a required branch and summarizes unsafe skipped areas without silently discarding them.
- [x] When no task is selectable, the command distinguishes complete work from unsafe work with observable diagnostics; malformed or cyclic dependency graphs fail validation before selection.
- [x] Existing `zdev next [area]` and bare-next default-area behavior remain unchanged.
- [x] Canonical skills route only explicit any-ready or any-unblocked intent through `--any`, and checked-in integrations match generated sources.
- [x] Focused multi-area tests cover candidate order, off-branch selection, skipped unsafe areas, default-area bypass, option conflict, and no-candidate results.

## Validation

- Run `cargo test --locked --test lean`.
- Run the repository's standard full validation from the area brief.

## Result

Added explicit deterministic project-wide ready-task selection with branch requirements and visible unsafe-area diagnostics, without checkout mutation.

Validation:

- Independent verification passed after adding explicit off-branch human output and removing an impossible blocked result; malformed dependency graphs remain validation errors.
- cargo test --locked --test lean (73 passed)
- cargo fmt --all -- --check
- cargo clippy --locked --all-targets --all-features -- -D warnings
- cargo test --locked (75 passed)
- cargo build --locked
- git diff --check

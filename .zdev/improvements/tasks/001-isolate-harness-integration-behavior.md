+++
schema_version = 1
id = "improvements-001"
key = "integrations-module"
area = "improvements"
status = "open"
blocked_by = []
+++
# Isolate harness integration behavior

## Outcome

Changes to supported harness inventories, destinations, guidance, installation, or checks can be made in one internal module without navigating task and area implementation.

## Context

Harness code is currently split between embedded template and inventory construction near the top of `src/lib.rs` and installation, checking, destination, and guidance behavior later in the file. Existing black-box tests in `tests/lean.rs` cover inventories, destinations, publication, replacement, guidance, and generated-template consistency.

## Boundaries

- Move the existing harness integration domain into `src/integrations.rs`, including embedded templates, domain types, inventory construction, destination resolution, guidance inspection, installation, and checking.
- Preserve all CLI arguments, help, text output, JSON output, destination rules, and generated artifact behavior.
- Do not change source templates or checked-in generated integrations, and do not add abstraction layers merely to support the move.
- Keep generic filesystem and repository helpers shared when they are genuinely used outside this domain.

## Done when

- [ ] Harness integration implementation has one clear home in `src/integrations.rs`.
- [ ] The CLI shell reaches harness behavior through a small command-level interface rather than integration internals spread through `src/lib.rs`.
- [ ] Existing harness and guidance behavior remains unchanged.
- [ ] No tests are added solely because code moved.

## Validation

- Run `cargo test --locked --test lean`.
- Run the full validation set in `brief.md`.

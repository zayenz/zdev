+++
schema_version = 1
id = "improvements-019"
key = "layered-config-mutation"
area = "improvements"
status = "done"
blocked_by = ["improvements-018"]
+++
# Add atomic layered config set and unset

## Outcome

Users can safely set or remove supported project and worker values at the correct scope while preserving valid files and deterministic fallback.

## Context

Complete docs/config-command.md on top of layered-config-inspection. Add set and unset routing in src/lib.rs and focused mutation functions in the configuration module. Reuse project validators, the project state lock, atomic writers, worker-profile serialization, and the effective resolver.

## Boundaries

- One command changes one file; add no migration, cross-file transaction, editor, arbitrary TOML API, or comment-preserving serializer.
- Default writes are local; global writes are limited to worker profiles and may run outside a repository.
- Project name and record policy remain read-only and point users to init.
- Worker profiles are written atomically as inherit or a complete model-effort pair.
- Create only the resolved global zdev directory and workers lock; do not inspect or store harness credentials.

## Done when

- [x] Set and unset enforce the exact scalar and worker value grammar, scope restrictions, read-only keys, and missing-value errors.
- [x] Project trunk, default-area, and guidance reuse their established branch, area, and safe-path validation.
- [x] Global mutations serialize through workers.lock and local mutations through the existing zdev state lock.
- [x] Publication is atomic, deterministic, and preserves previous bytes on validation, staging, or replacement failure.
- [x] Unsetting exposes the next layer and returns the documented effective value and origin.
- [x] Removing the final worker profile leaves only schema_version = 1, and config trunk remains behaviorally compatible.

## Validation

- Run focused black-box tests for one local project mutation, one global profile mutation, one unset fallback, one lock or publication failure with preserved bytes, and unchanged config trunk behavior.
- Run cargo test --locked --test lean.
- Run the repository's standard full validation from the area brief.

## Result

Added typed atomic config set and unset with scoped validation, layered fallback reporting, and preserved project and worker files on failure.

Validation:

- Independent verification confirmed exact value and scope rules, local/global locking, deterministic one-file publication, fallback origins, final file shape, and real failure preservation.
- cargo test --locked --test lean (86 passed)
- cargo fmt --all -- --check
- cargo clippy --locked --all-targets --all-features -- -D warnings
- cargo test --locked
- cargo build --locked
- git diff --check

+++
schema_version = 1
id = "delivery-001"
proposal_key = "baseline"
proposal_hash = "sha256:8f74e8224582ba9f6997a63e79951cd852f77c1469d623f1e8ee26be52cfec38"
area = "delivery"
kind = "implementation"
interaction = "afk"
priority = "high"
status = "open"
blocked_by = []
+++
# Freeze the baseline

## Source

The final Python v1 command contract.

## What to build

Preserve the initialized repository and issue bytes used at the Rust cutover.

## Non-goals

- Preserving runtime locks or transaction journals.

## Acceptance criteria

- [ ] Rust reads the frozen Python repository without migration.

## Validation

- cargo test --test rust_python_compatibility

+++
schema_version = 1
id = "delivery-002"
proposal_key = "verify-cutover"
proposal_hash = "sha256:0d43d46259c64502ce1a419110556a09e0fe2b81c153e36e6c001ca75010c29b"
area = "delivery"
kind = "verification"
interaction = "hitl"
priority = "normal"
status = "open"
blocked_by = ["delivery-001"]
+++
# Verify the cutover

## Source

The final Python v1 command contract.

## What to build

Confirm that Rust creates the same stable public repository bytes.

## Non-goals

- Comparing implementation-private runtime state.

## Acceptance criteria

- [ ] Rust-created public files match the Python v1 fixture byte for byte.

## Validation

- cargo test --test rust_python_compatibility

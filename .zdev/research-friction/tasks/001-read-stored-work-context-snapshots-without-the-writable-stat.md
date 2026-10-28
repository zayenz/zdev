+++
schema_version = 1
id = "research-friction-001"
key = "snapshot-show-read-only"
area = "research-friction"
status = "done"
complexity = "routine"
afk = true
priority = "high"
blocked_by = []
+++
# Read stored work-context snapshots without the writable state lock

## Outcome

`zdev work-context <area> --show <snapshot>` and the stored-snapshot read inside `--compare` work when the zdev state lock cannot be opened for writing. Missing, corrupt and cross-area snapshots still fail closed.

## Context

Brief item 1; feedback section 2, last paragraph ([source report](../../_notes/autoresearch-field-feedback-2026-10-17.md)). In `src/lib.rs`, `show_work_context` (~2141) calls `ZdevStateLock::acquire` before `read_work_context_snapshot`. `ZdevStateLock::acquire` (~1311) opens `git rev-parse --git-path zdev-state.lock` with read, write and create. A sandbox that denies writes under `.git` therefore fails a pure read, and in the campaign it forced a permission escalation. `compare_work_context` (~2155) takes the same lock only around its stored read.

The lock is not needed for these reads. Snapshots are content-addressed and immutable. `store_work_context` publishes them under the lock with `write_atomic` (temp file plus rename) and never rewrites or prunes them, so a reader sees either complete bytes or absence. `read_work_context_snapshot` already validates the ID, canonical bytes, JSON shape and area.

Relevant tests in `tests/lean.rs`: `work_context_snapshots_round_trip_exact_json_and_compare_fresh_state`, `work_context_snapshots_fail_closed_and_remain_available`, and `state_lock_recovers_a_stale_file_without_stealing_a_live_lock` (shows the lock path). To make the lock unopenable without depending on file permissions or a non-root user, create a directory at the lock's Git path.

## Boundaries

- Keep the state lock for `--store` and for every command that writes zdev state. Do not change `ZdevStateLock` behavior for other callers.
- Keep every existing fail-closed validation and its error text. Do not change the snapshot format or ID scheme.
- Leave research callers in `src/research/experiment.rs` that read snapshots while already holding the lock unchanged.

## Done when

- [x] When the state lock path cannot be opened for writing, `work-context <area> --show <snapshot>` succeeds and prints the exact stored JSON.
- [x] In the same state, `work-context <area> --compare <snapshot>` reports equality without acquiring the lock, and `--store` still fails because writes still need the lock.
- [x] Missing, corrupt and cross-area snapshots still fail with their existing errors.
- [x] One focused regression in `tests/lean.rs` covers the unopenable-lock read.

## Validation

- cargo test --locked --test lean work_context_show_reads_without_writable_state_lock (or the chosen name of the new focused test)
- cargo fmt --all -- --check
- cargo clippy --locked --all-targets --all-features -- -D warnings
- cargo test --locked
- cargo build --locked
- git diff --check

## Result

work-context --show and the stored-snapshot read in --compare no longer take the writable state lock; immutable, atomically published snapshots are read directly, so read-only inspection works when the lock cannot be opened. --store and every other lock user are unchanged. Independently verified; the new test fails on the baseline.

Validation:

- cargo fmt --all -- --check: pass
- cargo clippy --locked --all-targets --all-features -- -D warnings: pass
- cargo test --locked: pass (lean 202)
- cargo build --locked: pass
- git diff --check: pass

+++
schema_version = 1
id = "general-029"
key = "deflake-research-interruption-tests"
area = "general"
status = "done"
complexity = "standard"
afk = true
priority = "high"
blocked_by = []
+++
# Synchronize the research interruption tests on readiness instead of elapsed time

## Outcome

isolated_candidates_preserve_capacity_pause_and_recover_without_relaunch and local_experiment_interruption_keeps_cost_and_never_relaunches_unresolved_measurement pass reliably under load, because they wait on explicit readiness and process-group exit rather than fixed time windows. The behavior they check is unchanged.

## Context

On 2026-10-28 both tests failed twice in full `cargo test --locked` runs (once while two suites overlapped, once in a standalone run) and passed when rerun alone. A GPT-6 Astra review during research-friction-007 found them timing-flaky by construction (tests/lean.rs around lines 16524 and 19696): they impose 10-second startup and 5-second cleanup deadlines and rely on a fixed 30-second evaluator sleep; a published running_pid does not mean the evaluator has finished initializing; and the local test waits for the leader process rather than the whole process group before final recovery. These assumptions predate 2.1. A flaky suite blocks the 2.1 release checks.

## Boundaries

- Change the tests and their fixtures/evaluator scripts; change product code only if the review shows a real race in zdev itself, and then keep the change minimal and explain it.
- Keep every assertion about cost, recovery, relaunch refusal and capacity; do not weaken them or add retries that hide failures.

## Done when

- [x] Each test waits on an explicit readiness signal written by the evaluator (for example a marker file) before interrupting, and on the whole process group exiting before final recovery, instead of fixed sleeps or short deadlines.
- [x] Both tests pass in 5 consecutive full `cargo test --locked` runs, and in a run with the lean suite under parallel load (for example two concurrent runs of the two tests in separate target dirs).

## Validation

- Report the repeated-run results.
- cargo fmt --all -- --check; cargo clippy --locked --all-targets --all-features -- -D warnings; cargo test --locked; cargo build --locked; git diff --check

## Result

The two research interruption tests now hold the evaluator with a ready marker and hold file, wait until the marker, evaluation run and running_pid are all published before interrupting, and wait for the whole evaluator process group to exit before recovery. Fixed sleeps and short deadlines are replaced by one 120-second hang guard; every cost, recovery, relaunch and capacity assertion is unchanged and recovery retries were removed. Passed 5 consecutive full runs and two concurrent suites. Verified by GPT-6 Astra (high).

Validation:

- cargo fmt --all -- --check: pass
- cargo clippy --locked --all-targets --all-features -- -D warnings: pass
- cargo test --locked: pass x5 consecutive (lean 208)
- target tests under two concurrent suites: pass
- git diff --check: pass

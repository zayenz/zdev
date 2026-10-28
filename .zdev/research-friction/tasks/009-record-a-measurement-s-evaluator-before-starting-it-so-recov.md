+++
schema_version = 1
id = "research-friction-009"
key = "record-evaluator-before-spawn"
area = "research-friction"
status = "open"
complexity = "advanced"
afk = true
priority = "high"
blocked_by = []
+++
# Record a measurement's evaluator before starting it so recovery never loses it

## Outcome

If the zdev process that started an evaluator dies at any point after the evaluator process group begins, recovery can still identify that group and will not treat the measurement as stopped, restore files, or release capacity while any member of the group is alive.

## Context

Found during general-029 and confirmed by a GPT-6 Astra review (2026-10-28) as a real, high-severity crash-recovery race that predates 2.1. `evaluate_with_started` in src/research.rs (around line 955) spawns the evaluator process group, and only afterwards does the callback in src/research/experiment.rs `run_measured_command` (around line 1191) persist `running_pid` under the state lock. If the owning zdev process dies in that window, recovery (`recover_measurement`, around line 1366) sees a dead owner and no running_pid and concludes the measurement stopped. Recovery itself does not restore files immediately, but a later discard or finalization can restore them while the orphaned evaluator group still runs, and capacity can be released early. The maintainer decided to fix this before releasing 2.1.

The fix needs a durable record that exists before the spawn can produce a live process, plus a way for recovery to find the group without the pid. One approach for the planner to evaluate: persist a pending-measurement marker (with a unique token) before spawning, start the evaluator so that its process group can be found by that token (for example an environment variable or a process-group recorded through a pre-exec handshake), and have recovery refuse to conclude 'stopped' while a pending marker has no confirmed-dead group. The planner chooses the mechanism; it must work on macOS and Linux with the existing ps-based process-group checks. The general-029 tests (await_held_evaluator, await_process_group_exit in tests/lean.rs) show how to hold and observe an evaluator group.

## Boundaries

- Never weaken existing recovery guarantees: a confirmed-stopped measurement still recovers as today, and a live group still blocks restoration and capacity release.
- Old checkpoints without the new marker keep their current recovery behavior.
- No change to measurement results, accounting or settlement other than refusing premature release.

## Done when

- [ ] A durable record identifying the measurement is written before the evaluator can run, and recovery uses it to find the evaluator group when running_pid was never saved.
- [ ] If the owner dies between spawn and running_pid publication, recovery reports the measurement as still running (or unresolved) while any group member lives, and discard/finalize refuse to restore files until the group is gone.
- [ ] Once the orphaned group exits, recovery completes normally with costs and the attempt preserved.

## Validation

- Add one focused test in tests/lean.rs that kills the owner inside the window (for example via a test-only hook or by holding the evaluator before running_pid is saved) and checks recovery, discard refusal and final recovery; reuse the general-029 helpers. No live harness.
- cargo fmt --all -- --check; cargo clippy --locked --all-targets --all-features -- -D warnings; cargo test --locked; cargo build --locked; git diff --check

+++
schema_version = 1
id = "auto-003"
key = "local-experiment"
area = "auto"
status = "done"
complexity = "advanced"
afk = true
priority = "normal"
blocked_by = ["auto-001", "auto-002"]
+++
# Complete one local hypothesis safely

## Outcome

One hypothesis can be evaluated and finalized on the current area branch while preserving the accepted implementation, candidate code, and experiment evidence.

## Context

Read [Experiment mechanics](../background/experiment-mechanics.md), the settled Completion and verification rules, and the new measurement/result contract. Start with work-context/snapshot, task completion, attributable staging, and commit paths in src/lib.rs and src/tasks.rs plus templates/zdev/task-workflows.md. Reuse existing Git ownership checks.

## Boundaries

- Record observations before rollback; retain rejected code in accessible Git history or a retained reference.
- Preserve unrelated staged, unstaged, and untracked work; stop safely when ownership cannot be established.
- Require a fresh independent verifier only for candidate code proposed for acceptance; ordinary tasks keep their existing verification.

## Done when

- [x] A public one-experiment path records the hypothesis and starting accepted revision before work, evaluates it with correctness checks, and finalizes keep or discard.
- [x] Keep requires independent code verification and retained accepted code; discard preserves candidate code and task evidence while restoring accepted code.
- [x] Exhausted allotted attempts can finish as failed-to-run or inconclusive with evidence; interruption leaves the task unfinished and recoverable.
- [x] Rejected completion does not satisfy a later task's requirement for retained code; that work is admitted only after acceptance.
- [x] Recovery distinguishes measurement, retention, restore, completion, and commit checkpoints without duplicate completion or data loss.

## Validation

- Add public CLI/Git scenarios for keep, discard, terminal failure, interrupted recovery, and unrelated local edits. Use deterministic measurements and verifier stand-ins; run matching Rust checks and a fixture walkthrough.

## Result

Implemented a managed local hypothesis lifecycle with actual correctness and evaluation, retained Git candidates, snapshot-bound keep verification, safe owned-path discard, recoverable completion and commits, and verified retained-code prerequisites.

Validation:

- Fresh independent verifier PASS for the whole task against unchanged W58e40a50e9c299aa.
- cargo fmt --all -- --check; cargo clippy --locked --all-targets --all-features -- -D warnings; cargo test --locked (172 integration and 3 unit); cargo build --locked; git diff --check passed.
- Public CLI/Git scenarios and disposable walkthrough cover keep/discard, exhausted correctness and evaluator failure, interruption, live descendants, precision, partial finalization recovery, owned overlap refusal and unrelated local work; area and five generated harness fixture checks passed.

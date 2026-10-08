+++
schema_version = 1
id = "auto-003"
key = "local-experiment"
area = "auto"
status = "open"
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

- [ ] A public one-experiment path records the hypothesis and starting accepted revision before work, evaluates it with correctness checks, and finalizes keep or discard.
- [ ] Keep requires independent code verification and retained accepted code; discard preserves candidate code and task evidence while restoring accepted code.
- [ ] Exhausted allotted attempts can finish as failed-to-run or inconclusive with evidence; interruption leaves the task unfinished and recoverable.
- [ ] Rejected completion does not satisfy a later task's requirement for retained code; that work is admitted only after acceptance.
- [ ] Recovery distinguishes measurement, retention, restore, completion, and commit checkpoints without duplicate completion or data loss.

## Validation

- Add public CLI/Git scenarios for keep, discard, terminal failure, interrupted recovery, and unrelated local edits. Use deterministic measurements and verifier stand-ins; run matching Rust checks and a fixture walkthrough.

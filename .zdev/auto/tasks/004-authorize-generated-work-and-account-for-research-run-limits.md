+++
schema_version = 1
id = "auto-004"
key = "research-run"
area = "auto"
status = "done"
complexity = "advanced"
afk = true
priority = "normal"
blocked_by = ["auto-001"]
+++
# Authorize generated work and account for research run limits

## Outcome

A coordinator can start and recover a research run, admit in-scope supporting work and hypotheses, and enforce explicit limits without per-task approval.

## Context

Existing derived proposals in src/tasks.rs are source-task handoffs with restricted purposes, and src/goal.rs treats empty queues as no work. Read the brief's Runs and generated work rules. Add a research-specific admission/accounting path using the research-records contract, without changing ordinary task-bundle approval.

## Boundaries

- Finite runs by default; unlimited continuation requires explicit authorization and still respects any target or supplied resource budget.
- The harness judges proposal relevance; the binary validates deterministic admission and accounting. Keep retained run state small.
- Evaluator/workload changes require rebaselining; no generated task may silently redefine the metric or loosen correctness.

## Done when

- [x] Public commands establish explicit run scope and counted limits, and admit stored hypotheses and ordinary supporting tasks under that run's authorization.
- [x] Budget accounting includes repair attempts, reruns, confirmation, and reconciliation; recovery cannot reset consumed budget or duplicate an admission.
- [x] Stop state distinguishes target, budget, interruption, and blocker; exhausted queues allow further in-scope proposals and leave the area open.
- [x] Shared environment failures stop affected dispatch; evaluation changes invalidate affected comparisons until a new baseline exists.
- [x] Fresh context exposes accepted baseline, unfinished work, useful results, and remaining limits to a new coordinator.

## Validation

- Add focused command sequences for generated mixed work, finite/unlimited limits, resume accounting, evaluator changes, and shared failure. Check ordinary approval behavior remains intact; run matching Rust checks and document the public run contract.

## Result

Implemented authorized research runs with recoverable mixed-work admission, counted limits, protected comparison policy, actual rebaselining, and fresh recovery context.

Validation:

- Independent verifier PASS at W019a9f00e5d6e8be; reviewed every done condition and complete owned delta.
- cargo fmt --check, cargo clippy --all-targets --all-features -- -D warnings, cargo test, cargo build, and git diff --check passed; 3 unit and 177 integration tests.
- All five read-only generated skill fixture checks passed with an empty temporary XDG configuration.

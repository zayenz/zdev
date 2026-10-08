+++
schema_version = 1
id = "auto-001"
key = "research-records"
area = "auto"
status = "done"
complexity = "advanced"
afk = true
priority = "normal"
blocked_by = []
+++
# Declare research areas and record hypothesis evidence

## Outcome

Users can opt an area into research, create hypothesis tasks beside ordinary tasks, and retain validated observations through public CLI operations.

## Context

Start with AreaMetadata in src/project.rs, TaskHeader/TaskDraft and task completion in src/tasks.rs, projections in src/goal.rs and src/lib.rs, and command definitions/dispatch in src/lib.rs. These strict formats currently have no research fields. Read the brief's Model and records decisions and [ZDev implementation fit](../background/zdev-fit-and-plan.md). Establish one documented public settings/result contract for later tasks.

## Boundaries

- Keep ordinary areas and existing open/done task semantics compatible; do not add a separate experiment ledger.
- Choose minimal serialization; hypothesis ancestry, work dependencies, and starting revision must remain distinct.

## Done when

- [x] A documented CLI path creates and inspects research settings and hypothesis tasks in a mixed queue; text, JSON, and work-context identify them.
- [x] Task-owned results preserve repeated observations, comparison conditions, decision, and lesson; malformed results and missing required completion evidence are rejected.
- [x] The public contract defines baseline/evaluator identity, acceptance rules, budgets, and terminal versus interrupted evidence, with no duplicate source of executable settings.
- [x] Ordinary tasks retain their existing completion rules; research routing exposes the correct contract without claiming unfinished execution support.

## Validation

- Add focused black-box cases in tests/lean.rs for a mixed area, result round-trip and invalid completion; run matching Rust checks from CONTRIBUTING.md. Document the contract in repository docs and canonical skill templates.

## Result

Added optional research settings, mixed hypothesis tasks, appended validated evidence, completion gates, public projections and documented execution guards. Independently verified every task condition.

Validation:

- Independent verifier passed at unchanged snapshot Wfd5ec8623261c4ed.
- cargo test --locked: 165 black-box tests and 3 unit tests passed, including deterministic fixture matching and focused research scenarios.
- cargo fmt, strict clippy, cargo build, git diff --check, and zdev check auto passed.

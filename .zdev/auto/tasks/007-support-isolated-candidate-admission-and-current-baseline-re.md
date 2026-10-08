+++
schema_version = 1
id = "auto-007"
key = "parallel-candidates"
area = "auto"
status = "done"
complexity = "advanced"
afk = true
priority = "normal"
blocked_by = ["auto-003", "auto-004"]
+++
# Support isolated candidate admission and current-baseline reconciliation

## Outcome

A coordinator can prepare a hypothesis in an isolated worktree and safely accept or reject its returned candidate against the current accepted revision.

## Context

Read [Experiment mechanics](../background/experiment-mechanics.md) and the brief's Parallel execution rules. Existing assigned-source-worktree, snapshot, and no-commit integration contracts in templates/zdev/task-workflows.md and src/lib.rs are starting points. Extend deterministic support only where hypothesis retention, resource admission, or reconciliation differs.

## Boundaries

- One authoritative coordinator owns records and acceptance; source workers do not edit the authoritative research records.
- Worker capacity and constrained measurement capacity are separate limits; no general scheduler.
- Retain ambiguous or unfinished work until workers are confirmed stopped.

## Done when

- [x] Public candidate context captures task, accepted revision at dispatch, isolated source location, and required measurements using the established run contract.
- [x] Return and integration checks detect baseline drift, require reconciliation and fresh measurement before keep, and count this work against the run limits.
- [x] Serialized acceptance retains rejected candidate code/evidence, preserves unrelated destination work, and pauses affected candidates when evaluation conditions change.
- [x] Recovery identifies active, returned, accepted, rejected, and ambiguous candidates without duplicate dispatch or premature cleanup.

## Validation

- Add focused Git/CLI scenarios with two candidates, a baseline change, a measurement-capacity limit, evaluation drift, and interrupted recovery. Reuse local lifecycle fixtures rather than duplicate their coverage; run matching checks and document the candidate contract.

## Result

Added isolated hypothesis admission, stopped-worker return, counted current-baseline reconciliation and source measurement, serialized destination integration, and recovery; exclusive source ownership prevents concurrent candidates sharing a worktree or branch.

Validation:

- Independent whole-task review of the repaired candidate against its unchanged stored work-context snapshot.
- cargo fmt --all -- --check; cargo clippy --locked --all-targets --all-features -- -D warnings; cargo test --locked (182 integration and 3 unit tests); cargo build --locked; git diff --check.
- Both public isolated-candidate scenarios passed, including duplicate-source refusal, capacity separation, baseline drift and remeasurement, Git transport, evaluation drift, recovery, and historical rejection.
- All five generated integrations passed explicit read-only skill checks using fresh empty XDG configuration roots. No Claude Code CLI or provider execution.

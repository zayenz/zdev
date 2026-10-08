+++
schema_version = 1
id = "auto-008"
key = "codex-rolling"
area = "auto"
status = "done"
complexity = "advanced"
afk = true
priority = "normal"
blocked_by = ["auto-005", "auto-007"]
+++
# Continuously refill Codex research workers

## Outcome

Codex can run a rolling research campaign in isolated worktrees while processing acceptance serially.

## Context

Read the public candidate contract and Codex serial route. Start with templates/zdev/references/parallel.md, codex-skill.md, recovery.md and task-workflows.md. Ordinary Parallel is an explicitly finite task batch; add the distinct research route required by the brief.

## Boundaries

- Rolling continuation is authorized by the research run, without broadening ordinary finite Parallel authorization.
- Count planners, implementers, and acceptance verifiers against available harness role capacity; measurement slots remain separately constrained.

## Done when

- [x] Free worker capacity is refilled with stored in-scope work while other candidates remain active; dispatch uses the accepted revision current at that moment.
- [x] Returning candidates enter one acceptance lane, with stale candidates reconciled and remeasured before keep.
- [x] Budget, interruption, loss of capacity, and shared failure stop affected dispatch and retain unfinished work; resume rechecks current state.
- [x] Preparation can overlap while measurements use a single constrained slot, and installed docs/fixtures explain limits and cleanup behavior.

## Validation

- Use deterministic orchestration scenarios to prove refill before all siblings finish, changed-baseline acceptance, serialized measurement, and cancellation/recovery. Regenerate fixtures and run matching checks; reserve live Codex execution for live acceptance.

## Result

Added continuously refilled Codex research workers, counted role claims, serialized acceptance and recoverable integrated rework.

Validation:

- Independent whole-task PASS on Wa9dfa67bce859317; formatting, strict Clippy, 184 integration tests and 3 unit tests, build, diff check, and all five isolated read-only fixture checks passed. Live native acceptance remains in auto-010.

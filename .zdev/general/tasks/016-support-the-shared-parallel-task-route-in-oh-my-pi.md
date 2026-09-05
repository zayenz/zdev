+++
schema_version = 1
id = "general-016"
key = "run-parallel-tasks-in-omp"
area = "general"
status = "open"
complexity = "advanced"
afk = true
priority = "normal"
blocked_by = ["general-012"]
+++
# Support the shared parallel task route in Oh My Pi

## Outcome

Oh My Pi runs approved independent implementations concurrently while leaving source integration, verification, and task completion under zdev coordination.

## Context

Read Harness adaptation, User choices, Integration and completion, Stops and recovery, and Testing in the [parallel execution brief](../../../plans/003-parallel-task-execution-brief.md), then load the shared parallel contract. templates/zdev/omp/prompts/zdev-implement.md currently uses blocking task agents and hub for ordinary rework. The primary task documentation linked in the brief describes conditional batch shapes, background result delivery, and isolation that may merge and clean its workspace when a worker exits. Start with these prompt/agent assets, src/integrations.rs, and existing OMP structured-result and fixture checks. Check current native behavior before choosing dispatch and isolation settings.

## Boundaries

- Preserve the shared consent, total role limit, serial completion, and recovery rules, together with configured profiles and result validation.
- Do not enable native automatic merge or cleanup that bypasses zdev verification or loses unfinished work. Use coordinator-owned worktrees and explicit paths when that is the suitable supported path.
- Keep ordinary blocking one-task behavior and do not assume an isolated worker can be revived through hub.

## Done when

- [ ] The installed Oh My Pi integration discovers the parallel route and uses the actual available task schema to run independent assigned implementations concurrently within the agreed limit.
- [ ] Background or batch results retain task identity and are accepted individually for shared serial integration, independent verification, and completion.
- [ ] Native isolation, merge, cleanup, and revival behavior cannot bypass the coordinator's ownership or preservation gates.
- [ ] Task-local blockers, shared decisions, cancellation, and explicit resume follow the shared contract and preserve findings and unfinished source changes.
- [ ] Canonical prompts and agent handoffs, registration, documentation, and regenerated fixtures describe the feature and its runtime capability checks accurately.

## Validation

- Use controlled native-result cases and rendered scenario review for asynchronous identity, conditional task shapes, isolated-worker non-revival, and cleanup/merge ownership.
- Add focused executable coverage only where adapter logic changes; retain existing structured-output, escalation, handoff, discovery, and fixture checks.
- Regenerate affected integrations and run standard area validation.

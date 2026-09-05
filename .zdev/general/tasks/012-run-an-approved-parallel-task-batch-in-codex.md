+++
schema_version = 1
id = "general-012"
key = "run-bounded-parallel-tasks-in-codex"
area = "general"
status = "done"
complexity = "advanced"
afk = true
priority = "normal"
blocked_by = ["general-011"]
+++
# Run an approved parallel task batch in Codex

## Outcome

Codex can overlap independent approved implementations within a user-agreed worker limit while completing tasks through serial integration and verification.

## Context

Read User choices, Admission and isolation, Integration and completion, Stops and recovery, and Testing in the [parallel execution brief](../../../plans/003-parallel-task-execution-brief.md). Reuse the preceding task's complete isolated implementation path. Current templates/zdev/native-area-loop.md and docs/area-loop.md refresh selection only after a single task commit, and src/tasks.rs readiness checks dependency completion rather than compatibility between concurrent changes. Start with the shared router, Codex skill, implementation and recovery references, src/integrations.rs rendering, and existing harness handoff checks. Define the shared parallel contract here and expose it through Codex; subsequent adapter tasks consume it.

## Boundaries

- Use one finite approved batch from one area and one coordinating run. Keep ordinary Implement and goal/loop behavior sequential, and do not automatically draft tasks or start another batch.
- Count all active delegated roles against the agreed worker limit, propose two when undecided, and preserve existing model profiles and harness permissions.
- Keep allocation in the coordinating run and use existing task records, retained Git work, and normal summaries for recovery. Do not add a daemon, cross-session scheduler, automatic takeover, or cost-estimation service.

## Done when

- [x] An explicit parallel route collects or reuses authorization for the task batch, worktree/branch creation, destination, worker limit, and cleanup; discussion or a ready queue alone starts no workers.
- [x] Selection checks shared interfaces, likely changes, validation resources, and ownership. Unsuitable or unsupported execution offers a sequential option before source mutation.
- [x] At least two independent tasks can implement concurrently in separate assigned trees; no task is dispatched twice, delegated roles stay within the agreed and available limits, and completed candidates get priority for integration verification.
- [x] Candidates integrate and complete one at a time using the shared one-task boundary and explicit assigned task selection. Expected destination commits do not discard still-valid sibling work, and affected workers receive changed requirements before acceptance.
- [x] A task-local blocker preserves its findings while independent approved work continues; a shared decision pauses affected tasks. A derived split or follow-up proposal preserves its source work and proposal without creating children or completing the source in this batch, pending an explicitly authorized follow-up. Cancellation or capacity loss stops new dispatch and accounts for active workers before cleanup.
- [x] Explicit resume inspects retained work and current records, excludes committed task completions, recovers any completion awaiting its commit, and obtains fresh admission without depending on old agent IDs. The final summary distinguishes committed tasks, preserved unfinished work, and the stop reason.
- [x] The shared contract and Codex installation expose the feature accurately, while other adapters remain explicitly unsupported until their own implementation tasks land.

## Validation

- Review the rendered route against finite-batch consent, overlapping tasks, two independent tasks, role-limit exhaustion, task-local versus shared blockers, a derived proposal awaiting follow-up, and resume after one task has committed or its completion commit has failed.
- Use controlled workers in a generic repository to demonstrate overlapping implementation and serial verified completion. Make a non-default ready task finish first and verify that only its record is completed. Test executable orchestration boundaries if added, without requiring paid live-model calls.
- Regenerate integrations and run existing route/discovery/parity checks plus standard area validation.

## Result

Added the shared Parallel route, Codex collaboration-agent orchestration, recovery guidance, adapter fallbacks, generated installations, and deterministic overlapping-worker coverage with serial completion.

Validation:

- Independent verification passed from exact snapshot W8351c689ecbcc913: cargo fmt, clippy with warnings denied, all 155 tests, cargo build, git diff check, generated parity, and controlled two-worker serial integration.

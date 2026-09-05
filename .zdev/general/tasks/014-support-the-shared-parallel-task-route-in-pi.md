+++
schema_version = 1
id = "general-014"
key = "run-parallel-tasks-in-pi"
area = "general"
status = "done"
complexity = "advanced"
afk = true
priority = "normal"
blocked_by = ["general-012"]
+++
# Support the shared parallel task route in Pi

## Outcome

Pi runs approved task implementations concurrently in their assigned worktrees and returns individual results for serial zdev integration.

## Context

Read Harness adaptation, User choices, Stops and recovery, and Testing in the [parallel execution brief](../../../plans/003-parallel-task-execution-brief.md), then load the shared parallel contract. templates/zdev/pi/extensions/zdev-subagent.ts currently accepts one role and prompt and always starts its child with cwd: ctx.cwd. The upstream subagent example linked in the brief demonstrates worker cwd and bounded concurrency. Extend zdev's existing smaller tool and Pi prompt/skill assets rather than importing the example wholesale. Start tests with the existing Pi installation and deterministic fixture checks in tests/lean.rs.

## Boundaries

- Preserve existing single-worker calls, configured role profiles, and fresh-child rework semantics. Add only the dispatch controls needed by the shared contract.
- Preserve the shared approval and integration boundary; child processes must not publish task records, complete tasks, or commit the destination.
- Propagate cancellation to active child processes and preserve unfinished source work. Avoid a persistent process manager or session database.

## Done when

- [x] The installed Pi route can dispatch a bounded set of workers with distinct assigned directories, delivering each task result to coordination as it becomes available.
- [x] Current single-role invocations remain compatible, and all active child roles respect the agreed and available limit.
- [x] An individual child failure or malformed result is attributed to its task without losing completed sibling results; cancellation stops further dispatch and reports any child not confirmed stopped.
- [x] Coordination uses the shared serial integration, independent verification, completion, and resume path for the returned results.
- [x] Canonical extension and prompt sources, documentation, registration, and regenerated fixtures expose the supported route accurately.

## Validation

- Use a controlled child executable to verify distinct cwd values, overlapping execution within the limit, individual result delivery, and cancellation/failure preservation without model calls.
- Run existing Pi installation, profile, handoff, and fixture checks, adding focused assertions only for changed executable behavior.
- Regenerate affected integrations and run standard area validation.

## Result

Added Pi parallel support with transient task-keyed batch handles, distinct assigned directories, arrival-driven coordinator gates, bounded role accounting, cancellation preservation, and serial shared completion.

Validation:

- Independent verification passed from exact snapshot W095eff29b8aa0a7c: focused child-process timing and race tests, all 157 tests, fmt, clippy with warnings denied, build, diff check, Pi fixture parity, and legacy behavior.

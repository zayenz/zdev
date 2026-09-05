+++
schema_version = 1
id = "general-013"
key = "run-parallel-tasks-in-claude"
area = "general"
status = "open"
complexity = "advanced"
afk = true
priority = "normal"
blocked_by = ["general-012"]
+++
# Support the shared parallel task route in Claude Code

## Outcome

Claude Code executes the approved parallel batch contract through its existing native integration with bounded workers and coordinator-owned completion.

## Context

Read Harness adaptation, User choices, Integration and completion, Stops and recovery, and Testing in the [parallel execution brief](../../../plans/003-parallel-task-execution-brief.md), then load the shared parallel contract produced by the prerequisite. templates/zdev/claude/workflows/zdev-implement.js currently owns one-task role sequencing and result validation; zdev-loop.js stops after a one-task blocker. Extend the Claude skill, native workflow assets, and rendering registration in src/integrations.rs. Existing workflow simulations in tests/lean.rs provide controlled agent responses. Use the primary Claude workflow and subagent documentation linked from the brief to check actual dispatch and isolation behavior.

## Boundaries

- Keep the shared batch, consent, role-limit, integration, and recovery semantics. Use existing Claude role profiles and semantic worker results.
- Do not add a separate SDK service or let native isolation merge or clean unfinished work outside coordination.
- Keep the current sequential workflows and their ordinary-subagent fallback. State parallel capability honestly when the runtime lacks required support; do not silently run concurrent edits in the parent checkout.

## Done when

- [ ] The installed Claude integration discovers the parallel route and can dispatch independent implementations concurrently to their assigned worktrees after the shared approval and admission gates.
- [ ] Controlled worker completion in a different order is mapped to the correct tasks, respects the worker limit, and produces serial independent verification and one destination commit per completed task.
- [ ] A malformed result, a task-local blocker, a shared decision, and cancellation follow the shared preservation and stop behavior without losing verifier findings.
- [ ] Recovery uses retained Git work and fresh task admission, and runtime fallback reports unavailable parallel capability without changing sequential behavior.
- [ ] Canonical Claude sources, affected shared rendering, documentation, and regenerated fixtures agree.

## Validation

- Extend the existing Claude workflow simulation with bounded overlapping dispatch, out-of-order results with serial completion, and interruption preserving an unfinished sibling.
- Retain existing one-task parsing, verification, rework, blocker-detail, and area-loop checks; use scenario review for user questions.
- Regenerate affected integrations and run discovery/parity and standard area validation.

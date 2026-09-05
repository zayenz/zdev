+++
schema_version = 1
id = "general-015"
key = "run-parallel-tasks-in-opencode"
area = "general"
status = "open"
complexity = "advanced"
afk = true
priority = "normal"
blocked_by = ["general-012"]
+++
# Support the shared parallel task route in OpenCode

## Outcome

OpenCode can run an explicitly approved parallel task batch using its native worker surface and the shared zdev integration boundary.

## Context

Read Harness adaptation, User choices, Admission and isolation, Stops and recovery, and Testing in the [parallel execution brief](../../../plans/003-parallel-task-execution-brief.md), then load the shared parallel contract. Existing templates/zdev/opencode/commands/zdev-implement.md uses named subagents with compact filesystem handoffs; its area-loop adapter is currently bounded to one committed task. Start with these command/skill assets, src/integrations.rs, the non-Claude handoff tests, and the upstream OpenCode task source linked in the brief. Check the actual available task surface for concurrent calls and source-directory routing rather than assuming a background or cwd argument exists.

## Boundaries

- Keep ordinary bounded loop behavior and existing one-task commands. The new route handles only its approved finite batch.
- Use supported native tools and explicit source paths/working directories. Preserve permissions, profiles, fresh verification, and coordinator ownership of integration.
- Do not introduce a separate service or pretend to support isolation when required filesystem access or concurrent dispatch is unavailable.

## Done when

- [ ] The installed OpenCode command and root skill discover and route an explicit parallel request through the shared approval and readiness checks.
- [ ] Two independent implementations can run concurrently against distinct source trees, with explicit task identity and the agreed role limit.
- [ ] Coordination receives and attributes each result, performs serial integrated verification and completion, and preserves sibling work through task-local failure or interruption.
- [ ] The documented recovery path revalidates retained work, while unavailable runtime support gives a clear sequential option before mutation.
- [ ] Canonical commands, role handoffs, registration, user guidance, and regenerated fixtures agree with the supported native calls.

## Validation

- Review actual tool schemas and rendered payloads for directory routing, simultaneous dispatch, result identity, consent, and cancellation; use a generic controlled-worker smoke scenario for the complete route.
- Add focused tests for any executable adapter logic introduced; otherwise use existing handoff/discovery/parity checks and scenario review rather than prose-locking tests.
- Regenerate affected integrations and run standard area validation.

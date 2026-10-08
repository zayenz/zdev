+++
schema_version = 1
id = "auto-005"
key = "codex-serial"
area = "auto"
status = "open"
complexity = "advanced"
afk = true
priority = "normal"
blocked_by = ["auto-003", "auto-004"]
+++
# Run serial research through Codex

## Outcome

An authorized Codex research run repeatedly proposes, executes, and records work on the current branch within its limits.

## Context

Read the new one-experiment and run contracts. Start with templates/zdev/codex-skill.md, native-area-loop.md, shared-contract.md, task-workflows.md and src/integrations.rs. The ordinary area loop stops at queue exhaustion; research needs a distinct route that consumes previous lessons and generates further scoped work.

## Boundaries

- Use one coordinator and current-branch serial execution; preserve ordinary Implement/Loop behavior.
- Use resolved concrete worker profiles and fresh acceptance verification according to the brief, without verification agents for every discard.

## Done when

- [ ] The installed Codex route authorizes and resumes finite or explicit unlimited runs, generates both task kinds, and uses the local experiment lifecycle.
- [ ] It continues after rejected or terminal failed experiments, reads their lessons, and stops on the recorded target, limits, interruption, or shared blocker.
- [ ] A new session reconstructs progress from durable records and native continuation is used only where the harness supports it; any fallback states its actual extent.
- [ ] Canonical templates, generated fixtures, user documentation, and route discoverability agree.

## Validation

- Add or update deterministic integration checks for accepted, rejected, empty-queue generation, budget stop, and recovery behavior. Regenerate affected fixtures in this task, run generator checks plus matching repository checks; defer signed-in live execution to live acceptance.

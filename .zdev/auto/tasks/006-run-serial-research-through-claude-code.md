+++
schema_version = 1
id = "auto-006"
key = "claude-serial"
area = "auto"
status = "open"
complexity = "advanced"
afk = true
priority = "normal"
blocked_by = ["auto-003", "auto-004"]
+++
# Run serial research through Claude Code

## Outcome

Claude Code provides the same complete serial research behavior and recovery rules through its installed workflow.

## Context

Read the new experiment/run contracts. Start with templates/zdev/claude/workflows/zdev-loop.js, claude-area-loop.md, claude-skill.md, shared-contract.md and src/integrations.rs. Reuse native agent controls and established workflow test techniques in tests/lean.rs; this task can start independently of codex-serial.

## Boundaries

- Keep authoritative records and serial acceptance with one coordinator; preserve ordinary Claude task workflows.
- Use the brief's verification distinction and concrete harness role profiles.

## Done when

- [ ] The generated Claude entry point creates/resumes bounded or explicitly unlimited research runs and proposes and executes both task kinds.
- [ ] Recorded lessons guide continuation after discard or exhausted attempts, while accounting and stop reasons match the public run contract.
- [ ] Malformed worker returns, interruptions, and shared failures preserve recoverable state and cannot produce false completion or improvement.
- [ ] Canonical sources, generated fixtures, user documentation, and command discovery agree.

## Validation

- Use the existing workflow test approach with deterministic worker/evaluator results to exercise generation, decisions, limits, and resume. Regenerate affected fixtures and run matching checks; defer credentialed Claude execution to live acceptance.

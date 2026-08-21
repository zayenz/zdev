+++
schema_version = 1
id = "improvements-032"
key = "work-context-command"
area = "improvements"
status = "done"
blocked_by = ["improvements-031"]
+++
# Add one read-only work-context command

## Outcome

A single zdev invocation returns the fresh deterministic context needed to decide and dispatch task work.

## Context

Current harness flows separately invoke status, goal, and three Git inspections. Add the corrected work-context design from docs/workflow-round-trips.md: lifecycle is classified first, closed areas return minimal branch-independent no-work, and open areas return nested validated status, goal, and Git evidence.

## Boundaries

- The command is read-only, persists nothing, takes no write lock, and performs no caching.
- Use nested JSON values rather than JSON encoded inside strings.
- Collect sequentially and fail closed on command failure, invalid UTF-8, empty output, or disagreement.
- Do not combine completion, staging, commit, verification, or next-task selection into this command.

## Done when

- [x] One command returns a stable schema for closed no-work and for open ready, empty, exhausted, blocked, and unsafe states.
- [x] Open results require matching area, task, lifecycle, queue, safe task-work status, and the three Git evidence strings.
- [x] Closed results require no branch checkout or Git cleanliness evidence.
- [x] Human and JSON output give actionable errors without mutating the repository.

## Validation

- Add focused black-box tests for closed off-branch/detached behavior, ready work, each no-work class, mismatched state, and Git command failure.
- Run the area-wide validation from brief.md.

## Result

Added one read-only work-context command that returns nested goal/status projections and exact Git evidence while short-circuiting closed areas.

Validation:

- Six focused work-context tests, full 103-test suite, formatting, strict Clippy, build, zdev check, and diff check passed; fresh independent verification passed.

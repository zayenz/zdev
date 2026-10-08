+++
schema_version = 1
id = "auto-009"
key = "claude-rolling"
area = "auto"
status = "open"
complexity = "advanced"
afk = true
priority = "normal"
blocked_by = ["auto-006", "auto-007"]
+++
# Continuously refill Claude Code research workers

## Outcome

Claude Code provides rolling parallel research with the same admission, evidence, acceptance, and recovery rules.

## Context

Read the candidate contract and Claude serial route. Start with templates/zdev/claude/workflows/zdev-parallel.js, claude-skill.md and src/integrations.rs. The existing event/arrival handling is a useful seam, but its approved task set is finite; research must generate and refill within run authorization.

## Boundaries

- One coordinator serializes authoritative record updates and acceptance; worker preparation and measurement obey distinct capacity limits.
- Keep ordinary finite parallel workflows intact and use native supported worker controls.

## Done when

- [ ] The Claude workflow refills available capacity before the full active set finishes, generating both task kinds within run scope.
- [ ] Dispatch takes the latest accepted revision; returning stale candidates reconcile and remeasure under the run budget before acceptance.
- [ ] Acceptance verification, shared evaluator changes, worker errors, cancellation, and resume follow the common contract and preserve ambiguous work.
- [ ] Canonical templates, generated workflows/fixtures, and documentation expose the complete rolling route.

## Validation

- Extend deterministic workflow scenarios for rolling refill, out-of-order returns, drift, measurement serialization, and interrupted recovery. Regenerate fixtures and run matching checks; defer credentialed Claude execution to live acceptance.

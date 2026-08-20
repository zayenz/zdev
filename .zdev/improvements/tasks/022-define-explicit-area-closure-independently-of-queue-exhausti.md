+++
schema_version = 1
id = "improvements-022"
key = "explicit-area-closure-contract"
area = "improvements"
status = "open"
blocked_by = []
+++
# Define explicit area closure independently of queue exhaustion

## Outcome

Zdev has an approved, implementation-ready contract that distinguishes an open area with no remaining queued work from an area whose objective was explicitly closed.

## Context

Today `zdev goal` calls an all-done task set `complete`; `zdev next`, `zdev next --any`, status output, documentation, and installed workflows use similar language. No durable area lifecycle exists. This makes completion of an initial task bundle appear to close an ongoing objective such as `improvements`. Write `docs/area-lifecycle.md` and settle the contract before runtime changes.

## Boundaries

- Produce a concise design record and exact follow-up implementation task; do not change runtime behavior in this task.
- Keep task status and slice-derived progress unchanged, and never infer objective completion from task counts.
- Do not add execution claims, abandonment state, branch deletion, automatic integration, switching, or rebasing.
- Preserve existing area records through an explicit backward-compatibility rule.
- Prefer one small explicit close and reopen lifecycle over a general state machine.

## Done when

- [ ] The record defines a complete matrix for open and closed areas with zero tasks, ready work, blocked work, and all tasks done.
- [ ] It settles exact human and JSON vocabulary for goal, area-specific next, project-wide next, and status so queue exhaustion and objective closure cannot be confused.
- [ ] It settles durable representation, the default for existing area.toml files, and exact area close and reopen grammar.
- [ ] It defines whether empty areas may close and which branch or structural-safety gates closure requires.
- [ ] It defines behavior for task import, task reopen, general areas, parent and child areas, and closed-area selection.
- [ ] It ends with one narrow implementation seam and acceptance criteria requiring no further product decision.

## Validation

- Compare the lifecycle matrix against src/goal.rs, task selection in src/tasks.rs, status rendering, orchestration templates, and current goal fixtures.
- Review the contract for contradictions with general areas, parent areas, and existing area records.
- Run `git diff --check`.

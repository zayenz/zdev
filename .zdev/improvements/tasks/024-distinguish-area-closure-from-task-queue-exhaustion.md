+++
schema_version = 1
id = "improvements-024"
key = "implement-area-lifecycle"
area = "improvements"
status = "done"
blocked_by = []
+++
# Distinguish area closure from task-queue exhaustion

## Outcome

Implement the exact lifecycle, queue, command, selection, status, and orchestration contract in docs/area-lifecycle.md.

## Context

The design is settled and includes a narrow implementation seam. Current complete output conflates an all-done task queue with closure of the area objective.

## Boundaries

- Treat docs/area-lifecycle.md as authoritative behavior.
- Add only open and closed; task and slice lifecycle remain unchanged.
- Preserve task ordering, dependency validation, branch diagnostics, atomic writers, and the existing stale-base policy.
- Add no branch switching, deletion, integration, automatic closure, reason record, or migration command.
- Update canonical sources and regenerate installed harness artifacts through the existing renderer.

## Done when

- [x] Missing lifecycle reads as open, new areas write open, and invalid values fail.
- [x] Close and reopen are locked, atomic, idempotent, and enforce the documented task and branch gates.
- [x] Goal, area-specific next, project-wide next, and both status modes expose lifecycle and queue exactly as documented.
- [x] Open exhausted areas never appear closed or complete.
- [x] Closed areas cannot import or reopen tasks and never enter ready selection.
- [x] General and parent-child behavior follows the contract.
- [x] All harness workflows implement the documented no-work parsing and worker-start rules.

## Validation

- Add focused black-box coverage for the open and closed matrix across empty, ready, exhausted, and invalid combinations.
- Add exact human and JSON fixture coverage for close, reopen, goal, next, next --any, and status.
- Cover a backward-compatible area record without lifecycle, closed-area import and task-reopen rejection, and general and parent-child cases.
- Run canonical and generated all-harness install/check tests, the lean suite, package verification, and the area brief's full validation.

## Result

Implemented explicit open and closed area lifecycle independently of task-queue exhaustion across commands and harness workflows.

Validation:

- Independent verification passed after correcting strict Claude no-work state binding and exact lifecycle fixture coverage.
- Focused lifecycle, invariant, parser, stale-policy, parent-child, and generated-artifact tests passed.
- cargo test --locked --test lean passed (93/93), and full formatting, Clippy, tests, build, five-harness install/check, package verification, and git diff checks passed.

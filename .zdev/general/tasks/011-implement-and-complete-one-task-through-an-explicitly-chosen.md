+++
schema_version = 1
id = "general-011"
key = "implement-one-task-in-an-assigned-worktree"
area = "general"
status = "open"
complexity = "advanced"
afk = true
priority = "normal"
blocked_by = []
+++
# Implement and complete one task through an explicitly chosen worktree

## Outcome

An explicitly requested isolated implementation can produce one independently verified task commit on the area's destination branch without moving its authoritative task records.

## Context

Read Admission and isolation, Integration and completion, Stops and recovery, and Testing in the [parallel execution brief](../../../plans/003-parallel-task-execution-brief.md), together with the standing area brief. Current work-context admission requires the area's owning branch; snapshots are stored under the issuing worktree's Git directory. A personal record may not exist in a linked checkout. Start with src/lib.rs work-context collection and storage, src/project.rs branch admission, templates/zdev/task-workflows.md, templates/zdev/codex-skill.md, and the linked-worktree and Git ownership tests in tests/lean.rs. Deliver the complete one-task path through the Codex adapter so the later parallel route can reuse a working integration boundary. Existing general-007 and general-008 address shared questions and technical scope discretion; reuse their behavior when present without duplicating that work.

## Boundaries

- Keep ordinary implementation in its current checkout by default. Create a task branch or worktree only under explicit existing authority or after the needed harness question.
- Retain the area's branch binding and ordinary work-context branch checks. Separate authoritative task context from the assigned source root; do not mark tasks done in worker checkouts.
- Use existing Git and harness operations where sufficient. Add only helpers needed for a concrete validation or transport boundary, with no scheduler, new task status, or competing record copy.

## Done when

- [ ] The Codex one-task route can admit an explicit ready task in the authoritative checkout, assign a separate source worktree and baseline, and give a fresh worker the correct source, instruction, record, and snapshot locations. Coordinator refresh, snapshot capture, and integrated verification retain this task with --task <id> rather than selecting the queue default.
- [ ] The worker edits and validates only its assigned source checkout; coordination attributes its full delta and transports additions, deletions, binary content, and modes without publishing copied .zdev records or unrelated work.
- [ ] Coordination reconciles the candidate with the current destination, independently verifies the integrated task through existing snapshot gates, and publishes exactly one completion commit on the destination.
- [ ] Trunk and isolated areas, and personal and tracked record policies, retain correct task identity and branch ownership through this path.
- [ ] Failure before the completion commit preserves attributable work and reports its location, including recovery when task completion was written but its commit failed. Cleanup excludes unfinished work, active workers, and pre-existing user worktrees.
- [ ] Canonical guidance, relevant user documentation, and regenerated Codex integration describe and expose the complete opt-in one-task path without changing sequential defaults.

## Validation

- Exercise a generic temporary repository end to end, including personal records, a binary addition, and an independent destination change before integration. Check the resulting source commit and authoritative task index.
- Add focused regressions for executable isolation or transport behavior introduced here; use scenario review for instruction-only routing and avoid tests that freeze prose.
- Regenerate affected integrations, run existing work-context/branch/record and fixture checks, and run the standard validation in the area brief.

+++
schema_version = 1
id = "general-008"
key = "resolve-routine-technical-scope-adjustments"
area = "general"
status = "open"
complexity = "advanced"
afk = true
priority = "high"
blocked_by = []
+++
# Let coordination resolve routine technical scope adjustments

## Outcome

Coordination can relax a planner-written technical restriction necessary for the agreed outcome, record the adjustment, and resume verified work without requiring another user approval.

## Context

Read Coordinator discretion, its examples, and Boundaries in the [settled workflow brief](../../../plans/002-workflow-review-brief.md). The user approved coordinator discretion while retaining explicit user constraints, acceptance criteria, compatibility promises, and clear ownership. Current shared-contract.md requires renewed approval whenever an approved artifact changes; task-workflows.md treats every explicit boundary violation as scope expansion. src/tasks.rs also renders derived children with Task-owned paths (exact). Inspect those sources with references/implement.md, verify.md, task-format.md, to-tasks.md, and the executable Claude implementation workflow. File lists produced during planning or a split describe the initial allocation; they must not prevent a necessary adjustment that coordination can safely attribute.

## Boundaries

- Workers propose adjustments; coordination owns changes to brief or task records and explains them in the existing progress report.
- Ask before changing the agreed outcome, acceptance criteria, compatibility promises, or an explicit user constraint. Unclear ownership or an unresolved material choice remains a blocker.
- Preserve exact review and approval of pending task bundles. The exception concerns coordinator adjustments during authorized work, not silently changing a bundle awaiting approval.
- Preserve mechanical split validation, parent/sibling ownership checks, independent verification, and scoped commits. Use existing documents and transient coordination context; add no policy engine, approval ledger, or lifecycle.

## Done when

- [ ] A necessary helper edit or second file can proceed after coordination checks ownership, updates an incidental technical restriction when needed, and explains the adjustment.
- [ ] Derived children can extend planned paths only after coordination checks retained parent edits and sibling assignments; existing records remain readable and explicit user constraints remain binding.
- [ ] An intentional brief or task clarification can be followed by fresh work-context admission without being mistaken for unrelated drift. The original Git baseline remains available, and verification checks the updated requirements.
- [ ] Normal completion includes the attributable clarified records in the task commit; unrelated record changes and source changes still trigger ownership review.
- [ ] Canonical contracts, executable adapters where applicable, relevant docs, and regenerated integrations express the same rule.

## Validation

- Use focused behavioral regressions for any changed executable rendering, routing, or completion behavior; retain existing split-ownership, snapshot, and commit-scope checks.
- Review both the permitted helper/file adjustment and a prohibited user-constraint or ownership violation against the rendered coordinator and verifier instructions.
- Regenerate affected integrations and run the standard validation in the area brief.

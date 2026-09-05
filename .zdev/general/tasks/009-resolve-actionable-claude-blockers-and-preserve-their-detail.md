+++
schema_version = 1
id = "general-009"
key = "preserve-actionable-claude-blocker-context"
area = "general"
status = "done"
complexity = "standard"
afk = true
priority = "normal"
blocked_by = []
+++
# Resolve actionable Claude blockers and preserve their details

## Outcome

Claude continues when useful investigation leaves directly actionable task work, and genuine verifier blockers retain the details needed to resolve them.

## Context

Read Blocker handling in the [settled workflow brief](../../../plans/002-workflow-review-brief.md). In templates/zdev/claude/workflows/zdev-implement.js, resolveImplementerResult stops on snapshot equality before classifying a replacement blocker. classifyImplementerBlocker also says no progress requires stop. Read-only diagnosis can establish a next step without changing files. The terminal verifier-blocker branch reports summary and generated snapshot evidence but drops findings. Use the existing classifier and report fields, the shared task-workflows.md blocker contract, and the mocked Claude workflow tests in tests/lean.rs.

## Boundaries

- Keep snapshot comparison as evidence of checkout changes. It must continue to reject unattributed changes and invalid verification state.
- Use transient prior results and the existing classifier to distinguish actionable progress from repeating the same unresolved obstacle; add no persistent retry state or new workflow engine.
- Preserve real user decisions, unavailable prerequisites, unsafe ownership, and malformed worker output as blockers.

## Done when

- [x] An unchanged checkout reaches blocker classification, and new investigation or validation evidence can lead to a concrete continuation within the task.
- [x] Repeated attempts stop when the same obstacle remains unresolved with no actionable next step; existing safety and external-prerequisite stops still work.
- [x] Terminal verifier-blocker output includes its findings, including a concrete missing prerequisite or user decision, through existing public report fields.
- [x] Focused executable tests cover unchanged-state continuation, a repeated genuine impasse, and preservation of verifier findings; regenerated Claude integration matches its source.

## Validation

- Extend the existing Claude workflow fixtures for the three stated behaviors and run the focused workflow tests.
- Regenerate affected integrations and run the standard validation in the area brief.

## Result

Made Claude classify unchanged blocker retries using prior and current evidence, continue on actionable investigation, stop repeated impasses, and preserve terminal verifier findings.

Validation:

- Focused Claude blocker-routing and generated-template parity tests passed.
- cargo fmt --all -- --check; cargo clippy --locked --all-targets --all-features -- -D warnings; cargo test --locked; cargo build --locked; git diff --check all passed.

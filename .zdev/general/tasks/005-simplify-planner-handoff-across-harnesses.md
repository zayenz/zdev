+++
schema_version = 1
id = "general-005"
key = "simplify-planner-handoff-across-harnesses"
area = "general"
status = "open"
complexity = "advanced"
blocked_by = ["general-004"]
+++
# Simplify planner handoff across harnesses

## Outcome

Planners across all five harnesses return a small semantic planning result while coordination reconstructs the compatible public planner envelope and passes the validated semantic plan to implementation.

## Context

The planner currently carries the same nine-key transport envelope as implementers and must encode identity, fixed escalation, and three prefixed evidence strings. Replace only the planner-facing contract with an exact four-field semantic object. A plan is {"verdict":"plan","summary":"<non-empty>","plan":{"approach":"<non-empty>","paths":["<normalized repository-relative path>"],"validation":["<non-empty validation step>"]},"findings":[]}; a blocker has verdict blocker, a non-empty summary, plan null, and at least one non-empty finding. Coordination reconstructs the existing nine-key planner envelope. This task follows general-004 and replaces its legacy Claude planner schema with the semantic schema while retaining its one-dispatch structured-object and strict-string normalization.

## Boundaries

- For plan, require a non-null exact three-field plan object, at least one normalized repository-relative path, at least one validation step, and empty findings. For blocker, require plan null and at least one non-empty finding.
- Coordination supplies schema_version 1, kind planner, area, task_id, and escalation none. For plan it generates exactly Approach: <approach>, Paths: <comma-joined paths>, and Validation: <semicolon-joined validation steps> evidence entries; for blocker it generates empty evidence. It copies summary and findings and validates the complete compatible nine-key envelope.
- Pass the validated semantic plan object to the advanced implementer without changing its approach, path entries, or validation entries. Preserve exactly one fresh read-only planner before an advanced edit, product-decision blockers, plan-before-edit safety, inline contract fallback, branch and ownership gates, and public compatibility.
- Claude enforces normalization and reconstruction in executable workflow code. Codex, OpenCode, Pi, and OMP express the same exact coordinator contract and fixture parity on their prompt-driven surfaces; do not claim executable runtime enforcement where none exists.
- On Claude, retain general-004s structured semantic object or valid strict semantic JSON normalization from the same worker dispatch. Codex, OpenCode, Pi, and OMP use valid strict semantic JSON on their current prompt/text surfaces; OMP native structured-result preference belongs only to the blocked follow-up task. Do not add retries, replacement planners, second zdev-controlled model turns, Markdown extraction, wrappers, durable state, or changes to implementer and verifier contracts.

## Done when

- [ ] Canonical contracts define the exact semantic planner variants and exact coordinator reconstruction for both plan and blocker public envelopes.
- [ ] Claude, Codex, OpenCode, Pi, and OMP planner roles and coordinator adapters express the same semantic responsibility split without adding worker dispatches; executable parsing is tested only where executable adapter code exists.
- [ ] Valid plans preserve the exact approach string, ordered path array, and ordered validation array passed to the advanced implementer; coordinator-generated public evidence uses the settled deterministic joining rules.
- [ ] Planner blockers reconstruct with empty public evidence, copied summary and findings, fixed escalation none, and stop before edits.
- [ ] Malformed, extra, contradictory, legacy nine-key, mismatched, or unavailable planner output fails closed without an implementer or second planner.
- [ ] Generated fixtures match canonical templates, and Claude replaces general-004s legacy schema while retaining its structured-object and same-call strict-string paths.
- [ ] Focused coverage proves plan, blocker, path and validation ordering, malformed output, public-envelope reconstruction, all-harness contract parity, and exactly one planner dispatch.

## Validation

- Regenerate the Codex, Claude, OpenCode, Pi, and OMP integrations with their existing cargo run --locked -- skill install commands.
- Run the focused task-workflow, Claude structured-envelope, implementation-routing, harness-discovery, and generated-fixture tests in tests/lean.rs.
- Run the area-wide validation from brief.md.

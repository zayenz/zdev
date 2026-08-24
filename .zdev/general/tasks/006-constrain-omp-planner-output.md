+++
schema_version = 1
id = "general-006"
key = "constrain-omp-planner-output"
area = "general"
status = "done"
complexity = "standard"
blocked_by = ["general-005"]
+++
# Constrain OMP planner output

## Outcome

Oh My Pi planning uses its native focused output schema to improve first-response reliability while preserving progress from a valid semantic result returned by the same planner dispatch.

## Context

OMP agent frontmatter accepts an opaque JSON Schema in the output field, and settled task results expose parsed data at details.results[].structuredOutput.data while retaining the worker output text. After the universal planner handoff is simplified, add its exact semantic planner schema to templates/zdev/omp/agents/zdev-planner.md. Update the OMP implementation prompt to prefer structuredOutput.data and otherwise strictly validate the output text from that same task result.

## Boundaries

- Add the exact universal semantic planner JSON Schema under the output frontmatter field of the OMP planner agent and update only OMP planner coordination guidance and focused fixtures.
- Use OMPs existing permissive task-result behavior so a valid strict output string from the same dispatch remains usable when structured data is absent. Keep exactly one planner task dispatch; do not add task retries, hub revival, follow-up messages, replacement workers, or another coordinator pass for formatting.
- Preserve semantic plan/blocker behavior, coordinator-owned identity and public evidence, plan-before-edit safety, inline rendered contracts, and strict same-call text validation.
- Do not constrain the shared OMP verifier because it also serves audits. Do not schema-constrain implementers, change other harnesses, add a wrapper or workflow engine, or accept Markdown extraction and other permissive repair.

## Done when

- [x] The generated OMP planner definition carries the exact semantic planner schema in its output frontmatter field.
- [x] OMP implementation guidance prefers details.results[].structuredOutput.data when it validates, and otherwise proceeds from a valid strict semantic JSON string in the same result output.
- [x] Unavailable, malformed, contradictory, legacy, or mismatched results block without launching, reviving, or messaging another planner for formatting.
- [x] The shared OMP verifier and audit behavior remain unchanged.
- [x] Generated OMP agent, prompt, skill, and reference fixtures match their canonical templates.
- [x] Focused coverage proves the exact frontmatter schema and result-field guidance, structured-result preference, strict-text compatibility, and absence of formatting retry or revival paths.

## Validation

- Regenerate the OMP integration with cargo run --locked -- skill install omp --to .omp --force.
- Run the focused OMP discovery, generated-fixture, audit-preservation, and task-workflow tests in tests/lean.rs.
- Run the area-wide validation from brief.md.

## Result

Added the exact semantic planner output schema to OMP and made coordination prefer validated structured data with a strict same-result text fallback, without formatting retries or verifier constraints.

Validation:

- Focused OMP schema, workflow, discovery, audit-preservation, and generated-fixture tests passed.
- cargo fmt, clippy with warnings denied, all 137 tests, cargo build, temporary-directory OMP fixture parity, and git diff --check passed under independent verification.

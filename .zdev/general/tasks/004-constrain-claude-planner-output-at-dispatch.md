+++
schema_version = 1
id = "general-004"
key = "constrain-claude-planner-output-at-dispatch"
area = "general"
status = "open"
complexity = "standard"
blocked_by = []
+++
# Constrain Claude planner output at dispatch

## Outcome

Claude advanced-task planning reliably produces or recovers the required planner envelope from one worker dispatch, without a replacement planner run.

## Context

The Claude dynamic implementation workflow currently asks the planner for strict nine-key JSON in prose, then parses its text. Claude workflows support an agent-call schema that can return structured data directly. Apply that native boundary at the planner dispatch in templates/zdev/claude/workflows/zdev-implement.js, while accepting a valid strict JSON string from the same completed call as a compatibility fallback. Regenerate the composed goal and loop fixtures; keep the canonical task contract and existing semantic checks authoritative.

## Boundaries

- Use the native Claude workflow agent schema on the single advanced-task planner call, then normalize either its structured object or a valid strict JSON string from that same call through the existing semantic validation.
- Preserve the existing plan/blocker semantics, exact identity checks, evidence rules, planning-before-edit boundary, inline worker-contract fallback, and invalid-result blocker.
- Do not add a retry loop, replacement planner, second model turn controlled by zdev, SDK wrapper, new workflow engine, permissive Markdown extraction, or broad structured-output migration for implementers and verifiers.
- Do not set CLAUDE_PLUGIN_ROOT in Claude settings; the installed-contract fallback remains authoritative for direct skill installs.

## Done when

- [ ] The Claude planner call supplies a focused exact-object schema and consumes a returned structured object without serializing ceremony or weakened semantic checks.
- [ ] A valid strict planner JSON string returned by the same call remains usable as a compatibility fallback, so zdev proceeds without another planner when the result is already recoverable.
- [ ] Valid plan and planner-blocker results route as before; only unavailable, malformed, contradictory, or mismatched results block, without starting an implementer or a second planner.
- [ ] Generated Claude implement, goal, and loop workflows match their canonical templates and retain the absent-CLAUDE_PLUGIN_ROOT inline fallback.
- [ ] Focused coverage proves one planner dispatch, schema attachment, structured-object routing, same-call strict-string fallback, blocker routing, and unusable-result blocking without a rerun path.

## Validation

- Regenerate the Claude integration with cargo run --locked -- skill install claude --to .claude/skills/zdev --force.
- Run the focused Claude structured-envelope and implementation-routing tests in tests/lean.rs.
- Run the area-wide validation from brief.md.

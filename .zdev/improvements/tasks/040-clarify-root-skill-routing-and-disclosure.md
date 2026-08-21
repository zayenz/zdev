+++
schema_version = 1
id = "improvements-040"
key = "skill-routing-cleanup"
area = "improvements"
status = "open"
blocked_by = ["improvements-031"]
+++
# Clarify root skill routing and disclosure

## Outcome

The root zdev skill is a coherent workflow driver that loads each detailed contract once and routes natural requests unambiguously.

## Context

Apply the repository review and write-a-skill guidance to remove duplicated routing/preflight text and make supporting references discoverable at the point of use. The root is intentionally a main workflow driver and may exceed 100 lines; usefulness, not a line target, governs the edit.

## Boundaries

- Do not split material merely to satisfy a line-count recommendation.
- Keep safety-critical routing and the compact common contract visible in the root.
- Move or deduplicate only detail that has one clear routed owner.
- Preserve the integrated prose-quality guidance rather than creating a separate unslop skill.

## Done when

- [ ] Every explicit and active-zdev natural-language intent has one route, including audit, goal, and loop.
- [ ] Each routed reference is loaded once when needed and does not restate the full common preflight.
- [ ] Canonical root and harness-specific roots agree on shared semantics while retaining native adapter instructions.
- [ ] A new user can locate discuss, investigate, tasks, implement, verify, audit, goal/loop, recovery, and configuration behavior from the root.

## Validation

- Run focused discoverability and generated-fixture tests.
- Manually trace one request for each routed intent in every harness template.
- Run the area-wide validation from brief.md.

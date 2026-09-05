+++
schema_version = 1
id = "general-017"
key = "configure-named-execution-profiles"
area = "general"
status = "open"
complexity = "advanced"
afk = true
priority = "normal"
blocked_by = []
+++
# Configure and inspect named execution profiles

## Outcome

Users can define, inspect, and resolve normal, advanced, simple, and custom execution profiles without changing current worker defaults.

## Context

Read Named settings, Initial mappings, Selection and scope, and Testing and boundaries in the [execution profiles brief](../../../plans/004-execution-profiles-brief.md), plus the standing area brief. src/config.rs currently models four roles per harness, resolves whole rows from local/global/default settings, and exposes no named collections. Planning reuses advanced-implementer. src/integrations.rs resolves workers at installation. Extend the existing config surface with named definitions, an optional independent planner row, explicit saved-default selection, and read-only profile/role resolution that later adapters can consume. Start with docs/config-command.md, docs/worker-profiles.md, and the existing config layering, strict parsing, atomic mutation, and worker-profile tests in tests/lean.rs.

## Boundaries

- Preserve legacy worker files and current effective normal mappings. Existing worker.<harness>.<role> settings continue to describe normal; missing named role rows fall back according to the brief.
- Implement the approved Codex advanced/simple and Claude advanced seeds exactly; other harnesses accept custom mappings without invented provider defaults.
- Keep model and effort atomic and native inheritance explicit. Reject unavailable profile definitions or adapter-inexpressible values before dispatch; do not build provider discovery, a model ranking service, or new task metadata.

## Done when

- [ ] Typed zdev config commands can list, inspect, create/update, and remove named profile settings and explicitly save or clear a default, with useful read-only output for resolving a harness and role.
- [ ] Resolution follows explicit role choice, interaction/run choice, saved default, then normal; local/global named rows and missing-role fallback behave as the brief states without cycles or cross-profile leakage.
- [ ] An optional planner row can differ from implementation; when absent it uses the selected profile's advanced implementer. Existing four-role configurations preserve their behavior.
- [ ] The approved seed mappings and a custom advanced-max profile resolve to the intended concrete models and efforts, while unknown or undefined-for-harness names report an error rather than silently selecting normal.
- [ ] Legacy configuration and existing normal installation/check behavior remain compatible; invalid writes preserve existing files, and read-only resolution does not modify configuration.
- [ ] Configuration help and user documentation explain named profiles, role settings, fallback, supported harness controls, and explicit persistence using the existing vocabulary.

## Validation

- Extend existing black-box configuration tests for legacy normal behavior, named-row precedence, planner fallback, custom profiles, saved-default selection, unknown profile/harness pairs, and failed-write preservation.
- Reuse existing profile rendering and inheritance checks; do not add a model catalog test matrix or live provider calls.
- Run the standard area validation and regenerate any integration fixtures affected by additive planner support.

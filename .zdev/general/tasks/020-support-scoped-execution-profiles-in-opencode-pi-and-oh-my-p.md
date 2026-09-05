+++
schema_version = 1
id = "general-020"
key = "select-scoped-execution-profiles-in-portable-adapters"
area = "general"
status = "done"
complexity = "advanced"
afk = true
priority = "normal"
blocked_by = ["general-018"]
+++
# Support scoped execution profiles in OpenCode, Pi, and Oh My Pi

## Outcome

OpenCode, Pi, and Oh My Pi honor the same named profile and one-off role choices, including plan-only task requests, through their existing native adapters.

## Context

Read Named settings, Plan-only requests, Workflow coverage, and Testing and boundaries in the [execution profiles brief](../../../plans/004-execution-profiles-brief.md), then load the prerequisite's shared selection contract. OpenCode and Oh My Pi render native role metadata, while Pi's zdev-subagent.ts fixes role settings when the extension is generated and maps planner to advanced-implementer. Extend those existing adapters to carry selected settings without installing a different integration for each run. Start with templates/zdev/opencode, templates/zdev/pi, templates/zdev/omp, their root skills, src/integrations.rs, and existing native handoff/profile/installation tests in tests/lean.rs. Preserve each provider's actual model and effort representation.

## Boundaries

- Support normal plus user-defined mappings for each harness; do not invent built-in advanced/simple provider combinations not specified in the brief.
- Keep existing single-worker calls, native role tools, profile syntax, Pi fresh-child rework, and OMP result extraction/revival rules. Selecting a role profile does not grant new execution authority.
- Use actual native override controls or prepared definitions. Keep simultaneous selections independent and avoid per-run shared-file rewrites, a new process service, or silent unsupported-effort fallback.

## Done when

- [x] All three installed integrations discover the shared plan-only and scoped-profile behavior and pass the intended concrete model and native effort settings to the selected worker roles.
- [x] Each adapter demonstrates 'plan next task with advanced planner' using a configured advanced mapping, with explicit task identity and no source edits, implementation, verification, task creation, or Git workspace changes.
- [x] Run-level and one-off selections retain their intended scope through required planning, implementation, verification, audits, task-draft challenge, and authorized continuation; an omitted planner uses the documented fallback.
- [x] Pi child invocations and OpenCode/OMP native dispatch retain the run's resolved concrete model/effort values, including across mid-run preference edits, and keep differently selected runs independent without rewriting shared installation. Undefined profiles or inexpressible settings report a useful error before dispatch.
- [x] Normal behavior and worker result contracts stay compatible, and canonical adapter sources, user guidance, registration, and regenerated fixtures accurately describe these three adapters and the support already landed in the other harnesses.

## Validation

- Use controlled Pi child invocation to check selected model/thinking arguments and backward-compatible ordinary calls; reuse native handoff checks for OpenCode and OMP's model/effort representations.
- Review one-off planning, return to normal, custom high-effort profile, missing harness mapping, and two differing run choices for each rendered adapter. Add executable regressions only for concrete adapter logic introduced.
- Regenerate affected fixtures and run existing installation, profile, discovery/parity checks and the standard area validation without live model calls.

## Result

Added scoped execution profiles and plan-only work for OpenCode, Pi, and Oh My Pi using strict frozen dispatch specs, Pi native child overrides, deterministic prepared agents, and accurate capability fallbacks.

Validation:

- Independent verification passed from exact snapshot Wb3d6b0ca14928fcf: portable/Pi/prepared-agent cases, all 168 tests, generated parity, fmt, clippy with warnings denied, build, and diff check.

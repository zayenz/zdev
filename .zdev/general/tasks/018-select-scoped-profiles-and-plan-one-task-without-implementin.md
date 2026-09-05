+++
schema_version = 1
id = "general-018"
key = "select-scoped-profiles-and-plan-only-in-codex"
area = "general"
status = "open"
complexity = "advanced"
afk = true
priority = "normal"
blocked_by = ["general-017"]
+++
# Select scoped profiles and plan one task without implementing it in Codex

## Outcome

Codex can choose a profile for an authorized run or one worker role, including planning the next task with an advanced planner and then stopping.

## Context

Read Selection and scope, Plan-only requests, Workflow coverage, and Testing and boundaries in the [execution profiles brief](../../../plans/004-execution-profiles-brief.md). Use the prerequisite's profile resolver. templates/zdev/codex-skill.md currently bakes model settings into worker instructions; shared-contract.md has no standalone task-planning route, and references/implement.md plans only authored advanced work before edits. The current audit and task-draft challenge routes also need explicit verifier selection. Implement the shared selection and plan-only contract here, wire it end to end into Codex, and give later adapters the same contract. Start with src/integrations.rs, task-workflows.md, implementation/verification/recovery and task-drafting references, and existing compact handoff tests.

## Boundaries

- Treat 'plan next task with advanced planner' as read-only planning only, including for routine and standard tasks. Preserve task complexity, default settings, independent verification, and all phase/Git approval boundaries.
- Reuse the existing worker results, context snapshots, and conversation/workflow handoffs. A scoped choice selects a role's settings; it does not create a new agent lifecycle or automatically switch the coordinator.
- Preserve ordinary escalation within the selected profile. Selecting stronger workers does not authorize additional workers, task creation, implementation, or spending outside the selected scope.

## Done when

- [ ] The shared router and Codex integration distinguish profile choice, one-off role choice, and task complexity. At the start of the authorized interaction/run, coordination resolves and retains concrete model/effort values for its roles; it passes those values to later dispatches rather than re-resolving names after shared preferences change. A newly requested one-off choice is resolved for that logical step.
- [ ] The exact request 'plan next task with advanced planner' selects and retains the correct ready task, captures explicit task context, runs a fresh read-only advanced-profile planner, returns its plan and settings, and stops without implementation, verification, lifecycle changes, new tasks, branches, or worktrees.
- [ ] A subsequent explicit implementation request revalidates and reuses an available applicable plan; an unavailable or materially stale plan is explained and handled through normal planning requirements. The earlier planner override does not become the implementation or verifier profile.
- [ ] Run-level choices reach implementation, required planning, verification, audits, task-draft challenge, and any already-requested delegated discussion or investigation; a one-off override reaches only its logical role step and same-step retries.
- [ ] The retained concrete selections survive authorized continuation and are handed to parallel routes when present, without shared-configuration rewrites or a dependency on the parallel feature. Mid-run preference changes do not alter existing selections; a later independent run resolves its own request or saved default.
- [ ] Unsupported model/effort dispatch or an unknown profile reports the requested choice without silent substitution; observable runtime overrides are stated and no claim is made to have changed the coordinating conversation's model.
- [ ] Canonical guidance, route discovery, relevant documentation, and regenerated integrations describe the new shared behavior and Codex support while keeping other adapters' interim support accurate.

## Validation

- Review rendered scenarios for advanced planning only, normal implementation after that plan, role override within a simple run, task-draft review, explicit non-default task identity, stale plan, and unauthorized automatic upgrade. Change shared preferences between planning and implementation/verification and verify that the existing run retains its concrete choices while a later run sees the new settings.
- Use controlled role invocations to check model/effort payloads and phase boundaries without paid model calls; add focused tests only where executable routing logic changes.
- Regenerate affected fixtures, run existing snapshot, worker handoff, route/discovery/parity checks, and run standard area validation.

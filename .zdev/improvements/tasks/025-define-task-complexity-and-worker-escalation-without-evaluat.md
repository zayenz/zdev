+++
schema_version = 1
id = "improvements-025"
key = "design-task-complexity-routing"
area = "improvements"
status = "open"
blocked_by = []
+++
# Define task complexity and worker escalation without evaluation machinery

## Outcome

Produce an implementation-ready contract for cheap default implementation and verification, explicit expensive planning for complex tasks, and verifier-recommended escalation to a stronger implementer.

## Context

Tasks currently have identity, slice, status, and dependency metadata. Worker configuration has only whole-profile implementer and verifier roles. REWORK identifies task-owned defects but carries no machine-readable escalation recommendation.

## Boundaries

- Research and design only.
- Keep independent verification mandatory at every level.
- Do not infer complexity from token counts, file counts, or model self-confidence.
- Add no evaluator, benchmark runner, telemetry, automatic model search, provider catalog, or cost database.
- Keep coordinator policy distinct from implementer and verifier profiles.
- Treat task splitting and derived-task authority as the separate derived-work task.

## Done when

- [ ] The contract settles whether complexity is authored durable metadata, its exact values and default, and backward compatibility.
- [ ] It defines cheap and strong worker-profile vocabulary without partially merging profiles or needlessly expanding the fixed config registry.
- [ ] It defines when planning runs, what artifact it produces, and when it may be skipped.
- [ ] It defines a verifier recommendation separately from PASS, REWORK, and BLOCKER, including envelope compatibility and escalation limits.
- [ ] It defines coordinator routing for first attempt, rework, stronger replacement, and user-decision stops.
- [ ] It maps the smallest changes to task parsing, config, worker resolution, templates, and harness adapters.
- [ ] It produces narrow follow-up implementation tasks and rejects unnecessary variants.

## Validation

- Trace representative cheap, complex, ordinary REWORK, escalation, and product-decision cases through all five harness contracts.
- Check compatibility with existing task files and worker configuration.
- Review current official harness model-control documentation.
- Run documentation validation only; do not run broad model experiments.

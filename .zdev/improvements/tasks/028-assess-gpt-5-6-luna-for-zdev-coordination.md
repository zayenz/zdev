+++
schema_version = 1
id = "improvements-028"
key = "assess-luna-coordinator"
area = "improvements"
status = "open"
blocked_by = []
+++
# Assess gpt-5.6-luna for zdev coordination

## Outcome

Determine whether gpt-5.6-luna is reliable enough for the coordinator role, where it can actually be selected, and what safeguards or fallback are required.

## Context

The coordinator is intentionally not a worker profile. It owns user decisions, task identity, Git safety, dispatch, strict envelope parsing, rework routing, lifecycle mutation, and commits. Some harnesses may not let zdev choose the top-level coordinator model.

## Boundaries

- Study the coordinator only; do not revisit implementer or verifier recommendations.
- Use current official model and harness-control evidence plus a bounded workflow prototype.
- Do not build an evaluation framework, leaderboard, telemetry, cost database, or automated model selector.
- Do not claim runtime-model control where a harness cannot provide it.
- Separate model capability from reductions in workflow ceremony.

## Done when

- [ ] The investigation enumerates the coordinator's reasoning and mechanical duties and identifies the safety-critical ones.
- [ ] It maps whether and how each harness can select a coordinator model independently of workers.
- [ ] It defines a small representative prototype covering clean dispatch, unexplained Git state, a mismatched worker envelope, REWORK routing, and a product decision.
- [ ] It records dated evidence, observed failures, limitations, and confidence.
- [ ] It gives a clear go, no-go, or bounded-use recommendation.
- [ ] If viable, it defines the smallest editable selection and fallback contract; if not, it creates no implementation task.
- [ ] It explains how deterministic tooling can reduce intelligence requirements without moving judgment into unsafe heuristics.

## Validation

- Use current official OpenAI and harness documentation.
- Use reproducible bounded traces with exact inputs and expected coordinator decisions.
- Do not run a broad benchmark; report unavailable model access as a limitation rather than substituting another model.

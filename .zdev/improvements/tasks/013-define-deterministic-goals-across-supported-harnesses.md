+++
schema_version = 1
id = "improvements-013"
key = "goal-contract-research"
area = "improvements"
status = "open"
blocked_by = []
+++
# Define deterministic goals across supported harnesses

## Outcome

A source-backed contract specifies deterministic area-goal output and how each supported harness applies it natively or falls back without duplicating durable zdev state.

## Context

Research current official goal, prompt, command, and session mechanisms in Codex, Claude Code, OpenCode, Pi, and Oh My Pi. Write `docs/harness-goals.md`. The intended binary seam is `zdev goal <area>` with stable human and JSON output derived from existing area, slice, and task records; the document must settle its exact inputs and adapter behavior before implementation.

## Boundaries

- Keep area and task records as the only durable source of goal state.
- Do not persist a second mutable goal record, infer new objectives with a model, or require a harness to expose a native goal feature.
- Clearly distinguish observed harness capability from proposed zdev behavior.

## Done when

- [ ] A dated capability matrix cites official or primary sources for every supported harness.
- [ ] The contract defines the common goal vocabulary, exact record inputs, deterministic ordering, omission rules, and stable human and JSON examples for `zdev goal <area>`.
- [ ] It settles how generated output is applied to native goal mechanisms, skills, prompts, or explicit fallbacks in each harness.
- [ ] It defines precedence when a session already has a native goal and specifies failure behavior without hidden mutation.
- [ ] It ends with implementation acceptance criteria requiring no further product decision.

## Validation

- Check every capability claim and example against current primary documentation.
- Run `git diff --check`.

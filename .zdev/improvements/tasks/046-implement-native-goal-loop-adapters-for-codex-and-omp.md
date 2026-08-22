+++
schema_version = 1
id = "improvements-046"
key = "native-goal-area-loops"
area = "improvements"
status = "done"
blocked_by = ["improvements-044"]
+++
# Implement native goal/loop adapters for Codex and OMP

## Outcome

Codex and OMP expose goal/loop aliases through their supported native continuation mechanisms without disturbing unfinished goals.

## Context

Implement the native continuation slice from docs/area-loop.md for Codex and OMP. Both adapters use the shared one-task iteration and stop contract, and fall back honestly when native continuation is unavailable.

## Boundaries

- Goal and loop are synonyms in both adapters.
- Never replace, clear, edit, or layer over an unfinished native goal.
- Native unavailability falls back to at most one verified committed task and returns CONTINUE when ready work remains.
- Every restart obtains fresh work-context and stores no durable execution state.
- Do not change the binary goal projection.

## Done when

- [x] Codex and OMP install paired native continuation entrypoints with identical stop conditions and alias semantics.
- [x] Unfinished-goal protection, native unavailability, one-task boundaries, and restart behavior are explicit and deterministic.
- [x] Both adapters use configured worker tiers and fresh work-context.

## Validation

- Add focused generated-contract fixtures for alias equality, unfinished-goal protection, native fallback, one-task continuation, restart, and terminal states.
- Install and check Codex and OMP integrations.
- Run the area-wide validation from brief.md.

## Result

Added paired Codex and OMP native goal/loop adapters using model-callable goal operations, unfinished-goal protection, and honest one-task fallback.

Validation:

- Independent verifier PASS after callable-goal rework; focused native adapter tests and full fmt, clippy, test, build, package, and install/check passed.

+++
schema_version = 1
id = "improvements-044"
key = "goal-loop-routing"
area = "improvements"
status = "open"
blocked_by = ["improvements-034", "improvements-043", "improvements-040"]
+++
# Add common goal/loop routing and bounded fallbacks

## Outcome

Goal and loop are exact area-continuation synonyms, with one common contract and honest one-task OpenCode/Pi fallbacks.

## Context

Implement the shared portion of docs/area-loop.md. `zdev-loop <area>` is the canonical name and `zdev-goal <area>` is an exact semantic alias; active-zdev natural language using goal or loop routes identically. The binary's existing `zdev goal` remains the deterministic one-task projection consumed by workflows. This slice also installs the bounded OpenCode and Pi aliases.

## Boundaries

- One iteration implements, independently verifies, completes, and commits at most one task.
- Reconcile fresh work-context at each iteration; store no durable execution state.
- Stop on no-work, unsafe state, malformed worker result, user decision, or failed completion/commit.
- Do not assume every harness has native continuation; bounded fallback is acceptable.
- OpenCode and Pi complete at most one task and return CONTINUE only after a verified commit when ready work remains.

## Done when

- [ ] Canonical skill routing defines goal and loop as exact continuation aliases without shadowing the binary goal projection.
- [ ] The shared stop/continue contract covers ready, closed, empty, exhausted, blocked, unsafe, REWORK, and failure states.
- [ ] Each successful iteration has exactly one selected task and one independently verified commit.
- [ ] Generated adapters expose the names appropriate to their native command systems.
- [ ] OpenCode and Pi install paired aliases with identical one-task boundaries, restart behavior, and exact CONTINUE output.

## Validation

- Add focused shared routing and stop-state contract tests.
- Regenerate and check all harness artifacts.
- Run the area-wide validation from brief.md.

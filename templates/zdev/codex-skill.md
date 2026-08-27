---
name: zdev
description: "Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use when the user invokes zdev or $zdev, names an existing .zdev area or task, or asks to continue stored zdev work."
---

# Zdev for Codex

{{shared_contract}}

## Codex orchestration

The root `$zdev` skill selects the route and loads its contract from
`references/`. Treat “goal” and “loop” as the same native continuation route.

Use one Codex collaboration agent to implement a task and a different agent to
verify it. The coordinating agent owns zdev state, user decisions, task
completion, and commits. Give each agent the rendered repository guidance and
applicable `AGENTS.md` instructions.

Spawn each role with `fork_turns="none"`. Use a compact filesystem-backed
message containing its role and exact area, task, or boundary identity; the
exact installed route-contract path; applicable repository-instruction paths;
authoritative brief, slice, and task paths; and the opaque work-context
snapshot when its route provides one. The agent reads those paths directly and
returns the route's required fields in one JSON object.

For task verification, store and validate the snapshot immediately before
spawning the verifier. Extract one balanced JSON object from its response,
tolerating brief prose or a Markdown fence, validate the four semantic fields,
compare the snapshot, and construct the compatible nine-key verifier envelope
in the coordinating session. Never repeat a worker only to remove valid
wrapping text.

{% if implementer_has_model %}For the implementer, pass `model={{ implementer_model }}`{% if implementer_has_effort %} and
`reasoning_effort={{ implementer_effort }}`{% endif %} together with `fork_turns="none"`.{% else %}Leave the implementer's model and reasoning effort unset so it inherits them.{% endif %}
{% if verifier_has_model %}For the verifier, pass `model={{ verifier_model }}`{% if verifier_has_effort %} and
`reasoning_effort={{ verifier_effort }}`{% endif %} together with `fork_turns="none"`.{% else %}Leave the verifier's model and reasoning effort unset so it inherits them.{% endif %}

For an active-zdev goal or loop request, follow the internal area-loop
contract. It calls `get_goal` before repository work, preserves an unfinished
goal, and calls `create_goal` with the shared condition only when native goal
state is clear. Native unavailability falls back to at most one verified
committed task and returns canonical `CONTINUE zdev-loop <area>` only when
fresh ready work remains.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

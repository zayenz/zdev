---
name: zdev
description: "Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use when the user invokes zdev or $zdev, names an existing .zdev area or task, or asks to continue stored zdev work."
---

# Zdev for Codex

{{shared_contract}}

## Codex orchestration

The root `$zdev` skill selects the route and loads its contract from
`references/`. Treat “goal” and “loop” as the same native continuation route.
Codex supports the explicit **Parallel** route in `references/parallel.md`.
Codex also supports the read-only **Plan one task** route in
`references/plan-task.md`.
The exact installed task-workflows contract path for this installation is
{{ task_workflows_contract_path_json }}. Decode that JSON string and include the
resulting path in every worker payload.

Use one Codex collaboration agent to implement a task and a different agent to
verify it. The coordinating agent owns zdev state, user decisions, task
completion, and commits. Give each agent the rendered repository guidance and
applicable `AGENTS.md` instructions.

For an explicitly chosen assigned source worktree, the shared task-workflows
contract owns isolation and Git transport. Codex collaboration agents share the
parent directory by default, so every source implementer or rework payload must
name the absolute assigned source root and require all file and command tools to
use it as cwd. Keep the authoritative root, record paths, and destination
snapshot paths absolute and distinct. Confirm the agent can access its assigned
root; do not broaden permissions or fall back to concurrent destination edits.
The coordinating Codex session creates and inspects source commits, integrates
them in the destination, and dispatches the verifier against the integrated
destination snapshot.

Spawn each role with `fork_turns="none"`. Use a compact filesystem-backed
message containing its role and exact area, task, or boundary identity; the
exact installed route-contract path; applicable repository-instruction paths;
authoritative brief, slice, and task paths; and the opaque work-context
snapshot when its route provides one. The agent reads those paths directly and
returns the route's required fields in one JSON object.

For plan-only and implementation interactions, run `zdev config profile
dispatch-spec <plan-next-task|implement> --area <area> --task <task-id>
--harness codex --snapshot <snapshot> ... --format json` after storing the explicit task
context. Pass any run choice with `--run-profile`, a one-off choice with
`--role-profile ROLE=PROFILE`, and retained-plan evidence with
`--retained-plan <applicable|stale> --plan-snapshot <snapshot>`. Accept only its
strict `codex-dispatch-spec` object and execute its ordered `dispatches` until
the stated `stop` boundary. Retain the complete concrete specification in the
conversation handoff; same-step retry reuses it instead of resolving again.
The command compares the supplied explicit-task snapshot with current
authoritative context before emitting a dispatch. A stale-snapshot error has an
empty dispatch list; collect fresh explicit-task context and reassess the route
instead of dispatching from historical state.
Pass each emitted `model` and `reasoning_effort` to `spawn_agent`.
If Codex rejects those arguments or reports a substitution, preserve the
requested values in the result and report the rejection or observed values;
do not retry silently with the baked installation setting. The rendered values
below are the installed normal defaults only when no runtime selection was
requested or saved.

When an implementer will author human-facing prose, include the shared `Write
human-facing prose plainly` section in its compact payload. Other workers do not
need that editorial guidance.

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
{% if planner_has_model %}For the planner, pass `model={{ planner_model }}`{% if planner_has_effort %} and
`reasoning_effort={{ planner_effort }}`{% endif %} together with `fork_turns="none"`.{% else %}Leave the planner's model and reasoning effort unset so it inherits them.{% endif %}

For an active-zdev goal or loop request, follow the internal area-loop
contract. It calls `get_goal` before repository work, preserves an unfinished
goal, and calls `create_goal` with the shared condition only when native goal
state is clear. Native unavailability falls back to at most one verified
committed task and returns canonical `CONTINUE zdev-loop <area>` only when
fresh ready work remains.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

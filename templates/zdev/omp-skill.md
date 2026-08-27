---
name: zdev
description: Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use when the user invokes zdev or $zdev, names an existing .zdev area or task, or asks to continue stored zdev work.
---

# Zdev for Oh My Pi

{{shared_contract}}

## Oh My Pi orchestration

The root zdev skill selects the route and loads its contract from `references/`.
It may use the packaged prompts internally for a complete task cycle,
verification, audit, or native area continuation. “Goal” and “loop” select the
same continuation route.
The exact installed task-workflows contract path for this installation is
{{ task_workflows_contract_path_json }}. Decode that JSON string and include the
resulting path in every worker payload.

Route authored routine, standard/default, and advanced work to
`zdev-routine-implementer`, `zdev-implementer`, or
`zdev-advanced-implementer`. Advanced work first uses one blocking read-only
`zdev-planner`.
For its one settled task result, prefer
`details.results[].structuredOutput.data` when it passes semantic validation;
otherwise extract one unambiguous balanced JSON object from that result's
output, tolerating brief prose or a Markdown fence. Invalid planner data blocks
without a retry, revival, formatting follow-up, or extra coordinator pass.
Always verify with a fresh `zdev-verifier`. Return ordinary rework to the
selected profile with `hub` when possible; one valid standard-work escalation
starts an advanced replacement without replanning. The coordinator retains
task completion and commits.

Each agent starts with its short role definition. Give it the installed
route-contract path and a compact task payload: file paths for the brief, task,
guidance, and relevant source; the applicable snapshot IDs; and the short
result from the preceding role. Let the agent read those files instead of
copying their contents or the rendered contract into the prompt.

For an active-zdev goal or loop request, use the packaged continuation prompt. It calls the
model-facing `goal` tool with `op: "get"` before repository work, never drops,
replaces, or layers over an unfinished goal, and calls `op: "create"` with the
shared condition only when native goal state is clear. Native unavailability falls back to at most one verified committed task
and returns canonical `CONTINUE zdev-loop <area>` only when fresh ready work
remains.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

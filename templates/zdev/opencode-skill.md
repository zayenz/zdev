---
name: zdev-opencode
description: "Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use when the user invokes zdev or $zdev, names an existing .zdev area or task, or asks to continue stored zdev work."
compatibility: opencode
---

# Zdev for OpenCode

{{shared_contract}}

## OpenCode orchestration

The exact installed task-workflows contract path for this installation is
{{ task_workflows_contract_path_json }}. Decode that JSON string and include the
resulting path in every worker payload.

Route authored routine, standard/default, and advanced work to
`@zdev-routine-implementer`, `@zdev-implementer`, or
`@zdev-advanced-implementer`. Advanced work first uses one read-only
`@zdev-planner`. Always verify with a fresh `@zdev-verifier`. Ordinary rework
stays on the selected profile; one valid standard-work escalation uses an
advanced replacement without replanning. Include rendered repository guidance
and applicable instructions in every prompt.

Each subagent starts with its short role definition. Give it the installed
route-contract path and a compact task payload: file paths for the brief, task,
guidance, and relevant source; the applicable snapshot IDs; and the short
result from the preceding role. Let the worker read those files instead of
copying their contents or the full contract into the prompt. An implementer
loads the derived-work section only if it actually needs a split.

The root zdev skill selects the route and loads its contract from `references/`.
It may use the packaged commands internally for a complete task cycle,
verification, audit, or bounded area continuation. “Goal” and “loop” select the
same continuation route.

OpenCode has no required native continuation surface. For an active-zdev goal
or loop request, use the packaged continuation command. It completes at most one task using
the ordinary route, returns canonical `CONTINUE zdev-loop <area>` only after a
verified commit when fresh ready work remains, and never claims a continuing
loop was started.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

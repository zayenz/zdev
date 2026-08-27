---
name: zdev-pi
description: Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use when the user invokes zdev or /skill:zdev-pi, names an existing .zdev area or task, or asks to continue stored zdev work.
---

# Zdev for Pi

{{shared_contract}}

## Pi orchestration

The exact installed task-workflows contract path for this installation is
{{ task_workflows_contract_path_json }}. Decode that JSON string and include the
resulting path in every child payload.

Route authored routine, standard/default, and advanced work through
`zdev_subagent` roles `routine-implementer`, `implementer`, or
`advanced-implementer`.
Advanced work first uses one read-only `planner` call. Always use a fresh
`verifier` call for verification. Ordinary rework keeps the selected profile;
one valid standard-work escalation uses an advanced replacement without
replanning. Pi starts each child as a clean, role-specific process. Give every
child the installed route-contract path and a compact payload of brief, task,
guidance, and source file paths, applicable snapshot IDs, and the short result
from the preceding role. The child reads those files from the shared checkout;
do not copy the rendered contract into its prompt.

In Pi, `/skill:zdev-pi` activates this one skill explicitly. The skill selects
the route and loads its contract from `references/`. It may use the packaged
prompts internally for a complete task cycle, verification, audit, or bounded
area continuation. “Goal” and “loop” select the same continuation route. The
coordinating Pi process owns extension tools and delegation; child processes
focus on their assigned role.

Stock Pi has no native continuation surface. For an active-zdev goal or loop
request, use the packaged continuation prompt. It completes at most one task using the
ordinary route, returns canonical `CONTINUE zdev-loop <area>` only after a
verified commit when fresh ready work remains, and never claims a continuing
loop was started.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

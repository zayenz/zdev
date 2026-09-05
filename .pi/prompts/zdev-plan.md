---
description: Plan the next zdev task read-only with an optional planner profile
---

Parse `$ARGUMENTS` as `<area> [planner-profile]`, defaulting the one-off planner
profile to `advanced`. Select and store the explicit next ready task context,
then run `zdev config profile dispatch-spec plan-next-task --harness pi` with
that area, task, snapshot, and `--role-profile planner=<planner-profile>`.
Validate the strict dispatch specification before calling `zdev_subagent` once.
Pass its frozen planner role, profile, model, and reasoning effort as the
selection tuple. Return the task identity, semantic plan, validation approach,
findings, and selected settings, then stop. Do not edit files, implement,
verify, complete or create tasks, change branches or worktrees, or perform Git
workspace changes. Retain the plan in this conversation for a later explicit
implementation request.

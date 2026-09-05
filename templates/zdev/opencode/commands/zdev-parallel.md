---
description: Run one approved finite batch of independent zdev tasks in parallel
---

Load `references/parallel.md` from the installed `zdev-opencode` skill. Parse
`$ARGUMENTS` as the requested area and task IDs so the route is discoverable,
but do not perform admission that creates branches or worktrees.

The exact installed task-workflows contract path is
{{ task_workflows_contract_path_json }}. Decode it and include the resulting
absolute path in every worker payload.

Before any worktree or source mutation, inspect the Task tool description
visible in this session. The pinned native schema can start a fresh foreground
child with `description`, `prompt`, and `subagent_type`, optionally resuming a
child session with `task_id`. A foreground call returns only after that child
finishes. Multiple calls in one assistant message therefore return control to
the coordinator as one tool-call batch, rather than returning one coordinator
turn per completed child. The returned ID identifies an OpenCode child session;
it provides no zdev task identity, result queue, or supported per-child
interruption handle for arrival-driven integration. Experimental background
subagents are outside this integration.

These semantics cannot preserve zdev's requirement to accept candidates as
they arrive, give a newly available role slot to integrated verification, and
stop or retain each worker with confirmed state. Report that the current
OpenCode Task surface does not support this parallel route and offer ordinary
sequential `zdev-implement <area>` runs for the requested tasks. Do not create
branches or worktrees, dispatch a Task, edit source, or imply that a cwd,
directory, background flag, or OpenCode task ID fills the missing controls.

{{repository_guidance}}

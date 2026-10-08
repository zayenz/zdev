---
description: Run one approved finite batch of independent zdev tasks in parallel
---

Load `references/parallel.md` from the installed `zdev-opencode` skill. Parse
`$ARGUMENTS` as the requested area and task IDs so the route is discoverable,
but do not perform admission that creates branches or worktrees.

The exact installed task-workflows contract path is
"/Users/zayenz/.codex-workspaces/worktrees/83c5f3b8-0c89-4e75-b8f6-7831269edd01/zdev/.opencode/skills/zdev-opencode/references/task-workflows.md". Decode it and include the resulting
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

<!-- zdev:generated-repository-guidance:start -->
## Repository guidance discovery

Before inspecting, planning, changing, or validating code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.
<!-- zdev:generated-repository-guidance:end -->

## Coding guidance

Coding guidance file: "/Users/zayenz/projects/zdev/.opencode/skills/zdev-opencode/references/coding.md"

Before planning code, read Design and Code standards from this file. Before implementing or repairing code, also read Completion. Before verification, audit, or code review, read Code standards and Review. Pass this exact location to each relevant worker; read it directly even when the root skill is not loaded. Routine coordination and non-code tasks do not need it.

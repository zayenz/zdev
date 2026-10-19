# Reference

The [Working with zdev guide](areas-and-briefs.md) introduces areas, tasks,
discussion, research, and orchestration before the command details.

Use the [tutorial](getting-started.md) to try the ordinary workflow, or the
[command guide](commands/index.md) to choose a request for your agent. These
pages explain the underlying commands and records in more detail.

## Terminal commands

The [CLI reference](cli/index.md) has a page for each command, with invocation
examples and options. The binary manages records and local Git operations. It does not launch an
implementation just because you run `zdev next`. Planning and agent execution
belong to the installed harness workflow.

| Command | Purpose |
| --- | --- |
| `zdev init` | Initialize the work record and choose its storage policy. |
| `zdev skill` | Install or check a harness integration. |
| `zdev area` | Create, inspect, bind, close, or reopen an area. |
| `zdev tasks` | List, review, and import task bundles. |
| `zdev slice` | Create a short brief for a related increment within an area. |
| `zdev task` | Inspect or update one task and record completion. |
| `zdev status` | Inspect an area's state and branch relationship. |
| `zdev goal` | Inspect continuation state without starting agent work. |
| `zdev next` | Select a ready task without implementing it. |
| `zdev config` | Inspect and change project or worker settings. |
| `zdev check` | Check the planning record for inconsistencies. |
| `zdev work-context` | Collect the current task, branch, and working-tree context. |
| `zdev area rebase` | Start or recover a managed area rebase. |
| `zdev commit` | Commit staged changes with a stable change ID. |
| `zdev change` | Inspect or find a logical change by its stable ID. |
| `zdev research` | Manage local research runs and experiments. |
| `zdev cleanup squash` | Remove a pull-request planning record before squash merge. |

For exact options in your installed version, run:

```sh
zdev --help
zdev area --help
zdev tasks --help
zdev config --help
```

Most inspection commands offer `--format json` for agent use. The
[user guide](user-guide.md) has complete setup and task-import examples.

## Tasks and workflow

- [Workflow](workflow.md) explains selection, independent verification, completion, change IDs, and recovery.
- [Task format](task-format.md) defines task fields, dependencies, priority, and completion conditions.
- [Area lifecycle](area-lifecycle.md) covers opening and closing objectives.
- [Goal and loop](area-loop.md) describes continuation and when work stops.
- [Derived work](derived-work-handoffs.md) explains follow-up tasks and splits discovered during work.

## Branches and configuration

- [Trunk areas](trunk-area-mode.md) explains how several areas can share project trunk.
- [Configuration](config-command.md) lists settings, scopes, precedence, and change commands.
- [Worker profiles](worker-profiles.md) defines model and harness choices for delegated work.
- [Task complexity](task-complexity-routing.md) explains routine, standard, and advanced task routing.

## Harnesses and research

- [Harness support](user-guide.md#harness-notes) describes what each integration can do.
- [Orchestration](harness-orchestration.md) compares native delegation mechanisms.
- [Harness goals](harness-goals.md) covers continuation limits and recovery.
- [T3 Code](t3-code.md) explains explicitly selected coordination across Codex and Claude Code.
- [Research and experiments](research.md) covers measured runs, budgets, and accepted or discarded candidates.
- [Area reports](area-reports.md) describes reports of completed work.

[Upgrade to 2.0](upgrading-to-2.0.md) is for existing installations. Contributor
setup and release operations are in the Project section of the sidebar.

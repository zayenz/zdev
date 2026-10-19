# CLI reference

The `zdev` terminal command manages development records in `.zdev` and their
relationship to Git. Use it to inspect work, create areas, update tasks, and
commit completed changes. The [Agent commands](../commands/index.md) tell a
coding agent to plan, implement, or verify that work.

## Examples

```sh
zdev status greeting
zdev next greeting --format json
```

Examples use an area named `greeting` and, where needed, a task named
`greeting-001`. Create the area and import its tasks before running commands
that refer to them. The [tutorial](../getting-started.md) walks through setup.

## Find a command

| Command | Use it to |
| --- | --- |
| [instructions](instructions.md) | Print the short workflow instructions for an agent. |
| [init](init.md) | Initialize a repository and choose how to store its records. |
| [cleanup](cleanup.md) | Remove pull-request records before a squash merge. |
| [config](config.md) | Read or change project settings and worker profiles. |
| [area](area.md) | Create an objective and manage its branch and lifecycle. |
| [research](research.md) | Run measured experiments and track research limits. |
| [slice](slice.md) | Add a brief for a bounded increment within an area. |
| [tasks](tasks.md) | Review, import, list, and index task files. |
| [task](task.md) | Read, complete, reopen, or record evidence for one task. |
| [next](next.md) | Select ready work. |
| [goal](goal.md) | Render the current continuation goal for an area. |
| [work-context](work-context.md) | Capture task and Git context for a worker. |
| [status](status.md) | Read task counts and branch health. |
| [check](check.md) | Validate records and relationships. |
| [skill](skill.md) | Install or check a coding-harness integration. |
| [commit](commit.md) | Commit staged changes with a stable change ID. |
| [change-id](change-id.md) | Generate a stable change ID. |
| [change](change.md) | Inspect or find commits by their stable ID. |

## Global options

`--root PATH` selects the repository explicitly. Otherwise, zdev discovers it
from the current directory. `--format text` is the default; `--format json`
returns structured output for scripts and agents. Both options work on
subcommands as well as on `zdev` itself.

```sh
zdev --root . --format json status greeting
zdev task done --help
```

Use `--help` at any command level for its full argument list, or `--version`
to identify the installed binary. In syntax examples, uppercase names are
values to supply, brackets mark optional arguments, and `...` means a value
can repeat.

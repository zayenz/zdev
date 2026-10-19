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

## Ask the agent to run a command

You can ask an agent with terminal access to run any CLI example in this
reference. It needs zdev installed and access to the repository, just as you
would. For example, instead of running `zdev status greeting`, ask:

```text
Use zdev to show the status of greeting and explain what is ready next.
```

The agent runs the relevant commands and summarizes their output. It might
report: "One task is done and one is ready. The area's branch is safe for
task work." That reports state; it does not start implementation. You can also
paste an exact command and ask the agent to run it. The same approval and
completion requirements apply when an agent changes records on your behalf.

`zdev goal greeting` still only renders continuation context when run by an
agent. To start execution, ask it to implement a task or loop through the area.

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

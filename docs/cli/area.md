# zdev area

Create and manage an area: one development objective, its brief, and its task
queue. An area either belongs to an isolated branch or follows the project's
configured trunk.

## Examples

Create an area on the current branch:

```sh
zdev area create greeting \
  --title 'Greeting command' \
  --objective 'Greet a named person from the terminal'
```

After its tasks are complete, close it:

```sh
zdev area close greeting
```

## Create and close

```text
zdev area create TAG --title TITLE --objective OBJECTIVE [--branch BRANCH | --trunk]
zdev area close AREA
zdev area reopen AREA
```

Creation writes `area.toml`, `brief.md`, `TASKS.md`, and an empty `tasks`
directory under `.zdev/TAG`. Tags use lowercase letters, digits, and hyphens.
By default, the checked-out branch owns the area. Use `--branch` for an
explicit owner or `--trunk` to follow project trunk.

Closing requires an empty or exhausted task queue. Reopening preserves task
states. See [area lifecycle](../area-lifecycle.md) for completion and reopening.

## Branches and parent areas

| Form | Effect |
| --- | --- |
| `zdev area bind AREA [BRANCH]` | Bind to the named branch, or the current branch when omitted. |
| `zdev area bind AREA --trunk` | Follow the configured project trunk. |
| `zdev area parent AREA PARENT` | Use another area's branch as the base. |
| `zdev area parent AREA --remove` | Return to project trunk as the base. |
| `zdev area rebase AREA` | Rebase the checked-out area branch onto its current base. |
| `zdev area rebase AREA --continue` | Continue after resolving and staging conflicts. |
| `zdev area rebase AREA --abort` | Restore the state before the managed rebase. |

A new rebase requires a clean worktree. Read [branch orientation](../workflow.md#branch-orientation)
and [trunk areas](../trunk-area-mode.md) before changing an area's branch model.

## Execution and research settings

`zdev area profile AREA [HARNESS] [PROFILE]` inspects or assigns an execution
profile. Use `--clear` to remove the configured profile. Profile definitions
are managed by [config profile](config.md#execution-profiles).

`zdev area research AREA` reads research settings. Add `--from PATH` to store
validated JSON settings, or `--from -` to read standard input. This configures
the policy without running an evaluator; [research](research.md) runs experiments.

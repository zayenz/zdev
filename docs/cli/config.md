# zdev config

Read or change project settings and the worker profiles used by coding agents.
The effective view combines repository settings, user settings, and defaults,
so you can see what a worker will actually use.

## Examples

```sh
zdev config show
zdev config set project.default-area greeting
```

## Read and change settings

| Form | Effect |
| --- | --- |
| `zdev config show [--local | --global]` | Read all effective settings, or just one stored scope. |
| `zdev config get KEY [--local | --global]` | Read one setting. |
| `zdev config set KEY VALUE... [--local | --global]` | Write a typed value to a scope. |
| `zdev config unset KEY [--local | --global]` | Remove a value from a scope. |
| `zdev config trunk [BRANCH]` | Set trunk to a named branch or the checked-out branch. |

Project keys include `project.trunk`, `project.default-area`, and
`project.guidance`. Project name and record policy are set at initialization
and are read-only afterward. Worker keys have the form
`worker.HARNESS.ROLE`; their value is either `inherit` or a model and effort
provided together. `t3.verifier-harness` chooses the harness for T3 verifier
workers: `codex` (the default), `claude`, or `inherit`. `workspace.branching`
chooses when agents create branches and worktrees: `on-request` (the default)
keeps work on the current checkout and project trunk, and `isolated` gives new
areas and implementation their own branch or worktree.

Project settings are repository-only. Worker settings, `t3.verifier-harness`,
and `workspace.branching` can be local or global; use an explicit scope when
changing them. `--allow-divergent` on `set` or
`trunk` permits a project-trunk ancestry discontinuity. The
[configuration reference](../config-command.md) defines all keys, default
write scopes, precedence, and validation rules.

## Execution profiles

Named profiles collect role choices for a harness. Inspect available profiles
with `zdev config profile list`, then use `zdev config profile show normal codex`
to see resolved roles.

| Subcommand | Arguments and purpose |
| --- | --- |
| `list` | List built-in and configured names. |
| `show NAME HARNESS` | Show resolved roles in one profile. |
| `set NAME HARNESS ROLE MODEL EFFORT` | Set a role; use `inherit` in place of `MODEL EFFORT` to inherit it. |
| `unset NAME HARNESS ROLE` | Remove a custom role row. |
| `set-default NAME` | Save a default profile. |
| `unset-default` | Clear the saved default. |
| `resolve HARNESS ROLE` | Resolve a concrete role without changing settings. |
| `dispatch-spec ROUTE` | Produce a worker dispatch sequence for a task and stored context. |

These subcommands follow `zdev config profile`. Profile writes default to
repository-local configuration; add `--global` for user configuration.
`resolve` accepts `--profile`, `--run-profile`, and `--area` to model a specific
selection. `dispatch-spec` supports `plan-next-task` and `implement`, and
requires `--harness`, `--area`, `--task`, and `--snapshot`.

See [worker profiles](../worker-profiles.md) for role selection and
[work-context](work-context.md) for stored snapshots. Run
`zdev config profile dispatch-spec --help` for retained-plan and role-override
options.

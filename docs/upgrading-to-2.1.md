# Upgrade to zdev 2.1

Zdev 2.1 changes several defaults: where agents do their work, which Claude
models workers use, how coordination runs inside T3 Code, and how research runs
limit and report work. It also lets a task wait on a task in another area. This
page lists each change and, where a setting exists, how to restore the 2.0
behavior. The [changelog](../CHANGELOG.md) lists every change in the release.

Existing 2.0 areas, briefs, tasks and worker configuration remain valid. The
record schema stays at version 1; there is no record migration or
reinitialization.

## Refresh the binary and integrations

Install the 2.1 binary using your existing installation method. For Cargo:

```sh
cargo install zdev --version 2.1.0 --locked
```

Then refresh each binary-installed harness integration with the same scope and
destination:

```sh
zdev skill install codex --force
zdev skill check codex

# For an existing project installation:
zdev skill install codex --scope project --guidance auto --force
zdev skill check codex --scope project
```

Replace `codex` with your harness. Preserve an existing custom `--to` destination
and guidance selection. The new branching, T3 and research guidance, and the
fix for packaged Claude loops that stopped after every committed task, reach
the harness only through the refreshed integration. Update the Codex plugin through
its normal installation path.

## Branches and worktrees only on request

In 2.0, the harness implemented each task in an isolated worktree unless you
had chosen a workspace. In 2.1, agents create new areas on the checked-out
project trunk and implement single tasks in the current checkout. They create a
branch or worktree only when you ask for one, or when the route needs one:
parallel batches, rolling research and an assigned source worktree. Areas with
pull-request records still get their own branch.

`zdev area create` without `--branch` or `--trunk` now creates a trunk area when
the configured trunk is checked out and the record policy allows trunk areas.
On another branch it still binds the checked-out branch. The `zdev goal` native
condition no longer says "Use worktrees by default"; it keeps the run's
workspace choice and otherwise follows the same setting.

To restore the 2.0 behavior, set `workspace.branching` to `isolated`, either for
one repository or for every repository:

```sh
zdev config set workspace.branching isolated
zdev config set --global workspace.branching isolated
```

Existing areas keep their recorded mode. An explicit `--branch` or `--trunk`,
or a branch or worktree request in the conversation, always wins. See the
[configuration reference](config-command.md).

## Claude Opus 5.5 for every Claude role

Built-in Claude worker defaults now use Claude Opus 5.5 (`claude-opus-5-5`) for
every role and never use `max` effort:

| Role | 2.0 default | 2.1 default |
| --- | --- | --- |
| Routine implementer | `haiku low` | `claude-opus-5-5 low` |
| Implementer | `claude-opus-5 low` | `claude-opus-5-5 medium` |
| Verifier | `claude-opus-5 low` | `claude-opus-5-5 medium` |
| Advanced implementer | `claude-opus-5 high` | `claude-opus-5-5 high` |

The Claude `advanced` profile uses Opus 5.5 at high for standard implementation
and verification, and at xhigh for advanced implementation and planning. The
OpenCode, Pi and Oh My Pi verifiers default to `anthropic/claude-opus-5-5`.
The coordination steps inside Claude's packaged workflows moved from Haiku to
Opus 5.5 at low effort; no setting changes them. These defaults can change your
model usage.

A saved worker choice still takes precedence over the built-in default. To
keep a 2.0 model for a role, save it, for example:

```sh
zdev config set --global worker.claude.routine-implementer haiku low
zdev config set --global worker.claude.implementer claude-opus-5 low
zdev config set --global worker.opencode.verifier anthropic/claude-opus-5 inherit
```

Omit `--global` to save the choice for the current repository only. See
[worker profiles](worker-profiles.md) for the full mapping and resolution
order.

## The T3 route inside T3 Code

In 2.0, zdev used T3 orchestration only when you asked for it. In 2.1, the Codex
and Claude zdev skills use the T3 route by default whenever the session runs
inside T3 Code, recognized by T3's orchestration instructions or its
orchestration MCP server. Implementation, verification, plan-only work,
audits, standalone review and goal/loop then dispatch every worker as a T3
child task. A goal or loop runs in the main thread, which reports the selected
task, each phase result and each commit. Sessions outside T3 Code are
unchanged.

To keep the harness's own subagents or Claude's packaged workflows inside T3
Code, ask for native orchestration explicitly, for example "Use zdev with
native orchestration to loop through area scheduling." The choice lasts for
the whole run. If T3's orchestration tools are unreachable, the coordinator
reports a blocker that names this opt-out instead of switching routes on its
own. The T3 route does not support research runs; ask for native orchestration
to run research inside T3 Code.

Under T3 orchestration, verifier-role workers now default to Codex: task
verification, standalone review, audit and task-bundle challenge run on the
single available Codex instance with the Codex verifier profile, even when the
coordinator runs Claude Code. To keep verifiers on the coordinator's own
provider, as in 2.0, or to prefer Claude Code, set `t3.verifier-harness`:

```sh
zdev config set --global t3.verifier-harness inherit
zdev config set --global t3.verifier-harness claude
```

A harness or model named in your request still wins over this setting. See
[T3 Code](t3-code.md).

## Research run limits and inspection

A research run started with `--time-limit-seconds` and without
`--attempt-limit` no longer gets the default limit of 20 work attempts. Its
dispatch count is advisory, so supporting work cannot end the run early, and
`extend --add-attempts` fails on it. To keep a dispatch cap, pass it
explicitly:

```sh
zdev research run work start --scope "..." --time-limit-seconds 3600 --attempt-limit 20
```

A run with neither a time limit nor an attempt limit still defaults to 20
attempts unless you pass `--until-interrupted`. Run context now reports where
each limit came from and counts measured experiments separately from all
dispatches.

`zdev research run <area> inspect` now returns a compact routine view whose size
does not grow with recorded attempts, observations or tasks. A script that read
the complete run from `inspect` should call `inspect --full` or `context`
instead; both return the complete projection, as do the commands that change
the run. See the [research CLI reference](cli/research.md).

## Blockers in other areas

A task's `blocked_by` may now name a full task ID from another area of the same
project, such as `platform-009`. A task becomes ready only when each blocker is
`done` in the current checkout. Tasks that only name blockers in their own area
behave as before.

When every open task in an area waits on a task in another area, the area's
queue is the new value `blocked`, and JSON output lists each wait under
`cross_area_waits`. Loops, goals and research runs stop there without starting
a worker. Scripts that read `queue` should accept `blocked`.

`zdev task reopen` refuses while a done task in another area depends on the
task. When `zdev area close` refuses because tasks are open, it also lists the
open tasks that block other areas. `zdev task done` and `zdev task reopen`
regenerate the `TASKS.md` of each dependent area and report them in
`dependent_indexes`; commit those files with the task's own changes. See
[workflow](workflow.md) and [area lifecycle](area-lifecycle.md).

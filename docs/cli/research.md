# zdev research

Run measured experiments against an area's research policy and keep track of
results, accepted code, and remaining limits. Research runs can include both
hypothesis tasks and ordinary supporting work.

## Examples

Read the current research run and one experiment checkpoint:

```sh
zdev research run greeting context --format json
zdev research experiment greeting greeting-001 inspect --format json
```

## Before running experiments

Configure the evaluator, correctness commands, and comparison policy with
[zdev area research](area.md#execution-and-research-settings). Import
hypothesis tasks through [tasks](tasks.md). The
[research guide](../research.md) explains the full sequence and record formats.

## Runs

```text
zdev research run AREA COMMAND
```

| Subcommand | Purpose |
| --- | --- |
| `start --scope TEXT` | Authorize generated work within an explicit scope and area policy. |
| `context` or `inspect --full` | Read the complete run: policy, task results, checkpoints, reservations, and remaining limits. |
| `inspect` | Read a compact routine view: stop reason, comparison validity, consumed and remaining limits, active work, and the next action. |
| `admit` | Admit a task bundle under the run's authorization. |
| `reserve` | Charge a work attempt before dispatch. |
| `settle` | Settle completed supporting work. |
| `stop` or `resume` | Interrupt or resume the same authorization and allowance. |
| `extend` | Add explicitly granted allowance without resetting costs or unfinished work. |
| `block` or `resolve` | Record a shared environment failure or its resolution. |
| `rebaseline` | Measure current accepted code with correctness checks and evaluator. |
| `end` | End the authorization after reservations and checkpoints are settled. |

`start` accepts `--attempt-limit`, `--command-limit`, `--command-seconds`,
`--time-limit-seconds`, and `--target`. Without `--attempt-limit`, the default
is 20 work attempts; `--until-interrupted` or `--time-limit-seconds` removes
that default, so a time-only run has no dispatch cap. `--worker-limit` and
`--measurement-limit` both default to one. The repeatable `--estimated <limit>`
marks a supplied limit as the coordinator's own estimate, for example
`--estimated command-seconds`; it refuses a limit whose flag was not supplied.

Run context reports each limit under `limits` with its `value` and `origin`:
`user` for a supplied flag, `default` for a product default,
`coordinator_estimate` for a value marked with `--estimated`, and `unrecorded`
for runs started before origins were recorded. `extend` keeps a limit's origin
and cannot add attempts to a run without an attempt limit. `consumed` reports
`work_attempts` for every dispatch, `dispatches_by_kind` with a count for each
reservation kind, `experiments` for the dispatches that measured a candidate,
and `work_attempts_advisory`, which is true when no attempt limit applies.

`inspect` reports the run ID and scope, `stop_reason`, comparison validity and
invalidation, the accepted baseline aggregate without samples, `limits`,
`consumed` and `remaining`, unsettled reservations, unfinished checkpoints, pending
admissions, the acceptance-lane task, ready and blocked task counts, the next ready task, `unreachable_minimum_gain`, any
`cross_area_waits`, and `next_action`. Its size does not grow with recorded
attempts, observations or tasks. `context`, `inspect --full`, and every command
that changes the run return the complete projection.

Run operations that change existing authorization use the returned run and
request identities where required. For their exact arguments, use, for
example, `zdev research run greeting reserve --help`. Resume preserves the
current authorized limits and consumed resources.

## Experiments

```text
zdev research experiment AREA TASK COMMAND
```

For a local hypothesis, `begin --path PATH` records the clean baseline and
owned paths before editing. `evaluate` executes correctness checks and the
evaluator and records evidence. `finalize --decision DECISION --lesson TEXT`
keeps or restores code, completes the task, and commits the exact paths.
Decisions are `keep`, `discard`, `failed-to-run`, or `inconclusive`; keeping a
candidate requires fresh independent verification through `--verification`.

`inspect` reads a checkpoint. `recover` recovers recorded measurements or
resumes an already authorized finalization. Rolling work also uses `admit`,
`prepare`, `return`, `reconcile`, and `integrate` to manage source worktrees
and destination verification. Each has its own `--help`; the
[rolling research guide](../research.md#rolling-research-in-codex-and-claude-code)
explains the required ordering.

## Measure and compare records

```text
zdev research measure AREA --starting-revision REV --candidate-revision REV --checks PATH
zdev research compare AREA --from PATH
```

`measure` runs the area's evaluator and emits a task-compatible observation.
`--checks` supplies a JSON array of correctness-check attestations. `compare`
compares the latest recorded observation with current area settings. Both
input options accept `-` for standard input.

Use [task research](task.md#research-evidence) to append supplied evidence to a
task. For managed experiments, follow the begin, evaluate, and finalize path
so code decisions and measurements stay coordinated.

## Extend an existing authorization

To add an explicitly granted allowance to an existing run:

```sh
zdev research run work extend --run <run> --request another-day \
  --add-time-seconds 86400 --add-attempts 20 --format json
```

The additions increase finite limits without resetting costs or unfinished work.
Use the same request identity for retries. See
[authorization extension](../research.md#extend-an-authorized-run) for the
deadline, retry, and recovery rules.

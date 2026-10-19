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
| `context` or `inspect` | Read policy, results, unfinished checkpoints, and remaining limits. |
| `admit` | Admit a task bundle under the run's authorization. |
| `reserve` | Charge a work attempt before dispatch. |
| `settle` | Settle completed supporting work. |
| `stop` or `resume` | Interrupt or resume the same authorization and allowance. |
| `extend` | Add explicitly granted allowance without resetting costs or unfinished work. |
| `block` or `resolve` | Record a shared environment failure or its resolution. |
| `rebaseline` | Measure current accepted code with correctness checks and evaluator. |
| `end` | End the authorization after reservations and checkpoints are settled. |

`start` accepts `--attempt-limit`, `--command-limit`, `--command-seconds`,
`--time-limit-seconds`, and `--target`. The default is 20 work attempts;
`--until-interrupted` removes that default. `--worker-limit` and
`--measurement-limit` both default to one.

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

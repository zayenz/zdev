# Research and experiments

For an introduction, read [Research toward an open-ended goal](research-walkthrough.md).
This reference covers the execution and record details.

Research areas can mix ordinary tasks and hypothesis tasks. Configure an open
area with `zdev area research <area> --from <settings.json>`, then review/import
hypotheses through the normal task-bundle workflow. Append measured evidence
with `zdev task research <area> <task> --from <result.json>`; omit `--from` to
inspect settings or task evidence. The
[research record contract](../skills/zdev/references/task-format.md#research-settings-and-hypothesis-evidence)
contains complete JSON formats and examples.

Hypotheses keep the normal `open`/`done` status. Interrupted evidence leaves a
hypothesis open. Accepted, rejected, inconclusive, and failed-to-run evidence
have distinct completion rules. A valid rejected experiment can complete
without improving the metric. Completion does not accept candidate code or
close the area. These commands validate supplied evidence without launching
evaluators, retaining/restoring code, or proving recorded Git attestations.
Codex and Claude Code install separate serial research routes using the managed local
experiment commands. Ordinary Implement, Plan, Verify, Parallel and Loop stop
before hypothesis dispatch; ordinary supporting tasks retain their workflow.

## Serial research in Codex and Claude Code

First configure a research policy with actual baseline samples and a shell
command for every required correctness check. The brief defines allowed changes,
experimental reasoning and measurement policy. Then explicitly authorize the
installed skill, for example:

```text
$zdev run serial research in work within the brief, for 20 work attempts
$zdev run serial research in work until interrupted, with target 5 and command limit 100
$zdev resume the existing research run in work
```

State the scope and any attempt, command, cumulative command-time or wall-time
limits. The default run has 20 coordinator work attempts. `until interrupted`
removes that default; supplied limits and the directional target still apply.
This authorization allows the coordinator to generate both supporting tasks
and hypotheses inside the brief and frozen policy without reviewing every new
bundle. Ordinary task creation outside a run uses exact bundle approval by
default; you can delegate creation and import without confirmation.

The coordinator reads the accepted baseline and prior rejected, inconclusive
and failed lessons before proposing work. It commits admitted task records
before candidate preparation. Supporting tasks use ordinary planning,
implementation, independent verification and completion. Hypotheses use local
`begin`, `evaluate` and `finalize`: the binary runs correctness/evaluator
commands, records samples and costs, retains candidate Git references, and
keeps or restores exact owned files. Only a proposed keep gets a fresh
independent acceptance verifier; the coordinator checks discarded and terminal
failed results. Each completion has its own commit and leaves the area open.

Preparation, supporting work and same-task rework, repair, rerun, confirmation,
reconciliation and actual rebaseline consume allowance before work starts.
Every correctness/evaluator invocation counts before launch. Failure and
interruption receive no refunds. Workers use resolved concrete model/effort
settings retained for the run. Accepted supporting code or changed comparison
conditions require a counted actual rebaseline. A shared environment blocker
halts affected experimentation but permits counted in-scope supporting repair;
resolution also requires rebaseline.

Rebaseline normally refuses tracked changes and untracked files outside `.zdev`.
When commands keep writing saved measurements to the same place, declare that
place once as a generated output in the research settings (see
[Declare generated outputs](#declare-generated-outputs)). For a one-off output,
name its path on the command instead, for example `research run work rebaseline
--run RUN --request baseline-1 --untracked-output results`. Repeat
`--untracked-output` for additional repository-relative files or directories;
these add to the declared outputs for that request only. Either way, zdev
preserves those outputs without staging or ignoring them; other untracked paths
and all tracked changes still block measurement. Only name generated output that
does not change the code being measured. This option is not evidence of
evaluator independence.

Inspect durable progress with:

```sh
zdev research run work context --format json
zdev research experiment work work-001 inspect --format json
zdev research experiment work work-001 recover --format json
```

A new session recovers admissions, reservations, unfinished measurements and
finalization before new dispatch. Resume retains the original run ID, scope,
target, deadline and consumed resources. Unresolved measurements are not
relaunched. Task records preserve measurements and lessons, and Git preserves
accepted code and rejected candidates. If the old conversation lacks concrete
worker choices, the coordinator explains that and resolves fresh choices before
dispatch.

Codex uses the actual exposed native goal tools for continuation. It preserves
another unfinished goal. If native inspection proves clear but creation is
unavailable, it performs at most one foreground work-item iteration and states
that limit; uncertain native state blocks. It never claims background execution
or completes an unfinished objective because of a transient blocker or empty
queue. A queue with no ready work permits more scoped proposals while allowance
remains. Target, budget, interruption and shared blocker stop distinctly; none
implicitly closes the area or proves its optimization objective achieved.

The [installed Codex contract](../skills/zdev/references/research.md) gives the
ordered coordinator operations. The [installed Claude contract](../.claude/skills/zdev/skills/zdev/references/research.md)
uses `zdev-research` with structured authorization, a fresh invocation identity
and retained concrete profiles. Pause/continue of a live workflow is supported.
After stop or failure, use a new resume invocation with the same durable run;
replaying a stopped script can replay a cached reservation before restarting a
worker. When packaged execution is unavailable, complete at most one foreground
work-item iteration and report the required continuation. Claude does not use
its separate `/goal` command for this route. Other harnesses expose research
records and local commands without this installed execution route. Deterministic
installation and command scenarios validate the contracts; authenticated live
Codex and Claude acceptance remain separate.

## Extend an authorized run

When the user grants more allowance, extend the same run rather than ending it
and starting another. For example:

```sh
zdev research run work extend --run <run> --request another-day \
  --add-time-seconds 86400 --add-attempts 20 --format json
```

Supply only the additions the user authorized. `--add-commands` and
`--add-command-seconds` increase the corresponding finite limits. Time is added
to the existing deadline, including when it has expired. Unlimited limits do
not need extending. Additions must be positive; counts cannot overflow and time
must remain finite.

Retain the request identity: repeating it with the same additions has no extra
effect; changing its additions is rejected. The run keeps its identity, scope,
policy, target, consumed costs, active reservations, and checkpoints. An
interruption or shared blocker remains in place. Resume or resolve it through
the existing recovery route before dispatching more work.

## Amend a correctness command path

An active run freezes its correctness commands. When a file that a command names
moves or is renamed and the check keeps its meaning, declare the change
administrative instead of ending the run. For example, after editing the path in
the settings file:

```sh
zdev area research work --from settings.json \
  --administrative "Admission files moved to data/admission" --format json
```

zdev accepts the settings only when they differ from the run's frozen policy in
the command text of existing checks. Adding, removing or renaming a check, or
changing any other field, is refused. So is an amendment while an experiment,
measurement, rebaseline or acceptance is unfinished, or when the area has no
active run. The run keeps its ID, consumed costs and comparison validity, and
`research run work context` lists each amendment's reason and before and after
commands under `run.amendments`. zdev runs nothing during configuration; the
next managed evaluate or rebaseline runs the amended command.

A changed horizon, workload, solver or acceptance rule inside a command is a
material change, not an administrative one. It needs a new study and an actual
measured baseline.

## Declare relevant inputs

By default, any commit that changes a path outside `.zdev` after the accepted
baseline invalidates comparisons and blocks `begin`, even a documentation edit.
To narrow this, list the files and directories that the evaluator, workload and
correctness commands depend on in the research settings:

```json
"relevant_inputs": ["src", "bench/workload.txt", "evaluate"]
```

Entries are literal repository-relative paths outside `.git` and `.zdev`; a
directory covers everything under it. Globs are not supported. When in doubt,
list more: a missing input lets a real change go unnoticed. Every owned
candidate path must lie inside the list, so `begin` and isolated `admit` refuse
a `--path` outside it. Spell each entry exactly as Git tracks it, including
letter case, even on a case-insensitive filesystem; a differently cased entry
matches nothing.

An active run freezes the list. To adopt or change it, end the run, configure
the new settings and start a new run. Renaming a declared input also needs a
new run with an updated list, because changes at the new name fall outside the
frozen list. When a declared input changes, the
recorded `run.invalidation` names a changed path, and an actual counted
rebaseline is required as before. Reconciliation of isolated candidates still
compares the whole repository, so a destination that advanced only outside the
declared inputs still reconciles and remeasures the candidate.

## Declare generated outputs

Evaluators often write captures into the repository. By default, any untracked
file outside `.zdev` blocks rebaseline and the return or reconciliation of an
isolated candidate. To keep captures where they are, list the paths that
commands write generated output to:

```json
"generated_outputs": ["results", "logs/evaluator"]
```

Entries follow the same path rules as `relevant_inputs`. Only untracked files
under these paths are tolerated. Rebaseline, and isolated return, evaluate,
reconcile and integrate, then leave those files in place. Tracked or staged
changes under these paths still refuse, as do untracked files anywhere else and
any `.zdev` edit. zdev never stages, cleans or deletes declared outputs, and
work-context snapshots still include them. No directory counts as output just
because of its name, such as `results`; only declared paths do.

Outputs must never contain anything a measurement consumes. When
`relevant_inputs` is declared, `zdev area research` rejects an output that
equals, contains or lies inside a relevant input, ignoring letter case. `begin`
and isolated `admit` refuse an owned `--path` under an output. Isolated source
admission still requires a clean new source worktree.

Changing `generated_outputs` during an active run invalidates comparisons until
an actual counted rebaseline, like other evaluator or workload changes. It is
never an administrative amendment. An isolated candidate admitted before the
change keeps its earlier outputs until reconciliation adopts the new settings,
so move untracked files under newly declared outputs out of its source before
reconciling. If accepted code changes a tracked path that overlaps a declared
output (the output itself, a path under it, or a replaced ancestor, compared
without letter case) while an isolated source holds untracked files under that
output, reconcile refuses rather than let Git overwrite them. Move the source's untracked output, then run `recover`.


## Rolling research in Codex and Claude Code

Explicitly request rolling work and its capacities:

```text
$zdev run rolling research in work within the brief, with two workers and one measurement slot, for 20 work attempts
$zdev resume the existing rolling research run in work
```

This authorizes run-owned source worktrees within those limits. Ordinary Parallel
keeps its finite batch authorization. Each native planner, implementer, verifier
and delegated finding checker consumes a counted `reserve --worker-slot` claim.
The coordinator uses the tighter run/native capacity, refills when a role finishes
while siblings remain active, and starts each source from current accepted HEAD.
One measurement slot permits overlapping source preparation and one evaluator.

Returning sources enter one destination acceptance lane. Stale candidates
reconcile and remeasure. The owner gets a counted confirmation role for its fresh
destination verifier. Checked rework uses a counted Prepare to withdraw only
that candidate's integrated code to accepted HEAD before source edits. Unrelated
destination changes and historical evidence survive. Recovery finishes recorded
withdrawal without authorizing another worker; repair needs new measurements,
integration and independent verification.

Budget, target, interruption, lost capacity and shared failure stop affected
dispatch. Resume preserves costs and deadlines and rechecks actual worker/process
ownership. Unknown workers and measurements keep their slots. Shared conditions
or accepted supporting code require an actual counted rebaseline before paused
candidates reconcile and measure. Required advanced plans and concrete worker
settings remain unchanged through rework and recovery; missing evidence stops
that candidate. Cleanup requires a stopped, durably completed, run-owned source
and retained cleanup authority.

Claude uses its installed zdev-research workflow with execution rolling,
worker_limit and measurement_limit at start. Serial remains the default. Resume
the same durable run with a fresh invocation_id, execution, cleanup,
frozen_profiles, retained_plans and retained_candidates from the prior report.
All native roles consume worker-slot claims; asynchronous command mediation
lets one actual evaluator overlap source preparation. The workflow also counts
all native calls, including mediation and composed ordinary helpers, toward
Claude's documented 1,000-agent workflow cap. It stops refill at 900 calls and
retains room to drain and report. Continue with a fresh invocation; stopped-script
replay cannot grant fresh dispatch permission. Deterministic tests exercise
installed workflows against local public commands. Credentialed Claude
execution remains separate live acceptance.


# Research and experiments

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
If accepted code is clean but saved measurements remain untracked, declare their
output path explicitly, for example `research run work rebaseline --run RUN
--request baseline-1 --untracked-output results`. Repeat `--untracked-output`
for additional repository-relative files or directories. This preserves those
outputs without staging or ignoring them; other untracked paths and all tracked
changes still block measurement. Only name generated output that does not change
the code being measured. This option is not evidence of evaluator independence.

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


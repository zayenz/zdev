# Task format

## Project and area metadata

`.zdev/config.toml` records project `trunk`. An isolated
`.zdev/<area>/area.toml` stores its `branch`; optional `parent` and
`base_commit` describe its managed base relationship. An explicit trunk area
stores only `mode = "trunk"` and dynamically resolves its branch from
`project.trunk`; it stores no branch, parent, or base commit. Several areas may
share a branch, including areas with different modes. Never infer mode from a
branch name.

Use the default create form or `--branch` for isolated work. Use `--trunk` only
when the user explicitly wants a personal/project area on configured trunk;
pull-request records remain isolated. Trunk work requires the resolved trunk
checked out and safe, but never requires freshness or rebase ceremony.

Task dependencies never cross areas. Use an area parent for branch ancestry
and `blocked_by` for task sequencing within one area.

## Research settings and hypothesis evidence

A research area mixes ordinary tasks with hypothesis tasks. `open` and `done`
still describe task completion. A hypothesis decision describes the experiment;
it does not change the area's lifecycle or replace task dependencies.

The local experiment path executes correctness checks and evaluator commands,
records samples, retains candidate Git code, and keeps or restores owned files
before task completion and commit. Directional thresholds and an optional exact
paired sign test govern acceptance. `research measure` is a standalone utility
that repeatedly runs evaluator commands. `compare` and `task research` validate
supplied evidence. During an active run, managed `evaluate` owns measurement
and accounting; standalone measurement and supplied observations cannot bypass it.
{% if research_supported %}The separate [installed research route](research.md) coordinates authorized
runs, generated work and this lifecycle. Codex also supports explicitly
authorized rolling source worktrees; Claude remains serial.
{% else %}This harness has no installed serial or parallel research execution route.
{% endif %}Ordinary implementation, planning, and verification dispatch reject hypotheses
before emitting workers. Ordinary supporting tasks keep their existing routes.
Read this contract when a selected task has `hypothesis` frontmatter; do not
send it through the ordinary implementation or parallel workflow.

### Configure one authoritative area policy

Use the area's checked-out safe branch and an open area:

```sh
revision=$(git rev-parse HEAD)
cat > research-settings.json <<EOF_SETTINGS
{
  "comparison": {
    "metric": "elapsed milliseconds",
    "direction": "minimize",
    "evaluator_identity": "fixed-workload-v1",
    "conditions": "fixed input; one measurement slot; warm cache",
    "baseline_revision": "${revision}",
    "baseline_samples": [10.0, 12.0],
    "repetitions": 2,
    "aggregation": "median",
    "minimum_gain": 1.0,
    "required_checks": ["equivalent output"]
  },
  "evaluator_command": "./evaluate-fixed-workload",
  "permitted_changes": ["src/cache.rs"],
  "attempt_limit": 2,
  "timeout_seconds": 60.0
}
EOF_SETTINGS
zdev area research work --from research-settings.json
zdev area research work --format json
```

Replace the illustrative baseline samples with measured values before using
them. `--from -` reads JSON from standard input. Omitting `--from` inspects the
stored settings. These commands never create commits or run the evaluator.

The `research` table in `.zdev/<area>/area.toml` is the sole executable policy.
The brief supplies the objective, experimental reasoning, and shared boundaries.
All JSON objects reject unknown fields. Settings require the shown fields except `timeout_seconds`, which defaults to 60:

| Field | Rule |
| --- | --- |
| `comparison.metric` | One non-empty scalar metric name, including its units. |
| `direction` | `minimize` or `maximize`. |
| `evaluator_identity` | A stable identity for the evaluator version. A changed command requires a new identity. |
| `conditions` | A non-empty identity/description of the workload and comparison conditions. Change it when those conditions change. |
| `baseline_revision` | Full lowercase 40- or 64-digit Git object ID. |
| `baseline_samples` | Finite numeric values, exactly `repetitions` entries. |
| `repetitions` | Positive count; use one for a deterministic metric. |
| `aggregation` | `mean` or `median`; median averages the two middle values for an even count. |
| `minimum_gain` | Finite nonnegative absolute gain in the metric's units. Acceptance also requires strictly positive gain. |
| `required_checks` | At least one unique non-empty correctness check name. |
| `significance` | Optional `{"method":"paired_sign","alpha":0.05}`; see the assumptions below. |
| `evaluator_command` | Non-empty command; only the area setting supplies this command. |
| `permitted_changes` | At least one unique non-empty scope boundary. These are human-reviewed boundaries, not a path sandbox. |
| `attempt_limit` | Positive finite limit per hypothesis. Each observation counts as one attempt. |
| `timeout_seconds` | Positive finite timeout per correctness or evaluator invocation, in seconds; defaults to 60. |
| `correctness_commands` | Optional map from required check names to shell commands. Managed experiments require a command for every required check. |

Fields below `comparison.metric` through `significance` in the table also
belong to `comparison`. Each candidate preparation or repair attempt, measurement
rerun, confirmation, and reconciliation consumes an attempt; record its
observation even if it fails. `elapsed_seconds` records its cost. The managed
local experiment reserves attempts and records command invocation counts.
Outside an active run, `research measure` repeatedly executes evaluator commands;
`compare` and `task research` validate supplied evidence. Active runs use managed
`evaluate` for measurement and accounting. Run-wide limits and authorization
belong to the research run commands below.

A changed evaluator, workload, or other comparison condition requires a newly
measured baseline under those conditions. Reconfigure the area with that full
policy. Historical task evidence stays readable; it never becomes an alternate
source of evaluator commands. An unfinished hypothesis cannot complete with
evidence that differs from the current comparison or attempt limit.

### Review and import a mixed task bundle

Add `hypothesis` to an ordinary version-1 task draft:

```json
{
  "schema_version": 1,
  "area": "work",
  "tasks": [
    {
      "key": "setup",
      "title": "Prepare the fixed workload",
      "outcome": "The evaluator uses a fixed representative input.",
      "done_when": ["The workload and correctness check are available."],
      "validation": ["Exercise the fixed input."]
    },
    {
      "key": "cache",
      "title": "Measure caching repeated parsing",
      "outcome": "Retain a measured cache comparison and lesson.",
      "blocked_by": ["setup"],
      "done_when": ["Retain valid terminal evidence and a lesson."],
      "validation": ["Use the configured evaluator and correctness check."],
      "hypothesis": {
        "mechanism": "Caching removes repeated parsing from the hot path.",
        "context_tasks": ["setup"]
      }
    }
  ]
}
```

Save the bundle as `tasks.json`, review it with `zdev tasks review work --from
tasks.json`, and import its exact approved identity with `zdev tasks import
work --reviewed <review-id>`. Complete the ordinary setup task after its checks
pass, for example `zdev task done work work-001 --summary "Prepared the workload"
--validation "Fixed workload checked"`, before recording the blocked hypothesis. The review includes the hypothesis mechanism and
context links, and its fingerprint covers them. Hypotheses require configured
research settings. There is no separate experiment ledger.

`hypothesis.mechanism` is required and non-empty. `context_tasks` defaults to
`[]`; links may name existing IDs or keys from this bundle and resolve to durable
same-area IDs on import. Unknown, duplicate, and self links are rejected.
Context links do not block work. `blocked_by` remains the dependency list;
an observation's `starting_revision` records the actual code starting point.
These three relationships serve different purposes.

`tasks list`, `task show`, `next`, `goal`, and full `work-context` identify
hypotheses in text and JSON. Goal/status expose the area's settings; goal and
task projections expose task-owned evidence. Absent research fields are omitted
from ordinary records and projections.

### Collect and compare measurements

`zdev research measure` runs the area's `evaluator_command` through `sh -c` in
`--root`, once per configured repetition. Standard input is closed. Each
invocation receives `ZDEV_REPETITION`, a one-based index; paired noisy studies
must use this index to select the same predeclared block/seed as the baseline.
The command must print exactly one finite numeric scalar on stdout (up to 4096
bytes). Whitespace around the number is allowed. Diagnostics belong on stderr;
they are not retained by this utility. Evaluators may create their own files.

```sh
printf '[{"name":"equivalent output","passed":true}]' > checks.json
zdev research measure work --starting-revision "$revision" --candidate-revision "$(git rev-parse HEAD)" --checks checks.json --format json > measurement.json
```

`--checks -` reads a JSON array from stdin. Checks attest to correctness tests
already run by the caller; the utility does not execute or prove those checks,
revision identities, or physical comparison conditions. The caller must supply
truthful facts. Missing or failed required checks and a starting revision that
differs from the baseline make the assessment unusable.

The output contains `comparison`, a task-compatible `observation`, `summary`,
and `assessment`, plus `schema_version` and `area`. Copy `comparison` and put
`observation` in the owning task result's `observations` array. `summary` reports
mean, median, minimum, maximum, and `spread` (maximum minus minimum).
`assessment` reports `usable`, `improved`, directional `gain`, and, when selected,
the recomputed significance test. These derived values are not another ledger;
raw samples and comparison policy remain the stored evidence.

Collection stops at the first failure and retains finite earlier samples.
Nonzero exits/launch failures produce `crashed`, timeout produces `timed_out`,
and empty, malformed, nonfinite output or nonfinite summary/gain produces
`invalid`. Failures contain a description; summary is null and improvement is
false. Valid JSON evidence, including failure, exits successfully; command/input
errors exit unsuccessfully. `elapsed_seconds` includes collection and waiting;
`evaluation_runs` counts invocations attempted before stopping, so failed work
also consumes resources. On Unix each invocation kills remaining descendants in the evaluator's process
group when its shell exits, or kills the group and reaps the shell on timeout.
Processes that detach into another group are outside that cleanup. Evaluators
must finish their work before exiting.

`zdev research compare work --from result.json --format json` checks a result
record's comparison and attempt allowance against current area policy, then
summarizes and assesses its latest observation. It rejects changed evaluator,
workload, baseline, or policy facts. Both utilities leave task records and Git
state alone. They do not retain candidates, restore code, or run a research loop.

### Optional exact paired sign test

Set `comparison.significance` to `{"method":"paired_sign","alpha":0.05}` only
when the baseline and candidate samples are naturally paired by index. The
method requires 2 through 1024 repetitions and finite `0 < alpha < 1`. Omitting
it uses only the directional aggregate and minimum gain. Deterministic metrics
can use one repetition without inference.

For each pair, compute baseline minus candidate when minimizing, candidate
minus baseline when maximizing, then subtract `minimum_gain`. Positive
adjusted differences are wins. Exact zero differences are ties and are excluded.
With `n` non-tied pairs and `w` wins, the one-sided p-value is
`P(Binomial(n, 0.5) >= w)`. All ties give p = 1 and fail the test. The reported
`pairs`, `ties`, `wins`, `p_value`, `alpha`, `method`, and `passed` explain the
calculation. Passing requires p <= alpha. Acceptance still requires strictly
positive aggregate gain meeting the threshold, comparable evidence, correctness
checks, retained code, and independent review.

The null is that, conditional on a non-tie, positive and negative adjusted
differences are equally probable; the alternative favors positive differences.
Assume independent pairs with the same sign probability and a predeclared
pairing, threshold, sample count, and alpha. With continuous differences this
is a test of median paired gain at the threshold. It is not a confidence
interval or a guarantee about expected mean gain. For quantized data, excluding
ties tests the conditional sign probability, not the probability of exceeding
the threshold over all runs. Do not interpret it as the latter, and do not use
it to manufacture confidence for a deterministic metric.

This follows the [NIST Dataplot sign-test reference](https://www.itl.nist.gov/div898/software/dataplot/refman1/auxillar/signtest.htm).
Its worked example has 3 positive, 4 negative, and 3 tied differences: the
one-sided upper-tail probability is 99/128 = 0.7734375 (printed as 0.7734).
Focused tests also check ten wins (1/1024), all ties, directions, and thresholds.
Run `cargo test --locked research_` to exercise those samples and deterministic
evaluator failure fixtures. This small exact method makes no normality or
symmetry assumption, loses magnitude information, and may have low power.
Repeated search, adaptive reruns, and selecting promising candidates can inflate
false positives; the area must define confirmation or held-out procedures when
needed. A p-value alone establishes neither correctness nor generalization.

### Append observations and a decision

Use `zdev task research work work-002 --from result.json` to append one strict
JSON record. Use the same command without `--from` to inspect the declaration
and ordered result history. Writes require a ready open hypothesis on its safe
area branch. For the policy above, a rejected record looks like:

```sh
cat > result.json <<EOF_RESULT
{
  "comparison": {
    "metric": "elapsed milliseconds",
    "direction": "minimize",
    "evaluator_identity": "fixed-workload-v1",
    "conditions": "fixed input; one measurement slot; warm cache",
    "baseline_revision": "${revision}",
    "baseline_samples": [10.0, 12.0],
    "repetitions": 2,
    "aggregation": "median",
    "minimum_gain": 1.0,
    "required_checks": ["equivalent output"]
  },
  "attempt_limit": 2,
  "observations": [
    {
      "starting_revision": "${revision}",
      "candidate_revision": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
      "outcome": "measured",
      "samples": [12.0, 14.0],
      "checks": [{"name": "equivalent output", "passed": true}],
      "elapsed_seconds": 0.5
    }
  ],
  "decision": "rejected",
  "lesson": "The cache overhead outweighs parsing saved on this workload.",
  "retained_candidate": "refs/experiments/cache",
  "restored_revision": "${revision}"
}
EOF_RESULT
zdev task research work work-002 --from result.json
zdev task research work work-002 --format json
zdev task done work work-002 --summary "Rejected the cache hypothesis" --validation "Comparison and restoration recorded"
```

Replace the illustrative candidate revision and retention reference with the
actual retained candidate. Record observations before restoring candidate code.
Keep the candidate in accessible Git history or a retained reference. Restore
only its code changes; preserve the hypothesis, observations, lesson, and
unrelated local changes. The binary validates these supplied records and does
not perform or independently prove those Git operations.

Every result requires `comparison`, `attempt_limit`, `observations`, `decision`,
and a concise non-empty `lesson`. `comparison` records historical facts using
the area comparison shape, and must equal current settings when appended.
`attempt_limit` records the current allowance. The task frontmatter stores an
ordered `research_results` array; each append preserves all prior records.
Observations are attempts, and samples within one observation are repetitions.
Sum observation counts across the task for consumed attempts. Sum their finite,
nonnegative `elapsed_seconds` for recorded elapsed cost. Optional
`evaluation_runs` counts attempted evaluator invocations, including a failed
launch. It must cover retained samples, not exceed repetitions, and equal
repetitions for measured observations. Older observations may omit it; their
invocation cost is unknown, rather than zero.

Each observation requires the two full revision IDs, `outcome`, `samples`,
`checks`, and `elapsed_seconds`. Check objects contain exactly `name` and
`passed`; names are unique. A `measured` observation requires exactly
`repetitions` finite samples and forbids `failure`. `crashed`, `timed_out`, and `invalid`
require a non-empty `failure` description; they may preserve finite partial
samples but never count as usable comparisons. Failed or missing required
checks also make an observation unusable. A usable observation must start from
the compared baseline revision and pass every recorded and required check.
Non-finite samples, costs, aggregates, and gains are rejected.

| Decision | Evidence and completion |
| --- | --- |
| `observed` | At least one recorded observation; remains open. |
| `interrupted` | Remains open and may append evidence after resuming; observations may be empty when interrupted before the first attempt. |
| `accepted` | Latest observation is usable under this record's comparison; directional gain meets the threshold; `verification` records a passed independent review, and `retained_candidate` records retained code. |
| `rejected` | Latest observation is usable; non-empty `retained_candidate` preserves the candidate and `restored_revision` equals the baseline, or isolated rejection records the current `accepted_revision`. Improvement is not required for completion. |
| `inconclusive` | All allotted attempts are consumed; at least one observation is usable, but the experiment has no usable decision. |
| `failed_to_run` | All allotted attempts are consumed and none produced a usable observation. |

`verification`, when present, contains exactly `reviewer` (non-empty), `passed`
(boolean), and `summary` (non-empty). Acceptance requires `passed: true` and an
independent review; the binary checks the supplied attestation and cannot prove
the review happened. `retained_candidate` is a non-empty Git retention reference
or description. `restored_revision` must equal the full compared baseline ID. Isolated rejection uses
`accepted_revision` to identify the current accepted destination instead.

The four last decisions are terminal. A concluding record may have an empty
`observations` array and use the latest earlier observation, provided its
comparison facts match. Terminal decisions cannot receive further evidence.
Completion requires terminal evidence and the usual summary/validation; it
checks the evidence before changing status, checkboxes, or the generated index.
Accepted results require independent review and retained-code evidence before
completion; rejected results require retention and restoration evidence.
Record-only completion commits are valid research work.

Completion is separate from code acceptance and area closure. A rejected
hypothesis being `done` does not establish its candidate code. Create or admit
a later task that requires retained candidate code only after acceptance. Add
`requires_code = ["work-002"]` to its task frontmatter, or the equivalent JSON
array to its draft. These same-area task IDs must already identify completed,
verified acceptance with resolvable retained candidate code when the task is
reviewed or imported, selected with `next`, admitted with `work-context`, or
completed. Inspection and record validation check the accepted evidence without
requiring a clone-local ref. Availability checks resolve the retention ref or
the recorded candidate commit when it is already in local history. If neither
is available, fetch the named experiment ref from its originating clone before
admission; ordinary clones do not transfer `refs/zdev/experiments/` automatically.
Rejection cannot satisfy this requirement. `blocked_by`
continues to mean task completion; `context_tasks` supplies research context. A
justified retry is a new hypothesis linked through `context_tasks`; completed
hypotheses cannot be reopened. Later proposals read earlier lessons before
choosing a mechanism. Ordinary tasks retain their existing completion and
reopening rules.

### Complete one local experiment

Record and import the hypothesis before changing code. Commit its task, index
and area settings, then name the exact clean code files the experiment owns:

```text
zdev research experiment work work-002 begin --path src/cache.rs --path src/new-cache.rs
# Edit only the declared candidate files.
zdev research experiment work work-002 evaluate --format json
zdev research experiment work work-002 inspect --format json
```

`begin` records the accepted baseline and a work-context snapshot. Baseline code
must match HEAD; intervening commits containing only `.zdev` records are valid.
Owned files must have no preexisting staged, unstaged or untracked changes.
Unrelated changes on other files may remain. `permitted_changes` describes the
human-reviewed scope; `--path` names exact files for Git retention and restoration.
Leave candidate files unstaged. Directories, submodules and paths through
symlinks cannot be owned.

`evaluate` reserves one attempt, retains a Git candidate reference, executes
all named correctness commands and the evaluator, and appends actual samples,
failures, elapsed time and invocation counts to the task. Inspect reports the
candidate reference and `verification_snapshot`. A correctness command passes
only when it exits successfully. Each evaluator invocation must print one finite
number. Commands run in the repository with the configured timeout.

For keep, dispatch a fresh independent verifier against the reported snapshot.
Save its ordinary nine-key verifier envelope as JSON, including
`work_context_snapshot: <verification_snapshot>` in its evidence, then run:

```text
zdev research experiment work work-002 finalize --decision keep --lesson "The cache removes repeated parsing." --verification /tmp/verifier.json
```

Keep requires usable improving measurements, a passing verifier, the unchanged
candidate snapshot and actual retained Git code. It advances the area's baseline
revision and samples. For a measured candidate you reject, run:

```text
zdev research experiment work work-002 finalize --decision discard --lesson "The cache costs more than it saves."
```

Discard records the lesson, proves candidate retention, restores only owned code
to the accepted baseline, completes the task and commits its records. Keep also
commits accepted code. Both preserve unrelated staged, unstaged and untracked
work. Terminal `--decision failed-to-run` requires exhausted attempts without a
usable observation; `--decision inconclusive` requires exhausted attempts with
at least one usable observation. Both retain candidate references and restore
the baseline. Interruption leaves the task open. Managed tasks cannot use supplied
`task research` records or `task done` to bypass these gates.

Use `inspect` to read an interrupted checkpoint and `recover` to resume it.
Recovery retains completed samples and reserved invocation counts, never launches
an unresolved command again, and resumes pending retention, publication,
restoration, completion or commit. It refuses changed owned files, staged overlap,
unauthorized settings or ambiguous records. During an active run, authorized
comparison-condition changes still permit historical recording and discard;
unrelated area metadata edits remain protected. Candidate refs remain under
`refs/zdev/experiments/<area>/<task>/<attempt>`. Checkpoints and snapshots live in
the worktree's Git administrative directory. They are local recovery state;
the completed task and Git history retain the experiment result.

### Authorize and recover a research run

A run authorizes the coordinator to generate supporting tasks and hypotheses
within the area's policy and the supplied scope. The harness judges relevance
to the brief and scope; the binary validates authorization, graph references,
limits and exact publication. Ordinary reviewed task import and derived-work
approval retain their existing authority.

```text
zdev research run work start --scope "Improve parsing and its fixed benchmark" --attempt-limit 20 --command-limit 100 --time-limit-seconds 3600 --format json
# Keep the returned run.id as <run>.
zdev research run work admit --run <run> --request generation-1 --from /tmp/mixed-tasks.json --format json
zdev research run work context --format json
```

`admit` accepts the ordinary mixed TaskBundle schema. It allocates ordinary task
IDs and resolves bundle keys in dependencies and hypothesis context links.
`requires_code` still requires completed verified acceptance. Retry the exact
bundle with the same request identity after interrupted publication. Retry
returns the original IDs, preserves existing files, and refuses changed content
under that identity or unrelated destination edits. An admission does not
consume a work attempt; dispatch does. Commit newly admitted records through
the usual coordinator workflow before beginning a local experiment.

The default allowance is **20 coordinator work attempts**. Each dispatch for
supporting work, candidate preparation, repair, measurement rerun, confirmation,
reconciliation or actual rebaseline consumes one attempt, even if it produces
no usable observation. `begin` charges preparation and each `evaluate` charges
another attempt. Before dispatching ordinary supporting work, reserve it:

```text
zdev research run work reserve --run <run> --request support-1 --task work-003 --kind supporting
# Perform and independently verify the ordinary task, then complete and commit it.
zdev research run work settle --run <run> --request support-1
```

Generated supporting tasks require that reservation before completion. A
reservation retry reports its original state; it never authorizes a second
dispatch. Repeated work uses a new request. Kinds are `supporting`, `preparation`,
`repair`, `rerun`, `confirmation`, `reconciliation`, and `rebaseline`. Local
`begin` and `evaluate` reserve automatically, or consume a previously reserved
hypothesis dispatch with `--reservation <request>`; a bound reservation cannot
be reused by another attempt. Settle records completed external supporting
work. Bound experiment costs settle from their existing checkpoint, including
when a run stops. An orphan reservation created before checkpoint publication
can be settled without refunding its consumed attempt.

`--until-interrupted` explicitly removes the default attempt limit. Supplied
`--attempt-limit`, `--command-limit`, `--command-seconds`,
`--time-limit-seconds`, and directional `--target` remain effective. Command
limits count each ZDev correctness or evaluator invocation before launch.
`--command-seconds` limits their cumulative elapsed time; command timeouts are
clipped to both that remaining allowance and the original wall-clock deadline.
Supporting workers consume work attempts; the binary does not sandbox their
shells. Use the bound lifecycle for measurement while a run is active:
standalone `research measure` and supplied `task research` observations refuse
to bypass its accounting.

Reserved work and invocations remain consumed through failure or interruption.
Recover the owning experiment before dispatching more work when a command's
outcome or cost is unresolved. Recovery checks process ownership and never
relaunches an unresolved process. Completed observations remain in the owning
task; the empty terminal record adds no measurement cost. Repeated recovery or
finalization settles the checkpoint's cumulative cost once. An invocation
reserved before its checkpoint update retains a conservative timeout ceiling
until recovery can reconcile its cost.

```text
zdev research run work stop --run <run>
zdev research experiment work work-002 recover --format json
zdev research run work resume --run <run> --format json
```

Stop and resume retain the same identity, scope, consumed allowance and original
time boundary. Target, budget, interruption and shared environment blocker are
distinct stop reasons. Stopping permits recording, restoration and previously
authorized finalization without new costly dispatch. Ending authorization uses
`end --run <run>` after reservations and experiment checkpoints are settled;
a new `start` never replaces an unfinished run. Empty or exhausted queues
request further authorized proposals while allowance remains. They neither
close the area nor establish that its objective has been achieved.

A run freezes metric, direction, permitted changes, repetitions, aggregation,
minimum gain, significance policy and correctness requirements/commands. Evaluator identity,
command or workload condition changes invalidate comparisons;
supplied replacement baseline samples cannot clear invalidation. Changes to
accepted supporting code also require a fresh baseline. Actual rebaseline
measures clean accepted HEAD with the current executable correctness commands
and evaluator, charges its attempt and commands, and stores new samples in
area settings:

```text
zdev research run work rebaseline --run <run> --request baseline-1 --format json
```

A completed rebaseline request is idempotent, including recovery of pending area
publication. An interrupted request never launches commands again; resolve its
failure and use a new counted request. Retain or restore dirty candidates before
rebaselining. A candidate measured under previous conditions can still publish
historical evidence and be discarded; it cannot be accepted under the changed
conditions. Verified acceptance through the local lifecycle advances the
accepted baseline without requiring another baseline measurement.

Record a shared environment failure, such as an unavailable measurement host,
with `block --run <run> --reason "<failure>"`. A blocker prevents experimental
dispatch rather than creating repeated failed hypotheses. In-scope supporting
work can still be admitted and reserved to repair the shared environment, within
the retained allowance. After resolving the
external issue, `resolve --run <run>` preserves all costs and requires an actual
rebaseline before comparisons resume. Ordinary candidate failures remain task
results. Baseline correctness or evaluator failures establish a shared blocker.

`context` (also `inspect`) reads current settings, task results and lessons,
unfinished checkpoints, admissions, outstanding reservations and remaining
limits. Live elapsed and remaining wall time appear only in this run projection,
so ordinary immutable work-context snapshots do not change merely as time
passes. Run state lives beside local experiment checkpoints in the worktree's
Git administrative directory. Task records remain authoritative for hypotheses,
observations and lessons. Serial/parallel harness orchestration builds on these
commands; run authorization itself does not launch a harness.

## General area convention

`general` is the conventional tag for recurring one-off work. It is an ordinary
area, not an area kind. Create or switch to its isolated branch and run:

```sh
zdev area create general --title "General work" --objective "Keep concrete one-off improvements as reviewed tasks."
```

When the user explicitly prefers shared-trunk work, the same command may add
`--trunk`; this changes only area branch mode, not task lifecycle or ownership.

Keep `brief.md` minimal and reusable. It records shared engineering boundaries,
the testing level, and validation; each task records its own concrete outcome,
context, boundaries, done proof, and validation. Unsliced tasks are normal.
Slices remain optional for groups of related tasks. All ordinary task review,
approval, branch, verification, completion, and commit rules still apply.

## Slice briefs

Larger areas may keep lightweight slice briefs under
`.zdev/<area>/slices/<key>.md`. Create and inspect them with:

```sh
zdev slice create <area> <key> --title <title> --objective <objective> --boundary <text>
zdev slice list <area>
zdev slice show <area> <key>
```

Repeat `--boundary` for additional boundaries. Each file has TOML frontmatter
containing exactly `schema_version`, `key`, `area`, and `title`, followed by
non-empty `## Objective` and `## Boundaries` sections. The key is a lowercase
path segment and must match the filename.

A slice is durable planning context, not a second lifecycle. It has no status,
tasks need not belong to one, and the area brief remains authoritative for
shared decisions and testing. A task that uses a slice should link that slice
brief explicitly.

## Task files

Each task is one Markdown file under `.zdev/<area>/tasks/`.

```markdown
+++
schema_version = 1
id = "scheduling-001"
key = "model"
area = "scheduling"
status = "open"
complexity = "advanced"
afk = true
priority = "high"
slice = "api"
blocked_by = []
+++
# Add the scheduling model

## Outcome

The model represents the required scheduling decisions.

## Context

The existing scheduler accepts jobs but has no durable decision model. Add the
model beside the scheduler types in `src/scheduling/model.rs`; use the job
vocabulary settled in `brief.md` and extend the focused model tests in
`tests/scheduling_model.rs`.

## Boundaries

- Change the scheduling model and its focused tests.
- Preserve the solver API and follow the area's existing model-test patterns.

## Done when

- [ ] The model represents the required scheduling decisions.
- [ ] Focused model tests cover the decision behavior named in the brief.

## Validation

- Run the focused model tests.
```

For children created by an implementation split, the rendered path list is the
initial allocation used to preserve parent and sibling ownership. Coordination
may extend that allocation during authorized work only after checking retained
parent edits and sibling assignments. Existing task records that say
`Task-owned paths (exact)` remain valid and readable; treat that legacy text as
the same initial allocation unless it records an explicit user constraint.

The frontmatter contains only routing state:

- `id` is stable and allocated by zdev;
- `key` connects a temporary import bundle to the durable task;
- `area` prevents tasks from being moved between objectives accidentally;
- `status` is `open` or `done`;
- optional `complexity` is `routine`, `standard`, or `advanced`;
- optional `afk` marks work whose implementation and required validation can
  complete unattended with the available repository-local authority;
- optional `priority` is `high`, `normal`, or `low`;
- optional `slice` names an existing slice brief in the same area; and
- `blocked_by` contains stable task IDs.

Omitted complexity resolves to `standard` without rewriting the task. Use
`routine` only for tightly specified, low-risk mechanical work, `standard` for
ordinary bounded implementation, and `advanced` when the approved task needs
additional planning or reasoning. Complexity does not change readiness,
lifecycle, or dependencies.

An open task is ready when every blocker is done. Otherwise it is blocked. With
no user focus, zdev chooses from the ready frontier by AFK suitability first,
then priority, then numeric task ID. Omitted values behave as `afk = false` and
`priority = "normal"`. A fuzzy user focus is handled by the harness instead: it
reads every ready task and chooses the best fit from the full frontier.

Do not mark a task `afk = true` when its required completion evidence needs a
physical device, interactive sign-in, unavailable credential, production
access, or another human action. Prefer a committed unattended implementation
task followed by an `afk = false` attended acceptance task. Express that order
with `blocked_by`, give the attended gate low priority while useful ready work
remains, and leave unrelated unattended work independent. Credential or device
setup belongs at the point where the first task actually consumes it, not at
the start of the area.

`zdev task done` checks the done-condition boxes, changes the status, and appends
one result section:

```markdown
## Result

Implemented and independently verified the scheduling model.

Validation:

- Focused model tests passed.
```

`TASKS.md` is a generated list of all task files and their derived state. It is
never an authored source.

## Task bundle

`zdev tasks import` accepts a JSON object with `schema_version`, `area`, and a
`tasks` array. Each task contains:

- `key`;
- `title`;
- optional `complexity`; omitted means `standard`;
- optional `afk`; omitted means `false`;
- optional `priority`; omitted means `normal`;
- optional `slice`, naming an existing slice brief in the area;
- `blocked_by`, using keys from the same bundle or existing task IDs;
- `outcome`;
- optional `context`, rendered as prose under `## Context`;
- optional `boundaries` for task-specific non-goals and scope limits;
- one or more `done_when` strings; and
- optional `validation` strings.

Run `zdev tasks review <area> --from - --format json` to validate a draft bundle
and store its complete Markdown review, canonical bundle, and internal
fingerprint in Git administrative state. The small JSON result names the actual
Markdown path and an opaque `review` identity. Present the exact document with
`zdev tasks review <area> --show`; the user never handles the identity or
fingerprint. After approval or under delegated creation/import authority, use
`zdev tasks import <area> --reviewed <review-id>`. Zdev requires that identity to
remain current and revalidates the
stored bundle before publication. Direct `--from` import remains valid, and
`--approval` remains compatible with callers that already use it for direct
input.

Review, import, and `zdev check` reject a task whose `slice` does not name an
existing `.zdev/<area>/slices/<key>.md`. Unsliced tasks remain valid and appear
only in area-wide progress totals.

## Derived proposal

A worker may return one transient proposal for direct follow-up work instead of
editing task state. The first line is exact:

```text
PROPOSE zdev-derived <area> <source-task-id>
```

One JSON object follows. It contains only `schema_version: 1`, `proposal`,
`area`, `source_task`, `source_result`, `tasks`, and, for a split,
`split_ownership`. Proposal is `investigation_follow_up` or
`implementation_split`. The source result is `{status, summary, validation}`:
follow-up uses `complete` with non-empty validation; split uses `split` with an
empty validation list. `tasks` contains one through five ordinary TaskDraft
objects from the bundle format above, including optional complexity and slice.
It cannot contain another proposal.

A split requires `split_ownership` with `retained_parent_paths` and
`child_future_paths`. Each child entry is `{key, paths}`, names every proposed
key exactly once, and has at least one normalized repository-relative future
path. Retained and child paths are exact and pairwise disjoint. Before edits,
the retained list is empty; after edits it equals the complete unstaged
parent-owned path set. Unknown or duplicate fields, a second object, missing or
extra ownership, invalid dependencies, and nested proposals are invalid.

Splitting an AFK task for easier implementation or verification needs no new
approval when scope, boundaries, and testing are unchanged. Set unattended
children explicitly to `afk: true`; omission defaults to false. Keep the source's
original done conditions and use **Death loops** in [review.md](review.md)
when recursive splitting fails to produce implementation progress.

The worker never runs derive commands. The coordinator applies clear direct
work with `zdev tasks derive apply <area> --from -` and no approval. Only
semantic authority uncertainty uses `zdev tasks derive review`, after every
mechanical and current-state gate passes, then `zdev tasks derive review
<area> --show` presents its stored Markdown for manual approval. Apply the
approved opaque identity with `zdev tasks derive apply <area> --reviewed
<review-id>`; do not reconstruct or resend the proposal.
Invalid, unsafe, drifted, staged, incomplete-ownership, and mechanical-failure
states stop; a stored review cannot waive those gates. Derived work never uses
ordinary task import.

A path is accepted for manual use and remains in place after review or import.
For project and pull-request records, add `--commit --format json` to publish
the approved bundle, including the initial split. Personal records keep using
ordinary import. Use ordinary import under any policy when the user wants
uncommitted additions. A later committed import contains only the new task
files, regenerated `TASKS.md`, and the owning area's tracked modified brief when
present. An initial committed import also contains the required project config,
area metadata, brief, and referenced slice briefs. Leave a modified tracked
brief unstaged; zdev validates and commits it with the tasks. The JSON result
includes task IDs, the complete area ready frontier, paths, the commit hash, and
stable change ID. Zdev validates the brief and complete dependency graph before
creating any task file.

Keep shared context, including the required area-wide testing level, in
`brief.md`. For newly drafted implementation tasks, use `context` to select and
connect the task-specific reason, current repository behavior, settled
constraints, and relevant source or test seams. Point to the brief or
background documents instead of copying their shared material. Version 1
bundles and task files without `context` are accepted. Use task
boundaries only for limits the implementation and verification agents need for
that slice; do not turn expected source or test paths into an implicit
allowlist. Test-related done conditions and validation must apply the agreed
level, not silently expand it.

`zdev check` requires non-empty `Outcome`, `Done when`, and `Validation` sections.
Done conditions must be checklist items; a completed task must have every item
checked and include a non-empty `Result` section.

Codex and Claude Code install separate research execution references. Codex
supports serial and rolling runs; Claude remains serial.
Claude uses `zdev-research` with structured authorization and a fresh invocation
identity after stop/failure. Ordinary Loop remains ordinary task continuation.
Other harnesses expose these records and local commands without an installed
research execution route.

### Isolated hypothesis candidates

One authoritative checkout owns the task, run, observations, acceptance and
completion. Source workers edit and validate in separate linked Git worktrees.
They read the authoritative brief, task and repository instructions by the
absolute paths in the candidate context. They do not edit source `.zdev` records
or publish authoritative records. Create each source branch from the admitted
HEAD before dispatch; the binary admits existing sources and never launches
workers or removes worktrees.

Start an explicitly authorized run with separate positive capacities:

```text
zdev research run work start --scope "Changes allowed by the brief." --worker-limit 2 --measurement-limit 1
zdev research experiment work work-002 admit --source /absolute/source --path src/cache.rs --format json
```

Both limits default to one. Admission captures the explicit task snapshot,
destination HEAD, accepted revision and measurement requirements at dispatch,
source root/branch/base and authoritative record locations. It charges one
preparation reservation before worker dispatch. A source root or branch already
owned by an unfinished or ambiguous candidate in any area refuses admission
before claiming capacity or authorizing dispatch. A changed reservation identity
also refuses admission replay. Ownership continues until
durable completion and confirmed worker and measurement stop. Retrying the same
admission returns the stored context with `dispatch_authorized: false`; only a new admission
reports `dispatch_authorized: true`. It never authorizes another worker. An unconfirmed
worker keeps its slot even when a coordinator session ends. Preparation can
continue while a known live isolated measurement occupies another capacity.
Unknown invocation costs and measurement slots remain reserved.
For a later source repair, reserve a new `repair` request, then call `prepare
--reservation <request>` to claim its worker slot before dispatch. Its retry also
reports no dispatch authorization. Return confirms stop and releases that slot.

Use `reserve --worker-slot` before every native planner, implementer, verifier
or delegated finding checker. The existing reservation stores both its immutable
requested slot mode and current occupancy. Only its first successful Reserve
returns `dispatch_authorized: true`; exact retries return `false` even after
binding or settlement. Changed task, kind or requested mode refuses. Transfer
an unused preparation claim to Admit or a repair claim to Prepare with
`--reservation`; neither counts its own worker slot twice. Worker requests cannot
be bound as evaluator authorization. Confirm standalone roles stopped before
`settle`; bound source roles settle through Return. The coordinator also checks
actual native capacity and uses the tighter limit. Measurements remain separate.

After clean integration, only the acceptance owner may reserve a counted
`confirmation --worker-slot` or `repair --worker-slot` role. Other confirmations,
repairs and measurements remain gated. A same-owner Prepare can withdraw that
exact measured destination code to unchanged accepted HEAD, preserving unrelated
index/worktree/untracked changes, retained source and historical evidence. It
persists the repair claim and withdrawal before effects, invalidates verification
and releases the lane after restoration. Changed owned code/index or unresolved
integration conflicts refuse. Recover finishes authorized withdrawal without
worker permission; Prepare replay cannot dispatch again. Run stops cannot admit
new repair but permit recovery of recorded effects. Repaired source requires
confirmed Return, new Evaluate, integration and fresh destination verification.

After the coordinator confirms that source work has stopped:

```text
zdev research experiment work work-002 return --worker-stopped --format json
zdev research experiment work work-002 evaluate --format json
zdev research experiment work work-002 integrate --format json
```

Return checks the full staged, unstaged and untracked source delta, refuses
unowned paths and `.zdev` edits, and retains an ordinary Git transport commit
and reference. It preserves refused changes. Evaluate runs correctness and
measurement commands in the validated source directory, with authoritative
observations and counted reservations. A competing measurement cannot launch
while its capacity is occupied. Every attempt retains its own comparison facts.

Integration claims the run's single acceptance lane and rechecks the ready
task, baseline, conditions, retained source and destination ownership. It applies
Git transport with `cherry-pick --no-commit`, preserving binary bytes, modes,
additions, deletions and unrelated destination index/worktree/untracked work.
Overlapping destination changes refuse integration before mutation. Conflicts
remain in place for inspection. Only an unchanged fresh destination snapshot
and a passing independent verifier permit `finalize --decision keep`.

When accepted code changes, use `reconcile` before another measurement or
integration. Reconciliation charges a new reservation, retains the old source,
refreshes exclusively owned stopped source work against current accepted HEAD
and applies its Git transport. Repair preserved conflicts in the source and
return with confirmed stopped work. The refresh retains its counted reservation;
measurement capacity is claimed when evaluate begins its actual attempt.
Evaluate again against the current baseline;
old improvement evidence cannot authorize keep. Repeated baseline changes repeat
these gates. Record-only HEAD changes require explicit refresh through reconcile
(or integrate for an already integrated candidate) and fresh destination
verification, while unchanged code measurements remain valid.

Changed evaluator/workload conditions pause old comparisons. Run an actual
counted `research run ... rebaseline` on clean accepted destination code, then
reconcile and remeasure resumed candidates. Historical rejection may instead
retain its original evidence and lesson. Isolated rejection leaves the current
accepted destination implementation in place; its terminal result records
`accepted_revision`, rather than claiming restoration to an obsolete compared
baseline. Local rejection continues to use `restored_revision`.

Inspect/recover exposes `active`, `returned`, `accepted`, `rejected`, `paused`
and `ambiguous` candidate states. Recovery resumes retained commits, source
refresh, integration, publication, finalization and accounting; it never
redispatches a worker or relaunches an unresolved measurement. A live or unknown
process retains measurement capacity. `cleanup_eligible` requires confirmed
worker stop and durable completion. The coordinator may remove only a source it
created and owns; pre-existing and ambiguous worktrees stay in place.

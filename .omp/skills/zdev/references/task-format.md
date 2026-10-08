# Task format

## Project and area metadata

`.zdev/config.toml` records project `trunk`. An isolated
`.zdev/<area>/area.toml` stores its owning `branch`; optional `parent` and
`base_commit` describe its managed base relationship. An explicit trunk area
stores only `mode = "trunk"` and dynamically resolves its branch from
`project.trunk`; it stores no branch, parent, or base commit. Several explicit
trunk areas may share configured trunk, but an isolated area remains its
branch's sole owner. Never infer mode from a branch name.

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

This version configures research and validates supplied evidence. It does not
launch evaluators, compute statistical confidence, verify Git attestations,
retain or restore candidate code, or run serial or parallel experiments.
Automatic implementation, planning, and verification dispatch reject hypotheses
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
  "attempt_limit": 2
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
All JSON objects reject unknown fields. Settings require every shown field:

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
| `evaluator_command` | Non-empty command; only the area setting supplies this command. |
| `permitted_changes` | At least one unique non-empty scope boundary. These are human-reviewed boundaries, not a path sandbox. |
| `attempt_limit` | Positive finite limit per hypothesis. Each observation counts as one attempt. |

Fields below `comparison.metric` through `required_checks` in the table also
belong to `comparison`. Each candidate preparation or repair attempt, measurement
rerun, confirmation, and reconciliation consumes an attempt; record its
observation even if it fails. `elapsed_seconds` records its cost. This version
validates the supplied count and cost; it cannot enforce accounting for
unrecorded work. Run-wide limits and authorization belong to the later research
execution workflow.

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
nonnegative `elapsed_seconds` for recorded elapsed cost.

Each observation requires the two full revision IDs, `outcome`, `samples`,
`checks`, and `elapsed_seconds`. Check objects contain exactly `name` and
`passed`; names are unique. A `measured` observation requires exactly
`repetitions` finite samples and forbids `failure`. `crashed` and `timed_out`
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
| `rejected` | Latest observation is usable; non-empty `retained_candidate` preserves the candidate and `restored_revision` equals the baseline. Improvement is not required for completion. |
| `inconclusive` | All allotted attempts are consumed; at least one observation is usable, but the experiment has no usable decision. |
| `failed_to_run` | All allotted attempts are consumed and none produced a usable observation. |

`verification`, when present, contains exactly `reviewer` (non-empty), `passed`
(boolean), and `summary` (non-empty). Acceptance requires `passed: true` and an
independent review; the binary checks the supplied attestation and cannot prove
the review happened. `retained_candidate` is a non-empty Git retention reference
or description. `restored_revision` must equal the full compared baseline ID.

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
a later task that requires retained candidate code only after acceptance. A
justified retry is a new hypothesis linked through `context_tasks`; completed
hypotheses cannot be reopened. Later proposals read earlier lessons before
choosing a mechanism. Ordinary tasks retain their existing completion and
reopening rules.

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
fingerprint. After approval, use `zdev tasks import <area> --reviewed
<review-id>`. Zdev requires that identity to remain current and revalidates the
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

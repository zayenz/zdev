# ZDev fit and candidate implementation increments

Research date: 2026-10-07. Local code inspected at `dd641ca`; installed binary
reports `zdev 1.6.0`. This is a design proposal for discussion. It does not
define new supported commands, an approved schema, or imported tasks.
The [area brief](../brief.md) remains authoritative.

## Existing mechanisms to reuse

| Existing mechanism | Current source | Research use or limit |
| --- | --- | --- |
| Area objective, brief, lifecycle, and branch mode | [project.rs](../../../src/project.rs), [area lifecycle](../../../docs/area-lifecycle.md) | The campaign belongs at area level; queue exhaustion already differs from explicit closure. |
| Task records, outcome, context, boundaries, done conditions, and result | [tasks.rs](../../../src/tasks.rs), [task format](../../../skills/zdev/references/task-format.md) | Can retain the hypothesis and conclusion; typed research fields do not exist today. |
| Ready frontier and AFK/priority ordering | `tasks.rs`: `task_state`, `next`, `goal_tasks` | Good for ordinary tasks. Research selection must also consider evidence and exploration. |
| Exact task-bundle review and import | `tasks.rs`: `review`, `import` | Reuse validation and publication, but do not ask for every in-scope generated idea after campaign authorization. |
| Derived work proposals | [derived work](../../../docs/derived-work-handoffs.md), `tasks.rs`: `DerivedProposalKind`, `apply_derived` | Existing proposals are one-to-five direct follow-ups/splits tied to a selected source task; not a general campaign generator. |
| Work-context snapshots and branch safety | [round trips](../../../docs/workflow-round-trips.md), `project.rs`: branch checks | Retain baseline ownership and fresh acceptance gates; snapshots alone do not contain experimental evidence. |
| Finite parallel task batch | [parallel route](../../../skills/zdev/references/parallel.md), [task workflows](../../../skills/zdev/references/task-workflows.md) | Reuse isolated workspaces and serialized destination acceptance. Its ordinary compatibility rules need a research-specific alternative. |
| Native area loop and bounded fallback | [area-loop route](../../../skills/zdev/references/area-loop.md), [goal.rs](../../../src/goal.rs) | Existing loop consumes a ready queue and stops at exhaustion; research must generate new hypotheses inside the authorized budget. |
| Worker profiles and integration rendering | [config.rs](../../../src/config.rs), [integrations.rs](../../../src/integrations.rs) | Reuse model/effort resolution and the coordinator/worker boundary; do not create another provider runtime. |

All links in this table are repository sources. Read the named function in the
current checkout before implementing; paths identify seams, not an allowlist.

## Where the current contract does not fit

### No research discriminator or measurement data

`TaskHeader`, `TaskDraft`, and `AreaMetadata` deny unknown fields. There is no
task kind or area research setting. Task bodies allow extra Markdown sections,
so a manual prototype could describe a hypothesis in prose, but existing
selectors and workflows would still treat it as ordinary implementation work.
`task done` appends a textual summary and validation; it cannot currently accept
or validate typed observations and experimental decisions.

Design choice: add only the machine-readable distinction needed for routing and
validation. Prefer the existing task as the hypothesis record. Repeated runs
may need a small subordinate record, but avoid a second independent task store.
Decide storage before adding fields or commands; do not put unsupported
frontmatter in the research area now.

### Completion is written as implementation success

The common implement workflow verifies requirements, completes the task, and
commits accepted changes. For a hypothesis, the requirement can instead be a
valid measurement and a justified rejection. The verifier needs to assess the
promised experiment, its evaluator, and the conclusion. Metric improvement
should decide candidate acceptance, not whether a properly rejected hypothesis
can be `done`.

A prerequisite requiring a working feature is also different from a hypothesis
requiring a measurement. A completed rejected hypothesis must not accidentally
unblock ordinary work that assumes its candidate was retained. Prefer an
explicit acceptance-dependent task or a clearly stated follow-up decision
before inventing a new dependency language.

### Continuous generation needs area authority

The current loop stops when an open queue is empty or exhausted. Ordinary task
creation reviews an exact bundle. Derived work permits small direct follow-ups,
but still depends on a selected source task and its current outcome. The
research workflow needs authorization for an evolving set of hypotheses and
a bounded run, including what happens when there is no source task yet.

Reuse the coordinator's publication ownership and ordinary task validation.
Do not disguise a campaign as an unlimited chain of implementation splits.
Out-of-scope changes still need the user's decision, while routine in-scope
hypothesis generation should proceed under the agreed research objective.

### Existing parallel admission assumes compatible implementation work

The ordinary route requires mutually compatible source changes and forbids
processing derived follow-ups within a batch. Research candidates can be
mutually exclusive edits of the same file. Worktree isolation makes that safe
to test, but does not make both safe to integrate. Research needs candidate
admission by experimental validity and resources, followed by current-baseline
acceptance. Preserve the ordinary route's behavior for ordinary tasks.

### Branch rollback can erase task evidence

The original autoresearch recipe commits candidate code and resets losers.
ZDev's project record tracks `.zdev`. A whole-branch reset could remove the
hypothesis task, its result, or other completed work. Authoritative research
state belongs in the destination checkout throughout a parallel run. Local
rollback must preserve research records and unrelated Git changes.

## Recommended first model

Use a research-enabled area containing ordinary tasks and hypothesis tasks.
Keep ordinary lifecycle values and the existing dependency graph. Describe the
metric contract once at area level. Hypotheses store their proposed change and
predicted mechanism; result records store the actual comparison and lesson.
Let the harness select and generate hypotheses; the binary validates and
records them. This is the smallest extension that follows the user's request
while making research routing explicit.

Alternatives worth comparing before choosing the schema:

| Option | Benefit | Cost or limitation |
| --- | --- | --- |
| Instructions and extra Markdown only | Small manual prototype; no migration. | The binary cannot validate metric records or route hypotheses reliably. |
| Optional research area setting plus hypothesis task distinction | Reuses area/task ownership and permits mixed queues. | Requires changes across record parsing, projections, completion, and integrations. Recommended direction. |
| One campaign task with an external experiment ledger | Convenient single entry point. | Hypotheses become a second work system rather than the ZDev tasks the user requested. |
| Independent campaign/tree database | Supports elaborate search and population policies. | Duplicates ZDev ownership, persistence, and lifecycle. Defer unless simple task records prove insufficient. |

Use hypothesis ancestry only if it changes context selection. It does not
replace `blocked_by`, and neither replaces the actual starting code revision.
Likewise, a “best measured” candidate is not necessarily the current accepted
implementation; failed acceptance or pending integration can separate them.

## Candidate increments for the later task bundle

These increments are planning suggestions, not ready tasks. Draft exact tasks
after discussion resolves the brief's open questions. Each should deliver a
small usable piece and its required validation.

### 1. Settle and demonstrate the research contract

Write the minimal area/task/result model and a worked example in repository
documentation. Walk one ordinary setup task, one winning hypothesis, one valid
loser, and one interrupted attempt through existing lifecycle and branch rules.
Choose run authorization, evaluator output, storage, and candidate retention.

Done proof: a reader can distinguish task completion, experimental conclusion,
candidate acceptance, and area closure without guessing. A clear example names
where every record is authoritative and how a losing candidate is undone.
No new code or tests are needed merely to settle those choices.

### 2. Run and retain one hypothesis locally

Add the smallest record and command/workflow support for a research area with
mixed tasks. Establish a baseline, store a hypothesis, collect a valid result,
and accept or reject it on the current area branch. Retain the rejected
experiment's task and lesson after rollback. Ordinary task behavior continues.

Done proof: an existing deterministic repository-local evaluator demonstrates
a winner, a loser, and an invalid result. Focused command/Git tests exercise
completion without improvement and retention of unrelated changes. Avoid a new
benchmark subsystem.

### 3. Generate and execute a bounded serial research run

Add the harness route that reads past evidence, proposes ordinary hypothesis
tasks, selects work, and repeats within the authorized limit. Finish with an
accurate stop reason and current accepted result. Resume from durable tasks
and observations after interruption or context loss.

Done proof: a short run creates its own hypotheses, completes both an accepted
and a rejected experiment, honors its budget, and restarts without duplicating
a known completed experiment. Queue exhaustion and target completion are
reported separately.

### 4. Admit a finite parallel hypothesis batch

Prepare two or more candidates in assigned worktrees from a common baseline.
Account for live roles and benchmark resources separately. Collect results in
the destination and accept one at a time. Reconcile and remeasure a stale
sibling before accepting it; incompatible candidates may finish as rejected
results. Preserve ambiguous workers and worktrees on interruption.

Done proof: two hypotheses can safely edit the same file. After the first
acceptance, the second cannot enter based solely on its old score. A workload
with one constrained evaluation slot can prepare candidates in parallel while
measuring serially. Test only the new public branch/acceptance behavior.

### 5. Demonstrate useful optimization and extend harness support

Use a real existing benchmark or workload with a meaningful metric and
correctness check. Compare the accepted final implementation with a freshly
measured baseline under the same conditions. Add adapter behavior for other
supported harnesses using their actual continuation and worker facilities.

Done proof: the demonstration retains hypothesis reasoning and failed results,
reports resource use and limitations, and either shows a confirmed gain or
honestly reports no improvement. Feature acceptance should not depend on a
stochastic demo happening to find a gain. Each harness states its supported
serial/parallel and continuation behavior.

## Example future campaign

An example brief could optimize median runtime of a representative task-graph
read while requiring byte-for-byte equivalent JSON for valid inputs and the
same rejection behavior for invalid graphs. The actual workload and evaluator
would need to be chosen and measured; this research does not declare a ZDev
bottleneck.

| Work item | Kind | Completion evidence |
| --- | --- | --- |
| Establish the representative workload and evaluator | Ordinary | Runnable, fixed workload and recorded baseline. |
| Cache a repeated graph calculation | Hypothesis | Valid comparison and conclusion; complete whether accepted or rejected. |
| Change task-file read ordering | Hypothesis | Same evaluator and correctness evidence, with measured result. |
| Repair a benchmark that omits input failures | Ordinary | Corrected evaluator and new comparable baseline. |
| Retry parallel reads after the workload changes | Hypothesis | Names the earlier rejected assumption that changed. |

This demonstrates mixed work and experimental completion without committing
to these optimization ideas or creating tasks for them now.

## Validation proportional to risk

Research-only records need area validation and a checked diff. Subsequent
record and command changes need focused public-behavior tests using existing
repository patterns. The most valuable failures to exercise are invalid metric
output, accepted/rejected completion, interrupted record publication, local
rollback with unrelated edits, and parallel baseline drift. Do not test every
field or prompt sentence separately.

Use the repository's [contributing guidance](../../../CONTRIBUTING.md) and
established Cargo checks when source changes. A short real workflow supplies
evidence that the harness can use the feature; unit tests cannot demonstrate
that an autonomous search reliably finds useful hypotheses.

# Experiment mechanics worth adapting

Research date: 2026-10-07. The [brief](../brief.md) owns the current scope and
decisions. The recommendations here explain choices for later discussion.
The source catalog supplies revision-specific links. No external experiment
was executed during this investigation.

## The reusable loop

[Karpathy's program](https://github.com/karpathy/autoresearch/blob/228791fb499afffb54b46200aca536f79142f117/program.md)
establishes a baseline, edits one allowed file, runs a fixed training budget,
records the score, keeps improvements, and resets losing candidates. Its TSV
records keep, discard, and crash. The important transfer is a small, repeatable
measurement contract; five minutes, GPUs, and validation bits per byte are
specific to that domain.

[AIDE's search policy](https://github.com/WecoAI/aideml/blob/60b3978ddf65b71f86eb7c64506965048a1398cf/aide/agent.py)
first drafts solutions, sometimes debugs buggy leaves within a depth limit,
then improves the best valid node. Its
[journal](https://github.com/WecoAI/aideml/blob/60b3978ddf65b71f86eb7c64506965048a1398cf/aide/journal.py)
retains code, parent links, execution results, analysis, and metrics. This gives
ZDev a useful distinction between proposing an idea, repairing its experiment,
and improving a valid implementation. It does not require importing AIDE's
tree-search machinery.

Recommended ZDev sequence:

1. Read the area objective, measurement contract, current accepted result, and
   relevant previous hypotheses.
2. Propose a concrete hypothesis, expected effect, changed variables, and how
   the evaluator could disprove it. Store the task before testing it.
3. Select a ready task. Ordinary tasks follow the ordinary workflow; hypotheses
   use the research workflow.
4. Capture the candidate's starting implementation and experiment conditions.
5. Implement the candidate, run the evaluator, and record observations and
   failures. Bounded repairs can remain attempts of the same hypothesis.
6. Decide whether the evidence supports acceptance, rejection, or more work.
   Independently check accepted code and its measurements as required by the
   agreed policy; verify rejected results sufficiently to trust the record.
7. Preserve the record, accept or undo candidate code, complete the task when
   its experimental question is answered, then use the lesson to select or
   generate more work.

This is a proposed flow. Current ZDev has no command that implements it.

## Define the metric before generating candidates

A research brief needs an operational definition, not only “make it faster.”
For example: reduce median elapsed time for a fixed representative workload,
with an unchanged correctness check and memory below an agreed limit. The
contract should identify the evaluator command, metric name, unit, direction,
aggregation, workload, per-experiment budget, and acceptance rule. Identify
protected evaluator and input paths and the allowed implementation changes.

Missing or non-finite measurements are invalid evidence, not a score of zero.
A timeout is different from a completed but slower run. Keep correctness
requirements distinct from secondary metrics so a speed improvement cannot
silently waive correctness or a hard memory limit.

[OpenEvolve's configuration](https://github.com/algorithmicsuperintelligence/openevolve/blob/9196d8763300d1e46cc8b48cb0dc987966db3d48/configs/default_config.yaml)
supports evaluation cascades, timeouts, parallel evaluations, early stopping,
and a named stopping metric. It also explicitly marks some resource limits and
distributed evaluation as unimplemented. Borrow cheap rejection before an
expensive evaluation where the domain benefits; do not treat a documented
configuration field as proof that enforcement exists.

An ordinary task that changes the evaluator, dataset, hardware, or workload
can invalidate comparisons. It should establish a new baseline before further
hypotheses are judged. This is different from a candidate that changes the
implementation under a fixed measurement contract.

## Distinguish measurements from confidence

Repeatedly keeping the lowest observed runtime can select noise. Record actual
observations and the comparison procedure, rather than treating the running
best score as a reliable estimate.

[pi-autoresearch](https://github.com/davebcn87/pi-autoresearch/blob/04b8a50dac7efe80ffc91648bd00ed9b022e24c2/extensions/pi-autoresearch/index.ts)
separates the benchmark from an optional checks script. It computes an advisory
confidence ratio using variation across the session's metric values. Those
values come from different candidates: this is a heuristic, not a controlled
estimate of repeatability for one implementation. The code says the score
does not automatically discard candidates.

[The MLX fork's rigor helper](https://github.com/trevin-creator/autoresearch-mlx/blob/766a25ff22afa799efd8d0aa450a4348e4749df2/rigor.py)
collects repeated observations and bootstraps a comparison of candidate and
accepted means. Its `--seeds` argument counts repeated invocations; the inspected
`run_once` launches the same training command without passing a different seed.
It also stops after a losing first observation. Borrow the motivation to
confirm small gains; do not copy those choices as a general statistical policy.

Recommendation: use a domain-appropriate procedure stated in the brief. A
deterministic count may need one run. Noisy performance work may need warmup,
several comparable observations, and confirmation of promising gains. Include
that cost in the run budget. Do not make every cheap experiment pay for a
general statistics package.

## Holdout evaluation is a domain choice

[Arbor's idea records](https://github.com/RUC-NLPIR/Arbor/blob/7cdaf1fa6d779b3d5e340052357bf3fe55dfed93/src/coordinator/idea_tree.py)
separate development scores from test scores. Its
[Git acceptance code](https://github.com/RUC-NLPIR/Arbor/blob/7cdaf1fa6d779b3d5e340052357bf3fe55dfed93/src/coordinator/tools/git_ops.py)
evaluates candidates in temporary worktrees and compares against the current
accepted test score. Source inspection shows that a gain below the configured
merge threshold can produce a warning and still proceed; the threshold is not
universally a hard gate. Its generic score extraction can also fall back to an
LLM. ZDev should prefer a defined machine-readable result over model judgment
about which number to optimize.

[SpecBench's loop](https://github.com/WecoAI/SpecBench/blob/08607352adc8abd78be2193dd9f725f1f032b8f0/benchmarks/spec_bench/run_loop.py)
records public and private test trajectories while keeping private results out
of agent feedback. It stops at explicit time or cost limits. This is useful
when public feedback can be overfit. Repeated acceptance decisions on a
so-called holdout can themselves reveal information; a dataset is not truly
held out merely because its command is named “test.”

Recommendation: require correctness and comparable measurements for all
campaigns. Add held-out acceptance only when the domain needs it. Do not require
train/dev/test splits for bundle-size or deterministic algorithm experiments.

## A losing hypothesis can be done

Task status answers whether the task's promised work is complete. Experiment
outcome answers what happened to the candidate. Conflating them would leave
every rejected idea open and repeatedly selectable.

Proposed meanings:

| Experimental conclusion | Meaning for task completion | Candidate code |
| --- | --- | --- |
| Accepted | Required evidence supports keeping the change; complete after acceptance succeeds. | Retain and commit the checked implementation. |
| Rejected | A valid comparison answers the hypothesis against acceptance; complete with the measurements and lesson. | Undo the candidate; retain its task and result. |
| Inconclusive | The available measurement cannot answer the question. Complete only if reporting that limit satisfies the task's done conditions. | Keep outside the accepted implementation. |
| Failed to run | The experiment crashed, timed out, or could not obtain valid evidence. Completion depends on whether the task required a valid comparison or a bounded feasibility attempt. | Undo or preserve separately for a named follow-up. |
| Interrupted | A run stopped before required evidence or accounting was recorded. | Preserve recoverable state; do not claim completion. |

These are conceptual distinctions, not an approved enum or schema. An attempt
failure should not automatically become a terminal hypothesis rejection.
Ordinary prerequisites that need the hypothesis's successful code need a
stronger condition than merely `blocked_by` a completed experiment.

## Retain the explanation for a rejection

The [pi compaction implementation](https://github.com/davebcn87/pi-autoresearch/blob/04b8a50dac7efe80ffc91648bd00ed9b022e24c2/extensions/pi-autoresearch/compaction.ts)
reconstructs the objective, rules, ideas, best result, and recent observations
from disk. Its result guidance asks for a hypothesis, rollback reason, and next
action hint. This is a close fit for task records consumed by fresh ZDev workers.

The project's [discard-revisit example](https://github.com/davebcn87/pi-autoresearch/blob/04b8a50dac7efe80ffc91648bd00ed9b022e24c2/site/content/revisiting-discards.md)
illustrates retrying parallel page builds after caching removes the old CPU
bottleneck. Its useful rule is to state which old assumption changed before
reviving a rejected idea. Measurement-confirmation reruns are a separate case.

Recommendation: keep the hypothesis, starting revision, observed metric,
validity/correctness result, acceptance decision, concise lesson, and any
candidate artifact reference needed later. Represent an intentional changed-
assumption retry as a new task linked to the earlier result; keep measurement
reruns within the same hypothesis when appropriate. Begin with task links and
readable conclusions, not an additional knowledge graph.

## Git rollback must preserve research records

The minimal upstream loop assumes a dedicated clean experiment branch and an
untracked TSV. ZDev can have tracked task records and unrelated local changes.
Resetting that entire branch could erase the losing experiment's record.

pi-autoresearch's inspected implementation stages everything on keep and
restores tracked code plus cleans untracked files on discard, with exclusions
for its own files. Those operations demonstrate the lifecycle but are too broad
for ZDev's existing task ownership rules.

Recommendation: capture the task-owned baseline, save candidate evidence, then
restore only candidate-owned changes. Preserve deletions, additions, file modes,
and existing staged or untracked user work. Decide how to retain a rejected
patch only when later replay needs it; a separate candidate reference or patch
may suffice. A completed losing hypothesis can have a record-only commit.
Do not silently equate “no accepted code change” with “nothing to commit.”

## Parallel experiments differ from parallel implementation tasks

[Arbor](https://github.com/RUC-NLPIR/Arbor/blob/7cdaf1fa6d779b3d5e340052357bf3fe55dfed93/README.md)
uses a coordinator and short-lived executors in worktrees. Its
[worktree tool](https://github.com/RUC-NLPIR/Arbor/blob/7cdaf1fa6d779b3d5e340052357bf3fe55dfed93/src/coordinator/tools/worktree.py)
creates hypothesis branches from an indicated parent. This makes competing
changes to the same code possible without simultaneous edits in one checkout.

[n-autoresearch's orchestrator](https://github.com/iii-experimental/n-autoresearch/blob/26f239c4f0587b1a83c3b27e7b35f061242da9ee/workers/orchestrator/orchestrator.py)
records experiments before execution, manages GPU acquisition, and compares
results centrally. It also uses recent keep/crash rates and near misses to
suggest exploration, conservative improvement, combinations, or ablation.
Borrow capacity accounting and lessons; do not add its KV/HTTP service to ZDev.

Recommendation: separate three limits: live agent roles, concurrent experiments,
and simultaneous use of constrained benchmark resources. Two worktrees do not
give a machine two GPUs or remove CPU/cache contention. Benchmark locally in
sequence when resource interference makes scores incomparable, even if agents
prepare code concurrently.

Start a finite candidate batch from a common accepted revision. Return results
to one coordinator. Candidates are alternatives: do not merge every improvement
measured against the old baseline. If A is accepted first, B must be reconciled
and remeasured against the new implementation before acceptance. Combining A
and B is itself a new experimental claim. Incompatible candidates remain
recorded results even if only one can be accepted.

## Budget, stopping, and recovery

[SPDL's architecture](https://github.com/facebookresearch/spdl/blob/9beff53052004b289a24f22f9934dc1793e0b0f7/docs/source/autoresearch/autoresearch_architecture.rst)
describes a bounded async queue, workflow-owned stopping, persisted phases,
and resume that polls a known running job rather than launching it twice.
Cancellation behavior belongs to the domain coroutine. Its complete framework
is more machinery than a local ZDev first version needs; its checkpoint rule is
useful wherever experiments outlive a worker session.

An authorized run needs an experiment/time/compute limit, the target if one
exists, and how expensive confirmation or debugging counts. Plateau can stop
a run without proving the optimum. Report whether it stopped on target,
budget, interruption, unavailable resources, or a user decision.

Recovery should inspect authoritative tasks, the accepted revision, candidate
changes, last recorded observations, and actual running processes or jobs. An
unknown process state must not trigger a duplicate launch. Preserve ambiguous
worktrees until the worker and experiment have stopped. Durable local records
should be enough to restart the workflow without retaining old agent IDs.

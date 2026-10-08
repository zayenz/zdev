# Autonomous research

## Objective

Add an autonomous research workflow to ZDev. A research area owns a metric to
optimize, its measurement rules, and an evolving set of tasks. Ordinary tasks
prepare or improve the environment; hypothesis tasks propose changes, run
experiments, and retain measurements and lessons whether the changes win or
lose. Serial experiments use the current area branch. Parallel experiments use
isolated worktrees with concurrency suited to the workload.

This `auto` area develops that capability. It is not itself a campaign to
optimize an arbitrary ZDev metric. Research and product discussion are complete;
the next interaction turns this brief into a reviewed implementation bundle.

## Observable success

- A user can establish a research area with a primary metric, direction,
  evaluator, baseline, allowed changes, correctness requirements, and budget.
- The area can contain both ordinary work and hypothesis tasks. Hypotheses
  remain readable before execution and retain conclusions after execution.
- An authorized run generates and tests further hypotheses within the area's
  agreed scope without requiring a new planning interaction for every idea.
- Completing a well-run experiment does not require a metric improvement.
  Rejection, inconclusive evidence, and execution failure have distinct meanings.
- A serial run preserves the current accepted implementation and all experiment
  records. A parallel run evaluates isolated candidates, respects compute
  capacity, and verifies each proposed integration against the current baseline.
- A fresh session can recover the objective, current accepted result, unfinished
  work, useful failed ideas, and remaining budget from durable records.
- Codex demonstrates the complete serial and rolling workflow live. Claude
  Code receives local scenario validation, integration checks and review
  against primary workflow documentation; the user will test it live
  elsewhere later, without gating initial completion.
- Shared measurement and optional statistical helpers support area-defined
  evaluation rules, with recorded samples and an explained comparison method.
- Ending a run or exhausting the current queue does not claim that the area's
  optimization objective has been achieved or close the area implicitly.

## Boundaries

Use ZDev's existing area, task, dependency, branch, coordinator, verification,
and commit model wherever it fits. Domain-specific commands perform experiments;
the coding harness proposes hypotheses and coordinates workers. The binary owns
deterministic validation and durable state.

Start with one primary scalar metric and explicit correctness requirements.
Secondary measurements can explain trade-offs. Multi-objective search, a general
cluster scheduler, dashboards, paper writing, unrestricted changes to evaluation
rules, and cross-project memory are outside the first version. Retain readable
results, not conversation transcripts or large generated artifacts in `.zdev`.
The statistical helpers form a small reusable set, not a general statistics
framework or an automatic research-policy optimizer.

Avoid imposing new review, metadata, or test layers on every experiment. Spend
validation effort on trustworthy measurements, attributable changes, and safe
acceptance. Further structure must solve a demonstrated workflow problem.

## Terms and settled direction

These requirements come from the user's request:

- **Research area:** the brief describes what to optimize. It may mix ordinary
  tasks with tasks generated through hypothesis testing.
- **Hypothesis task:** a proposed explanation and change to test. It is durable
  work, not a transient suggestion lost with the conversation.
- **Experiment:** execution of a hypothesis under a defined evaluator and
  resource budget. Its observations inform subsequent tasks.
- **Serial execution:** work locally on the current branch.
- **Parallel execution:** use multiple worktrees; choose a suitable amount of
  concurrency for the target and resources.

### Model and records

- Research receives first-class support in the binary and skill. An area can
  declare research settings, and a task can be identified as a hypothesis.
  ZDev routes hypothesis work and validates experimental results while
  preserving ordinary tasks. The exact serialization remains an implementation
  choice; add only fields needed for these behaviors.
- The brief describes the objective, experimental reasoning, allowed changes,
  and measurement policy. Executable research settings have one authoritative
  area-level representation. Hypothesis results and repeated observations
  belong to their owning task; avoid an independent duplicate experiment ledger.
- Keep `open`/`done` task status. Store experiment decisions separately from
  task completion. Dependencies express required work; hypothesis context links
  and the starting code revision serve different purposes.
- Record a hypothesis before execution and its observations before restoring
  candidate code. Keep a concise lesson that later proposals read. A later task
  requiring retained candidate code must be created or admitted only after
  acceptance; completion of a rejected hypothesis does not establish that code.
- Retain candidate code for rejected hypotheses in accessible Git history or a
  retained reference. Restoring the accepted implementation must preserve the
  experiment's task, measurements, and lesson, plus unrelated local changes.

### Runs and generated work

- Runs use a finite time or experiment limit by default and support an explicit
  run-until-interrupted option. A target or a supplied budget can stop either
  form. Unlimited means continued in-scope experimentation, not unlimited
  resources or permission to change the objective.
- The run defines the counted unit for each limit. Repair attempts, measurement
  reruns, confirmation, and reconciliation consume the relevant budget; they
  are not free work outside it. Report target, budget, interruption, or blocker
  as distinct stop reasons. Queue exhaustion alone does not close the area or
  establish that the optimization objective has been achieved.
- An authorized research run may create and execute ordinary supporting tasks
  within the area's scope, as well as hypotheses. Changes to the evaluator,
  workload, or other comparison conditions require a new baseline before later
  candidate comparisons. A supporting task cannot silently redefine the metric
  or loosen correctness requirements to make a candidate win.

### Parallel execution and harnesses

- Keep one coordinator responsible for authoritative records and acceptance.
  Parallel workers return candidate changes and evidence from their worktrees.
- Parallel research continually refills available workers. Each candidate
  starts from the accepted revision at its dispatch; the coordinator processes
  returning results and candidate acceptance serially. If the accepted revision
  has changed, reconcile and remeasure the candidate before keeping it.
  Worker slots and constrained measurement resources remain separate limits.
- Choose concurrency within the run's limits and available harness and compute
  capacity. Parallel code preparation may share a serialized measurement slot
  when simultaneous benchmark runs would interfere. Changes to shared
  evaluation conditions pause affected candidates until comparisons are valid.
- Codex and Claude Code both support the complete initial workflow, including
  serial runs, rolling parallel worktrees, result recording, and recovery.
- Live acceptance is required for Codex. Claude Code readiness is established
  through local validation and documentation review without invoking Claude
  Code here; its later live testing is a user follow-up, not a completion gate.
- Implement the local hypothesis lifecycle before the serial loop and rolling
  parallel execution. Both execution modes and both harnesses belong to the
  initial feature; supporting other harnesses follows that complete increment.

### Completion and verification

- Run the evaluator and required correctness checks for every experiment.
  A fresh independent verifier reviews candidate code proposed for acceptance;
  the coordinator checks rejected results against the experiment's requirements
  and evidence. Rejected hypotheses do not need a fresh agent review by default.
  Ordinary tasks retain their existing independent-verification workflow.
- An accepted hypothesis completes after verification and retention of its code
  succeed. A valid rejected hypothesis completes with its measurements and
  lesson after restoring the accepted implementation. Record-only commits are
  valid completed research work.
- A hypothesis that consumes its allotted attempts without a usable answer
  completes with a distinct failed-to-run or inconclusive result, retaining the
  evidence. A justified later retry becomes a new task linked to that result.
  An interrupted hypothesis remains unfinished. A shared environment failure
  stops affected dispatch rather than generating repeated failed hypotheses.

### Measurement policy and common utilities

- Each area defines its measurement and acceptance policy; ZDev supplies common
  utilities that evaluators and workflows can use. Include repeated command
  execution, numeric sample collection, summaries such as mean/median/spread,
  directional baseline comparison and thresholds, and optional statistical
  confidence or significance helpers in the first version.
- The area selects repetition, aggregation, minimum useful gain, correctness
  checks, and any held-out acceptance procedure appropriate to its domain.
  Statistical helpers expose their assumptions and limitations and retain
  underlying samples and the method used. A confidence statistic alone does
  not establish correctness or a genuine improvement under repeated search.
- ZDev validates recorded results and applies the selected rules. Missing or
  non-finite measurements, crashes, and timeouts cannot count as improvements.
  A deterministic metric need not use repeated runs or a statistical method.

## Implementation choices remaining

No unresolved product choice blocks the task split. Implementation tasks should
choose the minimal settings/result format, the Git mechanism for retaining and
restoring candidates, and a small documented set of statistical methods.
Each choice must honor the decisions above and reuse existing repository seams.
Method-specific assumptions and focused validation belong with the utility
that implements them, not in an area-wide statistical framework.

## Research index

Research conducted 2026-10-07: 20 public repositories plus AlphaEvolve's
published design. Primary documentation was compared with selected source files;
the external systems were not run. Recommendations are design judgments, not
claims that a particular search policy will outperform alternatives in ZDev.

- [Implementation landscape](background/implementation-landscape.md): read when
  choosing upstream mechanisms to adapt. Includes source links, strengths,
  limits, and a shortlist for deeper implementation work.
- [Experiment mechanics](background/experiment-mechanics.md): read when deciding
  metric validity, hypothesis completion, memory, rollback, concurrency, and
  recovery. Separates documented behavior from source observations and proposals.
- [ZDev fit and candidate increments](background/zdev-fit-and-plan.md): read when
  settling the data model or drafting implementation tasks. Maps existing seams,
  conflicting workflow rules, and staged acceptance examples.

The background proposals predate discussion. Their finite-batch,
single-harness, and deferred-statistics recommendations are superseded by the
decisions above.

## Testing

Research and discussion change planning records only: no new tests. Validate
the area with `zdev check auto --format json`, check links and source statements,
and run `git diff --check`.

For subsequent implementation, use focused coverage for new externally visible
record and command behavior, with existing fixtures and Git workflow tests.
Exercise metric direction, invalid results, completion without improvement,
record retention through rollback, and parallel acceptance after baseline drift
where those behaviors are introduced. Do not create a broad statistical test
framework or one test per field. Statistical utility tests should check
meaningful comparison behavior and invalid inputs on small known samples;
they must not rely on stochastic live benchmarks. Use a documented trusted
reference when validating a method rather than mirroring its implementation.
An existing benchmark and live end-to-end walkthroughs in Codex demonstrate
the research loop, including rolling parallel execution. For Claude Code,
reuse passing deterministic installed-workflow scenarios, review the current
primary workflow documentation and run read-only integration checks. Do not
invoke Claude Code here; the user will test it elsewhere later. Its live
execution remains unverified and does not block completion. Finding a metric
improvement in a stochastic demo is not required for feature success.

## Validation

Use the candidate increments as a starting point and revise them for rolling
parallelism, both initial harnesses, and the common measurement/statistical
utilities. They are not an approved bundle. Create and review exact tasks next;
import only after approval through the ordinary reviewed task workflow. The
current task queue remains intentionally empty until that interaction.

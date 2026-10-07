# Autonomous research

## Objective

Add an autonomous research workflow to ZDev. A research area owns a metric to
optimize, its measurement rules, and an evolving set of tasks. Ordinary tasks
prepare or improve the environment; hypothesis tasks propose changes, run
experiments, and retain measurements and lessons whether the changes win or
lose. Serial experiments use the current area branch. Parallel experiments use
isolated worktrees with concurrency suited to the workload.

This `auto` area develops that capability. It is not itself a campaign to
optimize an arbitrary ZDev metric. This first increment collects research and
shapes the feature; implementation tasks follow discussion of the design.

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
- Ending a run or exhausting the current queue does not claim that the area's
  optimization objective has been achieved or close the area implicitly.

## Boundaries

Use ZDev's existing area, task, dependency, branch, coordinator, verification,
and commit model wherever it fits. Domain-specific commands perform experiments;
the coding harness proposes hypotheses and coordinates workers. The binary owns
deterministic validation and durable state.

Start with one primary scalar metric and explicit correctness requirements.
Secondary measurements can explain trade-offs. Multi-objective search, a general
cluster scheduler, dashboards, paper writing, self-modifying evaluators, and
cross-project memory are outside the first version. Retain readable results,
not conversation transcripts or large generated artifacts in `.zdev`.

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

Recommended design, still subject to discussion:

- Treat research as an area-level workflow with an optional hypothesis task
  distinction, rather than forcing an entire campaign into one ordinary task.
- Keep `open`/`done` task status. Store experiment decisions separately: a tested
  losing idea is completed work. Dependencies express required work, not a
  hypothesis family tree or the code revision used for an experiment.
- Record hypotheses before execution and measurements before undoing candidate
  code. Keep failed ideas with a concise explanation that later proposals read.
- Keep one coordinator responsible for authoritative records and acceptance.
  Parallel workers return candidate changes and evidence from their worktrees.
- Begin with a bounded serial loop. Add parallel batches after completion,
  measurement, rollback, and recovery semantics are demonstrated.

## Questions to settle before implementation

1. What is the smallest first-class representation: a research setting on an
   area, a hypothesis kind on a task, or both? Where should the measurement
   contract and repeated observations live without duplicate authority?
2. How much hypothesis generation and execution can one run authorize, and what
   budget or target stops it? Prefer explicit run limits over an unlimited loop.
3. What demonstrates completion of rejected, inconclusive, crashed, and
   interrupted hypotheses? Which require another attempt or a separate task?
4. How are candidate patches retained and undone locally without changing
   unrelated edits, staged files, or the durable experiment record?
5. Should parallelism begin with finite batches or immediately refill workers?
   Prefer finite batches first; distinguish agent slots from benchmark resources.
6. Which measurement policy is required per domain, including reruns, minimum
   useful improvement, and any genuinely held-out acceptance check?
7. Which supported harness receives the first complete walkthrough? Prefer one
   working harness before broad adapter changes.

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

## Testing

This research increment changes planning records only: no new tests. Validate
the area with `zdev check auto --format json`, check links and source statements,
and run `git diff --check`.

For subsequent implementation, use focused coverage for new externally visible
record and command behavior, with existing fixtures and Git workflow tests.
Exercise metric direction, invalid results, completion without improvement,
record retention through rollback, and parallel acceptance after baseline drift
where those behaviors are introduced. Do not create a broad statistical test
framework or one test per field. An existing benchmark and a short end-to-end
walkthrough provide the first demonstration of the research loop.

## Validation

Before task creation, resolve the product choices that change task completion,
authority to generate tasks, measurement validity, or Git behavior. Use the
candidate increments as a starting point, not an approved bundle. Import tasks
only through the ordinary reviewed task workflow. The current task queue is
intentionally empty during this research-first phase.

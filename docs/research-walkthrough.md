# Research toward an open-ended goal

"Make query evaluation faster" names a direction without choosing a solution.
You may need profiling, a different data structure, or several experiments
that fail before one helps. Zdev's research workflow keeps that work within
an agreed scope and records what each attempt taught you.

This walkthrough uses an illustrative query-evaluator project. It assumes
zdev is installed and an area named `evaluation` exists. The prompts are
messages to your coding agent. They do not run a research campaign merely by
being shown here.

For a feature with agreed behavior, follow
[A feature from idea to commit](feature-walkthrough.md).

## Define what improvement would mean

Start with exploration and discussion:

```text
Use zdev to explore evaluation. We want query evaluation to be faster,
but do not know the right approach yet. Identify representative workloads
and help agree on correctness, compatibility, and measurement boundaries.
Do not start optimization experiments yet.
```

The first useful result is a brief, not an optimization. It might settle that
runtime on a fixed query set is the primary metric, lower is better, and query
results and public language behavior must remain unchanged. It should also say
which performance trade-offs are acceptable and what remains outside scope.

If the project has no reliable benchmark, create and complete ordinary tasks
to establish one. Measure a baseline with the agreed correctness checks and
representative inputs. Decide how repeated samples will be compared and what
variation is too small to count as an improvement. Use actual measurements;
a plausible baseline value is not evidence.

Profiling can then suggest a checkable hypothesis, such as "reusing an existing
lookup result will reduce repeated work in these queries." It remains a
hypothesis until the candidate has been measured.

## Configure the experiment policy

Ask the agent to record research settings for `evaluation` using the agreed
evaluator command, correctness commands, baseline samples, and comparison
policy. Review those settings before authorizing experiments. The brief holds
the broader reasoning and boundaries; the settings tell the binary how to run
and compare measurements.

The CLI can inspect those settings without running the evaluator:

```sh
zdev area research evaluation
```

See the [research reference](research.md) for setup and execution details, or
[the record format](task-format.md) for the exact fields. The agent can prepare
the configuration from your agreed policy; this guide does not supply a
benchmark command for a project it has not inspected.

## Authorize a bounded run

Codex and Claude Code have installed research execution routes. With the area,
policy, and baseline ready, give the run a clear allowance:

```text
Use zdev to run serial research in evaluation within its brief, for
20 work attempts with a command limit of 100. Keep the agreed evaluator
and correctness policy.
```

This authorizes the coordinator to propose and admit hypotheses and supporting
tasks inside that scope, without asking you to approve every new bundle.
It does not authorize a different product objective or a change to the agreed
boundaries.

Twenty work attempts does not mean twenty completed experiments. Preparation,
supporting work, rework, and other counted operations consume allowance too.
Evaluator and correctness commands consume their own command allowance.
Failures and interruptions still count.

## Follow one hypothesis

The coordinator reads the accepted baseline and prior lessons before choosing
work. For a hypothesis, the workflow records the starting state, prepares a
candidate, and runs the configured correctness checks and measurements.

There are several useful outcomes:

| Observation | What happens |
| --- | --- |
| The candidate improves the metric under the policy and preserves correctness | A fresh acceptance verifier checks it before its code is kept. |
| The candidate is valid but does not improve the metric | The experiment can finish as rejected, with evidence and a lesson; its owned changes are restored. |
| The evidence is inconclusive or the experiment cannot run | The record distinguishes that result from a successful comparison. |
| Work is interrupted before a terminal result | The experiment remains unfinished and must be recovered before more work starts. |

A rejected hypothesis is useful when it rules out an approach or reveals a
bad assumption. Its evidence can prevent the next agent from repeating the
same experiment. The records retain lessons, while Git retains accepted code
and candidate references for inspection.

Supporting work follows the ordinary implementation and verification path.
When supporting code is accepted, the run must measure a new baseline before
more candidates can be compared. Changes to evaluation conditions also require
an actual rebaseline. Old measurements remain historical results, not
measurements of the new setup.

## Stop, inspect, and decide what remains

The run stops new work when its allowance or target is reached, you interrupt
it, or a blocker prevents progress. It reports the reason, accepted changes,
experimental results, and remaining allowance. Using the budget does not prove
that the broader performance objective is achieved.

Inspect progress in the terminal:

```sh
zdev research run evaluation context --format json
```

To continue an interrupted run, ask:

```text
Use zdev to resume the existing research run in evaluation.
```

Resume retains that run's identity, scope, limits, and consumed allowance. It
checks unfinished measurements and finalization before starting more work.
Resuming is not a way to reset the budget.

At the end, review the evidence against the objective. The next step may be
another explicitly authorized run, a narrower ordinary task, a revised brief,
or closing the area. No useful speedup is also a result worth recording.

## Overlap experiments when it helps

Once the serial workflow is understood, rolling research can overlap source
preparation while limiting measurements separately. For example:

```text
Use zdev to run rolling research in evaluation within its brief, with
two workers and one measurement slot, for 20 work attempts.
```

This is different from an ordinary parallel task batch. A research run can
propose more scoped work as evidence accumulates. Candidates based on older
accepted code may need reconciliation and fresh measurements before acceptance.
See [Longer runs and orchestration](longer-runs.md) for choosing the right form
of continuation, and the [research reference](research.md) for support limits
and recovery details.

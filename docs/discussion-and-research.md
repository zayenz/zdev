# Discussion and research

Some objectives start with a clear feature. Others start with a concern such
as "this is too slow" or "the errors are difficult to understand." Zdev can
help turn that concern into an agreed direction before you commit to an
implementation.

The useful distinction is between a decision you need to make and a fact you
need to establish. Discussion handles the first. Investigation gathers
evidence for the second. Measured research runs handle a sequence of
experiments once there is a suitable evaluation policy.

## Discuss what the result should mean

For the `evaluation` area, ask in your coding-agent chat:

```text
Use zdev to discuss the evaluation brief. Which query workloads should
improve, and which compatibility and validation requirements must hold?
Challenge assumptions that could make the benchmark misleading.
```

The agent reads the brief and relevant project material, then raises choices
that affect the outcome. For example, is a speedup on large query batches
useful if small interactive queries get slower? That is a product decision.
Whether a particular operation dominates runtime is a question for evidence.

Discussion updates the brief as decisions are made. You keep the conclusions
and reasons that later work needs, rather than a transcript of every exchange.
You can revisit a decision when new evidence exposes a real problem with it.

Discussion need not be a separate stage for a small, fully specified change.
The request itself may already settle the behavior and validation. In that
case, proceed to a task bundle.

## Investigate a question you can answer

Ask for a bounded investigation when a fact is missing:

```text
Use zdev to investigate where evaluation time is spent on the agreed
query set. Report the evidence and likely bottlenecks before proposing
production changes.
```

The agent can inspect code, read relevant research, reproduce a symptom, or
build a temporary prototype. The result should name the question, evidence,
conclusion, and remaining uncertainty. A promising prototype is evidence for
a decision; it does not by itself authorize production changes.

When an investigation informs an area, record the conclusion in its brief.
Keep reusable background material with the area when later tasks need it.
Standalone investigation is also possible without creating an area first.

## Use a research run for repeated experiments

A measured research run is useful when you have several plausible approaches
and a repeatable way to judge their results. It can generate hypotheses inside
the agreed scope, try them, and use the recorded lessons to choose the next
experiment.

That requires more preparation than asking an open question. Agree on the
metric, representative inputs, correctness checks, comparison policy, and
limits. Measure the baseline before starting the run. Changing the benchmark
to favor a candidate would change the question being asked.

Not every vague goal belongs in an optimization run. "Make errors easier to
understand" may first need examples of confusing errors and decisions about
the intended audience. Those can become ordinary implementation tasks once
the desired behavior is clear. A metric is useful only if it represents the
improvement you actually want.

The [research walkthrough](research-walkthrough.md) follows the performance
example through preparation, a bounded run, and interpretation of its results.
The [agent command pages](commands/index.md) describe the individual Explore,
Discuss, Investigate, and Research requests.

# Run measured research

Use research when progress is measured by experiments, such as reducing parser
runtime while preserving its output. A research area can contain ordinary
supporting tasks and hypothesis tasks. Use [investigate](investigate.md) for
a single question that does not need an ongoing experiment record.

## Examples

With the area and research policy ready, paste into chat:

```text
Use zdev to run serial research in area parsing within its brief
for 20 work attempts.
```

```text
Use zdev to resume the existing research run in area parsing.
```

## Prepare the area

A research run needs an open area with a brief and configured research policy.
The policy names the metric, direction, evaluator, measured baseline,
correctness checks, permitted changes, and attempt limits. For a `parsing`
area, you could ask:

```text
Use zdev to inspect the research settings for area parsing and explain which
measurements and correctness checks a run will use.
```

Inspecting or recording settings and hypothesis evidence does not run an
evaluator. The [research guide](../research.md) explains setup and execution;
the [research record contract](../../skills/zdev/references/task-format.md#research-settings-and-hypothesis-evidence)
defines the required settings and evidence. Supplied samples must be real
measurements.

## Authorize a run

With the brief and policy ready, explicitly authorize a run as shown above.

The agent can generate scoped hypotheses and supporting tasks, implement
candidates, run configured measurements, and retain the resulting lessons.
Keeping improving code requires both the measurement policy and an independent
acceptance review. Rejected candidates are retained for inspection while their
owned changes are restored; the completed hypothesis still records what was learned.

Twenty work attempts is the default run allowance, not twenty completed
experiments. Preparation, measurements, repair, and review consume attempts.
You can also specify a time limit, command limit, or target. A run can generate
more scoped work when its queue empties; an ordinary [loop](loop.md) stops there.

The report names accepted changes and commits, experimental decisions,
remaining allowance, and the stop reason. Reaching a budget ends new work; it
does not establish that the area's optimization objective is achieved.

## Resume or run in parallel

After an interruption, ask to resume the existing run.

Recovery retains the original run limits and consumed work, and checks pending
measurements before launching anything else. With a supporting integration,
you can explicitly request rolling parallel research with separate worker and
measurement limits. Codex supports this route; check your installed harness
integration before assuming the same continuation controls elsewhere.

See the [research guide](../research.md) for run controls and the
[local experiment lifecycle](../workflow.md#one-local-hypothesis) for a single
hypothesis. Use [recover](recover.md) for interrupted ordinary work.

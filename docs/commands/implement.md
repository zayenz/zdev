# Implement one ready task

Use implement when the area has approved tasks and you want one completed,
independently verified, and committed. "Continue" and "next task" also select
this interaction when zdev is active.

## Examples

Paste either request into your coding-agent chat:

```text
Use zdev to implement the next task in greeting.
```

```text
Use zdev to implement the next task in greeting in this checkout.
Stop after the verified commit.
```

## What happens

The agent selects a ready task, reads its brief and requirements, and checks
the branch and existing changes. It reuses an agreed workspace; otherwise the
workflow normally uses a worktree. The implementer makes the task's changes
and runs the agreed validation. An advanced task receives a planning pass first.

A different agent checks the complete result and required validation. Confirmed
problems receive one correction pass and a focused recheck. Once verification
passes, the coordinator marks the task done, stages its changes, and makes a
commit with a stable `Zdev-Change-Id`.

Expect a report naming the task, changed files, validation, review result, and
commit. The ordinary interaction stops after that commit. If the task needs an
in-scope split, it may instead commit the revised task graph and report that the
original task remains open. Missing decisions, unsafe state, or unresolved
required corrections stop with the work preserved.

The task and brief set the testing level. A small `hello.py` change does not
automatically require new test infrastructure. Existing local changes remain
protected; the agent asks if their ownership overlaps the task and is unclear.

Use [loop](loop.md) to continue through more tasks, [plan](plan.md) for planning
only, or [recover](recover.md) after interruption. See
[implementation](../workflow.md#implementation) and
[completion and commit](../workflow.md#completion-and-commit) for the full rules.

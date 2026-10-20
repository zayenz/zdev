# Tasks and completion

A task describes one piece of work that can be implemented, checked, and
recorded on its own. The area brief supplies the shared context; the task says
what this particular step must achieve.

The [feature walkthrough](feature-walkthrough.md) follows ordinary development
through discussion, task creation, implementation, review and verification,
and commits.

For the `evaluation` area, "make queries faster" is too broad for an ordinary
implementation task. "Add a repeatable benchmark for the agreed query set" is
a useful first step. It has an observable result even before you know which
optimization to try.

## Turn the brief into tasks

Once the outcome and testing choices are clear, ask your coding agent:

```text
Use zdev to create tasks for evaluation from its brief. Start with the
benchmark and evidence we need before choosing optimizations. Show the
proposed bundle before importing it.
```

Each proposed task should explain its outcome, boundaries, completion
conditions, and validation. Dependencies say which other tasks must finish
first. Priority helps order work, while the `afk` setting records whether a
task is suitable for unattended execution.

For example, an ordinary task might describe:

```text
Outcome: A repeatable benchmark for the agreed query set.
Boundaries: Preserve evaluator behavior and query fixtures.
Done when: The documented command reports samples for that set, and the
existing correctness checks still pass.
Validation: Run that benchmark and the project's agreed correctness checks.
```

This is an illustrative task outline, not an import bundle. Zdev's
[task format](task-format.md) defines the stored fields.

Review the proposed work before import, or explicitly delegate creation and
import to the agent. Importing tasks records the plan. It starts implementation
only when your request also authorizes that next step.

## A ready task has satisfied dependencies

Zdev selects from tasks whose dependencies are satisfied. A task can remain
open while blocked by another task, a missing decision, or an unsafe branch
state. Making it higher priority does not remove those prerequisites.

Ask for one task when you want to inspect the result before continuing:

```text
Use zdev to implement the next task in evaluation. Stop after its
verified commit.
```

The coordinating agent selects work, checks the workspace, and delegates the
implementation. A separate verifier checks the result against the task, the
area brief, and repository guidance. After a pass, the coordinator records
completion and commits the accepted changes. Required corrections get a
correction pass and a focused recheck; unresolved corrections leave the task
open for a continuation decision.

The task's validation should match its risk. A documentation change can use
existing checks. A change to evaluator behavior may need focused correctness
coverage. A task does not automatically justify a new test framework.

## Ordinary tasks and hypotheses have different outcomes

An ordinary task promises a result, such as a working benchmark or a fixed
bug. A hypothesis asks whether an experimental change produces the predicted
improvement under an agreed measurement policy.

Both use task records, but hypotheses run through the separate research
workflow. A rejected experiment can be complete: it answered the question,
recorded the evidence, and restored its owned changes. That does not mean its
candidate code was accepted or the area's objective was achieved. Interrupted
experiments remain open until their state is recovered and resolved.

Use an ordinary task when the result is specified. Use a hypothesis when the
result needs to be learned by experiment. The
[research walkthrough](research-walkthrough.md) shows how an area can contain
both.

## Completion leaves a record

Task inspection shows its outcome and validation. Accepted source changes
have a commit with a stable `Zdev-Change-Id`, which can still identify the
logical change after a rebase. The storage policy determines whether the task
record itself is committed or remains local.

A task can finish while the area still has unresolved questions. Before making
the next task, use [discussion and research](discussion-and-research.md) to
settle the choices that would materially change the work.

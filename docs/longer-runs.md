# Longer runs and orchestration

For more than one task, a coordinating agent keeps track of what is ready,
assigns work, checks the results, and records accepted changes. That coordination
is what orchestration means in zdev. The area and tasks keep the durable record;
your coding harness supplies the agents and continuation mechanism.

You will encounter names such as goal, loop, and workflow. They describe parts
of how work continues, rather than separate kinds of planning record.

## Choose what should continue

| What you want | Use |
| --- | --- |
| Complete one ready task and inspect the result | [Implement](commands/implement.md) |
| Continue through a named area's approved tasks | [Goal or loop](commands/loop.md) |
| Work on compatible ready tasks at the same time | [Parallel tasks](commands/parallel.md), on its own or inside an authorized loop |
| Generate and evaluate hypotheses within an agreed experimental scope | [A research run](research-walkthrough.md) |

Goal and loop are synonymous requests in an active zdev conversation. An
ordinary loop reads fresh task state after each commit and stops when the
approved queue is finished or a blocker needs attention. It does not invent
new tasks because the queue is empty. A research run has separate authority
to propose scoped work while its allowance remains.

Ordinary implementation, loop, and parallel routes stop before dispatching a
selected hypothesis task. In a research area, use the research route to
coordinate hypotheses and ordinary supporting work together.

## Ask for a run with a clear stopping point

For the `evaluation` area's ordinary supporting tasks, ask in chat:

```text
Use zdev to loop through evaluation's approved ordinary tasks. Stop when
they are finished or a blocker needs my input.
```

You can state a focus, such as prioritizing benchmark usability. Focus guides
selection among eligible tasks; it does not remove dependencies or change
the area's scope. Hypothesis tasks need the separate research route.

For compatible work, explicitly authorize parallel continuation and its limits:

```text
Use zdev to loop through evaluation's approved ordinary tasks using
compatible parallel batches. Use at most two task slots and keep the
worktrees for inspection when the run stops.
```

Parallel work uses separate source worktrees. Integration, verification, and
accepted task commits still happen one task at a time. Tasks that share a
changing interface or validation resource may need to run sequentially even
when their dependency lists are independent.

Each task keeps its agreed validation and independent verification. A longer
run changes how the next task starts, not what counts as an accepted result.

## How the harness carries the run

Codex can use its native goal tools for continuation. Claude Code packages
coordination in installed workflows. Other harnesses use their available
agent and continuation facilities; some return control after one task and
need another invocation to continue.

A workflow is the harness's execution procedure. It reads the same area and
task records as another session, but its running state belongs to the harness.
The CLI does not become a background agent service. In particular,
`zdev goal evaluation` prints continuation context; it does not start a loop.
Start execution through the installed skill in your coding-agent chat.

Use the [harness support notes](user-guide.md#harness-notes) to check the
integration you are using. A report that continuation is needed is not a
promise that more work is running in the background.

## Return to a longer run

Progress reports should name the selected task, accepted commit, validation,
and any blocker. You can inspect durable task state at any point:

```sh
zdev status evaluation
zdev tasks list evaluation
```

After interruption, ask the agent to recover the unfinished work before
selecting another task. It needs to distinguish a partial implementation,
a completed verification, and a commit that did not finish. Research recovery
also preserves measured evidence and the run's consumed allowance.

An exhausted queue leaves the area open. Decide whether the result meets the
objective before closing it or approving more work. The
[workflow reference](workflow.md) covers branch safety and completion, while
[continuation](area-loop.md) and [research](research.md) document the detailed
stop and recovery rules.

# Plan one ready task

Use plan when you want to examine an approach before allowing implementation
of an imported task. The exact phrase "plan next task with advanced planner"
selects this route, including for a routine or standard task.

## Examples

Paste either request into your coding-agent chat:

```text
Use zdev to plan next task with advanced planner in area greeting.
```

```text
Use zdev to plan next task with advanced planner in area greeting.
Focus on compatibility with the current command-line behavior. Do not implement it.
```

## What happens

The agent selects a ready task and gives a fresh planner the brief, task,
repository guidance, and current checkout context. For `--name`, the plan might
explain where argument parsing belongs, how the default greeting is preserved,
and which existing checks prove the behavior.

The result identifies the task and presents the approach, likely files,
validation, and any unresolved findings. It also reports the planner settings.
Planning stops there. Source files, `.zdev`, Git state, and task status remain
unchanged; the plan stays in the conversation.

To use it, explicitly request [implementation](implement.md). The agent checks
whether the task and relevant code still match the plan before reusing it. The
one-off advanced planner choice does not change the task's stored complexity or
select a different implementation or verification profile.

Use [create tasks](tasks.md) when you need to split an objective into tasks.
See [task complexity routing](../task-complexity-routing.md) and
[worker profiles](../worker-profiles.md) for how planning and worker choices differ.

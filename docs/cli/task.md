# zdev task

Read or update one task's durable record. Completion records what was achieved
and which checks passed, while reopening returns a completed task to the open
queue.

## Examples

```sh
zdev task show greeting greeting-001
zdev task done greeting greeting-001 \
  --summary 'Added the named greeting' \
  --validation 'Verified the greeting with a supplied name'
```

## Inspect and complete

```text
zdev task show AREA TASK
zdev task done AREA TASK --summary SUMMARY --validation RESULT...
zdev task reopen AREA TASK
```

`show` prints the full Markdown task. `done` requires a summary and at least
one validation result; repeat `--validation` for independent checks that
actually passed. Completion and reopening both regenerate `TASKS.md`.

Completion checks the area's branch state. It records validation supplied by
the caller, so run the required checks and independent verification before
marking the task done. Then stage the intended files and use
[commit](commit.md). See [workflow](../workflow.md#completion-and-commit) for
completion and [task format](../task-format.md) for task states.

## Research evidence

```text
zdev task research AREA TASK
zdev task research AREA TASK --from PATH
```

Without `--from`, read the hypothesis declaration and result history. With it,
append validated JSON evidence; `--from -` reads standard input. This operation
records supplied evidence. Use [research](research.md) to execute managed
measurements and finalize experiments.

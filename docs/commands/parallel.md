# Run compatible tasks in parallel

Use parallel when an area has at least two ready tasks that can proceed
independently. Two tasks changing the same parser or generated file may be a
poor match even when neither depends on the other. A single `--name` change
usually needs only [one implementation](implement.md).

Replace the example task IDs with ready tasks from your queue.

Codex, Claude Code, Pi, and capable Oh My Pi sessions support this route.
OpenCode currently offers sequential Implement because its foreground Task
surface cannot hand back each completion. T3 orchestration is also sequential.
See [harness support](../user-guide.md#harness-notes) for capability checks.

## Examples

Paste either request into your coding-agent chat:

```text
Use zdev to run greeting-003 and greeting-004 in parallel in area greeting.
Stop after this batch. Keep the worktrees for inspection.
```

```text
Use zdev to run greeting-003 and greeting-004 as one parallel batch
with at most two task slots. Keep the worktrees after the batch.
```

## What happens

The agent checks whether the tasks can safely share interfaces, validation
resources, and the destination checkout. Each admitted task gets its own source
worktree. The default is four task slots, bounded by available harness capacity;
you can request a smaller limit. Parallel work consumes model usage faster.

Implementations can overlap. Integration, independent verification, and the
final task commits happen one at a time in the destination. Each accepted task
has its own commit. A task-local blocker preserves that task's worktree while
unaffected work may continue.

A finite batch stops after the named tasks. To refill slots throughout an area,
explicitly request a [loop](loop.md) using parallel work. Cleanup also needs
authorization; unfinished or ambiguous worktrees are retained.

The final report separates committed tasks from preserved unfinished work and
names any recovery needed. See [recovery](recover.md), the
[workflow reference](../workflow.md), and the
[continuation reference](../area-loop.md) for the surrounding lifecycle.

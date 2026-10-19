# zdev goal

Render the current continuation goal for an area, including the task context
needed to decide what comes next. Use it to inspect how a continuing session
would approach the area's remaining work.

## Examples

```sh
zdev goal greeting
zdev goal greeting \
  --focus 'Finish the command-line greeting first' \
  --format json
```

## Usage

```text
zdev goal AREA [--focus TEXT] [--parallel]
```

`--focus` supplies guidance for choosing work across the area. `--parallel`
renders continuation for compatible tasks in independent worktree slots.

The command renders context; the harness's continuation mechanism runs the
work. The [goal and loop guide](../area-loop.md) explains stopping conditions,
and [harness goals](../harness-goals.md) describes support in each harness.
Use [next](next.md) when you only need the next ready task.

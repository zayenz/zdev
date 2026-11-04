# zdev goal

Render the current continuation goal for one area, several areas, or every
area. One area includes the task context needed to decide what comes next.
Several areas or `--all` list each area's queue and next task under one goal
that continues across them.

## Examples

```sh
zdev goal greeting
zdev goal greeting \
  --focus 'Finish the command-line greeting first' \
  --format json
zdev goal greeting docs
zdev goal --all --format json
```

## Usage

```text
zdev goal AREA... [--focus TEXT] [--parallel]
zdev goal --all [--focus TEXT] [--parallel]
```

Name areas in priority order: the goal works in the first listed area with
ready, safe work. `--all` covers every open area in tag order and cannot be
combined with area names. Naming an area twice is an error. `--focus` supplies
guidance for choosing work across the covered areas. `--parallel` renders
continuation for compatible tasks in independent worktree slots.

The command renders context; the harness's continuation mechanism runs the
work. The [goal and loop guide](../area-loop.md) explains stopping conditions,
and [harness goals](../harness-goals.md) describes support in each harness.
Use [next](next.md) when you only need the next ready task.

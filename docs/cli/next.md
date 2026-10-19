# zdev next

Select the next ready task from an area. The result gives an agent or a person
a concrete task to read before starting implementation.

## Examples

```sh
zdev next greeting
zdev next --any --format json
```

## Usage

```text
zdev next [AREA]
zdev next --any
```

An explicit area keeps selection within that objective. Omit the area when a
configured default or the sole area with open work makes the choice
unambiguous. `--any` selects ready work across structurally safe areas and
cannot be combined with an area argument.

Selection does not start an agent or change task state. Read the full task
with [task show](task.md), or use [work-context](work-context.md) to capture the
task alongside its Git context. See [task format](../task-format.md) for
readiness, dependencies, and priority.

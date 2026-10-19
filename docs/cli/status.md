# zdev status

Show how much work remains and whether an area's branch is ready for work.
The report combines ready, blocked, and completed task counts with branch
health, so it is a useful first check when returning to an area.

## Examples

```sh
zdev status greeting
zdev status --format json
```

## Usage

```text
zdev status [AREA]
```

With an area, the report describes that area. Without one, zdev uses the
configured default area or shows a project-wide summary. Set a default with
`zdev config set project.default-area greeting`.

The command reads the current state. Use [next](next.md) to select ready work,
[check](check.md) to validate the record, and [area](area.md) to manage branch
bindings. The [workflow guide](../workflow.md#branch-orientation) explains
branch health and when a rebase is needed.

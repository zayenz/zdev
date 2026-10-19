# zdev check

Validate zdev's briefs, task files, generated indexes, and area relationships.
Run it after editing records by hand or before handing an area to another
worker.

## Examples

```sh
zdev check greeting
zdev check --format json
```

## Usage

```text
zdev check [AREA]
```

Supply an area to check its records, or omit it to check every area. The command
reports inconsistencies without rewriting files.

If an index is stale, regenerate it with `zdev tasks index greeting`. Other
findings may need a change to the task, brief, or area configuration. The
[task format](../task-format.md) describes the record rules; [status](status.md)
shows current task counts and branch health.

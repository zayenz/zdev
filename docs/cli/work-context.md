# zdev work-context

Capture an area's current task and Git state together so a worker can start
from a checked context. You can also save a snapshot and later compare it with
the repository before acting on earlier work.

## Examples

```sh
zdev work-context greeting --format json
zdev work-context greeting --task greeting-001 --store
```

## Usage

```text
zdev work-context AREA [--task TASK] [--store]
zdev work-context AREA --show SNAPSHOT
zdev work-context AREA --compare SNAPSHOT
```

`--task` chooses a task from the current ready frontier. `--store` saves the
exact JSON context under Git administrative state and returns a reference.
Pass that reference to `--show` to read the saved context or `--compare` to
check it against fresh context. These three snapshot operations are mutually
exclusive; `--task` cannot accompany `--show` or `--compare`.

For an open area, context includes matching status and staged, unstaged, and
untracked changes. Closed areas return branch-independent goal context.
Reading or comparing context leaves tracked files, the index, and the worktree
unchanged. See [workflow](../workflow.md) for when workers capture fresh
context and [config](config.md) for dispatch specifications that use snapshots.

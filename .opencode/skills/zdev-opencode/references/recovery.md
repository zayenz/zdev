# Recover interrupted zdev work

## Rebase recovery

Use `zdev area rebase <area>` for ordinary trunk updates and parent-area updates.
Zdev rebases only the checked-out area branch. It does not merge or recursively
update descendant areas.

If conflicts stop Git, resolve and stage them, then continue or abort:

```text
zdev area rebase <area> --continue
zdev area rebase <area> --abort
```

If someone completed the operation with `git rebase --continue`, rerun the
normal zdev area rebase command to verify the result and finalize the base
anchor. For a longer chain, update one link at a time from parent to child.

Run `zdev status <area> --format json` again before task completion. Completion
requires the recorded branch, a fresh effective base, a valid anchor, and
finalized base state.

## Resume task work

Inspect area status and reconstruct the Git baseline: status with untracked
files, cached diff, and unstaged diff. Finish or abort an active rebase first,
or finalize a rebase completed directly through Git. Then rerun `zdev next`.

Resume or verify an open task only after attributing its changes. Restart an
open task without task-owned changes. Inspect a done task and the cached diff
before committing or reopening it. Ask the user when ownership cannot be
re-established from the task, baseline, and conversation. Do not assume an
existing diff belongs to the selected task.

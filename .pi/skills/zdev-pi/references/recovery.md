# Recover interrupted zdev work

## Rebase recovery

Use `zdev area rebase <area>` for ordinary trunk updates and parent-area updates.
Zdev rebases only the checked-out area branch. It does not merge or recursively
update descendant areas.

An explicit trunk area follows configured `project.trunk` directly. It never
needs rebase recovery or freshness ceremony; `zdev area rebase <area>` is a
read-only unchanged result. Resolve a wrong, detached, missing, or reconfigured
trunk through the reported status instead of rebasing the area.

If conflicts stop Git, resolve and stage them, then continue or abort:

```text
zdev area rebase <area> --continue
zdev area rebase <area> --abort
```

If someone completed the operation with `git rebase --continue`, rerun the
normal zdev area rebase command to verify the result and finalize the base
anchor. For a longer chain, update one link at a time from parent to child.

Run `zdev work-context <area> --format json` again before task completion. For
open work, completion requires nested `branch_status.task_work.safe` to be
true and the task and Git evidence to match the verified result. A
stale-but-safe link is advisory: report `zdev area rebase <area>` once and continue. Rebase explicitly
when the task needs newer base changes or is approaching integration. Wrong or
detached branches, invalid or unavailable ancestry, nonlinear child history,
and active Git recovery operations remain blockers.

## Resume task work

Run work-context and use its exact HEAD, untracked status, cached diff, and
unstaged diff as the reconstructed baseline. Finish or abort an active rebase
first, or finalize a rebase completed directly through Git. Then rerun
`zdev work-context <area> --format json`.

Resume or verify an open task only after attributing its changes. Restart an
open task without task-owned changes. Inspect a done task and the cached diff
before committing or reopening it. Ask the user when ownership cannot be
re-established from the task, baseline, and conversation. Do not assume an
existing diff belongs to the selected task.

Prefer a durable managed checkpoint when the work is already complete enough
for one. Finish lifecycle, exact staging, and `zdev commit` for an independently
verified task instead of repeatedly reporting its attributable diff as a dirty
state. Use a route-defined zdev commit for complete planning, import, derived
graph, or repair work before refreshing task context. Do not checkpoint
incomplete implementation, skip required verification, or include unrelated
paths merely to make task work appear clean.

Recover derived work from current filesystem state:

- For an unapplied proposal, preserve its bytes and collect fresh work-context.
- When current mechanical gates pass and the user must make the semantic
  choice, create a stored review, present its Markdown with `--show`, and apply
  the approved review ID with `--reviewed`.
- After a committed apply, discard the transient proposal and follow the fresh
  ordinary task graph. A split source remains open through its children and
  final integration.

Changed or unsafe state stays preserved for ordinary recovery. A later selected
task may produce a fresh proposal under fresh gates.

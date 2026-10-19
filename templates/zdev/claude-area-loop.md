# Claude Code area continuation

Resolve and retain the run's concrete role settings before the first admitted
iteration. Every continued iteration uses those values.

For explicit blanket approval of compatible parallel batches throughout this
run, use `references/parallel.md` under coordinator control. Invoke the packaged
`zdev-parallel` workflow for each admitted finite batch, passing the retained
five-role map as `frozen_profiles`. Retain the approval scope and concrete role
settings, and select afresh after it completes. Reuse
decided choices within that authority. The ordinary packaged `zdev-loop`
workflow below remains the sequential route.

Treat “goal” and “loop” as the same zdev route. When packaged workflows are
available, the root zdev skill uses `zdev-loop` internally; `zdev-goal` is the
same workflow under an alias. The workflow repeats the ordinary one-task route,
refreshes work context after each verified commit, and stops at the shared stop
states. It does not inspect or invoke Claude Code's separate `/goal` command.

When packaged workflows are unavailable, continue under coordinator control.
If reliable continuation is unavailable, complete at most one task and report
the fresh next state with the canonical `zdev-loop` result.

This ordinary loop does not generate work or dispatch hypotheses. An explicit
research-run authorization uses the separate installed research reference and
`zdev-research` workflow.

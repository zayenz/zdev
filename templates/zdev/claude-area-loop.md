# Claude Code area continuation

Treat “goal” and “loop” as the same zdev route. When packaged workflows are
available, the root zdev skill uses `zdev-loop` internally; `zdev-goal` is the
same workflow under an alias. The workflow repeats the ordinary one-task route,
refreshes work context after each verified commit, and stops at the shared stop
states. It does not inspect or invoke Claude Code's separate `/goal` command.

When packaged workflows are unavailable, continue under coordinator control.
If reliable continuation is unavailable, complete at most one task and report
the fresh next state with the canonical `zdev-loop` result.

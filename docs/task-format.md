# Task format

The [task-format reference](../skills/zdev/references/task-format.md) defines
project and area metadata, task files, and task bundles. The same reference is
bundled with the installed skill.

Its [research contract](../skills/zdev/references/task-format.md#research-settings-and-hypothesis-evidence)
defines `area research`, mixed hypothesis bundles, and `task research` evidence.
Task completion and experiment decisions remain separate. The
[local experiment path](../skills/zdev/references/task-format.md#complete-one-local-experiment)
executes correctness checks and measurements, retains candidate Git code, and
keeps or restores owned files before completing and committing the result.
`requires_code` admits later work only after verified acceptance. The installed
[Codex serial research route](../skills/zdev/references/research.md) generates
scoped supporting tasks and hypotheses and coordinates their execution.
Parallel research and execution routes for other harnesses follow later.

The [research run contract](../skills/zdev/references/task-format.md#authorize-and-recover-a-research-run)
authorizes generated mixed work, counts dispatch and command costs, and retains
limits through recovery. Its public commands expose current accepted samples,
task lessons, unfinished work and remaining allowance. Evaluator or accepted
supporting-code changes require an actual counted rebaseline. Queue exhaustion
and stopping a run leave the area open.

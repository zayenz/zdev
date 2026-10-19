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
[Codex research route](../skills/zdev/references/research.md) and
[Claude research route](../.claude/skills/zdev/skills/zdev/references/research.md)
generate scoped supporting tasks and hypotheses and coordinate execution.
Codex and Claude Code also coordinate rolling source worktrees. Other harnesses
can use the local experiment commands; automatic research routes are not provided
for them.

The [research run contract](../skills/zdev/references/task-format.md#authorize-and-recover-a-research-run)
authorizes generated mixed work, counts dispatch and command costs, and retains
limits through recovery. Its public commands expose current accepted samples,
task lessons, unfinished work and remaining allowance. Evaluator or accepted
supporting-code changes require an actual counted rebaseline. Queue exhaustion
and stopping a run leave the area open.


Isolated hypothesis candidates use one authoritative checkout for records and
acceptance. An authorized research run admits linked source worktrees with
`research experiment <area> <task> admit --source <root> --path <file>`. Workers
read authoritative records by absolute path and edit only source code. After
confirmed worker stop, `return --worker-stopped` retains Git transport; `evaluate`
measures in the source; `integrate` claims the serialized destination acceptance
lane and captures its verification snapshot. `reconcile` refreshes a stopped
candidate after baseline drift and charges fresh measurement work. Separate
positive `--worker-limit` and `--measurement-limit` run capacities default to one.

Each attempt keeps its comparison facts. Evaluation changes pause candidates
until an actual counted rebaseline; resumed candidates reconcile and remeasure.
Historical isolated rejection retains evidence and records `accepted_revision`
for the current destination. It preserves accepted sibling code. Inspect/recover
identifies active, returned, terminal, paused and ambiguous candidates, retaining
unknown worker/process capacity. The binary never removes worktrees. See the
installed task-format and recovery references for the complete command contract.


`reserve --worker-slot` counts native planning, implementation, verification and
finding checks against worker capacity, separately from measurements. Exact
retries never grant another dispatch; task, kind and requested mode stay fixed.
Admit and Prepare transfer their own unused claims without double-counting.
The integrated acceptance owner may claim confirmation or repair capacity.
Prepare records authorized withdrawal, restores only exact measured destination
paths to unchanged accepted HEAD and preserves unrelated work and historical
evidence. Recover completes that withdrawal without worker permission. Source
repair then requires Return, fresh Evaluate, Integrate and a fresh verifier.

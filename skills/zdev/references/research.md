# Run research in Codex

## When

Use this route only when the user explicitly authorizes a research run in a
named area, or asks to resume its existing run. For example: `$zdev research
run work within the brief, for 20 work attempts` or `$zdev resume research run
work`. Explicit `until interrupted` removes the default attempt limit; it
does not remove supplied targets, time or command limits. Ordinary Implement,
Plan, Verify, Parallel and Loop keep their hypothesis guards. Serial research uses the current branch. An explicit rolling parallel research
request also authorizes run-owned source worktrees within the stated limits.
Both modes keep one coordinator for authoritative records and acceptance.

Read the installed task-format reference's research sections for the exact
policy, TaskBundle, run and local experiment contracts.
The exact installed task-workflows contract path is
"references/task-workflows.md". Decode that string and resolve a
relative path against the directory containing the loaded root zdev SKILL.md. Its sibling
`coding.md` supplies role guidance. Include those
exact installed paths and this research reference in applicable worker payloads.

The coordinator owns proposals, admission, authoritative records, acceptance,
ordinary completion and commits. The binary owns authorization, accounting,
measurement, candidate retention/restoration and managed hypothesis completion.
Workers return the existing role results and never edit `.zdev`, stage, change
lifecycle, finalize or commit. Give every worker the absolute repository cwd,
brief, task, repository instructions, coding guidance and stored snapshot;
require all its file and command tools to use that cwd. Keep worker cleanup
and concrete model/effort handling from the root Codex skill.

## Authorize or resume

1. Before repository work, call the exposed `get_goal`. Preserve a different
   unfinished goal. Continue a matching active research goal. For paused,
   budget-limited or otherwise non-running state, follow the actual tool's
   lifecycle rules; ask for harness resume when model-facing resume is absent.
   Missing or uncertain inspection blocks this route.
2. Read the area brief first, then `zdev area research <area> --format json`
   and fresh `zdev work-context <area> --format json`. Require an open area,
   valid settings, safe task work and attributable staged, unstaged and
   untracked changes. Report a stale-but-safe advisory once and continue.
   Preserve unexplained or overlapping changes. An empty or exhausted queue
   permits generation after authorization; it is not a successful research stop.
   An open `blocked` queue means every open task waits on a task in another
   area. It is no-work: report each wait in the goal's `cross_area_waits` as
   `<task> blocked by <id> (<area>: <status>)` and stop without starting,
   resuming or generating work.
3. Read `zdev research run <area> context --format json`; a reported absence
   of a run is expected for first authorization. For a new explicit
   authorization, start only when no unfinished run exists:

   ```text
   zdev research run <area> start --scope "<authorized scope>" --format json
   ```

   Pass the user's supplied `--attempt-limit`, `--command-limit`,
   `--command-seconds`, `--time-limit-seconds` and directional `--target`.
   The default is 20 coordinator work attempts. Pass `--until-interrupted`
   only on an explicit request. Retain `run.id`; never start another run to
   refresh an exhausted allowance or deadline. On resume, reconstruct from
   context and recover unfinished effects first, then use `resume --run <run>`
   when authorized. Identity, scope, target, current deadline and costs survive.
   When the user grants more allowance, call `research run <area> extend --run
   <run> --request <stable-id>` with only the authorized `--add-attempts`,
   `--add-commands`, `--add-command-seconds`, or `--add-time-seconds`. Time adds
   to the existing deadline. Retry the same request with identical additions;
   it never adds twice. Extension preserves unresolved work and stop state;
   recover and resume normally. It changes no experimental policy or target.
4. Resolve and freeze all five roles with `zdev config profile resolve codex
   <role> --area <area> ... --format json` under the shared scoped-profile
   contract. Retain the returned concrete planner, routine-implementer,
   implementer, advanced-implementer and verifier settings in the handoff.
   Restore them on continuation. When a new session has no concrete handoff,
   explain that limitation and resolve fresh settings before the next dispatch.
   Do not call ordinary `dispatch-spec` for a hypothesis or remove its guard.
5. With clear native state, use exposed `create_goal` only for this explicitly
   requested run. Use an objective that names the area, authorized scope and
   current durable authorization, requires the selected serial or rolling contract and durable recovery, and
   distinguishes ending the run from achieving the area's optimization objective.
   Do not add a native token budget unless the user requested one. After native
   activation, carry out the run in the foreground; supported idle continuation
   may continue that same goal. Do not invent background scheduling or goal APIs.

If inspection proves no unfinished native goal but creation is absent, disabled
or fails without creating one, perform at most **one foreground work-item
iteration**, including its required counted rework and completion. State that
exact extent and return control. A generation or rebaseline-only iteration also
uses that bound. If creation outcome is uncertain, inspect again; uncertainty
blocks instead of layering fallback work over a possible goal.

## Coordinate each iteration

Read fresh run context before any new work. It supplies current settings,
accepted baseline, task evidence and lessons, admissions, reservations,
unfinished checkpoints, costs, remaining allowance, `stop_reason` and
`next_action`. Read rejected, inconclusive and failed-to-run lessons before
proposing another hypothesis. Fresh task/work-context admission remains required
at each worker boundary and after every commit. Never reuse a completed task's
selection or ignore `requires_code` and dependencies.

First recover pending admission publication with the identical bundle and
request identity. Recover measurement/finalization through the owning local
experiment's `inspect` and `recover`. Never relaunch an unresolved measurement.
Finish an ordinary independently verified completion/commit checkpoint under
the existing task-workflows rules. Settle its completed reservation. Do not
interpret an old reservation as permission for a second dispatch; inspect what
actually ran, and retain failure or interruption costs.

Check the recorded stop reason and comparison validity before dispatch. Target,
budget and interruption stop new costly work but permit recording, restoration
and previously authorized finalization. A shared blocker halts hypothesis
preparation and measurement dispatch. Within the retained allowance, the
coordinator may generate and reserve ordinary supporting repairs to that
blocker. After resolving it, use `resolve --run <run>` and a counted actual
`rebaseline --run <run> --request <new-request>` before comparisons resume.
Do not repeatedly fail hypotheses against the same broken environment.

When context reports `unreachable_minimum_gain`, the declared `metric_bound`
shows that no candidate can meet the frozen minimum gain from the accepted
baseline. Prepare no further candidates for this study. This is a study decision
for the user: keep the frozen study, end it, or declare a new study with its own
threshold and a newly measured baseline. Never lower the frozen threshold
yourself. The warning is advisory; changing only the bound never invalidates
comparisons or unfinished experiments.

When comparison conditions or accepted supporting code change, retain or
restore unfinished candidates, then run actual rebaseline on clean accepted
HEAD. With declared `relevant_inputs`, only changes inside them count as
accepted supporting code changes; without them, every path outside `.zdev`
counts. Supplied baseline samples cannot clear invalidation. A supporting task
cannot change the metric, direction or correctness rules to make a candidate win.
An unsafe checkout or a user-owned scope/correctness decision stops with the
current work preserved.

When a correctness command must change only because a file it names moved or
was renamed, and the check keeps its meaning, the coordinator may declare an
administrative amendment. First finish unfinished experiments, measurements and
rebaselines, then run
`zdev area research <area> --from <file> --administrative "<reason>"`.
The run keeps its costs and comparison validity and records the amendment in
run context. A changed horizon, workload, solver or acceptance rule inside a
command is a material change: stop for the user's decision about a new study
with a newly measured baseline. When it is unclear whether a command change
keeps the check's meaning, stop for the user instead of declaring it
administrative.

### Generate work

Use the brief, current accepted result and prior lessons to propose a small
mixed TaskBundle of ordinary supporting tasks and hypotheses. Describe a
testable mechanism, useful completion evidence and allowed code changes.
Keep dependencies and context links distinct; admit a `requires_code` link
only after the referenced candidate has completed verified acceptance. Empty
or exhausted queues allow another scoped proposal while allowance remains. When
run context reports no ready task and lists `cross_area_waits`, the remaining
work waits on other areas: record `stop --run <run>`, report those waits with
stop reason `cross-area-wait`, and propose nothing.

```text
zdev research run <area> admit --run <run> --request <unique-generation-request> --from <bundle.json> --format json
```

Admission uses this run's authorization; no separate approval is required for
work within its brief, policy and scope. Outside a run, ordinary review/import
authority remains unchanged. Reuse a request only to retry the same interrupted
admission. Inspect the allocated IDs and exact delta, then stage only admitted
task files and the generated `TASKS.md`, inspect the cached diff and use
`zdev commit`. Commit those records before experiment begin. Do not stage the
area directory or unrelated records. Refresh context after publication/commit.

### Execute ordinary supporting work

Admit the selected supporting task with `work-context <area> --task <task>
--store --format json`; keep that explicit task at subsequent worker boundaries.
Do not let area-only selection replace an admitted repair with an older hypothesis.
Use the installed ordinary task-workflows contract for complexity, required
advanced planning, implementation, fresh independent verification, rework,
task completion, exact staging and commit. Wrap each planning, implementation,
rework or verification dispatch with a new counted supporting reservation
before launching that worker:

```text
zdev research run <area> reserve --run <run> --request <unique-support-request> --task <task> --kind supporting --format json
```

Retain the current request with its worker. Settle completed dispatches with
`settle --run <run> --request <request>`; failed/interrupted work receives no
refund and is recovered before another dispatch. A same-task retry needs a
new request. Keep ordinary strict planner, implementer and verifier results,
snapshot comparison, completion and commit rules. After accepted supporting
code or authorized evaluator/workload changes, rebaseline before candidate
comparison. Ordinary support does not use experiment finalize.

### Execute one hypothesis

1. Admit the ready hypothesis with explicit `work-context --task <task>
   --store --format json`. Read its full task and policy. Choose exact clean
   owned candidate files before begin; include new files by name and exclude
   `.zdev`, directories, symlinks and unrelated changes. For advanced work,
   reserve a unique `preparation` request before the fresh read-only planner,
   use the ordinary strict semantic plan and public-envelope validation, and
   settle that completed planning dispatch. Pass its semantic plan unchanged
   to the advanced implementer; do not plan again during recovery or rework.
2. Call the local lifecycle before edits:

   ```text
   zdev research experiment <area> <task> begin --path <exact-file> ... --format json
   ```

   Begin charges candidate preparation before the implementer dispatch, or
   binds one unused preparation reservation with `--reservation <request>`.
   Preserve its starting baseline and snapshot. Refresh explicit-task admission
   before dispatch. Workers edit only the declared files and run applicable
   validation; coordination inspects their exact candidate delta and strict
   implementer result. A newly necessary file requires a safe declared-path
   resolution before editing it. Workers report that need without editing the
   file. Preserve the checkpoint and use its lifecycle to record/restore the
   current experiment before admitting a revised hypothesis with those exact
   files; never silently broaden checkpoint ownership.
3. Evaluate through `research experiment <area> <task> evaluate --format json`.
   This reserves measurement work and every correctness/evaluator invocation
   before launch, records actual samples and costs, and publishes an assessment.
   For candidate repair, rerun, confirmation or reconciliation, first use a new
   run `reserve` request of that kind; reserve before repair edits/worker launch,
   then bind it with `evaluate --reservation <request>`. Never reuse a request
   to launch another dispatch. Per-hypothesis attempts and run limits both apply.
4. Inspect the actual assessment and checkpoint. Only a candidate proposed for
   keep gets a fresh independent acceptance verifier. Reserve its confirmation
   dispatch first and settle it afterward. Give it the checkpoint's
   `verification_snapshot`, starting baseline, task, candidate paths, recorded
   samples and configured correctness/acceptance policy. Require it to read
   coding **Review** and **Code standards** and check the whole acceptance
   claim. It returns the ordinary strict four-field semantic verifier object.
   Coordination validates it, compares the supplied snapshot with fresh
   work-context, and constructs the nine-key public verifier envelope with
   `work_context_snapshot: <snapshot>` in evidence. A changed snapshot refuses
   keep. Candidate edits requested by review need counted repair, new evaluation
   and a new snapshot before another fresh acceptance verifier.
5. For verified improving code, call `finalize --decision keep --lesson
   "<measured lesson>" --verification <envelope.json>`. For a rejected usable
   result, the coordinator checks requirements, samples, assessment and candidate
   retention, then calls `finalize --decision discard --lesson "<lesson>"`
   without a verification agent. Exhausted attempts without a usable observation
   use `failed-to-run`; exhausted attempts with usable evidence but no answer
   use `inconclusive`. The coordinator checks these terminal results too.
   Interruption leaves the task open; never invent a terminal answer to clear it.
6. Finalize owns Git retention/restoration, terminal evidence, task completion
   and its exact-path commit. Do not repeat ordinary task done, staging or commit
   for a managed hypothesis. Keep advances the accepted baseline. Discard and
   terminal failure restore owned code and retain task results and candidate
   references. Feed their lessons into subsequent proposals and continue while
   authorized limits and safe context permit.

## Stop and recover

On interruption, interrupt owned active workers and confirm turn completion,
record run `stop --run <run>` when possible, and preserve candidate code,
checkpoints and outstanding reservations. End authorization with `end --run
<run>` only after all reservations and experiments are settled. Do not replace
an unfinished run, reset its deadline/costs or close the area.

Report the run identity, scope, native continuation or exact fallback extent,
completed tasks/decisions and commits, accepted baseline, remaining allowance,
stop reason and unfinished recovery actions. Distinguish `target`, `budget`,
`interruption`, shared `blocker`, `cross-area-wait` and unsafe/user-owned
state. Queue exhaustion
alone is a proposal boundary. A finite stop is not evidence that the area's
optimization objective has been achieved.

Use `update_goal` only according to its exposed lifecycle rules. Complete a
native objective only when it has actually been achieved and no required work
remains. Never mark an unfinished objective complete because of a transient
blocker, exhausted queue, exhausted allowance or impending turn end. Pausing
requires the user's explicit request; blocked status requires the tool's
repeated-blocker threshold. Do not invent a model-facing resume operation.

A new session reads durable run context, current task records/results, local
experiment inspect/recover and Git history. Recover pending effects before
generation, then resume the same authorized identity with its current authorized deadline
and consumed resources. Conversation handoffs retain concrete worker choices;
durable records establish work and measurement state. Signed-in live Codex
execution is separate acceptance evidence; deterministic command checks do not
claim that a provider-backed session ran.


### Isolated candidates

The task-format reference defines the common isolated candidate contract for
admission, stopped-worker return, source measurements, current-baseline
reconciliation, serialized integration and destination verification. Use separate
run `--worker-limit` and `--measurement-limit` values. Source workers read the
absolute authoritative record paths in their candidate context and never publish
source records. Read the recovery reference before resuming a paused or ambiguous
candidate. Rolling worker refill and harness role controls use this deterministic
contract; the binary does not schedule workers.

## Rolling research in Codex

Use this section when the user explicitly requests rolling parallel research.
For example: `$zdev run rolling research in work within the brief, with two
workers and one measurement slot, for 20 work attempts`. Pass those capacities
as `start --worker-limit 2 --measurement-limit 1`. This is continuing research
authorization; it does not change ordinary Parallel's finite approved batch.
Follow **Authorize or resume** above first. If native goal creation is
unavailable, the same one foreground work-item bound applies; do not launch a
rolling campaign through that fallback.

The coordinator's operation is
`coordinate_rolling(authoritative_root, run_id, frozen_roles, exposed_native_controls)`.
It inspects workers, recovers durable effects, reads fresh run and explicit task
context, claims a role, creates or reuses an owned source, dispatches, collects
stopped work, measures, and processes acceptance. After each event it repeats
those checks and fills available capacity while siblings remain active. It
returns the existing run report described above. Run reservations and experiment
checkpoints retain its progress; do not create a second scheduler or ledger.

### Claim roles and admit sources

1. Inspect native goal and worker state with the exposed `get_goal` and
   `collaboration.list_agents` tools. Use `collaboration.spawn_agent`,
   `send_message`, `followup_task`, `interrupt_agent` and `wait_agent` only for
   their exposed operations. A message or wait does not create capacity. Count
   every live planner, implementer, acceptance verifier and delegated finding
   checker, including competing workers. A retained worker also counts when the
   runtime says it occupies capacity. Use the tighter available native capacity
   and the run's worker limit. If capacity falls, stop affected dispatch and
   preserve unresolved claims until actual completion or stop is confirmed.
2. Read fresh `research run <area> context` and explicit
   `work-context <area> --task <task> --store` before each dispatch. Check the
   current accepted revision, comparison conditions, stop reason and allowance.
   Pick stored ready in-scope work before generating another small mixed bundle.
   Publish and commit new records only when the acceptance lane is free.
   Source preparation may continue during acceptance; destination mutations,
   supporting completion and other candidates' measurements wait for that lane.
3. Before every native role launch, claim a new counted request:

   ```text
   zdev research run <area> reserve --run <run> --request <request> --task <task> --kind preparation --worker-slot --format json
   ```

   Use `supporting` for each ordinary support role, `repair` for source rework,
   and `confirmation` for acceptance verification or its delegated finding
   check. Require first-call `dispatch_authorized: true` for a standalone role.
   Identical retries return stored costs and `false`; changed task, kind or
   requested slot mode refuse. Never interpret an old reservation as permission
   to launch again. Settle standalone roles only after confirming actual stop.
4. Advanced hypotheses first use a counted preparation planner. Validate its
   strict semantic plan through the existing task-workflows rules and settle
   that planner. Retain the exact plan, baseline, and concrete role settings for
   implementation and later rework. Do not plan again on resume. If required
   plan or frozen selection evidence is missing, preserve that candidate and
   report the missing evidence before dispatch.
5. Create a source from the authoritative HEAD current at this refill. Require
   its code to match the current accepted baseline. Use exposed
   `mcp__codex_app__create_worktree` and its returned workspace path when
   available; inspect attachments first and reuse only an exclusively owned
   suitable source. Pass the observed HEAD as `ref`, inspect asynchronous setup
   through the exposed status tool, and give the managed checkout its own source
   branch before admission. When these controls are absent, use an authorized
   local linked Git worktree. Keep source and authoritative roots absolute.
   Never reuse a stale sibling's source or silently change the user's checkout.
6. Claim implementation capacity, then transfer that claim into admission:

   ```text
   zdev research experiment <area> <task> admit --source <absolute-source> --path <exact-file> --reservation <request> --format json
   ```

   Wait for admission's first-call `dispatch_authorized: true` before launching
   this source worker; reserving alone does not authorize source edits. Transfer
   uses the existing claim without counting it twice, including at worker limit
   one. Admission captures the current baseline and authoritative record paths.
   Supply those paths, absolute source cwd, exact owned files, snapshot and
   unchanged advanced plan to the worker. It returns the existing strict
   implementer result and never edits `.zdev`, stages or commits.

Do not wait for all siblings before refill. A confirmed finished source role
releases its claim through `return --worker-stopped`. A planner or destination
verifier releases its standalone claim through run `settle`. Give a waiting
acceptance verifier the next free role slot before starting more preparation.
Keep each request with its actual worker and source. Disappearing worker IDs,
ended conversations and failed waits do not prove stop or release capacity.

### Measure and accept one candidate at a time

Return inspects the complete source delta and retains its Git transport. Start
`evaluate` only when the run reports measurement capacity and no acceptance
owner. Use a supported nonblocking command operation when measurement must
continue while other sources prepare. For example, `exec_command` may return a
process session; retain that session and poll with `write_stdin`. Read durable
checkpoint ownership before other dispatch. A failed command transport does
not prove that its evaluator exited. Never start a second evaluator from an
old permission. Preparation roles cannot substitute for measurement claims.
With measurement limit one, source edits can overlap one evaluator, while a
second Evaluate refuses without launching or adding cost.

Process improving returned candidates through one destination lane. If accepted
code changed since dispatch or measurement, use counted `reconcile` and a fresh
Evaluate before integration. If the baseline changes again, repeat those gates.
Record-only drift follows the explicit refresh rules in the task-format
reference. On resume, the experiment projection's `measurement_current` confirms
that a completed measurement still matches the owned code and comparison policy;
transport commit IDs need not match measurement commit IDs. `integrate` rechecks current measurements, claims the lane, applies
only owned code and supplies a fresh destination `verification_snapshot`.

Reserve a `confirmation --worker-slot` request for this acceptance owner, then
launch a fresh independent verifier in the authoritative destination. Supply
its snapshot, original baseline, declared paths, samples, comparison policy and
coding guidance. Validate its four semantic fields, compare the exact snapshot
and construct the ordinary public verifier envelope. A delegated substantive
finding check also needs its own counted confirmation slot and equal snapshot
comparisons. Settle each role after confirmed completion. Other tasks cannot
use this owner exception, and it never opens measurement capacity.

For checked verifier rework, stop the source worker and reserve a new
`repair --worker-slot` request for the same owner. Call:

```text
zdev research experiment <area> <task> prepare --reservation <repair-request> --format json
```

Prepare requires unchanged destination HEAD and exact measured bytes and index
entries on every owned path. It binds the counted repair claim and records
withdrawal before restoring those paths to accepted HEAD. It preserves unrelated
staged, unstaged and untracked work, the measured source, retained candidates and
historical observations. It invalidates the verifier snapshot and releases the
acceptance lane after restoration. Only the first completed Prepare grants
source dispatch. `recover` finishes an interrupted authorized withdrawal without
new permission. Confirm any unresolved worker stopped before requesting another
role; never turn a recovery response into a worker launch.

After repair, confirm Return, run a new counted Evaluate, integrate and obtain a
fresh independent destination verifier. Old measurements and verification cannot
accept edited code. Changed owned files, integration conflicts, exhausted budget,
reached target, interruption or a shared blocker refuse new repair. Recovery may
finish already authorized restoration even after a run stops.

Ordinary supporting tasks use the existing explicit-task complexity, planning,
independent verification, checked rework, completion and commit route. Reserve
and settle a worker slot for every native role. Serialize their destination
work with acceptance. Before shared evaluator or workload changes, pause
affected candidates and confirm active measurements stopped. Accepted support
or changed comparison conditions require an actual counted rebaseline on clean
accepted code. Only then reconcile and remeasure paused candidates; supplied
samples cannot clear this gate or loosen the frozen requirements.

### Stop, resume and clean up

On target, budget, interruption, lost native capacity or shared failure, stop
new affected dispatch. Interrupt owned active workers when supported, confirm
termination and preserve their source trees, deltas, requests and process
sessions. Keep ambiguous slots occupied. Shared failure permits only relevant
counted supporting repair while comparisons remain halted. Report the actual
stop reason and unfinished effects.

Resume the same authorized run after fresh native and durable inspection.
Recover authorized withdrawal, integration, publication and measurement effects
before refill. Retain deadlines, consumed costs, concrete settings and required
advanced plans. A live or unknown measurement stays owned and is never
relaunched. Recheck current accepted code and conditions at the next dispatch.

Clean up only with retained cleanup authority, confirmed worker and process
stop, durable completion, retained candidate history and an unambiguous run-owned
source. Use the exposed archive operation for a managed worktree, or remove an
owned local linked worktree. Keep pre-existing, unfinished and ambiguous sources.
A finished run leaves the area open and does not claim its optimization objective
achieved. Claude's installed zdev-research route accepts execution rolling with
separate worker_limit and measurement_limit start arguments. Its research
reference owns Claude's Promise arrival, native call-cap and fresh-invocation
recovery rules. Live native acceptance is separate from deterministic checks.

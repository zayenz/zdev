# Run serial research in Codex

## When

Use this route only when the user explicitly authorizes a research run in a
named area, or asks to resume its existing run. For example: `$zdev research
run work within the brief, for 20 work attempts` or `$zdev resume research run
work`. Explicit `until interrupted` removes the default attempt limit; it
does not remove supplied targets, time or command limits. Ordinary Implement,
Plan, Verify, Parallel and Loop keep their hypothesis guards. This route uses
the current branch and one coordinator; it does not create worktrees.

Read the installed task-format reference's research sections for the exact
policy, TaskBundle, run and local experiment contracts.
The exact installed task-workflows contract path is
"/Users/zayenz/.codex-workspaces/worktrees/83c5f3b8-0c89-4e75-b8f6-7831269edd01/zdev/skills/zdev/references/task-workflows.md". Decode that string. Its sibling
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
   when authorized. Identity, scope, target, original deadline and costs survive.
4. Resolve and freeze all five roles with `zdev config profile resolve codex
   <role> --area <area> ... --format json` under the shared scoped-profile
   contract. Retain the returned concrete planner, routine-implementer,
   implementer, advanced-implementer and verifier settings in the handoff.
   Restore them on continuation. When a new session has no concrete handoff,
   explain that limitation and resolve fresh settings before the next dispatch.
   Do not call ordinary `dispatch-spec` for a hypothesis or remove its guard.
5. With clear native state, use exposed `create_goal` only for this explicitly
   requested run. Use an objective that names the area, authorized scope and
   retained limits, requires this serial contract and durable recovery, and
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

When comparison conditions or accepted supporting code change, retain or
restore unfinished candidates, then run actual rebaseline on clean accepted
HEAD. Supplied baseline samples cannot clear invalidation. A supporting task
cannot change the metric, direction or correctness rules to make a candidate win.
An unsafe checkout or a user-owned scope/correctness decision stops with the
current work preserved.

### Generate work

Use the brief, current accepted result and prior lessons to propose a small
mixed TaskBundle of ordinary supporting tasks and hypotheses. Describe a
testable mechanism, useful completion evidence and allowed code changes.
Keep dependencies and context links distinct; admit a `requires_code` link
only after the referenced candidate has completed verified acceptance. Empty
or exhausted queues allow another scoped proposal while allowance remains.

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
`interruption`, shared `blocker` and unsafe/user-owned state. Queue exhaustion
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
generation, then resume the same authorized identity with its original deadline
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

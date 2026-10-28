# Run serial or rolling research in Claude Code

Use this route only for an explicit research-run authorization or continuation
in a named area. Ordinary Implement, Plan, Verify, Parallel and Loop keep their
hypothesis guards. Serial execution uses the current branch. Rolling execution
uses run-owned linked sources and one destination coordinator.
Read the research sections of the installed task-format reference completely.
The exact installed task-workflows contract path is
"/Users/zayenz/projects/zdev/.claude/skills/zdev/contracts/task-workflows.md". Decode that string.
The exact installed coding reference path is
"/Users/zayenz/projects/zdev/.claude/skills/zdev/skills/zdev/references/coding.md". Decode that string.
The exact installed research reference path is
"/Users/zayenz/projects/zdev/.claude/skills/zdev/skills/zdev/references/research.md". Decode that string.
Pass these paths, absolute repository cwd, brief, complete task, repository
guidance and stored context to every worker.

The coordinator owns selection, proposals, admission, records, acceptance,
ordinary completion and commits. The binary owns run authorization, costs,
measurements, candidate retention/restoration and managed hypothesis finalize.
Workers never edit `.zdev`, stage, complete tasks, finalize or commit.

## Start or resume the workflow

Use the installed `zdev-research` workflow when packaged execution is available.
Give it structured arguments, for example:

```json
{
  "area": "work",
  "authorization": "start",
  "invocation_id": "work-session-1",
  "scope": "Improve parsing within the brief",
  "execution": "rolling",
  "worker_limit": 2,
  "measurement_limit": 1,
  "attempt_limit": 20,
  "target": 5,
  "cleanup": false
}
```

Omit execution for the existing serial route, or set execution rolling explicitly.
Worker and measurement limits are positive integers used only at start; both
default to one. Rolling authorization includes creating linked source worktrees.
Cleanup is optional and requires explicit authority, confirmed stop, durable
completion and retained history.

The coordinator supplies a distinct invocation_id for each fresh invocation.
Use letters, numbers, underscores and hyphens. The first coordination prompt
includes that identity and reads live state before dispatch. Omit attempt_limit
to use the binary's default of 20 work attempts. With time_limit_seconds and no
attempt_limit, the run has no dispatch cap. Set until_interrupted true only
when explicitly requested; preserve supplied target, command_limit,
command_seconds and time_limit_seconds. Unlimited authorization still obeys
runtime and resource limits.

When you choose a limit value yourself rather than taking it from the user,
supply it and list its key in the start-only `estimated` array, for example
`"command_seconds": 600, "estimated": ["command_seconds"]`. Never list a value
the user stated. The returned report includes `limits`, where each limit has a
`value` and an `origin`: `user`, `default`, `coordinator_estimate`, or
`unrecorded` for runs started before origins were recorded. Its `consumed`
reports `work_attempts` for every dispatch, `experiments` for measured
experiments and `dispatches_by_kind` for the supporting work. When
`work_attempts_advisory` is true, the dispatch count never stops the run. Tell
the user each limit's origin and both counts.

Resume with authorization resume, the same run_id, a fresh invocation_id and
the execution, cleanup and frozen_profiles returned by the prior invocation.
Also pass retained_candidates to preserve unfinished source implementation or
checked acceptance repair requirements. An interrupted repair cannot use its old
measurements or verification to accept code. Do not supply replacement
scope, limits or target in the resume input. If the user explicitly grants more
allowance, extend the durable run through the task-format contract before
resuming. You may extend your own `coordinator_estimate` limits the same way
without asking. Raising a `user`, `default` or `unrecorded` limit needs the
user's grant. A time-only run has no attempt limit, so `extend --add-attempts`
fails on it. The identity, current deadline and costs survive. Pass retained_plans as the task-ID map of semantic plans returned by the
prior invocation. Restore those advanced plans when continuing the same task;
never plan again during same-task rework. If a conversation lacks concrete role
choices, explain that and resolve all five Claude roles afresh through `zdev
config profile resolve claude <role> --area <area>` before dispatch. Freeze the
concrete planner, routine-implementer, implementer, advanced-implementer and
verifier records. Pass their model/effort through native agent options on every
applicable call. Hypotheses never use ordinary dispatch-spec.

Claude supports pause and continuation of a live workflow. Its documented
recovery can replay cached completed calls before restarting an interrupted
worker. A cached reservation is not permission for another worker dispatch.
After stop or failure, start a **new zdev-research resume invocation**, recover
live durable effects, and reserve every replacement launch afresh. Do not restart
an interrupted worker through the workflow UI or replay a stopped script as an
accounting-safe continuation. The workflow uses args and agent mediation; it
has no direct shell/filesystem/module/clock API and does not depend on `/goal`.

When packaged execution is unavailable, the installed coordinator performs at
most one foreground work-item iteration, including counted rework and completion.
A generation-only or rebaseline-only iteration also consumes that bound. State
the exact extent, runtime limitation and required continuation. Apply every
accounting, recovery and verification rule below; do not promise background or
unlimited execution because the run has until-interrupted authorization.

## Coordinate rolling work

Set execution rolling for continuous refill. After each arrival, collect the
confirmed stopped source through Return, inspect fresh durable capacity and fill
an available worker slot while siblings are still preparing. Exclude assigned
unfinished tasks when selecting from the complete frontier. Generate and publish
small mixed bundles within scope when no unassigned work is ready. Create each
linked source branch from destination HEAD current at that dispatch, checking
its code against the accepted baseline. Pass the candidate's absolute source cwd
and authoritative brief/task/instruction paths to the worker.

Every named native role, including advanced planners, ordinary support,
acceptance verifiers and independent finding checkers, first requires a new
counted reserve --worker-slot claim with dispatch_authorized true. Exact replay
never grants permission. Settle standalone roles only after actual stop. Transfer
implementation claims into Admit and repair claims into Prepare; require those
operations' first-call dispatch_authorized true before source edits. Return with
confirmed worker stop retains transport and settles the bound claim, including
at worker limit one. Retain the original advanced semantic plan through repair.

Evaluate runs through asynchronous command mediation. Track pending calls as
well as durable measurement occupancy: an agent waiting to execute Evaluate
already consumes the workflow's available measurement capacity. Source
preparation may overlap an actual evaluator. Do not integrate while another
measurement is pending. Serialize publication, ordinary support, integration
and acceptance in the authoritative destination. Give a waiting acceptance
verifier the next free role slot before another preparation.

Before comparison or acceptance, refresh a stopped source against current HEAD.
Code or condition drift requires counted Reconcile and a new actual Evaluate;
record-only drift can retain unchanged code measurements through the public
refresh rules. Integrate supplies a fresh destination verification snapshot.
Reserve a confirmation role, independently verify the whole acceptance claim,
compare that snapshot and construct the public verifier envelope. Independently
check substantive rework findings with another counted confirmation role.
Checked repair uses a fresh repair claim and Prepare to withdraw integrated
owned paths before source edits, preserving unrelated destination changes,
retained history and observations. Return, new measurement, integration and a
fresh verifier follow every repair. Recovery never turns a replayed Prepare
into permission for another source worker.

Shared failures halt affected dispatch. Drain known source/measurement calls,
confirm actual stops, then use relevant counted ordinary support and an actual
accepted-code rebaseline before paused comparisons resume. Preserve the frozen
metric, direction, correctness and acceptance requirements. Unknown workers or
processes keep their claims and sources occupied.

Change correctness command text only for a moved or renamed file that keeps the
check's meaning. After unfinished experiments, measurements and rebaselines
finish, the coordinator declares it with
`zdev area research <area> --from <file> --administrative "<reason>"`; the run
keeps its costs and comparison validity. A changed horizon, workload, solver or
acceptance rule inside a command is material and needs the user's decision on a
new study with a newly measured baseline. Stop for the user when it is unclear
whether a command change keeps the check's meaning.

The workflow counts every agent call, including command mediation and composed
ordinary helpers, toward Claude's documented 1,000-agent workflow cap. It stops
refill at 900 calls with room to drain and report; a hard guard before 980 calls
preserves unresolved effects. Continue through a fresh invocation of the same
run using the returned execution, cleanup, concrete profiles, plans and repair
handoff. Native pause/replay is not fresh dispatch authority. See
[Claude workflow limits](https://code.claude.com/docs/en/workflows#behavior-and-limits).

## Coordinate serial work

Read the brief first, research settings, complete work-context and run context.
Require an open area, safe branch and attributable staged/unstaged/untracked
state. Preserve unrelated or ambiguous changes. Report a stale-but-safe advisory
once and continue. Start only with explicit scope and no unfinished run; resume
the existing authorization instead of replacing its exhausted allowance.

Before selection, inspect admissions, outstanding reservations, attributable
uncommitted records, experiment checkpoints and interrupted rebaseline requests.
Retry only identical admission content under its original request. Finish safe
publication and exact commit checkpoints through zdev commit. Recover recorded
measurement/finalization through experiment inspect/recover; never relaunch an
unknown or running measurement. Reconcile actual completed worker effects with
settle, without refunds or redispatch. Recover an ordinary verified completion
through its existing coordinator and retained verification evidence. Missing
evidence requires a newly counted fresh verification; preserve the checkpoint
until that evidence is available. Retain unfinished candidate ownership until
safe lifecycle restoration permits accepted supporting edits.

Read fresh run context and all rejected, inconclusive and failed-to-run lessons
at each boundary. Check stop_reason and comparison validity before new costly
work. When context reports `unreachable_minimum_gain`, prepare no further
candidates and report the study decision to the user: keep the frozen study, end
it, or declare a new study with its own threshold and measured baseline. Never
lower the frozen threshold; changing only the advisory `metric_bound` never
invalidates comparisons. Choose ready tasks using ordinary dependencies and
eligibility; under a shared blocker choose only relevant ordinary supporting
repairs. Keep explicit task selection through every refresh. When no task is
ready and fresh run context lists `cross_area_waits`, every open task waits on
another area: record `stop --run <run>`, report `stopped` with stop reason
`cross-area-wait` and those waits, and generate no work. Empty/exhausted queues
permit a small nonempty scoped mixed TaskBundle, not successful completion.
Check scope, graph links and requires_code; only verified acceptance establishes
retained code. Admit with a unique request, inspect allocated task IDs/files,
stage only those task records and TASKS.md, inspect the cached diff, and use
zdev commit. Refresh after publication. Invalid or nonprogressing proposals stop
with their concrete failure rather than generating forever.

Supporting tasks use the composed ordinary one-task workflow: advanced planning,
implementation, strict worker results, fresh independent verification, rework,
snapshot comparison and exact completion. Completion, including a recovered
ordinary completion, also stages each dependent `TASKS.md` path that `zdev
task done` reports in `dependent_indexes`. Reserve kind supporting before every
planner, implementer, replacement/rework and verifier call; settle completed
dispatches afterward. Command mediation and record helpers remain inexpensive
coordination. Confirm task kind before the frozen-profile path too. After
accepted supporting code or comparison-condition changes, run actual counted
rebaseline on clean accepted HEAD before comparing another candidate. With
declared `relevant_inputs`, only changes inside them count as accepted
supporting code changes. Untracked files under declared `generated_outputs` do
not make accepted HEAD unclean. Preserve metric, direction, correctness and acceptance
policy.

For hypotheses, explicitly admit the ready task and read full task/policy.
Declare exact clean owned files before edits, including named new files;
exclude `.zdev`, directories, symlinks and unrelated changes. Reserve preparation
before an advanced planner, validate its strict semantic plan and reconstruct
its public envelope, then settle planning. Keep that exact semantic plan for
same-task rework. Begin establishes the checkpoint and charges implementation
preparation before launch. Refresh admission, dispatch the selected implementer
with the declared file boundary, validate its strict envelope and inspect the
whole delta. A needed extra file is reported without edits; preserve the
checkpoint and resolve through its lifecycle before a revised hypothesis.

Evaluate uses the local lifecycle and binary-owned correctness/evaluator
accounting. Each repair, rerun, confirmation or reconciliation first gets a new
reservation; bind it to evaluate when applicable. Inspect actual attempts,
samples, checks, retained references, assessment and comparison validity.
Only proposed keep launches a fresh independent verifier, after confirmation
reservation. Supply verification_snapshot, starting baseline, owned files,
recorded samples and full acceptance policy. Validate the strict four-field
semantic result, compare the stable work-context snapshot, independently check
substantive findings and construct the public verifier envelope containing
`work_context_snapshot: <snapshot>`. Changed snapshots, invalid measurements
and unverified claims refuse keep. Count repairs, re-evaluate and obtain a new
snapshot before another fresh verifier.

The coordinator checks discard and terminal failed results without a default
discard-review worker. Finalize alone retains/restores code, records the lesson,
completes the hypothesis and commits exact paths; do not repeat ordinary task
done/staging/commit. Exhausted attempts without usable observations permit
failed-to-run; usable observations without an answer permit inconclusive.
Interruption remains unfinished. Keep advances the baseline; other terminal
decisions restore owned code and retain results and candidate references. Feed
all measured lessons into subsequent proposals.

Target, budget and interruption stop new costly dispatch but permit recording,
restoration and already authorized finalization. A reservation using the last
work unit authorizes only its existing dispatch, even when its returned context
now reports budget. It never permits another reservation or refund. Classify
shared failures from evidence and record block. Halt affected hypothesis work;
count relevant supporting repair within allowance, recover/restore candidates,
then resolve and run an actual counted rebaseline. If safe recovery cannot yet
restore a candidate, preserve it and report the required lifecycle action.

Malformed, multiple or duplicate JSON objects, null results, thrown calls,
unsafe state and incomplete commits preserve effects and costs. Inspect before
retrying. Record stop when possible without replacing an existing shared blocker.
No native cancellation API is assumed. Report run identity/scope, fresh
remaining allowance, precise stop reason and unfinished recovery actions. Then
report research progress and engineering progress under separate headings.
Research progress lists each question answered with its hypothesis decision,
each valid comparison under the frozen policy, each negative finding
(`rejected`, `inconclusive` or `failed_to_run`) with its lesson, the current
accepted result from the run's accepted baseline, and the next useful
experiment. Read decisions, lessons and observations from `zdev task research
<area> <task> --format json` and the accepted baseline from run `inspect`.
Engineering progress lists completed ordinary tasks, commits, verification,
`consumed.experiments` and `consumed.dispatches_by_kind`. Only an `accepted`
decision from a comparison accepted under the frozen policy is an accepted
result; finalize records `keep` as `accepted` and `discard` as `rejected`.
Never present a supporting task, a rejected candidate or an isolated screen win
as a measured improvement, and report an invalid measurement as invalid. A
large task count is not research progress.
Queue exhaustion and ending authorization neither close the area nor prove its
optimization objective achieved. Authenticated Claude execution is separate
live acceptance evidence.

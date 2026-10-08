# Run serial research in Claude Code

Use this route only for an explicit research-run authorization or continuation
in a named area. Ordinary Implement, Plan, Verify, Parallel and Loop keep their
hypothesis guards. This route uses the current branch and one coordinator.
Read the research sections of the installed task-format reference completely.
The exact installed task-workflows contract path is
{{ task_workflows_contract_path_json }}. Decode that string.
The exact installed coding reference path is
{{ claude_coding_reference_path_json }}. Decode that string.
The exact installed research reference path is
{{ claude_research_reference_path_json }}. Decode that string.
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
  "attempt_limit": 20,
  "target": 5
}
```

The coordinator supplies a distinct invocation_id for each fresh invocation.
Use letters, numbers, underscores and hyphens. The first coordination prompt
includes that identity and reads live state before dispatch. Omit attempt_limit
to use the binary's default of 20 work attempts. Set until_interrupted true only
when explicitly requested; preserve supplied target, command_limit,
command_seconds and time_limit_seconds. Unlimited authorization still obeys
runtime and resource limits.

Resume with authorization resume, the same run_id, a fresh invocation_id and
the frozen_profiles returned by the prior invocation. Do not supply replacement
scope, limits or target on resume. The original identity, deadline and costs
survive. Pass retained_plans as the task-ID map of semantic plans returned by the prior
invocation. Restore those advanced plans when continuing the same task;
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
work. Choose ready tasks using ordinary dependencies and eligibility; under a
shared blocker choose only relevant ordinary supporting repairs. Keep explicit
task selection through every refresh. Empty/exhausted queues permit a small
nonempty scoped mixed TaskBundle, not successful completion. Check scope,
graph links and requires_code; only verified acceptance establishes retained
code. Admit with a unique request, inspect allocated task IDs/files, stage only
those task records and TASKS.md, inspect the cached diff, and use zdev commit.
Refresh after publication. Invalid or nonprogressing proposals stop with their
concrete failure rather than generating forever.

Supporting tasks use the composed ordinary one-task workflow: advanced planning,
implementation, strict worker results, fresh independent verification, rework,
snapshot comparison and exact completion. Reserve kind supporting before every
planner, implementer, replacement/rework and verifier call; settle completed
dispatches afterward. Command mediation and record helpers remain inexpensive
coordination. Confirm task kind before the frozen-profile path too. After
accepted supporting code or comparison-condition changes, run actual counted
rebaseline on clean accepted HEAD before comparing another candidate. Preserve
metric, direction, correctness and acceptance policy.

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
No native cancellation API is assumed. Report run identity/scope, completed
ordinary tasks and hypothesis decisions/commits, accepted baseline, fresh
remaining allowance, precise stop reason and unfinished recovery actions.
Queue exhaustion and ending authorization neither close the area nor prove its
optimization objective achieved. Authenticated Claude execution is separate
live acceptance evidence.

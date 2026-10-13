# Coordinate zdev through T3 Code

## Admit the route

Use this route only after the root skill selected T3 orchestration. The main
thread must run in T3 Code with Codex or Claude Code. The coordinating agent
uses its own file and shell tools for zdev commands and Git, and T3's MCP for
worker execution. Goal/loop uses one matching parent goal displayed by T3;
its native provider continues this same coordinator. Run no separate native
zdev loop, Claude JavaScript workflow, or native worker dispatch alongside it.

Resolve the selected ordinary zdev intent. Load `task-workflows.md` for
implementation, verification, and planning, plus `implement.md`, `verify.md`,
or `verify-workflow.md` for the selected operation. Load `audit.md` for audits.
For goal/loop, use **Continue an area** below rather than the native
`area-loop.md`. For plan-only work, use **Plan only** below rather than the
native dispatch-spec route. Other intents keep their shared instructions;
any requested reviewer uses the T3 dispatch and profile rules here.

The installed task-workflows contract locator is
"/Users/zayenz/projects/zdev/.claude/skills/zdev/contracts/task-workflows.md". Resolve a locator beginning with
`references/` against the directory containing the loaded root zdev SKILL.md;
resolve other relative locators against the authoritative checkout. Give workers its absolute path and this reference's
absolute path; they load only their assigned worker role and applicable
contract sections.

Make one bounded `orchestrator_capabilities` call through T3's available MCP
tools, including the live tool-discovery mechanism when tools are attached
lazily. Tool prefixes vary by harness; use the exposed names. Require callable
`delegate_task`, `task_status`, and `task_cancel` and an available inherited
provider. If discovery or the call fails, report the concrete blocker before
dispatch. Do not start a substitute native workflow or install another runtime.

All delegated workers share the calling thread's T3 workspace binding. Before
dispatch, compare the main thread's actual cwd and branch with the admitted
authoritative checkout. Include that absolute root and require each worker to
confirm its cwd and branch before work. This route supports sequential workers
in that checkout. An assigned source worktree or parallel implementation request
is unsupported: report that before worker or source mutation, with ordinary
sequential work as an option. A `cd` instruction does not create T3 isolation.

## Resolve workers by their harness

Retain the user's role-to-provider choices in the conversation. Accept a chosen
harness for a role, or explicit provider instance and model choices. When a
role's harness is omitted, use the inherited main-thread provider instance.
An implementation target applies to routine, standard, and advanced
implementers unless the user chooses a target for a specific tier.
Map T3 driver `codex` to zdev harness `codex`, and `claudeAgent` to `claude`.
These are the qualified routing targets for this integration. Another driver
is unsupported until separately qualified; do not reinterpret it as Codex or
Claude. If a requested harness has several available instances, ask which one
to use rather than selecting an account silently.

Resolve all roles the run can use before their first dispatch. For task work,
freeze routine, standard, advanced, planner, and verifier selections for the
whole run; authored complexity still decides which roles actually run. A
planner without its own target follows the advanced implementer's target.
Audits and requested review steps need only the verifier selection.

For each role, run:

```text
zdev config profile resolve <worker-harness> <role> --area <area> --format json
```

Omit `--area` for work outside an area. Pass the user's run profile with
`--run-profile`. Use the worker harness, not the coordinator harness. Freeze
these baseline profiles, models, efforts, selected instances, and translated T3
options throughout the run, including later tasks, repairs, and escalation.
A one-off role profile adds `--profile` for that role's selected logical step.
Resolve and retain that override for its retries and replacement workers only;
then expire it and use the frozen baseline for subsequent steps and tasks.
Do not use a single-harness
`dispatch-spec` for mixed workers or change stored preferences to route them.

Match each resolved or explicitly requested model against that instance's live
advertised model IDs. Translate a harness alias only when the live catalog
establishes one unambiguous match. Map effort through the selected model's
advertised option descriptors, supplying the actual option ID and value. A
requested unsupported or ambiguous model or effort is a blocker before
dispatch; report the available choice without silently substituting it.
An explicit concrete model/effort choice overrides the corresponding resolved
field for that role and logical step only.

Whole-profile `inherit` may omit model and options only for the inherited
instance, retaining the main thread's current selection. On another instance,
require an explicit advertised model instead of accepting T3's first-model
default. For inherited effort, omit the effort control and retain T3's documented
native option behavior. Report the target instance, concrete model when set,
profile, and supplied effort at dispatch. Check the returned instance and model
against the requested selection before accepting a worker result.

## Dispatch and collect one role

Supply a self-contained compact task prompt: assigned role, absolute checkout,
area and task identity or audit boundary, repository instruction paths, brief,
slice and task paths, baseline snapshot and current snapshot when different,
this reference's assigned **Worker roles** section, and relevant shared contract
locators. Include the accepted semantic plan or exact repair findings when
needed. A child receives no parent conversation history. It acts only as that
worker and returns its required envelope as the final answer; it starts no
coordinator, native goal, or loop and delegates no nested work.

Use `delegate_task` for every worker, even when its harness matches the main
thread. T3 role tags are `design` for planners, `implementation` for implementers,
and `review` for verifiers; the actual zdev role instructions come from the
prompt and referenced contract. Set `target.providerInstanceId`, the concrete
`target.model` when selected, and `target.options` using the frozen advertised
values. Retain the parent's runtime permission ceiling; a review role or plan
interaction mode is not proof of filesystem isolation.

Give each dispatch a `clientRequestId` unique to this run, task, role, and
attempt. Reuse it only when retrying that exact dispatch with unchanged input.
Retain the returned `taskId`, child thread, identity, phase, and attempt in the
conversation. Prefer `mode: "async"` for long work. Before yielding, record
what is pending and what that result must trigger. Yield only after an accepted
dispatch is known to be outstanding, or after a terminal route result.

When T3 delivers a completion wake, read `task_status` for the retained task ID.
Ignore duplicate or unrelated wakes. A finished provider turn is insufficient:
require `status: "completed"`, `workState: "result_available"`, and no pending
child runs. Extract and validate the role result from the final `summary` using
the shared result rules. A transport completion is not a zdev `pass`.
Failed, cancelled, interrupted, missing, or malformed results stop with their
reason and preserved state; do not mark the task done.

A short dependent step may use `mode: "wait"` with a bounded timeout. A wait
timeout leaves the child alive. Retain its handle and yield for its completion,
or inspect `task_status` when the result is needed during the current turn.
Never launch a replacement merely because a wait or tool response was lost.
After an uncertain mutation, recover the existing child through the calling
thread's child list/read facilities before retrying the same dispatch key.

Run local coordination through the shared contract: refresh task admission,
attribute changes, store a fresh verifier snapshot, validate semantic results,
compare snapshots, and vet substantive findings. Every verifier is a new T3
delegation. Rework uses a new implementation delegation with the retained
baseline, current context, accepted plan, and complete findings. Each attempt
has a new key. Use the permitted one-time standard-to-advanced escalation
without replanning. A completed child's backing thread is not a continuation
target; do not use `t3_thread_send` to reopen it.

Only the coordinator performs task completion, explicit staging, staged-diff
inspection, and `zdev commit`. Require the unchanged accepted verifier snapshot
before completion, and inspect the resulting commit before recording success.
One Implement interaction ends with the shared `zdev-implement` envelope and
does not query the next task. Verify and audit retain their existing envelopes
and never complete or commit work.

## Plan only

For “plan next task with advanced planner,” store and show fresh explicit-task
context and require open, ready, safe work. Resolve only the planner, using
`--profile advanced` unless the user chose a more specific planner profile.
The run's provider target rules above still apply. Delegate one fresh read-only
planner, validate its four semantic fields, and construct the existing public
planner envelope. Return the plan, baseline, area/task, and concrete selection;
stop before implementation. Retain an accepted plan for the normal unchanged
snapshot check on a later authorized implementation request.

## Continue an area

“Goal” and “loop” are aliases. Continue one named area under this contract;
freeze worker settings once for the run and retain the optional focus text.
With no focus, let fresh `work-context --store` select the next task. With
focus, read the complete ready frontier and every ready task, rank the AFK
eligible set by the user's guidance, and admit the selected ID with
`work-context --task <task-id> --store`. Re-select after every committed
iteration. Focus is guidance, not a filter or stored preference.

After establishing the visible coordinator goal below, start the ordinary
one-task implementation sequence. Advanced tasks first receive a read-only plan.
Implementation completion dispatches a fresh verifier;
accepted rework follows **Bounded corrections** in [review.md](review.md), with
a repair and fresh focused recheck. Each accepted async
dispatch lets the coordinator yield, and T3's completion wake resumes the next
phase. Preserve area, focus, frozen selections, task ID, baseline, pending
handles, phase, review stage and findings, accepted plan, escalation, and
completed task/commit pairs in
the coordinator conversation across wakes and native goal continuations.

### Use the visible coordinator goal

Use T3's `/goal` surface for the parent coordinator only. T3 mirrors Codex or
Claude's native goal state; it supplies no provider-independent goal API.
Keep the T3 worker route selected throughout that goal. Workers receive their
task-sized contracts and never start goals or loops themselves.

Establish a matching active goal before the first loop worker. Inspect it with
exposed native goal tools when available, or the current T3 goal projection or
native activation receipt in the transcript. An unrelated unfinished goal is a
blocker: do not replace, clear, or layer the loop over it. Reuse the same goal
only when its area, focus, worker choices, and stop condition match this run.
A user-entered `/goal` may already have established it before this skill runs.
Account for subsequent Stop, pause, clear, completion, and limit receipts; an
old activation alone does not establish the current goal state.
An active condition missing the required stop rule is incompatible; report it
before work rather than editing or replacing that goal.

When clear, exposed native inspection and creation tools may establish the
goal; set no token budget unless the user requested one. If activation is
unavailable to the agent, return the exact ready-to-send
`/goal` condition for the user to enter in T3, before dispatch or mutation.
Do not simulate a composer command with shell execution or thread messaging,
claim that a prose request activated a native goal, or fall back silently to
the old event-only loop. Return a canonical BLOCKER with failed stage goal
activation, the prepared command, and unchanged state. One-task implementation,
verification, and audit do not require this goal.

The condition must select zdev **with T3 orchestration**, retain the named area,
optional focus and worker choices, and require one independently verified task
and commit per iteration. Its run is finished only when fresh validated context
is closed, open/empty, or open/exhausted, or a confirmed blocker or required
user decision has been reported with its reason and preserved state. All owned
workers must be terminal before that final report. Include this stop rule in
the condition itself, so a blocker ends continuation instead of prompting
repeated attempts. Goal completion describes the run ending; the public zdev
result still distinguishes PASS from BLOCKER and never closes an exhausted area.

On every native continuation or child-completion wake, reconcile retained
handles and phase before any new dispatch. If a worker is still active, retain
it and await its completion; a goal check is not a retry or replacement.
Ignore duplicate results and perform each phase once. Surface selected and
completed tasks, pending role, task/commit pairs, and final context in the
coordinator transcript so native evaluation sees the actual stop evidence.
An active owned child means the run is not finished. Fresh safe ready work
continues the run only when no terminal blocker or required user decision exists.

### Continue task phases

After one exact implementation PASS with its commit, collect fresh work-context
in that same coordinator turn. If open work remains ready and safe, dispatch
the next task's first worker before yielding. A successful derived-work commit
is also an iteration boundary; refresh the ordinary graph without counting a
split source as completed. The matching native goal may also continue the
coordinator across task boundaries; both entry paths use the same phase and
handle checks. Never add a scheduled task or second goal. If that goal becomes
unavailable after a committed iteration with fresh safe ready work, return
honest `CONTINUE`, naming the next task and the lost continuation. Do not mark
that ready-work state as goal completion or promise background execution.

Validated closed context returns PASS without Git gates. Open empty/exhausted
returns PASS after normal safety checks; the area stays open. A real blocker,
unsafe state, invalid result, failed completion or commit, or a user decision
stops immediately and preserves unfinished work. Actionable task-owned rework
follows the shared bounded correction cycle; exhausted corrections stop the
iteration and remain exhausted across completion wakes. A stopped or paused run's late completion wake does
not authorize dispatch or commit; resume only on an explicit user request.

Use canonical `PASS zdev-loop <area>`, `CONTINUE zdev-loop <area>`, or
`BLOCKER zdev-loop <area>`. Include Area, optional Focus, Lifecycle, Queue,
the stale Advisory once when observed, Tasks completed, Commits, and Stop
reason. CONTINUE adds Next task. BLOCKER adds Current task, Failed stage,
Reason, and Preserved state. Count only successfully completed task/commit
pairs; never treat exhaustion as explicit area closure. On terminal PASS or
BLOCKER, settle the matching active goal with an exposed native completion tool
when available; otherwise end the turn with the stop evidence for its native
evaluator. Never complete an unrelated, paused, or budget-limited goal. T3's
goal badge alone is not verification or completion evidence for a zdev task.

## Stop and recover

On explicit stop, pause, cancellation, or a terminal blocker or user decision,
stop new dispatch and use `task_cancel` for owned active work. Its result can be
`cancel_requested`; inspect status until termination is confirmed before any
replacement worker or cleanup, final stop report, or goal settlement. If
termination cannot be confirmed, report the pending handles and uncertainty
without claiming the run finished; retain the stop, preserve the state, and
explain T3's Stop control. Keep ambiguous execution and all unfinished source
changes preserved.
T3 child cancellation and runtime recovery do not complete a zdev task.

T3's Stop control also stops delegated descendants. Codex Stop pauses its native
goal; Claude Stop ends the turn but retains its goal, which may be checked after
the next message. Retain that the zdev run was stopped. An unrelated message,
native goal check, or late wake does not authorize resuming it: only an explicit
request does. Explain that Claude's retained goal can be removed with T3's Clear
control or `/goal clear`; do not issue that command on the user's behalf.
Budget or usage limits and an unexpectedly paused, cleared, or completed goal
also prevent new dispatch. Report the observed state and reconcile active
children before a later explicitly authorized resume.

After a server restart or uncertain delivery, inspect retained handles and
their child execution before a new dispatch. Read fresh zdev context and Git
state and apply `recovery.md`: retain ownership and finish any already-verified
completion or commit checkpoint only with its required evidence. A repeated
wake or restored provider session never authorizes skipping snapshot checks.
If conversation state lost a baseline, frozen selection, phase, or handle,
reconstruct it from task records, commits, and T3 thread history. If that evidence
is insufficient, report a blocker and preserve the state for explicit recovery.

## Worker roles

Workers read their assigned role here and the shared contract sections named
in their payload. Confirm the supplied cwd, branch, and admitted task or boundary
before acting. Follow repository instructions and the requested validation level.
Keep coordination, `.zdev`, staging, commits, and delegation with the parent.

- **Planner:** read-only. Load the baseline, brief, task, and relevant source;
  return exactly `verdict`, `summary`, `plan`, and `findings` under the shared
  semantic planner schema. Planned paths are starting seams, not an allowlist.
- **Routine implementer:** handle only authored routine work with its exact
  implementation scope and narrow validation. Return the shared nine-field
  implementer object with verdict `ready` or `blocker` and escalation `none`.
- **Standard or advanced implementer:** implement the whole assigned outcome,
  follow the accepted plan when supplied, and validate at the agreed level.
  Return that same implementer object with changed files and validation in
  evidence. In-scope remaining work or another attributable file is actionable,
  not a terminal blocker. Load the shared derived-work contract only if a split
  becomes necessary; the parent reviews and applies any proposal.
- **Verifier:** fresh and read-only. Load the supplied snapshot, inspect the
  entire task and diff on initial review; for a focused recheck follow the prior
  review and accepted corrections under [review.md](review.md). Run required
  checks. Return exactly `verdict`,
  `summary`, `findings`, and `escalation` under the shared semantic verifier
  schema. Report validation-written files under its existing rules. For audit
  work, use the shared textual audit envelope instead of task-verifier JSON.

<!-- zdev:generated-repository-guidance:start -->
## Repository guidance discovery

Before inspecting, planning, changing, or validating code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.
<!-- zdev:generated-repository-guidance:end -->

## Coding guidance

Coding guidance file: "/Users/zayenz/projects/zdev/.claude/skills/zdev/skills/zdev/references/coding.md"

Before planning code, read Design and Code standards from this file. Before implementing or repairing code, also read Completion. Before verification, audit, or code review, read Code standards and Review. Pass this exact location to each relevant worker; read it directly even when the root skill is not loaded. Routine coordination and non-code tasks do not need it.

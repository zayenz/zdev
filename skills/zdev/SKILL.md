---
name: zdev
description: "Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use when the user invokes zdev or $zdev, names an existing .zdev area, task, or note, or asks to continue stored zdev work."
---

# Zdev for Codex

## Select orchestration before the workflow

An explicit request to use zdev **with T3 orchestration** selects
[T3 Code coordination](references/t3-code.md). Read that reference before
selecting native dispatch, worker profiles, or continuation. It supports
implementation, verification, plan-only work, audits, and goal/loop in a T3
main thread running Codex or Claude Code. Keep this selection for the logical
run, its native goal continuations, completion wakes, and explicitly requested
recovery. Goal/loop uses the visible parent `/goal` described in that reference;
a leading `/goal` does not switch workers back to native dispatch. Ordinary
requests use the native harness route unless they continue that selected T3 run.

For the selected T3 route, the reference replaces the native dispatch, scoped
profile resolution, and goal/workflow instructions below. The shared zdev
ownership, safety, result, verification, and commit contracts still apply.
Availability of T3 tools alone does not select the route. An unavailable T3
route reports a blocker instead of substituting native orchestration.

Example: “Use zdev with T3 orchestration to loop through area scheduling,
using Codex for implementation and Claude Code for verification.”

## Activate zdev, then route intent

Activate this workflow when the user invokes `zdev` or `$zdev`, names a stored
`.zdev` area, task, or note, or asks to continue stored zdev work. Once active, words
such as “audit,” “goal,” “loop,” and “implement” select the matching route.

| Active zdev intent | One route |
| --- | --- |
| **Notes** — add a note, list notes, or make an area from a note or folder | [references/notes.md](references/notes.md) |
| **Explore an objective** — start or revise an area and its brief; aliases: “wayfind,” “shape” | [references/shape-work.md](references/shape-work.md) |
| **Discuss the brief** — challenge or sharpen an existing brief; alias: “grill” | [references/discuss.md](references/discuss.md) |
| **Improve** — broadly inspect the codebase and propose candidate work | [references/improve.md](references/improve.md) |
| **Review** — `zdev review` checks the requested changes or code boundary; defaults to local changes | **Standalone review** in [references/review.md](references/review.md) |
| **Audit** — inspect a named boundary and return only independently checked findings | [references/audit.md](references/audit.md) |
| **Investigate** — answer one named checkable uncertainty through research, diagnosis, or a prototype | [references/investigate.md](references/investigate.md) |
| **Create tasks** — draft, review, and import an approved task split; aliases: “tasks,” “to tasks” | [references/to-tasks.md](references/to-tasks.md) and [references/task-format.md](references/task-format.md) |
| **Implement** — complete and commit one next ready task; aliases: “continue,” “next task” | [references/task-workflows.md](references/task-workflows.md), [references/implement.md](references/implement.md), and [references/verify.md](references/verify.md) |
| **Parallel** — run compatible tasks in four independent worktree slots; aliases: “parallel tasks,” “parallel batch” | [references/parallel.md](references/parallel.md) and [references/task-workflows.md](references/task-workflows.md) |
| **Plan one task** — select and plan one ready task without implementation; exact request: “plan next task with advanced planner” | [references/plan-task.md](references/plan-task.md) and [references/task-workflows.md](references/task-workflows.md) |
| **Verify** — independently review the explicit current ready task | [references/verify-workflow.md](references/verify-workflow.md) and [references/verify.md](references/verify.md) |
| **Goal / loop** — synonymous requests to continue a named area one task and commit at a time | **Goal and loop** below and [references/area-loop.md](references/area-loop.md) |
| **Recover** — resume interrupted task work or a managed rebase | [references/recovery.md](references/recovery.md) |
| **Configure** — inspect or change project and worker settings; alias: “config” | Follow **Configuration** below and `zdev config --help` |
| **Research run** — authorize or resume research experiments and generate scoped work | [references/research.md](references/research.md) |
| **Research records** — configure research settings or record/inspect hypothesis evidence | [references/task-format.md](references/task-format.md#research-settings-and-hypothesis-evidence) |
| **Set up zdev** — initialize durable state or install/check a harness integration | [references/setup.md](references/setup.md) |

Use `zdev next --any --format json` only when the user explicitly asks for any
ready or unblocked task across areas. A generic request to continue, select the
next task, or work without naming an area keeps the ordinary area-specific
selection rules.

An area with research settings can mix ordinary tasks and hypotheses. Inspect
the selected task's `hypothesis` field before ordinary planning, implementation,
verification, loop, or parallel dispatch. Stop before ordinary workers;
an explicitly authorized **Research run** uses its separate installed contract.
Explain the research record contract for supplied settings/evidence operations.
`area research` and `task research` validate supplied settings and evidence;
they do not execute evaluators or establish recorded Git operations. Ordinary
supporting tasks keep their existing routes.

Load the references named by the selected row and read each completely once.
Read the applicable coding guidance sections when the route involves code. Do
not ask a reference to choose another route. Run requested interactions
in their requested order; load a shared reference only at its first use. After
the last interaction, report the result and wait. If a pending reviewed bundle
changes, renew approval only when the user retained exact-bundle review. During
authorized task work, coordination may instead clarify an incidental technical restriction
under the active-task rules in the task workflow. Ask which interaction comes
first only when the requested order is unclear.

## Ask only for undecided choices

Approval steps in this skill are workflow defaults. Explicit user instructions
take precedence: a user may delegate task creation and import, waive bundle
confirmation, or authorize in-scope revisions for an entire run. Retain that
authority across workers, recovery, and continuation until the user changes it.
For example, “create and import the tasks without asking me” authorizes the
coordinator to review and import the resulting bundle without a confirmation
turn. Keep normal validation and ownership checks; ask only about material
choices outside the delegated scope. Task creation alone does not authorize
implementation.
When a question is needed, name the concrete undecided choice and why its
answer matters. Treat a default confirmation step as a choice the user can
delegate, rather than reporting it as a capability or safety blocker.

Use one focused question when its answer determines the next step. Batch only
independent questions, give a recommended answer and its concrete trade-off,
and follow the available tool's restrictions. Use plain text when the tool
cannot request the needed input or approval. An unanswered question is not
consent, and a choice already stated for this work does not need confirmation.

Use Codex's `request_user_input` tool when it is available. It supports one to three questions per call: ask one focused question by default and batch only independent questions. Put the recommended option first for each question and explain its impact. Use plain text when the tool is unavailable or cannot request the needed input or approval.

Begin read-only orientation in the current checkout. For implementation,
reuse an explicit workspace choice. Otherwise run `zdev config get
workspace.branching --format json`. Under `on-request`, the default, implement
in the current checkout and create a branch or worktree only when the user asks
for one or the route needs one: parallel batches, rolling research, and an
assigned source worktree. Under `isolated`, use a worktree as the authoritative
checkout before admitting task work: reuse a suitable worktree or create one on
the area's required branch. State the selected workspace and apply the normal
branch-safety and ownership checks. Ask only when the destination or ownership
is ambiguous.

Keep interaction boundaries explicit. Exploration presents the resulting brief
for discussion unless the user also requested task creation. An explicit,
sufficiently specified task request may proceed to the exact task-bundle review.
Import an approved or delegated bundle, but begin implementation only when the
user's request also authorizes implementation; reuse that authorization when
it is already clear.

## Development model

An area moves from a brief to approved tasks, implementation, independent
verification, completion, and commit. **Explore** and **Discuss** shape the
brief, including scope and testing. **Create tasks** turns that brief into an
exact bundle for review, with user confirmation unless delegated. **Implement**
selects one ready task, records the Git baseline, and changes only attributable
task-owned paths. Paths named during
planning are starting points, not an allowlist, unless an explicit boundary
says otherwise. Immediately before a fresh
verifier, coordination stores and validates its work-context snapshot. The
verifier checks the task requirements, touched code, and required validation,
then returns four semantic fields. Coordination compares the snapshot and
constructs the compatible public envelope. The coordinating agent completes
and commits the task after `PASS`.

One **Implement** interaction stops after reporting that verified commit. An
explicit request to continue, or an active goal or loop, starts another
iteration only after collecting fresh post-commit task context; it does not
reuse the completed task's selection.

An investigation or implementation worker may propose a small direct follow-up
or split through the strict transient derived-work contract in the selected
reference or implementation entrypoint. The coordinator alone reviews and
applies it. Clear in-scope authority uses direct apply. An AFK task may split
when smaller tasks make implementation or verification easier; this needs no
additional user approval when its outcome, boundaries, and testing stay the
same. A decision outside retained authority uses a stored review and ordinary
approval. Derived tasks then use the normal area graph.

Larger areas may organize several related increments as slice briefs under
`.zdev/<area>/slices/`. A slice records only a title, objective, and boundaries;
it has no status or required task membership. The area brief remains
authoritative for shared decisions and testing.

The brief and selected task define the outcome, boundaries, testing level, and
done conditions throughout this process. A newly discovered file may become
task-owned when changing it is directly necessary for those requirements, its
baseline ownership is clear, and the change stays within the approved semantic
boundaries. File count alone is never scope expansion.

Use `general` as the conventional tag for recurring one-off work when the user
wants one standing area instead of a new area for each small improvement. With
personal or project records under the default `workspace.branching` value
`on-request`, it is a trunk area on configured trunk. It uses an isolated persistent branch when the user asks for
one, when the setting is `isolated`, or under pull-request records. Keep a minimal brief with shared
boundaries, testing, and validation. Unsliced tasks are normal; use slice briefs
only when several tasks share one narrower objective.

When discussion leaves no unresolved product or testing choice, an explicit
request may proceed directly to **Create tasks** and exact task-bundle review.
This shorter planning path still requires concrete outcomes, boundaries, done
proof, approval or delegated authority, branch safety, proportionate testing,
independent verification, and committed accepted work.

Confirm `zdev` is available before using durable state. When `.zdev` is absent,
run standalone **Improve** and **Investigate** without initialization. Load the
setup route only when the user asks to create durable work. If several areas
have open work and none is selected, show their tags and ask the user to
choose; do not infer an area from unrelated chat history.

For area planning interactions, report the selected area's mode, resolved
required branch, and base diagnostics. An isolated area uses its stored branch;
an explicit trunk area dynamically follows `project.trunk`. One thread per
area is the default, but several areas may share a thread, worktree, and branch.
Require the resolved branch before changing task or lifecycle state. Never request freshness or a managed rebase
for trunk mode; read-only interactions never rebase.

For ordinary task work, use `branch_status.task_work.safe` as the branch gate.
Mode does not grant ownership of other areas or unrelated trunk changes: retain
the exact Git baseline and stage only the selected area/task paths.
Report a stale-but-safe rebase advisory once and continue without requesting a
rebase. Unsafe branch, anchor, ancestry, history, or Git-operation state still
stops implementation, verification, completion, and commit preparation.

Keep existing Git changes in place. Establish ownership before touching an
overlapping path or changing the index.

## Apply shared coding guidance

Read [coding guidance](references/coding.md) before planning, authoring, or
reviewing code. Names explain intent, structure separates responsibilities, and
values make dependencies explicit; readability contributes to correctness.
Apply **Design** and **Completion** when authoring and **Review** when assessing
code, with **Code standards** applying to both. Naming and decomposition are
acceptance requirements: implementers complete the naming and decomposition
check before claiming readiness; reviewers inspect them explicitly before a
pass. Passing tests alone does not satisfy these requirements. Keep the work
proportional to the task and its agreed testing level.

Give every relevant worker the supplied installed coding guidance path and tell
it which sections to read. Carry changed contracts and available process
evidence in existing plans, summaries, and handoffs. Independently check
substantive findings before they drive rework or reporting, as specified there;
use the task workflow's snapshot and result rules for task verification.
Before routing corrections, read [review.md](references/review.md) and enforce
its bounded correction cycle. During retries, task splitting, and goal/loop
continuation, apply its **Death loops** guidance. Load the language and framework
supplements selected by **Code standards** for each relevant worker.

## Scoped execution profiles

A named execution profile chooses concrete model and effort settings for worker
roles. It does not choose the coordinator model, add workers, or authorize a
different route. Authored task complexity still chooses only
`routine-implementer`, `implementer`, or `advanced-implementer` and whether the
ordinary advanced planning step is required.

At the start of each interaction or authorized multi-task run, resolve every
role that the route can use with `zdev config profile resolve <harness> <role>
--area <area> --run-profile <name> --format json`. Pass `--area` whenever the
work belongs to a named area, and omit `--run-profile` when the user did not
choose one. Retain the returned profile name and concrete model and effort for
the whole logical run; pass those concrete values at every later dispatch,
including rework, escalation, verification, continuation, recovery, and a
parallel handoff. Do not resolve the name again after shared preferences
change. A later independent run resolves afresh.

A one-off role choice adds `--profile <name>` for that role and has precedence
over the run profile. Its concrete result lasts through retries or replacement
of that same logical step, then expires. Selection precedence is one-off role,
run, the current harness's area profile, saved local default, saved global
default, then `normal`. Use the resolver
for this logic; do not reproduce its fallback rules in a harness workflow or
rewrite `.zdev/workers.toml` or an installed integration.

Resolve workers before their first dispatch. This includes implementers,
required planners, verifiers used for implementation or standalone verify,
standalone review and audit verifiers, task-bundle challenge reviewers, and
workers already requested for delegated discussion or investigation. A profile
choice by itself never adds delegation. When an authorized continuation or parallel batch is already
active, its handoff retains the frozen concrete selections.

Report the selected profile and requested concrete model and effort at the
dispatch boundary. If the harness rejects or substitutes either value, report
the requested and observed values and stop or continue only according to the
harness's explicit result. Never silently substitute a worker or claim that a
worker selection changed the coordinating conversation's model. An unknown or
unsupported profile is a blocker before dispatch.

Claude's executable workflows pass both concrete fields to the native
`agent()` call. When those workflows are unavailable, the ordinary Agent
fallback may use an installed prepared role definition only when its model and
effort match the resolved row exactly. If the fallback cannot express the
requested effort or has no exact prepared definition, report that capability
limit and the refresh command instead of substituting another row or claiming
selection succeeded.

## Goal and loop

Inside active zdev, “goal” and “loop” are synonyms: continue one named area one
independently verified task and commit at a time. The binary command `zdev goal
<area>` remains the deterministic projection of one task; it is context for an
iteration, not this continuing user intent.

The canonical explicit route is `zdev-loop <area> [focus...]` and `zdev-goal
<area> [focus...]` is an exact semantic alias. Both use canonical `zdev-loop`
results. Everything after the area is optional fuzzy guidance, not a flag or
exact task filter. Do not treat the alias as a one-task mode or confuse either
route with the binary projection.

An explicit request to use parallel batches throughout this goal or loop
selects the supported **Parallel** route for compatible finite batches. Read
`references/parallel.md` and retain the user's blanket authorization in the
existing handoff. Select each later batch from fresh ready tasks and recheck
admission within the same area, worker, destination, worktree and cleanup
limits. Reuse those decided choices; request a new decision only when it falls
outside the retained authority. Without that authorization, keep the ordinary
one-task route and its single-batch boundaries.

With no focus, let `zdev work-context <area>` select one task using the binary's
AFK, priority, then numeric ordering. With any focus, obtain the complete ready
frontier with `zdev tasks list <area> --format json`, read every ready task with
`zdev task show <area> <task-id> --format json`, and let the coordinating model
choose the best fit from those full task records. When any ready task has
`afk = true`, choose only among those unattended tasks and use the focus to rank
that eligible set. Choose an `afk = false` attended task only when no unattended
task is ready. Do not keyword-filter or pre-rank the frontier before that
choice. Admit the chosen task with `zdev work-context <area> --task <task-id>`.
Repeat the same focused selection from a fresh frontier after every commit;
focus never becomes stored zdev state.

Each ordinary sequential iteration uses the **Implement** route and stops internally only on a
verified commit, a blocker, or a user-owned decision. Tell the user which task
was selected and when its verified commit completes. Before another task,
repeat the selection rule above from fresh evidence; never reuse the completed
task's selection. Finish on open `empty`, open `exhausted`, open `blocked`, or
validated `closed`. Open `blocked` means every open task waits on a task in
another area: report each wait in the goal's `cross_area_waits` as `<task>
blocked by <id> (<area>: <status>)` and select no work in the blocking area.
Closed is classified before Git and branch gates. Open work still
requires `branch_status.task_work.safe`; a stale-but-safe base is one advisory,
not a blocker.

When a workflow uses `zdev work-context <area> --store --format json`, pass its
compact filesystem reference instead of copying the complete JSON. Read exact
handoff bytes with `--show <snapshot>` and use `--compare <snapshot>` at a later
boundary. A stored snapshot is immutable evidence from its collection point,
never current authority: comparison collects fresh work-context. Stored
snapshots remain available so an active workflow can keep loading its original
baseline. Do not add approval or history ceremony around it.

The fixed results begin `PASS zdev-loop <area>`, `CONTINUE zdev-loop <area>`,
or `BLOCKER zdev-loop <area>`. `CONTINUE` is valid only after one independently
verified task was completed and committed and fresh work-context reports
another open, ready, safe task. `REWORK` stays inside the current task; worker,
validation, completion, commit, refresh, unsafe-state, and user-decision
failures stop as `BLOCKER` only when they are genuine impasses. Partial progress,
remaining in-scope work, or a newly discovered attributable path stays inside
the current iteration. No invocation stores durable loop state.

The harness-native section says whether continuation is native or bounded. A
bounded fallback completes at most one task, reports the fresh next state, and
never claims a background loop. On a harness with native goals, an unfinished
native goal wins: do not replace, clear, edit, or layer work over it. Native
goal failure leaves both session goal and zdev state unchanged.

## Configuration

Use `zdev config show`, `get <key>`, `set <key> <value>`, and `unset <key>` for
the fixed project and worker registry; use `zdev config trunk` for the
branch-aware trunk convenience. Read command help when the key, value, or scope
is unclear. Use `zdev area profile <area>` to show the area's harness-profile
mapping, `zdev area profile <area> <harness>` to inspect one harness, `zdev area
profile <area> <harness> <profile>` to set or replace it, and `zdev area profile
<area> <harness> --clear` to clear it. For a natural-language request, identify
the named area and harness, perform only the requested read or mutation, and
report the resulting setting plainly. Do not infer a harness the user did not
name. Configuration leaves slice and task records unchanged. A successful worker-profile mutation
reports the exact `zdev skill install <harness> ... --force` refresh command;
report it without installing or rewriting the integration automatically. An
area-profile mutation does not reinstall an integration or change the
coordinator model.

## Write human-facing prose plainly

When composing or revising human-facing prose written for zdev, preserve the
meaning and match the intended tone. Name the actor and the concrete action.
Prefer specific facts, plain words, active voice, and sentences a reader does
not need to backtrack through. Split dense sentences and cut adverbs that hide
an unmeasured claim. Remove puffery, promotional claims, vague attribution,
canned chatbot phrases, filler, excessive hedging, forced parallel structure,
abstract metaphor jargon, and synonym cycling. Vary sentence length where it
helps readability, but keep repository terms stable instead of inventing
synonyms. Replace generic claims with the zdev-specific mechanism, instruction,
fact, or number. If a sentence could describe any project unchanged, cut it.
Reread the finished draft and replace any remaining generic or formulaic
phrasing.

This editorial pass does not apply to user quotations or source text. Never use
it to rewrite code, commands, paths, literals, JSON, TOML, YAML, frontmatter,
generated records, or approved task content. Semantic accuracy, repository
terminology, explicit user instructions, and the area, slice, and task contracts
take priority over style preferences.

This guidance adapts Lauren Tan's MIT-licensed Cursor pstack `unslop` method at
commit `93b00b89ef425a9c1bac0d0b317dfc49c930ac99`.

## Write durable records for agents

Write briefs, slice briefs, and tasks for the agent branch that will consume
them. Keep each shared meaning in one authoritative place, and keep a concept's
definition, rules, and caveats together. A pointer names both the material and
the condition for reading it. Inline context every branch needs and point to
branch-specific detail only where that branch uses it. Treat source,
configuration, and command help as authoritative for facts an agent can inspect
cheaply instead of copying those facts into a record. State the behavior the
agent should perform. Each completion condition must be checkable, and together
the conditions must cover the promised outcome. Remove exposition and stale
restatements that do not change an agent's action or decision.

This section governs durable zdev records. Transient worker payloads follow the
separate task-workflow contract because those workers do not load every planning
route.

## State and reporting

Store metadata, `brief.md`, optional slice briefs, task files, generated
`TASKS.md`, indexed area research, and free-form notes under `.zdev`. Keep
unfinished plans, future ideas, and collected research documents in
`.zdev/_notes/`, as individual files or folders with optional nested folders.
Notes have no required metadata, status, or index and are not selectable work.
Use [Notes](references/notes.md) when saving, listing, or turning them into an
area. Read only notes relevant to the user's request; their contents do not
override an approved brief or task or authorize implementation.

Retain research under
`.zdev/<area>/background/` only during approved area shaping or an authorized
investigation task, and only when later tasks will reuse readable, stable,
source-backed material. Index every retained file from `brief.md`; link only
relevant files from tasks, and keep the brief as the authoritative synthesis.
Do not retain transcripts, raw tool or search dumps, repository source copies,
temporary prototypes, or lifecycle metadata as background files.

Task-bundle review artifacts live in repository-local Git administrative state
and are accessed through `zdev tasks review`; keep other transcripts and review
evidence in the conversation. Existing domain documentation and ADRs remain
authoritative across areas. Report what changed, what verification passed, and
what remains; mention commands only when they help the user continue or recover.

## Codex orchestration

The root `$zdev` skill selects the route and loads its contract from
`references/`. Treat “goal” and “loop” as the same native continuation route.
Codex supports the explicit **Parallel** route in `references/parallel.md`.
Codex also supports the read-only **Plan one task** route in
`references/plan-task.md`.
An explicitly authorized **Research run** uses `references/research.md` for
serial current-branch experiments or explicitly authorized rolling source
worktrees, mixed task generation, counted role/measurement dispatch and durable
recovery. Research continuation follows that contract; ordinary goal
and loop requests keep `references/area-loop.md` and stop at queue exhaustion.
The exact installed task-workflows contract path for this installation is
"references/task-workflows.md". Decode that JSON string. Resolve a
relative path against the directory containing this SKILL.md and include the
resulting absolute path in every worker payload.

Use one Codex collaboration agent to implement a task and a different agent to
verify it. The coordinating agent owns zdev state, user decisions, task
completion, and commits. Give each agent the rendered repository guidance and
applicable `AGENTS.md` instructions.

For an assigned source worktree, the shared task-workflows
contract owns isolation and Git transport. Codex collaboration agents share the
parent directory by default, so every source implementer or rework payload must
name the absolute assigned source root and require all file and command tools to
use it as cwd. Keep the authoritative root, record paths, and destination
snapshot paths absolute and distinct. Confirm the agent can access its assigned
root; do not broaden permissions or fall back to concurrent destination edits.
The coordinating Codex session creates and inspects source commits, integrates
them in the destination, and dispatches the verifier against the integrated
destination snapshot.

Spawn each role with `fork_turns="none"`. Use a compact filesystem-backed
message containing its role and exact area, task, or boundary identity; the
exact installed route-contract and supplied coding guidance paths; applicable
repository-instruction paths;
authoritative brief, slice, and task paths; and the opaque work-context
snapshot when its route provides one. The agent reads those paths directly and
returns the route's required result as its final response, then ends its turn.
Task roles use their JSON objects; standalone review and audit use their textual
reports.

The coordinator owns worker cleanup. Keep each worker's ID, role, task, and
status in the conversation. After collecting a result, close the worker with
the runtime's close operation when available; retain an implementer only while
same-task rework may need it. Release retained workers when the task finishes
or stops, and superseded workers before spawning replacements. Before retrying
a capacity failure, inspect existing workers and release completed ones no
longer needed. On cancellation, interrupt owned active workers and confirm they
have stopped. Follow the exposed tools: a finished turn or interruption does
not imply a closed thread, and runtimes without a close operation need only
confirmed turn completion.

For plan-only and implementation interactions, run `zdev config profile
dispatch-spec <plan-next-task|implement> --area <area> --task <task-id>
--harness codex --snapshot <snapshot> ... --format json` after storing the explicit task
context. Pass any run choice with `--run-profile`, a one-off choice with
`--role-profile ROLE=PROFILE`, and retained-plan evidence with
`--retained-plan <applicable|stale> --plan-snapshot <snapshot>`. Accept only its
strict `dispatch-spec` object and execute its ordered `dispatches` until
the stated `stop` boundary. Retain the complete concrete specification in the
conversation handoff; same-step retry reuses it instead of resolving again.
The command compares the supplied explicit-task snapshot with current
authoritative context before emitting a dispatch. A stale-snapshot error has an
empty dispatch list; collect fresh explicit-task context and reassess the route
instead of dispatching from historical state.
Pass each emitted `model` and `reasoning_effort` to `spawn_agent`.
If Codex rejects those arguments or reports a substitution, preserve the
requested values in the result and report the rejection or observed values;
do not retry silently with the baked installation setting. The rendered values
below are the installed normal defaults only when no runtime selection was
requested or saved.

When an implementer will author human-facing prose, include the shared `Write
human-facing prose plainly` section in its compact payload. Other workers do not
need that editorial guidance.

For task verification, store and validate the snapshot immediately before
spawning the verifier. Extract one balanced JSON object from its response,
tolerating brief prose or a Markdown fence, validate the four semantic fields,
compare the snapshot, independently vet substantive findings under the task
workflow, and construct the compatible nine-key verifier envelope in the
coordinating session. Never repeat a worker only to remove valid
wrapping text.

For the implementer, pass `model="gpt-6.1-sol"` and
`reasoning_effort="high"` together with `fork_turns="none"`.
For the verifier, pass `model="gpt-6.1-sol"` and
`reasoning_effort="high"` together with `fork_turns="none"`.
For the planner, pass `model="gpt-6.1-sol"` and
`reasoning_effort="xhigh"` together with `fork_turns="none"`.

For an active-zdev goal or loop request, follow the internal area-loop
contract. It calls `get_goal` before repository work, preserves an unfinished
goal, and calls `create_goal` with the shared condition only when native goal
state is clear. Native unavailability falls back to at most one verified
committed task and returns canonical `CONTINUE zdev-loop <area>` only when
fresh ready work remains.

For ordinary Area loop only, the native condition completes the run after
reporting a confirmed blocker or required user-owned decision. On that first terminal
`BLOCKER`, report its reason and preserved state, then call `update_goal` with
`status: "complete"` for the matching active goal in the same turn and return
the canonical `BLOCKER` result. This completes
the run's stop-and-report objective; unfinished tasks and the area retain their
exact state. Do not use `status: "blocked"` for this completed run, wait for
three goal turns, or repeat workers to confirm the same impasse. Complete the
matching active goal on terminal `PASS` as well. Preserve every other native
goal and follow its own objective and lifecycle rules. Research follows its
research reference: a blocker or exhausted allowance never completes an
unfinished research objective.

<!-- zdev:generated-repository-guidance:start -->
## Repository guidance discovery

Before inspecting, planning, changing, or validating code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.
<!-- zdev:generated-repository-guidance:end -->

## Coding guidance

Coding guidance file: "references/coding.md"

Before planning code, read Design and Code standards from this file. Before implementing or repairing code, also read Completion and finish its naming and decomposition check before claiming readiness. Before verification, audit, or code review, read Code standards and Review and perform the explicit naming and decomposition check before a pass. Pass this exact location to each relevant worker; read it directly even when the root skill is not loaded. Routine coordination and non-code tasks do not need it.
Resolve a relative coding guidance path against the directory containing the loaded root zdev SKILL.md before reading or forwarding it. Forward the resolved absolute path to workers.

The [task format](references/task-format.md) defines imported task bundles.

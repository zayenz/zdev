---
name: zdev
description: "Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use when the user invokes zdev or $zdev, names an existing .zdev area or task, or asks to continue stored zdev work."
---

# Zdev for Claude Code

## Activate zdev, then route intent

Activate this workflow when the user invokes `zdev` or `$zdev`, names a stored
`.zdev` area or task, or asks to continue stored zdev work. Once active, words
such as “audit,” “goal,” “loop,” and “implement” select the matching route.

| Active zdev intent | One route |
| --- | --- |
| **Explore an objective** — start or revise an area and its brief; aliases: “wayfind,” “shape” | [references/shape-work.md](references/shape-work.md) |
| **Discuss the brief** — challenge or sharpen an existing brief; alias: “grill” | [references/discuss.md](references/discuss.md) |
| **Improve** — broadly inspect the codebase and propose candidate work | [references/improve.md](references/improve.md) |
| **Audit** — inspect a named boundary and return only independently checked findings | [references/audit.md](references/audit.md) |
| **Investigate** — answer one named checkable uncertainty through research, diagnosis, or a prototype | [references/investigate.md](references/investigate.md) |
| **Create tasks** — draft, review, and import an approved task split; aliases: “tasks,” “to tasks” | [references/to-tasks.md](references/to-tasks.md) and [references/task-format.md](references/task-format.md) |
| **Implement** — complete and commit one next ready task; aliases: “continue,” “next task” | [references/task-workflows.md](references/task-workflows.md), [references/implement.md](references/implement.md), and [references/verify.md](references/verify.md) |
| **Parallel** — run one finite approved batch of compatible tasks; aliases: “parallel tasks,” “parallel batch” | [references/parallel.md](references/parallel.md) and [references/task-workflows.md](references/task-workflows.md) |
| **Plan one task** — select and plan one ready task without implementation; exact request: “plan next task with advanced planner” | [references/plan-task.md](references/plan-task.md) and [references/task-workflows.md](references/task-workflows.md) |
| **Verify** — independently review the explicit current ready task | [references/verify-workflow.md](references/verify-workflow.md) and [references/verify.md](references/verify.md) |
| **Goal / loop** — synonymous requests to continue a named area one task and commit at a time | **Goal and loop** below and [references/area-loop.md](references/area-loop.md) |
| **Recover** — resume interrupted task work or a managed rebase | [references/recovery.md](references/recovery.md) |
| **Configure** — inspect or change project and worker settings; alias: “config” | Follow **Configuration** below and `zdev config --help` |
| **Set up zdev** — initialize durable state or install/check a harness integration | [references/setup.md](references/setup.md) |

Use `zdev next --any --format json` only when the user explicitly asks for any
ready or unblocked task across areas. A generic request to continue, select the
next task, or work without naming an area keeps the ordinary area-specific
selection rules.

Load only the references named by the selected row, read each completely once,
and do not ask a reference to choose another route. Run requested interactions
in their requested order; load a shared reference only at its first use. After
the last interaction, report the result and wait. If a pending reviewed bundle
changes, show the revision and ask for exact approval again. During authorized
task work, coordination may instead clarify an incidental technical restriction
under the active-task rules in the task workflow. Ask which interaction comes
first only when the requested order is unclear.

## Ask only for undecided choices

Use one focused question when its answer determines the next step. Batch only
independent questions, give a recommended answer and its concrete trade-off,
and follow the available tool's restrictions. Use plain text when the tool
cannot request the needed input or approval. An unanswered question is not
consent, and a choice already stated for this work does not need confirmation.

Use Claude Code's `AskUserQuestion` tool when it can request the needed input. Ask one focused question by default and put independent questions in one call only when batching helps. Give each question concrete options, mark the recommended answer in its wording or description, and use plain text for free-form input or approvals the tool cannot request.

Begin discussion and other read-only orientation in the current checkout.
Reuse a branch or worktree choice the user already made for this work.
Otherwise ask before creating or switching branches, creating a worktree, or
making another user-owned choice. Complete useful read-only work first when it
can make the question concrete. Existing branch-safety checks still govern task
and lifecycle mutations.

Keep interaction boundaries explicit. Exploration presents the resulting brief
for discussion unless the user also requested task creation. An explicit,
sufficiently specified task request may proceed to the exact task-bundle review.
Import an approved bundle, but begin implementation only when the user's request
also authorizes implementation; reuse that authorization when it is already
clear.

## Development model

An area moves from a brief to approved tasks, implementation, independent
verification, completion, and commit. **Explore** and **Discuss** shape the
brief, including scope and testing. **Create tasks** turns that brief into an
exact bundle for approval. **Implement** selects one ready task, records the Git
baseline, and changes only attributable task-owned paths. Paths named during
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
applies it. Clear in-scope authority uses direct apply. A decision that belongs
to the user uses a stored review and ordinary approval. Derived tasks then use
the normal area graph.

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
wants one standing area instead of a new area for each small improvement. It
may use the default isolated branch or explicit `--trunk` mode when the user
wants several areas to share configured trunk. Keep a minimal brief with shared
boundaries, testing, and validation. Unsliced tasks are normal; use slice briefs
only when several tasks share one narrower objective.

When discussion leaves no unresolved product or testing choice, an explicit
request may proceed directly to **Create tasks** and exact task-bundle review.
This shorter planning path still requires concrete outcomes, boundaries, done
proof, approval, branch safety, proportionate testing, independent
verification, and committed accepted work.

Confirm `zdev` is available before using durable state. When `.zdev` is absent,
run standalone **Improve** and **Investigate** without initialization. Load the
setup route only when the user asks to create durable work. If several areas
have open work and none is selected, show their tags and ask the user to
choose; do not infer an area from unrelated chat history.

For area planning interactions, report the selected area's mode, resolved
required branch, and base diagnostics. An isolated area owns its stored branch;
an explicit trunk area dynamically follows `project.trunk` and may share it
only with other explicit trunk areas. Require the resolved branch before
changing task or lifecycle state. Never request freshness or a managed rebase
for trunk mode; read-only interactions never rebase.

For ordinary task work, use `branch_status.task_work.safe` as the branch gate.
Mode does not grant ownership of other areas or unrelated trunk changes: retain
the exact Git baseline and stage only the selected area/task paths.
Report a stale-but-safe rebase advisory once and continue without requesting a
rebase. Unsafe branch, anchor, ancestry, history, or Git-operation state still
stops implementation, verification, completion, and commit preparation.

Keep existing Git changes in place. Establish ownership before touching an
overlapping path or changing the index.

## Scoped execution profiles

A named execution profile chooses concrete model and effort settings for worker
roles. It does not choose the coordinator model, add workers, or authorize a
different route. Authored task complexity still chooses only
`routine-implementer`, `implementer`, or `advanced-implementer` and whether the
ordinary advanced planning step is required.

At the start of each interaction or authorized multi-task run, resolve every
role that the route can use with `zdev config profile resolve <harness> <role>
--run-profile <name> --format json`. Omit `--run-profile` when the user did not
choose one. Retain the returned profile name and concrete model and effort for
the whole logical run; pass those concrete values at every later dispatch,
including rework, escalation, verification, continuation, recovery, and a
parallel handoff. Do not resolve the name again after shared preferences
change. A later independent run resolves afresh.

A one-off role choice adds `--profile <name>` for that role and has precedence
over the run profile. Its concrete result lasts through retries or replacement
of that same logical step, then expires. Selection precedence is one-off role,
run, saved local default, saved global default, then `normal`. Use the resolver
for this logic; do not reproduce its fallback rules in a harness workflow or
rewrite `.zdev/workers.toml` or an installed integration.

Resolve workers before their first dispatch. This includes implementers,
required planners, verifiers used for implementation or standalone verify,
audit verifiers, task-bundle challenge reviewers, and workers already requested
for delegated discussion or investigation. A profile choice by itself never
adds delegation. When an authorized continuation or parallel batch is already
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

Each iteration uses the **Implement** route and stops internally only on a
verified commit, a blocker, or a user-owned decision. Tell the user which task
was selected and when its verified commit completes. Before another task,
repeat the selection rule above from fresh evidence; never reuse the completed
task's selection. Finish on open `empty`, open `exhausted`, or validated
`closed`. Closed is classified before Git and branch gates. Open work still
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
is unclear. Configuration leaves area, slice, and task records unchanged. A successful worker-profile mutation
reports the exact `zdev skill install <harness> ... --force` refresh command;
report it without installing or rewriting the integration automatically.

## Write human-facing prose plainly

When composing or revising human-facing prose written for zdev, preserve the
meaning and match the intended tone. Name the actor and the concrete action.
Prefer specific facts, plain words, active voice, and sentences a reader does
not need to backtrack through. Remove puffery, promotional claims, vague
attribution, canned chatbot phrases, filler, excessive hedging, forced parallel
structure, and synonym cycling. Vary sentence length where it helps readability,
but keep repository terms stable instead of inventing synonyms. Reread the
finished draft and replace any remaining generic or formulaic phrasing.

This editorial pass does not apply to user quotations or source text. Never use
it to rewrite code, commands, paths, literals, JSON, TOML, YAML, frontmatter,
generated records, or approved task content. Semantic accuracy, repository
terminology, explicit user instructions, and the area, slice, and task contracts
take priority over style preferences.

This guidance adapts Lauren Tan's MIT-licensed Cursor pstack `unslop` method at
commit `93b00b89ef425a9c1bac0d0b317dfc49c930ac99`.

## State and reporting

Store only metadata, `brief.md`, optional slice briefs, task files, generated
`TASKS.md`, and indexed area research under `.zdev`. Retain research under
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

## Claude Code orchestration

Route the goal's authored complexity through `zdev:zdev-routine-implementer`,
`zdev:zdev-implementer`, or `zdev:zdev-advanced-implementer`. Advanced work
first uses one read-only `zdev:zdev-planner`. Always verify with a fresh
`zdev:zdev-verifier`. Ordinary rework stays on the selected profile; one valid
standard-work escalation uses an advanced replacement without replanning.
Include rendered repository guidance in every prompt. If named agents are
unavailable, use ordinary Claude Code subagents with the same profiles and
boundaries.

When an implementer will author human-facing prose, include the shared `Write
human-facing prose plainly` section in its prompt. Other workers do not need
that editorial guidance.

The root zdev skill selects the route and loads its contract from `references/`.
When packaged workflows are available, it uses them internally for a full task
cycle, explicit verification, audit, or continuing area work. “Goal” and
“loop” select the same continuation workflow. The ordinary subagent loop also
works.

An explicit parallel request uses the packaged `zdev-parallel` workflow when
it is installed. Collect the finite task list, destination, worktree authority,
worker limit, cleanup choice, and explicit consent before launching it. If the
workflow runtime is unavailable, report that parallel execution is unavailable
and offer the unchanged sequential Implement route.

For an active-zdev goal or loop request, use the packaged continuation workflow
when available. It repeats the ordinary one-task route, refreshes work
context after every verified commit, and applies the shared stop states. It
does not inspect or invoke Claude Code's separate `/goal` command. If the
packaged workflows are unavailable, continue under coordinator control or stop
after one task and report the fresh next state.

Before launching a packaged workflow, tell the user which area and fuzzy focus
will be used. Workflow labels report selection, the chosen task ID, planning,
implementation, verification, rework, and commit stages; do not replace them
with a generic “waiting for dynamic workflow” update. Report completed task IDs
and commits from the final envelope.

<!-- zdev:generated-repository-guidance:start -->
## Repository guidance discovery

Before inspecting, planning, changing, or validating code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.
<!-- zdev:generated-repository-guidance:end -->

The [task format](references/task-format.md) defines imported task bundles.

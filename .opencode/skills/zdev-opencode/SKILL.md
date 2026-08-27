---
name: zdev-opencode
description: "Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use when the user invokes zdev or $zdev, names an existing .zdev area or task, or asks to continue stored zdev work."
compatibility: opencode
---

# Zdev for OpenCode

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
the last interaction, report the result and wait. If an approved artifact
changes, show the revision and ask for approval again. Ask which interaction
comes first only when the requested order is unclear.

## Development model

An area moves from a brief to approved tasks, implementation, independent
verification, completion, and commit. **Explore** and **Discuss** shape the
brief, including scope and testing. **Create tasks** turns that brief into an
exact bundle for approval. **Implement** selects one ready task, records the Git
baseline, and changes only task-owned paths. Immediately before a fresh
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
done conditions throughout this process.

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
choose the best fit from those full task records. Do not keyword-filter or
pre-rank the frontier before that choice. Admit the chosen task with
`zdev work-context <area> --task <task-id>`. Repeat the same focused selection
from a fresh frontier after every commit; focus never becomes stored zdev state.

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
never current authority: comparison collects fresh work-context, and an expired
ID requires a new snapshot. Do not add approval or history ceremony around it.

The fixed results begin `PASS zdev-loop <area>`, `CONTINUE zdev-loop <area>`,
or `BLOCKER zdev-loop <area>`. `CONTINUE` is valid only after one independently
verified task was completed and committed and fresh work-context reports
another open, ready, safe task. `REWORK` stays inside the current task; worker,
validation, completion, commit, refresh, unsafe-state, and user-decision
failures stop as `BLOCKER`. No invocation stores durable loop state.

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
meaning and match the intended tone. Prefer specific facts and plain words.
Remove puffery, promotional claims, vague attribution, canned chatbot phrases,
excessive hedging, forced parallel structure, synonym cycling, and decorative
formatting. Keep a natural sentence rhythm, repeat stable repository terms, and
use emphasis only when it helps. Reread the draft for formulaic AI phrasing and
fix any remaining tells.

This editorial pass does not apply to user quotations or source text. Never use
it to rewrite code, commands, paths, literals, JSON, TOML, YAML, frontmatter,
generated records, or approved task content. Semantic accuracy, repository
terminology, explicit user instructions, and the area, slice, and task contracts
take priority over style preferences.

This guidance adapts Lauren Tan's MIT-licensed Poteto Noodle `unslop` method at
commit `82d2921c52370f23f29086de81ccfb600939c037`.

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

## OpenCode orchestration

Route authored routine, standard/default, and advanced work to
`@zdev-routine-implementer`, `@zdev-implementer`, or
`@zdev-advanced-implementer`. Advanced work first uses one read-only
`@zdev-planner`. Always verify with a fresh `@zdev-verifier`. Ordinary rework
stays on the selected profile; one valid standard-work escalation uses an
advanced replacement without replanning. Include rendered repository guidance
and applicable instructions in every prompt.

Each subagent starts with its short role definition. Give it the installed
route-contract path and a compact task payload: file paths for the brief, task,
guidance, and relevant source; the applicable snapshot IDs; and the short
result from the preceding role. Let the worker read those files instead of
copying their contents or the full contract into the prompt. An implementer
loads the derived-work section only if it actually needs a split.

The root zdev skill selects the route and loads its contract from `references/`.
It may use the packaged commands internally for a complete task cycle,
verification, audit, or bounded area continuation. “Goal” and “loop” select the
same continuation route.

OpenCode has no required native continuation surface. For an active-zdev goal
or loop request, use the packaged continuation command. It completes at most one task using
the ordinary route, returns canonical `CONTINUE zdev-loop <area>` only after a
verified commit when fresh ready work remains, and never claims a continuing
loop was started.

<!-- zdev:generated-repository-guidance:start -->
## Repository guidance discovery

Before inspecting, planning, changing, or validating code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.
<!-- zdev:generated-repository-guidance:end -->

The [task format](references/task-format.md) defines imported task bundles.

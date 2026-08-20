---
name: zdev-opencode
description: "Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use only when the user invokes zdev or $zdev; asks to work through an existing .zdev area; or unmistakably refers to zdev's stored areas or tasks."
compatibility: opencode
---

# Zdev for OpenCode

## Activate zdev, then route intent

Activate this workflow only for `zdev`, `$zdev`, an existing `.zdev` area,
or an unmistakable reference to zdev's stored areas or tasks. Ordinary intent
words such as “audit,” “explore,” and “implement” route work after activation;
they do not activate zdev alone.

| Active zdev intent | Direct reference |
| --- | --- |
| **Explore an objective** — start or revise an area and its brief; aliases: “wayfind,” “shape” | [references/shape-work.md](references/shape-work.md) |
| **Discuss the brief** — challenge or sharpen an existing brief; alias: “grill” | [references/discuss.md](references/discuss.md) |
| **Improve** — broadly audit or review and propose candidate work | [references/improve.md](references/improve.md) |
| **Investigate** — answer one named checkable uncertainty through research, diagnosis, or a prototype | [references/investigate.md](references/investigate.md) |
| **Create tasks** — draft an approved task split | Read [references/to-tasks.md](references/to-tasks.md) and the authoritative [references/task-format.md](references/task-format.md) |
| **Implement** — continue with the next ready task | [references/implement.md](references/implement.md) |
| **Verify** — independently review an implementation | [references/verify.md](references/verify.md) |

Use `zdev next --any --format json` only when the user explicitly asks for any
ready or unblocked task across areas. A generic request to continue, select the
next task, or work without naming an area keeps the ordinary area-specific
selection rules.

Read every selected reference completely before starting its interaction. Run
the interactions the user requested, in their requested order. After the last
one, report the result and wait. If an approved artifact changes, show the
revision and ask for approval again. Ask which interaction comes first only
when the requested order is unclear.

## Development model

An area moves from a brief to approved tasks, implementation, independent
verification, completion, and commit. **Explore** and **Discuss** shape the
brief, including scope and testing. **Create tasks** turns that brief into an
exact bundle for approval. **Implement** selects one ready task, records the Git
baseline, and changes only task-owned paths. A fresh verifier checks the task
requirements, touched code, and required validation. The coordinating agent
completes and commits the task after `PASS`.

Larger areas may organize several related increments as slice briefs under
`.zdev/<area>/slices/`. A slice records only a title, objective, and boundaries;
it has no status or required task membership. The area brief remains
authoritative for shared decisions and testing.

The brief and selected task define the outcome, boundaries, testing level, and
done conditions throughout this process.

Use `general` as the conventional tag for recurring one-off work when the user
wants one standing area instead of a new area for each small improvement. It is
an ordinary area on an ordinary persistent branch, with a minimal brief that
keeps shared boundaries, testing, and validation. Unsliced tasks are normal;
use slice briefs only when several tasks share one narrower objective.

When discussion leaves no unresolved product or testing choice, an explicit
request may proceed directly to **Create tasks** and exact task-bundle review.
This shorter planning path still requires concrete outcomes, boundaries, done
proof, approval, branch safety, proportionate testing, independent
verification, and committed accepted work.

1. Confirm `zdev` is available.
2. Choose the direct interaction before creating state. When the repository has
   no `.zdev` directory, run standalone **Improve** and **Investigate** without
   initialization, ownership questions, or integration setup. If the user later
   wants to preserve findings as zdev work, offer **Explore an objective**.
3. When `.zdev` is absent and the user wants new durable work, read
   [references/setup.md](references/setup.md) completely before initialization.
4. Run `zdev status [<area>] --format json` for status or orientation.
   If several areas have open work and none is selected, present their tags and
   ask the user to choose. Do not infer an area from unrelated chat history.
5. For **Explore**, **Discuss**, **Improve**, **Investigate**, or **Create
   tasks**, report a selected area's branch and base diagnostics. Require the
   recorded branch before changing area state, but do not run `zdev area rebase`
   without explicit consent. Read-only interactions never rebase.
6. Before **Implement**, **Verify**, completion, or commit, read
   [references/implement.md](references/implement.md) and
   [references/verify.md](references/verify.md) completely. They define the
   required area gates, Git baseline, ownership checks, rework loop, validation,
   staging, and commit sequence. Read
   [references/recovery.md](references/recovery.md) when a gate fails, Git is
   rebasing, or task ownership must be reconstructed after interruption.

For ordinary task work, use `branch_status.task_work.safe` as the branch gate.
Report a stale-but-safe rebase advisory once and continue without requesting a
rebase. Unsafe branch, anchor, ancestry, history, or Git-operation state still
stops implementation, verification, completion, and commit preparation.

Keep existing Git changes in place. Establish ownership before touching an
overlapping path or changing the index.

## Deterministic task context

For work in a named area, run `zdev goal <area> --format json` first. An open
`empty` or `exhausted` queue, or a `closed` lifecycle, means there is no
executable task; report it without starting a worker or native goal. For an
open `ready` queue, use `zdev goal <area>` as
ordinary prompt context unless the user explicitly asks to apply a continuing
native goal. Do not reproduce the goal renderer in the harness.

On a harness with native goals, inspect the current native goal before applying
one. An active, paused, budget-limited, or otherwise unfinished native goal
wins. Do not edit, clear, replace, or layer task work over it; report the
conflict and ask the user whether to keep or explicitly replace it. When native
mode was explicitly requested and no unfinished goal exists, apply the exact
`native_goal` value. If the feature is absent, disabled, or unavailable, use
the ordinary prompt instead and say that no native continuation was started.

A native goal never completes a zdev task or commits. Goal command failure also
leaves the session goal unchanged.

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

Store only metadata, `brief.md`, optional slice briefs, task files, and generated
`TASKS.md` under `.zdev`. Keep transcripts and review evidence in the
conversation. Existing domain documentation and ADRs remain authoritative
across areas. Report what changed, what verification passed, and what remains;
mention commands only when they help the user continue or recover.

## OpenCode orchestration

Delegate one selected task to `@zdev-implementer`, then ask a different
`@zdev-verifier` to check the task requirements, touched code, and validation.
Return each task-owned `REWORK` finding to an implementer and verify the
correction with a different agent. Continue until `PASS` or `BLOCKER`. Include
the rendered repository guidance and applicable instructions in every prompt.

Use `/zdev-implement` for one complete task cycle, `/zdev-verify` for explicit
read-only task verification, and `/zdev-audit` for a read-only audit.

OpenCode has no required native goal surface. Use the rendered zdev goal as an
ordinary prompt, including when a native feature was requested but is
unavailable, and state that no native continuation was started.

<!-- zdev:generated-repository-guidance:start -->
## Repository guidance discovery

Before planning or changing code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.
<!-- zdev:generated-repository-guidance:end -->

The [task format](references/task-format.md) defines imported task bundles.

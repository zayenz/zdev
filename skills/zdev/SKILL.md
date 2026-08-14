---
name: zdev
description: "Shape, discuss, audit, investigate, task, implement, verify, and commit durable work with zdev. Use only when the user invokes zdev, zd, or $zdev; asks to work through an existing .zd area; or unmistakably refers to zdev's stored areas or tasks."
---

# Zdev for Codex

## Shared zdev contract

Zdev stores durable planning and task state. The active harness supplies
orchestration and judgment.

## Activate zdev, then route intent

Activate this workflow only when the user invokes `zdev`, `zd`, or `$zdev`,
asks to work through an existing `.zd` area, or unmistakably refers to zdev's
stored areas or tasks. Words such as “audit,” “explore,” “discuss,” “plan,”
“implement,” and “verify” do not activate zdev on their own. Once zdev is
active, use those ordinary intent words to choose one direct interaction or an
explicitly ordered sequence of interactions:

| Active zdev intent | Direct reference |
| --- | --- |
| **Explore an objective** — start or revise an area and its brief; aliases: “wayfind,” “shape” | [references/shape-work.md](references/shape-work.md) |
| **Discuss the brief** — challenge or sharpen an existing brief; alias: “grill” | [references/discuss.md](references/discuss.md) |
| **Improve** — broadly audit or review and propose candidate work | [references/improve.md](references/improve.md) |
| **Investigate** — answer one named checkable uncertainty through research, diagnosis, or a prototype | [references/investigate.md](references/investigate.md) |
| **Create tasks** — draft an approved task split | Read [references/to-tasks.md](references/to-tasks.md) and the authoritative [references/task-format.md](references/task-format.md) |
| **Implement** — continue with the next ready task | [references/implement.md](references/implement.md) |
| **Verify** — independently review an implementation | [references/verify.md](references/verify.md) |

Read each selected reference completely before starting that interaction.
Continue with another zdev interaction only when the user already requested
it. Otherwise, report the result, offer relevant next actions, and stop. For
example, “approve this exact bundle and then implement” authorizes task import
followed by **Implement**. Approval applies only to the artifact shown; if that
artifact changes, show the revision and obtain fresh approval. Investigation
does not authorize a fix, an audit does not authorize tasks, and an approved
brief does not authorize a task split or execution. Never invoke a separate
skill through an implied transition.
When the user requests both broad candidate discovery and one named
uncertainty, follow their requested order. If the order is unclear, ask which
interaction to run first.

1. Confirm `zd` is available.
2. Choose the direct interaction before creating state. When the repository has
   no `.zd` directory, run standalone **Improve** and **Investigate** without
   initialization, ownership questions, or integration setup. If the user later
   wants to preserve findings as zdev work, offer **Explore an objective**.
3. If the repository has no `.zd` directory and the user wants new durable
   zdev work,
   ask whether zdev planning should be a **personal**, **project**, or
   **pull-request** record. Recommend **personal** when the work is for this
   user and clone only; it stays local but does not travel to another clone or
   collaborator. Recommend **project** when the planning record should remain
   portable, reviewable, and collaborative after merge. Recommend
   **pull-request** when `.zd` should be committed for review on the branch but
   omitted from the squash-merged result. Wait for the user's choice; do not
   infer it or run `zd init` first.
4. Apply that choice before initialization. For **personal**, add the exact
   entry `/.zd/` to this clone's `.git/info/exclude`. For **project**, leave
   `.zd` visible to Git and treat its files as lasting project state to review
   and commit. For **pull-request**, also leave `.zd` visible to Git and commit
   it on the pull-request branch, explicitly note that it must not reach the
   squash-merged tree, and run `zd cleanup squash` before squash merge. If
   `.zd` already exists, skip this question and preserve the repository's
   current treatment of it.
5. Record ownership is separate from harness integration scope. For a new
   zdev repository, run `zd skill check <harness> --scope user` for every
   requested harness, including the active harness and any additional Codex,
   Claude Code, OpenCode, Pi, or Oh My Pi (`omp`) harness. If a check reports status
   `ok`, reuse that user integration; do not ask the user to reinstall it or
   choose its scope. Discuss installation or ask about scope only when a check
   reports `missing` or `conflict`, or when the user explicitly
   requests a checked-in project integration. For project scope, also ask
   whether guidance comes from `auto`, `agents`, `zdev`, or a
   repository-relative Markdown path. The user may explicitly skip
   installation.
6. Run `zd init --record <personal|project|pull-request>` only after the record
   choice and integration checks. Then run the corresponding `zd skill
   install` commands for integrations that need installation.
7. Run `zd status [<area>] --format json` for status or orientation.
   If several areas have open work and none is selected, present their tags and
   ask the user to choose. Do not infer an area from unrelated chat history.
8. For **Explore**, **Discuss**, **Improve**, **Investigate**, or **Create
   tasks**, report a selected area's branch and base diagnostics. Require the
   recorded branch before changing area state, but do not run `zd area rebase`
   without explicit consent. Read-only interactions never rebase.
9. For **Implement** and **Verify**, and before completion or commit, require
   all four area gates: the recorded branch is checked out, the effective-base
   link is fresh, its anchor is valid, and base finalization is complete. Bind
   missing trunk or area metadata explicitly.
10. Read the area's `brief.md`, including its required testing level. For
   execution, run
   `zd next [<area>] --format json` and read the selected task file.
11. Record the three-part Git baseline before delegation: status including
   untracked files, the cached diff, and the unstaged diff. Give the implementer
   that baseline with the brief, task, relevant source, repository guidance,
   and task-owned paths. Implementers may edit source and tests and run
   validation. They must not edit `.zd`, change task lifecycle state, or commit.
   Preserve user-owned state and stop when overlap is ambiguous; never stash,
   reset, restore, clean, or rearrange the index automatically.
12. While a task is active, ignore an intervening commit only when its complete
   diff adds one or more new `.zd/<area>/tasks/*.md` files, regenerates
   `.zd/<area>/TASKS.md`, and changes no other path. Keep the current selection
   and consider those additions at the next `zd next`. Stop and review every
   other intervening change.
13. Use a fresh read-only verifier for separate Spec and Standards passes over
   complete checkout evidence. Compare the three-part Git state before and
   after validation and do not discard files written by checks. Testing follows
   the brief's agreed level and repository patterns. If required validation is
   unsafe or unavailable, return `BLOCKER`; report limitations only for optional
   checks.
14. Use `PASS` only when both passes succeed, `REWORK` for concrete task-owned
   defects or task-owned validation writes, and `BLOCKER` for ambiguous
   ownership, unavailable required evidence or validation, or user-owned
   decisions. Send every `REWORK` through implementation and then a fresh
   verification. Repeat without a fixed retry limit; stop only for `PASS` or a
   real `BLOCKER`.
15. Only the caller may run `zd task done` and `zd commit`, after a fresh
   verification passes.

For a gated execution route, use `zd area rebase <area>` for a stale or
unfinalized base link. After interruptions, inspect Git, `zd status`, and the
task file and re-establish change ownership before resuming; do not assume an
existing diff belongs to the task. Complete work by staging explicit task-owned
source paths, the exact task file, and generated `TASKS.md`, then inspect the
full cached diff before committing. Do not create transcripts, run records,
prompt packets, or copied diffs in `.zd`.

The area brief and task remain authoritative for scope and done conditions.

## Keep state lean

Persist only project and area metadata, `brief.md`, task files, generated
`TASKS.md`, and accepted Git changes. Do not create plan directories, audit
reports, run records, packet files, evidence ledgers, claim tokens, execution
bindings, or manual hashes. Pass approved task bundles directly to
`zd tasks import <area> --from -`.

Existing project-wide domain documentation and ADRs remain authoritative for
cross-area knowledge.

## Finish in project terms

Report what changed, what verification passed, and what remains. Mention `zd`
commands only when they help the user continue or recover.

## Codex orchestration

Use Codex collaboration agents for bounded implementation and fresh
verification contexts. Keep the main Codex conversation responsible for zdev
state, user decisions, task completion, and commits. Give every delegated
agent the rendered repository guidance and any narrower applicable
`AGENTS.md` instructions.

For longer work, use a Codex goal only when the user explicitly requests one.

<!-- zdev:generated-repository-guidance:start -->
## Repository guidance discovery

Before planning or changing code, read applicable repository and directory-specific `AGENTS.md` files, `.zd/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.
<!-- zdev:generated-repository-guidance:end -->

The [task format](references/task-format.md) defines imported task bundles.

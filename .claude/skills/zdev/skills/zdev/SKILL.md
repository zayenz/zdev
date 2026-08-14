---
name: zdev
description: "Shape, discuss, audit, investigate, task, implement, verify, and commit durable work with zdev. Use only when the user invokes zdev, zd, or $zdev; asks to work through an existing .zd area; or unmistakably refers to zdev's stored areas or tasks."
---

# Zdev for Claude Code

## Shared zdev contract

Zdev stores briefs and tasks under `.zd`. The active harness reads that state,
asks for decisions, changes code, and verifies the result.

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
requirements, touched code, and required validation. The caller completes and
commits the task after `PASS`.

The brief and selected task define the outcome, boundaries, testing level, and
done conditions throughout this process.

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
9. Before **Implement**, **Verify**, completion, or commit, read
   [references/implement.md](references/implement.md) and
   [references/verify.md](references/verify.md) completely. They define the
   required area gates, Git baseline, ownership checks, rework loop, validation,
   rebase recovery, staging, and commit sequence.

Keep existing Git changes in place. Establish ownership before touching an
overlapping path or changing the index.

## Keep state lean

Store planning state as project and area metadata, `brief.md`, task files, and
generated `TASKS.md`. Keep transcripts, review evidence, and prompt packets in
the conversation. Pass approved task bundles directly to
`zd tasks import <area> --from -` and commit accepted source changes normally.

Existing project-wide domain documentation and ADRs remain authoritative for
cross-area knowledge.

## Finish in project terms

Report what changed, what verification passed, and what remains. Mention `zd`
commands only when they help the user continue or recover.

## Claude Code orchestration

For an ordinary task, delegate source changes to `zdev:zdev-implementer`, then
ask a different `zdev:zdev-verifier` to check the task requirements, touched
code, and validation. Return each task-owned `REWORK` finding to an implementer
and verify the correction with a different agent. Continue until `PASS` or
`BLOCKER`. Include the rendered repository guidance in every prompt. If the
named agents are unavailable, use ordinary Claude Code subagents with the same
boundaries.

On Claude Code v2.1.154 or later, `/zdev:zdev-task` runs this task cycle and
`/zdev:zdev-audit` runs a read-only audit. The ordinary subagent loop also
works. The main conversation runs `zd task done` and `zd commit`.

<!-- zdev:generated-repository-guidance:start -->
## Repository guidance discovery

Before planning or changing code, read applicable repository and directory-specific `AGENTS.md` files, `.zd/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.
<!-- zdev:generated-repository-guidance:end -->

The [task format](references/task-format.md) defines imported task bundles.

---
name: zdev
description: "Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use only when the user invokes zdev, zd, or $zdev; asks to work through an existing .zd area; or unmistakably refers to zdev's stored areas or tasks."
---

# Zdev for Codex

## Activate zdev, then route intent

Activate this workflow only for `zdev`, `zd`, `$zdev`, an existing `.zd` area,
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

The brief and selected task define the outcome, boundaries, testing level, and
done conditions throughout this process.

1. Confirm `zd` is available.
2. Choose the direct interaction before creating state. When the repository has
   no `.zd` directory, run standalone **Improve** and **Investigate** without
   initialization, ownership questions, or integration setup. If the user later
   wants to preserve findings as zdev work, offer **Explore an objective**.
3. When `.zd` is absent and the user wants new durable work, read
   [references/setup.md](references/setup.md) completely before initialization.
4. Run `zd status [<area>] --format json` for status or orientation.
   If several areas have open work and none is selected, present their tags and
   ask the user to choose. Do not infer an area from unrelated chat history.
5. For **Explore**, **Discuss**, **Improve**, **Investigate**, or **Create
   tasks**, report a selected area's branch and base diagnostics. Require the
   recorded branch before changing area state, but do not run `zd area rebase`
   without explicit consent. Read-only interactions never rebase.
6. Before **Implement**, **Verify**, completion, or commit, read
   [references/implement.md](references/implement.md) and
   [references/verify.md](references/verify.md) completely. They define the
   required area gates, Git baseline, ownership checks, rework loop, validation,
   staging, and commit sequence. Read
   [references/recovery.md](references/recovery.md) when a gate fails, Git is
   rebasing, or task ownership must be reconstructed after interruption.

Keep existing Git changes in place. Establish ownership before touching an
overlapping path or changing the index.

## State and reporting

Store only metadata, `brief.md`, task files, and generated `TASKS.md` under
`.zd`. Keep transcripts and review evidence in the conversation. Existing
domain documentation and ADRs remain authoritative across areas. Report what
changed, what verification passed, and what remains; mention commands only
when they help the user continue or recover.

## Codex orchestration

Use one Codex collaboration agent to implement a task and a different agent to
verify it. The coordinating agent owns zdev state, user decisions, task
completion, and commits. Give each agent the rendered repository guidance and
applicable `AGENTS.md` instructions.

For longer work, use a Codex goal only when the user explicitly requests one.

<!-- zdev:generated-repository-guidance:start -->
## Repository guidance discovery

Before planning or changing code, read applicable repository and directory-specific `AGENTS.md` files, `.zd/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.
<!-- zdev:generated-repository-guidance:end -->

The [task format](references/task-format.md) defines imported task bundles.

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

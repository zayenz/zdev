# Implement the next task

## When

Use this method for each task returned by `zdev next`. Use `zdev next --any`
only when the user explicitly asks for any ready or unblocked task across
areas; do not infer project-wide selection from an omitted area.

## Do

1. Run `zdev status <area> --format json` and inspect `branch_status.task_work`.
   Require `safe` to be true. When `stale_advisory` is true, report the single
   rebase advisory once and continue; do not ask for rebase consent. A stale
   link remains safe only while the recorded branch, anchor, ancestry, and
   linear history are valid and no Git recovery operation is active. If `safe`
   is false, follow [recovery.md](recovery.md) before selecting work. Recommend
   the explicit managed rebase when the task needs newer base changes or is
   approaching an integration boundary.
2. Read the area brief first. Inspect the selected task's routing frontmatter;
   when it names a slice, read that slice brief next, then read the complete
   task file, repository instructions, and the smallest relevant source and
   tests. When the brief indexes an area-local
   `background/` corpus, follow only links identified as relevant to the task,
   including any focused links in the task file; do not load the entire corpus
   by default. Treat `brief.md` as the authoritative synthesis when background
   detail differs from it. A slice supplies narrower objective and boundary
   context but does not override the area brief's decisions or testing. Confirm
   that the brief has a concrete `Testing`
   section that covers this task. If it is missing, return control and
   recommend **Discuss the brief** or **Explore an objective**. If repository
   evidence and the task do not make the intended level clear, ask the user
   with a recommended level and its cost/confidence trade-off before
   delegating implementation.
3. Before delegation, record the checkout baseline in the conversation:
   `git status --short --untracked-files=all`, `git diff --cached`, and
   `git diff`. This covers staged, unstaged, and untracked state without
   changing it. Identify which existing changes are user-owned and which paths
   this task may change. If an existing change overlaps a task path and
   ownership is unclear, stop and ask the user; do not stash, reset, restore,
   clean, or alter the index to manufacture a clean baseline.
4. Dispatch one implementer with the brief, task, repository
   guidance, relevant source, and task-owned path boundaries from the baseline.
   It may edit source and tests and run validation. It must not edit `.zdev`, run
   `zdev task done`, change task lifecycle state, or commit.
5. Ask it to satisfy every done condition, stay within the task boundaries, and
   follow the brief's testing level and run the listed validation. Reuse nearby
   test style, seams, fixtures, and helpers. Add or change tests only to the
   extent called for by the agreed level and task; do not invent a new testing
   approach or broaden coverage for robustness beyond scope. When new tests are
   called for and the repository uses test-first development, a focused failing
   behavior test through a stable public seam is appropriate. When the brief
   says no new tests, do not add them merely to demonstrate diligence.
6. Compare Git status, staged and unstaged diffs, and untracked paths with the
   recorded baseline. Attribute every new change to the task before treating it
   as implementation evidence. Stop on ambiguous overlap or unexplained state;
   pre-existing changes remain user-owned even when they are adjacent to the
   task. Ignore an intervening commit only when its complete diff adds one or more new
   `.zdev/<area>/tasks/*.md` files, regenerates `.zdev/<area>/TASKS.md`, and changes
   no other path. Keep the current selection and consider those additions only
   at the next `zdev next` boundary. Stop and review every other intervening
   change, including changes to an existing task, the selected task, `brief.md`,
   area metadata, lifecycle state, or source. Then apply the independent
   verification contract below with a fresh verifier.
7. Return every concrete task-owned verifier `rework` finding to the same implementer
   when possible.
   Otherwise give a replacement implementer the task, current diff, and exact
   findings. After correction, use a fresh verifier for both passes again.
8. Repeat implementation and fresh verification without a fixed retry count.
   Stop only for verifier `pass`, a real blocker, unsafe scope expansion, or a
   user-owned decision.

For verification, give a different agent the brief, task, actual checkout diff,
relevant source and tests, and repository verification instructions. It checks
every task requirement, inspects the touched code, runs task-listed validation,
and compares Git state before and after validation. The verifier returns the
strict typed object defined in [verify.md](verify.md). The
coordinating agent checks that the verdict covers the whole task.

## Finish

After verification passes:

```text
zdev task done <area> <task> --summary <summary> --validation <result>...
git add <explicit-task-source-path>... .zdev/<area>/tasks/<exact-task-file> .zdev/<area>/TASKS.md
git diff --cached
zdev commit -m <message>
zdev next <area> --format json
```

Stage only explicit task-owned source paths, the exact completed task file, and
the generated `TASKS.md`; never stage the whole area directory. Before commit,
inspect `git status --short --untracked-files=all` and the full cached diff
against the baseline and verified evidence. The index must contain only the
intended task changes. Stop if pre-existing staged content, unexplained changes,
or ambiguous ownership would enter the commit. Do not rearrange the user's
index automatically. `zdev commit` adds the stable change ID. Return control with
the next ready task; continue only when the user's existing zdev execution
request authorizes the loop.

After interruption, read [recovery.md](recovery.md) before resuming or assigning
change ownership.

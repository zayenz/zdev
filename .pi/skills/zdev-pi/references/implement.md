# Implement the next task

## When

Use this method for each task returned by `zdev next`. Use `zdev next --any`
only when the user explicitly asks for any ready or unblocked task across
areas; do not infer project-wide selection from an omitted area.

## Do

1. Run `zdev work-context <area> --format json`. The command validates goal
   lifecycle first. A closed result is branch-independent no-work. For open
   work, require its nested status and goal to agree, task work to be safe, and
   retain its exact staged, unstaged, and untracked Git evidence. When
   `stale_advisory` is true, report the single rebase advisory once and
   continue; do not ask for rebase consent. If the command reports unsafe
   state, return to the root **Recover** route. Recommend the explicit managed
   rebase when the task needs newer base changes or is approaching an
   integration boundary.
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
3. Use the work-context Git strings as the checkout baseline. They cover
   staged, unstaged, and untracked state without changing it. Inspect relevant
   untracked files, identify which existing changes are user-owned and which
   paths this task may change. If an existing change overlaps a task path and
   ownership is unclear, stop and ask the user; do not stash, reset, restore,
   clean, or alter the index to manufacture a clean baseline.
4. Read effective complexity from the selected goal. Dispatch
   `routine-implementer` only for authored `routine`, `implementer` for
   `standard` or omitted legacy complexity, and `advanced-implementer` for
   `advanced`. Before the first advanced edit, obtain one strict plan from a
   fresh read-only planner using the advanced profile and pass it unchanged to
   the advanced implementer. A planner blocker or user-owned decision stops
   before edits. Never infer routine, and never plan again on resume or rework.
   Give the selected implementer the brief, task, repository
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
6. Rerun work-context and compare its area, ready task, safety, and exact Git
   strings with the recorded baseline. Attribute every new change to the task before treating it
   as implementation evidence. Stop on ambiguous overlap or unexplained state;
   pre-existing changes remain user-owned even when they are adjacent to the
   task. Ignore an intervening commit only when its complete diff adds one or more new
   `.zdev/<area>/tasks/*.md` files, regenerates `.zdev/<area>/TASKS.md`, and changes
   no other path. Keep the current selection and consider those additions only
   at the next `zdev next` boundary. Stop and review every other intervening
   change, including changes to an existing task, the selected task, `brief.md`,
   area metadata, lifecycle state, or source. Then apply the independent
   verification contract below with a fresh verifier.
7. Return every concrete task-owned verifier `rework` finding with escalation
   `none` to the same selected profile when possible. Otherwise give a
   same-profile replacement the task, current diff, and exact findings. A
   verifier may request `advanced-implementer` once only after standard/default
   implementation; use an advanced replacement without replanning. Reject a
   second or inapplicable escalation. After every correction, use a fresh
   standard verifier.
8. Repeat implementation and fresh verification without a fixed retry count.
   Stop only for verifier `pass`, a real blocker, unsafe scope expansion, or a
   user-owned decision.

For verification, give a different agent the brief, task, actual checkout diff,
relevant source and tests, and repository verification instructions. It checks
every task requirement, inspects the touched code, runs task-listed validation,
and compares Git state before and after validation. The verifier returns the
strict typed object defined in the **Verify** reference loaded for this route. The
coordinating agent checks that the verdict covers the whole task.

## Finish

After verification passes:

```text
zdev task done <area> <task> --summary <summary> --validation <result>...
git add <explicit-task-source-path>... .zdev/<area>/tasks/<exact-task-file> .zdev/<area>/TASKS.md
git diff --cached
zdev commit -m <message>
```

Stage only explicit task-owned source paths, the exact completed task file, and
the generated `TASKS.md`; never stage the whole area directory. Before commit,
inspect `git status --short --untracked-files=all` and the full cached diff
against the baseline and verified evidence. The index must contain only the
intended task changes. Stop if pre-existing staged content, unexplained changes,
or ambiguous ownership would enter the commit. Do not rearrange the user's
index automatically. `zdev commit` adds the stable change ID. Return control
with the verified commit and stop without querying the next task. If the user
explicitly asks to continue, or an authorized goal or loop is active, begin a
new iteration with a fresh `zdev work-context <area> --format json` before
dispatching another worker. Never reuse the pre-commit selection.

After interruption, return to the root **Recover** route before resuming or
assigning change ownership.

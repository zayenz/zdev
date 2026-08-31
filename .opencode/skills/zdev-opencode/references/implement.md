# Implement the next task

## When

Use this method for each task returned by `zdev next`. Use `zdev next --any`
only when the user explicitly asks for any ready or unblocked task across
areas; do not infer project-wide selection from an omitted area.

## Do

1. Run `zdev work-context <area> --store --format json`, validate the compact
   result, and use `--show <snapshot>` in coordination when complete context is
   needed. Pass workers the locator instead of relaying the full payload. The command validates goal
   lifecycle first. A closed result is branch-independent no-work. For open
   work, require its nested status and goal to agree, task work to be safe, and
   retain its exact staged, unstaged, and untracked Git evidence. When
   `stale_advisory` is true for an isolated area, report the single rebase
   advisory once and continue; do not ask for rebase consent. Trunk mode never
   needs freshness or rebase ceremony. If the command reports unsafe
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
4. Read effective complexity from the selected task in work-context. Dispatch
   `routine-implementer` only for authored `routine`, `implementer` for
   `standard` or omitted legacy complexity, and `advanced-implementer` for
   `advanced`. Before the first advanced edit, obtain one strict plan from a
   fresh read-only planner using the advanced profile. Extract one unambiguous
   balanced JSON object from its response, tolerating brief prose or a Markdown
   fence, then validate its four semantic fields and reconstruct the compatible public
   nine-key planner envelope, and pass the semantic plan object unchanged to
   the advanced implementer. A planner blocker or user-owned decision stops
   before edits. Never infer routine, and never plan again on resume or rework.
   Give the selected implementer repository-relative locators for the brief,
   optional slice and linked background files, task, repository guidance, and
   relevant source and tests, plus the baseline snapshot and task-owned path
   evidence. Named and planned paths are expected seams, not an allowlist,
   unless the task states an explicit file boundary. The implementer may touch
   another path when it is directly necessary for the approved outcome, has
   unambiguous baseline ownership, and stays inside the task and area.
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
   area metadata, lifecycle state, or source. If the implementer reported a
   blocker, classify it under the next step. Otherwise apply the independent
   verification contract below with a fresh verifier.
7. Treat an implementer blocker as terminal only for a concrete impasse: a
   user-owned decision, unavailable external state, unsafe or ambiguous
   ownership, or an actual semantic scope change. Partial progress, remaining
   directly actionable work, an underestimated file count, and newly discovered
   attributable in-scope paths continue with the same profile or a replacement
   from the current diff. Do not commit incomplete implementation merely to
   clear the checkout.
8. Return every concrete task-owned verifier `rework` finding with escalation
   `none` to the same selected profile when possible. Otherwise give a
   same-profile replacement the task, current diff, and exact findings. A
   verifier may request `advanced-implementer` once only after standard/default
   implementation; use an advanced replacement without replanning. Reject a
   second or inapplicable escalation. After every correction, use a fresh
   standard verifier.
9. Repeat implementation and fresh verification without a fixed retry count.
   Stop only for verifier `pass`, a real blocker, unsafe semantic scope expansion, or a
   user-owned decision.

If the implementer discovers that the approved task must be split, it may
return one strict transient `implementation_split` proposal instead of asking
the user to perform a separate task import. It does so inside the unchanged
typed implementer object: verdict `blocker`, escalation `none`, no findings,
and the exact proposal as its sole evidence item. Before edits,
`retained_parent_paths` is empty. After edits, it must equal the complete
unstaged parent-owned path set, while every child has exact normalized future
paths disjoint from the parent and every other child. The worker never reviews,
applies, imports, or writes `.zdev` state.

Recognize that alternative before ordinary blocker handling. Refresh
work-context and keep the proposal unchanged. When the split is necessary
direct work inside the brief and source task, with unchanged attributable
context and no product, compatibility, destructive, ownership, cross-area, or
uncertainty decision, run `zdev tasks derive apply <area> --from - --format
json` directly without approval; apply revalidates mechanical authority under
its lock. When the user must make a semantic choice, run `zdev tasks derive
review`, require `mechanically_eligible: true`, and present the stored Markdown
with `zdev tasks derive review <area> --show`. After ordinary approval, apply
the stored identity with `zdev tasks derive apply <area> --reviewed <review-id>
--format json`. Changed or unsafe state is preserved for recovery and fresh
work-context. A successful split commit leaves the source open and blocked by its
children; report that commit and stop this one-task interaction. An active
goal, loop, or explicit continuation refreshes work-context and uses the
resulting normal graph. Do not apply another proposal from the same handoff; a
later independently selected task may propose once under fresh gates.

For verification, first have coordination store and validate a work-context
snapshot for the same admitted open, ready, safe area, task, HEAD, and checkout.
Give a different agent its opaque locator, the brief, task, current task
identity, relevant source and tests, and repository verification instructions.
It shows the supplied snapshot, checks every task requirement, inspects the
touched code, runs task-listed validation, and returns only the four-field
semantic object defined in the **Verify** reference. Coordination extracts one
balanced object with the same wrapper tolerance, runs the compact comparison, refuses a changed-state
pass, and constructs the compatible nine-key verifier envelope. The
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
index automatically. Sharing trunk with other explicit areas does not broaden
the selected task's ownership. `zdev commit` adds the stable change ID. Return control
with the verified commit and stop without querying the next task. If the user
explicitly asks to continue, or an authorized goal or loop is active, begin a
new iteration with a fresh `zdev work-context <area> --format json` before
dispatching another worker. Never reuse the pre-commit selection.

After interruption, return to the root **Recover** route before resuming or
assigning change ownership.

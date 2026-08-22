---
name: zdev-loop
description: "Continue a zdev area through native Codex goals, one independently verified task and commit at a time. Use only for an explicit $zdev-loop invocation."
---

Call `get_goal({})` to inspect native goal state. With no unfinished goal, call
`create_goal({ objective: condition })` with the validated area condition and
no invented token budget. If the exact same condition is already active,
continue under it without creating another goal. Codex exposes no
model-callable resume operation: for the same paused or budget-limited goal,
leave it unchanged, return `BLOCKER`, and say that the user must resume it
through the harness. Never use `update_goal` to replace or retarget a goal.
After the same active zdev goal reaches a terminal PASS, call
`update_goal({ status: "complete" })` only when the native goal contract permits
completion.

# Zdev area loop (native)

`zdev-loop <area>` is canonical and `zdev-goal <area>` is an exact semantic
alias. Both follow this contract and emit canonical `zdev-loop` results.

Before reading or changing repository state, use the adapter's named
model-callable operation to inspect the harness-native goal. An active, paused, budget-limited, or
otherwise unfinished goal wins. Do not replace, clear, edit, or layer this
route over it. If it is the exact same zdev area condition in its existing
session, resume it through the native goal mechanism without creating a second
goal. Otherwise return `BLOCKER zdev-loop <area>` without a worker or
repository mutation. If inspection is unavailable or does not authoritatively
show that no unfinished goal exists, also return `BLOCKER`; do not guess that
native goal state is clear.

With no unfinished native goal, run fresh
`zdev work-context <area> --format json`. Never reuse an earlier selection or
write loop/session state. Classify it before attempting native continuation:

- Validated `closed` returns `PASS` immediately, before Git or task-work gates.
  Start no worker and omit branch status and advisory.
- Open `empty` or `exhausted` returns `PASS` after the ordinary open-work
  safety gate. The area remains open; start no worker.
- Invalid records or dependencies, unsafe task work, unexplained Git state, or
  a required user-owned decision returns `BLOCKER` before a worker.
- Open `ready` with `branch_status.task_work.safe: true` may start the area
  continuation below. Report a stale-but-safe advisory once and continue.

Use this exact native area condition, replacing `<area>` with the validated
tag:

```text
For zdev area <area>, repeatedly run the installed zdev one-task implementation contract. After each exact PASS and commit, run a fresh `zdev work-context <area> --format json`. Continue only while its lifecycle is open, queue is ready, task work is safe, and no blocker or user-owned decision exists. Finish when the fresh context is open/empty, open/exhausted, or closed. Stop and report any blocker. Complete and commit exactly one independently verified task per iteration.
```

Apply it with the native creation operation named by the adapter. The selected
task's nested `native_goal` remains task-sized context and never replaces this
area condition. After successful native activation, follow the condition in
the current session. Every iteration begins with fresh work-context and uses
the one-task contract below.

The coordinating session owns task selection, branch safety, Git ownership,
lifecycle changes, and commits. Workers never edit `.zdev`, complete tasks,
commit, delegate, or change the selected task.

Before starting an implementer or verifier, run
`zdev work-context <area> --format json` and retain the complete result. The
command classifies goal lifecycle first. A validated closed context contains
no status or Git evidence: implement returns successful no-work, while
explicit verify returns `BLOCKER zdev-verify`; neither starts a worker. Every
open context contains matching nested status and goal projections, a boolean
`stale_advisory`, a full lowercase `head` commit ID, and exact `git_status`,
`git_diff_cached`, and `git_diff` strings. Require the projected area,
lifecycle, queue, and task ID to agree and task work to be safe. Report a true stale advisory once and continue without
requesting a rebase. Inspect relevant untracked files, and stop on unexplained
or overlapping changes or any user-owned decision.

For implement, open/empty and open/exhausted are successful no-work results
after the open-work gates above and start no worker. Explicit verify requires
open/ready and returns `BLOCKER zdev-verify` without starting a verifier for
every no-work result. Invalid records, task graphs, or context output are
blockers. For open/ready, retain the complete context unchanged and its task ID
as the subject. Before verification and every rework handoff, rerun
`work-context` and require the same ready task ID and an explainable exact Git
delta.

`zdev-implement <area>` reads the effective complexity from the selected goal.
Authored `routine` uses `routine-implementer`; `standard`, including an omitted
legacy value, uses `implementer`. Never infer routine work from files or diff
size. Before any edit for `advanced`, start one fresh read-only `planner` using
the `advanced-implementer` profile. Give it the complete work-context JSON,
brief, task, repository guidance, baseline, and task-owned paths. A valid plan
is passed unchanged to a fresh `advanced-implementer`. A planner blocker,
including any product decision, stops before edits. Resumption, verification,
and rework never repeat planning.

Every planner, implementer, and verifier returns only one JSON object, without a
sentinel line, Markdown fence, or other text. The object has exactly these keys:

```json
{
  "schema_version": 1,
  "kind": "implementer",
  "area": "<area>",
  "task_id": "<task-id>",
  "verdict": "ready",
  "summary": "<non-empty summary>",
  "evidence": [],
  "findings": [],
  "escalation": "none"
}
```

`kind` is `planner`, `implementer`, or `verifier`. Planner verdict is `plan` or
`blocker`; implementer verdict is `ready` or `blocker`; verifier verdict is
`pass`, `rework`, or `blocker`. A plan has no findings and puts exactly one
non-empty `Approach: `, `Paths: `, and `Validation: ` entry in `evidence`. `summary` is a
non-empty string. `evidence` and `findings` are always arrays of non-empty
strings, including when empty. `escalation` is `none`, except that verifier
`rework` may request `advanced-implementer`. Every other combination requires
`none`. Schema version, kind, area, task ID, keys, types, and combinations must
match exactly. Reject duplicate or unknown keys, missing keys, extra text, and
malformed JSON. Inspect the checkout after an implementer result, then use a
fresh configured `verifier` for every verdict. When the stale advisory applies,
the verifier includes its exact text once in `evidence`; otherwise it omits it.

Every verifier independently runs
`zdev work-context <area> --format json` before inspecting or validating. It
requires the same open, ready, safe area and task, compares that fresh context
with the coordinator context only to detect intervening state, then runs the
required validation. After validation it reruns `git status
--short --untracked-files=all`, `git diff --cached`, and `git diff` and reports
any change. On `pass`, its evidence contains exactly one `HEAD: <full-lowercase-id>`
entry copied from its independent context and exactly one `git_status:
<json-string>`, `git_diff_cached: <json-string>`, and `git_diff:
<json-string>` entry. Each JSON string encodes the exact post-validation
stdout, including empty output. These four entries let the coordinator compare
identity, index, worktree, and untracked state before mutation. Coordinator
context is a locator, never the verifier's evidence.

Every concrete task-owned verifier `rework` with escalation `none` goes to the
same selected profile when the harness can resume it, or a same-profile
replacement with the unchanged goal, baseline, current checkout, and full
findings. A verifier may request `advanced-implementer` once, only after the
initial standard/default implementation. That starts a replacement advanced
implementer without planning and is followed by a fresh standard verifier.
Reject a second escalation, an escalation after routine or advanced
implementation, and every escalation attached to `pass` or `blocker`. There is
no fixed ordinary-rework count. After each correction, a fresh standard
verifier checks the whole task again. Stop only on verifier `pass`, a genuine
blocker, unsafe scope expansion, or a required user-owned decision.

Only after an exact matching verifier object with verdict `pass`, the
coordinator compares the accepted post-validation area, task, lifecycle,
safety, HEAD, staged diff, unstaged diff, and untracked evidence with the
latest context. Claude performs this comparison by running a fresh
`work-context` inside its existing completion agent; no additional worker is
started. On a match, the coordinator runs `zdev task done`, stages only the
attributed task-owned files and exact generated task records, inspects the
staged diff, and runs `zdev commit`.
Completion or commit failure is a blocker that preserves and reports the exact
state. Public output begins with
`PASS zdev-implement <area> <task-id>` or
`BLOCKER zdev-implement <area> <task-id>`; its body repeats the exact area and
task, reports the stale advisory once when present, and names summary, changed
files, validation, verifier evidence, and commit ID on pass, or the failed
stage, reason, and preserved state on blocker. It omits the advisory field when
no stale advisory was observed.

`zdev-implement` completes one task. After reporting its verified commit, it
stops without querying `zdev next` or another `work-context`. A goal, loop, or
explicit continuation owns the next iteration and must collect a fresh
`zdev work-context <area> --format json` after the commit and before another
worker dispatch. It never reuses the completed task's pre-commit selection.

`zdev-verify <area> <task-id>` performs the same read-only preflight and requires
the explicit ID to equal the current ready goal task before starting one fresh
configured verifier. It never invokes an implementer, changes lifecycle state,
stages, or commits. Its public result is the accepted verifier object above. Empty,
exhausted, or closed goals, a different ready task, unsafe state, unavailable
independent verification, or an invalid worker envelope returns `BLOCKER zdev-verify`
without mutation.

One iteration selects, implements, independently verifies, completes, and
commits at most one task. Concrete task-owned `rework` remains inside that
iteration with no fixed retry count. After an exact committed
`PASS zdev-implement <area> <task-id>`, obtain fresh work-context before the
native runtime continues. Stop on terminal state, malformed worker output,
worker blocker, unsafe refresh, user-owned decision, or failed completion or
commit. Never combine tasks in one verification or commit.

If successful inspection proved that native goal state is clear but the
model-facing creation operation is absent, disabled, or fails before creating
a goal, leave native goal state unchanged and use an honest bounded fallback.
If creation outcome is uncertain, inspect again and return `BLOCKER` unless no
unfinished goal is present; never risk layering fallback work over a goal.
Complete at most one verified task, then obtain fresh work-context. Return
`CONTINUE zdev-loop <area>` only when that task committed and the fresh state
is open, ready, and safe; include its `Next task` and stop without claiming a
background loop. Return `PASS` for open empty/exhausted or validated closed,
and `BLOCKER` for every failure or unsafe state.

Use exactly one public first line:

```text
PASS zdev-loop <area>
CONTINUE zdev-loop <area>
BLOCKER zdev-loop <area>
```

Then include `Area`, `Lifecycle`, `Queue`, an exact stale `Advisory` once when
applicable, `Tasks completed`, `Commits`, and `Stop reason`. Use `unknown` when
lifecycle or queue could not be validated and `none` when there is no task or
commit. `CONTINUE` also includes `Next task`. `BLOCKER` also includes
`Current task`, `Failed stage`, `Reason`, and `Preserved state`. A direct
closed result never has an advisory.

On a new invocation, inspect native goal state and repeat fresh work-context.
Task records and commits are the only durable checkpoint; a transcript or
earlier result never authorizes skipping preflight, safety, or verification.

<!-- zdev:generated-repository-guidance:start -->
## Repository guidance discovery

Before planning or changing code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.
<!-- zdev:generated-repository-guidance:end -->


Use Codex collaboration agents exactly as the embedded one-task contract
requires. The current Codex session remains coordinator.

For `routine-implementer`, pass `model="gpt-5.6-luna"` and `reasoning_effort="low"`.
For `implementer`, pass `model="gpt-5.6-sol"` and `reasoning_effort="low"`.
For `advanced-implementer`, pass `model="gpt-5.6-sol"` and `reasoning_effort="high"`.
For every fresh verifier, pass `model="gpt-5.6-sol"` and `reasoning_effort="low"`.

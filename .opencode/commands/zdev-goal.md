---
description: Continue a zdev area, completing at most one independently verified task
---

# Zdev area loop (bounded)

Use `$ARGUMENTS` as the area. `zdev-loop` is the canonical name;
`zdev-goal` is an exact alias. Both commands follow this same contract and
always emit the canonical `zdev-loop` result.

Start every invocation by running `zdev work-context <area> --format json`.
Do not reuse context from an earlier invocation or write loop/session state.
Classify the result as follows:

- `closed` returns `PASS` immediately, before Git or task-work gates. Start no
  worker and omit branch status and advisory.
- Open `empty` or `exhausted` returns `PASS` after the ordinary open-work
  safety gate. The area remains open; start no worker.
- Open `ready` with `branch_status.task_work.safe: true` runs the one-task
  contract below for exactly the selected task. Report a stale-but-safe
  advisory once and continue.
- Invalid records, missing blockers, dependency cycles, unsafe task work,
  unexplained Git state, or a required user-owned decision returns `BLOCKER`
  before a worker or further mutation.

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

Concrete task-owned `rework` remains inside that one task cycle with fresh
independent verification and no fixed retry count. A malformed worker result,
worker blocker, unsafe refresh, failed completion, or failed commit returns
`BLOCKER` and stops. Each successful invocation completes and commits at most
one selected task through exactly one independently accepted verification.

After an exact committed `PASS zdev-implement <area> <task-id>`, run one fresh
`zdev work-context <area> --format json` before deciding the public result. If
it reports open `ready` and safe task work, return `CONTINUE`, name that fresh
next task, and stop. Do not start it or claim a background loop. If it reports
open `empty`, open `exhausted`, or validated `closed`, return `PASS` and stop.
Any validation, safety, or refresh failure returns `BLOCKER`; do not count or
start another task.

Use exactly one of these first lines:

```text
PASS zdev-loop <area>
CONTINUE zdev-loop <area>
BLOCKER zdev-loop <area>
```

Then include `Area`, `Lifecycle`, `Queue`, an exact stale `Advisory` once when
applicable, `Tasks completed`, `Commits`, and `Stop reason`. Use `unknown` for
lifecycle or queue when validation failed before classification, and `none`
when there is no task or commit. `CONTINUE` also includes `Next task` with the
fresh ready task ID. `BLOCKER` also includes `Current task`, `Failed stage`,
`Reason`, and `Preserved state`. A direct closed result never has an advisory.

On restart, repeat this contract from fresh repository evidence. Task records
and commits are the only checkpoint. A previous `CONTINUE`, transcript, or
session resume is context, never authority to skip preflight or verification.

<!-- zdev:generated-repository-guidance:start -->
## Repository guidance discovery

Before planning or changing code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.
<!-- zdev:generated-repository-guidance:end -->

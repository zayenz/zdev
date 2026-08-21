---
name: zdev-verify
description: "Independently verify the explicit current ready zdev task without lifecycle changes. Use when the user invokes $zdev-verify with an area and task ID."
---

# Zdev verify for Codex

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

`zdev-implement <area>` gives the complete work-context JSON, brief, task, repository guidance,
baseline, and task-owned paths to the configured `implementer`. Every
implementer and verifier returns only one JSON object, without a sentinel line,
Markdown fence, or other text. The object has exactly these keys:

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

`kind` is `implementer` or `verifier`. Implementer verdict is `ready` or
`blocker`; verifier verdict is `pass`, `rework`, or `blocker`. `summary` is a
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

Every concrete task-owned verifier `rework` goes to the same implementer when the
harness can resume it, or a replacement implementer with the unchanged goal,
baseline, current checkout, and full findings. There is no fixed rework count.
After each correction, a fresh verifier checks the whole task again. Stop only
on verifier `pass`, a genuine blocker, unsafe scope expansion, or a required
user-owned decision. Do not silently send an `advanced-implementer` escalation
to an ordinary implementer; stop if that role is unavailable.

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

Use one fresh read-only Codex collaboration agent with the configured verifier
profile. Pass `model="gpt-5.6-sol"` and
`reasoning_effort="high"` when spawning it.
The current Codex session performs preflight, checks the explicit task ID, and
validates the returned envelope without changing task or Git state.

<!-- zdev:generated-repository-guidance:start -->
## Repository guidance discovery

Before planning or changing code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.
<!-- zdev:generated-repository-guidance:end -->

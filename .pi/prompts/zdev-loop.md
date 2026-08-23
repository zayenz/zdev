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

An isolated area uses its stored branch and managed base relationship. An
explicit trunk area dynamically uses configured `project.trunk`, may share it
with other explicit trunk areas, and never needs a rebase or freshness step.
In both modes, `task_work.safe` and the exact selected area/task govern work;
sharing trunk never grants ownership of another area's or unrelated paths.

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

An implementer that cannot finish the source without splitting direct,
already-approved work may use one narrow exception to the ordinary blocker
path. It returns a valid implementer object with verdict `blocker`, escalation
`none`, no findings, and one evidence item containing the complete transient
proposal. That evidence string begins
`PROPOSE zdev-derived <area> <source-task-id>\n` and continues with exactly one
JSON object. It proposes one through five ordinary TaskDraft children and no
nested proposal. A pre-edit split has an empty `retained_parent_paths`; a
post-edit split names the exact complete unstaged parent-owned path set and
assigns every child exact, normalized, path-disjoint future paths. The worker
never runs derive review, apply, import, or any other `.zdev` mutation.

The coordinator recognizes this strict alternative before treating the worker
result as an ordinary blocker. It refreshes work-context and requires unchanged
area, source task, HEAD, safety, and attributable Git state. Automatic authority
also requires every child to be necessary, direct work inside the brief and
source task, with no product, compatibility, destructive, ownership,
cross-area, or uncertainty decision. When those semantic and retained-context
checks pass, send the unchanged proposal directly to `zdev tasks derive apply
<area> --from - --format json` with no approval; apply revalidates mechanical
authority under its lock.

Only when semantic authority is unclear, and the proposal, current state, and
path ownership are otherwise safe and mechanically eligible, send the proposal
to `zdev tasks derive review <area> --from - --format json`. Require its
`mechanically_eligible` result to remain true, show the returned ordinary
bundle, ask for ordinary approval, and only after approval run apply with the
returned opaque fingerprint. Approval resolves only the semantic choice.

An invalid proposal, unsafe or changed context, staged or incomplete ownership,
or any mechanical apply failure stops without review or apply. Preserve and
report the state, follow recovery, and obtain fresh work-context before
reconsidering it; a fingerprint cannot waive those gates. Never use ordinary
task import for a derived proposal.

One successful apply consumes this uninterrupted handoff. Do not accept a
second or nested proposal from it. An investigation follow-up completes its
source and may expose ready children. A split keeps its source open and blocked
by its children; retained parent edits stay with that source. Report the
derived commit and stop the one-task interaction. A goal, loop, or explicit
continuation obtains fresh work-context before selecting from the updated
ordinary graph. A later independently selected child or resumed source may
propose once under the same current gates; no derivation count or lineage is
stored.

Every verifier independently runs
`zdev work-context <area> --store --format json` before inspecting or
validating. It accepts only the compact locator for the same open, ready task
and HEAD, then uses `zdev work-context <area> --show <snapshot> --format json`
to inspect the complete immutable pre-validation context. It requires the same
open, ready, safe area and task and compares that context with the coordinator
identity only to detect intervening state. After validation it runs
`zdev work-context <area> --compare <snapshot> --format json` and accepts only
the exact compact comparison schema for the selected area and snapshot with
`equal: true`. A false comparison is `rework` for attributable task-owned
writes and otherwise `blocker`; missing, expired, corrupt, cross-area, or
malformed snapshot evidence is `blocker`. The verifier never repairs or
discards validation writes.

On `pass`, its evidence contains exactly one
`work_context_snapshot: W<16-lowercase-hex>` entry, apart from the existing
optional stale advisory. Put checked locations and validation conclusions in
`summary`, not additional evidence items. The snapshot is resolved only by
zdev; coordinators accept the opaque ID and never a worker-supplied path. This
one immutable snapshot proves both the independently collected pre-validation
state and, through the successful comparison, the equal post-validation state.

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
coordinator gives completion the opaque snapshot ID, not the verifier object,
worker-supplied path, inline context, or raw Git evidence. Completion runs
exactly one `zdev work-context <area> --compare <snapshot> --format json`
before mutation and accepts only the exact compact schema for that area and ID
with `equal: true`. This fresh binary comparison covers area, ready task,
lifecycle, safety, HEAD, index, worktree, and untracked state because all are
part of the stored canonical context. A false comparison or an unavailable,
expired, corrupt, cross-area, or malformed artifact blocks before mutation.
On an accepted comparison, the coordinator runs `zdev task done`, stages only
the attributed task-owned files and exact generated task records, inspects the
staged diff, and runs `zdev commit`.
Completion or commit failure is a blocker that preserves and reports the exact
state. Public output begins with
`PASS zdev-implement <area> <task-id>` or
`BLOCKER zdev-implement <area> <task-id>`; its body repeats the exact area and
task, reports the stale advisory once when present, and names summary, changed
files, validation, verifier evidence, and commit ID on pass, or the failed
stage, reason, and preserved state on blocker. It omits the advisory field when
no stale advisory was observed.

An ordinary `zdev-implement` pass completes one task. A successful split uses
the derived exception above and leaves its source open. After reporting the
ordinary verified commit or derived managed commit, it stops without querying
`zdev next` or another `work-context`. A goal, loop, or explicit continuation
owns the next iteration and must collect a fresh
`zdev work-context <area> --format json` after the commit and before another
worker dispatch. It never reuses the completed task's pre-commit selection.

`zdev-verify <area> <task-id>` performs the same read-only preflight and requires
the explicit ID to equal the current ready goal task before starting one fresh
configured verifier. It never invokes an implementer, changes lifecycle state,
stages, commits, or routes a derived proposal. Its public result is the accepted verifier object above. Empty,
exhausted, or closed goals, a different ready task, unsafe state, unavailable
independent verification, or an invalid worker envelope returns `BLOCKER zdev-verify`
without mutation.

Concrete task-owned `rework` remains inside that one task cycle with fresh
independent verification and no fixed retry count. A malformed worker result,
worker blocker, unsafe refresh, failed completion, or failed commit returns
`BLOCKER` and stops. Each ordinary successful invocation completes and commits
at most one selected task through exactly one independently accepted
verification. The split exception below commits the derived graph change
without completing or claiming verification of its source.

A successful derived apply is also an iteration boundary. An investigation
follow-up completes its source; a split leaves its source open and blocked by
the new children. In either case, collect fresh work-context from the updated
ordinary graph before deciding whether to continue. Do not apply a second
proposal from the same handoff. A later independently selected task may propose
again under fresh authority checks.

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

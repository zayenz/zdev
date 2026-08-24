---
description: Implement, independently verify, complete, and commit one ready zdev task
---

The coordinating session owns task selection, branch safety, Git ownership,
lifecycle changes, staging, commits, and delegation. Workers stay within the
selected task and return one role-specific result.

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

`zdev-implement <area>` reads effective complexity from the selected task in
work-context.
Authored `routine` uses `routine-implementer`; `standard`, including an omitted
legacy value, uses `implementer`. Never infer routine work from files or diff
size. Before any edit for `advanced`, start one fresh read-only `planner` using
the `advanced-implementer` profile. Give it the complete work-context JSON,
brief, task, repository guidance, baseline, and task-owned paths. A valid plan
is passed unchanged to a fresh `advanced-implementer`. A planner blocker,
including any product decision, stops before edits. Resumption, verification,
and rework never repeat planning.

Every planner and implementer returns only one JSON object, without a sentinel
line, Markdown fence, or other text. The object has exactly these keys:

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

`kind` is `planner` or `implementer`. Planner verdict is `plan` or `blocker`;
implementer verdict is `ready` or `blocker`. A plan has no findings and puts exactly one
non-empty `Approach: `, `Paths: `, and `Validation: ` entry in `evidence`.
`summary` is a non-empty string. `evidence` and `findings` are always arrays of non-empty
strings, including when empty. `escalation` is `none`. Schema version, kind, area, task ID,
keys, types, and combinations must
match exactly. Reject duplicate or unknown keys, missing keys, extra text, and
malformed JSON. Inspect the checkout after an implementer result, then use a
fresh configured `verifier` for every verdict.

## Derived work handoff

An implementer that needs to split necessary direct work already covered by
the approved brief and task returns a valid implementer object with verdict `blocker`, escalation
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
requires every child to be necessary direct work already covered by the brief
and source task. When those semantic and retained-context checks pass, send the
unchanged proposal directly to `zdev tasks derive apply
<area> --from - --format json` with no approval; apply revalidates mechanical
authority under its lock.

When the user must make a semantic choice and current state and path ownership
are safe and mechanically eligible, send the proposal
to `zdev tasks derive review <area> --from - --format json`. Require its
`mechanically_eligible` result to remain true, present its stored Markdown with
`zdev tasks derive review <area> --show`, and ask for ordinary approval. After
approval, apply the returned opaque identity with `zdev tasks derive apply
<area> --reviewed <review-id> --format json`. Do not reconstruct or resend the
proposal. Approval resolves only the semantic choice.

An invalid proposal, unsafe or changed context, staged or incomplete ownership,
or any mechanical apply failure stops without review or apply. Preserve and
report the state, follow recovery, and obtain fresh work-context before
reconsidering it; a stored review cannot waive those gates. Never use ordinary
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

Immediately before every verifier dispatch, coordination runs
`zdev work-context <area> --store --format json`, validates its compact result,
and uses `zdev work-context <area> --show <snapshot> --format json` to require
the same open, ready, safe area, task, HEAD, and checkout as the admitted
refresh. It supplies only the opaque `W<16-lowercase-hex>` locator and expected
identity to the verifier. The verifier resolves that immutable context with
`--show`, checks the whole task, runs required validation, reports validation
writes, and never repairs or discards them.

The verifier returns only this semantic JSON object with no surrounding text:

```json
{
  "verdict": "pass",
  "summary": "<non-empty summary>",
  "findings": [],
  "escalation": "none"
}
```

It has exactly those four unique keys. `verdict` is `pass`, `rework`, or
`blocker`; `summary` is non-empty; and `findings` is an array of non-empty
strings. `pass` has no findings, `rework` has at least one, and `blocker` may
have findings. `escalation` is `none`, except that `rework` may request
`advanced-implementer`. Reject legacy nine-key verifier envelopes, duplicate
or unknown keys, missing keys, extra text, malformed JSON, and contradictory
combinations.

For each concrete task-owned file written by validation, `rework` includes one
exact `validation_write: <normalized repository-relative path>` finding. The
verifier never uses that prefix for an ordinary implementation defect. An
ambiguous validation write is `blocker`, not a tagged finding.
When any finding starts with `validation_write:`, every such finding must use
the exact valid form; a mixed valid and malformed marker set is a blocker.

After the response, coordination runs
`zdev work-context <area> --compare <snapshot> --format json` and accepts only
the exact compact schema for the selected area and snapshot. It never accepts
`pass` unless `equal` is true. A false comparison preserves `rework` only when
the semantic result contains at least one tagged task-owned validation-write
path and every marker-prefixed finding is valid;
an ordinary implementation-defect rework plus unequal state is a coordinator
blocker because the mismatch is not attributed. Missing, expired,
corrupt, cross-area, or malformed snapshot or comparison evidence is also a
blocker.

Coordination then constructs the compatible public verifier envelope with
generated `schema_version: 1`, `kind: "verifier"`, selected `area`, selected
`task_id`, and `evidence`. Evidence contains exactly
`work_context_snapshot: <snapshot>` plus the exact stale advisory once when it
applies. It copies only the validated four semantic fields into that envelope
and validates the resulting nine keys and all combinations before routing or
returning it. Put checked locations and validation conclusions in `summary`.
The opaque snapshot is never accepted from worker output.

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

After an exact matching coordinator-constructed verifier object with verdict `pass`, the coordinator
gives completion the opaque snapshot ID plus the accepted implementation and
verifier summaries. Completion derives paths from the verified checkout and runs
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
the explicit ID to equal the current ready task before starting one fresh
configured verifier. It never invokes an implementer, changes lifecycle state,
stages, commits, or routes a derived proposal. Its public result is the coordinator-constructed verifier object above. Empty,
exhausted, or closed goals, a different ready task, unsafe state, unavailable
independent verification, or an invalid worker envelope returns `BLOCKER zdev-verify`
without mutation.

Use `$ARGUMENTS` as the area. The current Oh My Pi session is the coordinator.
After preflight, select `zdev-routine-implementer`, `zdev-implementer`, or
`zdev-advanced-implementer` from effective complexity. Run the blocking
read-only `zdev-planner` once before the first advanced edit. Use a fresh
blocking `zdev-verifier` for every verification. `hub` may return ordinary
rework to the selected profile; a valid one-time standard escalation starts an
advanced replacement without replanning.
Give every agent the complete rendered contract above and a compact payload of
brief, task, guidance, and source file paths, applicable snapshot IDs, and the
short result from the preceding role.
Immediately before each verifier task, the current session stores and validates
the snapshot; after the four-field semantic response it compares that snapshot
and constructs the strict public nine-key verifier envelope.

<!-- zdev:generated-repository-guidance:start -->
## Repository guidance discovery

Before inspecting, planning, changing, or validating code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.
<!-- zdev:generated-repository-guidance:end -->

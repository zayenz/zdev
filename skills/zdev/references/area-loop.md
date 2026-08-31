# Zdev area loop (native)

`zdev-loop <area> [focus...]` is canonical and `zdev-goal <area> [focus...]`
is an exact semantic alias. Parse everything after the area as optional fuzzy
focus. Both follow this contract and emit canonical `zdev-loop` results.

Before repository work, inspect the harness-native goal with the adapter's
named model-callable operation. Native goal state selects one action:

- No unfinished goal: validate the area, then create the shared condition.
- The same condition is active: continue in that goal.
- The same condition is paused or budget-limited: preserve it and ask the user
  to resume it through the harness when model-facing resume is unavailable.
- A different goal is unfinished, or inspection is unavailable: preserve the
  existing state and return `BLOCKER zdev-loop <area>`.

With no unfinished native goal, select from fresh evidence. With no focus, run
`zdev work-context <area> --store --format json` and let the binary choose by
AFK, priority, then numeric order. With a focus, run `zdev tasks list <area>
--format json`, read every ready task with `zdev task show <area> <task-id>
--format json`, and let the coordinating model choose the best fit from the
complete ready frontier. When any ready task has `afk = true`, choose only among
those unattended tasks and use the focus to rank that eligible set. Choose an
`afk = false` attended task only when no unattended task is ready. Focus is
fuzzy guidance, not an exact filter; do not keyword-filter or pre-rank that
frontier. Admit the choice with
`zdev work-context <area> --task <task-id> --store --format json`. For an empty
frontier, run the no-task work-context form once to classify no-work. Never
reuse an earlier selection or write focus, loop, or session state. Classify it
before attempting native continuation:

- Validated `closed` returns `PASS` immediately, before Git or task-work gates.
  Start no worker and omit branch status and advisory.
- Open `empty` or `exhausted` returns `PASS` after the ordinary open-work
  safety gate. The area remains open; start no worker.
- Invalid records or dependencies, unsafe task work, unexplained Git state, or
  a required user-owned decision returns `BLOCKER` before a worker.
- Open `ready` with `branch_status.task_work.safe: true` may start the area
  continuation below. Report a stale-but-safe advisory once and continue.

Use this native area condition, replacing `<area>` with the validated tag.
Replace the bracketed clause with its inner focus sentence when the user
supplied focus; otherwise remove it:

```text
For zdev area <area>, repeatedly run the installed zdev one-task implementation contract. [Fuzzy focus: <the user's exact focus words>. Before every iteration, inspect the complete ready frontier and choose the best-fitting task; do not treat the focus as an exact filter.] With no focus, let work-context choose the next task. After each exact PASS and commit, select again from fresh evidence. Continue only while lifecycle is open, queue is ready, task work is safe, and no blocker or user-owned decision exists. Finish when fresh context is open/empty, open/exhausted, or closed. Stop and report any blocker. Complete and commit exactly one independently verified task per iteration, and report each selected and completed task through normal progress updates.
```

Apply it with the native creation operation named by the adapter. The selected
task's nested `native_goal` remains task-sized context and never replaces this
area condition. After successful native activation, follow the condition in
the current session. Every iteration repeats the applicable selection rule and
uses the one-task contract below. Tell the user which task was selected and
when its verified commit completes.

The exact installed task-workflows contract path is
"/Users/zayenz/projects/zdev/skills/zdev/references/task-workflows.md". Decode it and include the resulting
path in every worker payload.

The coordinating session owns task selection, branch safety, Git ownership,
lifecycle changes, staging, commits, and delegation. Workers stay within the
selected task and return one role-specific result.

An isolated area uses its stored branch and managed base relationship. An
explicit trunk area dynamically uses configured `project.trunk`, may share it
with other explicit trunk areas, and never needs a rebase or freshness step.
In both modes, `task_work.safe` and the exact selected area/task govern work;
sharing trunk never grants ownership of another area's or unrelated paths.

Task scope is semantic. The brief, outcome, done conditions, and explicit
boundaries define it. Source, test, and planner paths identify expected seams;
they are not a closed allowlist unless an explicit boundary says so. A worker
may add a path when it explains why the change is directly necessary, the
path's baseline ownership is unambiguous, and the change remains within the
selected task and area. Coordination confirms that attribution on refresh.
Another file, by itself, is not scope expansion. A new product or compatibility
decision, a cross-area outcome, an explicit boundary violation, destructive or
external action without authority, or overlap with user-owned work is.

Before starting an implementer or verifier, collect fresh complete work-context
through one of the admitted forms below and retain the complete result. The
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
as the subject. Every worker handoff requires fresh work-context admission and
the same ready task ID. At a worker boundary, prefer `--store` and pass the
compact snapshot locator; the coordinator may use `--show` when it needs the
complete context. The verifier's store-and-show collection satisfies fresh
pre-verifier admission without a preceding duplicate ordinary collection.
Before rework implementation, retain the ordinary refresh and require an
explainable exact Git delta.

Safe attributable state is resumable. When an interrupted selected task has an
explainable unstaged delta, continue from it. When a complete independently
verified task is waiting only for lifecycle, staging, or commit, finish that
normal completion flow before selecting again. Likewise, use an existing
zdev-managed commit step for a complete planning or repair checkpoint when the
route defines one. Never create a checkpoint commit for incomplete
implementation or mix unrelated paths merely to clear the checkout.

`zdev-implement <area>` reads effective complexity from the selected task in
work-context.
Authored `routine` uses `routine-implementer`; `standard`, including an omitted
legacy value, uses `implementer`. Never infer routine work from files or diff
size. Before any edit for `advanced`, start one fresh read-only `planner` using
the `advanced-implementer` profile. Give it repository guidance and the stored
work-context locator; it loads the brief, task, baseline, and task-owned paths
from that snapshot. The planner returns the required fields
`verdict`, `summary`, `plan`, and `findings`. A plan uses
`{"verdict":"plan","summary":"<non-empty>","plan":{"approach":"<non-empty>","paths":["<normalized path>"],"validation":["<non-empty validation step>"]},"findings":["<supporting observation>"]}`.
Its paths may be repository-relative or absolute checkout paths, and its
findings may be empty or contain supporting observations. Plan paths are the
best known implementation seams, not an exhaustive ownership boundary. A
blocker uses verdict `blocker`,
`plan: null`, and at least one non-empty finding. Reject duplicate or missing
required keys, empty values, non-normalized paths, contradictory variants,
legacy nine-key output, multiple objects, unknown keys, and malformed JSON. A
brief sentence or Markdown fence around one balanced object is tolerated.

For every role response, extract exactly one unambiguous balanced JSON object
from the returned text before semantic validation. Brief prose and Markdown
fences around that object are harmless. Multiple objects, truncation, malformed
JSON, missing, unknown, or duplicate keys, or contradictory values are real
failures. Never rerun a worker merely to remove formatting around an otherwise
valid object.

The coordinator reconstructs the compatible public nine-key planner envelope
with fixed `schema_version: 1`, `kind: "planner"`, selected area and task ID,
and `escalation: "none"`. It copies summary and findings. For a plan, evidence
is exactly `Approach: <approach>`, `Paths: <comma-joined paths>`, and
`Validation: <semicolon-joined validation steps>` in that order; for a blocker,
evidence is empty. Validate this complete public envelope before routing. Pass
the validated semantic plan object, with its approach and ordered arrays
unchanged, to a fresh `advanced-implementer`. A planner blocker,
including any product decision, stops before edits. Resumption, verification,
and rework never repeat planning.

Every implementer returns one JSON object with these required keys:

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

`kind` is `implementer`; verdict is `ready` or `blocker`.
`summary` is a non-empty string. `evidence` and `findings` are always arrays of non-empty
strings, including when empty. `escalation` is `none`. Schema version, kind, area, task ID,
required keys, types, and combinations must
match. Reject unknown, duplicate, or missing keys and malformed JSON after the
tolerant extraction above. Inspect the checkout after every implementer result.
Route a split or nonterminal partial result as defined below. Use a fresh
configured `verifier` when the worker claims readiness, and when independent
checking can resolve whether a reported blocker is genuine.

An implementer blocker is terminal only when progress requires a user-owned
decision, unavailable external state, unsafe or ambiguous ownership, an
explicit scope change, or another concrete impasse. A partial implementation,
remaining directly actionable task work, an underestimated file count, or a
new attributable in-scope path is not terminal. With safe refreshed state,
return that work to the same profile or a replacement with the current diff and
remaining requirements, then verify normally. Stop repeated attempts only when
they make no meaningful progress for a concrete reason that cannot be resolved
inside the task.

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
boundary. It supplies only the opaque `W<16-lowercase-hex>` locator and expected
identity to the verifier. The verifier resolves that immutable context with
`--show`, checks the whole task, runs required validation, reports validation
writes, and never repairs or discards them.

The verifier returns one semantic JSON object:

```json
{
  "verdict": "pass",
  "summary": "<non-empty summary>",
  "findings": [],
  "escalation": "none"
}
```

The object has exactly those four unique keys. `verdict` is `pass`, `rework`, or
`blocker`; `summary` is non-empty; and `findings` is an array of non-empty
strings. `pass` has no findings, `rework` has at least one, and `blocker` may
have findings. `escalation` is `none`, except that `rework` may request
`advanced-implementer`. Workflow parsers extract one unambiguous balanced JSON
object, tolerating a brief sentence or Markdown fence around it. Reject legacy
nine-key verifier envelopes, unknown, duplicate, or missing keys, multiple
objects, malformed JSON, and contradictory combinations.

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
blocker because the mismatch is not attributed. Missing, corrupt, cross-area,
or malformed snapshot or comparison evidence is also a
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
blocker, unsafe semantic scope expansion, or a required user-owned decision.

After an exact matching coordinator-constructed verifier object with verdict `pass`, the coordinator
gives completion the opaque snapshot ID plus the accepted implementation and
verifier summaries. Completion derives paths from the verified checkout and runs
exactly one `zdev work-context <area> --compare <snapshot> --format json`
before mutation and accepts only the exact compact schema for that area and ID
with `equal: true`. This fresh binary comparison covers area, ready task,
lifecycle, safety, HEAD, index, worktree, and untracked state because all are
part of the stored canonical context. A false comparison or an unavailable,
corrupt, cross-area, or malformed artifact blocks before mutation.
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

One ordinary iteration selects, implements, independently verifies, completes,
and commits at most one task. The split exception below commits the derived
graph change without completing or claiming verification of its source.
Concrete task-owned `rework` remains inside that
iteration with no fixed retry count. A worker result that reports only partial
progress, remaining in-scope work, or an additional attributable path continues
inside the iteration; it is not a terminal blocker. After an exact committed
`PASS zdev-implement <area> <task-id>`, obtain fresh work-context before the
native runtime continues. Stop on terminal state, malformed worker output,
genuine worker blocker, unsafe refresh, user-owned decision, or failed completion or
commit. Never combine tasks in one verification or commit.

A successful derived apply is one managed commit and an iteration boundary.
An investigation follow-up completes its source; a split leaves its source open
and blocked by its children. Refresh work-context and continue only from the
updated ordinary graph. Never apply a second proposal from the same handoff; a
later independently selected task may propose once under fresh gates.

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

Then include `Area`, optional `Focus`, `Lifecycle`, `Queue`, an exact stale
`Advisory` once when applicable, `Tasks completed`, `Commits`, and `Stop reason`. Use `unknown` when
lifecycle or queue could not be validated and `none` when there is no task or
commit. `CONTINUE` also includes `Next task`. `BLOCKER` also includes
`Current task`, `Failed stage`, `Reason`, and `Preserved state`. A direct
closed result never has an advisory.

On a new invocation, inspect native goal state and repeat fresh work-context.
Task records and commits are the only durable checkpoint; a transcript or
earlier result never authorizes skipping preflight, safety, or verification.

<!-- zdev:generated-repository-guidance:start -->
## Repository guidance discovery

Before inspecting, planning, changing, or validating code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.
<!-- zdev:generated-repository-guidance:end -->

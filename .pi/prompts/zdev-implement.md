---
description: Implement, independently verify, complete, and commit one ready zdev task
argument-hint: <area>
---

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

## Write transient instructions for the worker's path

Transient payloads include only the instructions and context needed by that
worker branch. Keep ordered actions in the route that performs them. Point to an
exact contract instead of restating it, and tell the worker when to read it.
Keep branch-only reference material beside that branch. Name the required
behavior and completion condition. Remove facts the worker can inspect cheaply
and lines that do not change what the worker does.

For example, the implementer payload always names the brief, task, repository
guidance, and work-context snapshot. It names the derived-work section only
when a split is needed, and it includes the shared prose guidance only when the
task authors human-facing text. The verifier snapshot rules remain with the
verifier handoff below instead of being repeated in implementer instructions.

When that prose branch applies, preserve meaning and tone. Name actors and
concrete actions. Use plain active language, readable sentence rhythm, and
stable repository terms. Cut puffery, vague attribution, chatbot filler,
hedging, and synonym cycling. After drafting, reread the prose and replace
generic or formulaic phrasing. Apply these edits only to human-facing prose;
preserve exact commands, paths, literals, schemas, quotations, generated
records, technical meaning, and approved content.

During an authorized active task, a worker may report that an incidental
planner-written restriction prevents necessary direct work. Coordination checks
the proposed helper or path against the agreed outcome, acceptance criteria,
explicit user constraints, compatibility promises, the original Git baseline,
and current ownership. When those checks are clear, it may update only the
active task or area brief to correct that technical restriction, explain the
adjustment, capture fresh work-context, and resume the same task while retaining
the original baseline. A changed outcome, acceptance criterion, compatibility
promise, explicit user constraint, ambiguous ownership, or material unresolved
choice still stops for the user. This exception never changes a pending reviewed
bundle, which requires renewed exact approval after any revision.

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

Before the first worker in a logical run, apply the shared **Scoped execution
profiles** contract and retain its concrete role settings beside the admitted
context. Every dispatch below uses those retained values. A fresh one-off role
request resolves only that logical step. Authored complexity selects the role;
it never selects or upgrades an execution profile.
Before rework implementation, retain the ordinary refresh and require an
explainable exact Git delta.

## Optional assigned source worktree

Ordinary one-task work stays in the admitted checkout. Use a separate source
worktree only when the user has explicitly chosen or already authorized its
creation. The authoritative checkout remains the destination: it alone owns
admission, task records, snapshots used for verification, lifecycle changes,
exact staging, and the final `zdev commit`. Do not create another scheduler,
lifecycle, or record copy for this variant.

Before dispatch, admit the exact task in the destination with `work-context
<area> --task <task-id> --store --format json`. Record the absolute destination
root, brief, task, repository-instruction, and stored-snapshot paths. Create an
ordinary Git source branch and linked worktree from the admitted HEAD, then
record its absolute root, branch, and baseline commit. A personal record may be
absent from that linked checkout; workers always read the authoritative record
paths in the destination. Give every source worker the source root as its cwd
and instruct it to edit and validate only there. It must not use source `.zdev`,
change lifecycle, complete the task, stage coordination records, or commit.

After a ready result, inspect the full source delta from its baseline, including
untracked files. Reject unrelated changes and every `.zdev` change. In the
source worktree, stage only the accepted implementation paths, inspect the
complete staged diff, and create one ordinary Git source commit as durable
transport. Refresh the destination with `work-context <area> --task <task-id>`
and require the same ready task and safe attributable destination state. Apply
the source commit there with `git cherry-pick --no-commit <source-commit>` so additions,
deletions, binary bytes, executable modes, and independent destination changes
are preserved. Do not flatten the candidate into copied file contents. Leave
conflicts in place and report both roots, branch, commit, and destination state.

On a clean integration, capture a fresh destination snapshot with the same
explicit `--task <task-id>`. Run the existing independent verification,
snapshot comparison, completion, exact staging, and one final `zdev commit`
entirely in the destination. Route rework to the assigned source cwd; each
accepted correction becomes another ordinary source commit and passes through
the same refresh and no-commit integration gate. Never mark the source task
done or publish source task records.

Keep the assigned worktree and branch when work is unfinished, integration
conflicts, verification fails, or completion is done but uncommitted. Remove
only a run-owned worktree whose worker has stopped and whose accepted commits
are integrated and included in the successful final destination commit. Never
remove a pre-existing or ambiguous worktree.

Safe attributable state is resumable. When an interrupted selected task has an
explainable unstaged delta, continue from it. When a complete independently
verified task is waiting only for lifecycle, staging, or commit, finish that
normal completion flow before selecting again. Likewise, use an existing
zdev-managed commit step for a complete planning or repair checkpoint when the
route defines one. Never create a checkpoint commit for incomplete
implementation or mix unrelated paths merely to clear the checkout.

An intentional clarification made through the active-task rule is attributable
state, not unrelated drift. Fresh work-context must still admit the same ready
task at the original HEAD. Verification reads the updated brief and task while
also receiving the original baseline. Completion stages the clarified record
paths with the other attributable task-owned paths; any other record or source
change still requires ownership review.

`zdev-implement <area>` reads effective complexity from the selected task in
work-context.
Authored `routine` uses `routine-implementer`; `standard`, including an omitted
legacy value, uses `implementer`. Never infer routine work from files or diff
size. Before any edit for `advanced`, start one fresh read-only `planner` using
the run's retained planner settings. Give it repository guidance and the stored
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

A worker that finds an incidental technical restriction puts one transient
adjustment proposal in its blocker evidence. It identifies the active task or
brief record paths and explains the necessary correction. The coordinator owns
the record edit and applies the checks above before changing it. This proposal
adds no durable state or approval record. After the edit, coordination records
the exact clarified paths and explanation in the current handoff, captures
fresh work-context, and resumes implementation. Explicit constraints and unclear
ownership are ordinary terminal blockers, not adjustment proposals.

The implementer object keeps verdict `blocker`, escalation `none`, no findings,
and exactly one evidence string. That string is
`PROPOSE zdev-adjustment <area> <task-id>\n` followed by exactly one JSON object
with only `record_paths` and `explanation`. `record_paths` contains one or more
unique paths naming only `.zdev/<area>/brief.md` or the active task record.
The explanation names the incidental restriction and why the agreed
outcome requires correcting it. Prefixes, suffixes, unknown keys, duplicate
paths, and other managed records make the proposal invalid.

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
same retained concrete role settings when the harness can resume it, or a replacement with those settings and the
replacement with the unchanged goal, baseline, current checkout, and full
findings. A verifier may request `advanced-implementer` once, only after the
initial standard/default implementation. That starts a replacement advanced
implementer using the retained advanced-implementer settings without planning and is followed by a fresh verifier using the retained verifier settings.
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

The exact installed task-workflows contract path is
"/Users/zayenz/projects/zdev/.pi/skills/zdev-pi/references/task-workflows.md". Decode it and include the resulting
path in every `zdev_subagent` payload.

Use `$ARGUMENTS` as the area. The current Pi session is the coordinator. After
preflight, select role `routine-implementer`, `implementer`, or
`advanced-implementer` from effective complexity. Call read-only `planner` once
before the first advanced edit. Use a fresh `verifier` for every full
verification. Pi uses a fresh same-profile child for ordinary rework. A valid
one-time standard escalation uses an advanced replacement without replanning.
Extract one balanced object from the planner response, tolerating brief prose
or a Markdown fence. Validate its four semantic fields, reconstruct the
compatible nine-key public planner envelope, and pass the semantic plan object
unchanged to the advanced implementer. A blocker has empty public evidence and
stops before edits. Do not call another planner.
Each `zdev_subagent` call receives the installed route-contract path and a
compact payload of brief, task, guidance, and source file paths, applicable
snapshot IDs, and the short result from the preceding role. Pi children read
those files from the shared checkout; do not copy the rendered contract.
Immediately before each verifier child, the current session stores and
validates the snapshot; after extracting and validating one balanced semantic
object it compares that snapshot and constructs the public nine-key verifier
envelope. Never repeat a child only to remove wrapping text.

<!-- zdev:generated-repository-guidance:start -->
## Repository guidance discovery

Before inspecting, planning, changing, or validating code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.
<!-- zdev:generated-repository-guidance:end -->

# Derived work handoffs

> **Status: review implemented; publication remains design only.** Zdev parses
> and reviews derived proposals but does not yet apply them.

Zdev may publish a small follow-up bundle without asking the user to approve
work they have already approved. This exception applies only to direct work
inside the current task's area brief, outcome, boundaries, and testing policy.
It adds no task state, lineage metadata, or worker authority over `.zdev`.

The coordinator owns the decision and the mutation. Each worker result may
return one transient proposal. It never imports tasks, edits the source task,
completes work, stages files, or commits.

## Transient proposal

A proposal has exactly two parts: this first line and one JSON object. Any
prefix, suffix, Markdown fence, missing field, unknown field, duplicate JSON
key, invalid UTF-8, or second object makes it invalid.

```text
PROPOSE zdev-derived <area> <source-task-id>
{
  "schema_version": 1,
  "proposal": "investigation_follow_up",
  "area": "payments",
  "source_task": "payments-007",
  "source_result": {
    "status": "complete",
    "summary": "Settled the retry boundary.",
    "validation": ["Compared the design with the current retry implementation."]
  },
  "tasks": [
    {
      "key": "apply-retry-boundary",
      "title": "Apply the settled retry boundary",
      "blocked_by": [],
      "outcome": "Use the settled boundary in the runtime path.",
      "context": "The investigation settled where retries stop.",
      "boundaries": ["Do not change the public retry policy."],
      "done_when": ["The runtime uses the settled boundary."],
      "validation": ["Run the focused retry tests."]
    }
  ]
}
```

The top-level object denies unknown fields. `schema_version` is `1`.
`proposal` is exactly `investigation_follow_up` or `implementation_split`.
The envelope area and source task must match the selected goal and first line.
Every string is non-empty and single-line where the ordinary task-bundle
contract requires it. `tasks` contains one through five complete drafts in
dependency order and denies unknown draft fields. A task draft cannot contain
a proposal or another task bundle.

`source_result` also denies unknown fields. An investigation follow-up requires
`status: "complete"`, a completion summary, and the exact validation list that
will be written to the source task. A split requires `status: "split"`, a
summary of why the approved implementation must be divided, and an empty
validation list. A split does not add a Result section or complete its source.

An implementation split also requires this top-level object after
`source_result`; an investigation follow-up forbids it:

```json
"split_ownership": {
  "retained_parent_paths": ["src/payments.rs"],
  "child_future_paths": [
    {
      "key": "extract-retry-policy",
      "paths": ["src/retry_policy.rs", "tests/retry_policy.rs"]
    },
    {
      "key": "wire-retry-policy",
      "paths": ["src/retry_wiring.rs", "tests/retry_wiring.rs"]
    }
  ]
}
```

`split_ownership` and each child entry deny unknown fields and duplicate JSON
keys. `retained_parent_paths` equals, as a set, the paths in the coordinator's
captured attributed unstaged parent delta; neither side may omit or add a path.
Automatic apply also requires that delta to be the checkout's complete
unstaged path set and requires an empty index; other existing changes route the
split to manual review.
`child_future_paths` names every proposed key exactly once and no other key,
and each child has at least one path. Duplicate paths, overlap between children,
and overlap with a retained parent path are invalid.

Every path is a normalized repository-relative UTF-8 path with `/` separators:
no absolute path, empty segment, `.`, `..`, duplicate separator, directory, or
symlink is allowed. A retained path and an existing future path must be a
regular file. An absent future path is valid only when its existing parent is a
real directory, not a symlink, and the child explicitly creates a regular file
there. Deleted or renamed parent paths require manual review because they
cannot satisfy the retained regular-file check.

Staged, overlapping, incomplete, or uncertain assignments cannot apply
automatically. The coordinator shows the failure and sends a corrected proposal
through manual review; approval never makes an invalid path assignment valid.

Drafts use the existing TaskDraft fields, including optional `complexity` and
`slice`. A declared slice must already exist in the area and passes the same
slice validation as an ordinary task bundle; a proposal cannot create a slice.
`blocked_by` may name another proposed key or an existing task ID in the same
area. It may not name the source task. Existing bundle and graph validation
still reject missing dependencies, cycles, self-blocking tasks, and invalid
task bodies.

The source identity, proposal, result, and drafts live only in the handoff and
command input. Task files keep their current schema.

## Automatic authority

The coordinator may apply the proposal without another confirmation only when
all of these statements are true:

- the area is open and its task-work status is safe; the branch, goal, source
  task, HEAD, and three-part Git baseline still match the handoff;
- an investigation follow-up has a matching independent `PASS` for the complete
  result; for a split, any existing delta is either empty or exactly attributed
  unstaged work of the open source task, and every child names only future
  path-disjoint work;
- every draft is necessary to finish the approved objective and stays inside
  the brief, source task boundaries, and existing testing decision;
- the proposal introduces no product choice, compatibility policy, destructive
  operation, new ownership claim, cross-area dependency, or uncertain scope;
- all keys are exact, unique within the proposal, and absent from the area; and
- this worker result contains one proposal object, no nested proposal, and this
  uninterrupted handoff has not applied another proposal.

This is a narrow grant of authority, not a heuristic classifier. The
coordinator reads the proposal and repository evidence. It never infers safety
from filenames, a confidence score, or a worker's assertion that work is
in-scope.

Any failed statement routes the unchanged proposal through manual review before
mutation. Applying a proposal consumes that worker result's automatic authority;
a second object, nested proposal, or second apply in the same handoff cannot run
automatically. After an interruption the old handoff has no automatic
authority.

A later independently selected execution, including a derived child or the
resumed parent, may produce one new proposal and automatic apply if it passes
the same current scope, ownership, branch, graph, and safety gates. Each
proposal must add zero scope: any wider outcome, product choice, destructive
work, or uncertainty uses manual review. Zdev stores no global or cross-session
derivation count and does not infer lineage from task content or Git history.

Malformed proposals, unsafe or closed areas, duplicate keys, invalid graphs,
ambiguous ownership, destructive work, widened scope, and unresolved product
choices all stop before publication. A malformed handoff is shown as received;
only a corrected envelope can enter manual review. Reopening an area, changing
scope, or resolving a product choice remains a user decision. Duplicate
detection compares exact keys only; zdev neither guesses semantic equivalence
nor renames a key.

## Investigation follow-up and implementation split

An investigation follow-up completes the source and adds the accepted tasks in
one transaction. The new tasks use their declared dependencies and enter the
normal numeric ready order. They do not depend on the completed investigation
merely to record origin.

An implementation split keeps the source open. The transaction adds every new
child ID to the source task's existing `blocked_by` list. Children may depend
on one another as proposed, but never on the source. The source becomes the
final integration and whole-task verification step: it becomes ready only
after every child is done, and it can complete only after its original done
conditions pass independent verification. No child replaces or relaxes those
conditions.

An edited checkout does not by itself force manual review. The coordinator may
retain an exact captured delta with the open source task when each child names
only future work in `split_ownership` paths absent from that delta and none of
the retained delta is staged. For each child, apply appends this canonical
boundary to the rendered task:

```text
- Task-owned paths (exact): ["src/retry_policy.rs","tests/retry_policy.rs"]
```

The JSON array uses normalized paths in the proposal's order. This ordinary
task boundary makes ownership actionable after the transient envelope is gone;
it adds no TOML field or lineage record. The split transaction commits only the
task records and leaves the parent's unstaged bytes exactly as they were. Later
child work treats that delta as pre-existing parent-owned state under the
ordinary baseline rules and may change only its rendered path set.

If a child must change a path already changed by the parent, any path is
staged, another unstaged path exists, or attribution is uncertain, the split
needs manual review. The coordinator does not divide hunks, alter the index,
stash, reset, or assign partial work by guessing.

## Review, publication, and rollback

The implemented `zdev tasks derive review` command reports mechanical
eligibility without writing repository state. The coordinator still compares
the retained handoff context and decides whether the work is directly in scope.
The result includes the complete envelope, its opaque fingerprint, and the
ordinary task-bundle rendering, so a semantic authority failure can use manual
review without changing the proposal. The apply behavior below is not yet
implemented.

For automatic authority, the coordinator shows the exact proposal once with
`Authority: automatic` and proceeds without asking a redundant question. For
manual authority, `zdev tasks derive review` validates and renders the complete
envelope and returns an opaque review fingerprint over its canonical form. The
coordinator shows that rendering and asks for approval. Any change to the
source result, mode, order, dependency, or task content requires a new opaque
review fingerprint. Ordinary `zdev tasks review` and `tasks import --approval`
remain available for manually authored bundles.

`zdev tasks derive apply` accepts the unchanged envelope and an optional
opaque review fingerprint. Automatic use omits the fingerprint; reviewed use
requires it. Before writing, the command acquires the existing state lock,
rereads the area, source task, slices, tasks, branch state, Git operation,
worktree, and index, allocates IDs, renders all files, and validates the complete
hypothetical graph and index. For a split it also requires an empty index,
recaptures the complete unstaged path set, requires exact equality with
`retained_parent_paths`, validates every path and disjoint child assignment,
and renders the canonical ownership boundary. It runs the same check before
committing.

For an investigation follow-up, the coordinator first stages only the verified
task-owned artifact paths. The apply command requires the index to match that
attributed set, then includes the completed source task, child task files, and
`TASKS.md` in the same existing stable-change-ID commit. A split commits only
the changed open source task, children, and index; any retained parent delta
stays in its exact prior index and worktree state.
Thus a successful investigation never has a committed Result without its
follow-ups or committed follow-ups without its Result.

Proposal, dependency, graph, check, write, staging, or commit failure leaves no
accepted partial result. The command removes every new task, restores the
source task and `TASKS.md` byte for byte, and restores the exact prior index.
Pre-existing worktree bytes and unrelated staged or unstaged changes remain
unchanged. If rollback itself fails, the command reports every affected path
and stops; it never reports the proposal as accepted. Publication success
leaves managed paths clean and returns one JSON result with the exact source
result, allocated IDs in proposal order, post-import ready frontier, commit,
stable change ID, and committed paths.

The success result has this exact field set. Arrays preserve proposal, ready,
and committed-path order:

```json
{
  "schema_version": 1,
  "status": "committed",
  "area": "payments",
  "source_task": "payments-007",
  "proposal": "investigation_follow_up",
  "source_result": {
    "status": "complete",
    "summary": "Settled the retry boundary.",
    "validation": ["Compared the design with the current retry implementation."]
  },
  "tasks": ["payments-008"],
  "ready": ["payments-008"],
  "commit": "<full-commit-id>",
  "change_id": "<stable-change-id>",
  "paths": [
    "docs/retry-boundary.md",
    ".zdev/payments/tasks/007-investigate-retry-boundary.md",
    ".zdev/payments/tasks/008-apply-the-settled-retry-boundary.md",
    ".zdev/payments/TASKS.md"
  ]
}
```

This investigation result forbids `split_ownership`. A split success result has
the same field set and order but requires `split_ownership` immediately after
`source_result`, copied unchanged from the accepted proposal; its
`source_result.status` is `split`. The `paths` array contains only the changed
source task, new child task files, and `TASKS.md`; retained and future source
paths are not committed by the split.

After success, the coordinator shows that JSON unchanged. The pre-publication
proposal and post-publication result give the user the same evidence as an
approval round without adding another confirmation turn.

## Required scenario traces

| Scenario | Result |
| --- | --- |
| Successful investigation | Matching `PASS`; source completion, one to five children, index, artifact, and commit succeed together. |
| Investigation with no follow-up | No proposal is emitted; use ordinary verified completion and commit. |
| Safe implementation split | Empty delta, or exact unstaged parent-owned delta with path-disjoint future child work; task records commit, retained bytes stay unchanged, and the open source is blocked by every child. |
| Product-choice split | Automatic authority fails; show the exact proposal and ask through manual review; write nothing. |
| Duplicate proposal | An existing or repeated key fails before ID allocation or writing; never rename it automatically. |
| Invalid dependency | Missing, source-task, cyclic, or self dependency fails hypothetical graph validation; write nothing. |
| Publication or commit failure | Restore every managed byte and the prior index; preserve unrelated work and report rollback failure explicitly if restoration is incomplete. |
| Nested or repeated handoff proposal | Reject a nested object or second automatic apply from the same worker result; applying consumes that handoff. A later independently selected execution gets one new proposal only after all current gates pass again. |

## Small implementation follow-ups

1. **Parse and review derived proposals.** Add the strict envelope parser,
   mode-specific validation, exact rendering, and optional full-envelope
   fingerprint. Reuse task-draft validation and add focused black-box cases for
   malformed envelopes, one-object and 1–5-task limits, duplicate keys, slices,
   dependencies, and strict split-ownership paths.
2. **Publish one derived transaction.** Add `tasks derive apply` by composing
   existing state locking, ID allocation, graph/index rendering, task
   completion, stable commit, and rollback seams. Cover successful follow-up,
   clean and retained-delta splits, canonical child ownership boundaries,
   consumed handoffs, publication failure, and commit failure. Do not add a
   general transaction framework or durable derivation counter.
3. **Route the handoff in canonical guidance.** Update investigation and
   implementation coordination once, regenerate every harness integration, and
   test only the automatic/manual boundary, per-result consumption and later
   independent execution rule, exact user evidence, and preservation of the
   ordinary reviewed-import path.

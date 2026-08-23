# Turn approved work into tasks

Use this method to turn a sufficiently sharp objective, plan, specification, or
selected findings into individual zdev task files.

## Gather context

Require an existing area and read its `brief.md`, the conversation or
referenced source, relevant code and tests, domain vocabulary, and applicable
decisions. If no area exists, stop and offer **Explore an objective**; do not
create the area here. A task queue should contain
agent-ready work, not hidden human decisions.

Draft directly when the brief already settles the product and testing choices.
Require its `Testing` section to state the agreed level. When that section is
absent or a material choice remains, route to **Discuss the brief** or **Explore
an objective** so the user can settle it before task drafting.

The conventional `general` area may use a standing minimal brief for unrelated
one-off work. When that brief supplies shared engineering rules and the current
request or discussion settles the one-off product and testing choices, draft
the exact task bundle directly; do not require a separate research or Explore
interaction. Keep an unsliced one-off task unsliced. Create or use a slice only
when several tasks benefit from one narrower objective and shared boundaries.

## Draft tracer-bullet tasks

Each task should:

- deliver a narrow, complete path through every affected layer;
- fit in one fresh implementer context;
- be independently demonstrable or verifiable;
- state one outcome, useful task context, task-specific boundaries, observable
  done conditions, and useful validation; and
- depend only on tasks that genuinely prevent it from starting.

The general-area fast path changes planning depth, not task quality. Every task
still needs a useful outcome, explicit boundaries, observable done proof,
proportionate testing and validation, exact bundle review and approval, safe
branch state, independent implementation verification, completion evidence,
and a commit.

For each newly drafted implementation task, write concise `Context` that tells
a fresh implementer why the slice exists, how it connects to current repository
behavior, which settled constraints apply, and where the relevant source and
test seams are. Select and connect the facts needed for this slice; keep shared
material authoritative in `brief.md` and point to it rather than copying it. Do
not use word counts or prose length as a quality check.

Keep task files focused on task-specific outcomes. When an area has a
`background/` corpus, link only the documents relevant to that task instead of
copying their content or attaching the entire corpus. Links in a task file are
relative to `.zdev/<area>/tasks/`, so a retained source can be referenced as
`[Capacity rules](../background/capacity-rules.md)`. The brief's source index
remains the canonical entry point for shared area context and the brief remains
the authoritative synthesis. Task creation consumes background material
retained by approved shaping or an authorized investigation task; it does not
create transcripts, raw evidence dumps, source copies, prototypes, or other
background files as part of drafting a bundle.

Derive each task's test work from the brief's testing level and the repository's
established patterns. State the specific tests to add or update when new tests
are warranted, or state that no new tests are expected when they are not. Do
not turn generic goals such as "add comprehensive tests" or "cover edge cases"
into unbounded implementation work. Validation may run existing tests, builds,
linters, type checks, or focused manual checks without requiring new test code.

Prefer several thin vertical slices to horizontal phases. Add a small
behavior-preserving prefactor first only when it materially simplifies an
approved slice.

When a task belongs to an existing slice brief, set its optional `slice` key in
the bundle. Leave unrelated tasks unsliced. Use `blocked_by` for task ordering;
slice membership does not create another dependency mechanism.

For a wide mechanical change that cannot land as vertical slices, use
expand–migrate–contract: add the new form beside the old, migrate callers in
green batches, then remove the old form after every batch. Express those gates
with `blocked_by`.

## Challenge the drafts

First build the complete Task Bundle JSON defined in the task-format reference
loaded for this route. Keep every task and dependency in dependency order, then
validate and store the candidate with:

Supply the exact Task Bundle JSON on standard input:

```text
zdev tasks review <area> --from - --format json
```

The command returns a small opaque `review` identity and the path to the exact
stored `review.md`. Storage is not user approval. For non-trivial work, ask a
different agent to read that Markdown path and review the candidate. Give it
the area `brief.md`, relevant decisions, repository guidance, source and tests,
and linked background documents. It does not need the bundle JSON or adjacent
review metadata. Ask for concrete revisions covering:

- missing task-specific context or repository evidence;
- hidden decisions that still belong to the user;
- vague or non-observable done conditions and proof;
- incorrect boundaries or scope; and
- false dependencies.

The coordinating agent reconciles those suggestions in its draft and resolves
user choices. If the candidate changes, run `zdev tasks review` again to
atomically replace the stored candidate, then challenge the new returned
Markdown path. Repeat until the reviewer has no concrete revisions. An
unchanged candidate proceeds directly to presentation. If another agent is
unavailable, run the same review locally. A local review is enough for trivial
or fully specified work. Say which kind of review ran.

Before presentation, reconcile proposed tasks with existing task keys and
completed outcomes. Resolve every concrete design or testing choice that would
change the bundle. Only the final challenged stored artifact is shown for user
approval; neither storage nor independent challenge grants approval.

This route owns manually authored task bundles. A strict proposal returned by
an investigation or implementer follows the derived-work handoff in the
implementation contract, which owns source completion, child creation, review
when the user must decide, and its managed commit.

## Present the split

Run `zdev tasks review <area> --show` and show the final challenged Markdown
unchanged, then ask: `Approve this task bundle for import?` The coordinator
retains the current opaque identity automatically; the user never reads, copies, or
reasons about it or the internal fingerprint. If the candidate
changes after presentation, replace it, challenge the replacement, and present
the new stored Markdown for fresh approval. Do not reconstruct the document or
create a transport file.

## Import

After explicit approval, import the stored review with the retained identity
without another user step:

```text
zdev tasks import <area> --reviewed <review-id> --format json
```

When adding tasks to an existing task list, add `--commit --format json` to the
import command. Use ordinary import for the initial task split or when the user
wants uncommitted additions. If the approved work also modified the owning
area's brief, leave that tracked worktree change in place: the committed import
validates and includes it in the same managed commit. Report the returned
commit and stable change ID. Follow zdev's recovery message if it cannot commit.
Do not stage, unstage, or commit paths manually while recovering the import.

The managed commit contains the modified brief when present, new task files,
and regenerated `TASKS.md`. It does not interrupt a selected task. Keep the
current selection and consider additions at the next `zdev next` boundary.
Review any other concurrent commit that changes existing tasks, `brief.md`, area
metadata, lifecycle state, or source.

If the user supplies a bundle path, pass it to review unchanged; approved import
still reads the stored artifact.
Report allocated task IDs and the complete ready frontier returned by the import.
Offer **Implement** and stop unless the user already authorized implementation
of this approved bundle. When authorized, read `implement.md` completely and
apply its preconditions to the actual next ready task. A successful import has
already validated the graph; the Implement route collects the next fresh
work-context. Stop and report when no task is ready or an implementation
precondition fails.

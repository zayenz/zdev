# Turn approved work into tasks

Use this method to turn a sufficiently sharp objective, plan, specification, or
selected findings into individual zdev task files.

## Gather context

Require an existing area and read its `brief.md`, the conversation or
referenced source, relevant code and tests, domain vocabulary, and applicable
decisions. If no area exists, stop and offer **Explore an objective**; do not
create the area here. A task queue should contain
agent-ready work, not hidden human decisions.

Normally offer **Discuss the brief** before drafting tasks for non-trivial work.
The user may skip it when the brief is already sharp. Require the brief's
`Testing` section to state the agreed level. If it is absent, or a material
design or testing choice remains unresolved, stop and recommend **Discuss the
brief** or **Explore an objective** instead of making the choice here.

## Draft tracer-bullet tasks

Each task should:

- deliver a narrow, complete path through every affected layer;
- fit in one fresh implementer context;
- be independently demonstrable or verifiable;
- state one outcome, useful task context, task-specific boundaries, observable
  done conditions, and useful validation; and
- depend only on tasks that genuinely prevent it from starting.

For each newly drafted implementation task, write concise `Context` that tells
a fresh implementer why the slice exists, how it connects to current repository
behavior, which settled constraints apply, and where the relevant source and
test seams are. Select and connect the facts needed for this slice; keep shared
material authoritative in `brief.md` and point to it rather than copying it. Do
not use word counts or prose length as a quality check.

Keep task files focused on task-specific outcomes. When an area has a
`background/` corpus, link only the documents relevant to that task instead of
copying their content or attaching the entire corpus. Links in a task file are
relative to `.zd/<area>/tasks/`, so a retained source can be referenced as
`[Capacity rules](../background/capacity-rules.md)`. The brief's source index
remains the canonical entry point for shared area context.

Derive each task's test work from the brief's testing level and the repository's
established patterns. State the specific tests to add or update when new tests
are warranted, or state that no new tests are expected when they are not. Do
not turn generic goals such as "add comprehensive tests" or "cover edge cases"
into unbounded implementation work. Validation may run existing tests, builds,
linters, type checks, or focused manual checks without requiring new test code.

Prefer several thin vertical slices to horizontal phases. Add a small
behavior-preserving prefactor first only when it materially simplifies an
approved slice.

For a wide mechanical change that cannot land as vertical slices, use
expand–migrate–contract: add the new form beside the old, migrate callers in
green batches, then remove the old form after every batch. Express those gates
with `blocked_by`.

## Challenge the drafts

For non-trivial work, ask a different agent to review the draft. Give it the
proposed tasks, area `brief.md`, relevant decisions, repository guidance,
source and tests, and linked background documents. Ask for concrete revisions
covering:

- missing task-specific context or repository evidence;
- hidden decisions that still belong to the user;
- vague or non-observable done conditions and proof;
- incorrect boundaries or scope; and
- false dependencies.

The coordinating agent reconciles those suggestions, edits `.zd`, resolves
user choices, and asks for import approval. If another agent is unavailable,
run the same review locally. A local review is enough for trivial or fully
specified work. Say which kind of review ran.

Before approval, reconcile proposed tasks with existing task keys and completed
outcomes. Resolve every concrete design or testing choice that would change the
bundle before requesting approval.

## Review the split

Build the complete Task Bundle JSON defined in
[task-format.md](task-format.md). Keep every task and dependency in dependency
order. Send that JSON to:

```text
<task-bundle-json> | zd tasks review <area> --from - --format json
```

The command validates the bundle shape and returns an `approval` fingerprint
and a complete `markdown` document. Show the returned Markdown unchanged, then
ask: `Approve this task bundle for import?`

Keep the reviewed JSON unchanged. If any task content, order, or dependency
changes, run `zd tasks review` again, show the new Markdown, and request fresh
approval. Pass the bundle through standard input or use a path supplied by the
user; do not create a transport file.

## Import

After explicit approval, send the reviewed JSON and its fingerprint directly to
zdev:

```text
<reviewed-task-bundle-json> | zd tasks import <area> --from - --approval <approval-id>
zd check <area> --format json
zd tasks list <area> --format json
```

When adding tasks to an existing task list, add `--commit --format json` to the
import command. Use ordinary import for the initial task split or when the user
wants uncommitted additions. Report the returned commit and stable change ID
for a committed import. Follow zdev's recovery message if it cannot commit.
Do not stage, unstage, or commit paths manually while recovering the import.

A commit containing only new task files and regenerated `TASKS.md` does not
interrupt a selected task. Keep the current selection and consider additions at
the next `zd next` boundary. Review any concurrent commit that changes existing
tasks, `brief.md`, area metadata, lifecycle state, or source.

If the user supplies a bundle path, pass it to both review and import unchanged.
Report allocated task IDs and the ready frontier from the post-import task list.
Offer **Implement** and stop unless the user already authorized implementation
of this approved bundle. When authorized, read `implement.md` completely and
apply its preconditions to the actual next ready task. Continue only after the
import and checks succeed, without changing the approved content. Stop and
report the state when no task is ready or an implementation precondition fails.

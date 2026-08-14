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
- fit in one fresh implementation-agent context;
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

For non-trivial work, ask a fresh read-only reviewer to challenge the draft.
Give the reviewer the proposed tasks, the area `brief.md`, relevant decisions,
repository guidance, source and tests, and any linked background documents.
Ask it to propose concrete revisions and report:

- missing task-specific context or repository evidence;
- hidden decisions that still belong to the user;
- vague or non-observable done conditions and proof;
- incorrect boundaries or scope; and
- false dependencies.

The reviewer must not edit `.zd`, import tasks, approve the split, or decide
user-owned choices. Reconcile its suggestions against repository evidence and
return unresolved material choices to the user. If a fresh reviewer is not
available, perform the same evidence-based challenge locally. For trivial or
fully specified work, a local review is enough. Never claim an independent
review that did not run.

## Review the split

Present the reconciled draft in dependency order as one human-readable
Markdown document inside a fenced block. Use a fence longer than any run of
backticks in the values. The document must use this exact structure:

````markdown
```markdown
# Task Bundle

## Area
scheduling

## Schema version
1

## Task 1

### Key
model

### Title
Add the scheduling model

### Outcome
The model represents the required scheduling decisions.

### Context
Add the model beside the existing scheduler types.

### Boundaries
1. Do not change unrelated APIs.
2. Use the vocabulary settled in the brief.

### Blocked by
None

### Done when / proof
1. The model represents every decision named in the brief.
2. Focused tests cover the model.

### Validation / Testing
1. Run the focused model tests.
2. Apply the area's focused testing level; no broader tests are required.
```
````

Put each scalar value on the line after its heading. Render each list value as
a numbered item and preserve item order. Render an empty optional scalar or
empty list as the single value `None`. Repeat the complete `## Task N` section
for every task, preserving task order. Show every value that will be imported;
do not add presentation-only text inside the fence. In `Validation / Testing`,
include the task's validation steps and state how the area's agreed `Testing`
level applies.

The fenced Markdown document is the exact approval source. It must carry every
value losslessly so the approved fields, list order, task order, and dependency
edges can be serialized internally without interpretation or rewriting. Keep
the corresponding structured values in working context and serialize those
retained values after approval; do not reconstruct them by parsing or
paraphrasing the display. Do not summarize, omit, or rephrase task content in a
separate approval view.

Ask whether:

- the tasks are too coarse or too fine;
- each task delivers a complete result;
- the blocking edges are genuine; and
- any task should be merged, split, or removed.

Reconcile additions with existing task keys and outcomes. Do not duplicate
work already present or completed. Do not present the exact Task Bundle JSON by
default; show it only when the user explicitly asks to inspect it, and keep the
fenced Markdown document as the approval source. Ask for explicit approval
immediately before every import, including additions to or material revisions
of an existing queue. If the bundle changes after approval—meaning any task
content, task order, or dependency changes—re-render the complete Markdown list
and obtain fresh approval. Approval of a brief, earlier split, or objective is
not approval of the import.

## Import

Only after that explicit approval, serialize the exact approved Markdown
rendering internally as Task Bundle JSON using the format in
[task-format.md](task-format.md). Preserve every approved field value, list
order, task order, and dependency edge without additions, omissions, or
rewrites. Then send the serialization directly to `zd` on standard input:

```text
<internally-serialized-task-bundle-json> | zd tasks import <area> --from -
zd check <area> --format json
zd tasks list <area> --format json
```

When adding tasks to an existing task list, use:

```text
<internally-serialized-task-bundle-json> | zd tasks import <area> --from - --commit --format json
```

Use ordinary import for the initial task split or when the user explicitly
wants the additions left uncommitted. The committed import includes only the
new task files and regenerated `TASKS.md`. Report the returned commit and stable
change ID. If zdev refuses or cannot commit, follow its recovery message; do
not stage, unstage, or commit paths manually.

A commit containing only new task files and regenerated `TASKS.md` does not
interrupt a task already being implemented. Keep the current selection and
consider the additions at the next `zd next` boundary. Review any concurrent
commit that changes an existing task, `brief.md`, area metadata, lifecycle
state, or source.

Do not create a transport file. If the user instead supplies a bundle path,
pass it to `--from` unchanged and leave the file in place. Derive and report
allocated task IDs and the actual ready frontier from the post-import task
list, not from draft keys or assumptions. Do not edit `TASKS.md`; zdev
regenerates it. Offer **Implement** and stop unless the user's current message
explicitly authorized
implementation after importing this exact approved rendering. In that case,
continue only after the import and post-import checks succeed without changing
the approved task content, order, or dependencies. Read `implement.md`
completely, then apply every **Implement** precondition to the actual next ready
task. Stop and report the failed gate if no task is ready or any implementation
precondition fails.

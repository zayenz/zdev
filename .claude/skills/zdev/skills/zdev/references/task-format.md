# Task format

## Project and area metadata

`.zd/config.toml` records project `trunk` when HEAD names a branch. Each
`.zd/<area>/area.toml` requires the area's owning `branch`; `parent` and
`base_commit` are optional until their branches and boundary are available. The
parent area replaces trunk as the effective base. `base_commit` is the exact
base commit last incorporated into the area branch; zdev uses it as the
boundary before area-owned commits during a managed rebase.

Area metadata without `branch` is invalid. Configure an unbound trunk and
establish the area's base anchor before selecting, completing, or rebasing
work. Use `zd area bind` to correct an existing branch binding or establish its
anchor. Add a parent only after both local branches exist and the child has a
trustworthy base boundary.

Task dependencies never cross areas. Use an area parent for branch ancestry
and `blocked_by` for task sequencing within one area.

## Task files

Each task is one Markdown file under `.zd/<area>/tasks/`.

```markdown
+++
schema_version = 1
id = "scheduling-001"
key = "model"
area = "scheduling"
status = "open"
blocked_by = []
+++
# Add the scheduling model

## Outcome

The model represents the required scheduling decisions.

## Context

The existing scheduler accepts jobs but has no durable decision model. Add the
model beside the scheduler types in `src/scheduling/model.rs`; use the job
vocabulary settled in `brief.md` and extend the focused model tests in
`tests/scheduling_model.rs`.

## Boundaries

- Change the scheduling model and its focused tests.
- Preserve the solver API and follow the area's existing model-test patterns.

## Done when

- [ ] The model represents the required scheduling decisions.
- [ ] Focused model tests cover the decision behavior named in the brief.

## Validation

- Run the focused model tests.
```

The frontmatter contains only routing state:

- `id` is stable and allocated by zdev;
- `key` connects a temporary import bundle to the durable task;
- `area` prevents tasks from being moved between objectives accidentally;
- `status` is `open` or `done`; and
- `blocked_by` contains stable task IDs.

An open task is ready when every blocker is done. Otherwise it is blocked.
Zdev chooses the ready task with the lowest numeric ID.

`zd task done` checks the done-condition boxes, changes the status, and appends
one result section:

```markdown
## Result

Implemented and independently verified the scheduling model.

Validation:

- Focused model tests passed.
```

`TASKS.md` is a generated list of all task files and their derived state. It is
never an authored source.

## Task bundle

`zd tasks import` accepts a JSON object with `schema_version`, `area`, and a
`tasks` array. Each task contains:

- `key`;
- `title`;
- `blocked_by`, using keys from the same bundle or existing task IDs;
- `outcome`;
- optional `context`, rendered as prose under `## Context`;
- optional `boundaries` for task-specific non-goals and scope limits;
- one or more `done_when` strings; and
- optional `validation` strings.

Prefer passing an approved bundle directly with
`zd tasks import <area> --from -`. A path is also accepted for manual use and
is left in place after import. When adding tasks to an existing task list, use
`zd tasks import <area> --from - --commit --format json`. Use ordinary import
for the initial split or when the user explicitly wants uncommitted additions.
A committed import contains only the new task files and regenerated `TASKS.md`;
its JSON result includes task IDs, paths, the commit hash, and the stable change
ID. Zdev validates the complete dependency graph before creating any task file.

Keep shared context, including the required area-wide testing level, in
`brief.md`. For newly drafted implementation tasks, use `context` to select and
connect the task-specific reason, current repository behavior, settled
constraints, and relevant source or test seams. Point to the brief or
background documents instead of copying their shared material. Version 1
bundles and task files without `context` are accepted. Use task
boundaries only for limits the implementation and verification agents need for
that slice. Test-related done conditions and validation must apply the agreed
level, not silently expand it.

`zd check` requires non-empty `Outcome`, `Done when`, and `Validation` sections.
Done conditions must be checklist items; a completed task must have every item
checked and include a non-empty `Result` section.

# Task format

## Project and area metadata

`.zdev/config.toml` records project `trunk`. An isolated
`.zdev/<area>/area.toml` stores its owning `branch`; optional `parent` and
`base_commit` describe its managed base relationship. An explicit trunk area
stores only `mode = "trunk"` and dynamically resolves its branch from
`project.trunk`; it stores no branch, parent, or base commit. Several explicit
trunk areas may share configured trunk, but an isolated area remains its
branch's sole owner. Never infer mode from a branch name.

Use the default create form or `--branch` for isolated work. Use `--trunk` only
when the user explicitly wants a personal/project area on configured trunk;
pull-request records remain isolated. Trunk work requires the resolved trunk
checked out and safe, but never requires freshness or rebase ceremony.

Task dependencies never cross areas. Use an area parent for branch ancestry
and `blocked_by` for task sequencing within one area.

## General area convention

`general` is the conventional tag for recurring one-off work. It is an ordinary
area, not an area kind. Create or switch to its isolated branch and run:

```sh
zdev area create general --title "General work" --objective "Keep concrete one-off improvements as reviewed tasks."
```

When the user explicitly prefers shared-trunk work, the same command may add
`--trunk`; this changes only area branch mode, not task lifecycle or ownership.

Keep `brief.md` minimal and reusable. It records shared engineering boundaries,
the testing level, and validation; each task records its own concrete outcome,
context, boundaries, done proof, and validation. Unsliced tasks are normal.
Slices remain optional for groups of related tasks. All ordinary task review,
approval, branch, verification, completion, and commit rules still apply.

## Slice briefs

Larger areas may keep lightweight slice briefs under
`.zdev/<area>/slices/<key>.md`. Create and inspect them with:

```sh
zdev slice create <area> <key> --title <title> --objective <objective> --boundary <text>
zdev slice list <area>
zdev slice show <area> <key>
```

Repeat `--boundary` for additional boundaries. Each file has TOML frontmatter
containing exactly `schema_version`, `key`, `area`, and `title`, followed by
non-empty `## Objective` and `## Boundaries` sections. The key is a lowercase
path segment and must match the filename.

A slice is durable planning context, not a second lifecycle. It has no status,
tasks need not belong to one, and the area brief remains authoritative for
shared decisions and testing. A task that uses a slice should link that slice
brief explicitly.

## Task files

Each task is one Markdown file under `.zdev/<area>/tasks/`.

```markdown
+++
schema_version = 1
id = "scheduling-001"
key = "model"
area = "scheduling"
status = "open"
complexity = "advanced"
slice = "api"
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
- `status` is `open` or `done`;
- optional `complexity` is `routine`, `standard`, or `advanced`;
- optional `slice` names an existing slice brief in the same area; and
- `blocked_by` contains stable task IDs.

Omitted complexity resolves to `standard` without rewriting the task. Use
`routine` only for tightly specified, low-risk mechanical work, `standard` for
ordinary bounded implementation, and `advanced` when the approved task needs
additional planning or reasoning. Complexity does not change readiness,
lifecycle, or dependencies.

An open task is ready when every blocker is done. Otherwise it is blocked.
Zdev chooses the ready task with the lowest numeric ID.

`zdev task done` checks the done-condition boxes, changes the status, and appends
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

`zdev tasks import` accepts a JSON object with `schema_version`, `area`, and a
`tasks` array. Each task contains:

- `key`;
- `title`;
- optional `complexity`; omitted means `standard`;
- optional `slice`, naming an existing slice brief in the area;
- `blocked_by`, using keys from the same bundle or existing task IDs;
- `outcome`;
- optional `context`, rendered as prose under `## Context`;
- optional `boundaries` for task-specific non-goals and scope limits;
- one or more `done_when` strings; and
- optional `validation` strings.

Run `zdev tasks review <area> --from - --format json` to validate a draft bundle
and store its complete Markdown review, canonical bundle, and internal
fingerprint in Git administrative state. The small JSON result names the actual
Markdown path and an opaque `review` identity. Present the exact document with
`zdev tasks review <area> --show`; the user never handles the identity or
fingerprint. After approval, use `zdev tasks import <area> --reviewed
<review-id>`. Zdev requires that identity to remain current and revalidates the
stored bundle before publication. Direct `--from` import remains valid, and
`--approval` remains compatible with callers that already use it for direct
input.

Review, import, and `zdev check` reject a task whose `slice` does not name an
existing `.zdev/<area>/slices/<key>.md`. Unsliced tasks remain valid and appear
only in area-wide progress totals.

## Derived proposal

A worker may return one transient proposal for direct follow-up work instead of
editing task state. The first line is exact:

```text
PROPOSE zdev-derived <area> <source-task-id>
```

One JSON object follows. It contains only `schema_version: 1`, `proposal`,
`area`, `source_task`, `source_result`, `tasks`, and, for a split,
`split_ownership`. Proposal is `investigation_follow_up` or
`implementation_split`. The source result is `{status, summary, validation}`:
follow-up uses `complete` with non-empty validation; split uses `split` with an
empty validation list. `tasks` contains one through five ordinary TaskDraft
objects from the bundle format above, including optional complexity and slice.
It cannot contain another proposal.

A split requires `split_ownership` with `retained_parent_paths` and
`child_future_paths`. Each child entry is `{key, paths}`, names every proposed
key exactly once, and has at least one normalized repository-relative future
path. Retained and child paths are exact and pairwise disjoint. Before edits,
the retained list is empty; after edits it equals the complete unstaged
parent-owned path set. Unknown or duplicate fields, a second object, missing or
extra ownership, invalid dependencies, and nested proposals are invalid.

The worker never runs derive commands. The coordinator applies clear direct
work with `zdev tasks derive apply <area> --from -` and no approval. Only
semantic authority uncertainty uses `zdev tasks derive review`, after every
mechanical and current-state gate passes, then `zdev tasks derive review
<area> --show` presents its stored Markdown for manual approval. Apply the
approved opaque identity with `zdev tasks derive apply <area> --reviewed
<review-id>`; do not reconstruct or resend the proposal.
Invalid, unsafe, drifted, staged, incomplete-ownership, and mechanical-failure
states stop; a stored review cannot waive those gates. Derived work never uses
ordinary task import.

A path is accepted for manual use and remains in place after review or import.
For project and pull-request records, add `--commit --format json` to publish
the approved bundle, including the initial split. Personal records keep using
ordinary import. Use ordinary import under any policy when the user wants
uncommitted additions. A later committed import contains only the new task
files, regenerated `TASKS.md`, and the owning area's tracked modified brief when
present. An initial committed import also contains the required project config,
area metadata, brief, and referenced slice briefs. Leave a modified tracked
brief unstaged; zdev validates and commits it with the tasks. The JSON result
includes task IDs, the complete area ready frontier, paths, the commit hash, and
stable change ID. Zdev validates the brief and complete dependency graph before
creating any task file.

Keep shared context, including the required area-wide testing level, in
`brief.md`. For newly drafted implementation tasks, use `context` to select and
connect the task-specific reason, current repository behavior, settled
constraints, and relevant source or test seams. Point to the brief or
background documents instead of copying their shared material. Version 1
bundles and task files without `context` are accepted. Use task
boundaries only for limits the implementation and verification agents need for
that slice. Test-related done conditions and validation must apply the agreed
level, not silently expand it.

`zdev check` requires non-empty `Outcome`, `Done when`, and `Validation` sections.
Done conditions must be checklist items; a completed task must have every item
checked and include a non-empty `Result` section.

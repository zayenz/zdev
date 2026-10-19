# zdev tasks

Manage an area's collection of task files. Use this command to inspect the
queue, review a proposed bundle, import approved tasks, or rebuild the summary
after editing task files.

## Examples

```sh
zdev tasks list greeting
zdev tasks index greeting
```

## Inspect and index

```text
zdev tasks list [AREA]
zdev tasks index AREA
```

Without an area, `list` summarizes available areas. `index` regenerates
`TASKS.md` from individual task files. The task files remain the source of
truth. Use [task](task.md) for operations on one task.

## Review and import

```text
zdev tasks review AREA --from PATH
zdev tasks review AREA --show
zdev tasks import AREA --reviewed REVIEW_ID [--commit]
zdev tasks import AREA --from PATH [--commit]
```

`--from -` reads a JSON bundle from standard input. Review validates the bundle
and stores a rendered review under Git administrative state. After approval,
pass the review's returned identity to `--reviewed`. Direct import uses
`--from` when task creation and import are already authorized.

Import creates a Markdown file for each task and regenerates `TASKS.md`.
Existing files are preserved; conflicting task keys fail. `--commit` commits
the planning records for this import, including a modified tracked area brief.
An initial import can also include the area metadata, brief, slice briefs, and
previously untracked project configuration. Direct imports also accept
`--approval FINGERPRINT` for compatibility.

For an area named `work`, a minimal `tasks.json` is:

```json
{
  "schema_version": 1,
  "area": "work",
  "tasks": [{
    "key": "greeting",
    "title": "Add a named greeting",
    "outcome": "The program accepts --name and keeps its default greeting.",
    "blocked_by": [],
    "done_when": ["Both greeting forms produce the agreed output."],
    "validation": ["Run the program with and without --name."]
  }]
}
```

Import it with `zdev tasks import work --from tasks.json` when authorized.
See [task format](../task-format.md) for fields and dependencies. Approval
Markdown also shows any `requires_code` prerequisites.

## Derive follow-up work

```text
zdev tasks derive review AREA --from PATH
zdev tasks derive review AREA --show
zdev tasks derive apply AREA --reviewed REVIEW_ID
zdev tasks derive apply AREA --from PATH
```

A derived proposal can record follow-up work or split a task discovered during
implementation. Review stores the proposal without applying it. Apply
revalidates current state and creates a managed commit. Use `--reviewed` after
human approval, or direct `--from` when the coordinator has authority for the
proposal. Both review and apply accept `--from -`; direct apply also supports
the compatibility `--approval FINGERPRINT` option.

The [derived-work guide](../derived-work-handoffs.md) defines the proposal
format and when each path applies.

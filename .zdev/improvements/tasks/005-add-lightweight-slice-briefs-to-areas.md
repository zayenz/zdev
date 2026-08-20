+++
schema_version = 1
id = "improvements-005"
key = "slice-briefs"
area = "improvements"
status = "open"
blocked_by = []
+++
# Add lightweight slice briefs to areas

## Outcome

An area can store, validate, and inspect named lightweight slice briefs without adding a second lifecycle.

## Context

Areas currently have one `brief.md` and task files only. Add durable slice briefs under `.zdev/<area>/slices/<key>.md` through the project and CLI seams in `src/project.rs` and `src/lib.rs`. A slice frontmatter record contains the schema version, key, area, and title; its Markdown body contains non-empty `Objective` and `Boundaries`. Area-level Testing remains authoritative. Update `zdev check`, canonical state/workflow instructions, `docs/workflow.md`, `docs/user-guide.md`, generated integrations, and focused black-box coverage.

## Boundaries

- Use exactly `zdev slice create <area> <key> --title <title> --objective <objective> --boundary <text>...`, `zdev slice list <area>`, and `zdev slice show <area> <key>` as the command family.
- Require at least one boundary at creation; validate lowercase segment keys, exact key-to-filename identity, exact schema/key/area/title frontmatter, unknown-field rejection, and non-empty Objective and Boundaries sections.
- Do not store slice status or require every task to belong to a slice.
- Areas without a `slices` directory remain valid and unchanged.
- Do not add slice task membership or derived task counts in this task.

## Done when

- [ ] The slice create command publishes a valid brief atomically and rejects invalid identity or missing required content.
- [ ] Slice list and show expose stable human and JSON representations of every valid slice in an area.
- [ ] `zdev check` validates every present slice brief and accepts existing areas with no slices.
- [ ] Canonical workflow and user documentation describe slice briefs as durable area state, and checked-in harness integrations match their generated sources.
- [ ] Focused black-box tests cover create, list, show, validation failures, and backward compatibility.

## Validation

- Run `cargo test --locked --test lean`.
- Run the repository's standard full validation from the area brief.

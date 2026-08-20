+++
schema_version = 1
id = "improvements-007"
key = "general-area-workflow"
area = "improvements"
status = "open"
blocked_by = []
+++
# Support a lightweight general-area workflow

## Outcome

A user can keep discussed one-off tasks and slices in a conventional `general` area without a separate research or full-brief phase.

## Context

The general area should preserve the existing area and branch model. Treat the tag `general` as a workflow convention, not new metadata: create it through the existing `zdev area create general` command on its ordinary persistent branch, then maintain a standing minimal brief. Update canonical zdev routing, setup and task-shaping instructions, `docs/workflow.md`, `docs/user-guide.md`, and generated integrations so direct requests can discuss and draft concrete general tasks without first running research.

## Boundaries

- Do not add an area-kind field, new lifecycle, dedicated general-area CLI command, automatic branch creation, or automatic branch switching.
- Keep concrete task outcomes, boundaries, done proof, approval, validation, branch safety, independent verification, and commits mandatory.
- Allow both unsliced one-off tasks and lightweight slices once slice support is available, without making slice support a dependency of the basic general workflow.

## Done when

- [ ] Canonical zdev instructions recognize `general` as the conventional home for discussed one-off work and explain how to create its ordinary area and standing brief.
- [ ] The workflow can proceed from discussion to exact task-bundle review without requiring a separate research interaction when no unresolved product choice remains.
- [ ] User documentation distinguishes the lighter planning path from reduced engineering or verification standards.
- [ ] Checked-in harness integrations match their canonical generated sources.

## Validation

- Run `cargo test --locked --test lean checked_in_harness_skills_match_current_templates`.
- Run the repository's standard full validation from the area brief.

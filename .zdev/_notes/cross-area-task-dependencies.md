# Allow task dependencies between areas

Reported 2026-10-28 from the rostering repository (zdev 2.0.0, Claude Code).

## What happened

Rostering split one product restructuring into three areas on shared trunk:
`platform` (shell, grants, roles, master data), `kernel` (scenario model) and
`engines` (optimizer capabilities). The areas have different owners and
validation, but some of their tasks depend on each other:

- `kernel-004` (scenario tables) needs the `Site` model from `platform-009`.
- `kernel-005` (scenario routes) needs the organization-first route tree from
  `platform-002`.
- `kernel-014` (assembled multi-day scenarios) uses the starting-solution
  support from `engines-007` when it exists.
- `platform-012` (hosted release) must wait for `kernel-002` (production demo
  export), because the release migrates production.

`task-format.md` says "Task dependencies never cross areas", and
`validate_graph` in `src/tasks.rs` rejects a blocker ID that is not in the
same area. The bundle validator accepts only existing IDs or keys from the same
bundle.

The workaround was prose. Each dependent task states its prerequisite as an
observable condition in Context, for example "start only when `schema.prisma`
has a `Site` model with `organizationId` (platform task
`sites-and-work-types`)". The independent bundle reviewers flagged the first
draft because it named task keys from another area that did not exist yet. The
workaround has these costs:

- `zdev next` and `work-context` report `kernel-004` as ready before its
  prerequisite exists. An unattended loop selects it and relies on the
  implementer reading Context to stop.
- The release gate (`platform-012` after `kernel-002`) is enforced only by brief
  text. Its failure mode is migrating production before the demo data is saved.
- `TASKS.md` and `zdev status` do not show the dependency, so a human planning
  the next loop cannot see it.

Merging the areas avoids the problem but loses the separate briefs, testing
levels and validation commands that motivated the split.

## Proposal

Allow `blocked_by` to name an existing task ID from another area in the same
project. Task IDs already carry the area prefix, so the reference is unambiguous.

- **Validation.** Bundle review, import and `zdev check` accept a blocker that
  is an existing task ID in any area, and report a missing one with its area.
  Bundle keys stay local to their bundle. Cycle detection runs over the
  combined graph of all areas involved.
- **Readiness.** A task is ready when every blocker, in any area, is done. In an
  isolated area, a blocker from another area counts as done only when its
  completion commit is reachable from this area's branch. Otherwise an isolated
  area could start on work it cannot see. Trunk areas use trunk reachability.
- **Visibility.** `TASKS.md`, `zdev status`, `next` and `work-context` show
  cross-area blockers with their area and state, for example
  `blocked by platform-009 (platform: open)`.
- **Lifecycle.** Closing an area whose open tasks block tasks elsewhere warns,
  or requires an explicit flag. Removing or renaming a task that another area
  depends on is rejected.
- **Documentation.** Replace "Task dependencies never cross areas" in
  `skills/zdev/references/task-format.md` with the rule above. Keep area
  `parent` for branch ancestry; it remains a separate relation.

## Validation

Validation must exercise:

- an import with a cross-area blocker and with a missing one;
- a cycle across two areas;
- readiness on trunk before and after the blocker completes;
- readiness of an isolated area whose blocker is done in another branch but not
  yet reachable;
- `zdev check` after the blocking task file is deleted;
- the generated `TASKS.md` and status output.

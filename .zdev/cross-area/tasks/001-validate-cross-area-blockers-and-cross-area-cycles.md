+++
schema_version = 1
id = "cross-area-001"
key = "validate-cross-area-blockers"
area = "cross-area"
status = "open"
complexity = "standard"
afk = true
priority = "high"
blocked_by = []
+++
# Validate cross-area blockers and cross-area cycles

## Outcome

`zdev tasks review`, `zdev tasks import` and `zdev check` accept a `blocked_by` entry that is an existing task ID in another area of the project, reject a missing one with the dependent task and expected area named, and reject dependency cycles that span areas with the IDs in the cycle.

## Context

Scope, settled decisions and the testing level are in the [brief](../brief.md); the motivating example is in the [field report](../../_notes/cross-area-task-dependencies.md).

Current behavior in `src/tasks.rs`:

- `validate_graph` (about line 2714) rejects any blocker that is not a task in the same area (`Task <id> has missing blocker <blocker>`), and its cycle check reports only one ID (`Task dependency cycle includes <id>`).
- `load_tasks` (about line 2371) runs `validate_graph` on one area. Almost every command, including `zdev check` (`check_output` in `src/lib.rs` -> `tasks::validate_index`), goes through it.
- `allocate_bundle` (about line 1765), used by import, resolves blockers against bundle keys first and then same-area IDs only (`Task <key> has unknown blocker <blocker>`).
- `validate_task_bundle` (about line 296), used by `zdev tasks review`, does not check `blocked_by` at all. Review currently stores a bundle whose blockers import later rejects.

The expected area of a blocker ID is everything before its last `-` (`parse_number` uses `rsplit_once('-')`; area tags may contain hyphens, e.g. `cross-area-001`). Loading another area's tasks must not recurse back into the dependent area's graph validation, so per-area parsing has to be separable from combined-graph validation.

The brief's open question about blockers created on trunk after an isolated area branched is settled: when the dependent area is isolated, the missing-blocker error also suggests `zdev area rebase <area>`, and that hint is enough.

The brief requires this task to confirm that completion records status in the completion commit. Drafting found: `complete_with_checkpoint` (`src/tasks.rs`, about line 3441) sets `status = "done"` in the task frontmatter and, through `publish_task_and_index_with_checkpoint`, rewrites the task file and the area's `TASKS.md`. The Finish step in `templates/zdev/references/implement.md` (about line 170) stages that exact task file and `TASKS.md`, and `zdev commit` (`commit` in `src/lib.rs`, about line 2416) commits the staged paths. `zdev commit` does not itself check task files, so the status change reaches the commit through the documented staging step. Re-read these paths, state the confirmation in the task Result, and stop and report if it does not hold.

Derived proposals have their own blocker check in `validate_derived_proposal` (about lines 816-829, `Derived task <key> has unknown blocker <blocker>`). It accepts only same-area existing IDs and proposed keys. Leave it alone when you extend `validate_task_bundle` and the shared graph validation; it must not start accepting cross-area IDs.

Interim window: until `blocked-queue-and-blocker-display` lands, an area whose open tasks are all blocked by another area fails `zdev check` and project-wide `zdev status` with "Validated task graph has open work but no ready task". Fixtures in this task must therefore keep an independent ready task in every dependent area.

Existing tests that pin error text: `malformed_dependencies_fail_before_any_task_file_is_created` (`unknown blocker`), `malformed_goal_graph_fails_without_mutation` (`missing blocker broken-999`) and `work_context_rejects_unsafe_and_blocked_open_work_with_structured_errors` (`missing blocker general-999`). For fixtures, use `create_area`/`import_one_task` and `--trunk` areas as in `stored_task_review_failures_publish_no_tasks`.

## Boundaries

- Bundle keys stay local to their bundle and take precedence over task IDs; only full task IDs may cross areas. Add no new reference syntax.
- Leave readiness, queue projection and TASKS.md rendering unchanged here: an open cross-area blocker leaves its dependent blocked until `cross-area-readiness-and-index`. Every test fixture keeps an independent ready task in each dependent area, because `zdev check` and project-wide `zdev status` reject blocked-only areas until `blocked-queue-and-blocker-display` lands.
- `requires_code`, hypothesis `context_tasks` and derived proposals keep their same-area rules; do not loosen the derived-proposal blocker check in `validate_derived_proposal`.
- Area `parent`, rebase order and freshness checks are unchanged.
- Keep the existing `unknown blocker` and `missing blocker <id>` substrings in error messages.

## Done when

- [ ] `zdev tasks review` and `zdev tasks import` accept a bundle whose task names an existing task ID from another area, and the imported task file records that full ID in `blocked_by`.
- [ ] `zdev tasks review` and `zdev tasks import` reject a blocker that is neither a bundle key nor an existing task ID; the error names the dependent task, the blocker and its expected area (saying so when that area does not exist); no review is stored and no task file is created.
- [ ] After the blocking task file in another area is deleted, `zdev check` fails naming the dependent task, the missing blocker and its expected area.
- [ ] For a dependent in an isolated area, the missing-blocker error also suggests `zdev area rebase <area>`.
- [ ] A dependency cycle spanning two areas fails `zdev check` and an import into either area, and the error lists the task IDs in the cycle.
- [ ] The task Result states whether `zdev task done` plus the documented staging and `zdev commit` record the blocker's `done` status in the completion commit.

## Validation

- New focused tests in `tests/lean.rs`: review and import with a cross-area blocker and with a missing one; a hand-edited cycle across two areas; `zdev check` after the blocking task file is deleted.
- Existing tests that assert `unknown blocker` and `missing blocker` still pass unchanged.
- `cargo fmt --all -- --check`
- `cargo clippy --locked --all-targets --all-features -- -D warnings`
- `cargo test --locked`
- `cargo build --locked`
- `git diff --check`

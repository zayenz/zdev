+++
schema_version = 1
id = "cross-area-003"
key = "refresh-dependent-indexes"
area = "cross-area"
status = "open"
complexity = "advanced"
afk = true
priority = "high"
blocked_by = ["cross-area-002"]
+++
# Refresh dependent indexes on blocker status changes and guard cross-area reopen

## Outcome

`zdev task done`, `zdev task reopen` and `zdev tasks derive apply` regenerate the `TASKS.md` of every other area in the checkout whose tasks are blocked by the changed task. They write atomically with rollback and report the paths. `zdev task reopen` refuses to reopen a task that a completed task in another area depends on.

## Context

After `cross-area-readiness-and-index`, a change to a blocker's status changes the rendered `TASKS.md` of every dependent area. Decided in bundle review: commands that change task status regenerate those dependent indexes in the checkout. The regenerated files go into the completion commit; `harness-routes-handle-blocked-queue` makes the harness routes stage them. Read the [brief](../brief.md) for scope.

Mutation paths in `src/tasks.rs` that change a task's status:

- `complete_with_checkpoint` (about line 3441) and `reopen` (about line 3495) publish through `publish_task_and_index_with_checkpoint` (about line 3566). It rewrites only the changed area's task file and `TASKS.md`, with rollback, and passes the planned writes to a `checkpoint` callback that research runs use for durable recovery.
- `apply_derived` (about line 1316) sets an investigation follow-up's source task to `Done` (about line 1500). It writes only its own area's `TASKS.md` (about lines 1554-1560), with its own rollback of the source task, index and retained bytes (see `failed_derived_apply_restores_tasks_index_and_retained_bytes`).

Each path regenerates every dependent area's index under the same lock. The dependent writes join the existing rollback and, for the checkpointed path, the checkpoint list. The JSON result reports the regenerated paths. Committed derived apply (`derived_apply_commits_investigation_result_and_children_together`) must include them in its managed commit.

Reopen guard: `reopen` already refuses when a completed task in the same area depends on the task ("Cannot reopen <id>: a completed task depends on it"). Decided in bundle review: it also refuses when a completed task in another area lists the task in `blocked_by`, and the error names that dependent and its area.

## Boundaries

- Regenerate only indexes of areas present in the checkout that have a task blocked by the changed task; leave every other area's files untouched.
- Keep the existing JSON fields and values of `task done`, `task reopen` and `tasks derive apply`; add the regenerated paths as a new field.
- Do not change readiness rules, rendering or queue projection from `validate-cross-area-blockers` and `cross-area-readiness-and-index`.

## Done when

- [ ] `zdev task done` on a blocker regenerates the TASKS.md of each dependent area in the checkout, reports those paths in JSON, and leaves `zdev check` passing without `zdev tasks index`.
- [ ] `zdev task reopen` on a blocker whose dependents in another area are all open regenerates their TASKS.md and reports the paths.
- [ ] `zdev tasks derive apply` with an investigation follow-up whose source task blocks a task in another area regenerates that area's TASKS.md, reports the path, and includes it in the managed commit when committing.
- [ ] A failed write in any of these commands restores every task file and TASKS.md it touched.
- [ ] `zdev task reopen` on a done task refuses when a completed task in another area lists it in `blocked_by`, names that dependent and its area, and leaves every task file and TASKS.md unchanged.

## Validation

- New focused tests in `tests/lean.rs`: dependent index regeneration after `task done` (checked with `zdev check`) and after derived apply; cross-area reopen refusal. Extend an existing rollback test only if rollback of dependent indexes is not otherwise exercised.
- `cargo fmt --all -- --check`
- `cargo clippy --locked --all-targets --all-features -- -D warnings`
- `cargo test --locked`
- `cargo build --locked`
- `git diff --check`

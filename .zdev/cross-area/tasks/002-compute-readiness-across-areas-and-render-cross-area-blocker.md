+++
schema_version = 1
id = "cross-area-002"
key = "cross-area-readiness-and-index"
area = "cross-area"
status = "done"
complexity = "standard"
afk = true
priority = "high"
blocked_by = ["cross-area-001"]
+++
# Compute readiness across areas and render cross-area blockers in TASKS.md

## Outcome

An open task is ready only when every blocker, in any area, has status `done` in the current checkout, through one shared readiness source used by every consumer. Each area's generated `TASKS.md` shows a cross-area blocker as `platform-009 (platform: open)`. A stale dependent index after a blocker changes is reported with `zdev tasks index <area>` as the fix.

## Context

Read the [brief](../brief.md) settled decision "Readiness reads the current checkout": a blocker counts as done when its task file in the checkout being evaluated has `status = "done"`. There is no Git reachability computation. On an isolated branch this matches "the completion commit is reachable", because the completion commit carries the status (confirmed in the Result of `validate-cross-area-blockers`).

Current behavior in `src/tasks.rs`:

- `task_state` (about line 2775) builds its status map only from the area's own tasks. A cross-area blocker is therefore never done.
- Every readiness consumer goes through `task_state(task, tasks)`. They are `next_task`, `views` (tasks list), `summary_selected` (status and queue), `goal_tasks` (goal and work-context), `render_index`, `task_is_ready`, `complete_with_checkpoint` ("Cannot complete task ... it is blocked"), research selection (`research_result`, around line 3350) and the derived-proposal check (`derived_mechanical_ineligibility`, around line 1023). Two returned ready frontiers also use it: `apply_derived`'s (about line 1550) and `allocate_bundle`'s for import (about line 1848). One shared source of blocker status keeps these consistent.
- `render_index` (about line 3609) prints the Blocked by column as comma-joined IDs. `validate_index` (about line 3926) byte-compares the rendered index with the file, and `zdev check`, `area close`/`reopen` and `next` on a closed area call it.

Once state depends on another area, a change to a blocker's status makes the dependent area's `TASKS.md` stale. Decided in bundle review: `refresh-dependent-indexes` makes `task done`, `task reopen` and `apply_derived` regenerate dependent indexes. This task makes the stale-index error name `zdev tasks index <area>` as the fix, which also covers rebases that bring in a blocker's status change. Trunk areas are becoming the default, so occasional rebase conflicts on generated `TASKS.md` in isolated branches are accepted.

Interim window: until `blocked-queue-and-blocker-display` lands, an area whose open tasks are all blocked by another area still fails `zdev check` and `zdev status`. Fixtures keep an independent ready task in the dependent area.

Tests to model on: `task_files_drive_selection_completion_and_generated_summary` for TASKS.md and completion, `managed_rebase_uses_a_parent_area_as_the_effective_base` and `stale_independent_base_is_advisory_for_task_work` for isolated area branches, and `trunk_task_workflows_resolve_config_and_isolate_shared_area_lifecycle` for areas sharing trunk.

## Boundaries

- Readiness reads task files in the current checkout only; do not inspect other branches or commit reachability.
- Same-area blockers render exactly as before, so an existing area without cross-area blockers produces a byte-identical TASKS.md and existing indexes do not go stale on upgrade.
- Regenerating dependent indexes from `task done`, `task reopen` and `apply_derived`, and the cross-area reopen guard, belong to `refresh-dependent-indexes`. Until it lands, tests regenerate a dependent index with `zdev tasks index <area>` after completing a blocker.
- Leave the `blocked` queue projection and blocker annotations in `next`, `work-context`, `goal`, `status` and `tasks list` to `blocked-queue-and-blocker-display`. Tests here observe readiness through `tasks list` state, TASKS.md, `zdev check` and task completion, and keep an independent ready task in the dependent area.
- Do not change area `parent`, rebase or freshness behavior, and do not add a soft-dependency field.

## Done when

- [x] On trunk, a task blocked by an open task in another area is `blocked` in `zdev tasks list`, and `zdev task done` refuses it. After `zdev task done` on the blocker, the dependent is `ready`.
- [x] The ready frontiers returned by `zdev tasks import` and by derived apply count cross-area blockers by their status in the checkout.
- [x] On an isolated area branch whose checkout still has the blocker open (it is done on another branch), the dependent stays blocked. After `zdev area rebase` brings in the completion, it is ready.
- [x] The dependent area's TASKS.md shows the cross-area blocker as `platform-001 (platform: open)` in the Blocked by column and the task's derived state, and `(platform: done)` once the blocker completes and the index is regenerated.
- [x] When a blocker's status changes without its dependent index being regenerated, `zdev check` fails with a message naming `zdev tasks index <area>`, and passes after that command runs.
- [x] Existing TASKS.md assertions in `tests/lean.rs` pass unchanged.

## Validation

- New focused tests in `tests/lean.rs`: readiness on trunk before and after the blocker completes, including the rendered dependent TASKS.md; readiness on an isolated branch where the blocker is done on another branch, before and after rebase, including the stale-index message.
- `cargo fmt --all -- --check`
- `cargo clippy --locked --all-targets --all-features -- -D warnings`
- `cargo test --locked`
- `cargo build --locked`
- `git diff --check`

## Result

Readiness now has one source: a task is ready only when every blocker, in any area, has status done in the current checkout. All readiness consumers (next, tasks list, show, status, goal, task done, research and derived paths, import and derive ready frontiers) use it. TASKS.md renders cross-area blockers as 'platform-001 (platform: open)' and is byte-identical for areas without them; the stale-index error names zdev tasks index <area>.

Validation:

- cargo fmt --all -- --check: pass
- cargo clippy --locked --all-targets --all-features -- -D warnings: pass
- cargo test --locked: pass (lean 196)
- cargo build --locked: pass
- git diff --check: pass
- zdev check on this repository: pass

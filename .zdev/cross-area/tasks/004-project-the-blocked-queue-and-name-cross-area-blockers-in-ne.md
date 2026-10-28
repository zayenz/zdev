+++
schema_version = 1
id = "cross-area-004"
key = "blocked-queue-and-blocker-display"
area = "cross-area"
status = "open"
complexity = "standard"
afk = true
priority = "normal"
blocked_by = ["cross-area-002"]
+++
# Project the blocked queue and name cross-area blockers in next, status and close

## Outcome

When every open task in an area waits on another area, `zdev next`, `work-context`, `goal`, `tasks list` and `zdev status` succeed with queue `blocked` and name each cross-area blocker as `blocked by platform-009 (platform: open)`. A refused `zdev area close` names the open tasks that block other areas. `research run context` reports a research area blocked only by other areas as no-work with its blockers instead of asking for more work.

## Context

The [brief](../brief.md) requires all of these surfaces to treat a task as blocked while any blocker in any area is not done, and to show it with the blocker's area and state.

The `blocked` queue is currently unreachable by design. `docs/area-lifecycle.md` ("`blocked` is diagnostic vocabulary, not a reachable successful projection") and `docs/harness-goals.md` ("There is no successful `blocked` goal queue") both say so. Code depends on that:

- `validate_area_lifecycle` (`src/tasks.rs`, about line 3805) fails with "Validated task graph has open work but no ready task" whenever the queue is `Blocked`. Its callers are `next`, `next_any`, `check_output` (`src/lib.rs`, about line 2339), single-area `status_output_selected` and its project-wide loop (`src/lib.rs`, about line 2275), `goal::render` and `set_area_lifecycle`. All must accept a cross-area `blocked` queue.
- `next` (about line 2965) returns the same error when no task is ready. `next_any` (about line 3051) says "Open task queues are empty or exhausted".
- `goal::render` (`src/goal.rs`) prints the exhausted-queue sentence for every non-ready open queue.
- `work_context_output` (`src/lib.rs`, about line 1761) requires the status and goal queues to agree. Stored-snapshot admission (about line 1599) requires `ready`; that stays.

After `cross-area-readiness-and-index`, a `blocked` queue is legitimate when every open, unready task has at least one cross-area blocker that is not done. The existing error still applies when no open task waits on another area, because that indicates a corrupt graph. Reuse the annotation helper `cross-area-readiness-and-index` added for TASKS.md, so text matches across surfaces.

JSON consumers rely on `blocked_by` being an array of IDs in authored order (`docs/harness-goals.md`, "Exact inputs"). The Claude workflows also parse `queue` and `task_id` strictly. Keep existing fields and values, and add blocker detail (ID, area, status) only as new fields.

Research decision from bundle review: `research run context` (`src/research/run.rs`, about lines 1482-1490) currently tells any area with `summary.ready == 0` to "generate and admit more in-scope supporting work or hypotheses". When every open task in a research area is blocked only by other areas, it must instead report the cross-area blockers and present the state as no-work, not as a prompt to generate work, consistent with loops.

Area close: `set_area_lifecycle` (`src/project.rs`, about line 1113) already refuses to close an area with any open task ("Cannot close area <tag>: N tasks are open"), and `validate_area_lifecycle` enforces "closed => no open tasks". A closed area therefore never holds an open blocker. This supersedes the brief's settled decision "Closing requires an override": close already refuses open tasks, so no override flag is added. Extend the refusal to name each open task that blocks a task in another area, for example `platform-001 blocks kernel-001 (kernel)`. Readiness reads only task status, so reopening a closed area has no effect on dependents. Confirm that, as the brief asks.

Tests to model on: `work_context_returns_open_empty_and_exhausted_contexts`, `goal_handles_empty_unsliced_and_exhausted_states`, `next_any_distinguishes_exhausted_work_from_unsafe_open_work`, `status_reports_wrong_branch_and_a_stale_effective_base_without_failing` and `area_lifecycle_distinguishes_queue_exhaustion_from_explicit_closure`.

## Boundaries

- Selection stays per area: `next`, `next --any`, loops and parallel batches never select or start work in the blocking area on behalf of the blocked one.
- Implementation admission of a stored work context still requires queue `ready`.
- Do not add an override flag to `zdev area close` and do not relax "closed => no open tasks".
- Harness templates, Claude workflow scripts and docs are updated in `harness-routes-handle-blocked-queue` and `document-cross-area-blockers`; keep this task to the CLI and its tests.
- Keep existing JSON field names and values; add new fields only.

## Done when

- [ ] With every open task in `kernel` blocked by an open `platform` task, `zdev next kernel` exits 0 in text and JSON with queue `blocked`, no task, and each blocker shown as `blocked by platform-001 (platform: open)`.
- [ ] `zdev goal kernel` and `zdev work-context kernel` succeed with queue `blocked` and no task, their status and goal projections agree, and the text names the blockers.
- [ ] `zdev status kernel` and project-wide `zdev status` report queue `blocked` and show the cross-area blocker with its area and state.
- [ ] `zdev tasks list kernel` shows the annotation next to each task blocked by another area.
- [ ] `zdev next --any` skips externally blocked areas, and its no-work message distinguishes them from empty or exhausted areas.
- [ ] `zdev check` passes for such an area. An area with no ready task and no cross-area wait still fails with the existing error.
- [ ] `zdev research run <area> context` for an active run whose open tasks are all blocked by another area names the cross-area blockers and no longer advises generating more work. An area with no open tasks keeps the existing advice.
- [ ] `zdev area close platform` refuses while `platform-001` is open and blocks `kernel-001`, names both IDs and the `kernel` area, and leaves `area.toml` unchanged. It succeeds once platform has no open tasks.

## Validation

- New focused tests in `tests/lean.rs`: one assertion each for `next`, `work-context` and `status` on a cross-area blocked queue; `research run context` on a research area blocked only by another area (model on `research_run_admits_mixed_work_recovers_publication_and_refills_an_exhausted_queue` and `start_research_run`); the `area close` refusal followed by a successful close.
- Existing work-context, goal, next and lifecycle tests pass unchanged.
- `cargo fmt --all -- --check`
- `cargo clippy --locked --all-targets --all-features -- -D warnings`
- `cargo test --locked`
- `cargo build --locked`
- `git diff --check`

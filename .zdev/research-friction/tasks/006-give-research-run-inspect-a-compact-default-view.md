+++
schema_version = 1
id = "research-friction-006"
key = "compact-run-inspect"
area = "research-friction"
status = "open"
complexity = "standard"
afk = true
priority = "normal"
blocked_by = []
+++
# Give research run inspect a compact default view

## Outcome

`zdev research run <area> inspect` returns a short routine view by default, and `inspect --full` returns the complete existing projection. `context` and all mutating run actions keep their current full output.

## Context

Brief item 5, second half; feedback section 2, paragraph on the compact snapshot locator. `Action::Context` and `Action::Inspect` in `src/research/run.rs` (~41) both render `context()` (~1473). That projection embeds the whole run (every reservation, admission and extension), the full task list, every experiment checkpoint with attempts and observations, and full baseline samples. With many checkpoints, this output repeatedly truncated coordinator responses.

The Claude research workflow depends on the full `context` shape and on the full context returned by mutating actions (`templates/zdev/claude/workflows/zdev-research.js`, `validRunContext` ~106), so those stay unchanged. The compact work-context projection (`work_context_snapshot_projection` in `src/lib.rs`) is the precedent for a locator view.

The compact view contains:
- run ID and scope
- `stop_reason`
- comparison validity and invalidation reason
- accepted baseline revision and aggregate, without samples
- consumed and remaining limits
- active work: unsettled reservations by request, task and kind, plus unfinished checkpoints by task
- ready and blocked counts and the next ready task
- `next_action`
- any decision or warning fields that context exposes when this task lands

Update "Coordinate each iteration" in `templates/zdev/references/research.md`: use `inspect` for routine checks, and `context` or `inspect --full` when recovery needs checkpoints, lessons or reservations. Regenerate the fixtures.

Other places that change:
- The `Action::Inspect` matches in `run` (`src/research/run.rs` ~256 and ~417), which currently route `inspect` to the same full `context()` path.
- `tests/lean.rs` ~16338-16360 (`research_run_extension_preserves_costs_and_retries_without_double_allowance`) reads `run.id`, `run.started_at`, `run.policy`, `run.reservations` and `run.stop` from `inspect`. Switch it to `context` or `inspect --full`.
- Many tests read `consumed` and `remaining` (`tests/lean.rs` ~17484, 17499, 17535, 17785, 17887, 17897, 17916, 17950, 18000, 18005, 18030), so the compact view keeps both with their current shape.
- `docs/cli/research.md` (~32) describes `context` or `inspect` as the same projection; update it.

## Boundaries

- `inspect --full` is exactly today's projection: the same JSON as `context`.
- Do not change the output of `context` or of any other run action, and do not change run state.
- The compact view summarizes; it must never omit a stop reason, invalidation, unsettled reservation, unfinished checkpoint or the next action.
- Do not change the Claude workflow script. Shared task packets and generated envelopes stay out of scope (brief non-goal).

## Done when

- [ ] The default `inspect` JSON contains only the compact fields, with `consumed` and `remaining` in their current shape. Its size does not grow with checkpoint, observation or task count: no checkpoint attempts, observations, baseline samples or full task list.
- [ ] `inspect --full` equals `context` output for the same state.
- [ ] The text format is a short human-readable summary.
- [ ] The Codex research reference uses compact inspection for routine checks, `docs/cli/research.md` describes both views, the existing extension test reads full state through `context` or `inspect --full`, and the checked-in fixtures are regenerated.
- [ ] One focused regression in `tests/lean.rs` covers the compact and full views.

## Validation

- cargo test --locked --test lean research_run_inspect_defaults_to_compact_view (or the chosen name of the new focused test)
- cargo test --locked --test lean executable_templates_realize_deterministically_and_match_generated_fixtures
- cargo fmt --all -- --check
- cargo clippy --locked --all-targets --all-features -- -D warnings
- cargo test --locked
- cargo build --locked
- git diff --check

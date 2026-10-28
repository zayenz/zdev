+++
schema_version = 1
id = "cross-area-006"
key = "document-cross-area-blockers"
area = "cross-area"
status = "done"
complexity = "routine"
afk = true
priority = "normal"
blocked_by = ["cross-area-005"]
+++
# Document cross-area blockers and record them in the changelog

## Outcome

The canonical task-format reference, the user docs and the changelog describe cross-area `blocked_by`, readiness from the current checkout, the `blocked` queue and the close refusal, and no generated or authored copy still says task dependencies never cross areas.

## Context

The rule to replace is "Task dependencies never cross areas. Use an area parent for branch ancestry and `blocked_by` for task sequencing within one area." Its canonical source is `templates/zdev/references/task-format.md` (line 18). The brief names `skills/zdev/references/task-format.md`, but that file is a generated copy, as are the copies under `.claude/skills/zdev`, `.opencode`, `.pi` and `.omp` (`CONTRIBUTING.md`). The other relevant passages in the same file are the readiness paragraph ("An open task is ready when every blocker is done", about line 671), the `blocked_by` field list (about line 663), the Task bundle field list (about line 713) and the `TASKS.md` note.

User docs that state the old rules:

- `docs/workflow.md` (about line 121): "`blocked_by` models task order inside one area".
- `docs/area-lifecycle.md`: `blocked` is called unreachable, plus the state matrix and the `area close` section.
- `docs/harness-goals.md`: the queue list, "There is no successful `blocked` goal queue", and the malformed-projection paragraph near line 336.
- `docs/area-loop.md`: the stop-state table near line 130.

`docs/derived-work-handoffs.md` keeps excluding cross-area dependencies from derived proposals; leave it. Describe the behavior that `validate-cross-area-blockers`, `cross-area-readiness-and-index`, `refresh-dependent-indexes`, `blocked-queue-and-blocker-display` and `harness-routes-handle-blocked-queue` shipped: read their Results and the current CLI output rather than this draft. Follow the [brief](../brief.md) for the rule wording: full task IDs may cross areas, bundle keys stay local, area `parent` remains branch ancestry, soft prerequisites stay in Context prose, and readiness reads the current checkout. Add an `Unreleased` entry to `CHANGELOG.md` in the existing style. Regenerate the integration copies with the same `cargo run --locked -- skill install ...` commands as `harness-routes-handle-blocked-queue`.

Coordinator clarification (2026-10-28), from findings in `harness-routes-handle-blocked-queue`: two wording gaps belong here. `templates/zdev/references/task-format.md` (about line 518) still says empty or exhausted queues allow research generation without the `blocked` exception; state that a research area whose open work only waits on other areas stops as no-work. The `native_goal` sentence in `src/goal.rs` (about line 37, "Finish when fresh context is closed or the queue is empty or exhausted") does not name `blocked`; add it so the goal condition matches the shipped no-work stop. Update any test that pins that sentence.

## Boundaries

- Edit `templates/zdev/` sources and regenerate; never patch generated copies by hand.
- Change documentation and wording only; no workflow behavior changes. The single exception is the `native_goal` string in `src/goal.rs` named above.

## Done when

- [x] `grep -rn "Task dependencies never cross areas"` over `templates`, `skills`, `.claude`, `.opencode`, `.pi`, `.omp` and `docs` finds nothing, and the canonical task-format reference states the new rule, including that bundle keys stay local and area `parent` remains branch ancestry.
- [x] The task-format readiness paragraph says a blocker in any area counts as done when its task file in the current checkout is `done`, and shows the `blocked by <id> (<area>: <state>)` form.
- [x] `docs/area-lifecycle.md`, `docs/harness-goals.md`, `docs/area-loop.md` and `docs/workflow.md` describe `blocked` as a reachable queue when every open task waits on another area, and describe the shipped close refusal.
- [x] `CHANGELOG.md` has an `Unreleased` entry for cross-area task dependencies.
- [x] The generated integrations match the templates.

## Validation

- No new tests expected; `executable_templates_realize_deterministically_and_match_generated_fixtures` covers the regenerated copies.
- `cargo fmt --all -- --check`
- `cargo clippy --locked --all-targets --all-features -- -D warnings`
- `cargo test --locked`
- `cargo build --locked`
- `git diff --check`

## Result

The canonical task-format reference, regenerated integrations, user docs (workflow, area-lifecycle, harness-goals, area-loop, commands/loop) and CHANGELOG describe cross-area blocked_by, readiness from the current checkout, the reachable blocked queue with cross_area_waits/cross_area_blockers, dependent_indexes, and the reopen and close refusals. native_goal now names a blocked queue. The brief was aligned with the shipped close behavior (no override flag). Independently verified, with one close-guidance correction rechecked.

Validation:

- cargo fmt --all -- --check: pass
- cargo clippy --locked --all-targets --all-features -- -D warnings: pass
- cargo test --locked: pass (clean run, lean 201)
- cargo build --locked: pass
- git diff --check: pass
- zdev skill check (codex portable, claude, opencode, pi, omp): ready

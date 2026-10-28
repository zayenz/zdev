+++
schema_version = 1
id = "cross-area-005"
key = "harness-routes-handle-blocked-queue"
area = "cross-area"
status = "done"
complexity = "standard"
afk = true
priority = "normal"
blocked_by = ["cross-area-004", "cross-area-003"]
+++
# Teach harness routes the blocked queue and dependent index staging

## Outcome

The installed zdev loop, implement, verify, plan, T3 and research routes for every harness treat an open `blocked` queue as a no-work result that names the cross-area blockers. Every completion step, including parallel and research completion, stages the dependent `TASKS.md` files that `zdev task done` regenerates. The checked-in generated integrations are regenerated.

## Context

`blocked-queue-and-blocker-display` makes `queue: blocked` a valid open-area projection `refresh-dependent-indexes` makes `zdev task done` regenerate and report dependent areas' `TASKS.md`. The harness sources under `templates/zdev/` assume neither:

- The Claude workflows accept only `ready`, `empty` and `exhausted`: `templates/zdev/claude/workflows/zdev-loop.js` (`stateFrom`, about line 109) and `zdev-implement.js` (`parseStoredContext`, about line 140). A blocked area currently reaches `unknown` and a confusing BLOCKER.
- `templates/zdev/bounded-area-loop.md` (about lines 40 and 82), `native-area-loop.md` (about lines 49 and 109), `shared-contract.md` (about line 275) and `task-workflows.md` (about line 103) list the no-work states as empty and exhausted.
- `templates/zdev/verify-workflow.md` (about line 25) and `templates/zdev/references/plan-task.md` (about line 12) list closed, empty and exhausted as no-work states. Verify keeps returning `BLOCKER` for `blocked`.
- `templates/zdev/references/t3-code.md` (about line 241) describes open empty/exhausted handling. Another task (`general-024`) is editing this file concurrently, so work from its current version at implementation time.
- Staging after `zdev task done` names only the area's own `TASKS.md`. The prompts and steps are in `templates/zdev/references/implement.md` (about line 172), `templates/zdev/claude-research.md` (about line 171), `zdev-implement.js` (about line 906), `zdev-parallel.js` (about line 446) and `zdev-research.js` (about line 209). They must stage the dependent `TASKS.md` paths that `zdev task done` now reports.
- Research routes treat every no-ready queue as a prompt to generate work: `templates/zdev/claude-research.md` (about line 167) and `templates/zdev/references/research.md` (about lines 42 and 128). Decided in bundle review: a research area blocked only by other areas reports its cross-area blockers and stops as no-work, matching the `research run context` change in `blocked-queue-and-blocker-display`.

Decided in bundle review: loops and implement treat open `blocked` like open `empty`/`exhausted`. They return the ordinary no-work `PASS`, start no worker, leave the area open and report each `blocked by <id> (<area>: <state>)`. Explicit verify returns its usual no-work `BLOCKER`. Following the [brief](../brief.md), there is no cross-area scheduling.

Canonical sources live in `templates/zdev/`; the copies under `skills/`, `.claude/skills/zdev`, `.opencode`, `.pi` and `.omp` are generated (`CONTRIBUTING.md`). Regenerate them with `cargo run --locked -- skill install <harness> --to <dir> --force` for `claude` (`.claude/skills/zdev`), `opencode` (`.opencode`), `pi` (`.pi`) and `omp` (`.omp`), and with `cargo run --locked -- skill install codex --to skills --portable --force`. `executable_templates_realize_deterministically_and_match_generated_fixtures` checks them. Workflow behavior tests to extend: `claude_area_loop_executes_continuation_stop_rework_resume_and_failure`, `claude_implementation_routes_complexity_planning_rework_and_escalation` and `bounded_area_loop_aliases_share_one_stop_and_restart_contract`.

## Boundaries

- Edit only canonical sources under `templates/zdev/` and regenerate the generated copies; never patch generated copies by hand.
- Do not add cross-area selection or scheduling to any route.
- Leave reference docs (`docs/`, `templates/zdev/references/task-format.md`) and CHANGELOG to `document-cross-area-blockers`.
- Keep completion staging explicit: stage only the dependent `TASKS.md` paths that `zdev task done` reports, never whole area directories.

## Done when

- [x] The Claude loop and implement workflows accept an open `blocked` work-context and finish with the ordinary no-work `PASS`, start no worker, and report the cross-area blockers.
- [x] Every installed bounded, native, shared, plan-task and T3 contract lists open `blocked` with empty and exhausted as a no-work state, and explicit verify (`verify-workflow.md` and the verify routes) still returns no-work `BLOCKER` for it.
- [x] The research routes (`claude-research.md`, `references/research.md`) stop as no-work and report the blockers for a research area blocked only by other areas, instead of generating more work.
- [x] The implement Finish step and the completion prompts in `zdev-implement.js`, `zdev-parallel.js`, `zdev-research.js` and `claude-research.md` stage the dependent `TASKS.md` paths reported by `zdev task done`.
- [x] The checked-in generated integrations match the templates, and the fixture test passes.

## Validation

- Extend the Claude loop and implementation workflow tests in `tests/lean.rs` with one open `blocked` work-context case each, and assert the parallel and research completion prompts mention the dependent `TASKS.md` paths where those prompts are already asserted.
- `executable_templates_realize_deterministically_and_match_generated_fixtures` passes.
- `cargo fmt --all -- --check`
- `cargo clippy --locked --all-targets --all-features -- -D warnings`
- `cargo test --locked`
- `cargo build --locked`
- `git diff --check`

## Result

Loop, implement, plan-task, T3 and research routes treat an open blocked queue as a no-work stop that names each cross-area wait; explicit verify keeps its BLOCKER. Every completion route (Claude implement, parallel and research workflows; native implement, task-workflows, parallel and research references) stages each dependent TASKS.md path that zdev task done reports in dependent_indexes. The Claude research workflow stops a run when only cross-area waits remain. Independently verified, with one parallel staging correction rechecked.

Validation:

- cargo fmt --all -- --check: pass
- cargo clippy --locked --all-targets --all-features -- -D warnings: pass
- cargo test --locked: pass (lean 201)
- cargo build --locked: pass
- git diff --check: pass
- zdev skill check (codex portable, claude, opencode, pi, omp): ready

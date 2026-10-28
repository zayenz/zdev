+++
schema_version = 1
id = "research-friction-007"
key = "limit-provenance"
area = "research-friction"
status = "open"
complexity = "advanced"
afk = true
priority = "normal"
blocked_by = []
+++
# Report research limit provenance and count dispatches separately from experiments

## Outcome

Each research run limit reports whether it came from an explicit user constraint, a product default or a coordinator estimate. A time-bounded authorization without an explicit attempt limit no longer silently becomes a 20-dispatch run. Run context reports dispatches by kind separately from measured experiments.

## Context

Brief item 6; feedback section 3.

`start` in `src/research/run.rs` (~534) applies `DEFAULT_ATTEMPT_LIMIT` (20) whenever `--attempt-limit` and `--until-interrupted` are both absent, even when `--time-limit-seconds` is given. `load` (~460) rejects a run that has neither. `attempt_limit` counts every reservation (`limits_reason` ~855 uses `run.reservations.len()`): supporting, preparation, repair, rerun, confirmation, reconciliation and rebaseline dispatches, not experiments. `worker_limit` and `measurement_limit` use clap `default_value_t = 1`, so an explicit 1 is indistinguishable from the default. `Run` stores no origin, and `context` (~1473) reports only `consumed.work_attempts = reservations.len()`.

Reservation accounting was checked as the brief requires. `settle` (~1232) charges actual elapsed time and adds the reserved ceiling only for unknown cost after a crash. `reserve_command` holds the ceiling only while an invocation is live. Supporting reservations record no CPU time at all. The 1,700 vs 51.9 CPU-second gap therefore came from campaign-side envelopes, so settlement does not change.

Settled contract (decided by the user):
- Store an origin per limit: `user` for a supplied flag, `default` for a product default, and `coordinator_estimate` for supplied values the coordinator marks with the repeatable start flag `--estimated <limit>` (for example `--estimated command-seconds`). Older stored runs report `unrecorded`, and extensions keep their limit's origin.
- With `--time-limit-seconds` and no `--attempt-limit`, there is no hard dispatch cap. Run context reports the dispatch count as advisory.
- Context adds dispatch counts by kind and an experiment count, keeping the existing keys. The experiment count is the number of reservations (`Reservation`, `src/research/run.rs` ~213-227) whose kind is not `Rebaseline` and that were bound as measurements (`measurement == true` when bound). `settle` clears `measurement` on completion (~1255-1257), so the count must not read the live flag after settlement. Retain the fact durably, for example with a serde-default `measured` marker that is never cleared. Set it wherever `measurement` becomes true, not only at binding: `bind_work_with_capacity` (~1145 and ~1193) and `claim_measurement` (~1228). Evaluate (`src/research/experiment.rs` ~952) calls `claim_measurement` for reconciled isolated candidates. Older runs without the marker fall back to non-`Rebaseline` reservations with `commands > 0`.
- Templates report origins. The coordinator may raise its own `coordinator_estimate` limits through `extend` without asking the user, because they are its estimates, not user authorization. It must never raise a `user` or `default` limit that way; those still need the user's grant.

`load` (~467) currently rejects `!until_interrupted && attempt_limit.is_none()`. It must accept a run that has a deadline and no attempt limit. `extend` (~640-650) fails with "Cannot extend an unlimited or overflowing count limit" when `--add-attempts` is applied to a run with no attempt limit. That stays, and the templates must say that `extend --add-attempts` fails for a time-only run.

Templates to update: `templates/zdev/references/research.md` (Authorize or resume, step 3, "The default is 20 coordinator work attempts"), `templates/zdev/claude-research.md` (~36-54), and the start-flag mapping in `templates/zdev/claude/workflows/zdev-research.js` (~715). Docs to update:
- `docs/cli/research.md` (~42-45) and `docs/research.md` (~37-39), which repeat the 20-attempt default.
- `docs/task-format.md` (~37), which states the worker and measurement capacity defaults; add their origin there.

Starting test: `research_run_counts_reserved_repairs_and_commands_and_preserves_limits_on_resume`.

## Boundaries

- Do not change `settle`, `reserve_command`, reservation charging or capacity semantics. Never reset, refund or reclassify consumed costs.
- Keep existing CLI flags and JSON keys (`--attempt-limit`, `consumed.work_attempts`, `remaining.*`) so `validRunContext` and existing callers keep working.
- Enforce every limit regardless of origin. A coordinator estimate is still a hard limit until it is extended.
- Runs with neither a time limit nor an attempt limit keep the default of 20 unless `--until-interrupted` is given. Older stored runs still load.
- Change templates, not generated copies, and regenerate the fixtures.

## Done when

- [ ] Start without limits reports `attempt_limit` 20 with origin `default`. Default worker and measurement limits report `default`.
- [ ] `--attempt-limit 5` reports `user`, and `--command-seconds 600 --estimated command-seconds` reports `coordinator_estimate`.
- [ ] A run started with only `--time-limit-seconds` stores no attempt limit and `load` accepts it. More than 20 supporting reservations succeed before the deadline, and run context reports the dispatch count as advisory.
- [ ] Run context reports dispatch counts by kind and an experiment count separately from `work_attempts`. The experiment count covers non-`Rebaseline` reservations marked as measurements wherever `measurement` became true, including binding in `bind_work_with_capacity` and `claim_measurement` for reconciled isolated candidates, and still counts them after settlement.
- [ ] A stored run without origins loads and reports `unrecorded`.
- [ ] The research templates and `docs/cli/research.md`, `docs/research.md` and `docs/task-format.md` explain origins, the time-only rule and dispatch-versus-experiment counts. The templates let the coordinator extend its own estimates without asking, require the user's grant for `user` and `default` limits, and say that `extend --add-attempts` fails for a time-only run. The checked-in fixtures are regenerated.
- [ ] One focused regression in `tests/lean.rs` covers origins, the time-only start, and the separate counts. The counts must include one serial evaluate and one reconciled isolated measurement, both still counted after settlement.

## Validation

- cargo test --locked --test lean research_run_reports_limit_origins_and_separate_dispatch_counts (or the chosen name of the new focused test)
- cargo test --locked --test lean executable_templates_realize_deterministically_and_match_generated_fixtures
- cargo fmt --all -- --check
- cargo clippy --locked --all-targets --all-features -- -D warnings
- cargo test --locked
- cargo build --locked
- git diff --check

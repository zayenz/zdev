+++
schema_version = 1
id = "research-friction-002"
key = "unreachable-gain-warning"
area = "research-friction"
status = "done"
complexity = "standard"
afk = true
priority = "high"
blocked_by = []
+++
# Warn when the frozen minimum gain is unreachable

## Outcome

When research settings declare the best value a metric can reach, zdev detects that the current baseline makes the frozen `minimum_gain` impossible even for a perfect candidate. Research run context presents that as the next research decision instead of more candidate preparation.

## Context

Brief item 2; feedback section 6. `compare` in `src/research.rs` (~730) accepts only when `gain > 0 && gain >= minimum_gain`. Gain is baseline aggregate minus candidate aggregate when minimizing, and the reverse when maximizing. In the Gecode campaign the accepted baseline fell to about 0.0127 s while the absolute gate stayed at about 0.125 s. For a nonnegative metric no candidate could pass, and the coordinator learned this only from repeated rejection.

zdev has no notion of a metric's achievable bound, so it cannot detect this today. Add an optional advisory `metric_bound` to `Settings` in `src/research.rs`: the best value the metric can reach (0 for a nonnegative minimized metric). Keep it outside `comparison` so recorded comparison facts and existing result records are unchanged. The gate is unreachable when the largest possible gain is not positive or is below `minimum_gain`. The largest possible gain is baseline aggregate minus bound when minimizing, and bound minus baseline aggregate when maximizing.

Settled decision: `metric_bound` must never invalidate. Compare settings through one helper that ignores `metric_bound`, and use it at every site that currently compares whole `Settings`:
- `run::configure` `previous != current` (`src/research/run.rs` ~742)
- `require_policy` (~773)
- `comparison_valid` (~1316)
- `context` (~1477)
- the unfinished-checkpoint settings comparisons in `src/research/experiment.rs` (~285 attempt settings versus checkpoint settings, ~2547 and ~2569 in the reconcile/refresh check)

A bound change during an unfinished experiment must not force reconcile or re-measurement.

Where to surface it:
- `context` in `src/research/run.rs` (~1473): add an explicit field, and set `next_action` to this decision unless a stop reason or invalidation takes precedence.
- `project::configure_research` in `src/project.rs`: show the warning in `zdev area research` output.

Document the field in the policy table of `templates/zdev/references/task-format.md`. In `templates/zdev/references/research.md` and `templates/zdev/claude-research.md`, state that this is a study decision for the user: keep the frozen study, end it, or declare a new study with its own threshold and measured baseline. Regenerate the checked-in fixtures as described in `CONTRIBUTING.md` (Canonical harness sources).

The brief's optional predeclared relative-gain rule is excluded. It would change acceptance in `compare`, `paired_sign`, `Comparison::validate`, `policy_matches` and the documented policy, so it is not a small addition to this slice.

## Boundaries

- Never change `minimum_gain`, acceptance in `compare` or `paired_sign`, or any recorded comparison. The warning is advisory only (settled decision): it never refuses candidate preparation, reservations, evaluations or finalization.
- Without `metric_bound`, behavior and output stay as today apart from an absent or null warning field. Existing `area.toml` research tables and stored runs still load.
- Adding or changing only `metric_bound` on an active run must not invalidate the comparison or require a rebaseline. All other settings changes keep their existing rules.
- The warning covers the aggregate minimum-gain rule only. It does not model the paired-sign significance rule.
- Do not add a relative-gain rule.
- Change templates, not generated copies, and regenerate the fixtures.

## Done when

- [x] With a declared bound whose largest possible gain is below `minimum_gain`, `research run <area> context --format json` reports the gate as unreachable with the baseline aggregate, bound and `minimum_gain`, and `next_action` names the study decision.
- [x] When the gate is reachable or no bound is declared, `next_action` is unchanged.
- [x] `zdev area research <area>` reports the same warning for stored settings with an unreachable gate.
- [x] Declaring or changing the bound on an active run leaves `comparison_valid` unchanged.
- [x] Changing only the bound while an experiment checkpoint is unfinished does not force reconcile or re-measurement. That experiment's evaluate, inspect and finalize behave as if the bound were unchanged.
- [x] The task-format and research references document the field and the decision, and the checked-in fixtures are regenerated.
- [x] One focused regression in `tests/lean.rs` covers the warning, the unchanged validity on an active run, and a bound change during an unfinished experiment. For example, `local_experiment_fixture` has median baseline 11 and `minimum_gain` 1, so a bound of 10.5 is unreachable.

## Validation

- cargo test --locked --test lean research_run_reports_unreachable_minimum_gain (or the chosen name of the new focused test)
- cargo test --locked --test lean executable_templates_realize_deterministically_and_match_generated_fixtures
- cargo fmt --all -- --check
- cargo clippy --locked --all-targets --all-features -- -D warnings
- cargo test --locked
- cargo build --locked
- git diff --check

## Result

Research settings gain an optional advisory metric_bound. research run context and zdev area research warn (unreachable_minimum_gain) when even a candidate at the bound cannot meet the frozen minimum gain, and next_action names the study decision. The bound never refuses work and never invalidates comparisons: every whole-settings comparison uses Settings::eq_ignoring_advisory, run-less experiments accept bound-only changes, and keep finalization accepts a snapshot that differs only by the metric_bound line in the area's [research] table while every other byte is still compared exactly. Escalated once to the advanced implementer; independently verified.

Validation:

- cargo fmt --all -- --check: pass
- cargo clippy --locked --all-targets --all-features -- -D warnings: pass
- cargo test --locked: pass (lean 203)
- cargo build --locked: pass
- git diff --check: pass
- zdev skill check (codex portable, claude, opencode, pi, omp): ready

+++
schema_version = 1
id = "research-friction-003"
key = "administrative-policy-amendment"
area = "research-friction"
status = "open"
complexity = "advanced"
afk = true
priority = "high"
blocked_by = []
+++
# Allow declared administrative correctness-command amendments in an active run

## Outcome

Under an active research run, a coordinator can change only the command strings of existing correctness checks by declaring the change administrative with a reason. The run keeps its identity, costs and comparison validity and records the amendment. Every material policy change is still refused or invalidated as today.

## Context

Brief item 3; feedback section 4, second half. `policy_matches` in `src/research/run.rs` (~706) compares `correctness_commands` as exact strings, alongside metric, direction, repetitions, aggregation, `minimum_gain`, `required_checks`, significance and `permitted_changes`. `run::configure` (~721) refuses any mismatch with "Active run freezes...". It is reached from `zdev area research <area> --from` through `project::configure_research` in `src/project.rs` (~1007). In the campaign the correctness commands contained literal admission and preparation paths. Moving a path forced the coordinator to end the run, start another, and carry costs in separate notes.

Evaluator, workload and condition changes already pass `policy_matches`, but they invalidate the comparison until an actual counted rebaseline. Other `policy_matches` callers: `record_policy_compatible` (used by `require_environment` in `src/research/experiment.rs`), `require_policy` and `accepted`.

Settled contract (decided by the user):
- `zdev area research <area> --from <file> --administrative "<reason>"`.
- Accept only when the new settings differ from the run's frozen policy solely in command strings of existing check names.
- Record the reason, check names and before/after strings on the run, show them in run context, and adopt the amended policy without invalidating. Concretely, the amendment skips the `previous != current` invalidation in `run::configure` (`src/research/run.rs` ~742) and sets `run.policy` to the new settings. Then `require_policy` (~773), `comparison_valid` (~1316) and `context` (~1477) see matching settings and do not invalidate on the next call. Follow the existing pattern in `accepted` (~1319-1327), which checks `policy_matches` and then sets `run.policy = settings.clone()`.
- Refuse the amendment while any experiment checkpoint is unfinished or any measurement or rebaseline reservation is unsettled.
- Without the flag, a correctness-command change keeps today's refusal.

The brief keeps the comparison policy frozen during a study and requires a declared transition and new measured baseline for material changes.

## Boundaries

- Never treat a change to any other field as administrative. This includes adding, removing or renaming checks, metric, direction, repetitions, aggregation, `minimum_gain`, significance and `permitted_changes`. Evaluator and workload changes keep their invalidate-and-rebaseline rule.
- zdev must not execute any command during configuration. The amended command runs only at the next evaluate or rebaseline.
- Historical observations, checkpoints and accepted results keep their recorded settings. No amendment can retroactively accept or revalidate anything.
- Outside an active run, `area research` behaves as today. `--administrative` without an active run is rejected with a clear error.
- Stored runs without amendment records still load.
- Update `templates/zdev/references/research.md`, `templates/zdev/claude-research.md`, `docs/research.md` and the policy section of `templates/zdev/references/task-format.md` (~98). An administrative amendment is only for relocations or renamings that keep a check's meaning. A changed horizon, workload, solver or acceptance rule inside a command is a material transition that needs a new study and an actual baseline. Regenerate the checked-in fixtures.

## Done when

- [ ] Under an active run, amending a correctness command string with `--administrative` succeeds. The run ID, consumed costs and `comparison_valid` are unchanged, and run context shows the amendment with its reason.
- [ ] The same change without `--administrative` is still refused. With `--administrative`, adding a check or changing `minimum_gain` is refused.
- [ ] An amendment is refused while an experiment checkpoint is unfinished.
- [ ] The next managed evaluate or rebaseline executes the amended command.
- [ ] The templates, `docs/research.md` and `templates/zdev/references/task-format.md` describe the administrative-versus-material rule, and the checked-in fixtures are regenerated.
- [ ] One focused regression in `tests/lean.rs` covers it, following `research_run_guards_policy_rebaselines_actual_code_and_resolves_shared_failure`.

## Validation

- cargo test --locked --test lean research_run_accepts_declared_administrative_correctness_amendment (or the chosen name of the new focused test)
- cargo test --locked --test lean executable_templates_realize_deterministically_and_match_generated_fixtures
- cargo fmt --all -- --check
- cargo clippy --locked --all-targets --all-features -- -D warnings
- cargo test --locked
- cargo build --locked
- git diff --check

+++
schema_version = 1
id = "research-friction-004"
key = "relevant-input-boundary"
area = "research-friction"
status = "open"
complexity = "advanced"
afk = true
priority = "normal"
blocked_by = []
+++
# Limit accepted-code invalidation to declared relevant inputs

## Outcome

When research settings declare the experiment's relevant inputs, commits that change only other paths no longer invalidate the accepted baseline or block experiment begin. Without a declaration, zdev keeps today's whole-repository comparison. Invalidation reasons name the changed relevant path.

## Context

Brief item 4; feedback section 5. `accepted_code_matches` in `src/research/run.rs` (~684) diffs `baseline_revision..HEAD` over `.` excluding `.zdev`. Callers: `start`, `require_policy`, `comparison_valid`, `context` and `publish_baseline`. A documentation-only commit therefore invalidates the run and forces a counted rebaseline. `require_policy` then records a generic reason (`src/research/run.rs` ~775: "Evaluator/workload or accepted supporting code changed"). This task changes that reason to name at least one changed relevant path, because that is where Done-when observes it. Two other places compare the accepted baseline with HEAD across the whole repository, and they must share the boundary so that the run and begin agree:
- experiment begin's diff in `src/research/experiment.rs` (~488-505)
- `require_accepted_destination` (~2685)

No declaration exists today. Add an optional `relevant_inputs` to `Settings` in `src/research.rs` with a serde default so stored settings stay valid. Entries are repository-relative literal paths (files or directories), unique, outside `.git` and `.zdev`, and the list is non-empty when present. Use literal top-level pathspecs. Document the field in the policy table of `templates/zdev/references/task-format.md` and regenerate the fixtures. Test helpers: `local_experiment_fixture`, `start_research_run`, `research_run_action`, `configure_research_fixture` and `begin_local_experiment` in `tests/lean.rs`.

## Boundaries

- Without `relevant_inputs`, every check stays whole-repository (excluding `.zdev`) exactly as today.
- `relevant_inputs` is frozen material policy. Include it in `policy_matches` so it cannot change under an active run. If task `administrative-policy-amendment` has landed, classify it as material, never administrative.
- Leave candidate-integrity and reconciliation diffs whole-repository in `src/research/experiment.rs`: checkpoint HEAD versus current HEAD (~2550), measured versus returned commit (~2610), and the reconcile/record-only refresh diff (~2784). Leave rebaseline's clean-worktree checks in `src/research/run.rs` (~1550, ~1623, ~1757) whole-repository too.
- Experiment begin and isolated admit must refuse an owned candidate path outside the declared relevant inputs, so measured code always lies inside the boundary.
- No globs and no name-based assumptions about which directories are irrelevant. Never reanalyze or revalidate an earlier invalid measurement.

## Done when

- [ ] With `relevant_inputs` declared, committing a change to an undeclared path (for example a docs file) leaves run context `comparison_valid` true and experiment begin succeeds.
- [ ] Committing a change to a declared input invalidates the comparison, and the invalidation reason recorded by `require_policy` (run context `run.invalidation`) names at least one changed relevant path.
- [ ] Without a declaration, a docs-only commit still invalidates as today.
- [ ] `zdev area research` rejects invalid declarations. Begin and admit refuse owned paths outside a declared boundary.
- [ ] The task-format reference documents the field, and the checked-in fixtures are regenerated.
- [ ] One focused regression in `tests/lean.rs` covers the declared and undeclared cases.

## Validation

- cargo test --locked --test lean research_relevant_inputs_ignore_unrelated_commits (or the chosen name of the new focused test)
- cargo test --locked --test lean executable_templates_realize_deterministically_and_match_generated_fixtures
- cargo fmt --all -- --check
- cargo clippy --locked --all-targets --all-features -- -D warnings
- cargo test --locked
- cargo build --locked
- git diff --check

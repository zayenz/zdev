+++
schema_version = 1
id = "research-friction-005"
key = "generated-output-boundary"
area = "research-friction"
status = "open"
complexity = "advanced"
afk = true
priority = "normal"
blocked_by = ["research-friction-004"]
+++
# Honor declared generated-output paths in research source-ownership checks

## Outcome

Research settings can declare generated-output paths. Rebaseline and the isolated-source ownership checks then preserve and tolerate untracked files under those paths, while tracked changes, undeclared untracked files and declared relevant inputs stay strictly checked.

## Context

Brief item 5, first half; feedback section 2, second bullet. The campaign worktree held about 52,000 untracked capture files.

Rebaseline in `src/research/run.rs` (~1518) already accepts a per-invocation `--untracked-output`, recorded as `Baseline.untracked_outputs`. The coordinator must repeat it on every call, and no other check knows about it.

Isolated-source checks reject every untracked file that the candidate does not own:
- `require_source_delta` in `src/research/experiment.rs` (~2370) lists `ls-files --others --exclude-standard` in the source and fails with "Source has unowned or research record changes".
- Reconcile's cleanliness test (`status --porcelain --untracked-files=all`, ~2680) has the same effect.

As a result, evaluator captures written in a source block return and reconcile.

Add an optional `generated_outputs` to `Settings` in `src/research.rs` (serde default). Validate entries like rebaseline's `--untracked-output` and use literal top-level pathspecs. Rebaseline uses the union of declared and flagged outputs. This task depends on `relevant_inputs` because declared outputs must be rejected when they overlap a declared relevant input. That overlap rule keeps checks strict for captures an experiment actually consumes. Existing coverage: `research_rebaseline_allows_only_explicit_untracked_outputs`. For the isolated-source CLI paths, see `admit_isolated_candidate` and the `codex_rolling_*` tests in `tests/lean.rs`. Document the field in `templates/zdev/references/task-format.md` and regenerate the fixtures.

## Boundaries

- Tolerate only untracked files under declared paths. Tracked modifications, staged changes and untracked files elsewhere still fail with their existing messages. zdev never deletes, cleans or stages declared outputs.
- No implicit outputs: no directory name (for example `results/`) is treated as disposable.
- Reject `generated_outputs` that overlap `relevant_inputs`, and refuse owned candidate paths under a declared output in begin and admit.
- Isolated source admission still requires a clean new source worktree.
- Leave work-context collection and hashing unchanged (settled decision): declared outputs are still hashed, so the verifier's evidence stays complete.
- `--untracked-output` keeps working. Changing `generated_outputs` under an active run invalidates the comparison until an actual rebaseline (settled decision), following the existing rule for non-frozen settings changes.

## Done when

- [ ] With declared outputs, rebaseline succeeds without `--untracked-output` when untracked files exist only under them, and those files keep their bytes.
- [ ] An isolated source with evaluator captures under a declared output can return and reconcile. An undeclared untracked file, or a tracked change under the output path, still refuses.
- [ ] `zdev area research` rejects outputs that overlap relevant inputs, and begin refuses owned paths under a declared output.
- [ ] The task-format reference documents the field, and the checked-in fixtures are regenerated.
- [ ] One focused regression in `tests/lean.rs` covers rebaseline and source return with declared outputs.

## Validation

- cargo test --locked --test lean research_declared_generated_outputs_preserve_captures (or the chosen name of the new focused test)
- cargo test --locked --test lean research_rebaseline_allows_only_explicit_untracked_outputs
- cargo test --locked --test lean executable_templates_realize_deterministically_and_match_generated_fixtures
- cargo fmt --all -- --check
- cargo clippy --locked --all-targets --all-features -- -D warnings
- cargo test --locked
- cargo build --locked
- git diff --check

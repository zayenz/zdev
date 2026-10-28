+++
schema_version = 1
id = "general-032"
key = "pre-tag-2-1-polish"
area = "general"
status = "open"
complexity = "standard"
afk = true
priority = "high"
blocked_by = []
+++
# Name the invalidation reason in read-only research views and drop stale worktree wording

## Outcome

Read-only `research run context` and `inspect` never show `comparison_valid: false` with `invalidation: null`: when they recompute the comparison as invalid, they report the same reason a mutating gate would record, naming the changed path or the settings change. The shared task-workflow contract no longer says ordinary one-task work runs 'normally' in a worktree.

## Context

Found by the final GPT-6 Astra release review of 2.1 (2026-10-28); the maintainer chose to fix both before tagging v2.1.0.

1. src/research/run.rs `live_run` (around line 1764) recomputes `run.comparison_valid` from the frozen policy (`settings.eq_ignoring_advisory(&run.policy)`) and `accepted_code_matches`, but leaves `run.invalidation` as stored. `require_policy` (the mutating gate) records 'Evaluator or workload settings changed; actual rebaseline required' or `input_invalidation(&changed_relevant_paths(..))` ('Accepted relevant input changed since the baseline: <path> (and N more); actual rebaseline required'). After a relevant-input commit, read-only context and inspect (the compact `Inspection` reads the reason around line 2069) therefore show `comparison_valid: false` with `invalidation: null` until a mutating command runs, while CHANGELOG.md [2.1.0] says invalidation names the changed path. Compute the reason in `live_run` with the same rule require_policy uses, without saving, and only when the stored reason is absent; share one helper with require_policy so the two cannot drift.

2. templates/zdev/task-workflows.md line 141 says 'Ordinary one-task work stays in the admitted checkout, normally a worktree under the shared workspace rules.' Since general-025 (workspace.branching, default on-request) the admitted checkout is normally the current checkout. Reword it to follow the shared workspace rules without implying a worktree, and regenerate the rendered copies.

## Boundaries

- Read-only views must not write run state; only the reported reason changes.
- Keep the stored reason when one exists, and keep require_policy's recorded reasons and save behavior unchanged.
- Edit templates and regenerate copies with the repo-built binary (CONTRIBUTING --to forms); never hand-edit generated files.
- Keep the version at 2.1.0; adjust the CHANGELOG [2.1.0] wording only if it needs to match.

## Done when

- [ ] After a commit to a declared relevant input, read-only `research run context` and compact `inspect` report the path-naming reason while comparison_valid is false, and the run file is unchanged; a settings change reports the settings reason.
- [ ] require_policy and live_run use one shared helper for the reason.
- [ ] task-workflows.md and its rendered copies no longer say 'normally a worktree'; `zdev skill check` reports every harness ready.

## Validation

- Extend research_relevant_inputs_ignore_unrelated_commits (or the nearest existing test) to read context and inspect after the relevant-input commit, before any mutating command, and assert the reason and an unchanged run file.
- cargo fmt --all -- --check; cargo clippy --locked --all-targets --all-features -- -D warnings; cargo test --locked; cargo build --locked; git diff --check

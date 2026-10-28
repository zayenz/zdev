+++
schema_version = 1
id = "general-031"
key = "prepare-2-1-0-release"
area = "general"
status = "done"
complexity = "standard"
afk = true
priority = "high"
blocked_by = []
+++
# Prepare the zdev 2.1.0 release

## Outcome

The repository is ready to tag v2.1.0: the package version is 2.1.0, CHANGELOG.md has a complete and accurate [2.1.0] section, users have an upgrade note for changed defaults, version references and generated fixtures are current, and every release check except the clean-tree gate passes on the uncommitted tree.

## Context

All 2.1 work is committed on main since the 2.0.0 release commit ee54046 (see `git log --oneline ee54046..HEAD`). Follow the 2.0 preparation in commits 1fe5a66 and ee54046 for style. Release mechanics are in scripts/check-release.sh (clean tree, Cargo version equals the tag, fmt, clippy, test, `cargo package`, packaged build and smoke test via scripts/release-smoke.sh, dist workflow sync and `dist plan`); `dist` 0.32.0 is installed and matches dist-workspace.toml.

CHANGELOG.md currently has an `## Unreleased` section collected task by task. Known gaps found during the work: there is no entry for research-friction-001 (work-context --show/--compare no longer take the writable state lock; commit 151f917) or research-friction-002 (advisory metric_bound and the unreachable-minimum-gain warning; commit 5e6122f), and the research-friction-005 entry still says reconcile refuses 'when accepted code changes tracked paths under an output' although it now covers overlapping tracked paths, including a replaced ancestor and case-only variants (commit 0cb1c18). Check every commit in the range for a user-visible change without an entry, and leave out test-only and internal changes (for example the probe flush 0bb2b53, the deflake tasks general-029/030).

Default changes users must know about (for the upgrade note): Claude worker defaults now use Claude Opus 5.5 for every role, never max (93fa6d1); `workspace.branching` defaults to on-request, reversing the 2.0 per-implementation worktree default (d84378b); inside T3 Code the T3 route is the default, verifiers default to Codex via `t3.verifier-harness`, and native orchestration is an explicit opt-out (de04b87, 8a61c45); time-only research runs no longer get the 20-dispatch default and `research run inspect` is compact by default with `--full` (cdab226, b3e36c1); cross-area `blocked_by` and the `blocked` queue (cross-area commits). Version strings to update include README.md:40 and docs/getting-started.md:28 (`cargo install zdev --version 2.0.0 --locked`); keep docs/upgrading-to-2.0.md as historical.

## Boundaries

- Do not create the tag, push, or publish anything.
- Do not change product behavior; this is version, changelog, docs and regenerated fixtures only.
- Edit templates only if a version string is templated; regenerate fixtures with the repo-built binary using the CONTRIBUTING --to forms.

## Done when

- [x] Cargo.toml and Cargo.lock declare zdev 2.1.0.
- [x] CHANGELOG.md turns Unreleased into `## [2.1.0] - 2026-10-28` with Added, Changed and Fixed subsections, covers every user-visible change in ee54046..HEAD including the two missing research-friction entries, and has the corrected generated-output wording; entries are plain and non-duplicated.
- [x] A new docs/upgrading-to-2.1.md explains each changed default listed in Context and how to restore the old behavior where a setting exists, and it is linked wherever docs link upgrading-to-2.0.md.
- [x] README.md and docs/getting-started.md install 2.1.0; generated fixtures are regenerated and `zdev skill check` reports every harness ready.
- [x] `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets --all-features -- -D warnings`, `cargo test --locked`, `cargo package --locked --allow-dirty`, and `scripts/release-smoke.sh` against a locally installed packaged build pass.

## Validation

- Report each command's result. The coordinator runs `scripts/check-release.sh v2.1.0` after committing, because it requires a clean tree.
- git diff --check

## Result

zdev is at 2.1.0 (Cargo.toml, Cargo.lock, both plugin manifests). CHANGELOG.md has a complete [2.1.0] section with Added/Changed/Fixed/Compatibility and the compare link, including previously missing entries. New docs/upgrading-to-2.1.md explains each changed default and how to restore it, linked from README, reference, landing page and site sidebar. Install commands use 2.1.0. A packaging blocker was fixed: a test read a package-excluded .omp file. Verified by GPT-6 Astra (high), with one wording correction rechecked.

Validation:

- cargo fmt --all -- --check: pass
- cargo clippy --locked --all-targets --all-features -- -D warnings: pass
- cargo test --locked: pass (213)
- cargo package --locked and packaged test build: pass
- release smoke on packaged install: pass
- git diff --check: pass

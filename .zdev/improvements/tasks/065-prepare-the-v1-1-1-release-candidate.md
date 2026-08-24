+++
schema_version = 1
id = "improvements-065"
key = "prepare-v1-1-1"
area = "improvements"
status = "done"
complexity = "standard"
blocked_by = []
+++
# Prepare the v1.1.1 release candidate

## Outcome

The source tree consistently identifies as zdev 1.1.1 and documents the patch behavior without changing compatibility claims or publishing a release.

## Context

The v1.1.1 audit found the implementation sound after rebasing onto the released v1.1.0 tree, but Cargo.toml, Cargo.lock, and CHANGELOG.md still identify 1.1.0. Update the existing release metadata seams used by scripts/check-release.sh and cargo-dist; this is a patch release for committed initial task bundles, not a broader feature release.

## Boundaries

- Set the package and lockfile version to 1.1.1 using the repository’s normal Cargo workflow.
- Add a concise 1.1.1 changelog entry dated 2026-08-24 and its comparison link, describing committed initial project/pull-request task bundles, local personal behavior, and preserved later-import compatibility.
- Do not tag, push, publish, alter release workflows, or claim qualification before the clean committed candidate is checked.
- Do not fold unrelated post-v1.1.0 changes into the release notes.

## Done when

- [x] Cargo metadata, Cargo.toml, and Cargo.lock consistently report zdev 1.1.1.
- [x] CHANGELOG.md contains an accurate 1.1.1 patch entry and comparison link.
- [x] The release candidate remains based on v1.1.0/current main and retains the verified improvements-064 behavior.
- [x] Formatting, Clippy, tests, build, package listing, generated integration equality, and git diff checks pass before committing the candidate.

## Validation

- Run cargo metadata --locked --no-deps and verify zdev 1.1.1.
- Run cargo fmt --all -- --check, cargo clippy --locked --all-targets --all-features -- -D warnings, cargo test --locked, cargo build --locked, cargo package --locked --allow-dirty --list, and git diff --check.

## Result

Prepared consistent zdev 1.1.1 package, plugin-manifest, and changelog metadata without publishing.

Validation:

- Independent verifier confirmed version consistency, changelog scope, ancestry, generated integration equality, and all required checks.
- Formatting, strict Clippy, all 140 tests, build, package listing, and git diff checks passed.

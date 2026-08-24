+++
schema_version = 1
id = "improvements-066"
key = "qualify-v1-1-1"
area = "improvements"
status = "open"
complexity = "standard"
blocked_by = ["improvements-065"]
+++
# Qualify the v1.1.1 release candidate

## Outcome

A checked-in v1.1.1 qualification record reports the clean candidate’s actual automated release evidence and remaining external limitations, leaving main ready to tag but not publishing anything.

## Context

After the version and changelog candidate is committed, scripts/check-release.sh v1.1.1 is the authoritative clean-tree gate. The 1.1.0 qualification record in docs/release-qualification-1.1.0.md provides the structure, but v1.1.1 needs only proportionate patch-release evidence for the initial-import change and must not copy stale harness claims.

## Boundaries

- Run scripts/check-release.sh v1.1.1 against the clean committed candidate before authoring the record; it must cover locked formatting, Clippy, tests, packaging, install/smoke, integration synchronization, and cargo-dist planning.
- Create docs/release-qualification-1.1.1.md with the date, exact candidate revision and environment, actual automated results, focused improvements-064 coverage, and explicit limitations. Do not claim manual external-harness runs that did not occur.
- Do not modify source behavior, version metadata, release workflows, tags, remotes, or published releases in this task.
- After the documentation commit, the coordinator will rerun the clean-tree release gate on final main as the final readiness check.

## Done when

- [ ] scripts/check-release.sh v1.1.1 passes on the clean committed candidate and its output is accurately summarized.
- [ ] The v1.1.1 qualification record names the exact checked revision, tools/environment, checks performed, focused import-policy and rollback evidence, and omitted external checks.
- [ ] Documentation checks and git diff checks pass with no unsupported release claim.
- [ ] The resulting committed candidate can be fast-forwarded to main without dropping v1.1.0 ancestry or either improvements-064 commit.

## Validation

- Run scripts/check-release.sh v1.1.1 before editing the qualification record.
- Run the focused documentation contract tests, cargo fmt --all -- --check, and git diff --check after writing the record.

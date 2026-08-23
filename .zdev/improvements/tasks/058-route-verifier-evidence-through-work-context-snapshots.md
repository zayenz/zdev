+++
schema_version = 1
id = "improvements-058"
key = "use-snapshots-for-verifier-evidence"
area = "improvements"
status = "done"
complexity = "advanced"
blocked_by = ["improvements-057"]
+++
# Route verifier evidence through work-context snapshots

## Outcome

Verification and completion exchange a compact zdev-resolved post-validation snapshot identity instead of serializing complete Git diffs through worker envelopes and prompts.

## Context

A passing verifier currently JSON-encodes HEAD, status, staged diff, and unstaged diff into evidence, after which Claude parses and reinjects the same bytes. Use the stored snapshot foundation across standalone verification and implementation workflows without weakening independent evidence.

## Boundaries

- A passing verifier returns exactly one stored post-validation snapshot locator in place of inline git_status, git_diff_cached, and git_diff evidence.
- Zdev resolves and validates the locator; no workflow trusts or opens an arbitrary worker-supplied path.
- The verifier still independently captures pre-validation state and proves that validation made no checkout change.
- Completion performs one fresh binary comparison against the accepted snapshot before mutation without loading either full snapshot into its prompt.
- Apply the contract through canonical sources and generated Codex, Claude, OpenCode, Pi, and Oh My Pi artifacts; retain all identity, ownership, rework, task-done, staging, and commit gates.

## Done when

- [x] Standalone verify and implement/rework completion use the compact snapshot locator contract in every supported harness.
- [x] Passing verifier and completion prompts no longer transport raw Git diffs.
- [x] Mismatch, expiration, corruption, wrong area, wrong task, wrong HEAD, and validation-written files block before lifecycle or Git mutation.
- [x] Generated integrations, workflow documentation, and the changelog describe the shipped behavior coherently.

## Validation

- Add focused executable workflow tests for pass, rework, validation writes, malformed or mismatched locators, and pre-mutation blocking.
- Run generation and all five integration install/check tests.
- Run the repository standard full validation.

## Result

Routed verifier PASS and completion through one compact work-context snapshot identity.

Validation:

- Focused locator, validation-write, mismatch, and pre-mutation workflow tests passed.
- All five integration checks, format, clippy, all 135 tests, build, and diff checks passed.

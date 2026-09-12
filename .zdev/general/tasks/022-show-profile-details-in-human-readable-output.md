+++
schema_version = 1
id = "general-022"
key = "show-profile-details-in-human-readable-output"
area = "general"
status = "done"
complexity = "routine"
afk = true
priority = "normal"
blocked_by = []
+++
# Show profile details in human-readable output

## Outcome

The text form of `zdev config profile show NAME HARNESS` displays the resolved settings for every worker role instead of only a heading.

## Context

`src/config.rs::profile_show` already resolves all five roles and returns their complete model, effort, origin, and fallback data in JSON, but its text output is hardcoded to `Profile NAME for HARNESS`. Reuse the resolved values to render useful human-readable rows and add a regression at the existing CLI test seam in `tests/lean.rs`.

## Boundaries

- Show routine implementer, implementer, advanced implementer, planner, and verifier in the existing semantic order.
- Include each role's resolved model and effort or native inheritance, its origin, and any fallback; keep the heading and preserve the JSON contract unchanged.
- Do not change profile resolution, precedence, configuration files, or harness integrations.

## Done when

- [x] `zdev config profile show normal codex` prints the heading followed by concrete details for all five resolved roles.
- [x] Named profiles and inherited or fallback roles are represented accurately in text output.
- [x] The command remains read-only and its JSON output is unchanged.

## Validation

- Add focused CLI assertions for normal and fallback/inherited profile text output at the existing black-box seam.
- Run the focused test, cargo fmt --all -- --check, cargo clippy --locked --all-targets --all-features -- -D warnings, cargo test --locked, cargo build --locked, and git diff --check.

## Result

Showed complete resolved role details in human-readable profile output while preserving JSON and resolution behavior.

Validation:

- Independent verifier passed normal, named, inherited, fallback, semantic ordering, origin, and read-only checks.
- Focused regression, cargo fmt, clippy with warnings denied, all 165 tests, cargo build, and git diff --check passed.

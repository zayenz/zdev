+++
schema_version = 1
id = "general-026"
key = "loop-commit-id-contract"
area = "general"
status = "open"
complexity = "standard"
afk = true
priority = "high"
blocked_by = []
+++
# Make the implement PASS report the full commit ID the loop requires

## Outcome

A verified and committed task reported by the Claude implement workflow is always accepted by the Claude loop. The implement workflow asks for, checks and returns the full 40-character commit ID, so the loop no longer stops with "the one-task PASS omitted its commit ID" after a successful commit.

## Context

Field report, 2026-10-28: in another repository the packaged Claude loop stopped after every task with "PASS omitted its commit ID", although each task had been verified and committed. The user had to relaunch the loop after each task, so unattended loops did not work at all.

Cause: the contracts disagree. `templates/zdev/claude/workflows/zdev-implement.js` (the completion coordinator prompt around line 928, label `complete and commit`) asks for a `Commit ID` field without stating its format. `validPass` (around line 935) only checks that the field exists, and the implement test fixture in `tests/lean.rs` (around line 8733) even uses a short ID, `abc123`. `templates/zdev/claude/workflows/zdev-loop.js` (`plainCommit` at line 49 and the check around line 236) accepts only exactly 40 lowercase hex characters, optionally in backticks. A completion coordinator that reports a short SHA, or the SHA followed by the change ID as `zdev commit` prints it (`Committed <sha> (<change-id>): <subject>`), passes implement and fails the loop. The defect dates from adb7169 (2026-08-22); it is not caused by the 2.1 model-default change. `zdev commit --format json` returns the full SHA as `commit` (src/lib.rs around line 2469).

## Boundaries

- Keep the loop strict: it must only count a commit it can identify by its full 40-character ID. Do not accept short prefixes in the loop.
- Do not add a git or shell call to the loop workflow; the completion coordinator already runs zdev commit and is the right place to report the ID.
- Edit canonical templates and regenerate checked-in copies with the repo-built binary; do not hand-edit generated files.

## Done when

- [ ] The completion prompt in zdev-implement.js states that `Commit ID` is the full 40-character commit SHA reported by `zdev commit` (for example the `commit` field of `zdev commit --format json`), with nothing else on the line.
- [ ] zdev-implement.js rejects a PASS whose Commit ID is not a full 40-character lowercase hex SHA (bare or in backticks), returning a BLOCKER that names the malformed commit ID instead of passing it on.
- [ ] zdev-loop.js accepts the PASS when the Commit ID line holds the full SHA followed by other text, such as a trailing change ID in parentheses, by taking the leading 40-hex token. A short or missing SHA still blocks with the existing message.
- [ ] Any other workflow that parses an implement PASS commit ID (check zdev-goal.js and zdev-parallel.js) follows the same rule.
- [ ] Generated copies match the templates and `zdev skill check` reports every harness ready.

## Validation

- In tests/lean.rs, update the implement fixture that uses `abc123` to a full SHA, and add focused workflow cases: implement rejects a short commit ID; the loop accepts a full SHA with a trailing change ID and still rejects a short one. Use the existing scripted-worker workflow tests; no live harness run.
- cargo fmt --all -- --check; cargo clippy --locked --all-targets --all-features -- -D warnings; cargo test --locked; cargo build --locked; git diff --check

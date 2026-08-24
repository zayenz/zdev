+++
schema_version = 1
id = "general-001"
key = "proportionate-task-challenge-follow-up"
area = "general"
status = "done"
complexity = "routine"
blocked_by = []
+++
# Make task-challenge follow-up proportionate

## Outcome

A revised task bundle is checked by the same reviewer against the prior findings unless the revision materially reshapes the bundle and warrants a fresh full challenge.

## Context

The Create tasks route currently says every changed candidate is challenged again without distinguishing a focused correction from a materially new split. Revise the canonical task-creation guidance and generated harness copies so small corrections receive a focused follow-up on the complete revised artifact, while scope, boundary, dependency, or testing-strategy changes receive a fresh full challenge.

## Boundaries

- Keep independent challenge for non-trivial initial bundles and keep exact user approval of the final stored artifact.
- Do not add review state, revision classifications, counters, or runtime machinery; this is a simple coordinator judgment in guidance.
- Keep the complete revised review document available to the follow-up reviewer so knock-on effects remain visible.

## Done when

- [x] Canonical and generated task-creation guidance distinguishes focused same-reviewer follow-up from a fresh full challenge after material restructuring.
- [x] README and workflow documentation describe the same proportional rule without weakening final bundle approval.

## Validation

- Regenerate the checked-in integrations with `cargo run --locked -- skill install codex --to skills --force`, `cargo run --locked -- skill install claude --to .claude/skills/zdev --force`, and the corresponding `opencode --to .opencode`, `pi --to .pi`, and `omp --to .omp` commands.
- Run `cargo test --locked --test documentation-contract` and `cargo test --locked --test lean executable_templates_realize_deterministically_and_match_generated_fixtures`.
- Run the area-wide validation from brief.md.

## Result

Made task-challenge follow-up proportionate while preserving independent initial review and explicit final approval.

Validation:

- All five checked-in integrations regenerated and matched canonical templates.
- Focused documentation and generated-fixture tests passed.
- Formatting, strict Clippy, full tests, build, diff check, and fresh independent verification passed.

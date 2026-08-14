+++
schema_version = 1
id = "simplify-zdev-text-001"
key = "contract"
area = "simplify-zdev-text"
status = "done"
blocked_by = []
+++
# Establish one rendered zdev contract

## Outcome

templates/zdev/shared-contract.md is the single behavioral source for zdev's core contract, and packaged and installed harness skills are thin rendered wrappers around it.

## Context

The packaged Codex skill and shared-contract.md currently contain materially different rules. Installed Codex uses a wrapper plus the shared contract, while the packaged plugin uses another hand-maintained contract. This makes drift possible and prevents proving that all harnesses receive the same behavior. For example, a core rule should not appear as 'the primary conversation retains authority' in one harness and 'return control before another method' in another. The shared source should instead say: continue with another interaction only when the user already requested it; otherwise report the result, offer relevant next actions, and stop.

## Boundaries

- Preserve activation rules, personal versus project storage, the exact /.zd/ exclusion, integration checks, branch and rebase consent rules, and artifact-specific approval.
- Express the shared transition rule directly: continue to another interaction only when the user already requested it; otherwise report, offer next actions, and stop.
- Do not rewrite the detailed planning methods, execution loop, CLI messages, or user documentation in this task.

## Done when

- [x] The shared contract is the canonical behavioral source.
- [x] Packaged Codex and installed harness skills contain the same rendered contract.
- [x] Regeneration produces no diff.
- [x] Tests assert behavioral invariants rather than incidental prose.

## Validation

- Run focused skill rendering and parity tests.
- Run zd skill check for Claude, OpenCode, and Pi project integrations.

## Result

Established one rendered zdev contract across packaged and installed harness skills.

Validation:

- Focused and full lean tests, formatting, clippy, integration checks, and diff checks passed; fresh Spec and Standards verification returned PASS.

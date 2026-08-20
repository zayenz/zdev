+++
schema_version = 1
id = "improvements-021"
key = "workflow-task-entrypoints"
area = "improvements"
status = "done"
blocked_by = ["improvements-017", "improvements-020"]
+++
# Install common implement and verify workflows

## Outcome

Every supported harness exposes zdev-implement and zdev-verify with deterministic task selection, independent verification, rework, completion, and commit boundaries.

## Context

Complete docs/harness-orchestration.md after the goal command and audit entrypoint establish the shared projection and integration layout. Replace the legacy zdev-task entrypoint with zdev-implement, add zdev-verify, adapt Claude's JavaScript workflow, and render equivalent native Codex, OpenCode, Pi, and Oh My Pi artifacts. Reuse resolved worker profiles and the exact zdev goal JSON as worker context.

## Boundaries

- The coordinator owns selection, branch gates, baseline, overlap decisions, lifecycle changes, and commits.
- Implementers change only task-owned source and tests; every verdict uses a fresh read-only verifier.
- zdev-verify never completes or commits a task.
- Keep the selected task ID stable across goal refreshes and stop on unsafe state, changed focus, invalid envelopes, unavailable independent verification, or user-owned decisions.
- Use no fixed rework count, scheduler, process manager, durable worker ID, or cross-harness session state.
- Remove only the hard-coded legacy zdev-task entrypoint paths during forced migration and preserve unrelated harness files.

## Done when

- [x] All five harnesses install exactly one discoverable zdev-implement and zdev-verify entrypoint under the common public names.
- [x] Implement preflight requires the four area gates, captures complete Git evidence, selects the ready goal task, and rechecks the same task before verification and every rework handoff.
- [x] Implement uses the configured implementer and a fresh configured verifier, routes every concrete REWORK through implementation and full fresh verification, and completes and commits only after PASS.
- [x] Verify requires its explicit task ID to match the current ready goal and returns only PASS zdev-verify, REWORK zdev-verify, or BLOCKER zdev-verify without lifecycle mutation.
- [x] Empty and complete goals return no-work success without delegation; invalid, changed, or unsafe state fails before a worker starts.
- [x] Claude exposes three namespaced plugin workflows and preserves native JavaScript rework behavior; other harnesses use their documented native workers and fallback rules.
- [x] Install and check are deterministic, obsolete zdev-task files are removed safely, and unrelated harness files remain untouched.
- [x] Focused contract tests cover artifact discovery, role selection, pass, rework, invalid envelope, no-work, pre-publication failure, and generated-fixture consistency without adding a harness simulator.

## Validation

- Run focused all-harness workflow artifact, migration, and Claude control-flow tests.
- Run cargo test --locked --test lean.
- Run the all-harness install/check release smoke.
- Run cargo package --locked --allow-dirty.
- Run the repository's standard full validation from the area brief.

## Result

Installed common zdev-implement and zdev-verify entrypoints for all five harnesses with deterministic goal handoffs, configured workers, strict envelopes, independent verification, and safe legacy migration.

Validation:

- Independent verification passed after restoring stale-safe continuation and tightening Claude ready, no-work, status, task, area, Git evidence, advisory, verifier, and final envelope validation.
- cargo test --locked --test lean (91 passed)
- cargo fmt --all -- --check
- cargo clippy --locked --all-targets --all-features -- -D warnings
- cargo test --locked
- cargo build --locked
- all-harness release smoke
- cargo package --locked --allow-dirty
- git diff --check

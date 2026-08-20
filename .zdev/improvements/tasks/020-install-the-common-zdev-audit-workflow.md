+++
schema_version = 1
id = "improvements-020"
key = "workflow-audit-entrypoints"
area = "improvements"
status = "done"
blocked_by = ["improvements-016"]
+++
# Install the common zdev audit workflow

## Outcome

Every supported harness installs one invocable zdev-audit entrypoint with a common checked-findings envelope and native verifier routing.

## Context

Implement the audit slice of docs/harness-orchestration.md through src/integrations.rs and the canonical templates. This task establishes the new multi-entrypoint integration layout and safe migration of known legacy zdev-owned files. Adapt Claude's existing audit workflow; add the Codex, OpenCode, Pi, and Oh My Pi entrypoints in their documented native locations.

## Boundaries

- Audit is read-only and never creates tasks, changes zdev lifecycle state, or commits.
- Use the resolved verifier profile; do not invent an auditor profile.
- Keep fan-out optional for a large boundary or explicit swarm request and require a fresh final evidence-vetting pass.
- Do not add a scheduler, harness simulator, session database, or background-job abstraction.
- Treat Codex's existing zdev skill plus the three sibling workflow skills as one managed shared-root bundle.
- During forced replacement remove only the hard-coded legacy zdev-owned entrypoint paths named by this contract, and preserve every unrelated file under shared harness roots.

## Done when

- [x] Codex, Claude Code, OpenCode, Pi, and Oh My Pi each install exactly one discoverable zdev-audit entrypoint at the documented native path.
- [x] Every adapter accepts PASS zdev-audit, FINDINGS zdev-audit, or BLOCKER zdev-audit and requires boundary, inspected and omitted scope, and located checked evidence.
- [x] Claude retains its native review-and-vet pipeline while the other harnesses express the same contract through native skills, commands, prompts, and workers.
- [x] Missing or invalid worker output fails closed and no adapter claims unchecked findings.
- [x] Install and check agree byte-for-byte, known legacy entrypoints are removed on forced replacement, and unrelated harness files are preserved.
- [x] Focused artifact tests cover discovery, verifier-profile routing, envelopes, deterministic realization, safe legacy migration, and one pre-publication failure without executing fake harnesses.

## Validation

- Run focused all-harness audit artifact and migration tests.
- Run cargo test --locked --test lean.
- Run the all-harness install/check release smoke.
- Run the repository's standard full validation from the area brief.

## Result

Installed one native zdev-audit entrypoint per harness with verifier-only checked findings, deterministic generation, and safe allowlisted legacy migration.

Validation:

- Independent verification passed after making non-forced OpenCode legacy upgrades fail closed and confirming safe forced migration for files, symlinks, directories, and unrelated shared-root content.
- cargo test --locked --test lean (88 passed)
- cargo fmt --all -- --check
- cargo clippy --locked --all-targets --all-features -- -D warnings
- cargo test --locked
- cargo build --locked
- all-harness release smoke
- cargo package --locked --allow-dirty
- git diff --check

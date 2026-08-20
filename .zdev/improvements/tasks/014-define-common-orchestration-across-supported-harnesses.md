+++
schema_version = 1
id = "improvements-014"
key = "orchestration-contract-research"
area = "improvements"
status = "done"
blocked_by = ["improvements-011", "improvements-013"]
+++
# Define common orchestration across supported harnesses

## Outcome

A versioned harness contract gives zdev workflows common public names and behavior while mapping them to each harness's native orchestration facilities and explicit fallbacks.

## Context

Research current native orchestration for Codex, Claude Code JavaScript workflows, OpenCode, Pi, and Oh My Pi, then write `docs/harness-orchestration.md`. Reuse the settled worker-role and goal vocabulary. Cover the existing implement, independent verify, rework, and audit flows, including how native agents, commands, workflows, prompts, or extensions are installed.

## Boundaries

- Do not build a scheduler, cross-harness process manager, session database, or lowest-common-denominator runtime.
- Separate observed versioned capability from proposed zdev adapter behavior and state unsupported features plainly.
- Keep one public zdev workflow name per intent even when installation artifacts differ by harness.

## Done when

- [x] A dated, versioned capability matrix cites primary sources and identifies each harness's native agent and workflow mechanisms.
- [x] The contract settles common names for implement, verify/rework, and audit workflows plus the installed artifact form for every harness.
- [x] It specifies worker-role selection, deterministic goal interaction, delegation boundaries, failure reporting, retry or rework behavior, and fallback behavior.
- [x] It identifies which current Claude Code JavaScript workflows are portable concepts and which behavior remains Claude-specific.
- [x] It ends with narrow per-harness implementation seams and acceptance criteria that do not require another shared product decision.

## Validation

- Check every capability claim against current primary documentation or source code.
- Run `git diff --check`.

## Result

Defined three common zdev workflows with native per-harness artifacts, deterministic goal handoffs, independent verification, and evidence-driven rework without new runtime state.

Validation:

- Independent verification passed against 30 primary links and pinned harness revisions, including current Claude Code plugin workflow distribution and existing zdev retry semantics.
- cargo fmt --all -- --check
- cargo clippy --locked --all-targets --all-features -- -D warnings
- cargo test --locked (78 passed)
- cargo build --locked
- git diff --check

# Changelog

This file records notable changes to zdev.

## [1.1.0] - 2026-08-22

### Added

- Explicit trunk areas for personal and project records. Several areas can
  safely share the configured project trunk while task selection, lifecycle,
  and commits retain exact area and task ownership.
- Lightweight slice briefs, a standing `general` area convention, explicit
  area closure, and `zdev next --any` for deterministic cross-area discovery.
- Read-only `zdev goal` and `zdev work-context` projections, plus installed
  implement, verify, audit, and goal/loop entrypoints for all five harnesses.
- Optional content-addressed work-context files with exact show and compact
  fresh-compare commands for large worker handoffs.
- Layered worker configuration and routine, standard, and advanced task
  complexity with fresh independent verification and bounded escalation.
- Atomic derived-task review and application for authorized investigation
  follow-ups and implementation splits.

### Changed

- Harness integrations now render from one strict canonical template path and
  use typed worker-result envelopes. Small audits use one verifier unless the
  user explicitly supplies bounded review lenses.
- Task imports return their ready frontier, committed imports can include an
  approved area brief, and review identities are opaque coordinator handoffs.
- Task review stores a presentable Markdown document and canonical bundle in
  repository-local Git state, so zdev can show and import the current review
  without reconstructing Markdown or asking users to transport a fingerprint.
- Verifier PASS and completion exchange one stored work-context ID instead of
  serializing raw Git diffs through worker envelopes and prompts; zdev performs
  the fresh comparison before task mutation.
- Safe-but-stale isolated branches remain workable with one advisory; explicit
  trunk areas do not use freshness or rebase ceremony.

### Compatibility

- Existing project, area, task, and slice records remain valid. New optional
  lifecycle, complexity, slice, and area-mode fields are written only when the
  corresponding feature is used.
- Reinstall integrations with `zdev skill install <harness> --force` to receive
  the 1.1.0 commands, workflows, worker profiles, and strict envelope contract.
- Claude Code installs now include native plugin workflows. Codex and Oh My Pi
  use native goal tools when available; OpenCode and Pi report an honest
  one-task continuation boundary.

## [1.0.0] - 2026-08-17

### Added

- A file-based task loop for planning, selecting, completing, and checking work.
- Personal, project, and pull-request policies for storing `.zdev` planning records.
- Stable `Zdev-Change-Id` trailers for commits created with `zdev commit`.
- Managed area relationships, rebasing, recovery guidance, and squash cleanup.
- Reviewed JSON task bundles with approval IDs that bind review to import.
- Native integrations for Codex, Claude Code, OpenCode, Pi, and Oh My Pi.
- Human-readable output and versioned JSON output for scripting.
- Release archives for macOS and Linux on x86-64 and Arm64.

[1.1.0]: https://github.com/zayenz/zdev/releases/tag/v1.1.0
[1.0.0]: https://github.com/zayenz/zdev/releases/tag/v1.0.0

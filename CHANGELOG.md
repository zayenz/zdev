# Changelog

This file records notable changes to zdev.

## [1.2.0] - 2026-08-27

### Added

- Tasks may declare optional `afk` and `priority` metadata. With no focus, zdev
  selects ready work by AFK suitability, priority, then numeric task ID.
- Area loops accept fuzzy focus text. The coordinating model inspects the full
  ready frontier, chooses the best-fitting task, and admits that exact task
  through `zdev work-context <area> --task <task-id>`.

### Changed

- Claude Code workflows pass compact stored work-context locators instead of
  relaying full task and Git context through model output.
- Worker-result parsing accepts one valid JSON object inside brief prose or a
  Markdown fence while still rejecting malformed, truncated, ambiguous, or
  semantically invalid results.
- Worker prompts are concise and role-specific across Claude Code, Codex,
  OpenCode, Pi, and Oh My Pi. Loop progress names task selection, worker stages,
  completed tasks, and commits.
- Reopening a completed task preserves its earlier Result under History instead
  of discarding it or moving it into live Context.

### Fixed

- Focused loops no longer silently fall back to the lowest ready task.
- Long task context and harmless JSON wrapping no longer cause whole-workflow
  retries in Claude Code.
- Compact snapshot handoffs remain readable throughout implementation,
  verification, and rework.
- Installed worker contract paths resolve across user, project, and explicit
  integration destinations.

### Compatibility

- Existing tasks remain valid. Omitted `afk` and `priority` values behave as
  `false` and `normal`, preserving the earlier numeric ordering.
- Existing pending task reviews remain usable when their bundles omit the new
  optional task metadata.
- Reinstall integrations with `zdev skill install <harness> --force` to receive
  the 1.2.0 workflows and worker contracts.

## [1.1.2] - 2026-08-24

### Changed

- Planner workers now return a small, harness-independent semantic result.
  Claude Code and Oh My Pi constrain that result at dispatch, while all
  harnesses share the same strict planner contract without a formatting retry.
- Verifier workers report only semantic conclusions. Coordinators own snapshot
  comparison and public-envelope bookkeeping, and Claude Code avoids redundant
  full-context collection around verification.
- Task-challenge follow-up is proportional to the change instead of requiring a
  full new verifier for every adjustment.

### Fixed

- Claude Code workflows use the rendered inline contract when
  `CLAUDE_PLUGIN_ROOT` is unset or does not point to a readable installed
  contract.
- Claude Code planner validation now rejects duplicate keys inside nested plan
  objects as well as malformed top-level results.

### Compatibility

- Existing records and task state remain valid. Reinstall integrations with
  `zdev skill install <harness> --force` to receive the updated worker
  contracts and harness workflows.

## [1.1.1] - 2026-08-24

### Changed

- Committed initial task imports now publish complete project and pull-request
  records. Personal records remain local, and later imports into existing
  tracked areas retain their previous behavior.

## [1.1.0] - 2026-08-23

### Added

- Explicit trunk areas for personal and project records. Several areas can
  safely share the configured project trunk while task selection, lifecycle,
  and commits retain exact area and task ownership.
- Lightweight slice briefs, a standing `general` area convention, explicit
  area closure, and `zdev next --any` for deterministic cross-area discovery.
- Read-only `zdev goal` and `zdev work-context` projections, plus one installed
  skill that routes implement, verify, audit, and goal/loop work on each of the
  five harnesses.
- Optional content-addressed work-context files with exact show and compact
  fresh-compare commands for large worker handoffs.
- Layered worker configuration and routine, standard, and advanced task
  complexity with fresh independent verification and bounded escalation.
- Atomic derived-task review and application for authorized investigation
  follow-ups and implementation splits.

### Changed

- Derived-task manual review now stores its proposal and Markdown in Git
  administrative state, so coordinators can show and apply an opaque review
  identity without replaying proposal content or carrying a fingerprint.
- Harness integrations now render from one strict canonical template path and
  use typed worker-result envelopes. Route contracts live inside the single
  skill; native commands, prompts, and workflows remain harness adapters. Small
  audits use one verifier unless the user explicitly supplies bounded review
  lenses.
- Task imports return their ready frontier, committed imports can include an
  approved area brief, and review identities are opaque coordinator handoffs.
- Task review stores a presentable Markdown document and canonical bundle in
  repository-local Git state, so zdev can show and import the current review
  without reconstructing Markdown or asking users to transport a fingerprint.
- Verifier PASS and completion exchange one stored work-context ID instead of
  serializing raw Git diffs through worker envelopes and prompts; zdev performs
  the fresh comparison before task mutation.
- Claude task workflows load one rendered contract from the installed plugin
  instead of repeating it in every worker prompt, while keeping the detailed
  role instruction as a same-call fallback.
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

[1.2.0]: https://github.com/zayenz/zdev/compare/v1.1.2...v1.2.0
[1.1.2]: https://github.com/zayenz/zdev/compare/v1.1.1...v1.1.2
[1.1.1]: https://github.com/zayenz/zdev/compare/v1.1.0...v1.1.1
[1.1.0]: https://github.com/zayenz/zdev/releases/tag/v1.1.0
[1.0.0]: https://github.com/zayenz/zdev/releases/tag/v1.0.0

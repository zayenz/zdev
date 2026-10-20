# Changelog

This file records notable changes to zdev.

## [2.0.0] - 2026-10-20

### Changed

- Coding guidance now accompanies planning, implementation and independent
  review. Shared rules cover design, names, contracts, structure and completion;
  focused supplements cover Rust, TypeScript, MiniZinc, Java, Kotlin, Python,
  C++, web behavior, React and React Router framework mode.
- Reviews distinguish required corrections from optional suggestions. Ordinary
  implementation uses one full review, one correction pass and one focused
  recheck. Recovery retains that review stage.
- Explicit blanket approval for compatible parallel batches remains valid
  throughout an ongoing goal or loop, within the approved area and execution
  limits. Each batch still rechecks admission and independently reviews and
  verifies each task.
- Implementation reuses an explicit workspace choice and otherwise uses an
  isolated worktree. Continuing parallel work defaults to four task slots within
  live capacity. Explicit task/import delegation remains valid within its scope.
- Several areas may share a branch. Rebaseline can preserve explicitly declared
  untracked outputs, and snapshots batch untracked-file hashing.
- Research workflow tests use three representative local scenarios instead of
  a broad fault-injection matrix. Tests use scripted workers and never invoke
  an LLM or a live coding harness.

### Added

- `research run <area> extend` adds explicitly granted allowance to an existing
  run without resetting costs, policy, or unfinished work. Request identities
  make retries safe.
- Notes capture candidate work before it becomes an area. Standalone skill
  review works without `.zdev` and explicitly checks naming and decomposition.
- Documentation at [zayenz.se/zdev](https://zayenz.se/zdev/) covers the first task,
  feature and research walkthroughs, agent commands, and the CLI.
- Codex and Claude Code support authorized serial and rolling research with
  measured baselines, correctness checks, retained candidates, bounded work
  and command allowances, and recovery from durable records.
- Codex and Claude Code can coordinate sequential workers through T3 Code.
- The companion area-report skill produces editable MDX and standalone HTML.
- `zdev skill install codex --to <directory> --portable` generates a relocatable
  bundle for Codex plugin distribution. The corresponding `skill check` accepts
  the same flag.

### Fixed

- The report generator locks a compatible `http-cache-semantics` update that
  fixes a cache-response disclosure advisory.
- Rolling source reconciliation preserves both existing-file edits and owned
  additions. Claude rolling recovery reuses current completed measurements.
- Claude parallel acceptance pauses at unresolved destination checkpoints,
  retains concrete profiles between batches, and accepts stale-but-safe work.
- Task approval includes retained-code prerequisites. Dispatch collects fresh
  working-tree evidence once per decision, including retained-plan comparison.
- Public references match delegated task import, default worktrees, and current
  harness support; research links and the manual task-bundle example are fixed.
- `zdev tasks list` without an area prints the available areas as normal output
  and exits successfully, including when no areas exist.
- The normal profile uses GPT-6.1 Sol for routine OpenAI implementation roles,
  matching standard and advanced work. Luna requires the explicit Codex simple
  profile or a worker override. Standard Sol implementation and verification
  use high reasoning.
- Failed derived-task publication removes only files created by that operation,
  preserving existing task files when a generated filename collides.
- Work-context capture and comparison reject dirty tracked submodules instead
  of accepting incomplete evidence of their working contents.
- Rebaseline rejects untracked code created during measurement before accepting
  samples as measurements of the committed revision.
- Parent changes and managed rebases acquire the existing state lock before
  reading and updating area metadata.
- Rolling supporting work uses the ordinary workflow's findings check once,
  preserving work allowances for implementation and verification.
- Standalone Claude verification preserves task complexity when checking the
  resolved worker dispatch.
- Research completion commits use the standard `Zdev-Change-Id` trailer.
- Rolling integration accepts unchanged measured code after actual record-only
  changes without rerunning the evaluator.
- Shared research records remain readable when clone-local experiment refs are
  absent. Admission checks the recorded candidate commit when available and
  reports how to fetch missing retained code.
- Cargo source packages include the research probe needed to compile tests.
  Release checks compile packaged tests and install from the extracted package.

### Compatibility

- Existing 1.x records and worker configuration remain valid. The record schema
  remains version 1. See [Upgrade to 2.0](docs/upgrading-to-2.0.md).
- Refresh binary-installed integrations with
  `zdev skill install <harness> --force` to receive the new guidance and fixes.
  Use the same scope, destination and guidance selection as the existing install.

## [1.6.0] - 2026-09-21

### Changed

- `zdev tasks list` without an area now reports the available areas, their
  lifecycle, and their open-task counts. Open areas appear before closed areas
  in both text and JSON output.

### Fixed

- Verification no longer starts a corrective implementation round when its
  only findings are validation writes and the canonical work context remains
  unchanged. Semantic findings and retained checkout changes still require
  correction.

### Compatibility

- Existing records and worker configuration remain valid. Reinstall
  integrations with `zdev skill install <harness> --force` to receive the
  updated verification behavior.

## [1.5.1] - 2026-09-12

### Added

- Areas can select an execution profile independently for each harness. Area
  profiles take precedence over saved local and global defaults while leaving
  one-off and run-specific profile choices in control.
- Installed harness skills can inspect and change area profiles from a plain
  configuration request when the area and harness are named.

### Fixed

- `zdev config profile show <profile> <harness>` now prints every resolved
  role, including its model, effort, and any fallback source.
- Command help now explains profile precedence, derived-task review and apply
  paths, and the safeguards and recovery steps for squash cleanup. Automated
  coverage includes every reachable help screen.

### Compatibility

- Existing areas without profile mappings keep their previous defaults.
  Reinstall integrations with `zdev skill install <harness> --force` to receive
  the updated configuration route and area-aware profile resolution.

## [1.5.0] - 2026-09-09

### Changed

- Shared prose guidance now incorporates Cursor pstack's `unslop` method. Zdev
  asks every supported harness to preserve meaning while using concrete facts,
  plain active language, readable sentences, and stable repository terms.
- Briefs, slice briefs, tasks, and worker instructions now incorporate Matt
  Pocock's `writing-for-agents` method. Durable records keep shared meaning in
  one place, use branch-specific reading pointers, avoid restating facts an
  agent can inspect cheaply, and define checkable completion conditions.
- Durable planning records and transient worker payloads have separate writing
  rules, removing repeated instructions while preserving the context each
  worker actually loads.

### Compatibility

- Existing records, tasks, and worker configuration remain valid. Reinstall
  integrations with `zdev skill install <harness> --force` to receive the new
  writing guidance.

## [1.4.0] - 2026-09-07

### Added

- Google Antigravity (`agy`) is now a supported harness integration, with
  workspace and user-scoped Skills and custom subagents under Antigravity's
  native `.agents` and `~/.gemini/config` layouts.
- Antigravity worker profiles use Gemini 3.8 Flash with role-specific zdev
  effort levels: low for routine work, medium for implementation and
  verification, and high for planning and advanced implementation.

### Changed

- Antigravity subagents are explicitly sandboxed and marked as subagent-only;
  they cannot be selected as the primary agent.
- Antigravity coordinates zdev workflows through the installed Skill and
  `invoke_subagent`; durable goal state remains in `.zdev`.

### Compatibility

- Existing records and integrations remain valid. Install or refresh the new
  integration with `zdev skill install agy --force`.

## [1.3.0] - 2026-09-06

### Added

- An approved parallel-task route coordinates isolated worktrees, serial
  integration, independent verification, and one durable commit per task. The
  route is available in Codex, Claude Code, Pi, OpenCode, and Oh My Pi, with
  harness-specific fallbacks where native orchestration is unavailable.
- Named execution profiles configure planner, implementer, and verifier model
  settings. Profiles can be selected globally, per harness, or for one command,
  and `zdev plan` exposes the same scoped routing for plan-only work.

### Changed

- Workflow and Git transitions that change the shape of the work now require
  explicit approval, while directly necessary technical adjustments can remain
  within an accepted task's scope.
- Task-writing and worker guidance use plainer, agent-focused language and keep
  actionable blocker context available for recovery.

### Fixed

- `zdev commit --root` commits the selected repository even when inherited
  `GIT_DIR` or `GIT_WORK_TREE` variables point elsewhere.
- Work-context snapshots detect content changes in existing untracked files,
  so validation cannot silently change a new implementation file while its
  snapshot still compares equal.

### Compatibility

- Existing records, tasks, and worker configuration remain valid. Reinstall
  integrations with `zdev skill install <harness> --force` to receive the new
  parallel, planning, profile-routing, and recovery workflows.

## [1.2.3] - 2026-08-31

### Fixed

- Task and planner paths describe expected changes rather than exhaustive
  lists of permitted files. Necessary files with clear task ownership no longer
  turn work within the approved scope into a blocker.
- Goal loops distinguish external or safety blockers from unfinished work that
  can still proceed. Claude Code retries actionable worker blockers and stops
  repeated attempts when snapshot comparison shows no attributable progress.
- Task drafting and focused selection keep unattended repository work ahead of
  work that requires physical devices, credentials, production access, or
  other user involvement.
- Verified, coherent work proceeds through the normal completion and commit
  flow instead of remaining dirty solely because a later attended check exists.
- Verifiers prefer check or dry-run forms for generators, avoiding dirty
  worktrees caused by verification when generation belongs to implementation.
- Claude Code project integrations resolve their installed task-workflow
  contract from the project destination instead of a user-scoped path.

### Compatibility

- Existing records and task state remain valid. Reinstall integrations with
  `zdev skill install <harness> --force` to receive the updated loop,
  task-writing, worker, and verifier contracts.

## [1.2.2] - 2026-08-27

### Changed

- Claude Code uses Haiku for deterministic work-context preflight, snapshot,
  and comparison agents instead of inheriting the session model.
- Verifier instructions explicitly forbid extra semantic keys and require
  validation-written files as `rework` findings across supported harnesses.

### Fixed

- Claude Code accepts completion fields written as blocks, brief prose before
  a status line, and commit IDs wrapped in backticks. These presentation
  differences no longer stop a valid workflow result.

### Compatibility

- Existing records and task state remain valid. Reinstall integrations with
  `zdev skill install <harness> --force` to receive the updated workflows and
  verifier instructions.

## [1.2.1] - 2026-08-27

### Fixed

- Claude Code accepts advanced-task plans that include supporting findings or
  normalized absolute checkout paths. These useful planner results no longer
  fail the workflow's semantic envelope check.

### Compatibility

- Existing records and task state remain valid. Reinstall integrations with
  `zdev skill install <harness> --force` to receive the updated planner
  contract and Claude Code workflow.

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
  new full review for every adjustment.

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
  trunk areas do not use freshness checks or rebasing.

### Compatibility

- Existing project, area, task, and slice records remain valid. New optional
  lifecycle, complexity, slice, and area-mode fields are written only when the
  corresponding feature is used.
- Reinstall integrations with `zdev skill install <harness> --force` to receive
  the 1.1.0 commands, workflows, worker profiles, and strict envelope contract.
- Claude Code installs now include native plugin workflows. Codex and Oh My Pi
  use native goal tools when available; OpenCode and Pi stop after one task and
  report whether more work is ready.

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

[2.0.0]: https://github.com/zayenz/zdev/compare/v1.6.0...v2.0.0
[1.6.0]: https://github.com/zayenz/zdev/compare/v1.5.1...v1.6.0
[1.5.1]: https://github.com/zayenz/zdev/compare/v1.5.0...v1.5.1
[1.5.0]: https://github.com/zayenz/zdev/compare/v1.4.0...v1.5.0
[1.4.0]: https://github.com/zayenz/zdev/compare/v1.3.0...v1.4.0
[1.3.0]: https://github.com/zayenz/zdev/compare/v1.2.3...v1.3.0
[1.2.3]: https://github.com/zayenz/zdev/compare/v1.2.2...v1.2.3
[1.2.2]: https://github.com/zayenz/zdev/compare/v1.2.1...v1.2.2
[1.2.1]: https://github.com/zayenz/zdev/compare/v1.2.0...v1.2.1
[1.2.0]: https://github.com/zayenz/zdev/compare/v1.1.2...v1.2.0
[1.1.2]: https://github.com/zayenz/zdev/compare/v1.1.1...v1.1.2
[1.1.1]: https://github.com/zayenz/zdev/compare/v1.1.0...v1.1.1
[1.1.0]: https://github.com/zayenz/zdev/releases/tag/v1.1.0
[1.0.0]: https://github.com/zayenz/zdev/releases/tag/v1.0.0

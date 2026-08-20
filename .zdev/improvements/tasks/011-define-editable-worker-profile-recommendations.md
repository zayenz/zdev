+++
schema_version = 1
id = "improvements-011"
key = "worker-profile-research"
area = "improvements"
status = "done"
blocked_by = []
+++
# Define editable worker-profile recommendations

## Outcome

A source-backed design defines understandable worker roles, suggested model and effort mappings for every supported harness, and a small editable override contract ready for implementation.

## Context

Research current primary material for DeepSWE, FrontierCode, Artificial Analysis, and official model-control documentation for Codex, Claude Code, OpenCode, Pi, and Oh My Pi. Write `docs/worker-profiles.md`. Public benchmark results seed recommendations; they do not become zdev's own evaluation system. Separate observed evidence from zdev's inference, and record source and access dates because model availability changes.

## Boundaries

- Do not build an evaluator, benchmark runner, task corpus, telemetry, leaderboard synchronization, automatic model selection, or cost database.
- Do not present public benchmark rank as a harness-independent measure of engineering quality.
- Keep the proposed configuration easy to edit by hand and small enough to explain without a configuration framework.

## Done when

- [x] The document cites primary sources with access dates and separates benchmark observations from zdev recommendations.
- [x] It settles harness-neutral role names and the capability or risk each role is meant to cover.
- [x] It supplies dated suggested model and effort mappings for each supported harness, including explicit gaps where a harness cannot express an option.
- [x] It settles override location, precedence, validation, unsupported-value behavior, and fallback behavior.
- [x] It ends with a narrow implementation seam and acceptance criteria that can be turned into an implementation task without another product decision.

## Validation

- Check every cited source and current harness capability against its primary documentation.
- Run `git diff --check`.

## Result

Defined source-backed implementer and verifier profiles, dated harness mappings, and a strict hand-editable worker override contract without evaluation infrastructure.

Validation:

- Independent source verification passed against 17 current primary URLs, including corrected DeepSWE v1.1 evidence and official controls for all five harnesses.
- git diff --check

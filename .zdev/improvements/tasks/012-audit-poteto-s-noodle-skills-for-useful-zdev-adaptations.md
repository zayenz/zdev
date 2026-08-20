+++
schema_version = 1
id = "improvements-012"
key = "poteto-skills-audit"
area = "improvements"
status = "done"
blocked_by = []
+++
# Audit Poteto's Noodle skills for useful zdev adaptations

## Outcome

Zdev has a pinned, source-backed decision for every skill in Poteto's current Noodle inventory: adopt, adapt a specific method, or skip.

## Context

Inspect the actual `.agents/skills` tree in `poteto/noodle` at one pinned commit and update `docs/adapted-methods.md`. Treat the unslop guidance adaptation as already selected so this audit does not duplicate that work. Separate portable instructions and review methods from Noodle-specific schedules, events, worktrees, brain storage, providers, and runtime assumptions.

## Boundaries

- Do not install additional skills, build a skill marketplace, or copy a Noodle orchestration runtime in this task.
- Recommend a follow-up only when the skill adds demonstrated value beyond zdev's existing shape, investigate, implement, verify, and recovery methods.
- Record license status and attribution implications for adopted or adapted material.

## Done when

- [x] The document pins the audited upstream commit and records its license.
- [x] Every skill present at that revision is classified adopt, adapt, or skip with concise rationale and relevant runtime assumptions.
- [x] The audit identifies overlap with existing zdev methods and avoids duplicate concepts under new names.
- [x] Any recommended follow-up names a concrete user benefit and narrow integration boundary; weak candidates remain skipped.

## Validation

- Verify the inventory and cited skill contents against the pinned GitHub revision.
- Run `git diff --check`.

## Result

Audited all 29 skills in the pinned Noodle revision; retained the existing unslop adaptation and skipped 28 overlapping, domain-specific, or runtime-bound skills.

Validation:

- Independent verification matched all 29 upstream skills exactly once, confirmed rationales and zdev overlap, and verified MIT and nested Apache-2.0 licensing notes.
- git diff --check

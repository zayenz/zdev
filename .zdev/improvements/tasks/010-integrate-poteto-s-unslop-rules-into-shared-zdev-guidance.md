+++
schema_version = 1
id = "improvements-010"
key = "integrate-unslop-guidance"
area = "improvements"
status = "open"
blocked_by = ["improvements-009"]
+++
# Integrate Poteto's unslop rules into shared zdev guidance

## Outcome

Every supported harness receives the same attributed prose-quality guidance as part of zdev itself, without installing or advertising a separate `unslop` skill.

## Context

Adapt the contents of the canonical `unslop` skill from `poteto/noodle`, not the skills.sh copy, into zdev's shared guidance templates. Pin the upstream commit reviewed during implementation, distinguish adapted upstream material from original zdev instructions, and record Lauren Tan's MIT attribution in `docs/adapted-methods.md`. The result guides agents while they compose or revise human-facing zdev prose; it is not a runtime text-rewriting feature.

## Boundaries

- Apply the adaptation only to human-facing prose and preserve user quotations and source text.
- Never rewrite code, commands, paths, literals, JSON, TOML, YAML, frontmatter, generated records, or already approved task content.
- Keep semantic accuracy, repository terminology, explicit user instructions, area briefs, slice briefs, and task contracts authoritative over style preferences.
- Add no separate unslop skill, inventory entry, install destination, discovery mechanism, command, automatic post-processing step, or independent lifecycle.
- Do not import Noodle events, schedules, brain files, worktree behavior, or unrelated skills; upstream updates remain manual reviews against a newly pinned revision.

## Done when

- [ ] The pinned upstream commit, source URL, MIT license, and adaptation note are recorded, including which guidance is adapted and which is original zdev material.
- [ ] Realized integrations for Codex, Claude Code, OpenCode, Pi, and Oh My Pi contain the shared prose guidance exactly once.
- [ ] No supported harness installs, discovers, or advertises a separate `unslop` skill.
- [ ] Focused tests assert the prose-only scope, explicit exclusions, attribution, and generated-integration consistency.
- [ ] Existing install, check, and package validation passes through the canonical MiniJinja realization path.

## Validation

- Run `cargo test --locked --test lean`.
- Run the repository's standard full validation from the area brief.

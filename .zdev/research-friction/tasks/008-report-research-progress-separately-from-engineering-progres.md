+++
schema_version = 1
id = "research-friction-008"
key = "research-progress-report"
area = "research-friction"
status = "open"
complexity = "standard"
afk = true
priority = "low"
blocked_by = []
+++
# Report research progress separately from engineering progress

## Outcome

The research route templates tell the coordinator to report research progress separately from engineering progress (supporting tasks, commits and verification). Research progress covers questions answered, valid comparisons, negative findings, the current accepted result and the next useful experiment.

## Context

Brief item 7; feedback section 8. The report instructions are in two places:
- "Stop and recover" in `templates/zdev/references/research.md` ("Report the run identity, scope, ...").
- `templates/zdev/claude-research.md` (~230: "Report run identity/scope, completed ordinary tasks and hypothesis decisions/commits...").

Both list counts and decisions, mixing supporting work with measured results. As the report notes, a large task count or a retained isolated screen win does not establish an eight-worker gain.

Data sources the coordinator can already read: hypothesis results (`zdev task research <area> <task> --format json`: decision, lesson, observations), run context `accepted_baseline`, and rejected, inconclusive and failed-to-run lessons. `templates/zdev/codex-skill.md` mentions the research lifecycle (~19, ~122); change it only if it carries a report list. The brief sets the testing level: template-only, no new tests.

## Boundaries

- Change template prose only: `templates/zdev/references/research.md`, `templates/zdev/claude-research.md`, and `templates/zdev/codex-skill.md` only if needed. No Rust changes. No change to the report JSON or `validRunContext` in `zdev-research.js`.
- Do not present a supporting task, a rejected candidate or an isolated screen win as a measured improvement. Only comparisons accepted under the frozen policy count as accepted results. Report invalid measurements as invalid.
- Keep the existing stop-reason and objective-lifecycle wording, which the brief lists as already reconciled.
- Regenerate the checked-in fixtures; do not patch generated copies.

## Done when

- [ ] The report instructions of both research routes list research progress separately from engineering progress. Research progress covers questions answered with their decision, valid comparisons, negative findings with lessons, the current accepted result and the next useful experiment.
- [ ] The checked-in fixtures are regenerated, and the existing template and fixture checks pass without new tests.

## Validation

- cargo test --locked --test lean executable_templates_realize_deterministically_and_match_generated_fixtures
- cargo test --locked --test lean codex_and_claude_install_separate_serial_research_routes_with_resolved_contracts
- cargo fmt --all -- --check
- cargo clippy --locked --all-targets --all-features -- -D warnings
- cargo test --locked
- cargo build --locked
- git diff --check

+++
schema_version = 1
id = "improvements-004"
key = "advisory-base-staleness"
area = "improvements"
status = "done"
blocked_by = []
+++
# Make base staleness advisory during ordinary task work

## Outcome

A task can be selected and completed when its effective base has advanced but its recorded relationship remains safe, while zdev reports one concise, actionable rebase advisory.

## Context

Today `require_fresh_area_link` hard-blocks both `zdev next` and `zdev task done`, and the canonical implement, verify, and recovery instructions require freshness before work. This repeatedly interrupts a child area such as `bar` whenever its active parent `foo` advances, even when the child anchor and history remain valid. Reuse the branch-health facts in `src/project.rs`; update the task commands in `src/tasks.rs`, status rendering in `src/lib.rs`, canonical workflow sources under `skills/zdev` and `templates/zdev`, generated integrations, focused black-box tests in `tests/lean.rs`, and the relevant workflow documentation.

## Boundaries

- Do not switch branches, run a rebase, resolve conflicts, or advance an anchor automatically.
- A stale relationship is advisory only when the recorded anchor exists in the area branch, child history after it is linear, both branches exist, ancestry is inspectable, the checked-out branch matches, and no Git recovery operation is active.
- Keep detached or wrong branches, missing branches, missing or uncontained anchors, nonlinear history, unavailable ancestry, and active rebase, merge, cherry-pick, revert, bisect, or sequencer state as blockers.
- Keep `zdev area rebase` as the explicit way to incorporate a newer base.

## Done when

- [x] `zdev status`, `zdev next`, and `zdev task done` distinguish stale-but-safe relationships from unsafe branch state in human and JSON output.
- [x] Human output gives at most one concise `zdev area rebase <area>` advisory, while JSON exposes structured branch diagnostics.
- [x] Canonical implement, verify, and recovery workflows continue after reporting stale-but-safe state once instead of asking for rebase consent.
- [x] Focused tests cover advancing an independent base, advancing a parent area, and at least one unsafe state that still blocks.
- [x] Checked-in harness integrations match their canonical generated sources.

## Validation

- Run `cargo test --locked --test lean`.
- Run the repository's standard full validation from the area brief.

## Result

Allowed independently verified task work to continue on stale-but-safe base links while preserving explicit rebase control and hard unsafe-state gates.

Validation:

- Focused independent-base and parent-base behavior tests passed.
- cargo test --locked --test lean passed (65 tests).
- Formatting, clippy with warnings denied, full tests, locked build, and git diff checks passed.

# General improvements

## Objective

Improve zdev through useful, well-engineered changes that remain clear, maintainable, appropriately tested, and demonstrably valuable.

Success is observable in each completed task: it solves a concrete user or
maintenance problem, leaves the affected code easier to understand and change,
and supplies evidence appropriate to the risk of the change.

## Boundaries

- Keep each improvement narrow enough to implement and verify independently.
- Preserve documented behavior and compatibility unless an approved task says
  otherwise.
- Do not introduce abstractions, test infrastructure, or broad rewrites without
  a concrete problem that justifies them.
- This area is an ongoing home for general improvements; its task list may grow
  as specific, reviewed opportunities arise.

## Settled decisions

- A task must state why its result is useful. Generic cleanup and coverage goals
  are not sufficient outcomes.
- Prefer simple code with clear domain boundaries over additional framework or
  indirection.
- Treat checked-in harness integrations and `TASKS.md` as generated artifacts;
  change their sources and regenerate them through the established commands.

## Testing

Focused coverage. Use testing proportional to the change:

- Add or update focused regression coverage when behavior changes or a defect
  is fixed.
- Use the existing black-box suite for behavior-preserving refactors; do not add
  tests that merely assert that code moved.
- Use documentation, packaging, or release checks when those are the affected
  artifact.

Every implementation must pass the focused checks relevant to it. Before task
completion, run the repository's standard full validation unless the task
records a concrete reason that a check is unavailable.

## Validation

- `cargo fmt --all -- --check`
- `cargo clippy --locked --all-targets --all-features -- -D warnings`
- `cargo test --locked`
- `cargo build --locked`
- `git diff --check`

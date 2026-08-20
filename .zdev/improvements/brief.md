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
- Organize larger improvements as lightweight slices inside an area. A slice
  has a short brief, may group several tasks, and derives its progress from
  those tasks instead of maintaining a second lifecycle.
- Use the conventional tag `general` for an ordinary area with a standing
  minimal brief for one-off tasks and slices. Do not add a separate area kind or
  lifecycle. Those items still need concrete outcomes and useful validation,
  but they do not require a separate research phase.
- Preserve area-specific task selection and add an explicit project-wide mode
  for requests to work on any ready task. Selection must report the owning area
  and branch; it must not hide branch changes.
- Treat a stale link to an area's parent or trunk as advisory while its recorded
  anchor, branches, ancestry, and linear child history remain valid and no Git
  recovery operation is active. Keep managed rebasing explicit and recommend it
  when work needs newer base changes or approaches an integration boundary.
- Define worker levels as editable, harness-specific recommendations seeded
  from published coding-agent evidence. Zdev will not run model evaluations or
  maintain an evaluation framework.
- Use one common zdev vocabulary for goals and orchestration, rendered into
  each harness's native mechanisms. Keep canonical skill and workflow sources
  as unexpanded templates and realize them during installation.
- Integrate focused external skills only when they improve zdev without
  importing another tool's runtime assumptions. Poteto's `unslop` skill is the
  first intended core addition; other Noodle skills require individual review.

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

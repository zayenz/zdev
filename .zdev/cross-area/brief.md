# Task dependencies across areas

## Objective

A task's `blocked_by` may name an existing task ID from another area in the
same project. zdev then treats that task as blocked until the blocker is done,
shows the blocker with its area and state, and refuses lifecycle changes that
would leave dependents permanently blocked.

Source: [field report from the rostering repository](../_notes/cross-area-task-dependencies.md).
Read it for the motivating example (`platform`, `kernel` and `engines` areas
and the production-release gate).

## Observable success

- Bundle review, import and `zdev check` accept a blocker that is an existing
  task ID in any area and reject a missing one, naming the expected area.
- Cycle detection covers every area reached through cross-area blockers; a
  cycle spanning two areas is rejected with the IDs involved.
- `zdev next`, `work-context`, `tasks list`, `zdev status` and the generated
  `TASKS.md` treat a task as blocked while any blocker, in any area, is not
  done, and show it as for example `blocked by platform-009 (platform: open)`.
- `zdev area close` refuses an area whose open tasks block tasks in another
  area and names those dependents.
- `skills/zdev/references/task-format.md` (canonical template) states the new
  rule, and the generated copies are regenerated.

## Boundaries and non-goals

- Bundle keys stay local to their bundle. Only full task IDs may cross areas.
- Area `parent` remains the branch-ancestry relation; cross-area blockers do not
  change branch parents, rebase order or freshness checks.
- Soft prerequisites ("uses X when it exists") stay in task Context prose. Do not
  add a soft-dependency field.
- No project-wide scheduler or cross-area loop. Loops and parallel batches still
  work one area at a time; they only see more accurate readiness.

## Settled decisions

- **Task IDs are unambiguous.** IDs carry the area prefix, so `platform-009`
  resolves to `.zdev/platform/tasks/` without new syntax.
- **Readiness reads the current checkout.** A blocker counts as done when its
  task file in the checkout being evaluated has status `done`. Task completion
  commits the status change, so on an isolated branch this is the same as "the
  blocker's completion commit is reachable from this branch". On trunk it
  follows trunk. The first task must confirm that completion writes status in
  the completion commit; if it does not, stop and report before building on
  this assumption.
- **Closing refuses open blockers.** `zdev area close` already refuses an area
  with open tasks, so no override flag is needed. The refusal names each open
  task that blocks another area. This supersedes the earlier override decision.
- **Deleting or renaming a blocker is caught by `zdev check`.** A missing blocker
  fails validation with the dependent task and expected area named. zdev has
  no task-rename command to guard separately.

## Open questions

- A blocker created on trunk after an isolated area branched is absent from that
  branch until it is rebased. Import there fails as "missing blocker". The
  error should suggest rebasing the area; decide in task review whether that
  hint is enough.
- Reopening a closed area: it has no effect on dependents, since readiness only
  looks at task status. Confirm during implementation; no extra rule expected.

## Testing

Focused coverage in `tests/lean.rs`, following the existing area and task
fixtures. Cover:

- import with a cross-area blocker, and with a missing one;
- a cycle across two areas;
- readiness on trunk before and after the blocker completes;
- readiness on an isolated branch where the blocker is done on another branch
  but not yet in this branch;
- `zdev check` after the blocking task file is deleted;
- generated `TASKS.md` and `zdev status` output for a cross-area blocker;
- `area close` refusal naming the dependents.

Do not add a broad matrix over every command; one assertion per surface is
enough when the commands share a readiness function.

## Validation

- `cargo fmt --all -- --check`
- `cargo clippy --locked --all-targets --all-features -- -D warnings`
- `cargo test --locked`
- `cargo build --locked`
- `git diff --check`

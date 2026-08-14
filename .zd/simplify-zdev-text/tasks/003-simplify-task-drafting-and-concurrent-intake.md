+++
schema_version = 1
id = "simplify-zdev-text-003"
key = "tasking"
area = "simplify-zdev-text"
status = "done"
blocked_by = ["simplify-zdev-text-001"]
+++
# Simplify task drafting and concurrent intake

## Outcome

Create-tasks guidance gives a direct drafting, challenge, approval, and import workflow, with committed additions as the normal rule for an existing queue.

## Context

Create-tasks guidance currently spends substantial attention on reviewer grouping, model strength, reasoning effort, conversational roles, and disclosure when delegation is unavailable. The useful rule is simpler: for non-trivial work, ask a fresh read-only reviewer to challenge missing context, hidden decisions, unobservable completion, scope errors, and false dependencies; reconcile the suggestions against repository evidence. Task intake should also be determined by queue state, not chat identity. The initial split uses ordinary import. Adding tasks to an existing queue uses zd tasks import <area> --from - --commit --format json. A commit containing only new task files and regenerated TASKS.md does not interrupt the selected task. The implementation chat keeps its current selection and considers the additions at the next zd next.

## Boundaries

- Preserve the task schema, required headings, ready ordering, dependency validation, lifecycle rules, and exact fenced Markdown approval.
- Require fresh approval whenever the displayed bundle changes.
- For non-trivial work, request a fresh read-only planning challenge; perform the same check locally if delegation is unavailable.
- Use ordinary import for the initial split. For additions to an existing queue, use zd tasks import <area> --from - --commit --format json unless the user explicitly wants uncommitted additions.
- Preserve exact-path commit isolation and recovery diagnostics.

## Done when

- [x] Drafting guidance focuses on missing context, hidden decisions, observable completion, scope, and genuine dependencies.
- [x] Existing-queue additions have one deterministic command.
- [x] New task-only commits do not interrupt the selected task and are considered at the next zd next.
- [x] Approval and harness-parity tests cover the exact rendered workflow.

## Validation

- Run focused task-format, approval, import, and harness-parity tests.
- Exercise initial and additive import paths.

## Result

Simplified task drafting, review, approval, and concurrent intake guidance.

Validation:

- Focused approval, import, rollback, and harness tests plus formatting, clippy, full tests, and diff checks passed; fresh verification returned PASS.

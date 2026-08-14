+++
schema_version = 1
id = "simplify-zdev-text-005"
key = "cli"
area = "simplify-zdev-text"
status = "done"
blocked_by = []
+++
# Make CLI messages action-oriented

## Outcome

CLI help, normal output, and recoverable errors lead with what happened and what the user should do next.

## Context

Several CLI messages expose implementation vocabulary before telling the user what to do. For example, 'uncommitted or ignored untracked state' and 'reconcile that planning state' can become: 'Cannot add and commit tasks: this area already has local changes. Commit or resolve them first.' Likewise, 'another zdev state mutation owns the lock' can become: 'Another zdev update is running. Retry when it finishes.' The short message should lead with the action; detailed rollback information, diagnostic codes, and JSON fields remain available where they are needed. Initialization output should confirm initialization and point to the integration check instead of printing a long harness-governance policy after a successful command.

## Boundaries

- Cover initialization and integration, area branch and rebase handling, task import and rollback, selection and completion, skill installation and checks, commits, and change IDs.
- Preserve JSON keys, schemas, status values, diagnostic codes, commands, exit behavior, and necessary low-level recovery details.
- Do not change workflow semantics while rewriting messages.

## Done when

- [x] Common success messages are concise.
- [x] Recoverable failures state the problem and exact next action.
- [x] Human-readable wording no longer exposes unnecessary orchestration jargon.
- [x] JSON output remains unchanged.

## Validation

- Run focused black-box CLI tests for changed messages.
- Run release smoke tests and confirm unchanged JSON fixtures.

## Result

Rewrote CLI help, success output, and recoverable errors around concrete next actions.

Validation:

- Focused black-box tests, full tests, formatting, clippy, build, release smoke, and diff checks passed after one rework; fresh verification returned PASS.

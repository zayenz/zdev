+++
schema_version = 1
id = "simplify-zdev-text-006"
key = "docs"
area = "simplify-zdev-text"
status = "done"
blocked_by = ["simplify-zdev-text-002", "simplify-zdev-text-003", "simplify-zdev-text-004", "simplify-zdev-text-005"]
+++
# Rewrite user documentation around visible behavior

## Outcome

The README, user guide, workflow guide, and method-provenance documentation explain what users will observe and which command to run, without requiring knowledge of zdev's internal agent organization.

## Context

The README, user guide, workflow guide, and provenance documentation expose internal model organization through terms such as 'main conversation', 'primary conversation', 'authority', 'return control', and 'harness boundary'. Users should not need to understand these concepts to use zdev correctly. For example, 'the main conversation completes and commits the verified task' can become 'after verification, mark the task done and commit it.' The concurrent-change rule can say: 'New task-only commits do not interrupt the current task. Keep the selected task and consider additions at the next zd next.' The user guide should make existing-queue additions easy to discover, explain visible implementation and verification behavior, and keep exact recovery commands. The provenance document should resolve its contradiction between questioning one branch at a time and surveying high-impact branches breadth-first.

## Boundaries

- Preserve exact commands, status meanings, task schema, rebase recovery, task-import defaults, change IDs, harness names, and the documented OpenMP limitation.
- Remove conversational topology and internal orchestration rationale from user-facing text.
- Fix the provenance contradiction between one-branch-at-a-time questioning and breadth-first coverage.
- Keep detailed machine-facing contracts in canonical references instead of duplicating them in user documentation.

## Done when

- [x] Documentation describes visible behavior and actionable commands.
- [x] Terminology is consistent and unnecessary duplicate policy is removed.
- [x] Concurrent task intake is easy to discover and presented as normal behavior.
- [x] Documentation contract tests protect the retained facts without freezing incidental wording.

## Validation

- Run documentation contract and rendering tests.
- Run full formatting, test, clippy, build, release-smoke, and diff checks.

## Result

Rewrote zdev user documentation around visible behavior and actionable commands.

Validation:

- Documentation contracts, formatting, full tests, clippy, debug and release builds, release smoke, package checks, and diff checks passed; fresh verification returned PASS.

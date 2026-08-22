+++
schema_version = 1
id = "improvements-048"
key = "derived-proposal-apply"
area = "improvements"
status = "done"
blocked_by = ["improvements-047", "improvements-036"]
+++
# Apply derived tasks atomically

## Outcome

An authorized derived proposal updates source and child tasks through one locked rollback-safe commit without separate import ceremony.

## Context

Implement the transactional apply slice from docs/derived-work-handoffs.md by reusing task import, graph validation, create-only publication, state locking, and managed commit machinery. Investigation follow-up completes the source and publishes children together. Implementation split leaves the source open and blocks it on every child.

## Boundaries

- One automatic apply consumes one transient proposal; a later independently selected task may make a new proposal after normal gates.
- Split children inherit the source slice and persist their exact future paths in ordinary Boundaries text.
- Do not auto-rename keys, create slices, cross areas, or recursively apply another proposal.
- Preserve allowed unstaged parent-owned implementation bytes; reject staged, overlapping, incomplete, symlinked, or uncertain ownership.
- Failure restores task files, index, and retained implementation bytes exactly.

## Done when

- [x] Investigation follow-up completes the source and imports its children in one managed commit.
- [x] Implementation split creates children, adds all child IDs as blockers of the still-open source, and preserves disjoint retained edits.
- [x] Ready ordering and lifecycle invariants remain ordinary task-graph behavior.
- [x] Success JSON reports source, derived tasks, ready frontier, commit, change ID, and split ownership when applicable.
- [x] Hook, publication, graph, and commit failures leave the pre-apply state recoverable and unchanged.

## Validation

- Add focused black-box tests for follow-up, pre-edit split, post-edit split, ready ordering, and exact rollback under representative failures.
- Run the area-wide validation from brief.md.

## Result

Added atomic derived-task apply for investigation follow-ups and implementation splits with locked validation, managed commit, exact ownership, and rollback.

Validation:

- Independent verifier PASS after managed-path and slice corrections; focused apply/rollback tests and full fmt, clippy, test, build, and diff checks passed.

+++
schema_version = 1
id = "improvements-057"
key = "store-work-context-snapshots"
area = "improvements"
status = "done"
complexity = "advanced"
blocked_by = []
+++
# Store immutable work-context snapshots

## Outcome

Zdev can materialize exact work-context output as a compactly referenced filesystem snapshot and compare fresh checkout state with it without returning full diffs to the coordinator.

## Context

work-context currently nests the full goal and status projections with unbounded Git output. Add a separate transport mode using the linked-worktree-safe Git administrative storage pattern already used for task review artifacts; keep ordinary inline work-context unchanged.

## Boundaries

- Store mode writes the exact existing work-context JSON and returns only schema, area, opaque snapshot ID, path, lifecycle, queue, task ID, and HEAD when applicable.
- Show returns the exact stored JSON; compare collects fresh work-context and returns a compact equal or mismatch result without echoing either complete snapshot.
- Snapshots are immutable, content-addressed, cross-area checked, repository-local Git administrative files and are never treated as fresh authority after their handoff.
- Use a simple bounded automatic retention policy; old IDs may expire, with no cleanup command, history UI, approval, fingerprint display, or user management.
- Preserve existing inline work-context behavior and errors for compatibility.

## Done when

- [x] The binary stores, shows, and fresh-compares exact work-context snapshots through a documented compact JSON contract.
- [x] Storage resolves through git rev-parse --git-path and works correctly from a linked worktree.
- [x] Equal state compares equal, while changed state, corruption, expiration, cross-area identity, and partial-publication failure produce clear non-mutating results.
- [x] Canonical and user guidance distinguishes an immutable handoff from forbidden reuse of stale evidence.

## Validation

- Add focused black-box coverage for exact round-trip, equality and mismatch, corruption, cross-area identity, expiration, linked worktrees, and failure rollback.
- Run the repository standard full validation.

## Result

Added immutable work-context snapshots with exact show and compact fresh comparison.

Validation:

- Focused snapshot, retention, corruption, linked-worktree, and rollback tests passed.
- Format, clippy, all 134 tests, build, generated integration checks, and diff checks passed.

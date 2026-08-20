+++
schema_version = 1
id = "improvements-023"
key = "commit-brief-with-task-import"
area = "improvements"
status = "done"
blocked_by = []
+++
# Include an approved area brief in committed task imports

## Outcome

`zdev tasks import --commit` atomically commits the owning area's already-modified valid brief together with new tasks and regenerated TASKS.md, without requiring a separate brief commit.

## Context

`require_committable_task_import` currently rejects every pre-existing change under `.zdev/<area>`. The managed commit and rollback paths already isolate imported tasks and preserve unrelated index and worktree changes. Extend that allowlist narrowly to the owning area's `brief.md`, which is normally updated by the research that produces the approved task bundle.

## Boundaries

- Change only committed import; ordinary import and task-bundle review fingerprints remain unchanged.
- Permit exactly the owning area's tracked modified brief.md as pre-existing area state, and validate it before creating, staging, or committing imported files.
- Reject deleted, untracked, symlinked, conflicted, or ambiguous partially staged-and-modified brief state.
- Continue rejecting changes to area.toml, slices, existing tasks, unexpected or ignored area files, and every other durable area record.
- Never stage or include source, other-area, or unrelated .zdev paths; preserve their existing staged and unstaged state as the current concurrent-import behavior does.
- Add no brief approval ID, provenance record, or general cross-file transaction framework.
- Retain the existing import commit subject and stable change-ID behavior.

## Done when

- [x] A valid modified brief plus an approved bundle produces one commit containing only brief.md, new task files, and regenerated TASKS.md.
- [x] JSON paths includes the brief only when it was committed, in deterministic order; an unchanged brief preserves the existing task-only commit contract.
- [x] Invalid or ambiguous brief state and every other owning-area modification fail before task publication.
- [x] Commit or staging failure removes imported tasks, restores the prior TASKS.md, preserves exact brief bytes and its prior index and worktree state, and preserves unrelated changes.
- [x] Successful import leaves the committed brief, task, and index paths clean while retaining unrelated staged and unstaged changes.
- [x] Canonical task-authoring guidance, generated copies, README, and user guide describe the single managed commit and no longer request a separate brief commit.

## Validation

- Run focused happy-path coverage with a modified brief, exact commit and JSON paths, and unrelated staged and unstaged source changes preserved.
- Run focused rejection coverage for malformed or ambiguous brief state and other owning-area record changes.
- Run focused hook or staging-failure recovery coverage proving task and index rollback plus exact brief and unrelated-index preservation.
- Run the existing committed-import, approval, concurrent-change, and recovery tests.
- Run cargo test --locked --test lean and the repository's standard full validation from the area brief.

## Result

Committed valid owning-area brief updates atomically with approved task imports while preserving unrelated Git state.

Validation:

- Independent code and contract review passed, including direct unsafe-state probes and rollback audit.
- Focused committed-import and generated-fixture tests passed.
- cargo test --locked --test lean passed (92/92).
- Formatting, Clippy with warnings denied, full tests, build, package verification, and git diff --check passed.

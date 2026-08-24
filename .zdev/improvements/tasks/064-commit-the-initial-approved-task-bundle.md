+++
schema_version = 1
id = "improvements-064"
key = "commit-initial-task-bundle"
area = "improvements"
status = "open"
complexity = "standard"
blocked_by = []
+++
# Commit the initial approved task bundle

## Outcome

The first approved task import is committed as durable planning state for tracked zdev records instead of being forced to remain as a worktree-only split.

## Context

The task-authoring workflow currently omits --commit for every initial split because src/tasks.rs::require_committable_task_import rejects a newly created area’s untracked planning files. This leaves TASKS.md and future task records absent from HEAD and lets later implementation commits reference planning state that was never committed. Extend the existing commit_task_import and import rollback boundary, use project::RecordPolicy/read_config to distinguish local records, and add focused coverage beside the committed-import tests in tests/lean.rs.

## Boundaries

- For project and pull-request policies, accept both normal first-import layouts: untracked config plus an untracked new area, or tracked config plus an untracked new area. The initial managed commit may include config.toml when untracked, the owning area.toml, brief.md, valid slice briefs, imported task files, and regenerated TASKS.md; reject unexpected owning-area files and require referenced parent-area state to be tracked.
- For personal policy, the task-authoring workflow keeps using ordinary import. An explicit --commit is rejected before publication rather than silently downgraded.
- Preserve explicit ordinary import without --commit for users who request uncommitted tasks under any policy.
- Retain the narrow later-import path contract, preservation of unrelated index and worktree changes, stable change IDs, checked-out-branch gates, and rollback guarantees.
- Reject symlinked, malformed, conflicted, unexpectedly staged, ambiguously partially tracked, or otherwise unsafe initial planning state before publication. A project or pull-request record may be force-added despite an ignore rule, matching existing committed task-file behavior; personal state is never force-added.
- Do not add a generic directory transaction or support arbitrary background files in the initial managed commit; an area with additional durable files must commit them separately before managed import.
- Update canonical skill and documentation sources and regenerate checked-in integration copies through the established command.

## Done when

- [ ] A first approved import with --commit for a project or pull-request record creates one managed commit containing the exact required initial planning paths, imported tasks, and regenerated TASKS.md.
- [ ] The committed initial planning record is sufficient when checked out from Git: required config and area metadata are present, and TASKS.md references no absent task or slice file.
- [ ] A personal record’s ordinary first import remains local, while explicit --commit fails before task publication; ordinary uncommitted import remains available under every policy.
- [ ] A new area under an already tracked project config and a new project record with untracked config both succeed without admitting unrelated planning files.
- [ ] Later imports retain their current narrow path contract and preserve unrelated staged and unstaged Git state.
- [ ] Unsafe initial state fails before publication. Commit failure removes created tasks, restores prior TASKS.md bytes and the pre-import index, preserves every pre-existing planning file’s bytes and tracking state, and preserves unrelated staged and unstaged changes.
- [ ] Canonical task-authoring guidance no longer says the initial split must remain uncommitted and generated integrations match it.

## Validation

- Add focused first-import tests for project, pull-request, personal, both accepted config tracking layouts, exact committed paths, checkout completeness, unsafe-state rejection, and rollback behavior where those exercise distinct behavior.
- Run the existing committed-import and documentation-contract tests.
- Run cargo fmt --all -- --check, cargo clippy --locked --all-targets --all-features -- -D warnings, cargo test --locked, cargo build --locked, and git diff --check.

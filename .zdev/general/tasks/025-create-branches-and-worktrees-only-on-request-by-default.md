+++
schema_version = 1
id = "general-025"
key = "branches-on-request"
area = "general"
status = "open"
complexity = "standard"
afk = true
priority = "normal"
blocked_by = []
+++
# Create branches and worktrees only on request by default

## Outcome

By default zdev keeps work on the current checkout and project trunk: agents create new areas in trunk mode and run single-task implementation in the current checkout, creating a branch or worktree only when the user asks or the selected route needs one. A global or per-repository setting restores the isolated behavior.

## Context

On 2026-10-28 the maintainer decided that branches should mostly be created when the user asks for them, and that this is a zdev preference rather than a per-agent habit. The trigger: following today's guidance, an agent created two isolated area branches nobody asked for and had to move both areas back to trunk.

Current behavior to change:
- `zdev area create` without `--branch` or `--trunk` binds the area to the checked-out branch (`src/lib.rs` around 485-497, `src/project.rs` around 592-628).
- `templates/zdev/references/shape-work.md` step 1 (lines 19-22) and its closing paragraph (line 92, "Most areas remain isolated branches") make the isolated area the default; `templates/zdev/references/setup.md` Finish (lines 55-56) offers isolated `general` first.
- `templates/zdev/shared-contract.md` lines 76-81 ("Begin read-only orientation") and `templates/zdev/references/implement.md` lines 28-31 tell implementation to use a worktree when no workspace was chosen. This was a 2.0 decision (`CHANGELOG.md` line 33, "otherwise uses an isolated worktree"); this task deliberately reverses it as the default.
- `templates/zdev/shared-contract.md` lines 131-134 call isolated the default for the `general` area.

Add one fixed config key, `workspace.branching`, with values `on-request` (default) and `isolated`. It is a personal working preference, so it is writable in both global and repository scopes, with the repository value winning. Store it in the existing dual-scope preference pair that holds worker profiles (the global worker file and `.zdev/workers.toml`, read and merged in `src/config.rs` around 958-965 and 1161-1190) as a new `[workspace]` table, extending that file's strict validation (`validate_worker_file`, around 996-1008) and origin reporting (around 1395-1404). Do not put it in `.zdev/config.toml`, whose `project.*` keys are rejected in global scope. `zdev area create` without a mode flag uses trunk under `on-request` and today's checked-out-branch behavior under `isolated`; an explicit `--branch` or `--trunk` always wins. The templates read the setting with `zdev config get workspace.branching` and describe both behaviors.

Docs that describe these defaults or the key reference: `docs/config-command.md` (key table, validation rules, `config show` examples), `docs/user-guide.md`, `docs/workflow.md`, `docs/trunk-area-mode.md` and `docs/upgrading-to-2.0.md`.

## Boundaries

- Existing areas keep their recorded mode; nothing rebinds them.
- Pull-request record policy still requires isolated areas (`src/project.rs` around 592-596 rejects trunk areas there). Under that policy `area create` without a flag keeps producing an isolated area regardless of the setting.
- Routes that inherently need separate checkouts keep them when requested: parallel batches, rolling research and an explicitly assigned source worktree.
- An explicit branch or worktree request from the user always wins over the setting.
- Edit canonical templates and regenerate checked-in copies; do not hand-edit generated files.

## Done when

- [ ] `zdev config get workspace.branching` returns `on-request` with default origin; `set`/`unset` work in global and repository scopes, the repository value wins, values other than `on-request` and `isolated` are rejected like other fixed keys, and `zdev config show` (including `--scope global` and `--scope local`) reports the correct origin for each.
- [ ] On trunk with no mode flag, `zdev area create` creates a trunk-mode area under `on-request` and binds the checked-out branch under `isolated`; `--branch` and `--trunk` override both; with pull-request records and no flag it still creates an isolated area without error under `on-request`.
- [ ] `shape-work.md` (step 1 and the closing paragraph), `setup.md` Finish, `shared-contract.md` (orientation and `general` paragraphs) and `implement.md` step 1 default to trunk areas and the current checkout under `on-request`, describe the `isolated` alternative, and still use worktrees for parallel, rolling research and explicit requests.
- [ ] Generated harness copies match the templates and `zdev skill check` reports every harness ready.
- [ ] `area create --help` states that the mode without a flag follows `workspace.branching` (default `on-request`, trunk). The listed docs, including the key table and examples in `docs/config-command.md`, describe the key and new default, and `CHANGELOG.md` has an Unreleased entry noting the change from the 2.0 worktree default.

## Validation

- In `tests/lean.rs`, add focused tests for the config key (default, global and repository set, unset, invalid value) and for `area create` mode selection under both settings, an explicit flag, and pull-request records. Extend existing template rendering assertions for the changed workspace guidance. No live harness run.
- cargo fmt --all -- --check; cargo clippy --locked --all-targets --all-features -- -D warnings; cargo test --locked; cargo build --locked; git diff --check

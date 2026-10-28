+++
schema_version = 1
id = "general-024"
key = "t3-codex-verifier-default"
area = "general"
status = "open"
complexity = "standard"
afk = true
priority = "normal"
blocked_by = []
+++
# Default T3 verifiers to Codex when it is available

## Outcome

Under T3 orchestration, when the request does not name a verifier provider and exactly one Codex instance is available, every verifier-role worker (task verification, standalone review, audit and task-bundle challenge) runs on Codex with the Codex verifier profile (gpt-6.1-sol, high by default). A stored setting can change or disable this.

## Context

Today `templates/zdev/references/t3-code.md` (section "Resolve workers by their harness") routes every role whose harness is omitted to the main thread's provider, so a Claude coordinator verifies with Claude unless the user names Codex in every request. On 2026-10-28 the maintainer decided that zdev should ship a default: when both Claude and Codex are available in T3, Sol verifies. Verification by a different model family is the motivation; implementers and planners keep inheriting the main thread's provider.

The model and effort need no new defaults: `zdev config profile resolve codex verifier` already returns gpt-6.1-sol high. What is missing is a stored role-to-harness preference. Add one fixed config key, `t3.verifier-harness`, with values `codex`, `claude` and `inherit`, default `codex`, following how existing keys are registered, scoped and shown in `src/config.rs` (see `project.default-area` for a non-worker key) and tested in `tests/lean.rs`. Then make the T3 reference read it with `zdev config get t3.verifier-harness` during role resolution.

Resolution order for the verifier role: an explicit harness or provider choice in the user's request wins; otherwise use the setting; `inherit` keeps today's behavior. When the preferred harness has no available instance, or its instance does not advertise the resolved model, fall back to the inherited provider and state the fallback at dispatch instead of reporting a blocker. Several matching instances still require asking the user, as the reference already says. Existing T3 rendering tests are around `tests/lean.rs` lines 6850 and 7690.

## Boundaries

- Change only verifier-role routing. Implementer, planner and coordinator selection stay as they are.
- Do not change any worker profile default, and do not add per-role routing for other roles.
- Keep the decision in the binary's config and the T3 reference; do not hard-code provider choice in Claude workflow scripts or native dispatch routes.
- Edit `templates/zdev/references/t3-code.md` and regenerate checked-in copies; do not hand-edit generated files.

## Done when

- [ ] `zdev config get t3.verifier-harness` returns `codex` with default origin, and `config set`/`unset` accept `codex`, `claude` and `inherit` and reject other values in the same way other fixed keys do.
- [ ] `zdev config show` lists the key with its origin.
- [ ] The T3 reference states the resolution order, the fallback when the preferred harness or model is unavailable, and that reviews, audits and bundle challenges use the same verifier selection.
- [ ] Generated copies of the T3 reference match the template, and `zdev skill check` reports every harness ready.
- [ ] `docs/worker-profiles.md` and `docs/config-command.md` describe the key and the T3 default, and `CHANGELOG.md` has an Unreleased entry.

## Validation

- Add focused config tests for the default, set, unset and an invalid value; extend an existing T3 rendering assertion to check the new routing text. No live T3 or harness run.
- cargo fmt --all -- --check; cargo clippy --locked --all-targets --all-features -- -D warnings; cargo test --locked; cargo build --locked; git diff --check

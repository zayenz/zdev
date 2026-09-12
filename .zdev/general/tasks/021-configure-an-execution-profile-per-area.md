+++
schema_version = 1
id = "general-021"
key = "configure-an-execution-profile-per-area"
area = "general"
status = "open"
complexity = "advanced"
afk = true
priority = "normal"
blocked_by = []
+++
# Configure an execution profile per area

## Outcome

Each area can name an existing execution profile that becomes the default model-and-effort profile for all worker-backed work in that area.

## Context

Named execution profiles currently resolve one-off role choice, run choice, saved repository/user defaults, then normal. Area metadata in src/project.rs has no profile setting, while dispatch-spec calls already carry an area and standalone worker routes resolve profiles through the shared contracts and harness templates. Add one optional named-profile reference to area.toml and thread the area through existing resolution seams; do not duplicate model or effort rows in area records.

## Boundaries

- An explicit one-off role profile remains highest priority, followed by an explicit run profile, then the area's configured profile, saved local and global defaults, and normal.
- The area profile applies to planning, implementation, verification, audits, discussion, investigation, task-draft challenge, parallel work, and loops whenever those routes dispatch a worker for a named area.
- Keep legacy area.toml files valid and unchanged when no area profile is configured.
- Reuse named-profile validation and resolution. Do not add per-area model/effort tables, provider discovery, task metadata, or automatic profile selection.

## Done when

- [ ] A typed zdev command can set, inspect, replace, and clear an area's execution profile, rejecting unknown profiles before changing area.toml.
- [ ] Profile resolution reports the area-selected profile and concrete role settings when no explicit role or run selection exists, and preserves the established precedence when either explicit selection is present.
- [ ] Every supported harness route that dispatches workers for an area uses the area profile consistently, including continuation and parallel workflows, without changing the coordinator model.
- [ ] Existing areas and workflows without an area profile retain their current effective settings and serialized metadata.

## Validation

- Add focused black-box coverage for area profile mutation, strict/legacy area metadata, precedence, unknown or harness-undefined profiles, and failed-write preservation.
- Extend controlled dispatch and rendered-integration checks only where area selection changes executable behavior; do not make live model calls.
- Run cargo fmt --all -- --check, cargo clippy --locked --all-targets --all-features -- -D warnings, cargo test --locked, cargo build --locked, and git diff --check.

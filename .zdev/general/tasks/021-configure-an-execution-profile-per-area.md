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
# Configure harness-specific execution profiles per area

## Outcome

Each area can select an existing execution profile independently for each harness, and users can inspect or change those settings by asking a supported harness to use zdev's configuration route.

## Context

Named execution profiles currently resolve one-off role choice, run choice, saved repository/user defaults, then normal. Area metadata in `src/project.rs` has no profile settings, while dispatch-spec calls already carry an area and standalone worker routes resolve profiles through shared contracts and harness templates. Add an optional harness-to-profile mapping to `area.toml` and thread the selected harness's area profile through existing resolution seams. Extend the root zdev skill's configuration route so natural-language requests can reliably inspect, set, replace, or clear these settings.

## Boundaries

- An explicit one-off role profile remains highest priority, followed by an explicit run profile, the area profile configured for the current harness, saved local and global defaults, and normal.
- Different harnesses in the same area may select different named profiles; for example, Codex may use `advanced` while Claude uses `normal`.
- The area profile applies to planning, implementation, verification, audits, discussion, investigation, task-draft challenge, parallel work, and loops whenever those routes dispatch a worker for the named area and harness.
- Keep legacy `area.toml` files valid and unchanged when no area profiles are configured.
- Reuse named-profile and harness validation. Do not add per-area model/effort tables, provider discovery, task metadata, or automatic profile selection.
- Configuration through the skill remains an explicit user-directed mutation and does not implicitly reinstall harness integrations or change the coordinator model.

## Done when

- [ ] Typed zdev commands can show an area's harness-profile mapping, set or replace one harness's profile, and clear one harness's profile without disturbing other harnesses.
- [ ] Setting rejects unknown harnesses, unknown profiles, and profiles undefined for the selected harness before changing `area.toml`.
- [ ] Profile resolution reports the harness-specific area profile and concrete role settings when no explicit role or run selection exists, while preserving the established precedence when either explicit selection is present.
- [ ] Every supported harness route that dispatches workers for an area uses that harness's area profile consistently, including continuation and parallel workflows, without changing the coordinator model.
- [ ] The root zdev skill's configuration route explains how an agent handles natural-language requests to inspect, set, replace, or clear per-area harness profiles and reports the resulting setting plainly.
- [ ] Existing areas and workflows without area profiles retain their current effective settings and serialized metadata.

## Validation

- Add focused black-box coverage for per-harness area profile inspection, set, replace, and clear; preservation of other harness rows; strict and legacy area metadata; invalid harness/profile rejection; and failed-write byte preservation.
- Cover role/run/area/saved-default precedence and explicit `normal` selection for one harness.
- Extend controlled dispatch and rendered-integration checks only where area selection changes executable behavior, including the shared natural-language configuration route; do not make live model calls.
- Regenerate checked-in harness integrations from canonical templates.
- Run cargo fmt --all -- --check, cargo clippy --locked --all-targets --all-features -- -D warnings, cargo test --locked, cargo build --locked, and git diff --check.

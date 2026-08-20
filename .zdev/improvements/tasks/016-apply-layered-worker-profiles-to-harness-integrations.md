+++
schema_version = 1
id = "improvements-016"
key = "worker-profile-integrations"
area = "improvements"
status = "open"
blocked_by = []
+++
# Apply layered worker profiles to harness integrations

## Outcome

Integration install and check resolve strict implementer and verifier profiles and render them through each harness's native model and effort controls.

## Context

Implement the settled contracts in docs/worker-profiles.md and the layering rules in docs/config-command.md. Add a focused configuration module for strict worker files, built-in defaults, global-path resolution, whole-profile precedence, and adapter validation. Feed resolved profiles into the existing all-or-nothing MiniJinja realization path in src/integrations.rs. The later config-command tasks expose this machinery through the CLI.

## Boundaries

- Use local, global, then built-in precedence for project integrations and global then built-in precedence for user integrations; the later config-command contract supersedes the earlier local-only research proposal.
- Keep profiles atomic. Explicit inherit wins and never falls through.
- Render native controls for all five harnesses, including Pi model and thinking arguments; reject explicit fields an adapter cannot express instead of dropping them.
- For OpenCode, permit non-inherit effort only for model identifiers with the openai/ prefix; other provider prefixes require effort = "inherit" until a later reviewed contract adds a native mapping.
- Do not add config show, get, set, or unset commands in this task.
- Do not add provider probes, model discovery, evaluation, telemetry, credentials, or a model catalog.
- Resolve and render every artifact before replacing any destination.

## Done when

- [ ] Strict global and local worker files reject unknown schemas, harnesses, roles, keys, efforts, empty models, invalid inherit combinations, and unsupported adapter pairs with the source path and value.
- [ ] Absent files and tables produce the dated built-ins; local and global whole-profile precedence and explicit inherit match the settled contract.
- [ ] Codex, Claude Code, OpenCode, Pi, and Oh My Pi render the resolved model and effort through their native controls, including every documented omission.
- [ ] Project and user install and check use the same resolver and deterministic renderer.
- [ ] Invalid explicit configuration fails before any installed integration changes.
- [ ] Focused tests cover one built-in, one local-over-global override, one inherit or adapter gap, and one preserved-destination failure without constructing a provider matrix.

## Validation

- Run the focused worker-profile integration tests.
- Run cargo test --locked --test lean.
- Run the repository's standard full validation from the area brief.

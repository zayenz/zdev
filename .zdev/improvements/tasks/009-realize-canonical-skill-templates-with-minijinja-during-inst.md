+++
schema_version = 1
id = "improvements-009"
key = "template-realization"
area = "improvements"
status = "done"
blocked_by = []
+++
# Realize canonical skill templates with MiniJinja during installation

## Outcome

Zdev keeps every canonical harness skill, reference, agent, workflow, command, prompt, and extension as an unexpanded Jinja template and uses one deterministic MiniJinja realization path for installation and checking.

## Context

`src/integrations.rs` currently mixes `templates/zdev` sources with shared files from `skills/zdev` and performs ad hoc string replacement for `shared_contract`, `repository_guidance`, `question_tool_guidance`, and `version`. Consolidate canonical renderable sources under the template boundary and render them with the Rust `minijinja` crate. Keep expanded files only as clearly generated installation fixtures where repository coverage needs them.

## Boundaries

- Use one `minijinja::Environment` with strict undefined behavior, auto-escaping disabled, and only the four named string variables `shared_contract`, `repository_guidance`, `question_tool_guidance`, and `version`.
- Treat inserted values as trusted pre-rendered fragments for their Markdown, YAML, JSON, JavaScript, or TypeScript destinations; escape or prepare each value before adding it to the context when its destination syntax requires it.
- Do not register template loaders, arbitrary user context, custom functions, filters, tests, or a general templating configuration surface.
- For identical template and context inputs, produce byte-identical output with no timestamps, ambient paths, or machine-specific state.
- A parse or render failure must occur before any destination is replaced, and canonical source files must never be rendered in place.

## Done when

- [x] Every canonical template parses under MiniJinja while retaining its Jinja expressions in the checked-in source.
- [x] Every supported harness and installation scope realizes complete artifacts with no unresolved Jinja expressions.
- [x] `zdev skill install` and `zdev skill check` call the same rendering function and agree across the supported harness, scope, and guidance matrix.
- [x] Missing variables, unknown variables, invalid syntax, and destination-value escaping failures produce clear errors before publication.
- [x] Focused tests prove byte determinism, trusted fragment insertion, canonical-source preservation, installed-output realization, and generated-fixture consistency.
- [x] Packaging includes the canonical templates and MiniJinja runtime needed by the binary.

## Validation

- Run `cargo test --locked --test lean`.
- Run the repository's standard full validation from the area brief.

## Result

Moved canonical harness artifacts under one template boundary and realized them deterministically with a strict shared MiniJinja path for install and check.

Validation:

- Independent verification confirmed pre-publication rendering, destination-safe JSON handling, canonical-source preservation, fixture consistency, and complete package contents.
- cargo test --locked --test lean (73 passed)
- cargo fmt --all -- --check
- cargo clippy --locked --all-targets --all-features -- -D warnings
- cargo test --locked (77 passed)
- cargo build --locked
- cargo package --locked --allow-dirty
- git diff --check

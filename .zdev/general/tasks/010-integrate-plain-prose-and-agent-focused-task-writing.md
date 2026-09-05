+++
schema_version = 1
id = "general-010"
key = "integrate-human-and-agent-writing-guidance"
area = "general"
status = "done"
complexity = "standard"
afk = true
priority = "normal"
blocked_by = []
+++
# Integrate plain prose and agent-focused task writing

## Outcome

Zdev applies Cursor's unslop guidance to authored prose and Matt Pocock's writing-for-agents method to task and workflow instructions, with guidance available to fresh workers.

## Context

Read Writing and Testing and validation in the [settled workflow brief](../../../plans/002-workflow-review-brief.md). templates/zdev/shared-contract.md already contains a short Noodle-based editorial pass, while references/to-tasks.md describes task context. The root's prose rules are not explicitly included in fresh Codex worker handoffs, and workflow requirements are repeated across shared, implementation, and verification contracts. Adapt https://github.com/cursor/plugins/blob/main/pstack/skills/unslop/SKILL.md and https://github.com/mattpocock/skills/blob/main/skills/productivity/writing-for-agents/SKILL.md. Use templates/zdev/*-skill.md, task-workflows.md, relevant references and worker prompts, docs/adapted-methods.md, and tests/documentation_contract.rs as starting points.

## Boundaries

- Integrate the useful guidance into existing zdev instructions and update existing source attribution. Do not require another installed plugin, add a mandatory reviewer, or create a prose-lint framework.
- Preserve exact syntax, commands, schemas, quotations, technical meaning, and authorized task content. Apply prose editing before serialization or bundle presentation.
- Limit consolidation to instructions affected by this work. Preserve hard workflow guarantees and leave completed task records intact.

## Done when

- [x] Human-facing prose guidance calls for concrete language, clear actors, readable sentences, stable terms, and a final editorial pass without applying stylistic edits to machine syntax.
- [x] Task drafting gives Context the reason and relevant evidence, Boundaries genuine constraints, and Done when observable acceptance; exact requirements are defined once with explicit reading instructions.
- [x] Fresh workers receive or can reliably reach applicable writing guidance when authoring prose, without loading unrelated reference material by default.
- [x] Affected workflow instructions group ordered actions and relevant reference material coherently, and remove duplicated meanings without weakening behavior.
- [x] Representative draft-task and workflow examples demonstrate the guidance. Existing documentation assertions are loosened where they freeze incidental wording while meaningful command and contract checks remain.

## Validation

- Review representative authored prose, one task draft, and fresh-worker handoffs against both source methods; use the existing review rather than another required stage.
- Regenerate affected integrations and run existing documentation, rendering, and fixture-parity checks.
- Run the standard validation in the area brief.

## Result

Integrated concrete human-facing prose guidance and agent-focused task-writing structure into canonical zdev instructions and all generated harness integrations.

Validation:

- Reviewed representative prose, task drafting, and fresh-worker handoffs against both upstream methods; attribution and license notes are correct.
- Focused documentation-contract and generated-fixture parity checks passed.
- cargo fmt --all -- --check; cargo clippy --locked --all-targets --all-features -- -D warnings; cargo test --locked; cargo build --locked; git diff --check all passed.

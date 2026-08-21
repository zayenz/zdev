+++
schema_version = 1
id = "improvements-031"
key = "coherent-workflow-contracts"
area = "improvements"
status = "open"
blocked_by = []
+++
# Repair release safety and current-status guidance

## Outcome

Shipped harness guidance and public documentation agree on safe current behavior and clearly label future designs.

## Context

The release review found four concrete contradictions in canonical templates and public docs: closed areas are branch-independent in the binary but branch-gated in workflow guidance; Claude claims it can inspect or apply `/goal`; natural-language audit can route to either Improve or the strict audit entrypoint; and README/user-guide staging examples can include unrelated `.zdev/scheduling` state. Correct canonical sources first and regenerate every affected harness artifact. Also mark the already-implemented config, goal, and orchestration records as current and the unimplemented complexity, loop, derived-work, and trunk records as designs.

## Boundaries

- Classify a validated closed area before branch and Git work gates; keep those gates for open work.
- Do not add reconciliation agents, durable state, or new approval steps.
- Change generated harness files only through canonical templates and normal regeneration.
- Do not add tests for prose-only staging wording; add focused behavior/contract coverage only where parsing or routing changes.

## Done when

- [ ] Closed areas produce successful no-work off-branch and from detached HEAD in every applicable workflow contract.
- [ ] Claude guidance no longer claims programmatic inspection or application of `/goal`.
- [ ] Active-zdev audit intent has one explicit route to the dedicated audit behavior.
- [ ] README and user guide stage only the completed task file, TASKS.md, and explicitly named implementation paths.
- [ ] Packaged design records clearly distinguish already shipped behavior from the future work represented by this bundle.
- [ ] Affected checked-in harness artifacts match regenerated canonical templates.

## Validation

- Run focused closed-area and harness contract tests.
- Run the canonical template/fixture consistency test.
- Run the area-wide validation from brief.md.

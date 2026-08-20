+++
schema_version = 1
id = "improvements-029"
key = "design-derived-work-handoffs"
area = "improvements"
status = "open"
blocked_by = []
+++
# Define safe derived tasks without repeated approval ceremony

## Outcome

Settle how successful investigations and implementation splits can propose and add concrete follow-up tasks without another routine approval round, while keeping the coordinator responsible for durable mutation and stopping on scope or product changes.

## Context

Task-bundle approval is optional in the binary but workflow guidance treats review as the normal human checkpoint. Workers never mutate .zdev. Imports allocate IDs, validate the graph, lock state, publish transactionally, and can make a managed commit. Investigation-derived work and implementation splitting need an authority contract, not another task state.

## Boundaries

- Research and design only.
- Keep .zdev mutation with the coordinator; workers may return structured proposals but never import directly.
- Do not make approval IDs mandatory, add provenance, create nested task trees, duplicate prompts or transcripts, or add an execution-claim lifecycle.
- Allow automatic derivation only inside the approved objective and boundaries; product choices, widened scope, destructive work, and conflicting ownership still stop for the user.
- Do not choose models or complexity routing here.

## Done when

- [ ] The contract defines when investigation success may add follow-up tasks automatically and when explicit approval remains mandatory.
- [ ] It defines a structured worker-to-coordinator proposal envelope and fail-closed parsing.
- [ ] It settles implementation splitting semantics: original-task status, child dependencies, ready ordering, completion eligibility, and whether new metadata is necessary.
- [ ] It defines duplicate detection and how derived tasks relate to existing slices and task keys.
- [ ] It defines atomic validation, import, and commit behavior plus exact rollback on proposal, graph, publication, or commit failure.
- [ ] It defines what the user sees before and after automatic derivation without requiring a redundant confirmation turn.
- [ ] It defines limits on task count, recursive splitting, repeated derivation, and scope growth.
- [ ] It maps the smallest changes to workflow guidance and existing import and commit seams, with follow-up implementation tasks.

## Validation

- Walk through successful investigation, no-follow-up investigation, safe implementation split, product-choice split, duplicate proposal, invalid dependency, publication failure, and recursive-split cases.
- Compare against the current bundle fingerprint, state lock, managed import, rollback, and coordinator ownership contracts.
- Confirm that existing manually reviewed import remains available.
- Run documentation validation only.

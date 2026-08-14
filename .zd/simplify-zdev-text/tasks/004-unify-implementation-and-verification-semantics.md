+++
schema_version = 1
id = "simplify-zdev-text-004"
key = "execution"
area = "simplify-zdev-text"
status = "done"
blocked_by = ["simplify-zdev-text-001"]
+++
# Unify implementation and verification semantics

## Outcome

The shared contract, agent definitions, commands, and workflows describe one implementation, verification, verdict, and rework loop.

## Context

Execution safety is repeated across the shared contract, implementer and verifier definitions, commands, and workflow files using different organizational language. This creates room for harnesses to stop or proceed differently. The motivating failure was concrete: a side chat added a task, producing a new commit; the implementation chat saw unexpected drift and stopped because its instruction treated any intervening commit as suspicious. The replacement rule should inspect the diff, not identify the chat: tolerate only a commit that adds new .zd/<area>/tasks/*.md files and regenerates .zd/<area>/TASKS.md. Stop for changes to existing tasks, the brief, area metadata, lifecycle state, or source. Likewise, say directly that implementers edit source and tests, verifiers remain fresh and read-only, missing required validation is BLOCKER, and every task-owned REWORK returns through implementation and fresh verification. Avoid explaining those rules through authority, handoff, or fixed retry bureaucracy.

## Boundaries

- Preserve the four required area-status fields and the three-part Git baseline.
- During active work, tolerate only an intervening commit that adds new .zd/<area>/tasks/*.md files and regenerates .zd/<area>/TASKS.md; all other relevant drift stops for review.
- Implementers may edit source and tests but not .zd, task lifecycle state, or commits.
- Verification is fresh and read-only, with separate specification and standards review.
- Missing required validation is BLOCKER; limitations apply only to optional checks.
- Preserve the canonical PASS, REWORK, and BLOCKER meanings.
- Every task-owned REWORK returns to implementation and receives fresh verification, without a fixed retry limit.
- Remove obsolete fields such as Mechanical: yes without weakening parser contracts.

## Done when

- [x] Every model-facing execution source states the same baseline, drift allowlist, edit boundary, verification duties, verdict meanings, and rework loop.
- [x] Harness-specific files contain only harness mechanics.
- [x] Semantic tests cover the invariant table and parser-required tokens.
- [x] Repeated rework is explicitly covered for the Claude workflow.

## Validation

- Run focused execution-contract and harness tests.
- Run full Rust tests and clippy.

## Result

Unified implementation, verification, verdict, drift, and rework semantics across all harnesses.

Validation:

- Focused and full tests, formatting, clippy, build, release smoke, parity, and diff checks passed after one rework; fresh Spec and Standards verification returned PASS.

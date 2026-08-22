+++
schema_version = 1
id = "improvements-047"
key = "derived-proposal-review"
area = "improvements"
status = "done"
blocked_by = ["improvements-033"]
+++
# Parse and review derived task proposals

## Outcome

Investigation and implementation workers can return a strict transient proposal that zdev either validates for automatic authority or renders through ordinary review.

## Context

Implement the first slice of docs/derived-work-handoffs.md. A worker may return one follow-up or split proposal with one to five ordinary TaskDraft-shaped children. The coordinator, never the worker, parses it, checks source identity and current authority, and renders a normal task bundle when automatic authority is unavailable.

## Boundaries

- Workers never write `.zdev` task files or run import.
- Allow one proposal object per uninterrupted worker handoff, no nesting.
- Automatic authority is limited to direct in-brief work with unchanged safe identity and no product, scope, destructive, ownership, or uncertainty decision.
- Do not add durable lineage, proposal counts, provenance, or session state.
- Require exact explicit future path ownership for post-edit splits.

## Done when

- [x] A strict parser accepts follow-up and split envelopes and rejects unknown, nested, oversized, duplicate, or mismatched content.
- [x] Authority failures produce the ordinary rendered review bundle and fingerprint without mutation.
- [x] Valid post-edit split ownership covers the exact unstaged retained-parent set and path-disjoint child futures.
- [x] Child drafts use existing task fields including complexity and optional slice; no new routing metadata is invented.

## Validation

- Add focused parser and authority tests for accepted follow-up/split and every manual-review boundary.
- Run the area-wide validation from brief.md.

## Result

Added strict read-only derived proposal review with mechanical eligibility, ordinary task-bundle fallback, and exact split path ownership validation.

Validation:

- Independent verifier PASS; focused parser/authority tests and full fmt, clippy, test, build, and diff checks passed.

+++
schema_version = 1
id = "improvements-034"
key = "adopt-work-context"
area = "improvements"
status = "open"
blocked_by = ["improvements-032"]
+++
# Use work-context throughout task workflows

## Outcome

Every harness uses the combined context command at load-bearing task boundaries, including a fresh simple check before Claude completion.

## Context

Replace repeated external status/goal/Git runs in implement, rework, verify, and completion flows. The release blocker in Claude is solved by giving its existing completion agent a freshly collected context after verifier PASS and requiring the ordinary task identity and Git postconditions, not by adding a reconciliation agent or byte manifest.

## Boundaries

- The verifier invokes work-context independently rather than reusing coordinator evidence.
- Claude keeps its existing completion step but receives fresh context; do not add another worker, durable replay ledger, clone, byte manifest, or human approval.
- Freshness is a useful guard, not a security protocol; use ordinary deterministic equality and state checks.
- For open work, compare area, task, lifecycle, safety, HEAD, staged diff, unstaged diff, and untracked evidence before coordinator mutation.
- Preserve visible task-done, exact staging, cached-diff review, and commit failure boundaries.

## Done when

- [ ] Canonical and generated implement/verify routes use work-context instead of separate status, goal, and Git invocations.
- [ ] Claude completion receives context collected after verifier PASS and rejects mismatched task, unsafe state, or unexpected Git changes.
- [ ] Closed no-work exits without branch or Git preflight.
- [ ] The ordinary PASS, one-REWORK, verify-only, and no-work routes use fewer external zdev/Git calls than the audited baseline.

## Validation

- Exercise extracted Claude workflow parsing and call ordering for live and resumed results, including changed task, lifecycle, HEAD, index, worktree, untracked state, and malformed context.
- Add focused all-harness contract tests for ready, closed, PASS, REWORK, and verify-only flows.
- Record the new round-trip counts in docs/workflow-round-trips.md.
- Run the area-wide validation from brief.md.

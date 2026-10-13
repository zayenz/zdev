+++
schema_version = 1
id = "general-023"
key = "honor-blanket-parallel-authorization"
area = "general"
status = "open"
complexity = "standard"
afk = true
priority = "normal"
blocked_by = []
+++
# Honor blanket authorization across parallel batches

## Outcome

An ongoing zdev goal or loop reuses explicit blanket approval for compatible later parallel batches instead of asking again after each batch.

## Context

Reported during feature/interfaces: the maintainer approved parallel implementation with four workers, isolated temporary source worktrees, serial integration and cleanup, but templates/zdev/references/parallel.md and generated skills/zdev/references/parallel.md say "Completing this batch authorizes no additional task or batch." The coordinator consequently requested repeated batch approvals and blocked the active goal. On 2026-10-10 the maintainer identified this as a zdev bug and explicitly granted blanket approval. Start with the canonical parallel contract, active goal/loop continuation and recovery guidance, generated harness artifacts, and established rendered-workflow tests.

## Boundaries

- Keep each operational batch finite and revalidate task compatibility, branch safety, ownership, capacity, independent verification and exact commits.
- Do not infer blanket approval from a single-batch approval or an unanswered question. Honor its expressed area, worker limit, worktree/destination and cleanup constraints; ask only for an undecided choice or a change beyond that authority.
- Reuse conversation/run handoffs for approval scope; avoid a new approval database or scheduler. This task records and fixes workflow behavior, not the interfaces implementation.

## Done when

- [x] The canonical parallel workflow distinguishes one-batch approval from explicit approval for compatible future batches in the same ongoing run.
- [x] A goal/loop with blanket approval can choose, admit and start later compatible batches without another permission question while preserving all stated execution limits.
- [x] Single-batch approval still stops before an unauthorized next batch, and a changed user-owned choice still requires clarification.
- [x] All affected generated harness guidance and continuation/recovery descriptions agree with the canonical contract.

## Validation

- Use existing workflow/rendering checks for a blanket-approved multi-batch goal, a single-batch approval, and a later batch that exceeds the approved worker limit or changes the destination. Add only focused coverage where an executable boundary changes.
- Regenerate affected artifacts, run the general-area standard validation and git diff --check.

## Implementation

Implemented in `706111e` as part of the 2.0 release repairs. Canonical parallel,
goal/loop and recovery guidance retain explicit blanket scope and require fresh
finite-batch admission. Generated integrations were refreshed and checked.
Independent source review found no required corrections. The release gate,
including formatting, strict Clippy, all tests and packaged-binary smoke checks,
passed on 2026-10-13. The full Rust suite took 136.99 seconds with scripted local
workers and no LLM calls.

The record remains open until this change is integrated into `main`. The release
checkout has a detached HEAD, while this trunk area's lifecycle commands require
the configured `main` branch. Complete the task through zdev after integration;
the implementation and validation are finished.

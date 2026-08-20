+++
schema_version = 1
id = "improvements-027"
key = "audit-workflow-round-trips"
area = "improvements"
status = "done"
blocked_by = []
+++
# Reduce zdev workflow round trips without weakening evidence

## Outcome

Produce a measured, ranked set of concrete reductions in binary calls, coordinator turns, and worker handoffs for ordinary implement, verify, rework, audit, and task-import flows.

## Context

Current workflows repeatedly collect status, goal, three-part Git evidence, worker envelopes, and completion evidence. Some repetition is load-bearing; some may be combined or reused.

## Boundaries

- Audit and proposal only.
- Count semantic calls separately from cheap local shell invocations.
- Preserve approval, current-task identity, branch safety, fresh independent verification, rollback, stable output, and fail-closed parsing.
- Add no telemetry service, benchmark framework, cache daemon, hidden mutable session state, or broad CLI framework.
- Do not redesign model profiles, loops, or derived-work authority.

## Done when

- [x] The audit provides a per-harness baseline trace from invocation through commit, with calls and turns grouped by purpose.
- [x] It marks every repeated check as required, safely reusable, combinable, or redundant.
- [x] It ranks proposals by saved calls, implementation size, risk, and affected harnesses.
- [x] It gives exact before and after traces for each recommended reduction.
- [x] It explicitly rejects optimizations that weaken freshness, worker independence, ownership attribution, or recovery.
- [x] It produces small follow-up implementation tasks only for worthwhile reductions.

## Validation

- Trace current canonical templates and executable Claude workflows.
- Exercise representative clean, stale-advisory, REWORK, invalid-envelope, and commit-failure paths.
- Confirm counts against actual command and tool boundaries where locally observable.
- Make no synthetic latency claims and run no provider-wide benchmark.

## Result

Measured current zdev workflow boundaries and defined three concrete reductions without weakening freshness, verification, attribution, or rollback.

Validation:

- Independent trace review confirmed actual per-harness counts, proposed savings, and normative integration gaps.
- Focused workflow, parser, audit, import, and rollback tests plus full formatting, Clippy, test, build, documentation, and diff checks passed.

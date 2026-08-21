+++
schema_version = 1
id = "improvements-043"
key = "complexity-worker-routing"
area = "improvements"
status = "open"
blocked_by = ["improvements-041", "improvements-042", "improvements-033"]
+++
# Route task complexity through planning and escalation

## Outcome

Every harness deterministically selects routine, standard, or advanced workers, adds planning only for advanced work, and supports one verifier-requested escalation.

## Context

Implement the settled routing behavior with the revised tier names. Routine and standard tasks go directly to implementation. Advanced tasks first receive one fresh read-only plan from the advanced implementer. Verification is always fresh and standard. A standard verifier may return REWORK with a one-time advanced-implementer escalation.

## Boundaries

- Routine tasks use routine-implementer only when their authored level says routine; do not infer cheapness from file count.
- Advanced planning is read-only and produces a strict plan/blocker result before edits.
- Verification always uses the standard verifier and cannot be downgraded to routine.
- Escalation is one-way, at most once for the current task run, and only after REWORK; product decisions still stop for the user.
- Do not persist routing attempts or add a workflow engine.

## Done when

- [ ] All five harnesses route each level to the configured native profile.
- [ ] Only advanced tasks plan before first edits; resume, verification, and ordinary rework do not repeat planning.
- [ ] Verifier REWORK can request one advanced implementation retry and then requires fresh standard verification.
- [ ] Strict tests cover routine PASS, standard PASS, advanced plan/PASS, ordinary REWORK, escalation, and user-decision BLOCKER.

## Validation

- Run focused all-harness routing and envelope tests.
- Regenerate and check all harness artifacts.
- Run the area-wide validation from brief.md.

+++
schema_version = 1
id = "improvements-041"
key = "worker-tier-profiles"
area = "improvements"
status = "open"
blocked_by = []
+++
# Add routine, standard, and advanced worker profiles

## Outcome

Worker configuration exposes three understandable cost/capability tiers with editable harness-native defaults.

## Context

Replace the earlier standard/complex profile proposal with routine, standard, and advanced tiers. Standard is the normal default: Sol low for Codex-style implementation and Opus low where Claude is native, with corresponding harness-native forms. Advanced uses the same frontier families at high reasoning. Routine is explicitly selected for tightly specified low-risk mechanical tasks and uses Luna or the harness's genuinely cheaper equivalent; it may edit only task-owned implementation files and may collect narrow read-only evidence.

## Boundaries

- Keep the existing implementer and verifier roles as standard; add routine-implementer and advanced-implementer rather than a role matrix.
- The planner reuses advanced-implementer read-only; do not add planner or advanced-verifier config keys.
- Routine implementers may edit only the selected task's implementation paths; they never perform final verification, coordinate lifecycle, stage, commit, or make product decisions.
- Routine is never inferred or selected by default; only authored routine complexity activates it.
- Do not build model evaluations, provider discovery, pricing logic, or a catalog; defaults are seeded suggestions and remain easy to override.
- For harnesses without Luna, choose their documented cheap native model rather than pretending adapters are identical.

## Done when

- [ ] Built-in and layered config resolves routine-implementer, implementer, verifier, and advanced-implementer for all five harnesses with origin reporting.
- [ ] Standard defaults use Sol/Opus at low reasoning where supported; advanced uses high reasoning; routine uses a documented cheaper model.
- [ ] Install/check renders each profile into the native harness artifacts deterministically.
- [ ] Legacy implementer/verifier configuration remains valid with whole-profile override semantics.

## Validation

- Add focused strict parsing, layering, origin, default, adapter-validation, and realization tests.
- Install and check all five harnesses in temporary destinations.
- Run the area-wide validation from brief.md.

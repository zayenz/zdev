+++
schema_version = 1
id = "improvements-042"
key = "task-complexity-levels"
area = "improvements"
status = "open"
blocked_by = []
+++
# Persist routine, standard, and advanced task complexity

## Outcome

Tasks can select a durable complexity level while old tasks and unchanged reviewed bundles keep their current behavior.

## Context

Add optional task complexity using the same three names as worker tiers. Omitted legacy tasks are standard. New task bundles may explicitly choose a level without changing task readiness, lifecycle, or dependency semantics.

## Boundaries

- Use `routine`, `standard`, and `advanced`; omitted means standard.
- Routine is limited to tightly specified low-risk mechanical work.
- Preserve legacy task bytes and existing approval fingerprints when complexity is omitted.
- Do not add estimates, points, provider names, or execution history to task metadata.

## Done when

- [ ] Strict task parsing, review, import, show, list, status, check, and goal projection support the optional complexity field.
- [ ] Invalid values and unknown fields fail with actionable errors.
- [ ] Legacy tasks behave as standard without rewrite.
- [ ] New explicit complexity survives review/import and is visible to routing.

## Validation

- Add focused legacy compatibility, explicit round-trip, invalid-value, and approval-fingerprint tests.
- Run the area-wide validation from brief.md.

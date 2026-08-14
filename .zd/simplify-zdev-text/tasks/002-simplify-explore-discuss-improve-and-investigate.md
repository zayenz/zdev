+++
schema_version = 1
id = "simplify-zdev-text-002"
key = "methods"
area = "simplify-zdev-text"
status = "done"
blocked_by = ["simplify-zdev-text-001"]
+++
# Simplify Explore, Discuss, Improve, and Investigate

## Outcome

The four planning and research interactions use short trigger, action, and stop rules instead of process choreography.

## Context

The planning references repeat process phrases such as 'return control', 'visible transition', and 'authoritative synthesis'. These make the model reason about zdev's organization instead of the user's request and repository state. Discuss currently risks turning breadth-first questioning into an exhaustive gate through instructions such as 'survey every source', track every material branch, and confirm 'full branch coverage'. Replace that with an observable stopping rule: identify choices that could materially change behavior, scope, task splitting, or validation; read relevant sources; ask high-impact independent questions; stop when no such unresolved choice remains. Improve and Investigate should similarly follow the requested order when both are requested instead of asking the user to choose between internal interaction names.

## Boundaries

- Preserve confirmation before editing project documentation outside .zd.
- Preserve read-only boundaries, evidence requirements, source indexing, testing-level selection, and the distinction between observation and inference.
- Discuss identifies material decisions, reads relevant sources, asks up to three independent high-impact questions per round, updates the brief, and stops when no unresolved choice would materially change the objective or task split.
- Do not change task approval, import, implementation, verification, or CLI behavior.

## Done when

- [x] Each interaction clearly states when to use it, what it may do, and when it stops.
- [x] Discuss remains breadth-first without claiming exhaustive source or branch coverage.
- [x] Mixed Improve and Investigate requests can follow the user's order without a procedural question.
- [x] Rendered harness references remain synchronized.

## Validation

- Run focused reference-rendering and contract tests.
- Inspect generated harness instructions for semantic parity.

## Result

Rewrote Explore, Discuss, Improve, and Investigate as concise operational contracts.

Validation:

- Formatting, focused and full tests, integration parity, build, and diff checks passed; fresh Spec and Standards verification returned PASS.

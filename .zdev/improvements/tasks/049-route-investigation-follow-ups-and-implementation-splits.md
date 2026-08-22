+++
schema_version = 1
id = "improvements-049"
key = "derived-proposal-guidance"
area = "improvements"
status = "done"
blocked_by = ["improvements-048"]
+++
# Route investigation follow-ups and implementation splits

## Outcome

All harnesses can propose and apply direct derived work without user approval when authority is clear, and fall back to normal review when it is not.

## Context

Update investigate, implement, loop, recovery, and task-creation guidance to use the derived proposal command. The ordinary behavior should make adding newly discovered in-scope work cheap while keeping user-owned choices visible.

## Boundaries

- Do not ask for approval when the binary accepts automatic authority.
- Display and request ordinary task-bundle approval when it does not.
- A split pauses the source until children finish; investigation follow-up may advance the loop to newly ready work.
- Use the same semantics in all five harnesses and regenerate from canonical templates.

## Done when

- [x] Investigation workers are invited to propose necessary follow-up tasks as a normal result.
- [x] Implementation workers can propose a split before or after edits under the exact ownership rules.
- [x] Automatic apply proceeds without review/import ceremony; rejected authority routes to the existing fingerprinted review flow.
- [x] Loop behavior handles new ready children deterministically without recursive proposal application.

## Validation

- Add focused all-harness guidance and routing fixtures for automatic and manual paths.
- Install and check all five integrations.
- Run the area-wide validation from brief.md.

## Result

Integrated derived follow-up and split proposals across all harness guidance with direct automatic apply, semantic-only manual review, and mechanical safety stops.

Validation:

- Independent verifier PASS after fallback correction; focused Claude/all-harness routes and full fmt, clippy, test, build, install/check, and diff validation passed.

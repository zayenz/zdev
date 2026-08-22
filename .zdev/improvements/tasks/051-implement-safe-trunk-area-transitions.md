+++
schema_version = 1
id = "improvements-051"
key = "trunk-area-transitions"
area = "improvements"
status = "done"
blocked_by = ["improvements-050"]
+++
# Implement safe trunk area transitions

## Outcome

Users can create, bind, convert, and reconfigure trunk areas through locked atomic operations that preserve reachable work.

## Context

Implement the transition and project.trunk mutation rules from docs/trunk-area-mode.md. Reconfiguring trunk changes one project config value for every trunk area, requires old-tip containment by default, and offers one explicit allow-divergent escape that waives only ancestry.

## Boundaries

- Trunk mode forbids parent and managed rebase operations.
- Unset project.trunk is rejected while any trunk area exists.
- Candidate branches must exist locally and not be owned by an isolated area.
- Default reconfiguration requires the old trunk tip to be an ancestor of the candidate; `--allow-divergent` is explicit and does not move refs.
- Use existing locking and atomic config publication; failure preserves prior bytes.

## Done when

- [x] Create/bind grammar supports explicit trunk mode and safe isolated-to-trunk and trunk-to-isolated transitions.
- [x] Project trunk set/unset validates every affected area and reports old/new branches, tips, affected areas, and ancestry result.
- [x] Descendant changes succeed by default; behind, divergent, missing-old, collision, and invalid-unset cases fail without mutation.
- [x] The explicit divergence override is recorded in output but adds no persistent waiver state.

## Validation

- Add focused transition, reconfiguration, rollback, ancestry, and override tests.
- Run the area-wide validation from brief.md.

## Result

Added locked safe trunk-area transitions and project.trunk reconfiguration with containment checks, atomic publication, and explicit divergence override.

Validation:

- Independent verifier PASS after ancestry recovery correction; focused transition/reconfiguration tests and full fmt, clippy, test, build, and diff checks passed.

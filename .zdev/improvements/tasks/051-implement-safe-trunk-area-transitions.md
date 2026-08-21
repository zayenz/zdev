+++
schema_version = 1
id = "improvements-051"
key = "trunk-area-transitions"
area = "improvements"
status = "open"
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

- [ ] Create/bind grammar supports explicit trunk mode and safe isolated-to-trunk and trunk-to-isolated transitions.
- [ ] Project trunk set/unset validates every affected area and reports old/new branches, tips, affected areas, and ancestry result.
- [ ] Descendant changes succeed by default; behind, divergent, missing-old, collision, and invalid-unset cases fail without mutation.
- [ ] The explicit divergence override is recorded in output but adds no persistent waiver state.

## Validation

- Add focused transition, reconfiguration, rollback, ancestry, and override tests.
- Run the area-wide validation from brief.md.

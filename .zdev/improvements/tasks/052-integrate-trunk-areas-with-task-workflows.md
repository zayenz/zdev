+++
schema_version = 1
id = "improvements-052"
key = "trunk-area-workflows"
area = "improvements"
status = "done"
blocked_by = ["improvements-051", "improvements-034"]
+++
# Integrate trunk areas with task workflows

## Outcome

Selection, status, completion, lifecycle, cleanup, and harness guidance treat explicit trunk areas coherently without forcing branches.

## Context

Finish the vertical slice from docs/trunk-area-mode.md. Trunk areas use current project.trunk for task-work safety, may share that branch, do not rebase, and participate in deterministic next --any ordering alongside isolated areas.

## Boundaries

- Keep goal projection branch-independent.
- Expose mode and resolved-branch facts in status, next, and work-context only where useful.
- On trunk, matching candidates still sort by area tag then each area's numeric task order.
- Completion commits remain attributed to the selected area/task and use exact-path staging.
- Do not add a global trunk work queue or automatic checkout.

## Done when

- [x] Task selection, next --any, status, work-context, import, reopen/close, completion, and cleanup accept safe trunk areas and reject unsafe ownership states.
- [x] Trunk areas never request rebase or freshness ceremony and always resolve configuration changes dynamically.
- [x] Multiple trunk areas on the checked-out trunk have deterministic selection and correct lifecycle isolation.
- [x] Canonical and generated harness guidance offers trunk mode as an explicit alternative to isolated branches.

## Validation

- Add focused multi-area black-box tests across selection, lifecycle, import, completion, config change, and cleanup.
- Regenerate and check all harness artifacts.
- Run the area-wide validation from brief.md.

## Result

Integrated explicit trunk areas across selection, import, lifecycle, completion, cleanup, and all harness guidance without branch or rebase ceremony.

Validation:

- Independent verifier PASS; focused multi-area workflow and all-harness generation tests plus full fmt, clippy, test, build, and install/check passed.

+++
schema_version = 1
id = "improvements-045"
key = "claude-area-loop"
area = "improvements"
status = "open"
blocked_by = ["improvements-044"]
+++
# Implement Claude goal and loop workflows

## Outcome

Claude provides native namespaced goal and loop workflows that continue the area using the ordinary one-task contract.

## Context

Render `zdev-goal` and `zdev-loop` from one canonical Claude workflow source. The workflow owns its continuation loop and does not inspect or invoke Claude's separate `/goal` command. Reuse the corrected implement/verify/completion steps rather than layering a second reconciliation protocol.

## Boundaries

- The two entrypoints differ only in their user-facing name.
- Do not call or claim access to `/goal`.
- Use fresh work-context between iterations and after resumed/cached workflow results.
- Do not add a separate reconciliation worker; ordinary completion postconditions are sufficient.

## Done when

- [ ] Both Claude workflow names install, check, and execute the same area-continuation semantics.
- [ ] A successful task commit advances to the next iteration; closed or other valid no-work stops successfully.
- [ ] Cached/resumed results cannot skip the next fresh work-context check.
- [ ] Failure and user-decision states stop without starting another task.

## Validation

- Exercise the extracted real workflow with focused Node fixtures for two-task continuation, closed no-work, REWORK, resumed result, and failure.
- Install and check the Claude integration.
- Run the area-wide validation from brief.md.

---
description: Review a codebase and return checked findings
---

{{audit_contract}}

Boundary: `$ARGUMENTS`

With no explicit lenses, use exactly one blocking agent `zdev-verifier` through
the native `task` tool, with this contract and the boundary. When the user
supplies up to four explicit lenses, start all independent lens verifier tasks
together in one native task-tool batch. Then use a different fresh verifier
task to open, check, and deduplicate every candidate location. Validate the
final first line and required body before returning it.

{{repository_guidance}}

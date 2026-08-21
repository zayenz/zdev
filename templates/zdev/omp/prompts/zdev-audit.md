---
description: Review a codebase and return checked findings
---

{{audit_contract}}

Boundary: `$ARGUMENTS`

With no explicit lenses, use exactly one blocking agent `zdev-verifier` through
the native `task` tool, with this contract and the boundary. With one to four
explicit lenses, start one independent verifier task per lens and use a
different fresh verifier task to open, check, and deduplicate every candidate
location. Reject more than four before starting a task. Validate the final
first line and required body before returning it.

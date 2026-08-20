---
description: Review a codebase and return checked findings
---

{{audit_contract}}

Boundary: `$ARGUMENTS`

Use the native `task` tool with blocking agent `zdev-verifier`, this contract,
and the boundary. For warranted fan-out, start independent verifier tasks and
use a different fresh verifier task to open, check, and deduplicate every
candidate location. Validate the final first line and required body before
returning it.

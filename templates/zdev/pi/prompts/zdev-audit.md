---
description: Review a codebase and return checked findings
---

{{audit_contract}}

Boundary: `$ARGUMENTS`

With no explicit lenses, call `zdev_subagent` exactly once with role `verifier`,
this contract, and the boundary. With one to four explicit lenses, use one
independent verifier call per lens and a different final verifier call to open,
check, and deduplicate every candidate location. Reject more than four before
starting a subagent. Validate the final first line and required body before
returning it. The coordinating agent decides whether to record any work.

---
description: Review a codebase and return checked findings
---

{{audit_contract}}

Boundary: `$ARGUMENTS`

Call `zdev_subagent` with role `verifier`, this contract, and the boundary. For
warranted fan-out, use independent verifier calls and a different final
verifier call to open, check, and deduplicate every candidate location.
Validate the final first line and required body before returning it. The
coordinating agent decides whether to record any work.

---
description: Review a codebase and return checked findings
argument-hint: <boundary> [lenses]
---

{{audit_contract}}

Boundary: `$ARGUMENTS`

With no explicit lenses, call `zdev_subagent` exactly once with role `verifier`,
this contract, and the boundary. When the user supplies up to four explicit
lenses, use one independent verifier call per lens and a different final
verifier call to open, check, and deduplicate every candidate location. Validate
the final first line and required body before returning it. The coordinating
agent decides whether to record any work.

{{repository_guidance}}

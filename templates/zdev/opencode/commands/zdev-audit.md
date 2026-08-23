---
description: Review a codebase and return checked findings
agent: plan
---

{{audit_contract}}

Boundary: `$ARGUMENTS`

With no explicit lenses, use the task tool exactly once with a fresh
`zdev-verifier` subagent, this contract, and the boundary. When the user supplies
up to four explicit lenses, use one independent verifier call per lens and a
different final verifier call to open, check, and deduplicate every candidate
location. Validate the final first line and required body before returning it.

{{repository_guidance}}

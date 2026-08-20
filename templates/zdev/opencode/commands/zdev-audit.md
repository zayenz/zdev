---
description: Review a codebase and return checked findings
agent: plan
---

{{audit_contract}}

Boundary: `$ARGUMENTS`

Use the task tool with the fresh `zdev-verifier` subagent. Supply this contract
and the boundary. For warranted fan-out, use independent verifier calls and a
different final verifier call to open, check, and deduplicate every candidate
location. Validate the final first line and required body before returning it.

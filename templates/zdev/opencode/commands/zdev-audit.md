---
description: Review a codebase and return checked findings
agent: plan
---

{{audit_contract}}

Boundary: `$ARGUMENTS`

With no explicit lenses, start one initial review through the task tool with a fresh `zdev-verifier` subagent.
Supply this contract, the boundary, and the installed coding guidance path.
Independently vet substantive findings under this contract before returning
results, using another verifier with frozen settings when coordination cannot
inspect the evidence independently. With up to four explicit lenses, use one
independent verifier per lens and a different final verifier to check candidate
findings when any exist. Validate the final first line and required body.

{{repository_guidance}}

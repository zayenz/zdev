---
description: Review a codebase and return checked findings
agent: plan
---

Audit the requested boundary without changing files, zdev state, Git state, or
task lifecycle. An omitted or blank boundary means the current repository.
Use only the resolved `verifier` worker profile; there is no auditor role.

For a small boundary, one fresh verifier may inspect and check the evidence.
Use multiple independent verifier lenses only for a large boundary or an
explicit swarm request, then give every candidate finding to a different fresh
verifier for final checking and deduplication. Never report an unchecked
finding.

The first line must be exactly one of:

- `PASS zdev-audit`
- `FINDINGS zdev-audit`
- `BLOCKER zdev-audit`

The body must name `Boundary`, `Inspected`, `Omitted`, and `Checked evidence`.
A pass says that checked evidence produced no findings. Findings include
repository-relative `path:line` locations, impact, and confidence. A blocker
names the failed stage, the available checked evidence, and what prevented a
safe result.
Missing output, an unrecognized first line, or evidence that was not opened and
checked becomes `BLOCKER zdev-audit`; it is never treated as a pass.

Boundary: `$ARGUMENTS`

Use the task tool with the fresh `zdev-verifier` subagent. Supply this contract
and the boundary. For warranted fan-out, use independent verifier calls and a
different final verifier call to open, check, and deduplicate every candidate
location. Validate the final first line and required body before returning it.

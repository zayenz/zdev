---
name: zdev-audit
description: "Run a read-only zdev codebase audit with independently checked findings. Use when the user invokes $zdev-audit or asks active zdev to audit a named boundary."
---

# Zdev audit for Codex

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

Use a fresh Codex collaboration agent as the verifier. Pass
`model="gpt-5.6-sol"` and
`reasoning_effort="high"` when spawning it.
Give it the boundary, repository guidance, applicable `AGENTS.md` instructions,
and the audit contract above. If fan-out is warranted, use fresh verifier
agents for the lenses and a different fresh verifier for final evidence
vetting. Validate the returned first line and required body before reporting
it. Do not create tasks automatically; the user decides whether findings
become durable work.

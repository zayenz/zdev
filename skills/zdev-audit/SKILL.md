---
name: zdev-audit
description: "Runs a read-only zdev codebase audit with independently checked findings. Use when the user invokes $zdev-audit or asks active zdev to audit or review a named repository boundary."
---

# Zdev audit for Codex

Audit the requested boundary without changing files, zdev state, Git state, or
task lifecycle. An omitted or blank boundary means the current repository.
Use only the resolved `verifier` worker profile; there is no auditor role.

With no explicit lenses, use exactly one fresh verifier to inspect the boundary,
check the evidence, and return the public result. An explicit list of one to
four lenses selects the larger audit path: use one independent verifier per
lens, then give every candidate finding to one different fresh verifier for
final checking and deduplication. Reject more than four lenses before starting
any worker. Never report an unchecked finding.

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

Use a fresh Codex collaboration agent as the verifier and set
`fork_turns="none"`. Pass
`model="gpt-5.6-sol"` and
`reasoning_effort="low"` together with
`fork_turns="none"` when spawning it.
Give it a compact filesystem-backed message containing its role, the exact
boundary or lens, and exact paths to the installed audit contract, repository
guidance, and applicable `AGENTS.md` files. The agent reads those files and
returns the contract's public result. With no explicit lenses, start exactly that one
verifier. With one to four explicit lenses, use fresh verifier agents for the
lenses and a different fresh verifier for final evidence vetting. Reject more
than four before starting an agent. Validate the returned first line and
required body before reporting it. Do not create tasks automatically; the user
decides whether findings become durable work.

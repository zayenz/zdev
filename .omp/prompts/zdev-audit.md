---
description: Review a codebase and return checked findings
---

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

Boundary: `$ARGUMENTS`

With no explicit lenses, use exactly one blocking agent `zdev-verifier` through
the native `task` tool, with this contract and the boundary. When the user
supplies up to four explicit lenses, start all independent lens verifier tasks
together in one native task-tool batch. Then use a different fresh verifier
task to open, check, and deduplicate every candidate location. Validate the
final first line and required body before returning it.

<!-- zdev:generated-repository-guidance:start -->
## Repository guidance discovery

Before inspecting, planning, changing, or validating code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.
<!-- zdev:generated-repository-guidance:end -->

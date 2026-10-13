---
description: Review a codebase and return checked findings
agent: plan
---

Audit the requested boundary without changing files, zdev state, Git state, or
task lifecycle. An omitted or blank boundary means the current repository.
Use only the resolved `verifier` worker profile; there is no auditor role.
Resolve it once at audit start under the shared scoped-profile contract and use
the retained concrete settings for every lens and the final evidence check.
When the requested audit names an area, pass that area to profile resolution so
the current harness's area profile applies.

Read the supplied coding guidance's **Code standards** and **Review** sections, and give
its installed path to each verifier. Assess applicable rules within the audit
boundary; missing historical authoring evidence remains unknown. Retain the
reviewed code state and compare it before accepting checked findings. Changed
code requires fresh review.

With no explicit lenses, use one fresh verifier for the initial boundary
review. Before substantive findings are reported, someone other than their
author must inspect the cited code and check locations, rule applications,
consequences, and severity. The coordinating agent may do this when independent
and able to inspect the evidence; otherwise use a different fresh verifier with
the retained settings. No additional check is needed for zero findings.

An explicit list of one to four lenses uses one independent verifier per lens,
then a different fresh verifier to check and deduplicate candidate findings
when any exist. Reject more than four lenses before starting any worker. Do
not repeat the whole audit to vet findings or recursively check the checking
reviewer. Correct or remove overturned claims with reasons in `Checked
evidence`. Unresolved or unchecked claims remain open questions in a blocker,
never established findings. Never report an unchecked finding as verified.

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

With no explicit lenses, start one initial review through the task tool with a fresh `zdev-verifier` subagent.
Supply this contract, the boundary, and the installed coding guidance path.
Independently vet substantive findings under this contract before returning
results, using another verifier with frozen settings when coordination cannot
inspect the evidence independently. With up to four explicit lenses, use one
independent verifier per lens and a different final verifier to check candidate
findings when any exist. Validate the final first line and required body.

<!-- zdev:generated-repository-guidance:start -->
## Repository guidance discovery

Before inspecting, planning, changing, or validating code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.
<!-- zdev:generated-repository-guidance:end -->

## Coding guidance

Coding guidance file: "/Users/zayenz/projects/zdev/.opencode/skills/zdev-opencode/references/coding.md"

Before planning code, read Design and Code standards from this file. Before implementing or repairing code, also read Completion. Before verification, audit, or code review, read Code standards and Review. Pass this exact location to each relevant worker; read it directly even when the root skill is not loaded. Routine coordination and non-code tasks do not need it.

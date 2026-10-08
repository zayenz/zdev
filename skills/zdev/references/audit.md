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

# Explore an objective

## When

Use **Explore an objective** after zdev is active when the user wants to start
an area, compare possible paths, or sharpen an objective before creating tasks.
Exploration creates or revises the area's authoritative `brief.md`; it does not
create tasks or a second plan.

## Do

1. For a new objective, initialize zdev on project trunk when needed, create or
   switch to the area's branch, and create the area. For an existing area,
   preserve its identity and recorded branch.
2. Read the conversation, current brief, repository instructions, relevant
   source and tests, domain documentation, and applicable ADRs.
3. Resolve repository facts directly. Ask the user only about choices that
   materially change behavior, scope, or a hard-to-reverse decision.
4. State the objective as an observable result. When several paths remain
   plausible, compare their trade-offs and recommend one. Leave product and
   scope choices to the user.
5. Preserve useful brief text and record the smallest useful set of sections,
   normally:

   - Objective and observable success
   - Boundaries and non-goals
   - Terms and settled decisions
   - Open questions
   - Testing
   - Validation

Keep `brief.md` as the only area-level synthesis. When a settled term or model
applies across objectives, propose an update to existing domain documentation.
When a hard-to-reverse or surprising trade-off deserves an ADR, follow the
repository's convention. Unless the user already requested those changes,
show their scope and wait for confirmation before writing outside `.zd`. Link
approved project records from the brief instead of copying them.

For a large source corpus, keep readable source files under
`.zd/<area>/background/` and add a brief index that names the question each
source informs. Link to canonical repository sources instead of copying them.
Read the sources relevant to the current decision; background material does
not override the brief.

Inspect nearby tests, seams, helpers, and validation commands. Choose and
record the smallest useful testing level: no new tests, existing checks only,
focused coverage, or broader regression coverage. Record the behavior or risk
that justifies it, established commands and patterns, and known limits. Do not
add a harness, helper layer, or broad matrix merely for robustness. If testing
level remains a material user choice, recommend one and state its confidence
and cost.

## Stop

Run `zd check <area> --format json`. Summarize the objective, boundaries,
testing level, settled decisions, and remaining material questions. Recommend
one next interaction when the result points clearly to it, then stop unless the
user already requested more work. A sharp brief may proceed directly to
**Create tasks**; **Discuss the brief** remains optional.

Most areas remain independent branches based on trunk. When a new area builds
on another in-progress area, create its branch from the parent branch, create
the area, then run `zd area parent <child> <parent>`.

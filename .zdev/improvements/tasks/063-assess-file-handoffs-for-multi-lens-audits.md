+++
schema_version = 1
id = "improvements-063"
key = "investigate-multilens-file-handoffs"
area = "improvements"
status = "done"
complexity = "standard"
blocked_by = []
+++
# Assess file handoffs for multi-lens audits

## Outcome

Determine from structural measurements whether transient file handoffs reduce total prompt transport for explicit multi-lens audits without adding coordinator rounds or ceremony.

## Context

Claude concatenates up to four reviewer outputs into the final vetter prompt; other adapters express the same semantics through native workers. This task investigates the transport boundary without running model evaluations and may derive a narrow implementation task only if the result is worthwhile.

## Boundaries

- Separate observed Claude prompt construction from the declarative behavior of Codex, OpenCode, Pi, and Oh My Pi.
- Measure contract bytes, reviewer-output amplification for one through four lenses, worker calls, and any additional file-read or tool calls.
- Do not run model evaluations or create a generic artifact framework.
- Recommend file transport only when it reduces total transported prompt material without another coordinator round or user-visible ceremony.
- If worthwhile, derive narrowly scoped implementation tasks under existing authority; otherwise record the rejection and add no work.

## Done when

- [x] The investigation records reproducible structural measurements for the current canonical and generated audit flows.
- [x] The conclusion distinguishes prompt transport savings from text merely moved into a file and states confidence and limitations.
- [x] A worthwhile design produces only the necessary derived implementation task or tasks; a rejected design records why no task was added.

## Validation

- Run the measurement or inspection commands recorded by the investigation.
- Run documentation checks for any updated decision record.

## Result

Structural measurements show that file-backed multi-lens audit handoffs would add writes and reads without reducing evidence consumption, workers, or coordinator rounds, so no implementation task was added.

Validation:

- Independent verification reproduced all byte and one-through-four-lens measurements; brief indexing, documentation, full cargo checks, diff check, and fresh work-context comparison passed.

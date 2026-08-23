+++
schema_version = 1
id = "improvements-060"
key = "reference-worker-contract-files"
area = "improvements"
status = "open"
complexity = "standard"
blocked_by = []
+++
# Reference installed worker-contract files

## Outcome

Harness workers can load one rendered canonical task contract from a deterministic installed file instead of receiving repeated full-contract text where that measurably reduces generated prompts.

## Context

The 10.6 KB task workflow contract is currently interpolated into every Claude agent call. Install it once as a managed resource and make the Claude workflow pass a resolvable path plus small role-specific instructions; inspect other harnesses rather than assuming equivalent behavior.

## Boundaries

- Install one rendered canonical contract file at a deterministic harness-local path and keep MiniJinja as the realization mechanism.
- For Claude, replace repeated full-string injection with a resolvable path and concise role-specific instruction, with no additional coordinator round-trip.
- Change another harness only when its child workers can reliably resolve the installed path; otherwise retain inline guidance and document the checked reason.
- Preserve an inline fallback when the installed resource cannot be read.
- Do not claim total model-context savings solely because text moved to a file.

## Done when

- [ ] Claude generated agent prompts contain the contract path rather than a repeated complete contract and workers are instructed to load it before acting.
- [ ] Measured generated Claude agent-call prompt bytes are lower before task-specific context, with the measurement recorded in workflow documentation.
- [ ] Install, check, force-refresh, packaging, and generated-equality behavior manage the new resource atomically.
- [ ] Other harness decisions are implemented or documented from their actual file-resolution behavior.

## Validation

- Add focused generated and executable workflow coverage for readable resource and inline fallback behavior.
- Run generation, package inventory, and all five integration install/check tests.
- Run the repository standard full validation.

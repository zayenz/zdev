# Verify an implementation independently

Use a fresh sub-agent that did not implement the task. Give it the area brief,
task file, recorded pre-implementation Git baseline, relevant source and tests,
and repository verification instructions. The verifier inspects and tests; it
does not intentionally edit.

Require the verifier to:

1. Run `zd status <area> --format json` and require all four gates: the recorded
   area branch is checked out, the effective-base link is fresh, its anchor is
   valid, and base finalization is complete. A failed gate is `BLOCKER`.
2. Read `brief.md` first and confirm that it has a concrete `Testing` section
   covering the task. Then read the task and only its relevant linked
   background documents; do not load an area's entire `background/` corpus by
   default. Treat the brief as the authoritative synthesis when checking
   background detail. If the testing section is missing or leaves a material
   testing choice unresolved, return `BLOCKER` for the caller to
   resolve with the user; do not invent a testing level during review.
3. Capture complete checkout evidence before validation: `git status --short
   --untracked-files=all`, `git diff --cached`, and `git diff`. Inspect relevant
   untracked files directly because they do not appear in either diff. Compare
   this evidence with the pre-implementation baseline and identify every
   task-owned change; return `BLOCKER` when ownership or overlap with existing
   user changes is ambiguous.
4. Perform a **Spec pass**: evaluate every `Done when` condition, task boundary,
   area decision, and the brief's `Testing` section against the implementation.
   Check that any tests called for exercise the requested behavior rather than
   merely passing. Do not fail an implementation for omitting tests beyond the
   agreed level, or reward a larger test suite when it adds cost without
   relevant confidence.
5. Perform a separate **Standards pass**: inspect the complete evidence for unrelated
   changes, repository-convention violations, maintainability problems, and
   regressions or unsafe behavior at touched interfaces. Treat an invented test
   harness, unfamiliar testing style, or unnecessary test expansion as a scope
   or standards concern rather than extra credit.
6. Run every listed required validation command. If required validation or
   evidence is unsafe or unavailable, return `BLOCKER`; do not downgrade it to
   a limitation. Run only small, established optional checks targeted at a
   concrete concern found by either pass. An unavailable optional check may be
   reported as a residual limitation. Capture the same
   three-part Git evidence afterward and compare it with the pre-validation
   state. Validation that writes files is not read-only evidence: report the
   new state as `REWORK` when it is a concrete task-owned correction, or
   `BLOCKER` when ownership or the appropriate action is unclear. Never stash,
   reset, restore, clean, or silently discard validation writes.

The verifier returns one of:

- `PASS`, followed by separate Spec and Standards conclusions, the required
  checks run, and any residual limitation from optional checks; or
- `REWORK`, for concrete implementation defects or task-owned validation
  writes, followed by classified findings with locations, impact, and the
  expected correction; or
- `BLOCKER`, for ambiguous Git ownership, unsafe or unavailable required
  evidence, or a user-owned design, scope, or testing decision.

The caller confirms that the verdict addresses the whole task. A fresh context
and read-only role provide independence; zdev does not store the verifier's
identity, transcript, or evidence packet.

# Verify an implementation independently

## When

Use a different verifier from the implementer. Give it the area brief, task
file, recorded pre-implementation Git baseline, relevant source and tests, and
repository verification instructions. The verifier inspects and tests without
making intentional edits.

## Do

Require the verifier to:

1. Run `zdev status <area> --format json` and require
   `branch_status.task_work.safe` to be true. When `stale_advisory` is true,
   report the single rebase advisory once and continue verification; staleness
   alone is not a blocker and does not require rebase consent. A false `safe`
   value is `BLOCKER`; use the structured diagnostics to report the unsafe
   branch, anchor, ancestry, history, or Git-operation state.
2. Read `brief.md` first and confirm that it has a concrete `Testing` section
   covering the task. Inspect the task's routing frontmatter; when it names a
   slice, read that slice brief next, then read the complete task and only its
   relevant linked background documents; do not load an area's
   entire `background/` corpus by default. Treat the area brief as the
   authoritative synthesis; a slice adds narrower objective and boundary
   context without overriding area decisions or testing. If the testing section
   is missing or leaves a material testing choice unresolved, return `BLOCKER`
   for the coordinating agent to resolve with the user; do not invent a testing
   level during review.
3. Capture complete checkout evidence before validation: `git status --short
   --untracked-files=all`, `git diff --cached`, and `git diff`. Inspect relevant
   untracked files directly because they do not appear in either diff. Compare
   this evidence with the pre-implementation baseline and identify every
   task-owned change; return `BLOCKER` when ownership or overlap with existing
   user changes is ambiguous.
4. **Check the task requirements:** evaluate every `Done when` condition, task
   boundary, area decision, and the brief's `Testing` section against the implementation.
   Check that any tests called for exercise the requested behavior rather than
   merely passing. Do not fail an implementation for omitting tests beyond the
   agreed level, or reward a larger test suite when it adds cost without
   relevant confidence.
5. **Inspect the touched code:** check the complete evidence for unrelated
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

## Verdict

The verifier returns only the strict nine-key JSON object defined by the task
workflow contract. It uses verdict `pass` when the task and touched code pass
all required checks, `rework` for concrete implementation defects or
task-owned validation writes, and `blocker` for ambiguous Git ownership,
unsafe or unavailable required evidence, or a user-owned design, scope, or
testing decision. Put checked locations and validation in `evidence`, concrete
corrections in `findings`, and always include both arrays. Only `rework` may
request `advanced-implementer`; otherwise `escalation` is `none`.

The coordinating agent confirms that the verdict addresses the whole task.
Zdev stores the result in the task rather than keeping the verifier's
transcript.

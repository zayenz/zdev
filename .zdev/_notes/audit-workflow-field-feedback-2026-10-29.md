# Audit workflow feedback from the Gecode instance-reader review

29 October 2026. Claude Code (Opus 5.5) inside T3 Code, macOS. The audit was
run with the installed `zdev:zdev-audit` workflow; it matches
`templates/zdev/claude/workflows/zdev-audit.js` at `b0c3139` apart from the
coding-guidance path.

The user asked for an independent zdev review of a Gecode branch, and for a
draft PR once the review was happy. The workflow was run five times: one run
was stopped after a redesign, and four ran to completion with no explicit
lenses. All four completed runs returned `BLOCKER zdev-audit` with "the
verifier returned an invalid envelope" and "Failed stage: validation", so the
workflow never reached a pass. The reviews themselves were good. Each run
found real defects, most of them reproduced with sanitizer builds, and the
findings shrank from a crash and O(n³) data access in round one to two medium
hardening issues in round four. The problem is in the workflow around the
reviewer, not in the review.

## What to keep

The verifier's reviews were worth their cost (110k to 180k tokens each). They
reproduced claims with ASan/UBSan, named the reviewed commit, separated
confirmed fixes from new findings, and graded severity sensibly. When the last
run was scoped to "verify the previous fixes and report only medium or higher
defects", the reviewer followed that scope and listed lower-severity items as
optional notes. Keep that behaviour.

## 1. Markdown field labels make every result invalid

`isAuditResult` requires the literal strings `\nBoundary:`, `\nInspected:`,
`\nOmitted:` and `\nChecked evidence:`. In all four completed runs the
verifier wrote the labels in Markdown bold. Running the workflow's own check
over the stored journals gives:

| Run | First line | Label form | Valid |
| --- | --- | --- | --- |
| wf_01845ed5-4f9 | `BLOCKER zdev-audit` | `**Boundary**` | no |
| wf_4308861d-44b | `BLOCKER zdev-audit` | `**Boundary:**` | no |
| wf_459fe775-8a3 | `FINDINGS zdev-audit` | `**Boundary**` | no |
| wf_70ba250f-b9a | `FINDINGS zdev-audit` | `**Boundary:**` | no |

The first line was always correct, and every required section was present.
Only the emphasis around the label differed. The contract asks the body to
"name" the sections, which a model reasonably satisfies with a bold heading.

**First fix:** accept common Markdown forms of the labels: optional `**` or
`__` around the label, an optional colon inside or outside the emphasis, and
an optional `#` heading prefix. Alternatively, normalize the worker output by
removing emphasis before validating. The more robust fix is to request the
envelope through the workflow `schema` option and render the public text
envelope in the script, so formatting can no longer decide the outcome.

## 2. A format failure silently skips the independent check

The evidence vetter only runs when the first result passes `isAuditResult` and
starts with `FINDINGS zdev-audit`. In the two runs that returned `FINDINGS`
with bold labels, validation failed, so the vetter never started. The
workflow then reported a validation blocker as if the verifier had misbehaved,
and the second-reader check that the contract requires never happened. The
coordinator was left with unchecked candidate findings and had to judge them
alone, which is exactly what the vetting stage exists to prevent.

**Fix:** validate leniently before deciding whether to vet (see 1). When
validation still fails, report that the envelope was malformed, list the
missing or unrecognized sections, and say that vetting was skipped. "The
verifier returned an invalid envelope" names neither the cause nor its
consequence.

## 3. The initial reviewer is given the coordinator's duties

The first worker receives the whole `auditContract`, including "someone other
than their author must inspect the cited code ... otherwise use a different
fresh verifier". A `zdev:zdev-verifier` cannot start agents. In the first two
runs the reviewer concluded that it could not arrange the independent check
and returned `BLOCKER zdev-audit`, saying that the session had no way to
start a second verifier, even though the workflow would have run the vetter
itself. In the later runs the same reviewer returned `FINDINGS` but still
added that "no independent second reader has checked them yet, so
coordination must do that check".

**Fix:** give the initial reviewer a role-specific instruction: return
`FINDINGS` with candidate findings, and leave the independent check to the
workflow. The parts of the contract about choosing and running vetters belong
to the coordinating script, not to the worker prompt.

## 4. Smaller observations

- When the runs ended in a blocker, the usable content was only in "Raw worker
  result". In the task notification that section was cut off near its end, so
  the full findings had to be read from the task output file. A result that
  ends in a validation blocker should still put the candidate findings where a
  coordinator will read them, or say where the full text is.
- A rerun after fixes had to restate the previous findings by hand so the
  reviewer would verify them. The workflow could accept a previous audit
  result, or a reviewed commit, and ask the reviewer to confirm earlier
  findings before looking for new ones.
- The tests in `tests/lean.rs` (around line 9884) use only plain-label
  fixtures. Add a fixture with bold labels and one with heading labels, and
  check that a `FINDINGS` result in those forms reaches the vetter.

## Suggested order

1. Lenient label validation, or schema-based output (fixes all four runs).
2. Role-specific prompt for the initial reviewer.
3. Clearer blocker text when validation fails, including that vetting was
   skipped.
4. Regression fixtures for Markdown-formatted envelopes.

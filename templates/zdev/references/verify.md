# Verify an implementation independently

## When

Use a different verifier from the implementer. Give it repository-relative
locators for the area brief, task, relevant source and tests, repository
verification instructions, and the recorded pre-implementation baseline
snapshot. The verifier inspects and tests without making intentional edits.
It receives the coordinator-stored opaque snapshot and shows it before review.

## Do

Require the verifier to:

1. Coordination runs `zdev work-context <area> --store --format json`
   immediately before dispatch, accepts only its compact locator for the
   requested area, task, and HEAD, then inspects the exact context with `zdev
   work-context <area> --show <snapshot> --format json`. Require the requested
   task to remain open, ready, and safe in its
   nested status and goal projections. When `stale_advisory` is true, report the single rebase
   advisory once and continue verification; staleness alone is not a blocker.
   Trunk mode has no rebase or freshness ceremony; its resolved configured
   trunk must simply remain safe and checked out.
   A command failure or unsafe context is `BLOCKER`; report its structured
   diagnostics.
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
3. Use the coordinator-stored work-context Git strings as complete
   pre-validation evidence. Inspect relevant untracked files directly because
   they do not appear in either diff. Compare this evidence with the
   pre-implementation baseline and identify every task-owned change; return
   `BLOCKER` when ownership or overlap with existing user changes is ambiguous.
   Other areas sharing trunk and unrelated trunk paths remain outside this
   task's ownership.
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
   reported as a residual limitation. After the semantic response,
   coordination runs `zdev work-context <area> --compare <snapshot> --format
   json` and requires the exact compact comparison for that area and snapshot.
   Validation that
   writes files is not read-only evidence: report the
   new state as `REWORK` when it is a concrete task-owned correction, with one
   exact `validation_write: <normalized repository-relative path>` finding per
   written file, or
   `BLOCKER` when ownership or the appropriate action is unclear. Never stash,
   reset, restore, clean, or silently discard validation writes.

## Verdict

The verifier returns only the strict four-field semantic JSON object defined by
the task workflow contract: `verdict`, `summary`, `findings`, and `escalation`.
Its summary carries checked locations and validation conclusions. It uses
verdict `pass` when the task and touched code pass
all required checks, `rework` for concrete implementation defects or
task-owned validation writes, and `blocker` for ambiguous Git ownership,
unsafe or unavailable required evidence, or a user-owned design, scope, or
testing decision. Put concrete corrections in `findings`. Only `rework` may
request `advanced-implementer`; otherwise `escalation` is `none`. The
coordinator accepts that request at most once and only after standard/default
implementation. Verification itself always uses a fresh standard verifier.

The coordinating agent confirms that the verdict addresses the whole task,
runs the post-response comparison, and generates the compatible public
nine-key envelope including identity, snapshot evidence, and optional stale
advisory. On completion it writes a concise result and validation summary to
the task. Explicit verification returns that envelope without mutation.

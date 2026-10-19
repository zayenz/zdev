# Review and feedback

Use [coding guidance](coding.md) and applicable supplements as acceptance
criteria alongside task completion. Preserve the route's ownership, snapshot,
validation, and result contracts. The verifier is also the review agent.

## Standalone review

`zdev review` selects this read-only route. Review the user's specified diff,
commit range, branch, files, or code boundary. With no target, inspect staged,
unstaged, and relevant untracked local changes. State that selection; for a clean
checkout, report no local changes and stop without a worker. Ask only when a
supplied target or comparison base is ambiguous. Resolve named Git references before
review and keep their commit IDs in the handoff.

This route works without `.zdev`; keep task records and Git state unchanged.
If the request names an area, read its brief and relevant task or slice records
as context. Task readiness and branch admission gates belong to **Verify**;
standalone review does not accept, complete, or commit a task.

The coordinator owns selection, dispatch, finding checks, and final reporting;
the verifier inspects only its assigned scope. Retain HEAD, Git status, both
local diffs, and relevant untracked contents in the existing handoff so the
reviewed state can be compared before reporting. Resolve
and freeze only the `verifier` profile under the shared scoped-profile rules,
including the area when supplied. Give one fresh verifier the review target,
comparison base, repository instructions, supplied coding guidance path, and
relevant requirements and validation commands. Tell it to follow **Full review**
below and return a textual review instead of task-verification JSON. A reviewer
must be independent of the implementation being reviewed.

Check substantive findings under **Full review**, then compare the reviewed
state. Changed code or unavailable required evidence prevents a pass; preserve
the work and identify the gap. Use established checks suited to the change and
read-only or dry-run forms where available. Temporary reproducers may live
outside the repository; remove them when finished. Report unexpected validation
writes and preserve them. Review alone authorizes no repairs or task creation.

Begin the report with `PASS zdev-review`, `FINDINGS zdev-review`, or
`BLOCKER zdev-review`. State the inspected scope, checked findings with
`path:line`, consequence and proportionate correction, validation, and limitations.
Summarize the naming and decomposition assessment as well as behavior. Keep
optional suggestions separate from required corrections. A pass means the
inspected scope meets the applicable requirements with no required corrections.

## Full review

1. Retain the inspected state through the route's existing snapshot or handoff.
   Read the whole change from its entry point, relevant callers, and both sides
   of changed contracts. Check requirements, readability, applicable coding
   rules, regressions, and required validation. Derive dependency direction from
   the project's actual modules and layers; check that behavior belongs in its
   owning module. Follow invariants through alternate legal entry points, reused
   objects, copies, replay, caching, and filtering where relevant. Check whether
   examples teach the public interface. These are inspection prompts, not a
   required test matrix. Continue through the whole change after finding a defect.
   Missing design history leaves the process unknown.
2. **Check naming and decomposition explicitly.** Apply **Names** and **Structure
   and control flow** in the coding guidance to the changed functions, types,
   and modules. Read names at callers and inspect responsibility boundaries and
   the main call sequence. Require a correction when a name misstates an effect
   or forces readers to reconstruct its meaning, or mixed responsibilities force
   readers to track unrelated details. State the reasoning burden and a useful
   rename or split; a runtime failure is not needed for a readability finding.
   Judge retained long functions by cohesion and clear invariants. Arbitrary size
   limits, extra layers, and personal word preferences do not justify rework.
3. Give every required correction a checked location, concrete consequence, and
   proportionate remedy. Search before claiming an omission; seek counterevidence.
   Require readability rework when you can explain the misunderstanding or
   reasoning burden, even if tests pass. Distinguish required corrections from
   optional suggestions and personal preferences.
4. Have another reader check substantive findings against the same state before
   reporting them as established or directing repairs. The main agent can do
   this when independent of their author; otherwise use a fresh review agent
   under the route's worker rules. Check only the claims, without recursively
   reviewing that check. Zero findings and mechanical validation-write markers
   need no second reader. Correct or withdraw overturned claims with reasons;
   unresolved claims remain open questions.
5. Report all supported corrections together. In task verification, `findings`
   contains required corrections only; label optional suggestions in `summary`.
   An optional suggestion never blocks a pass. State missing evidence and
   limitations honestly. Include a concise naming and decomposition assessment
   in the existing summary, identifying consequential issues or why the changed
   boundaries are adequate; avoid boilerplate assertions of compliance.
   A review with no findings is valid. Required evidence and accepted corrections
   must be resolved before acceptance.

## Bounded corrections

For ordinary task implementation, use one full review, one correction pass,
then one focused recheck. Standalone review, explicit verification, and audits
report feedback; repairs require implementation authority. Research routes
retain their own attempt budgets.

The main agent checks findings before dispatching corrections. Return accepted
corrections together to the implementation agent, with the prior review and
baseline. Then give a fresh review agent the prior review, accepted corrections,
and updated state. Recheck the corrections, affected contracts, and regressions
caused by the repair. Retain conclusions for unchanged code and run required
validation. Report any newly discovered serious defect; a recheck does not
restart a general search for improvements.

If required corrections remain after the recheck, stop automatic repair. Keep
the task open, preserve the work, and report the remaining problem and decision
needed as a blocker. Further repair requires an explicit user continuation
decision. Optional suggestions do not consume another pass; reaching the limit
does not justify acceptance.

Carry the review stage and prior findings in existing handoffs across worker
replacement, interruption, recovery, and goal/loop continuation. These events
must not reset the correction allowance. Unexpected state changes invalidate
review evidence: preserve the work and report the mismatch rather than silently
restarting the cycle.

## Death loops

Watch for repeated actions with no meaningful progress: the same failing fix,
unchanged command or approval retry, alternating edits that undo each other,
restarting reviews, or splitting tasks repeatedly without implementing work.
A new worker, task ID, or refreshed snapshot does not itself count as progress.

When a failure repeats, compare the last attempts and identify what changed in
the code, evidence, or remaining requirements. Before retrying, choose a
concrete different approach with a checkable reason it could help. Continue
while evidence shows progress; a long build or difficult task alone is not a
death loop. Respect the route's existing correction and attempt budgets.

If the changed approach returns to the same failure without useful new
evidence, end automatic retries. Preserve the work, stop owned workers, and
report the repeated failure, attempts, and the specific input or external
change needed to continue. Never relax acceptance criteria or report success
to escape the loop. Carry this history in existing handoffs across splitting,
replacement, recovery, and continuation; these events do not reset the budget.

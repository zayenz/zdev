# Review and feedback

Use [coding guidance](coding.md) and applicable supplements as acceptance
criteria alongside task completion. Preserve the route's ownership, snapshot,
validation, and result contracts. The verifier is also the review agent.

## Full review

1. Retain the inspected state through the route's existing snapshot or handoff.
   Read the whole change from its entry point, relevant callers, and both sides
   of changed contracts. Check requirements, readability, applicable coding
   rules, regressions, and required validation. Continue through the whole change
   after finding a defect. Missing design history leaves the process unknown.
2. Give every required correction a checked location, concrete consequence, and
   proportionate remedy. Search before claiming an omission; seek counterevidence.
   Require readability rework when you can explain the misunderstanding or
   reasoning burden, even if tests pass. Distinguish required corrections from
   optional suggestions and personal preferences.
3. Have another reader check substantive findings against the same state before
   reporting them as established or directing repairs. The main agent can do
   this when independent of their author; otherwise use a fresh review agent
   under the route's worker rules. Check only the claims, without recursively
   reviewing that check. Zero findings and mechanical validation-write markers
   need no second reader. Correct or withdraw overturned claims with reasons;
   unresolved claims remain open questions.
4. Report all supported corrections together. In task verification, `findings`
   contains required corrections only; label optional suggestions in `summary`.
   An optional suggestion never blocks a pass. State missing evidence and
   limitations honestly. A review with no findings is valid. Required evidence
   and accepted corrections must be resolved before acceptance.

## Bounded corrections

For ordinary task implementation, use one full review, one correction pass,
then one focused recheck. Explicit verification and audits report feedback;
repairs require implementation authority. Research routes retain their own
attempt budgets.

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

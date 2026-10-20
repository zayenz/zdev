# Review and verification

Review asks whether code meets its requirements and the
[coding guidelines](coding-guidelines.md). Verification applies that review to
one task and checks the evidence needed to accept it. It includes code review
and required validation, rather than just running tests.

The agent command names distinguish the scope and stopping point:

| Request | What it checks | Where it stops |
| --- | --- | --- |
| [Review](commands/review.md) | A change, commit range, or named code boundary; an area can supply context. | Findings or a pass, with no repairs or task completion. |
| [Verify](commands/verify.md) | The area's current ready task, its brief, coding guidelines, and required validation. | A verification result, without intentional code edits, task completion, or a commit. |
| [Implement](commands/implement.md) | One authorized task through implementation and independent review and verification. | A completed task and commit after acceptance, or preserved work and a blocker. |

Standalone Review is being added to the integrations. If your installed
integration does not include it, [Audit](commands/audit.md) can inspect a named
boundary. Verify remains the task-specific request. These are agent requests,
not `zdev review` or `zdev verify` terminal subcommands.

## What the agent reports

Ask for task verification in chat:

```text
Use zdev to verify result-export-001 in area result-export.
Report the result without changing code or committing it.
```

Replace that ID with the area's current ready task. A fresh verifier reads the
brief and task, inspects the change and affected callers, and runs the required
validation. Substantive findings are checked by someone other than their author
before they become accepted corrections.

A concise report might look like this:

```text
result-export-001: pass.
Requirements: JSON results, empty results, and default text output checked.
Code review: names and responsibility boundaries fit the existing CLI.
Validation: agreed CLI checks passed.
Task remains open; this request did not authorize completion or a commit.
```

This is an illustrative summary; wording varies by harness. The useful parts
are the inspected scope, result, validation, and any limitation. A required
correction should identify its location and consequence. A blocker should say
which decision or evidence is missing. Optional suggestions do not turn a pass
into a failure.

## How implementation uses the result

Implement includes independent review and verification. You do not need separate Review
and Verify requests for an ordinary task. The verifier checks behavior and
code quality in the same acceptance step.

Confirmed corrections receive one correction pass and a focused recheck.
Unresolved required corrections leave the task open. After a pass, the
coordinator can record completion and commit the accepted changes under the
existing implementation authority. The [feature walkthrough](feature-walkthrough.md)
shows that sequence; the [workflow reference](workflow.md#verification) gives
the detailed acceptance rules.

# Verify the current ready task

Use verify for independent review and verification of an existing implementation
against its task. Name both the area and the task ID. The task must be the area's current
ready task and the checkout must be safe for task work.

Verification includes code review against the [coding guidelines](../coding-guidelines.md)
and the task's required validation. It is the task-specific acceptance check;
[standalone Review](review.md) checks a change without accepting a task.

Replace the example task ID with your current ready task.

## Examples

Paste either request into your coding-agent chat:

```text
Use zdev to verify greeting-001 in area greeting.
```

```text
Use zdev to independently verify greeting-001 in area greeting
against its brief and agreed validation. Report the result without changing code.
```

## What happens

A fresh verifier reads the brief and full task, checks the touched code, and
runs the required validation. For the greeting change, that includes the
specified named greeting and preserved default behavior. Verification follows
the agreed testing level; it does not demand unrelated additional tests.

The result is a pass, required rework, or blocker. Findings identify concrete
corrections, while a blocker explains missing evidence, an unsafe state, or a
decision that needs you. The reviewed checkout is compared before acceptance,
so changed code cannot inherit an earlier pass.

Explicit verification ends at the result. It does not repair the code, mark the
task done, or commit it. [Implement](implement.md) includes those later steps
after its own independent verification. Use [recover](recover.md) if the work
was interrupted partway through that flow.

For a diff or code boundary that is not the current task, use [audit](audit.md)
or [standalone review](review.md), if your installed integration supports it. See
[verification](../workflow.md#verification) for the detailed acceptance rules.
The [Review and verification guide](../review-and-verification.md) includes an
example of the report you receive.

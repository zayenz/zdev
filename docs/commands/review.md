# Review a change

Use standalone review when you want feedback on local changes, a commit range,
branch, or code boundary. The standalone review route is being added to the
integrations. Until your installed integration includes it, use
[audit](audit.md) for a bounded code inspection.

## Examples

When your installed integration supports Review, paste either request into
your coding-agent chat:

```text
Use zdev review to check the local changes for area greeting.
```

```text
Use zdev review to check hello.py against the greeting brief.
Report required corrections separately from optional suggestions.
```

## What happens

With the standalone route available, the agent selects staged, unstaged, and
relevant untracked local changes by
default. You can name a different target, such as `hello.py` or a commit range.
It states what it will inspect. If the checkout has no local changes and no
other target was named, it reports that and stops.

A fresh reviewer checks the change, callers, requirements, naming, and
responsibility boundaries. Substantive findings are checked independently
before the report is accepted. Expect a result beginning with
`PASS zdev-review`, `FINDINGS zdev-review`, or `BLOCKER zdev-review`, followed by
the inspected scope, locations and consequences, validation, and limitations.
Optional suggestions are separate from required corrections.

The review ends at feedback. It works without `.zdev` and does not repair code,
create tasks, accept an existing task, or make a commit. Naming an area supplies
its brief as context; it does not make task readiness a requirement.

Use [verify](verify.md) when you need acceptance evidence for the current ready
task. Use [implement](implement.md) to carry an authorized task through fixes,
verification, and commit. The [workflow reference](../workflow.md#coding-and-review-guidance)
describes the shared review standards.

The [Review and verification guide](../review-and-verification.md) explains how
these requests differ. Both assess the [coding guidelines](../coding-guidelines.md).

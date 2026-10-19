# Audit a code boundary

Use audit to inspect a named part of the repository and receive independently
checked findings. A boundary can be a file, directory, or subsystem. Without
one, the audit covers the current repository.

## Examples

Paste either request into your coding-agent chat:

```text
Use zdev to audit hello.py in the context of area greeting.
Check argument parsing and preservation of the default greeting.
```

```text
Use zdev to audit the command-line interface for correctness and usability.
Use separate review lenses for those two concerns.
```

## What happens

The agent assigns a fresh reviewer to the boundary. Someone other than the
author of a substantive finding checks its location and claimed consequence
before it reaches the final report. You can request up to four explicit
review lenses, such as correctness and security; these use separate reviewers.

The report begins with one of these results:

| Result | Meaning |
| --- | --- |
| `PASS zdev-audit` | The checked evidence produced no findings. |
| `FINDINGS zdev-audit` | The report contains supported findings with locations and impact. |
| `BLOCKER zdev-audit` | A required review or evidence check could not be completed. |

Expect the boundary, inspected and omitted material, and checked evidence to
be named. An audit ends at this report. Repository files, Git state, and task
status stay unchanged; fixing findings is a separate request.

For a general list of promising work, use [improve](improve.md). For acceptance
of the current ready task, use [verify](verify.md). See the
[workflow reference](../workflow.md#coding-and-review-guidance) for review standards.

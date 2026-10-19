# Find useful improvements

Use improve when you want a broad assessment of worthwhile work. It is useful
before deciding which objective deserves an area or which findings to take on
next.

## Examples

Paste either request into your coding-agent chat:

```text
Use zdev to improve the code around hello.py, using the greeting brief as
context. Look for correctness and usability problems worth fixing in a small
command-line tool. Recommend only changes whose benefit justifies the effort.
```

```text
Use zdev to improve this repository. Look for a small set of changes
that would make it easier to maintain, and explain their likely benefit.
```

## What happens

The agent inspects the requested code, relevant tests, and project conventions.
It checks candidate findings before reporting them, then orders the useful
ones by impact. Each retained finding includes a source location, its effect,
estimated effort, risk, confidence, and a short recommendation. Product ideas
are identified separately from defects.

The result is advice in the conversation. The agent names the inspected scope
and any gaps, recommends a small set of changes, and stops. It leaves source
and task records unchanged. Improve also works in a repository without `.zdev`
and does not initialize one.

Use [investigate](investigate.md) for one specific uncertainty or
[audit](audit.md) for a bounded inspection with a formal findings result.
Selected improvements can become an [area brief](explore.md) and then
[tasks](tasks.md). The [workflow reference](../workflow.md#coding-and-review-guidance)
describes the coding and review standards used during inspection.

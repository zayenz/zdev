# Investigate one question

Use investigate when a concrete uncertainty prevents a good decision. The
agent can research external facts, diagnose a symptom, or build a small
throwaway prototype to answer it.

## Examples

Paste either request into your coding-agent chat:

```text
Use zdev to investigate whether hello.py can support --name "Ada Lovelace"
with the Python standard library alone. Use area greeting as context and
report the smallest workable approach and any compatibility limits.
```

```text
Use zdev to investigate why hello.py rejects names containing spaces.
Reproduce the behavior and report the cause before proposing production changes.
```

## What happens

The agent states the question, gathers evidence, and separates what it observed
from what it inferred. A diagnosis reproduces the symptom and checks a proposed
cause. A prototype stays separate from production work.

You receive the question, observations, conclusion, confidence, and limitations.
When the answer settles a planning question in `greeting`, the agent updates
the relevant part of its brief. A standalone investigation works without
`.zdev` and creates no durable record unless you ask to save one.

Investigation stops at the answer. Production changes and task creation need
their own request. To keep useful material for later, ask to save it as a
[note](notes.md). An authorized investigation task can retain reusable sources
under the area's `background/` directory and link them from the brief.

Use [improve](improve.md) for broad discovery or [research](research.md) for a
measured sequence of optimization experiments. The
[user guide](../user-guide.md#5-explore-and-discuss-the-objective) covers how
investigation informs planning.

# Areas and briefs

An area keeps the work toward one objective together. It contains the agreed
brief, the tasks, and the record of what happened. The objective can be a
specific feature or a question whose answer will determine what to build.

For example, "make query evaluation faster" is a reasonable starting point for
an area. It is not yet a task. You still need to decide which queries matter,
what must stay compatible, and how to tell whether a change helped.

## Start with an objective

In your coding-agent chat, ask:

```text
Use zdev to explore an area named evaluation. We want query evaluation to
be faster, but do not yet know where the time goes. Preserve the language
and query results. Help define the scope and useful evidence first.
```

The agent inspects the project and works with you on the brief. It reads facts
from the repository and asks about choices that need your judgment. It should
not turn the first plausible approach into a commitment before those choices
are settled.

The brief usually describes:

- the result you want and what would count as success;
- the boundaries, including behavior that must remain unchanged;
- decisions already made and questions still open;
- the testing and validation appropriate to this work.

A useful brief gives the next agent enough context to make sound decisions.
It need not predict every file or implementation detail.

## What the area stores

The `evaluation` area lives under `.zdev/evaluation/`:

```text
.zdev/evaluation/
  area.toml
  brief.md
  TASKS.md
  tasks/
```

`brief.md` holds the shared decisions. Individual files in `tasks/` describe
the work; `TASKS.md` is a generated index. Area metadata records its branch
relationship and lifecycle. Research work can also have settings and evidence,
which the [research walkthrough](research-walkthrough.md) explains.

The record policy determines whether these files stay local to your clone or
are committed for other people to read. The
[setup guide](getting-started.md#make-a-small-repository) explains the choice.
Source-code commits and the planning record's storage policy are separate.

## Choose a useful boundary

Keep related decisions in one area. A performance objective may include
profiling, several experiments, and supporting changes to the benchmark. Those
steps share the same compatibility and measurement rules.

Create another area when the objective or its boundaries are independent.
Adding a new language feature should not quietly become part of an area that
promises to preserve the language. A short note under `.zdev/_notes/` is enough
for an idea you are not ready to explore.

An area normally belongs to a feature branch. Several areas can share a
branch or use explicit trunk mode when that is the chosen workflow. The
[branch reference](workflow.md#branch-orientation) covers the details; the
important thing during a run is to use the checkout and branch reported by the
agent.

## An area can outlive a task or conversation

An area may begin with discussion, produce a few tasks, and continue after new
evidence changes the plan. The brief should reflect the current decisions.
Useful research that later work needs can be saved under `background/` and
linked from the brief.

Completing all its tasks leaves the area open. Review whether the objective
has actually been met before closing it. A finished experiment may establish
that an approach does not work, leaving the wider objective unresolved.

Next, read [Tasks and completion](working-with-tasks.md) for how the objective
becomes work an agent can finish and verify.

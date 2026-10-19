# Explore an objective

Use explore when you can describe the desired result but still need to settle
its scope, behavior, or testing. It creates or revises the area's `brief.md`.
You can also say "shape" or "wayfind".

## Examples

Paste either request into your coding-agent chat:

```text
Use zdev to explore area greeting: add --name to hello.py so that
python hello.py --name Ada prints Hello, Ada! Preserve the default greeting.
Keep the change small and agree on the necessary validation before making tasks.
```

```text
Use zdev to shape area greeting from its existing brief. Add support
for names containing spaces and settle how an empty name should behave.
```

## What happens

The agent reads the existing code and repository guidance, then asks about
choices that affect the result. For this example, the behavior of an empty name
may need a decision; the current default greeting can be read from the code.

The resulting `.zdev/greeting/brief.md` records the observable outcome,
boundaries, settled decisions, open questions, testing level, and validation.
Testing may mean existing checks or a focused manual check when that is enough.
Creating a test framework is not a prerequisite for a small script change.

Exploration ends by checking and presenting the brief. You can then
[discuss](discuss.md) an unresolved choice or [create tasks](tasks.md). If you
already know you want both steps, say "explore greeting, then create tasks".
Changes to project documentation outside `.zdev` need to be part of your
request or a separately agreed edit.

See [planning and task creation](../workflow.md#planning-and-task-creation)
for the detailed workflow and [setup](setup.md) for the first area in a repository.

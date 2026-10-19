# Discuss the brief

Use discuss to challenge decisions in an existing brief before committing to a
task split. "Grill the brief" selects the same interaction.

## Examples

Paste either request into your coding-agent chat:

```text
Use zdev to discuss the greeting brief. Challenge the --name behavior with
concrete examples, especially empty input and names containing spaces.
Keep the testing effort proportionate to this small script.
```

```text
Use zdev to grill the greeting brief, then create tasks once the
behavior and validation are settled.
```

## What happens

The agent reads the brief and relevant code before asking questions. It checks
repository facts itself and brings you decisions about intended behavior or
trade-offs. Questions come in small rounds, with a recommendation and its cost.

As you decide, the agent updates `.zdev/greeting/brief.md`. The file keeps the
conclusions, such as whether `--name ""` is an error, so a later implementer
does not have to reconstruct the conversation.

Discussion stops when no unresolved choice would materially change the
objective or task split. Small implementation details can remain for the task.
It checks the finished brief and reports the settled decisions. To proceed
directly, ask to "discuss the greeting brief, then create tasks"; existing
authority to import those tasks is reused.

Use [explore](explore.md) when there is no brief yet, or [create tasks](tasks.md)
when it is already clear enough. The [workflow reference](../workflow.md#planning-and-task-creation)
describes how the brief supplies shared decisions to tasks.

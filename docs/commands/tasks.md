# Create tasks from the brief

Use create tasks when an existing area's brief settles the desired behavior
and testing. Each task should describe one useful result that can be checked
and committed on its own.

## Examples

Paste either request into your coding-agent chat:

```text
Use zdev to create tasks for greeting from its brief. Keep the --name change
small enough for one task if it can be implemented and verified together.
Show me the task bundle before importing it.
```

```text
Use zdev to create and import tasks for greeting without asking me
to approve the bundle. Stop after import; do not implement them.
```

## What happens

The agent reads the brief and relevant code, drafts the tasks, and checks the
split. For non-trivial work, another agent challenges the draft. You then see
the exact bundle proposed for import, including outcomes, boundaries,
dependencies, done conditions, and validation.

After approval, zdev allocates IDs such as `greeting-001`, writes the individual
task files, and regenerates `.zdev/greeting/TASKS.md`. The report lists the IDs
and tasks that are ready to start. With project or pull-request records, the
normal import also commits the planning records; personal records remain local.

Either form stops after import unless you also authorize implementation. Task
creation requires an existing area; start with [explore](explore.md) if it is
missing. Use [implement](implement.md) for one imported task or [loop](loop.md)
for the approved queue. See the [task format](../task-format.md) and
[reviewed import guide](../user-guide.md#6-import-reviewed-tasks) for details.

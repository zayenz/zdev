# Save notes for later

Use notes for an idea, unfinished plan, or research collection that you are not
ready to turn into tasks. Notes can exist before an area does.

## Examples

Paste either request into your coding-agent chat:

```text
Use zdev to save a note for a future greeting area: hello.py should accept
--name, but I have not decided how an empty name should behave.
```

```text
Use zdev to list my notes.
```

## What happens

The agent saves a descriptive file under `.zdev/_notes/` and reports its path.
It preserves the open question. Related documents can share a folder, with
subfolders when useful; notes have no required template or task status.

To turn a note into an area later:

```text
Use zdev to make area greeting from the greeting note.
```

That request reads the relevant material and starts [exploration](explore.md).
It produces an area brief and keeps the source note by default. Saving or
listing notes ends after reporting the files; making an area stops at the brief
unless you also request tasks.

Notes use ordinary files, so there is no `zdev note` shell command. If saving
the first note requires a repository record, the agent follows [setup](setup.md).
See the [user guide](../user-guide.md) for how notes fit into durable work.

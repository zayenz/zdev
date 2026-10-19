# zdev slice

Create a short brief for a bounded increment within an existing area. A slice
lets related work share an objective and scope boundary without creating
another area.

## Examples

```sh
zdev slice create greeting named-output \
  --title 'Named greeting' \
  --objective 'Include the supplied name in the greeting' \
  --boundary 'Command-line output only'
zdev slice show greeting named-output
```

## Subcommands

```text
zdev slice create AREA KEY --title TITLE --objective OBJECTIVE --boundary BOUNDARY...
zdev slice list AREA
zdev slice show AREA KEY
```

The slice key is a lowercase identifier used for its filename. Supply at least
one `--boundary`, repeating the option for additional boundaries. `list` shows
all slice briefs in an area; `show` reads one brief.

Creating a slice writes its brief. Task creation and import remain separate;
use [tasks](tasks.md) for the task queue. See [task format](../task-format.md)
for associating tasks with briefs and [area](area.md) when the work needs a
separate objective.

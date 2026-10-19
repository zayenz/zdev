# Area reports

An area's task files record what happened, but they can be difficult to read
without knowing the project. The `zdev-area-report` skill uses those files,
the brief and relevant code to write a report for someone who has not followed
the work. It produces editable MDX and an HTML file you can share.

The skill is available in this repository and the Codex plugin's skills
directory. It is separate from `zdev skill install`, which installs the task
workflow skill.

## Request a report

Invoke the companion skill and name the area, audience and question the report
should answer. For example:

```text
Use zdev-area-report to explain the scheduling area for a developer who
has not followed the work. Cover the design, what was verified and the
remaining limitations.
```

The skill asks about the report's scope and reads the material it needs.
Most reports use prose and tables. Small widgets are available when changing
an input would help the reader understand a result.

## Report files

Reports live under `.zdev/reports/<area>/<date>-<name>/`. Each directory contains
the editable `report.mdx` and the built `index.html`. The HTML embeds its
styles and any widgets so readers need no server, Node installation or
adjacent assets.

Reports follow the repository's existing `.zdev` tracking policy. Creating one
does not create a task or automatically commit or publish it. Review the
report's evidence and source links before sharing the HTML.

For dependency setup, fixtures, build commands and recipient-style browser
checks, see the companion skill's [generation guide](../skills/zdev-area-report/references/generation.md).
Its [writing guide](../skills/zdev-area-report/references/writing.md) describes
how to distinguish measured results, implementation facts and remaining work.

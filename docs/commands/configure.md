# Configure zdev

Use configure to inspect or change project settings and worker preferences.
Name the area and harness when changing an area profile, and name the setting
when changing a project value.

## Examples

Paste either request into your coding-agent chat:

```text
Use zdev to show the Codex execution profile for area greeting.
```

```text
Use zdev to make greeting the default area for this project.
```

## What happens

The agent performs the requested read or mutation and reports the resulting
value. Configuration stops there; it does not start tasks or change their
complexity. Worker preferences choose the agents used for tasks and reviews;
they do not change the model running your coordinating conversation.

These are also real shell commands:

```sh
zdev config show
zdev config get project.default-area
zdev config set project.default-area greeting
zdev area profile greeting codex
```

Project settings are local to the initialized repository. Worker preferences
can also have global defaults and local overrides. Named profiles can be
selected for an area, a run, or one role; the
[worker profile guide](../worker-profiles.md#runtime-selection) explains precedence.

When a worker-setting change requires refreshing an installed integration,
zdev reports the exact install command. That refresh is a separate action.
Changing an area's profile mapping does not require a reinstall.

See the [configuration reference](../config-command.md) for supported keys,
scopes, and commands, or [setup](setup.md) to install an integration.

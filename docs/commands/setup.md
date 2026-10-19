# Set up zdev

Use setup to initialize repository records or install and check the skill for
your coding harness. The `zdev` binary must already be available; the
[getting started guide](../getting-started.md) includes installation steps.

## Examples

Paste either request into your coding-agent chat:

```text
Use zdev to set up this repository for a future greeting area. Keep the work
record personal to this clone and check the Codex user integration.
```

```text
Use zdev to check the Claude Code user integration and refresh it
if it is out of date.
```

## What happens

Setup separates two choices. Record ownership determines where `.zdev` belongs.
Integration scope determines where the agent's zdev skill is installed.

| Record policy | Use it when |
| --- | --- |
| `personal` | The record belongs to you and this clone; `/.zdev/` is added to `.git/info/exclude`. |
| `project` | Briefs and tasks should be committed and shared with the project. |
| `pull-request` | Reviewers should see the records on the feature branch, then remove them before squash merge. |

The agent reuses an existing record and checks requested integrations. A user
integration can serve several repositories. A project integration is checked
into one repository and needs a repository-guidance source. If a choice is
still undecided, setup asks for it; an existing choice is reused.

The result reports the record policy and integration status. Setup alone does
not create the `greeting` objective or start implementation. Ask to
[explore greeting](explore.md) next, or include exploration in the setup request.

For manual integration checks, these are shell commands:

```sh
zdev skill install codex
zdev skill check codex --scope user
```

See the [user guide](../user-guide.md#harness-notes) for supported harnesses and
[configure](configure.md) for project and worker settings.

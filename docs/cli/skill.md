# zdev skill

Install or check the integration that teaches a coding harness how to use
zdev. A user installation works across repositories; a project installation
keeps the harness files in the current repository.

## Examples

```sh
zdev skill install codex
zdev skill check codex --scope user
```

## Usage

```text
zdev skill install HARNESS [--scope user|project]
zdev skill check HARNESS [--scope user|project]
```

| Harness value | Application |
| --- | --- |
| `codex` | Codex |
| `claude` | Claude Code |
| `opencode` | OpenCode |
| `pi` | Pi |
| `omp` | Oh My Pi |
| `agy` | Google Antigravity |

User scope is the default. Both subcommands accept `--to PATH` to target an
exact directory, `--portable` for relocatable Codex references, and
`--guidance SOURCE` to select project guidance. Guidance can be `auto`,
`agents`, `zdev`, or a repository-relative Markdown path.

Installation accepts `--force` to replace integration files that differ from
the installed version. A check returns `ok` on success; `missing` and
`conflict` return a nonzero exit status. Use the same scope and options when
checking the installation you created.

See [harness notes](../user-guide.md#harness-notes) for integration details and
[Agent commands](../commands/index.md) for the requests available afterward.

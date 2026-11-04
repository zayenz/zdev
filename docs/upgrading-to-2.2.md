# Upgrade to zdev 2.2

Zdev 2.2 adds goals that cover several areas or every area. Defaults are the
same as in 2.1. Existing areas, briefs, tasks, worker configuration and
integrations remain valid; the record schema stays at version 1. The
[changelog](../CHANGELOG.md) lists every change in the release. From 2.0, read
[Upgrade to 2.1](upgrading-to-2.1.md) first.

## Refresh the binary and integrations

Install the 2.2 binary using your existing installation method. For Cargo:

```sh
cargo install zdev --version 2.2.0 --locked
```

Then refresh each binary-installed harness integration with the same scope and
destination:

```sh
zdev skill install codex --force
zdev skill check codex

# For an existing project installation:
zdev skill install codex --scope project --guidance auto --force
zdev skill check codex --scope project
```

Replace `codex` with your harness. Preserve an existing custom `--to`
destination and guidance selection. Agents learn the new route only through
the refreshed integration. Update the Codex plugin through its normal
installation path.

## Goals across areas

`zdev goal` still accepts one area and prints the same output for it. It now
also accepts several areas in priority order, or `--all` for every open area:

```sh
zdev goal api docs
zdev goal --all --focus "release blockers first"
```

The output lists each covered area's lifecycle, queue, counts and next task,
followed by one `native_goal`. That goal works in the first covered area with
ready, safe work, skips areas without any, and finishes when no covered area
has ready work. JSON output for several areas has `scope`, `areas` and
`ready_areas` instead of the one-area fields; see
[several areas](harness-goals.md#several-areas).

Ask the agent to set a goal over all areas, or over the areas you name. Codex
and Oh My Pi set the goal through their goal tools. Claude Code has no goal
command the model can call, so the skill shows a `/goal` command in a code
block for you to enter; OpenCode and Pi show the bare condition. Inside T3
Code, the condition keeps its `Use zdev with T3 orchestration.` prefix.

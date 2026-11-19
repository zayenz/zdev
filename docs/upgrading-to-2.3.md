# Upgrade to zdev 2.3

Zdev 2.3 adds shadow records, keeps zdev out of the main history for personal
and shadow records, and pauses goals on a confirmed blocker where the agent
can. Existing areas, briefs, tasks, worker configuration and integrations
remain valid; the record schema stays at version 1. The
[changelog](../CHANGELOG.md) lists every change in the release. From 2.1, read
[Upgrade to 2.2](upgrading-to-2.2.md) first.

## Refresh the binary and integrations

Install the 2.3 binary using your existing installation method. For Cargo:

```sh
cargo install zdev --version 2.3.0 --locked
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
destination and guidance selection. The refreshed integration carries the
shadow staging rules, the trailer change and the goal pause. Update the Codex
plugin through its normal installation path.

## Shadow records

A shadow record keeps `.zdev` out of the repository it plans and versions it in
its own Git repository at `.zdev/.git`, optionally pushed to a remote. Start a
new one, or convert an existing personal record:

```sh
zdev init --record shadow --shadow-remote git@example.com:me/project-zdev.git
zdev shadow adopt --shadow-remote git@example.com:me/project-zdev.git
```

In a fresh clone of the main repository, the same `init` command clones the
shadow record from its remote. The shadow repository's branch follows the main
repository's branch, and `zdev commit` commits the record alongside the main
commit. Set `zdev config set shadow.push after-commit` to push after every
shadow commit. See [zdev shadow](cli/shadow.md) for how branches are followed,
integrated and recovered.

Project and pull-request records are unchanged.

## No change IDs in the main history for personal records

With a personal or shadow record, `zdev commit` no longer adds a
`Zdev-Change-Id` trailer to the main commit, and its JSON `change_id` is
`null`. Under a shadow record, the shadow commit carries the change ID and names
the main commit in `Zdev-Source-Commit`; `zdev change inspect` and
`zdev change lookup` follow that link. Under a personal record, new main
commits have no ID to look up. Commits made before upgrading keep their
trailers.

Project and pull-request records, and repositories without `zdev init`, keep
the trailer.

## Goals pause on a blocker

A goal or loop that stops on a confirmed blocker or required decision now
pauses its goal instead of completing it, where the agent can pause it. Codex,
including Codex in T3 Code, pauses the goal; send `/goal resume` once the
blocker is resolved. Claude Code and Oh My Pi goals still end. Every blocker
report ends with `Resume goal:` and a ready-to-send command.

Goals set before upgrading keep the old wording and still complete on a
blocker. Set them again from `zdev goal` to get the pause.

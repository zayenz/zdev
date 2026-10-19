# zdev change

Read the stable change ID attached to a commit or find commits that carry a
known ID. This is useful after a rebase, when a Git hash can change while the
`Zdev-Change-Id` trailer remains the same.

## Examples

Inspect the latest commit, then the commit before it:

```sh
zdev change inspect HEAD
zdev change inspect HEAD~1 --format json
```

## Subcommands

```text
zdev change inspect REVISION
zdev change lookup CHANGE_ID
```

`inspect` accepts a Git revision such as `HEAD`, a branch name, or a commit
hash, and reports its ID and subject. `lookup` finds all reachable commits
with the supplied stable ID. Copy the full ID beginning with `Z` from an
inspection result.

Both commands read Git history. Use [commit](commit.md) to create a commit with
an ID, or [change-id](change-id.md) to generate an unattached ID.

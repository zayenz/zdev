# zdev change-id

Generate a new stable ID for a `Zdev-Change-Id` commit trailer. Use this when
preparing a commit outside the usual `zdev commit` command.

## Examples

```sh
zdev change-id
zdev change-id --format json
```

## Usage

```text
zdev change-id
```

The command prints an ID without changing Git state. It does not attach the ID
to a commit. [zdev commit](commit.md) generates and adds an ID automatically,
which is the usual way to commit completed work.

Use [zdev change](change.md) to inspect a commit's existing ID or find commits
that carry one.

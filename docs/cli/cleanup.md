# zdev cleanup

Remove development records that were tracked only for a pull request. The
`squash` subcommand deletes tracked `.zdev` files in a dedicated commit so they
are absent from the eventual squash merge.

## Examples

Create the cleanup commit:

```sh
zdev cleanup squash
```

Or request a structured result from the same operation:

```sh
zdev cleanup squash --format json
```

## Usage

```text
zdev cleanup squash
```

Cleanup requires the `pull-request` record policy, a checked-out non-trunk
branch with at least one commit, a clean index and worktree, and no Git
operation in progress. It creates a plain Git commit; it does not perform the
squash merge.

If the commit fails, the deletions remain staged so you can inspect and recover
them. See [record policies](init.md) and the [user guide](../user-guide.md).

# zdev init

Initialize zdev in a Git repository and choose how its development records are
shared. The command creates `.zdev/config.toml` and records the checked-out
branch as the project trunk.

## Examples

Keep the records local to this clone:

```sh
zdev init --record personal
```

For personal records, append `/.zdev/` to `.git/info/exclude` in this clone.
Initialization records the policy but does not add that ignore rule.

To version the records in their own Git repository at `.zdev/.git`, kept out
of this repository:

```sh
zdev init --record shadow
```

For a different repository where the team keeps its plans in Git:

```sh
zdev init --record project
```

## Record policies

```text
zdev init --record personal|project|pull-request|shadow [--shadow-remote URL]
```

| Policy | Where the records belong |
| --- | --- |
| `personal` | Local to this clone. |
| `project` | Tracked as lasting project records. |
| `pull-request` | Tracked for review, then removed before squash merge. |
| `shadow` | In a separate Git repository at `.zdev/.git` whose branches follow this repository's branches. |

A shadow record adds `/.zdev/` to `.git/info/exclude` automatically.
`--shadow-remote URL` attaches a remote, or clones an existing shadow record
from it. See [zdev shadow](shadow.md).

Choose the policy when initializing. On a detached HEAD, trunk remains unset;
use `zdev config trunk main` after choosing the intended branch.

Initialization creates the project configuration. Follow it with
[zdev area create](area.md) for an objective and [zdev skill](skill.md) if the
harness integration still needs installation. See the
[user guide](../user-guide.md) for the complete setup sequence and
[zdev cleanup](cleanup.md) for pull-request records.

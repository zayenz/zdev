# zdev shadow

Manage a shadow record: a `.zdev` record with its own Git history, kept out of
the repository it plans. Use it when you want zdev's records under version
control, and perhaps pushed to a remote, but not in the main repository.

## How a shadow record works

`.zdev` is its own Git repository at `.zdev/.git`. The main repository ignores
`.zdev` through its `.git/info/exclude`, so it treats a shadow record like a
personal one: nothing under `.zdev` is staged or committed there.

The shadow repository's checked-out branch follows the main repository's
branch. Before each command, zdev compares the two:

- If the main repository is on another branch, zdev switches the shadow
  repository to the branch with the same name. When that branch does not exist
  yet, zdev creates it from the current shadow branch. Uncommitted record
  changes carry over, as with `git switch`. If they would be overwritten, the
  command stops and asks you to commit them first. `status`, `check`, and
  `change` still run and show the problem.
- With a detached HEAD, the shadow repository stays where it is.
- A shadow branch whose main branch is already merged into the current branch
  is integrated when it fast-forwards cleanly. Otherwise `status` lists it as
  awaiting integration.

zdev reports each switch or integration on standard error, prefixed with
`zdev:`.

When zdev runs Git itself, it changes the shadow repository in the same
command:

- `zdev commit` commits the main index, then the shadow record. The shadow
  commit gets the same `Zdev-Change-Id` plus `Zdev-Source-Commit` and
  `Zdev-Source-Branch` trailers that name the main commit. When the shadow
  index has staged changes, only those are committed; otherwise every record
  change is. With nothing staged in the main repository, `zdev commit` commits
  only the shadow record. If the main commit succeeds but the shadow commit
  fails, the command reports the problem and exits with status 1.
- `zdev tasks import --commit` and `zdev tasks derive apply` commit the task
  files in the shadow repository. When derived work also commits files in the
  main repository, both commits share one change ID.
- `zdev area rebase` rebases the area's shadow branch onto its base's shadow
  branch. If the shadow record has uncommitted changes, or the shadow rebase
  conflicts, zdev leaves that branch unchanged and prints the command to finish
  it by hand.

Git does not track empty directories. When aligning, zdev recreates the empty
`tasks` directory of an area that has no tasks yet.

## Examples

Start a shadow record, optionally with a remote:

```sh
zdev init --record shadow
zdev init --record shadow --shadow-remote git@example.com:me/project-zdev.git
```

In a fresh clone of the main repository, the same command clones the existing
shadow record from the remote.

Convert an existing personal record:

```sh
zdev shadow adopt --shadow-remote git@example.com:me/project-zdev.git
```

Inspect, commit, push, and integrate:

```sh
zdev shadow status
zdev shadow commit -m "Record planning notes"
zdev shadow push
zdev shadow integrate scheduling
```

Push automatically after every shadow commit:

```sh
zdev config set shadow.push after-commit
```

## Usage

```text
zdev init --record shadow [--shadow-remote URL]
zdev shadow status
zdev shadow commit --message SUBJECT [--body PARAGRAPH]...
zdev shadow push
zdev shadow integrate BRANCH
zdev shadow adopt [--shadow-remote URL]
```

`init --record shadow` refuses when the main repository tracks `.zdev` files.
It creates `.zdev`, writes the configuration, creates the shadow repository on
the main repository's current branch, makes the first commit, and adds
`/.zdev/` to `.git/info/exclude` unless `.zdev` is already ignored. With
`--shadow-remote`, a remote that already holds a shadow record is cloned
instead. zdev starts on the main repository's branch if the remote has it,
otherwise on the remote's default branch. An empty remote is added as
`origin`. A relative local path is stored as an absolute path.

`status` shows the shadow branch and whether it matches the main branch,
uncommitted changes, ahead and behind counts against the upstream, the push
setting, branches awaiting integration, and orphaned branches. An orphaned
branch has records that are not merged and no main branch with its name. zdev
never deletes shadow branches. `zdev status` and `zdev check` include the same
report. `check` fails when the shadow repository is missing or the main
repository tracks `.zdev` files.

`commit` commits record changes without a main commit, under a new change ID.

`push` pushes the current shadow branch to its upstream. Without an upstream,
it pushes to `origin`, or to the only remote, and sets the upstream. It ignores
`shadow.push`.

`integrate BRANCH` brings another shadow branch's records into the current one.
It fast-forwards when it can. Otherwise it never rewrites the configured trunk:
on trunk it rebases `BRANCH` onto trunk first, and on any other branch it
rebases the current branch onto `BRANCH`. On conflict, the shadow rebase stays
in progress. Resolve it with `git -C .zdev rebase --continue`, or cancel with
`git -C .zdev rebase --abort`. Integration needs a clean shadow record.

`adopt` converts a personal record. It refuses when `.zdev/.git` exists, the
main repository tracks `.zdev` files, or the remote already has history.

The `shadow.push` preference is `never` by default. With `after-commit`, zdev
pushes the shadow branch after every shadow commit it makes and after
automatic integration. A failed push is a warning; the commit stands. See
[config](config.md) for scopes. A repository-local value lives in
`.zdev/workers.toml`, which the shadow repository versions.

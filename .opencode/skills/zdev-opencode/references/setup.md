# Set up durable zdev work

## When

Use this workflow only when the repository has no `.zd` directory and the user
wants to create durable zdev work. Standalone **Improve** and **Investigate** do
not require setup.

## Choose record ownership

Ask whether the zdev record should be **personal**, **project**, or
**pull-request**:

- Recommend **personal** for work limited to this user and clone. Add the exact
  entry `/.zd/` to this clone's `.git/info/exclude` before initialization.
- Recommend **project** for a portable, reviewable record that remains after
  merge. Leave `.zd` visible to Git.
- Recommend **pull-request** when `.zd` should be reviewed on the branch but
  omitted from the squash-merged result. Leave `.zd` visible to Git, commit it
  on the branch, and run `zd cleanup squash` before squash merge.

Wait for the user's choice, then apply its Git visibility rule. Check harness
integrations before initialization.

## Check harness integrations

Record ownership is separate from integration scope. Run
`zd skill check <harness> --scope user` for every requested harness, including
the active harness. Reuse integrations with status `ok`. Discuss installation
or scope only for `missing` or `conflict`, or when the user requests a
checked-in project integration.

For project scope, ask whether repository guidance comes from `auto`, `agents`,
`zdev`, or a repository-relative Markdown path. The user may skip installation.

## Initialize

Run `zd init --record <personal|project|pull-request>` with the selected policy.
Then run `zd skill install` only for integrations that need it.

## Finish

Confirm the selected record policy and installed integrations. Continue with
the interaction that required durable state.

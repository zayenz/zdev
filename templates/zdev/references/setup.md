# Set up durable zdev work

## When

Use this workflow only when the repository has no `.zdev` directory and the user
wants to create durable zdev work. Standalone **Improve** and **Investigate** do
not require setup.

## Choose record ownership

Ask whether the zdev record should be **personal**, **project**, or
**pull-request**:

- Recommend **personal** for work limited to this user and clone. Add the exact
  entry `/.zdev/` to this clone's `.git/info/exclude` before initialization.
- Recommend **project** for a portable, reviewable record that remains after
  merge. Leave `.zdev` visible to Git.
- Recommend **pull-request** when `.zdev` should be reviewed on the branch but
  omitted from the squash-merged result. Leave `.zdev` visible to Git, commit it
  on the branch, and run `zdev cleanup squash` before squash merge.

Wait for the user's choice, then apply its Git visibility rule. Check harness
integrations before initialization.

## Check harness integrations

Record ownership is separate from integration scope. Run
`zdev skill check <harness> --scope user` for every requested harness, including
the active harness. Reuse integrations with status `ok`. Discuss installation
or scope only for `missing` or `conflict`, or when the user requests a
checked-in project integration.

For project scope, ask whether repository guidance comes from `auto`, `agents`,
`zdev`, or a repository-relative Markdown path. The user may skip installation.

## Initialize

Run `zdev init --record <personal|project|pull-request>` with the selected policy.
Then run `zdev skill install` only for integrations that need it.

## Finish

Confirm the selected record policy and installed integrations. Continue with
the interaction that required durable state.

When the user wants a standing home for one-off work, offer the conventional
tag `general`. It uses the normal area command on an ordinary persistent branch:

```sh
zdev area create general --title "General work" --objective "Keep concrete one-off improvements as reviewed tasks."
```

The user creates or switches to that branch before the command, just as for any
other area. Zdev does not create or switch branches automatically. Keep the
area's brief minimal and standing: record shared boundaries, the testing level,
and validation that should apply across its work. Do not create `general`
unless the user asks for durable one-off work.

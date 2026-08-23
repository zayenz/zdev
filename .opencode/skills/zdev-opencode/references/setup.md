# Set up durable zdev work

## When

Use this workflow to initialize durable zdev work or to install and check
harness integrations. An existing `.zdev` directory skips initialization and
goes directly to integration setup. Standalone **Improve** and **Investigate**
can run without durable state.

## Choose record ownership

When `.zdev` is absent, use a record policy already stated by the user. When it
is still undecided, ask whether the record should be **personal**, **project**,
or **pull-request**:

- Recommend **personal** for work limited to this user and clone. Add the exact
  entry `/.zdev/` to this clone's `.git/info/exclude` before initialization.
- Recommend **project** for a portable, reviewable record that remains after
  merge. Leave `.zdev` visible to Git.
- Recommend **pull-request** when `.zdev` should be reviewed on the branch but
  omitted from the squash-merged result. Leave `.zdev` visible to Git, commit it
  on the branch, and run `zdev cleanup squash` before squash merge.

Apply the selected Git visibility rule, then check harness integrations before
initialization.

## Check harness integrations

Record ownership is separate from integration scope. Run
`zdev skill check <harness> --scope user` for every requested harness, including
the active harness. Reuse integrations with status `ok`. Discuss installation
or scope only for `missing` or `conflict`, or when the user requests a
checked-in project integration.

For project scope, reuse the stated or configured repository-guidance source.
When it is missing, ask whether guidance comes from `auto`, `agents`, `zdev`,
or a repository-relative Markdown path. The user may skip installation.

## Initialize

When `.zdev` is absent, run `zdev init --record
<personal|project|pull-request>` with the selected policy. Then run `zdev skill
install` for integrations that need installation or refresh.

## Finish

Confirm the selected record policy and installed integrations. Continue with
the interaction that required durable state.

When the user wants a standing home for one-off work, offer the conventional
tag `general`. Use the normal isolated form on its own persistent branch:

```sh
zdev area create general --title "General work" --objective "Keep concrete one-off improvements as reviewed tasks."
```

Or, for personal/project records when the user explicitly wants several areas
to share configured trunk, offer:

```sh
zdev area create general --title "General work" --objective "Keep concrete one-off improvements as reviewed tasks." --trunk
```

Zdev does not create or switch branches automatically. Pull-request records
use isolated areas. Keep the area's brief minimal and standing: record shared
boundaries, the testing level, and validation that should apply across its
work. Do not create `general` unless the user asks for durable one-off work.

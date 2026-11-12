# zdev commit

Commit the changes already staged in Git. With a project or pull-request
record, or before `zdev init`, zdev adds a stable `Zdev-Change-Id` trailer. The
ID lets you find the same logical change after a rebase changes its Git commit
hash.

With a personal or shadow record, the commit gets no trailer, so zdev leaves
no trace in the repository's history. A shadow record's own commit carries the
change ID instead and names the main commit in a `Zdev-Source-Commit` trailer;
see [shadow](shadow.md).

## Examples

After staging the intended files, choose either a short message:

```sh
zdev commit -m 'Add the greeting command'
```

Or a message with a separate explanation:

```sh
zdev commit -m 'Add the greeting command' \
  --body 'Print a greeting for the supplied name.'
```

## Usage

```text
zdev commit --message SUBJECT [--body PARAGRAPH]...
```

`-m` is the short form of `--message`. Repeat `--body` for separate paragraphs.
The command runs `git commit` against the existing index, so stage the intended
source and record files first.

Completing a task is a separate operation: [task done](task.md) records its
outcome and validation before you stage and commit the changes. Use
[change inspect](change.md) to read the resulting ID. See
[completion and commit](../workflow.md#completion-and-commit) for the workflow.

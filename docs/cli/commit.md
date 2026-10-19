# zdev commit

Commit the changes already staged in Git and add a stable `Zdev-Change-Id`
trailer. The ID lets you find the same logical change after a rebase changes
its Git commit hash.

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

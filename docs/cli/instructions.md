# zdev instructions

Print a short explanation of how a coding agent should use zdev. This is useful
when checking the workflow guidance or supplying it to a harness that does not
yet have the integration installed.

## Examples

```sh
zdev instructions
zdev instructions --format json
```

## Usage

```text
zdev instructions
```

There are no command-specific options. The output describes the working
conventions; reading it does not initialize a repository or install files.
Use [zdev skill](skill.md) to install the full integration and
[Agent commands](../commands/index.md) to choose an agent request.

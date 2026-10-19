# A feature from idea to commit

Suppose a query tool prints a table in the terminal, and you want to use its
results in other programs. "Add JSON export" leaves several choices open.
What should the JSON contain? Should it go to standard output or a file?
What happens when a destination already exists?

This walkthrough follows that idea through discussion, tasks, implementation,
independent verification, and commits. The query tool is an illustrative
project; adapt the prompts and validation to your repository. It assumes zdev
and your harness integration are installed and the repository is initialized.
See [Your first task](getting-started.md) for that setup.

You can run the terminal commands below yourself or ask your agent to run them.
The response examples are illustrative summaries, not fixed output formats.

## Give the idea an area

Start on the project's normal base branch with a safe checkout. Create a
feature branch and an area for this objective:

```sh
git switch -c result-export
zdev area create result-export \
  --title "Export query results" \
  --objective "Let other programs consume query results as JSON."
```

In your coding-agent chat, ask:

```text
Use zdev to explore result-export. Our query tool prints a table today.
We want JSON output for use by other programs, with an option to save it
to a file. Inspect the current output and argument handling, then help
define the behavior and validation. Preserve the default text output.
Stop after the brief; do not create tasks or implement yet.
```

The agent reads the code and writes `.zdev/result-export/brief.md`. The brief
should describe the intended result and identify choices that still need you.
It should use existing argument handling and serialization facilities where
suitable. The request does not justify replacing them.

A first response might be:

```text
Brief saved to .zdev/result-export/brief.md. Default text output is preserved.
Still to decide: JSON shape and whether file output may overwrite a file.
No tasks created yet.
```

## Discuss the behavior before the task split

Ask the agent to challenge the draft:

```text
Use zdev to discuss the result-export brief. Work through the JSON shape,
empty results, query failures, file output, and existing destinations.
Recommend behavior that fits the current CLI, and settle the validation
with me before creating tasks.
```

For this example, suppose the discussion settles these choices:

- `--format json` writes an array of row objects to standard output. An empty
  result is `[]`; missing values use JSON `null`.
- Text output remains the default. Query failures write diagnostics to
  standard error and return a nonzero exit status.
- `--output PATH` saves JSON to a new file and leaves standard output empty.
  An existing destination is an error. File output is only supported with
  `--format json` in this change.
- Validation uses representative result fixtures, the existing CLI checks,
  and temporary files for the destination cases.

These decisions belong in the brief. Resolve project-specific issues too,
such as how duplicate column names map to object keys, before promising a
JSON representation. Internal helper names can wait for implementation.

Expect concrete questions, such as: "Should an existing output file be an
error? I recommend that for this change so export cannot overwrite data
silently." Once settled, the agent reports the decisions saved in the brief.

## Review and import the tasks

Once the brief is clear, ask:

```text
Use zdev to create tasks for result-export from the agreed brief.
Each task should leave a useful, checked result. Show the proposed bundle
before importing it.
```

A reasonable split here is:

| Task | Result | Depends on |
| --- | --- | --- |
| JSON output | Add `--format json`, help and usage documentation, and the agreed result and error checks. Preserve default text output. | Nothing |
| File output | Add `--output PATH` for JSON, document it, and check new files, existing destinations, write failures, and standard output. | JSON output |

The first task is useful on its own because another program can read standard
output. The second adds the file option without changing the JSON shape. For
a smaller feature, one task covering behavior, documentation, and validation
may be enough. Split work because the results can be checked separately.

Read the actual bundle, including boundaries, done conditions, dependencies,
and validation. If you ask for changes, review the revised bundle. When the
displayed version matches the brief, reply:

```text
Import this exact task bundle for result-export. Stop after import.
```

Zdev allocates task IDs and writes the task files and generated index. Import
records the approved work; it does not start implementation. With project or
pull-request records, normal import also commits the planning records.
Personal records stay local to the clone.

An import report might say: "Imported result-export-001 (JSON output) and
result-export-002 (file output, depends on 001). Task 001 is ready;
implementation has not started." Use the actual IDs from your run.

## Implement one task and verify it independently

Start with one task so you can inspect the result:

```text
Use zdev to implement the next task in result-export.
Stop after its verified commit.
```

The coordinator checks the branch and existing changes, selects the ready task,
and delegates implementation. Use the checkout or worktree it reports. The
implementer reads the brief and task, makes the changes, and runs the agreed
validation. For the first task, that includes JSON results, empty results,
errors, and the preserved default output.

A fresh verifier checks the result against the task and brief, runs required
validation, and reviews repository conventions and affected interfaces. The
implementer's report alone is not acceptance. A changed default output can
require correction even if the JSON example works.

Required corrections get one correction pass and a focused recheck. If required
problems remain, the task stays open and the work is preserved for a continuation
decision. A changed requirement that affects scope should be settled before
continuing. Testing follows the agreed level; it does not automatically require
a new test framework.

You do not need a separate verify request in this flow: independent verification
is included in implement. The [Verify command](commands/verify.md) checks an
existing implementation explicitly and reports a result without committing it.

## Inspect the commit, then continue

After verification passes, the coordinator records completion and commits the
accepted changes with a `Zdev-Change-Id` trailer. Expect a report naming the
task, validation, verification result, checkout, and commit.

For example:

```text
Completed result-export-001: JSON output, help, and usage documentation.
Validation: agreed CLI checks passed; default text output preserved.
Independent verification: passed requirements and coding-guideline review.
Commit: <commit-id>, in the reported result-export checkout.
Next: result-export-002 is ready. Stopped after one task.
```

In that checkout, inspect the record and commit:

```sh
zdev status result-export
zdev tasks list result-export
zdev next result-export
git log -1 --format=full
git show --stat HEAD
```

The JSON task should be done and the dependent file-output task ready. Start
the next task with the same request:

```text
Use zdev to implement the next task in result-export.
Stop after its verified commit.
```

That task receives its own validation, independent verification, and commit.
If you want the whole approved queue completed in one run,
[goal or loop](commands/loop.md) can continue through it instead.

Once both tasks are done, inspect the behavior against the brief. An empty
queue leaves the area open. When the objective is met, close it:

```sh
zdev area close result-export
```

Closing the area does not merge the feature branch or publish a release.
Follow the repository's integration process for those steps. The accepted
source commits remain in Git; the planning record stays local or travels with
them according to its record policy.

For work where the result is still a hypothesis to measure, continue with
[Research toward an open-ended goal](research-walkthrough.md).

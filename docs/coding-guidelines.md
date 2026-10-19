# Coding guidelines

Zdev gives planning, implementation, and review agents shared expectations for
the code they work on. The guidelines cover design, naming, data, control flow,
and completion. They help agents make compatible decisions across tasks and
conversations, while leaving room for the project's own conventions.

They apply during ordinary feature work and research changes alike. You do not
need to repeat them in every request; the installed integration supplies the
relevant guidance to its workers.

## What the guidelines ask for

- Understand the affected behavior before choosing a design. Read its callers,
  contracts, relevant tests, and project instructions. Choose the smallest
  coherent change that meets the agreed outcome.
- Use names that explain intent and effects at the call site. A function that
  writes a report should make that effect clear; a name that suggests a simple
  lookup can mislead its callers.
- Make data and contracts clear. Preserve meaningful distinctions between
  absence, failure, and valid results. Make ownership and mutation apparent,
  and validate external data at its entry point.
- Structure code so the main operation is understandable before its details.
  Keep related logic together and separate responsibilities where that helps
  readers. A cohesive long function can be clearer than many tiny helpers;
  extra layers need a concrete benefit.
- Use the project's formatter, error handling, configuration, and established
  conventions. Improve names and structure before adding comments to explain
  confusing code. Comments should preserve constraints and reasons.
- Match validation to the agreed testing level and the change's risk. Existing
  checks or a focused reproduction may be enough. A new test should provide
  useful confidence rather than repeat the implementation.

These are criteria for judgment, not size limits or a requirement to introduce
a particular architecture. Unrelated cleanup remains separate from the task.

## The same expectations through the task

During planning, the agent identifies where behavior belongs and which
contracts will change. During implementation, it reassesses that plan against
the code and checks the finished result before handing it over.

The reviewer then checks both the requested behavior and the coding guidelines.
Passing tests alone does not establish that a change is ready. A misleading
name or tangled responsibility can require correction when the reviewer can
explain the misunderstanding or reasoning burden it creates.

For example, a useful finding might say:

```text
export.ts:42 — getResults also writes the output file. Its caller looks
like a read-only query, which hides a filesystem effect. Rename the
operation to make the write explicit, or separate it if callers need
the results without writing them.
```

This is an illustrative finding, not a rule that every operation must be split.
Required corrections need a checked location, consequence, and proportionate
remedy. Optional preferences stay separate and do not block acceptance. See
[Review and verification](review-and-verification.md) for how this fits the
task workflow.

## Language, framework, and project guidance

The common guidelines have supplements for Rust, TypeScript, MiniZinc, Java,
Kotlin, Python, and C++, plus web behavior, React, and React Router. Agents load
the ones relevant to the affected code and responsibilities. A React dependency
does not make every change a React task.

Project instructions, the area brief, and task requirements supply the local
context. Explicit user requirements and applicable project constraints govern
conflicts; material exceptions should be explained. The guidelines preserve
the agreed outcome and testing level rather than expanding the work during
review.

The [shared coding guidance](../skills/zdev/references/coding.md) contains the
full rules and links to its supplements. The installed copy travels with your
harness integration; refreshing that integration refreshes its guidance too.

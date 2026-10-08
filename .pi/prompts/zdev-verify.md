---
description: Independently verify the explicit current ready zdev task
---

# Explicit verification contract

`zdev-verify <area> <task-id>` is a read-only review of the explicit current
ready task. The coordinating session owns preflight and envelope validation;
one fresh configured verifier owns the initial independent review. Substantive
findings then receive the bounded independent check in the task workflow.

If the explicit task has `hypothesis`, return `BLOCKER zdev-verify` before
starting a verifier: automatic experiment execution is not yet supported.
The installed task-format reference defines research evidence, including the
independent review attestation required for an accepted result.

## Preflight and handoff

Parse exactly one area and task ID. Run
`zdev work-context <area> --store --format json`, validate the compact result,
and use `--show <snapshot>` in coordination when complete context is needed.
Start the verifier when the context is open and ready, `task_work.safe` is true, and
the nested status and goal projections agree on the requested area and task.
A true `stale_advisory` is useful context: report its exact advisory once and
continue verification.

Return `BLOCKER zdev-verify <area> <task-id>` without starting a worker when
the context is invalid or unsafe, the goal is closed, empty, or exhausted, or
another task is ready. Include the failed stage, reason, and preserved state.

The stored result is the dispatch snapshot for the admitted open, ready, safe
area, task, HEAD, and checkout.
Give one fresh verifier that opaque locator, the brief, complete task,
repository guidance, supplied coding guidance, relevant source and tests, and
the recorded baseline. The verifier reads the **Code standards** and **Review** sections
and validates without intentional edits. Findings may need a different verifier
when coordination cannot independently inspect the evidence. The route leaves
task lifecycle, Git state, recovery, and derived work unchanged.

## Verifier context

Coordination accepts only the exact compact preflight result and reads its
immutable context with
`zdev work-context <area> --show <snapshot> --format json`. It requires the
same open, ready, safe area, task, HEAD, and checkout as preflight before
supplying the locator to the verifier. The verifier shows that supplied
snapshot before inspection or validation.

After the verifier checks the whole task and runs its required validation,
coordination runs `zdev work-context <area> --compare <snapshot> --format json`.
A pass requires the exact four-key compact result for the selected area and
snapshot with `equal: true`. A false comparison preserves rework only when all
findings are concrete task-owned validation writes reported as exact
`validation_write: <normalized repository-relative path>` findings. An
ordinary defect rework plus unequal state, a mixed valid and malformed marker
set, or any unclear ownership is a blocker. An unavailable or invalid snapshot
is a blocker. The verifier leaves validation writes in place for coordination
to assess.

## Result envelope

The verifier returns one JSON object:

```json
{
  "verdict": "pass",
  "summary": "<non-empty summary>",
  "findings": [],
  "escalation": "none"
}
```

These four unique keys are required. `summary` is a non-empty string
and `findings` is an array of non-empty strings. Accept only valid JSON with
the required value types. A brief sentence or Markdown fence around one
unambiguous balanced object is tolerated. Multiple objects, malformed JSON, or
duplicate required keys are invalid. Legacy nine-key verifier objects are
invalid worker output.

Use `pass` when the implementation satisfies the whole task and all required
checks. Its findings are empty. Put checked locations and validation
conclusions in the summary.

Use `rework` for concrete implementation defects or attributable task-owned
validation writes, and put the required corrections in findings. Its
escalation is `none`, or `advanced-implementer` when the standard/default
implementation genuinely needs the advanced profile.

Use `blocker` when required evidence is unsafe or unavailable, ownership is
ambiguous, or a user-owned decision is needed. Its escalation is `none`.

The coordinating session strictly validates the semantic result, validates
the post-response comparison, and independently vets substantive findings under
the task workflow before reporting. Require equal comparisons before and after
vetting; changed code needs fresh review. Use the existing semantic result and
summary for checked findings and reasons for corrections. Unresolved or
unchecked claims are blockers; zero findings and mechanical validation-write
markers need no extra pass. Coordination constructs the compatible public nine-key
envelope. It generates schema version, kind, area, task ID, snapshot evidence,
and optional stale advisory; the worker cannot supply or override them. Invalid
or unavailable independent verification returns
`BLOCKER zdev-verify <area> <task-id>` with no mutation.

The exact installed task-workflows contract path is
"/Users/zayenz/.codex-workspaces/worktrees/83c5f3b8-0c89-4e75-b8f6-7831269edd01/zdev/.pi/skills/zdev-pi/references/task-workflows.md". Decode it and include the resulting
path in the `zdev_subagent` verifier payload.

Parse `$ARGUMENTS` as `<area> <task-id>`. The current Pi session performs
preflight and exact ID matching before one fresh `zdev_subagent` call with role
`verifier`. Store and validate the snapshot immediately before dispatch,
extract and validate one balanced four-field object with wrapper tolerance,
compare the snapshot afterward, and
return the coordinator-constructed strict envelope without lifecycle or Git
mutation.

<!-- zdev:generated-repository-guidance:start -->
## Repository guidance discovery

Before inspecting, planning, changing, or validating code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.
<!-- zdev:generated-repository-guidance:end -->

## Coding guidance

Coding guidance file: "/Users/zayenz/projects/zdev/.pi/skills/zdev-pi/references/coding.md"

Before planning code, read Design and Code standards from this file. Before implementing or repairing code, also read Completion. Before verification, audit, or code review, read Code standards and Review. Pass this exact location to each relevant worker; read it directly even when the root skill is not loaded. Routine coordination and non-code tasks do not need it.

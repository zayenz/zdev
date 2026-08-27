# Explicit verification contract

`zdev-verify <area> <task-id>` is a read-only review of the explicit current
ready task. The coordinating session owns preflight and envelope validation;
one fresh configured verifier owns the independent review.

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
repository guidance, relevant source and tests, and the recorded baseline. The
verifier reads and validates without intentional edits. This route uses only
this verifier. It leaves task lifecycle, Git state, recovery, and derived work
unchanged.

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
snapshot with `equal: true`. A false comparison preserves rework only for
concrete task-owned validation writes reported as exact
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
the post-response comparison, and constructs the compatible public nine-key
envelope. It generates schema version, kind, area, task ID, snapshot evidence,
and optional stale advisory; the worker cannot supply or override them. Invalid
or unavailable independent verification returns
`BLOCKER zdev-verify <area> <task-id>` with no mutation.

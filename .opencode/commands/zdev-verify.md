---
description: Independently verify the explicit current ready zdev task
---

# Explicit verification contract

`zdev-verify <area> <task-id>` is a read-only review of the explicit current
ready task. The coordinating session owns preflight and envelope validation;
one fresh configured verifier owns the independent review.

## Preflight and handoff

Parse exactly one area and task ID. Run
`zdev work-context <area> --format json` and retain the complete result. Start
the verifier when the context is open and ready, `task_work.safe` is true, and
the nested status and goal projections agree on the requested area and task.
A true `stale_advisory` is useful context: report its exact advisory once and
continue verification.

Return `BLOCKER zdev-verify <area> <task-id>` without starting a worker when
the context is invalid or unsafe, the goal is closed, empty, or exhausted, or
another task is ready. Include the failed stage, reason, and preserved state.

Give one fresh verifier the retained context, brief, complete task, repository
guidance, relevant source and tests, and the recorded baseline. The verifier
reads and validates without intentional edits. This route uses only this
verifier. It leaves task lifecycle, Git state, recovery, and derived work
unchanged.

## Verifier context

Before inspection or validation, the verifier runs
`zdev work-context <area> --store --format json`. It accepts the compact
locator only when it names the same open, ready, safe task and HEAD, then reads
the immutable context with
`zdev work-context <area> --show <snapshot> --format json`.

After checking the whole task and running its required validation, the
verifier runs
`zdev work-context <area> --compare <snapshot> --format json`. A pass requires
the exact four-key compact result for the selected area and snapshot with
`equal: true`. A false comparison is rework for attributable task-owned writes
and a blocker when ownership is unclear. An unavailable or invalid snapshot is
a blocker. The verifier leaves validation writes in place for the coordinator
to assess.

## Result envelope

The verifier returns one JSON object and no surrounding text:

```json
{
  "schema_version": 1,
  "kind": "verifier",
  "area": "<area>",
  "task_id": "<task-id>",
  "verdict": "pass",
  "summary": "<non-empty summary>",
  "evidence": [],
  "findings": [],
  "escalation": "none"
}
```

The object has exactly these nine keys. `schema_version` is `1`; `kind` is
`verifier`; `area` and `task_id` match the request; `summary` is a non-empty
string; and `evidence` and `findings` are arrays of non-empty strings. Accept
only valid JSON with unique known keys and the exact value types.

Use `pass` when the implementation satisfies the whole task and all required
checks. Its findings are empty and its evidence contains exactly one
`work_context_snapshot: W<16-lowercase-hex>` item, plus the exact stale
advisory when one applies. Put checked locations and validation conclusions in
the summary.

Use `rework` for concrete implementation defects or attributable task-owned
validation writes, and put the required corrections in findings. Its
escalation is `none`, or `advanced-implementer` when the standard/default
implementation genuinely needs the advanced profile.

Use `blocker` when required evidence is unsafe or unavailable, ownership is
ambiguous, or a user-owned decision is needed. Its escalation is `none`.

The coordinating session validates the complete envelope. On a valid worker
result, return that object unchanged as the public read-only result. Invalid or
unavailable independent verification returns
`BLOCKER zdev-verify <area> <task-id>` with no mutation.

Parse `$ARGUMENTS` as `<area> <task-id>`. The primary agent performs preflight
and exact ID matching before invoking one new `zdev-verifier` subagent. Return
its strict public envelope without lifecycle or Git mutation.

<!-- zdev:generated-repository-guidance:start -->
## Repository guidance discovery

Before inspecting, planning, changing, or validating code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.
<!-- zdev:generated-repository-guidance:end -->

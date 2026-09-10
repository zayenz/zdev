# Task complexity and worker escalation

A task's declared complexity selects its implementation route. Routine work
uses a constrained implementer, standard work uses the normal implementer, and
advanced work starts with a read-only plan. A verifier can also recommend an
advanced implementer for a repair within the approved scope.

The harness evidence and editable defaults were checked on 2026-08-20 and are
recorded in [Worker profiles](worker-profiles.md). That document describes the
current roles and runtime; this record describes the current routing policy
they follow.

## Declaring complexity

A task has one of three complexity values:

- `routine`: tightly specified, low-risk mechanical work;
- `standard`: bounded work with a reviewed outcome, scope, and validation; or
- `advanced`: work whose approved implementation needs additional planning or
  reasoning.

The task bundle accepts an optional `complexity` field. An omitted value means
`standard`, so every existing version 1 bundle and task file remains valid.
Task-authoring guidance emits the field when the user selects a level. Imported
task frontmatter stores the explicit value. Existing task files that omit it
remain unchanged when completed or reopened.

Complexity is durable because it changes the approved execution contract. It
is not inferred from tokens, files, estimated cost, model confidence, or the
result of a worker run. Changing it after approval is an ordinary task-content
change and requires the existing review discipline.

Bundle review displays complexity and includes an authored value in the review
fingerprint. To preserve old review fingerprints, deserializing an omitted field
must not add `standard` to the canonical bundle used for fingerprinting.
`tasks list`, `tasks show`, `next`, selected-area `status`, and `goal` expose the
effective value. The coordinator reads it from the goal nested in a fresh
work-context result. `TASKS.md` needs no new column.

## Worker roles

Implementation and verification use four worker roles:
`routine-implementer`, `implementer`, `verifier`, and
`advanced-implementer`. Routine and advanced are explicit implementation
tiers; `implementer` and `verifier` are the standard defaults. Profile
precedence, validation, install refresh, and the current built-ins remain as
documented in [Worker profiles](worker-profiles.md).

The current built-ins use Luna low for routine work, Sol low for standard Codex
and OpenAI-backed implementation, Opus 5 low for Claude standard work, and Sol
high for advanced implementation. Verification uses the current Sol or Opus 5
profile for its harness. Projects may override each whole profile through the
existing config contract.

Independent verification always uses a fresh `verifier`. Advanced planning
uses the `planner` role, which falls back to the selected profile's
`advanced-implementer` when no planner row is configured. The coordinator has
no worker profile.

## Planning advanced work

Before the first code edit for an `advanced` task, the coordinator starts a
fresh read-only planner using the resolved planner profile. The planner
receives the same work-context, brief, task, and repository guidance as an
implementer. It returns exactly four semantic fields: `verdict`, `summary`,
`plan`, and `findings`. A `plan` verdict carries an exact three-field plan
object with `approach`, `paths`, and `validation`, with optional supporting
findings; `blocker` carries a null plan and concrete findings. The coordinator
strictly parses that result and constructs the compatible public planner
envelope with identity, evidence, and `escalation: "none"`. The semantic plan
itself is the conversation handoff and is passed unchanged to a fresh advanced
implementer. It is not a repository file or zdev record and cannot add scope,
relax validation, or amend the approved task.

Planning is skipped for `routine` and `standard` tasks, explicit `zdev-verify`,
and a resumed workflow that already holds a valid plan for the same task and
unchanged baseline. It is also skipped when implementation or rework exists;
planning after attributed task edits would not protect the first implementation
choice. Unexplained or ambiguously owned edits still block under the existing
baseline rules.

## Escalating a repair

The current strict verifier object always contains `escalation`. Its value is
`none`, except that verifier `rework` may request `advanced-implementer`. The
coordinator routes that request to `advanced-implementer`. An unknown value,
duplicate key, or advanced escalation with `pass` or `blocker` is invalid and
therefore blocking under the current fail-closed rule.

The verifier recommends escalation only when its concrete findings show that
the repair needs broader reasoning within the already approved scope. It does
not recommend escalation for an unavailable model, transport failure, missing
evidence, unsafe scope, or a product decision; those are blockers. The
coordinator may move a standard implementation to `advanced-implementer` once
per task run. A routine or already advanced route cannot escalate. There is no
higher tier or automatic change to the task's recorded complexity. Rework has
no fixed retry count. An advanced implementation that receives verifier
verdict `rework` returns to an advanced implementer and then to a new
verifier.

## Coordinator routing

The coordinator retains branch checks, baseline ownership, task selection,
user questions, envelope validation, lifecycle changes, and commits.

1. Run `zdev work-context <area> --format json` and read the effective task
   complexity from its nested goal.
2. For `routine`, start `routine-implementer`. For `standard`, start
   `implementer`. For `advanced`, obtain the valid plan above, then start a
   fresh `advanced-implementer` with that plan.
3. Inspect the checkout and start a fresh resolved `verifier`.
4. On ordinary verifier verdict `rework`, return findings to the same profile,
   resuming the worker only where the harness safely supports it.
5. On verifier verdict `rework` with envelope `escalation` set to
   `advanced-implementer`, start a replacement worker using that current
   profile, with the goal, baseline, current diff, and all findings.
6. After every repair, start another fresh verifier and check the whole task.
7. Stop for verdict `blocker`, an unsafe or changed task/baseline, scope outside
   the approved task, or any choice that belongs to the user. Only verifier
   verdict `pass` permits task completion and commit.

Independent verification remains mandatory in every route. A verifier may
recommend the advanced implementer, but never verifies its own work or turns
its recommendation into acceptance.

## Cases across five harnesses

The case policy is common:

| Case | Route |
| --- | --- |
| Routine success | routine implementer → fresh verifier verdict `pass` → complete and commit |
| Standard success | standard implementer → fresh verifier verdict `pass` → complete and commit |
| Advanced success | read-only advanced planner → fresh advanced implementer → fresh verifier verdict `pass` |
| Ordinary repair | selected implementer → verifier verdict `rework`, escalation `none` → same-profile repair → fresh verifier |
| Escalated repair | standard implementer → verifier verdict `rework`, escalation `advanced-implementer` → replacement advanced implementer → fresh verifier |
| Product decision | planner blocks or a worker returns verdict `blocker` → coordinator asks the user; no completion or commit |

Each harness uses its native workers for these routes:

| Harness | Planner | Implementation | Verification and rework |
| --- | --- | --- | --- |
| Codex | fresh read-only subagent using the resolved planner model/effort | fresh subagent using the routine, standard, or advanced profile | fresh verifier each time; follow up only for same-profile repair; escalation spawns a replacement |
| Claude Code | read-only `zdev-planner` agent using the resolved planner profile | `zdev-routine-implementer`, `zdev-implementer`, or `zdev-advanced-implementer` | `zdev-verifier`; workflow resumes only same-profile repair and starts an advanced replacement on escalation |
| OpenCode | read-only `zdev-planner` subagent using the resolved planner profile | `zdev-routine-implementer`, `zdev-implementer`, or `zdev-advanced-implementer` | new verifier task each time; `task_id` resume only for same-profile repair |
| Pi | read-only `planner` role using the resolved planner profile | `routine-implementer`, `implementer`, or `advanced-implementer` role | `verifier`; every repair is a fresh process using the selected profile |
| Oh My Pi | blocking read-only `zdev-planner` task agent using the resolved planner profile | `zdev-routine-implementer`, `zdev-implementer`, or `zdev-advanced-implementer` | fresh `zdev-verifier`; `hub` only for same-profile repair, replacement task for escalation |

The product-decision case stops in the coordinating session in all five
harnesses. Native transport, resumption, background jobs, teams, and fan-out do
not change the routing contract.

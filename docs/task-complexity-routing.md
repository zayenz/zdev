# Task complexity and worker escalation

> **Status: current behavior.** Zdev persists and projects the three task
> complexity values and realizes the routing policy below in every installed
> harness integration.

This record defines a small routing policy for zdev task work. It keeps task
complexity explicit, keeps routine work bounded, and reserves the advanced
implementer for planned advanced work or a verifier-requested repair. It adds no
evaluation system, provider catalog, or automatic complexity classifier.

The harness evidence and editable defaults were checked on 2026-08-20 and are
recorded in [Worker profiles](worker-profiles.md). That document describes the
current roles and runtime; this record describes the current routing policy
they realize.

## Decisions

### Task complexity is authored metadata

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
effective value; goal projection is the coordinator's routing input. `TASKS.md`
needs no new column.

### Routing builds on the current worker roles

Zdev already resolves and renders four whole worker profiles for every harness:
`routine-implementer`, `implementer`, `verifier`, and
`advanced-implementer`. Routine and advanced are explicit implementation
tiers; `implementer` and `verifier` are the standard defaults. Profile
precedence, validation, install refresh, and the current built-ins remain as
documented in [Worker profiles](worker-profiles.md); this design adds no profile
key or model default.

The current built-ins use Luna low for routine work, Sol low for standard Codex
and OpenAI-backed implementation, Opus 5 low for Claude standard work, and Sol
high for advanced implementation. Verification uses the current Sol or Opus 5
profile for its harness. Projects may override each whole profile through the
existing config contract.

Independent verification always uses a fresh `verifier`. There is no planner,
coordinator, or advanced-verifier profile. The planner below is a
read-only dispatch of the resolved `advanced-implementer` profile; it does not
add a durable role or configuration key.

### Advanced work gets one explicit plan

Before the first code edit for an `advanced` task, the coordinator starts a
fresh read-only planner using `advanced-implementer`. The planner receives the
same goal, brief, task, repository guidance, and three-part Git baseline as an
implementer. It returns the strict nine-key worker object with
`kind: "planner"`, `verdict: "plan"`, and `escalation: "none"`. A plan has
exactly one non-empty `Approach: `, `Paths: `, and `Validation: ` evidence
entry and no findings. `verdict: "blocker"` is the only alternative. The plan is a
conversation handoff, not a repository file or zdev record. It cannot add
scope, relax validation, or amend the approved task. The coordinator checks
the subject, baseline, paths, and absence of unresolved decisions, then passes
the plan unchanged to a fresh advanced implementer.

Planning is skipped for `routine` and `standard` tasks, explicit `zdev-verify`,
and a resumed workflow that already holds a valid plan for the same task and
unchanged baseline. It is also skipped when implementation or rework exists;
planning after attributed task edits would not protect the first implementation
choice. Unexplained or ambiguously owned edits still block under the existing
baseline rules.

### Escalation is a recommendation, not a verdict

The current strict verifier object always contains `escalation`. Its value is
`none`, except that verifier `rework` may request `advanced-implementer`. This
design routes that request to the current `advanced-implementer` role. An
unknown value, duplicate key, or advanced escalation with `pass` or `blocker`
is invalid and therefore blocking under the current fail-closed rule.

The verifier recommends escalation only when its concrete findings show that
the repair needs broader reasoning within the already approved scope. It does
not recommend escalation for an unavailable model, transport failure, missing
evidence, unsafe scope, or a product decision; those are blockers. The
coordinator may move a standard implementation to `advanced-implementer` once
per task run. A routine or already advanced route cannot escalate. There is no
higher tier, downgrade,
retry count, model search, or automatic change to durable complexity. An
advanced implementation that receives verifier verdict `rework` returns to an
advanced implementer and then to a new verifier.

## Coordinator routing

The coordinator retains branch checks, baseline ownership, task selection,
user questions, envelope validation, lifecycle changes, and commits.

1. Read the effective task complexity from `zdev goal`.
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

Each harness realizes every row through these native seams:

| Harness | Planner | Implementation | Verification and rework |
| --- | --- | --- | --- |
| Codex | fresh read-only subagent using the resolved advanced model/effort | fresh subagent using the routine, standard, or advanced profile | fresh verifier each time; follow up only for same-profile repair; escalation spawns a replacement |
| Claude Code | read-only `zdev-planner` agent rendered from the advanced profile | `zdev-routine-implementer`, `zdev-implementer`, or `zdev-advanced-implementer` | `zdev-verifier`; workflow resumes only same-profile repair and starts an advanced replacement on escalation |
| OpenCode | read-only `zdev-planner` subagent rendered from the advanced profile | `zdev-routine-implementer`, `zdev-implementer`, or `zdev-advanced-implementer` | new verifier task each time; `task_id` resume only for same-profile repair |
| Pi | read-only `planner` role using the advanced profile | `routine-implementer`, `implementer`, or `advanced-implementer` role | `verifier`; every repair is a fresh process using the selected profile |
| Oh My Pi | blocking read-only `zdev-planner` task agent rendered from the advanced profile | `zdev-routine-implementer`, `zdev-implementer`, or `zdev-advanced-implementer` | fresh `zdev-verifier`; `hub` only for same-profile repair, replacement task for escalation |

The product-decision case stops in the coordinating session in all five
harnesses. Native transport, resumption, background jobs, teams, and fan-out do
not change the routing contract.

## Smallest implementation seam

- `src/tasks.rs` and `src/goal.rs` currently own the strict three-value task
  schema, omitted-field compatibility, review fingerprints, and effective task
  projections. The parsed field stays optional with a `standard` accessor, so
  old files remain byte-stable through completion and reopen.
- `src/integrations.rs` and canonical templates: render any read-only planner
  artifact from the already resolved advanced profile, then add the common
  routing rules. Install and check continue to share one MiniJinja render path
  and publish only after all artifacts validate.
- Harness adapters: select the current routine, standard, verifier, and
  advanced roles. Planner dispatches reuse the advanced profile with read-only
  tools; they do not add another configuration key.
- Tests: cover one legacy omitted-complexity task, one advanced goal, one
  standard route, one planned route, one ordinary rework, one escalation, and
  one invalid escalation envelope. Reuse the existing deterministic template
  and harness-contract tests; do not build a provider matrix or harness
  simulator.

The implementation adds no evaluation, benchmarking, telemetry, model
discovery, provider catalog, derived-task authority, or optional verification.

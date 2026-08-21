# Task complexity and worker escalation

> **Status: design only.** Zdev does not implement this routing contract.

This record defines a small routing policy for zdev task work. It keeps task
complexity explicit, makes ordinary work cheaper, and reserves a stronger
implementer for planned complex work or a verifier-requested repair. It adds no
evaluation system, provider catalog, or automatic complexity classifier.

The design was checked against current harness documentation on 2026-08-20.
It supersedes the suggested mappings and two-profile vocabulary in
[Worker profiles](worker-profiles.md) for the follow-up implementation. That
document still describes the current runtime until those tasks land.

## Decisions

### Task complexity is authored metadata

A task has one of two complexity values:

- `standard`: a bounded task with a reviewed outcome, scope, and validation;
- `complex`: a task whose approved implementation needs a separate design pass
  before code changes.

The task bundle accepts an optional `complexity` field. An omitted value means
`standard`, so every existing version 1 bundle and task file remains valid.
Task-authoring guidance should emit the field for new tasks. Imported task
frontmatter stores the explicit value. Existing task files that omit it remain
unchanged when completed or reopened.

Complexity is durable because it changes the approved execution contract. It
is not inferred from tokens, files, estimated cost, model confidence, or the
result of a worker run. Changing it after approval is an ordinary task-content
change and requires the existing review discipline.

Bundle review displays complexity and includes an authored value in the
review fingerprint. To preserve old review fingerprints, deserializing an omitted
field must not add `standard` to the canonical bundle used for fingerprinting.
`tasks list`, `tasks show`, `next`, and `goal` expose the effective value; goal
projection is the coordinator's routing input. `TASKS.md` needs no new column.

### Three profiles are enough

Each harness has three whole worker profiles:

- `implementer`: the cost-conscious default for standard implementation;
- `verifier`: the cost-conscious, always-independent reviewer;
- `strong-implementer`: the profile for complex planning, complex
  implementation, and verifier-recommended escalation.

There is no planner profile. A planner is a read-only worker rendered from the
resolved `strong-implementer` profile. There is no strong verifier, coordinator,
or auditor profile. Independent verification always uses a fresh `verifier`;
fresh context and read-only evidence handling define independence, not model
price.

This adds only `worker.<harness>.strong-implementer` for each of the five
harnesses. It keeps the existing whole-profile local, global, and built-in
precedence. Profiles never merge model and effort across layers. Existing
`implementer` and `verifier` configuration remains valid and wins over the new
built-ins, even when an existing override is more expensive than the name
suggests. Zdev cannot compare arbitrary models, so “strong” is a routing label,
not a capability claim about a user override.

The cheaper built-ins intentionally replace the current quality-first
defaults. Existing installed integrations do not change in place; `skill check`
reports drift, and the next explicit install renders the new profiles. This is
the only default-behavior change. Explicit local and global profiles retain
their current meaning and precedence.

The initial built-ins are:

| Harness | `implementer` | `verifier` | `strong-implementer` |
| --- | --- | --- | --- |
| Codex | `gpt-5.6-terra medium` | `gpt-5.6-terra medium` | `gpt-5.6-sol high` |
| Claude Code | `sonnet medium` | `sonnet medium` | `opus high` |
| OpenCode | `openai/gpt-5.6-terra medium` | `openai/gpt-5.6-terra medium` | `openai/gpt-5.6-sol high` |
| Pi | `openai/gpt-5.6-terra medium` | `openai/gpt-5.6-terra medium` | `openai/gpt-5.6-sol high` |
| Oh My Pi | `openai/gpt-5.6-terra medium` | `openai/gpt-5.6-terra medium` | `openai/gpt-5.6-sol high` |

These defaults deliberately use only the providers already assumed by zdev's
current built-ins. Codex describes Terra as its everyday workhorse and Sol as
the choice for complex work; it also recommends increasing reasoning effort
only when work needs more planning or analysis. Codex supports explicit model
and effort overrides for spawned agents. [Codex models](https://learn.chatgpt.com/docs/models)
and [configuration reference](https://learn.chatgpt.com/docs/config-file/config-reference)
(accessed 2026-08-20).

Claude Code describes Sonnet as its daily coding model and Opus as its complex
reasoning model. Named subagents accept `model` and `effort`, while environment
and organization policy may override them. [Claude Code model
configuration](https://code.claude.com/docs/en/model-config), [subagent
configuration](https://code.claude.com/docs/en/sub-agents), and [cost
guidance](https://code.claude.com/docs/en/costs) (accessed 2026-08-20).

OpenCode subagents accept a `provider/model-id`; omitted models inherit from
the caller, and extra fields such as `reasoningEffort` are provider-specific.
The existing adapter rule therefore remains: explicit effort is allowed for
`openai/*`, while other provider prefixes require `inherit`. [OpenCode
agents](https://opencode.ai/docs/agents/) (accessed 2026-08-20).

Pi accepts `--model provider/id` and `--thinking`; zdev already starts a fresh
Pi process per worker and can pass the resolved profile without adding a Pi
scheduler. [Pi coding-agent README](https://github.com/earendil-works/pi/blob/main/packages/coding-agent/README.md)
(accessed 2026-08-20).

Oh My Pi task agents accept a prioritized model and a thinking level in agent
definitions. The task tool resolves named agents and may apply native settings
or supported-level clamping, so zdev reports its rendered profile rather than
claiming the provider's final choice. [Oh My Pi agent
discovery](https://github.com/can1357/oh-my-pi/blob/main/docs/task-agent-discovery.md)
and [task tool](https://github.com/can1357/oh-my-pi/blob/main/docs/tools/task.md)
(accessed 2026-08-20).

### Complex work gets one explicit plan

Before the first code edit for a `complex` task, the coordinator starts a
fresh read-only planner using `strong-implementer`. The planner receives the
same goal, brief, task, repository guidance, and three-part Git baseline as an
implementer. It returns:

```text
PLAN planner <area> <task-id>

Baseline: <HEAD commit>
Approach: <ordered implementation approach>
Paths: <expected task-owned paths>
Validation: <checks from the approved contract>
Decisions: none
```

`BLOCKER planner <area> <task-id>` is the only alternative. The plan is a
conversation handoff, not a repository file or zdev record. It cannot add
scope, relax validation, or amend the approved task. The coordinator checks
the subject, baseline, paths, and absence of unresolved decisions, then passes
the plan unchanged to a fresh strong implementer.

Planning is skipped for `standard` tasks, explicit `zdev-verify`, and a resumed
workflow that already holds a valid plan for the same task and unchanged
baseline. It is also skipped when implementation or rework already exists;
planning after attributed task edits would not protect the first implementation
choice. Unexplained or ambiguously owned edits still block under the existing
baseline rules. An explicit user request may add planning to a standard task,
but may not suppress planning for an approved complex task.

### Escalation is a recommendation, not a verdict

The current strict verifier object always contains `escalation`. Its value is
`none`, except that verifier `rework` may request `advanced-implementer`. This
design would map that request to its proposed strong implementer role. An
unknown value, duplicate key, or advanced escalation with `pass` or `blocker`
is invalid and therefore blocking under the current fail-closed rule.

The verifier recommends escalation only when its concrete findings show that
the repair needs broader reasoning within the already approved scope. It does
not recommend escalation for an unavailable model, transport failure, missing
evidence, unsafe scope, or a product decision; those are blockers. The
coordinator may move only from `implementer` to `strong-implementer`, once per
task run. There is no stronger tier, downgrade, retry count, model search, or
automatic change to durable complexity. A strong implementation that receives
verifier verdict `rework` returns to the strong implementer and then to a new
verifier.

## Coordinator routing

The coordinator retains branch checks, baseline ownership, task selection,
user questions, envelope validation, lifecycle changes, and commits.

1. Read the effective task complexity from `zdev goal`.
2. For `standard`, start the resolved `implementer`. For `complex`, obtain the
   valid plan above, then start a fresh `strong-implementer` with that plan.
3. Inspect the checkout and start a fresh resolved `verifier`.
4. On ordinary verifier verdict `rework`, return findings to the same profile,
   resuming the worker only where the harness safely supports it.
5. On verifier verdict `rework` with envelope `escalation` set to
   `advanced-implementer`, start a replacement worker using this design's
   proposed `strong-implementer` profile, with the goal, baseline, current
   diff, and all findings.
6. After every repair, start another fresh verifier and check the whole task.
7. Stop for verdict `blocker`, an unsafe or changed task/baseline, scope outside
   the approved task, or any choice that belongs to the user. Only verifier
   verdict `pass` permits task completion and commit.

Independent verification remains mandatory in every route. A cheaper verifier
may recommend a stronger implementer, but never verifies its own work or turns
its recommendation into acceptance.

## Five cases across five harnesses

The case policy is common:

| Case | Route |
| --- | --- |
| Standard success | default implementer → fresh verifier verdict `pass` → complete and commit |
| Complex success | read-only strong planner → fresh strong implementer → fresh verifier verdict `pass` |
| Ordinary repair | default implementer → verifier verdict `rework`, escalation `none` → same-profile repair → fresh verifier |
| Escalated repair | default implementer → verifier verdict `rework`, escalation `advanced-implementer` → replacement worker using the proposed strong profile → fresh verifier |
| Product decision | planner blocks or a worker returns verdict `blocker` → coordinator asks the user; no completion or commit |

Each harness realizes every row through these native seams:

| Harness | Planner | Default and strong implementation | Verification and rework |
| --- | --- | --- | --- |
| Codex | fresh read-only subagent with the strong model/effort override | fresh subagent with the selected profile | fresh verifier each time; follow up only for same-profile repair; escalation spawns a replacement |
| Claude Code | `zdev-planner` named agent rendered from the strong profile | `zdev-implementer` or `zdev-strong-implementer` in the existing workflow | `zdev-verifier`; workflow resumes only same-profile repair and starts a strong replacement on escalation |
| OpenCode | read-only `zdev-planner` named subagent | `zdev-implementer` or `zdev-strong-implementer` | new verifier task each time; `task_id` resume only for same-profile repair |
| Pi | `zdev_subagent` role `planner` with read-only tools | role `implementer` or `strong-implementer` | role `verifier`; every repair is a fresh process, with the selected profile |
| Oh My Pi | blocking read-only `zdev-planner` task agent | blocking `zdev-implementer` or `zdev-strong-implementer` | fresh `zdev-verifier`; `hub` only for same-profile repair, replacement task for escalation |

The product-decision case stops in the coordinating session in all five
harnesses. Native transport, resumption, background jobs, teams, and fan-out do
not change the routing contract.

## Smallest implementation seam

- `src/tasks.rs`: add the strict two-value complexity type to bundle and task
  parsing, preserve omitted-field compatibility and review fingerprints, and expose
  the effective value through task views. Keep the parsed field optional with
  a `standard` accessor; new imports render an explicit value while old files
  stay byte-stable through completion and reopen.
- `src/goal.rs`: add complexity to the goal's task projection and human output.
- `src/config.rs`: add one optional `strong-implementer` profile per harness to
  the existing strict worker schema, fixed key registry, whole-profile
  resolver, show/get/set/unset surfaces, and built-ins. Do not add a tier or
  provider registry.
- `src/integrations.rs` and canonical templates: render the resolved strong
  profile, read-only planner artifacts, strong implementer artifacts, the
  common routing rules, and the optional escalation field. Install and check
  continue to share one MiniJinja render path and publish only after all
  artifacts validate.
- Harness adapters: use explicit Codex spawn overrides; named Claude,
  OpenCode, and Oh My Pi agents; and two added Pi roles. Planner artifacts reuse
  the strong profile but remove edit/write tools.
- Tests: cover one legacy omitted-complexity task, one complex goal, one
  standard route, one planned route, one ordinary rework, one escalation, and
  one invalid escalation envelope. Reuse the existing deterministic template
  and harness-contract tests; do not build a provider matrix or harness
  simulator.

## Follow-up implementation tasks

1. **Add task complexity and goal projection.** Implement the two-value task
   schema, old-file and old-fingerprint compatibility, authoring/review output,
   and list/show/next/goal exposure with focused black-box coverage.
2. **Add the strong implementer profile and native worker artifacts.** Extend
   the fixed worker registry and whole-profile resolver by one role per harness;
   render strong implementer and read-only planner artifacts from that profile;
   update exact config documentation and focused resolver/rendering coverage.
3. **Route planning, repair, and escalation.** Update the common workflow and
   five adapters, preserve the typed verifier verdicts, use the existing
   constrained escalation field, regenerate checked-in integrations, and trace the
   five cases above through focused contract tests.

Tasks 1 and 2 can proceed independently. Task 3 depends on both. None adds
evaluation, benchmarking, telemetry, model discovery, provider catalogs,
derived-task authority, or optional verification.

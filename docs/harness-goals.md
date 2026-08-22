# Deterministic goals across harnesses

> **Status: current behavior.** The `zdev goal` command and installed prompt
> integration described here are implemented. Harness research was checked on
> 2026-08-20; harness features can change, but `zdev goal` output is stable.

## What the harnesses provide today

| Harness | Observed capability | zdev integration point |
| --- | --- | --- |
| Codex | Codex has a session goal command. `/goal <objective>` sets a goal, `/goal` shows it, and `edit`, `pause`, `resume`, and `clear` manage it. The objective is limited to 4,000 characters. The feature can be disabled, so it is not universally available. Codex also loads reusable `SKILL.md` workflows. [Codex developer commands](https://learn.chatgpt.com/docs/developer-commands?surface=cli), [goal use case](https://learn.chatgpt.com/use-cases/follow-goals), and [skill documentation](https://learn.chatgpt.com/docs/build-skills) (accessed 2026-08-20). | The installed continuation skills use model-callable `get_goal` and `create_goal`; they do not try to enter interactive composer commands. Ordinary task work uses the complete validated work-context. |
| Claude Code | `/goal` keeps a session running until a model judges a completion condition satisfied. One goal can be active; setting another replaces it. An active goal is restored on session resume, and conditions are limited to 4,000 characters. The evaluator reads the transcript but does not run tools. Plugin skills are namespaced, invocable workflows. [Claude Code goal documentation](https://code.claude.com/docs/en/goal) and [skill documentation](https://code.claude.com/docs/en/slash-commands) (accessed 2026-08-20). | `/zdev:zdev-loop` and `/zdev:zdev-goal` run the same standalone area workflow; `/zdev:zdev-implement` remains one task. Zdev's workflows do not inspect or apply Claude Code's separate `/goal` command. |
| OpenCode | The documented extension is a custom command whose Markdown body becomes a prompt. Command templates accept arguments, shell output, and file references. The official command guide documents built-ins such as `/init`, `/undo`, and `/share`, but no native session-goal lifecycle. OpenCode separately persists and resumes sessions. [OpenCode commands](https://opencode.ai/docs/commands/) and [OpenCode CLI sessions](https://dev.opencode.ai/docs/cli) (accessed 2026-08-20). | The packaged `/zdev-implement <area>` command, or the zdev skill directly, runs `zdev work-context <area> --format json`. It cross-validates the nested goal and status projections with HEAD and exact Git evidence, then supplies the complete context to ordinary task work. No goal emulation or extra state is needed. |
| Pi | Pi prompt templates are Markdown expanded into ordinary prompts and invoked as `/name`; project templates live under `.pi/prompts/`. Skills are loaded on demand and can be invoked as `/skill:<name>`. Sessions are persisted as JSONL and can be resumed, but the documented built-in and extension surfaces do not define a native goal lifecycle. [Pi prompt templates](https://github.com/earendil-works/pi/blob/main/packages/coding-agent/docs/prompt-templates.md), [Pi skills](https://github.com/earendil-works/pi/blob/main/packages/coding-agent/docs/skills.md), and [Pi sessions](https://github.com/earendil-works/pi/blob/main/packages/coding-agent/docs/sessions.md) (accessed 2026-08-20). | The packaged `/zdev-implement <area>` prompt or zdev skill runs `zdev work-context <area> --format json`. It cross-validates the nested goal and status projections with HEAD and exact Git evidence, then uses the complete context for ordinary task work. |
| Oh My Pi | Oh My Pi has a persistent goal runtime. Its create operation refuses to overwrite an unfinished session goal; the runtime supports pause, resume, drop, completion, accounting, and autonomous continuation. Interactive `/goal set <objective>` creates a goal, while `/goal show`, `pause`, `resume`, and `drop` manage it. [Oh My Pi goal runtime](https://github.com/can1357/oh-my-pi/blob/main/packages/coding-agent/src/goals/runtime.ts), [interactive goal command](https://github.com/can1357/oh-my-pi/blob/main/packages/coding-agent/src/modes/interactive-mode.ts), and [goal continuation prompt](https://github.com/can1357/oh-my-pi/blob/main/packages/coding-agent/src/prompts/goals/goal-continuation.md) (accessed 2026-08-20). | The installed continuation prompts use the model-facing `goal` tool with `get`, `create`, and same-goal `resume`; they never invoke the interactive composer. Ordinary task work uses the complete validated work-context. |

The absence statements above are deliberately narrow: they say what the
current official documentation exposes, not what a plugin could build. Skills,
commands, and prompt templates are enough for the zdev behavior, so zdev does
not need to emulate a native goal.

## Zdev meaning of “goal”

A zdev goal is a read-only, deterministic projection of the next ready task in
an area. It is not a new lifecycle object.

The common vocabulary is:

- **Area objective**: the durable reason for the area.
- **Focus task**: the first ready task in the area's existing numeric task
  order, using the same readiness rule as `zdev next`.
- **Slice context**: the focus task's optional slice objective and boundaries.
- **Outcome**: the behavior or artifact the focus task must produce.
- **Context**: optional recorded background for that task.
- **Boundaries**: limits recorded on the slice and task. These remain separate
  so their authority is visible.
- **Done when**: the task's recorded proof conditions.
- **Validation**: the task's recorded checks.
- **Native goal**: a short session condition that points back to the durable
  records. It is a transport value, not durable zdev state.

This definition intentionally selects one task. It fits zdev's approval,
independent-verification, and per-task commit boundaries, and it remains useful
for standing areas such as `general` that are not expected to close. Completing
the focus task completes this rendered goal; the next goal is obtained by
running the command again.

### Command

```text
zdev goal <area>
zdev goal <area> --format json
```

`<area>` is required. Unlike `zdev next`, this command does not infer a default
area and does not require a clean branch or a task-work branch. It reads and
validates records but never changes files, Git state, or a harness session.

### Exact inputs

The projection reads only current `.zdev` records:

1. `area.toml`: `tag`, `title`, and `objective`.
2. Every task header in the area: `id`, `key`, `status`, `slice`, and
   `blocked_by`, plus the task title and the `Outcome`, optional `Context`,
   optional `Boundaries`, `Done when`, and `Validation` sections.
3. If the selected task names a slice, that slice's `key`, `title`, `Objective`,
   and `Boundaries`.

The area `branch`, `parent`, and `base_commit` fields describe workspace
topology rather than intent and are excluded. Free-form area-brief sections are
also excluded: the area metadata objective is the structured source for this
projection. The full area brief remains mandatory reading before
implementation, as it is today. Task result text and done-task details are not
goal inputs.

The command uses the existing record parsers and validators. It must not accept
malformed input merely to produce a partial goal.

### Lifecycle, queue, and ordering

The top-level `lifecycle` is the durable area value, `open` or `closed`. The
derived `queue` is one of:

- `ready`: at least one open task has no unfinished blocker. `task` is the
  first such task.
- `empty`: the area has no tasks. `task` is `null`.
- `exhausted`: tasks exist and all are done. `task` is `null`.

A closed area reports `queue: empty` or `queue: exhausted`, according to its
records, and never selects a task.

Tasks use the current numeric-ID order, with the full ID as the tie-breaker.
Each `blocked_by` array retains its authored order. The associated slice is
read by key. No filesystem enumeration order, clock, current branch, harness,
model, or Git status enters the output.

A validated acyclic graph with an open task always has at least one ready task:
the open subgraph has a task whose dependencies are already done. Missing
blockers, cycles, and malformed graphs fail existing validation before this
queue is projected. There is no successful `blocked` goal queue.

The counts are always present and ordered `total`, `open`, `ready`, `blocked`,
`done`. They describe all tasks, not just the selected task.

### Omission rules

- `task` is always present and is `null` unless `lifecycle` is `open` and
  `queue` is `ready`.
- `native_goal` is present only for `ready`.
- `slice` is omitted from the task when the task has no slice.
- `context` and task `boundaries` are omitted when their Markdown sections are
  absent. A present validated section is emitted even if another section has
  the same text.
- Slice files that are not attached to the selected task are omitted. A slice
  is context, not executable work; an area containing slices but no tasks is
  still `empty`.
- Human output follows the same omissions: it does not print empty headings,
  `null`, or placeholder slice text.

Markdown section values are trimmed exactly as the existing task and slice
parsers trim them. Bullet and checklist markers are preserved. This avoids a
second interpretation of proof conditions.

## Stable output

For the same validated records and zdev version, both formats must be
byte-for-byte stable and end with one newline. JSON key order follows the
examples below; pretty printing uses two-space indentation.

Assume `checkout` contains three tasks. `checkout-001` is done,
`checkout-002` is ready and belongs to the `payments` slice, and
`checkout-003` is blocked by `checkout-002`.

### Human form

```text
Area: checkout — Checkout reliability
Lifecycle: open
Queue: ready
Objective:
Make checkout failures safe and understandable.
Counts: 3 total; 2 open; 1 ready; 1 blocked; 1 done

Task: checkout-002 — Reject duplicate payment submission
Task source: .zdev/checkout/tasks/002-reject-duplicate-payment.md
Outcome:
A repeated submission returns the original payment result without charging again.

Context:
The provider can retry after losing our first response.

Slice: payments — Payment submission
Slice source: .zdev/checkout/slices/payments.md
Slice objective:
Make payment submission safe to retry.
Slice boundaries:
- Do not change provider selection.

Boundaries:
- Keep the public response schema unchanged.

Done when:
- [ ] Duplicate provider calls are prevented.
- [ ] The original result is returned.

Validation:
- Run the focused payment integration test.

Native goal:
Complete zdev task checkout-002 in area checkout. Treat .zdev/checkout/area.toml, .zdev/checkout/slices/payments.md, and .zdev/checkout/tasks/002-reject-duplicate-payment.md as authoritative. Meet the recorded outcome, boundaries, done-when conditions, and validation; preserve zdev approval, branch-safety, independent-verification, task-completion, and commit rules. Stop and report if the task is no longer ready or needs a product decision.
```

The native condition is rendered from a fixed sentence template. For an
unsliced task, the slice path and its preceding comma are omitted. Paths are
repository-relative and use `/` separators.

### JSON form

```json
{
  "schema_version": 1,
  "area": {
    "tag": "checkout",
    "title": "Checkout reliability",
    "objective": "Make checkout failures safe and understandable.",
    "path": ".zdev/checkout"
  },
  "lifecycle": "open",
  "queue": "ready",
  "counts": {
    "total": 3,
    "open": 2,
    "ready": 1,
    "blocked": 1,
    "done": 1
  },
  "task": {
    "id": "checkout-002",
    "key": "reject-duplicate-payment",
    "title": "Reject duplicate payment submission",
    "path": ".zdev/checkout/tasks/002-reject-duplicate-payment.md",
    "outcome": "A repeated submission returns the original payment result without charging again.",
    "context": "The provider can retry after losing our first response.",
    "boundaries": "- Keep the public response schema unchanged.",
    "done_when": "- [ ] Duplicate provider calls are prevented.\n- [ ] The original result is returned.",
    "validation": "- Run the focused payment integration test.",
    "blocked_by": [],
    "slice": {
      "key": "payments",
      "title": "Payment submission",
      "path": ".zdev/checkout/slices/payments.md",
      "objective": "Make payment submission safe to retry.",
      "boundaries": "- Do not change provider selection."
    }
  },
  "native_goal": "Complete zdev task checkout-002 in area checkout. Treat .zdev/checkout/area.toml, .zdev/checkout/slices/payments.md, and .zdev/checkout/tasks/002-reject-duplicate-payment.md as authoritative. Meet the recorded outcome, boundaries, done-when conditions, and validation; preserve zdev approval, branch-safety, independent-verification, task-completion, and commit rules. Stop and report if the task is no longer ready or needs a product decision."
}
```

### No executable work

These are successful observations with exit code 0, not errors:

- `open` / `empty` means that no tasks are recorded and the objective remains
  open.
- `open` / `exhausted` means that tasks exist and all are done, but the area
  has not been explicitly closed.
- `closed` is an explicit lifecycle decision, not a synonym for queue
  exhaustion.

The following fixtures define their complete byte-level output, including the
area header, objective, counts, blank lines, key order, and final newline.

#### Empty human form

```text
Area: general — General improvements
Lifecycle: open
Queue: empty
Objective:
Capture small approved improvements without inventing a product roadmap.
Counts: 0 total; 0 open; 0 ready; 0 blocked; 0 done

The open area has no tasks. Create and approve a task, or close the area.
```

#### Empty JSON form

```json
{
  "schema_version": 1,
  "area": {
    "tag": "general",
    "title": "General improvements",
    "objective": "Capture small approved improvements without inventing a product roadmap.",
    "path": ".zdev/general"
  },
  "lifecycle": "open",
  "queue": "empty",
  "counts": {
    "total": 0,
    "open": 0,
    "ready": 0,
    "blocked": 0,
    "done": 0
  },
  "task": null
}
```

#### Exhausted human form

```text
Area: release-notes — Release notes
Lifecycle: open
Queue: exhausted
Objective:
Keep shipped behavior documented for users.
Counts: 2 total; 0 open; 0 ready; 0 blocked; 2 done

The open area's task queue is exhausted. Add approved work, reopen a task, or close the area.
```

#### Exhausted JSON form

```json
{
  "schema_version": 1,
  "area": {
    "tag": "release-notes",
    "title": "Release notes",
    "objective": "Keep shipped behavior documented for users.",
    "path": ".zdev/release-notes"
  },
  "lifecycle": "open",
  "queue": "exhausted",
  "counts": {
    "total": 2,
    "open": 0,
    "ready": 0,
    "blocked": 0,
    "done": 2
  },
  "task": null
}
```

#### Closed human form

```text
Area: release-notes — Release notes
Lifecycle: closed
Queue: exhausted
Objective:
Keep shipped behavior documented for users.
Counts: 2 total; 0 open; 0 ready; 0 blocked; 2 done

The area is closed. Reopen it before adding or selecting work.
```

#### Closed JSON form

```json
{
  "schema_version": 1,
  "area": {
    "tag": "release-notes",
    "title": "Release notes",
    "objective": "Keep shipped behavior documented for users.",
    "path": ".zdev/release-notes"
  },
  "lifecycle": "closed",
  "queue": "exhausted",
  "counts": {
    "total": 2,
    "open": 0,
    "ready": 0,
    "blocked": 0,
    "done": 2
  },
  "task": null
}
```

No native condition is generated for no-work states. An adapter must report
the lifecycle and queue and stop rather than inventing work from an area
objective or an unattached slice.

## Direct goal application (superseded design)

The standalone projection remains current, but harness task work no longer
uses the call sequence below. One-task implementation and verification use
`zdev work-context <area> --format json`, which nests this goal with matching
status and Git evidence. Explicit area continuation uses the installed
`zdev-loop` route; `zdev-goal` is its exact alias. The earlier direct-application
design is retained here to explain the native-goal conflict rule.

The portable behavior is an ordinary prompt. Native goal mode is an optional
execution aid and is used only when the user explicitly asks to set or apply a
continuing goal.

Every adapter follows the applicable steps in this order. Only Codex and Oh My
Pi use steps 3 and 5; Claude Code, OpenCode, and Pi proceed from step 2 to the
ordinary-prompt path in step 4.

1. Run `zdev goal <area> --format json` and check the command result.
2. Unless lifecycle is `open` and queue is `ready`, report the result and do not start a native
   goal.
3. On a harness with native goals, inspect the current native goal before
   applying any generated context. An active, paused, budget-limited, or
   otherwise unfinished native goal wins. Do not edit, clear, replace, or layer
   an ordinary task prompt over it. Report the conflict and ask the user to keep
   it or explicitly clear/replace it.
4. If ordinary task work was requested and no native-goal conflict exists,
   run `zdev goal <area>` and pass that human rendering to the zdev workflow as
   current context. The adapter does not reproduce the text renderer.
5. If a native goal was explicitly requested and none exists, apply the exact
   `native_goal` string. If the feature is absent, disabled, or unavailable in
   that surface, fall back to the ordinary prompt and say that no native
   continuation was started.

The harness-specific application is:

- **Codex:** call `get_goal`; if clear and a native goal was requested, call
  `create_goal` with the exact condition. Unavailable inspection blocks. When
  native mode is not requested, or inspection proved clear but creation is
  unavailable, use the zdev skill as an ordinary prompt. Codex's own guidance says goals
  suit substantial work with a clear stopping condition and validation loop,
  which is why zdev sends the task-sized condition rather than the whole area
  backlog.
- **Claude Code:** use the plugin skill or `/zdev:zdev-implement <area>` with
  `zdev goal` as ordinary workflow context. The integration neither inspects
  nor applies Claude Code's separate `/goal` command.
- **OpenCode:** use the zdev skill or `/zdev-implement <area>` command to run the
  binary and place the human output in the normal prompt. Do not create a
  project file or plugin-owned goal to imitate a native feature.
- **Pi:** use the zdev skill or `/zdev-implement <area>` prompt template to run the
  binary and place the human output in the normal prompt. The persisted session
  transcript carries that prompt; no separate goal record is written.
- **Oh My Pi:** call `goal` with `op: "get"`; if clear and a native goal was requested,
  call it with `op: "create"` and the exact condition. Use `op: "resume"` only
  for the same paused goal. Unavailable inspection blocks. When native mode is
  not requested, or inspection proved clear but creation is unavailable, use the zdev skill with an ordinary
  prompt. Do not call the runtime's replacement or drop operation implicitly.

Native goal completion never marks a zdev task done and never commits. The
coordinator still performs current-state validation, independent verification,
`zdev task done`, and `zdev commit` under the existing workflow. Conversely,
changing a zdev record does not silently rewrite a session goal. Rerun
`zdev goal`; replacing an already-applied native goal requires an explicit user
decision.

## Failure behavior

Missing repositories or areas, unreadable files, invalid schemas, malformed
Markdown sections, invalid dependencies, and missing referenced slices are
command errors. Text mode writes the normal `error: ...` form to standard error.
JSON mode uses zdev's existing error envelope with `schema_version`, `command`,
`ok: false`, and `error`. Both return nonzero and write no successful payload.

In particular, a missing blocker or dependency cycle fails the existing graph
validator. If open tasks somehow remain with no ready task after validation,
that is a malformed internal projection and also fails; it is not rendered as
a fourth state.

Failure is non-mutating. `zdev goal` does not acquire a write lock, repair an
index, create a task, update status, touch Git, or call a harness. An adapter
must parse a successful complete JSON document before sending any prompt or
native-goal command. On command or parse failure it leaves the current session
goal unchanged and reports the error.

## Implemented projection seam and retained acceptance record

The implementation added one read-only `Goal` CLI variant routed from
`src/lib.rs` to a small goal
projection/rendering module. It reuses the existing area, slice, task, Markdown,
dependency, and ordering logic through narrow internal read views rather than
parsing the files a second way. The module returns the ordinary `CommandOutput`
with typed serializable fields and renders the fixed native condition. It needs
no storage, lifecycle service, model call, Git operation, or harness detector.

The five canonical integration templates originally applied the adapter order
above. Current templates instead use work-context for task work and the
dedicated continuation routes for native or bounded loops. A native goal API
is not part of the binary seam.

The projection acceptance record requires:

1. `zdev goal <area>` and JSON mode match the fields, ordering, omissions,
   lifecycle and queue rules, and canonical bytes shown for the reachable
   projections in
   this document.
2. Selection is identical to `zdev next` for the same valid task graph, without
   enforcing branch-work gates.
3. Ready goals include the exact structured area, optional slice, and task
   content plus the bounded native condition; no-work states never invent one.
4. Repeated runs over unchanged records produce identical bytes.
5. Invalid input returns the existing text or JSON error contract and leaves
   files, Git, and session state unchanged.
6. Each generated harness integration uses ordinary prompts everywhere and the
   documented native mechanism only when available and explicitly requested.
7. Each native adapter preserves an unfinished session goal and reports the
   conflict instead of replacing it.
8. Focused black-box coverage proves ready output, an unsliced omission, the
   open-empty, open-exhausted, and closed states, deterministic reruns, and representative
   non-mutating malformed-dependency failure. Existing full validation remains
   green.

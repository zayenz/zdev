# Explicit area continuation across harnesses

> **Status: partially implemented.** Shared routing, bounded OpenCode and Pi
> aliases, and native Claude Code continuation are current. Native Codex and
> Oh My Pi continuation adapters remain design.

This record defines an explicit zdev route that completes approved work one
task at a time while an area remains open and ready. Research was checked on
2026-08-20. Observed harness behavior and proposed zdev behavior are kept
separate.

## Current harness evidence

The observation points are Codex CLI 0.148.0 ([release and source
revision](https://github.com/openai/codex/releases/tag/rust-v0.148.0)), Claude
Code 2.1.237 ([release](https://github.com/anthropics/claude-code/releases/tag/v2.1.237)),
OpenCode 1.18.19 ([release](https://github.com/anomalyco/opencode/releases/tag/v1.18.19)),
Pi 0.84.2 ([release](https://github.com/earendil-works/pi/releases/tag/v0.84.2)),
and Oh My Pi 17.4.0 ([release](https://github.com/can1357/oh-my-pi/releases/tag/v17.4.0)).
They are evidence points, not proposed minimum versions. All links in this
record were accessed on 2026-08-20.

| Harness | Observed continuation and installed-command surfaces |
| --- | --- |
| Codex | `/goal` creates one session goal and supports show, edit, pause, resume, and clear. Skills have explicit `$name` invocation, and `agents/openai.yaml` can disable implicit invocation. [Developer commands](https://learn.chatgpt.com/docs/developer-commands?surface=cli) and [skills](https://learn.chatgpt.com/docs/build-skills). |
| Claude Code | Dynamic JavaScript workflows own loops and intermediate values, resume within the same session, and are distributed under a plugin namespace. The workflow runtime provides `agent()` and `pipeline()` but no module loading, direct shell access, or API for the separate user-facing `/goal` command. [Workflows](https://code.claude.com/docs/en/workflows), [goals](https://code.claude.com/docs/en/goal), and [plugin paths](https://code.claude.com/docs/en/plugins-reference#component-path-fields). |
| OpenCode | Markdown files under `commands/` become slash commands. Sessions can continue or resume, but the documented command and session surfaces do not define a native goal lifecycle. [Commands](https://opencode.ai/docs/commands/), [TUI sessions](https://opencode.ai/docs/tui/), and [CLI sessions](https://dev.opencode.ai/docs/cli). |
| Pi | Markdown prompt templates become `/name` prompts. Sessions are persisted and resumable, but the documented prompts, skills, sessions, and extension surfaces do not define a native goal lifecycle. [Prompt templates](https://github.com/earendil-works/pi/blob/914cf1472e715297caa30db4b9535d534a9eb718/packages/coding-agent/docs/prompt-templates.md), [skills](https://github.com/earendil-works/pi/blob/914cf1472e715297caa30db4b9535d534a9eb718/packages/coding-agent/docs/skills.md), and [sessions](https://github.com/earendil-works/pi/blob/914cf1472e715297caa30db4b9535d534a9eb718/packages/coding-agent/docs/sessions.md). |
| Oh My Pi | The goal runtime refuses to overwrite unfinished goals and supports pause, resume, drop, completion, and continuation. Prompt templates become slash commands. [Goal runtime](https://github.com/can1357/oh-my-pi/blob/72000acfeb902e21816252699482887f34d1a5a4/packages/coding-agent/src/goals/runtime.ts), [continuation prompt](https://github.com/can1357/oh-my-pi/blob/72000acfeb902e21816252699482887f34d1a5a4/packages/coding-agent/src/prompts/goals/goal-continuation.md), and [prompt loader](https://github.com/can1357/oh-my-pi/blob/72000acfeb902e21816252699482887f34d1a5a4/packages/coding-agent/src/config/prompt-templates.ts). |

These facts support three native continuations, not a claim that their
runtimes behave alike. Codex and Oh My Pi use their goal mechanisms. Claude
uses a plugin JavaScript workflow and does not inspect, set, or depend on
Claude's separate `/goal`. OpenCode and Pi expose the same public intent but
honestly stop after one committed task.

## Public routes and activation

The canonical route is `zdev-loop`; `zdev-goal` is an exact semantic alias:

```text
zdev-loop <area>
zdev-goal <area>
```

Both invocations mean: continue the named area one approved task at a time
while it is open, ready, and safe. They use the same stop states and emit the
same canonical `zdev-loop` envelopes. `zdev-goal` is not a one-task mode and
has no `--native` variant. Retaining both names avoids an unnecessary naming
migration while giving the implementation one behavioral contract.

Inside an active zdev context, natural-language requests to “goal the
`<area>` area” and “loop the `<area>` area” are synonyms for that continuing
route. Zdev is active when the user invoked a zdev entry point, referred to a
specific `.zdev` area, or explicitly asked to use zdev. Generic uses of
“goal” or “loop” outside that context never activate zdev.

This user-facing route does not change the binary. Each iteration uses
`zdev work-context <area> --format json`, whose nested goal remains the
deterministic projection of one ready task.

The installed invocations are exact:

| Harness | Canonical | Alias |
| --- | --- | --- |
| Codex | `$zdev-loop <area>` | `$zdev-goal <area>` |
| Claude Code | `/zdev:zdev-loop <area>` | `/zdev:zdev-goal <area>` |
| OpenCode | `/zdev-loop <area>` | `/zdev-goal <area>` |
| Pi | `/zdev-loop <area>` | `/zdev-goal <area>` |
| Oh My Pi | `/zdev-loop <area>` | `/zdev-goal <area>` |

Both dedicated names always activate zdev. Codex skills set
`allow_implicit_invocation: false`; both Claude workflows run only by explicit
namespaced invocation. Shared skill guidance carries the active-context rule
for natural-language synonyms.

## Existing guidance used by this design

Current Claude guidance routes both names to the packaged workflow and does not
inspect or apply Claude Code's separate native goal. Codex and Oh My Pi retain
their supported native-goal conflict behavior. The continuation design keeps
that division.

The current task workflow classifies validated closed context before status,
Git, and task-work checks. Open work still requires task-work safety and
complete Git evidence. Continuation preserves those rules.

## One-task iteration

An iteration follows the installed `zdev-implement <area>` contract unchanged.
That contract owns complexity routing, optional advanced planning,
implementation, fresh independent verification, concrete REWORK, task
completion, exact staging, and one zdev commit. Claude composes that same
one-task body into its loop artifact at install time; it does not import or
invoke another workflow at runtime.

Before every iteration, the coordinator runs one fresh
`zdev work-context <area> --format json`. The command validates lifecycle,
records, dependencies, selected task, branch safety, HEAD, and exact Git
evidence. Validated closed context returns no work before branch or Git
collection. Open empty or exhausted context remains fully gated. A true
`stale_advisory` is reported once and does not stop work.

A successful iteration returns an exact
`PASS zdev-implement <area> <task-id>` with a nonempty commit ID. The outer
workflow records that pair and obtains new work-context before deciding whether
to continue. Any one-task blocker, malformed result, missing commit ID, or
user-owned decision stops immediately. The area loop never combines two tasks
into one verification or commit and never uses pre-commit selection for the
next iteration.

## Area condition and stop matrix

Codex and Oh My Pi native continuation use this fixed area-sized condition,
with `<area>` replaced by the validated tag. Claude's JavaScript workflow
encodes the same predicate directly in its loop:

```text
For zdev area <area>, repeatedly run the installed zdev one-task implementation contract. After each exact PASS and commit, run a fresh `zdev work-context <area> --format json`. Continue only while its lifecycle is open, queue is ready, task work is safe, and no blocker or user-owned decision exists. Finish when the fresh context is open/empty, open/exhausted, or closed. Stop and report any blocker. Complete and commit exactly one independently verified task per iteration.
```

This condition never replaces the selected task's `native_goal`. The area
condition governs continuation; every `zdev-implement` handoff still receives
the unchanged task-sized projection and `native_goal` from `zdev goal`.

| Fresh state or event | Result |
| --- | --- |
| `open` / `ready`, safe | Run exactly one task iteration. After PASS and commit, collect fresh work-context before deciding or dispatching again. |
| `open` / `empty` | `PASS`; no worker. The area objective remains open. |
| `open` / `exhausted` | `PASS`; no worker. The area objective remains open. |
| `closed` / `empty` or `closed` / `exhausted` | `PASS`; no worker. Report the explicit closed lifecycle without branch status or advisory. The result works off-branch, detached, or during an unrelated Git operation. |
| Unsafe task-work state or unexplained Git state | `BLOCKER`; no worker. Preserve and report exact state. |
| Malformed records, missing blockers, or a dependency cycle | Existing validation error, surfaced as `BLOCKER`; no worker or mutation. |
| Implementer or verifier `BLOCKER`, or completion or commit failure | `BLOCKER`; stop with the one-task workflow's preserved state. Do not count the task or retry it as new work. |
| Concrete task-owned `REWORK` | Stay inside the same iteration. Correct and fully reverify with no fixed retry count. |
| Required user-owned product, scope, or unsafe-expansion decision | `BLOCKER`; stop and ask for that decision. |
| Known unfinished native goal on Codex or Oh My Pi | `BLOCKER`; do not replace, clear, edit, or layer the zdev loop over it. |

For a Codex, Claude, or Oh My Pi native continuation, a refreshed `open` /
`ready` state begins the next iteration. For OpenCode and Pi, the first
successful task commit returns `CONTINUE` with the refreshed next task and
stops. A bounded invocation never claims that the area loop is running in the
background.

The public first lines are:

```text
PASS zdev-loop <area>
CONTINUE zdev-loop <area>
BLOCKER zdev-loop <area>
```

Every body includes `Area`, `Lifecycle`, and `Queue`; then the existing exact
`Advisory` once if any iteration observed stale advisory; then
`Tasks completed`, `Commits`, and `Stop reason`. Tasks and commits include only
successful one-task pairs. Lifecycle or queue is
`unknown` when validation failed before it could be read; commits is `none` or
a comma-separated list of full commit IDs. A direct closed result never has an
advisory. `CONTINUE` then includes `Next
task`. `BLOCKER` then includes `Current task` (`none` when no task was
selected), `Failed stage`, `Reason`, and `Preserved state`. These are
coordinator envelopes; the exact inner implement and verify envelopes remain
those in [Harness orchestration](harness-orchestration.md).

## Per-harness adapters

| Harness | Installed artifacts | Continuation behavior |
| --- | --- | --- |
| Codex | Bundle siblings `zdev-loop/SKILL.md` and `zdev-goal/SKILL.md`, each with `agents/openai.yaml`. | Both skills implement the same continuation contract. They inspect `/goal`, attach the area condition when clear, and fall back to one task with `CONTINUE` if native continuation is unavailable. |
| Claude Code | Plugin workflows `workflows/zdev-loop.js` and `workflows/zdev-goal.js`, rendered from one canonical loop source with only `meta.name` differing. | Both workflows run the same standalone JavaScript loop. They do not read or use `/goal`. If dynamic workflows are disabled or unsupported, the shared zdev skill handles an active-context natural goal/loop request as one bounded task, returns `CONTINUE`, and says the named workflow command was unavailable. |
| OpenCode | `commands/zdev-loop.md` and `commands/zdev-goal.md`, rendered from one command source. | Both prompt commands perform one committed task at most and return `CONTINUE` for refreshed ready work. Session resume is a convenience, not a continuation guarantee. |
| Pi | `prompts/zdev-loop.md` and `prompts/zdev-goal.md`, rendered from one prompt source. | Both prompt templates use the existing zdev skill and subagent extension, perform one committed task at most, and return `CONTINUE`. |
| Oh My Pi | `prompts/zdev-loop.md` and `prompts/zdev-goal.md`, rendered from one prompt source. | Both inspect the current goal, create the area condition when clear, and use the native runtime to continue. If it is unavailable, they perform one task and return `CONTINUE`. |

The common sources belong beside `templates/zdev/task-workflows.md`; adapters
render the files above through the existing all-or-nothing integration path.
No harness receives a fake JavaScript runtime, session database, or scheduler.

### Claude workflow control flow

The installed `zdev-loop` and `zdev-goal` workflows are rendered from one
canonical source; only `meta.name` differs. The source composes the existing
one-task implementation body at install time, so planning, complexity routing,
verification, REWORK, completion, exact staging, and commit validation stay the
same as `zdev-implement` without runtime imports or another workflow call.

Each iteration has four steps:

1. Run one fresh `zdev work-context <area> --format json` preflight.
2. Supply that exact result to the ordinary one-task workflow's initial
   preflight. A valid no-work result stops successfully without a worker.
3. On an exact task PASS with a commit ID, record the task and commit. Any
   implementer, planner, verifier, completion, commit, malformed-result, or
   user-decision blocker stops immediately and does not select another task.
4. Before another iteration, make a new work-context call. The next task is
   never selected from pre-commit context.

Claude may cache a completed agent result when resuming a workflow. That does
not authorize continuation: the next step after a live or cached completion
PASS is still the outer workflow's new work-context call. The ordinary
completion step already refreshes and compares task identity, safety, HEAD,
index, worktree, and untracked evidence before mutation. The loop adds no
reconciliation worker, approval manifest, replay ledger, or second completion
protocol.

Both public names emit the canonical `PASS zdev-loop <area>` or
`BLOCKER zdev-loop <area>` result. The body reports the latest lifecycle and
queue, the stale advisory once when observed, completed task IDs, commit IDs,
and the stop reason. The workflows do not inspect, invoke, or depend on
Claude's separate native goal command.

## Native-goal conflict, restart, and resume

On Codex and Oh My Pi, an active, paused, budget-limited, or otherwise
unfinished native goal wins. Starting either alias reports the conflict and
does nothing. It never silently falls back to an ordinary prompt because that
would layer work over the unfinished goal. Resuming the exact same zdev native
goal in its existing session is continuation, not replacement.

Claude's zdev workflows are a separate native workflow mechanism. They do not
inspect or mutate the separate native goal, which is not an input or fallback.
The workflow runtime owns pause and same-session resume. Repository effects
outlive cached workflow results, so every live or cached task PASS is followed
by fresh work-context before the workflow can select another task. The ordinary
completion step retains its own fresh identity, safety, HEAD, index, worktree,
and untracked comparisons before mutation.

Zdev writes no loop execution record. A new invocation reruns work-context.
Completed task records and commits are the durable checkpoint. Fresh ready
state may start the next task; empty, exhausted, or closed stops successfully;
partial or unexplained state blocks and uses existing recovery guidance.

## Required scenario behavior

- **Ready to ready:** Claude refreshes after the first verified task commit and
  starts the next task. OpenCode and Pi return `CONTINUE` naming it.
- **Ready to exhausted or empty:** the task commits, fresh context reports no
  ready work, and the loop returns `PASS` without closing the area.
- **Closed:** record validation succeeds and returns `PASS` without status,
  Git evidence, task-work gating, advisory, or a worker.
- **Unsafe or malformed:** preflight returns `BLOCKER` before a worker and
  preserves the checkout.
- **REWORK:** correction and fresh verification stay inside the same one-task
  iteration.
- **Failure or user decision:** the loop returns `BLOCKER` and does not
  collect or dispatch the next task.
- **Claude resume:** a live or cached completion PASS is followed by a fresh
  work-context call before another worker.
- **Active native goal:** future Codex and Oh My Pi adapters return `BLOCKER`
  without replacing, clearing, or layering over that goal. Claude does not use
  that state.

## Implementation seams and acceptance

Claude installs `workflows/zdev-loop.js` and `workflows/zdev-goal.js` from
one canonical loop template. Install-time composition embeds the current
one-task workflow body once; the two rendered files differ only in
`meta.name`. Installation and check use the existing all-or-nothing renderer.
Focused executable fixtures cover two-task continuation, closed no-work,
REWORK, cached-result freshness, completion failure, and a user-owned decision.

The remaining Codex and Oh My Pi work keeps its existing native-goal
requirements: paired aliases, unfinished-goal preservation, the fixed area
condition, one independently verified commit per task, and a bounded
`CONTINUE` fallback when native continuation is unavailable. It must not
change Claude's standalone workflow or the bounded OpenCode/Pi behavior.

Across all adapters, closed no-work remains branch-independent, every open
state keeps the complete task-work and Git gate, stale base remains advisory,
and no route switches branches, rebases, stores loop state, weakens independent
verification, or combines task commits.

## Confidence and limitations

Confidence is high in the common one-task and stop contracts because they
reuse current zdev status, goal, lifecycle, and strict workflow behavior.
Confidence is high that Claude's workflow runtime can own this loop: its
[workflow documentation](https://code.claude.com/docs/en/workflows) explicitly
places loops and intermediate results in JavaScript and documents same-session
resume. Confidence is also high that it must not depend on `/goal`, because
the documented workflow API has no such interface or module loading. Codex,
Claude, and Oh My Pi native integration confidence is otherwise moderate until
their adapters are exercised on supported live surfaces (accessed 2026-08-20).

This research did not execute provider-backed harness sessions. Claude users
will receive a native zdev workflow, but its strict cycle and restart behavior
are design conclusions validated against documentation and current source, not
a live provider run.

This design does not add a scheduler, daemon, process manager, cross-harness
session state, duplicate queue, branch switching, rebasing, round-trip
optimization, coordinator model selection, derived-task execution, or trunk
integration mode. Those concerns are outside this task.

# Explicit area continuation across harnesses

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
| Claude Code | The user-facing `/goal` command supports one continuing session goal; setting another replaces it, and a resumed session restores it. The documented skill and isolated JavaScript workflow APIs do not expose a way to inspect or invoke that command. Dynamic workflows can own loops and resume within the same session, and plugin skills and workflows are namespaced. [Goals](https://code.claude.com/docs/en/goal), [workflows](https://code.claude.com/docs/en/workflows), [skills](https://code.claude.com/docs/en/slash-commands), and [plugin paths](https://code.claude.com/docs/en/plugins-reference#component-path-fields). |
| OpenCode | Markdown files under `commands/` become slash commands. Sessions can continue or resume, but the documented command and session surfaces do not define a native goal lifecycle. [Commands](https://opencode.ai/docs/commands/), [TUI sessions](https://opencode.ai/docs/tui/), and [CLI sessions](https://dev.opencode.ai/docs/cli). |
| Pi | Markdown prompt templates become `/name` prompts. Sessions are persisted and resumable, but the documented prompts, skills, sessions, and extension surfaces do not define a native goal lifecycle. [Prompt templates](https://github.com/earendil-works/pi/blob/914cf1472e715297caa30db4b9535d534a9eb718/packages/coding-agent/docs/prompt-templates.md), [skills](https://github.com/earendil-works/pi/blob/914cf1472e715297caa30db4b9535d534a9eb718/packages/coding-agent/docs/skills.md), and [sessions](https://github.com/earendil-works/pi/blob/914cf1472e715297caa30db4b9535d534a9eb718/packages/coding-agent/docs/sessions.md). |
| Oh My Pi | The goal runtime refuses to overwrite unfinished goals and supports pause, resume, drop, completion, and continuation. Prompt templates become slash commands. [Goal runtime](https://github.com/can1357/oh-my-pi/blob/72000acfeb902e21816252699482887f34d1a5a4/packages/coding-agent/src/goals/runtime.ts), [continuation prompt](https://github.com/can1357/oh-my-pi/blob/72000acfeb902e21816252699482887f34d1a5a4/packages/coding-agent/src/prompts/goals/goal-continuation.md), and [prompt loader](https://github.com/can1357/oh-my-pi/blob/72000acfeb902e21816252699482887f34d1a5a4/packages/coding-agent/src/config/prompt-templates.ts). |

These facts support two autonomous native adapters, not a claim that native
goals behave alike. Codex and Oh My Pi can attach an area continuation
condition through their documented native goal surfaces. The installed Claude
Code, OpenCode, and Pi routes are honestly bounded to one task. A Claude user
may separately enter the built-in `/goal` command, but zdev cannot inspect or
perform that action.

## Public routes and activation

The common route names are `zdev-goal` and `zdev-loop`:

```text
zdev-goal <area>
zdev-goal <area> --native
zdev-loop <area>
```

`zdev-goal` obtains the current one-task projection. Without `--native`, it
uses that projection as ordinary prompt context. With `--native`, it applies
the projection's exact `native_goal` when the adapter supports native goals.
On Claude, it instead prints the exact condition and built-in command for the
user and stops. This reuses the projection schema and non-replacement policy
from [Harness goals](harness-goals.md), but supersedes that document's Claude
application instructions because the installed artifacts have no documented
goal API.

`zdev-loop` is an explicit request for continuation. Codex and Oh My Pi may
start their native area loops. Claude, OpenCode, and Pi complete at most one
task and return a continuation result. Neither route changes the binary CLI:
`zdev goal <area>` remains the source of the projection.

The installed invocations are exact:

| Harness | Goal | Loop |
| --- | --- | --- |
| Codex | `$zdev-goal <area> [--native]` | `$zdev-loop <area>` |
| Claude Code | `/zdev:zdev-goal <area> [--native]` | `/zdev:zdev-loop <area>` |
| OpenCode | `/zdev-goal <area> [--native]` | `/zdev-loop <area>` |
| Pi | `/zdev-goal <area> [--native]` | `/zdev-loop <area>` |
| Oh My Pi | `/zdev-goal <area> [--native]` | `/zdev-loop <area>` |

`--native` on OpenCode or Pi says that native continuation was unavailable and
uses ordinary one-task context; it never invents a goal record. On Claude, it
prints the exact `/goal <native_goal>` command, tells the user to run `/goal`
first to inspect any existing goal, and stops without applying task context or
running the command.

The dedicated invocation always activates zdev. The phrases `zdev goal
<area>` and `zdev loop <area>` may route to the same entry points only when
zdev is already active: the user invoked a zdev entry point, referred to a
specific `.zdev` area, or explicitly asked to use zdev. A bare English request
containing “goal” or “loop” does not activate zdev. Codex entry points set
`allow_implicit_invocation: false`; the Claude goal skill sets
`disable-model-invocation: true`. The shared skill guidance for every harness
states the same boundary.

## Current Claude contract to replace

The repository has not implemented this correction yet. Its current guidance
contains four stale claims:

- `docs/harness-goals.md` lines 361–370 give every harness with native goals an
  inspect-then-apply adapter, and lines 382–385 specifically tell Claude to
  inspect `/goal` and run `/goal <native_goal>`. Its acceptance text at lines
  449–452 then requires every generated native adapter to use that mechanism
  and report conflicts, which Claude cannot do.
- `templates/zdev/shared-contract.md` lines 98–104 tell every native-goal
  harness to inspect the current goal and apply `native_goal` when clear. That
  shared text is rendered into Claude guidance even though Claude exposes no
  such skill or workflow API.
- `templates/zdev/claude-skill.md` lines 25–27 explicitly tell the Claude skill
  to inspect `/goal` and apply the command.
- The checked-in generated Claude copy at
  `.claude/skills/zdev/skills/zdev/SKILL.md` contains both stale forms: the
  shared inspect/apply text at lines 105–110 and the Claude-specific command at
  lines 158–160.

Those passages describe current proposed adapter behavior, not a capability
the artifacts can perform. This record supersedes them for Claude. Until a
follow-up changes the canonical sources and regenerates the checked-in copy,
the installed guidance remains stale; this research document alone does not
correct runtime installation. Codex and Oh My Pi keep their native inspection,
conflict, and application behavior.

## One-task iteration

An iteration invokes the installed `zdev-implement <area>` contract unchanged.
That contract owns implementation, fresh independent verification, unlimited
concrete REWORK, task completion, exact staging, and one zdev commit. The area
loop never combines two tasks into one verification or commit.

Before each iteration, and again after its commit, the coordinator obtains:

1. complete `zdev status <area> --format json` output;
2. complete `zdev goal <area> --format json` output;
3. `git status --short --untracked-files=all`, `git diff --cached`, and
   `git diff`, retaining explicit empty results.

The status and goal area, lifecycle, queue, and selected task must agree. A
ready iteration requires `branch_status.task_work.safe: true`. A true
`stale_advisory` is reported once and does not stop work. Invalid records,
dependencies, status, goal, or unexplained Git changes fail before a worker.
The loop deliberately repeats the existing one-task preflight instead of
introducing a shared cache or weaker round trip.

One successful iteration must return an exact matching
`PASS zdev-implement <area> <task-id>` with the verified task and commit ID.
Only then may the loop count that task and refresh state. A result mismatch or
an uncommitted pass is a blocker.

## Area condition and stop matrix

Codex and Oh My Pi native continuation use this fixed area-sized condition,
with `<area>` replaced by the validated tag. Claude's optional user-assisted
path prints the same condition for the user to enter through built-in `/goal`:

```text
For zdev area <area>, repeatedly run the installed zdev one-task implementation contract. After each exact PASS and commit, refresh complete status, goal, and Git evidence. Continue only while lifecycle is open, queue is ready, task work is safe, and no blocker or user-owned decision exists. Finish when a fresh goal is open/empty, open/exhausted, or closed. Stop and report any blocker. Complete and commit exactly one independently verified task per iteration.
```

This condition never replaces the selected task's `native_goal`. The area
condition governs continuation; every `zdev-implement` handoff still receives
the unchanged task-sized projection and `native_goal` from `zdev goal`.

| Fresh state or event | Result |
| --- | --- |
| `open` / `ready`, safe | Run exactly one task iteration. After PASS and commit, refresh before deciding again. |
| `open` / `empty` | `PASS`; no worker. The area objective remains open. |
| `open` / `exhausted` | `PASS`; no worker. The area objective remains open. |
| `closed` / `empty` or `closed` / `exhausted` | `PASS`; no worker. Report the explicit closed lifecycle. Closed observation is branch-independent as defined by [Area lifecycle](area-lifecycle.md). |
| Unsafe task-work state or unexplained Git state | `BLOCKER`; no worker. Preserve and report exact state. |
| Malformed records, missing blockers, or a dependency cycle | Existing validation error, surfaced as `BLOCKER`; no worker or mutation. |
| Implementer or verifier `BLOCKER`, completion failure, or commit failure | `BLOCKER`; stop with the one-task workflow's preserved state. |
| Concrete task-owned `REWORK` | Stay inside the same iteration. Correct and fully reverify with no fixed retry count. |
| Required user-owned product, scope, or unsafe-expansion decision | `BLOCKER`; stop and ask for that decision. |
| Known unfinished native goal on Codex or Oh My Pi | `BLOCKER`; do not replace, clear, edit, or layer the zdev loop over it. |
| Claude user asks for native continuation | Print the exact condition and command, explain that zdev cannot inspect `/goal`, and stop. The user must inspect and, only if clear, separately enter the built-in command. |

For a Codex or Oh My Pi native loop, a refreshed `open` / `ready` state begins
the next iteration. For every bounded loop, including Claude, the first
successful task commit returns `CONTINUE` with the refreshed next task and
stops. A bounded invocation never claims that the area loop is running in the
background. If a Claude user separately starts `/goal`, that built-in runtime,
not the installed zdev skill, owns continuation.

The public first lines are:

```text
PASS zdev-loop <area>
CONTINUE zdev-loop <area>
BLOCKER zdev-loop <area>
```

Every body includes `Area`, `Lifecycle`, `Queue`, `Tasks completed`, `Commits`,
and `Stop reason`, in that order. Lifecycle or queue is `unknown` when
validation failed before it could be read; commits is `none` or a
comma-separated list of full commit IDs. `CONTINUE` then includes `Next task`.
`BLOCKER` then includes `Current task` (`none` when no task was selected),
`Failed stage`, `Reason`, and `Preserved state`. These are coordinator
envelopes; the exact inner implement and verify envelopes remain those in
[Harness orchestration](harness-orchestration.md).

## Per-harness adapters

| Harness | Installed artifacts | Continuation behavior |
| --- | --- | --- |
| Codex | Bundle siblings `zdev-goal/SKILL.md` and `zdev-loop/SKILL.md`, each with `agents/openai.yaml`. | The goal skill uses ordinary projection unless `--native` was explicit. The loop skill inspects `/goal`, then attaches the fixed area condition through the native goal mechanism. If that mechanism is disabled or unavailable, it runs one task and returns `CONTINUE`. |
| Claude Code | Plugin skills `skills/zdev-goal/SKILL.md` and `skills/zdev-loop/SKILL.md`, both explicit-only. | Goal uses ordinary projection. With `--native`, it prints the exact task condition and `/goal` command and stops for the user. Loop performs one committed task at most and returns `CONTINUE`. It may print the exact area condition for optional, separate user invocation of `/goal`, but never inspects, creates, or applies a native goal. |
| OpenCode | `commands/zdev-goal.md` and `commands/zdev-loop.md`. | Both are prompt commands. Loop performs one committed task at most and returns `CONTINUE` for refreshed ready work. Session resume is a convenience, not a continuation guarantee. |
| Pi | `prompts/zdev-goal.md` and `prompts/zdev-loop.md`. | Both are prompt templates using the existing zdev skill and subagent extension. Loop performs one committed task at most and returns `CONTINUE`. |
| Oh My Pi | `prompts/zdev-goal.md` and `prompts/zdev-loop.md`. | Goal uses the native runtime only with `--native`. Loop inspects the current goal, creates the fixed area condition, and lets the native runtime continue. If the goal tool is unavailable, it performs one task and returns `CONTINUE`. |

The common sources belong beside `templates/zdev/task-workflows.md`; adapters
render the files above through the existing all-or-nothing integration path.
No harness receives a fake JavaScript runtime, session database, or scheduler.

The strict Claude JavaScript remains the Claude-specific artifact for an
explicitly invoked one-task `zdev-implement`. Its complete status, goal, and
Git evidence; exact subject matching; stale advisory; verifier envelope; and
unlimited REWORK rules are portable to every area-loop iteration. Its
`agent()` calls, script variables, and workflow progress are Claude-specific. A new loop
workflow would be the wrong boundary: the documented isolated workflow runtime
has no native-goal API, direct shell access, or module loading, so it could not
enforce the `/goal` conflict itself or call the existing script as a module.
[Workflow behavior and limits](https://code.claude.com/docs/en/workflows#behavior-and-limits)
and [Claude goals](https://code.claude.com/docs/en/goal) (accessed 2026-08-20).

## Native-goal conflict, restart, and resume

On Codex and Oh My Pi, an active, paused, budget-limited, or otherwise
unfinished native goal wins. Starting `zdev-goal --native` or `zdev-loop`
reports the conflict and does nothing. It never silently falls back to an
ordinary prompt because that would layer work over the unfinished goal.
Resuming the exact same zdev native goal in its existing session is
continuation, not replacement.

Claude's installed skill cannot make that inspection. Its ordinary bounded
route does not touch `/goal`. When the user asks for the optional native path,
the skill renders the command and stops with this instruction: run `/goal` to
inspect the current state; if any goal is unfinished, do not enter the rendered
command; if none exists, enter it separately. Zdev neither claims the state is
clear nor reports that a native loop has started.

Zdev writes no loop execution record. On a new invocation or a session whose
native continuation cannot resume, the coordinator re-runs status, goal, and
all three Git evidence commands. Completed task records and commits are the
durable checkpoint. A fresh ready state may start the next task; empty,
exhausted, or closed stops successfully; partial or unexplained state blocks
and uses existing recovery guidance. A harness transcript or native goal may
help the user resume, but it is never authoritative over repository state.

## Required scenario behavior

- **Ready to ready:** one task passes, completes, and commits. Codex and Oh My
  Pi native adapters refresh and start the next task; Claude, OpenCode, and Pi
  return `CONTINUE` naming it.
- **Ready to exhausted:** the task commits, refresh reports `open` /
  `exhausted`, and the loop returns `PASS` without closing the area.
- **Closed:** record validation succeeds, then the loop returns `PASS` without
  task-work branch gating or a worker.
- **Unsafe:** preflight returns `BLOCKER` before a worker and preserves the
  checkout.
- **REWORK:** the same selected task remains inside its one-task workflow until
  a fresh verifier passes or a genuine blocker or user decision occurs.
- **Active native goal:** Codex and Oh My Pi return `BLOCKER` without changing
  either goal or applying ordinary zdev context. Claude's bounded route does
  not inspect or touch the goal. Its optional native path prints the condition
  and stops for the user to inspect `/goal`; it never reports the conflict as
  known or starts continuation.

## Implementation seams and acceptance

The follow-up should remain three narrow tasks:

1. Correct the existing goal contract, then add `zdev-goal` routes and exact
   artifacts for all five harnesses. In `docs/harness-goals.md`, replace the
   Claude inspect/apply instructions and their acceptance claim with the
   render-and-stop behavior. In `templates/zdev/shared-contract.md`, replace
   the generic inspect/apply paragraph with a capability-neutral
   non-replacement rule; retain exact inspection and application instructions
   in the Codex and Oh My Pi adapter guidance. Replace the stale block in
   `templates/zdev/claude-skill.md`, then regenerate the checked-in Claude copy
   and every other managed fixture through zdev. Reuse the existing goal
   renderer; add no binary command or stored state.
2. Add the common loop condition, envelopes, activation text, and bounded
   Claude, OpenCode, and Pi adapters. The Claude adapter may render the
   user-assisted `/goal` command but must stop before inspecting or invoking
   it. Prove one-task commit boundaries and exact
   `PASS`/`CONTINUE`/`BLOCKER` parsing without executing a harness.
3. Add native-goal continuation adapters for Codex and Oh My Pi. Leave the
   Claude JavaScript one-task workflows intact; reuse their strict evidence
   and envelope contract in all loop guidance. Test conflict, ready-to-ready,
   ready-to-stop, invalid envelope, and restart reconciliation as
   artifact/control-flow contracts rather than a harness simulator.

Across those tasks, install and check must render identical bytes, validate
all artifacts before publication, preserve unrelated shared-root files, and
regenerate checked-in fixtures through zdev. The first task must prove that no
canonical or generated Claude guidance retains an inspect/create/apply claim,
while Codex and Oh My Pi still inspect and preserve unfinished goals before
native application. Acceptance also requires the exact paths and invocations
above, explicit-only activation, every stop-matrix row, fresh evidence before
and after each commit, one verifier and commit boundary per task, no
replacement of unfinished native goals, honest bounded fallbacks, and no claim
that Claude zdev artifacts can inspect or apply `/goal`.

## Confidence and limitations

Confidence is high in the common one-task and stop contracts because they
reuse current zdev status, goal, lifecycle, and strict workflow behavior.
Confidence is high that Claude must remain bounded: its current public
[skill](https://code.claude.com/docs/en/slash-commands) and
[workflow](https://code.claude.com/docs/en/workflows#behavior-and-limits)
documentation exposes no interface for inspecting or invoking the built-in
goal command (accessed 2026-08-20). Codex and Oh My Pi native integration
confidence is moderate until their adapters are exercised on supported live
surfaces.

This research did not execute provider-backed harness sessions. Claude users
can opt into native continuation only through a separate user-entered `/goal`
command after their own conflict check. That is an honest manual bridge, not
an installed zdev native adapter.

This design does not add a scheduler, daemon, process manager, cross-harness
session state, duplicate queue, branch switching, rebasing, round-trip
optimization, coordinator model selection, derived-task execution, or trunk
integration mode. Those concerns are outside this task.

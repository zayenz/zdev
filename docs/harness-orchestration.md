# Harness orchestration

> **Status: current behavior.** The three workflows and installed artifacts
> described here are implemented. Harness research was checked on 2026-08-20;
> implementation-seam sections preserve the decision record that led to them.

## Versions inspected

| Harness | Version and source revision |
| --- | --- |
| Codex | CLI 0.148.0 (`rust-v0.148.0`), release source revision [`3ba0f711642a888aec92a611a3f3b2211157ff89`](https://github.com/openai/codex/commit/3ba0f711642a888aec92a611a3f3b2211157ff89), including the native multi-agent handlers. [Release](https://github.com/openai/codex/releases/tag/rust-v0.148.0) (accessed 2026-08-20). |
| Claude Code | Claude Code 2.1.237, source/changelog revision [`770933ea1ad2fa7b858191e397a65e6644771c64`](https://github.com/anthropics/claude-code/commit/770933ea1ad2fa7b858191e397a65e6644771c64); Claude Agent SDK 0.3.237, revision [`591a180a197a73ce90042a6f97a7c59c100d2c3a`](https://github.com/anthropics/claude-agent-sdk-typescript/commit/591a180a197a73ce90042a6f97a7c59c100d2c3a). [Claude Code release](https://github.com/anthropics/claude-code/releases/tag/v2.1.237) and [SDK release](https://github.com/anthropics/claude-agent-sdk-typescript/releases/tag/v0.3.237) (accessed 2026-08-20). |
| OpenCode | 1.18.19, release source revision [`2b72179c663cadcb54f54d9f19221b3fb3d11fb6`](https://github.com/anomalyco/opencode/commit/2b72179c663cadcb54f54d9f19221b3fb3d11fb6). [Release](https://github.com/anomalyco/opencode/releases/tag/v1.18.19) (accessed 2026-08-20). |
| Pi | 0.84.2, release documentation/source revision [`914cf1472e715297caa30db4b9535d534a9eb718`](https://github.com/earendil-works/pi/commit/914cf1472e715297caa30db4b9535d534a9eb718). [Release](https://github.com/earendil-works/pi/releases/tag/v0.84.2) (accessed 2026-08-20). |
| Oh My Pi | 17.4.0, source revision [`72000acfeb902e21816252699482887f34d1a5a4`](https://github.com/can1357/oh-my-pi/commit/72000acfeb902e21816252699482887f34d1a5a4). [Release](https://github.com/can1357/oh-my-pi/releases/tag/v17.4.0) (accessed 2026-08-20). |

The version numbers above are observation points, not minimum supported
versions. An implementation should test its actual compatibility floor rather
than infer one from this research.

## Observed harness capabilities

### Codex

Codex supports reusable skills and native subagents. Skills are invoked with
`$name`; the main thread can spawn, message, wait for, interrupt, and close
agent threads. Project or skill instructions may request delegation, and a
custom agent can set its own model, reasoning effort, instructions, and
sandbox. The parent runtime still controls live approval and sandbox choices.
[Codex skills](https://learn.chatgpt.com/docs/build-skills), [subagent
orchestration](https://learn.chatgpt.com/docs/agent-configuration/subagents),
and the pinned [multi-agent spawn
implementation](https://github.com/openai/codex/blob/3ba0f711642a888aec92a611a3f3b2211157ff89/codex-rs/core/src/tools/handlers/multi_agents_v2/spawn.rs)
(accessed 2026-08-20).

This is enough for a zdev skill to remain the coordinator and use fresh native
workers. Zdev does not need an SDK process, task queue, or stored session ID.

### Claude Code

Claude Code plugins can package skills, named agents, and JavaScript workflows.
A workflow script lives in `workflows/` at the plugin root by default; the
manifest's `workflows` string-or-array field can replace that default path.
Plugin workflows are distributed and resolved under the plugin namespace, so
`meta.name = 'release-audit'` in plugin `acme-tools` becomes
`/acme-tools:release-audit`. The feature request that originally identified the
distribution gap was closed as completed on 2026-08-17. [Dynamic workflow
distribution](https://code.claude.com/docs/en/workflows#distribute-a-workflow-in-a-plugin),
[plugin component paths](https://code.claude.com/docs/en/plugins-reference#component-path-fields),
and [Claude Code issue
66032](https://github.com/anthropics/claude-code/issues/66032) (accessed
2026-08-20).

Dynamic workflows require Claude Code 2.1.154 or later. Their runtime executes
plain JavaScript with top-level `await`; `meta` supplies identity, `args`
supplies invocation data, `agent()` starts a subagent, and `pipeline()` maps a
list to agents. Script variables hold intermediate results, and the runtime
keeps the orchestration repeatable and resumable within the session. Named
plugin agents still provide the role prompts, tool constraints, models, and
effort controls used by the workflow. [Workflow behavior and
API](https://code.claude.com/docs/en/workflows) and [plugin subagent
configuration](https://code.claude.com/docs/en/sub-agents) (accessed
2026-08-20).

The previous zdev `zdev-task.js` and current `zdev-audit.js` files therefore use
the right native mechanism. The task workflow is renamed and adapted to the
settled goal, role, and envelope behavior; neither needs replacement with
skills or an Agent SDK wrapper.

### OpenCode

OpenCode discovers Markdown commands from `commands/`; the file name becomes
the slash command and its body becomes a prompt. It also discovers named
Markdown subagents. A primary agent can invoke a subagent through the task tool,
which creates a child session and returns its text. Supplying the prior
`task_id` resumes that child; omitting it creates a fresh child. Background
subagents remain experimental, but zdev's sequential implementation and
verification cycle does not need them. [Commands](https://opencode.ai/docs/commands/),
[agents](https://opencode.ai/docs/agents/), and the pinned [task-tool
implementation](https://github.com/anomalyco/opencode/blob/2b72179c663cadcb54f54d9f19221b3fb3d11fb6/packages/opencode/src/tool/task.ts)
(accessed 2026-08-20).

The OpenCode SDK can create sessions and send prompts or commands, including
structured output. Those calls are useful for external applications, but the
native command and task surfaces already cover zdev's in-session workflow.
[OpenCode SDK](https://opencode.ai/docs/sdk/) (accessed 2026-08-20).

### Pi

Stock Pi intentionally has no built-in subagent system. It supports skills,
Markdown prompt templates, and TypeScript extensions that can register tools
and commands. Its own subagent example starts a separate `pi` process with an
isolated context and captures JSON output. [Pi README and extension
model](https://github.com/earendil-works/pi/blob/914cf1472e715297caa30db4b9535d534a9eb718/packages/coding-agent/README.md)
and the pinned [subagent extension
example](https://github.com/earendil-works/pi/blob/914cf1472e715297caa30db4b9535d534a9eb718/packages/coding-agent/examples/extensions/subagent/index.ts)
(accessed 2026-08-20).

Zdev already follows that pattern with `zdev_subagent`: each call starts a
no-session child with a restricted tool list. That gives implementation and
verification fresh contexts, but it cannot resume an implementer for rework.
A rework pass must start another child with the earlier findings and current
checkout state.

### Oh My Pi

Oh My Pi has native `task` and `hub` tools. `task` discovers named agents,
supports blocking or background execution, returns structured result details,
and can keep a completed non-isolated agent available. `hub` can message a
finished agent and revive it when necessary. Custom prompt templates under
`.omp/prompts/` expand as slash commands. [Pinned task
documentation](https://github.com/can1357/oh-my-pi/blob/72000acfeb902e21816252699482887f34d1a5a4/docs/tools/task.md),
[hub documentation](https://github.com/can1357/oh-my-pi/blob/72000acfeb902e21816252699482887f34d1a5a4/docs/tools/hub.md),
[agent discovery](https://github.com/can1357/oh-my-pi/blob/72000acfeb902e21816252699482887f34d1a5a4/docs/task-agent-discovery.md),
and the pinned [prompt-template
loader](https://github.com/can1357/oh-my-pi/blob/72000acfeb902e21816252699482887f34d1a5a4/packages/coding-agent/src/config/prompt-templates.ts)
(accessed 2026-08-20).

Oh My Pi also offers background jobs, batch fan-out, isolation, process
supervision, and session artifacts. None is required for the ordinary zdev
cycle. The adapter should use blocking named workers in the current checkout;
zdev, not Oh My Pi's job system, remains the workflow owner.

## Public workflows

Zdev should use one stable name for each user intent:

- `zdev-implement <area>` selects and completes one ready task through
  implementation, independent verification, any task-owned rework, task
  completion, and commit.
- `zdev-verify <area> <task-id>` independently verifies the current work for
  one explicit task. `REWORK` is a verdict, not a fourth public workflow. This
  workflow never completes or commits the task.
- `zdev-audit [<boundary>]` performs a read-only codebase audit, checks the
  candidate findings, and returns either no findings, checked findings, or a
  blocker. An omitted boundary means the current repository.

The names do not change with the harness. Only native invocation syntax does:
Codex uses `$zdev-implement`, `$zdev-verify`, and `$zdev-audit`; OpenCode, Pi,
and Oh My Pi use `/zdev-implement`, `/zdev-verify`, and `/zdev-audit`; Claude
Code's workflow namespace produces `/zdev:zdev-implement`,
`/zdev:zdev-verify`, and `/zdev:zdev-audit`.

### Installed artifacts

| Harness | Installed entry points | Native workers |
| --- | --- | --- |
| Codex | `skills/zdev-implement/SKILL.md`, `skills/zdev-verify/SKILL.md`, `skills/zdev-audit/SKILL.md` under the selected Codex scope | The skills pass the resolved `routine-implementer`, `implementer`, `verifier`, or `advanced-implementer` profile when spawning the corresponding native Codex subagent. No durable child ID is installed. |
| Claude Code | One skills-directory plugin whose `.claude-plugin/plugin.json` declares `"workflows": "./workflows/"`, containing `workflows/zdev-implement.js`, `workflows/zdev-verify.js`, and `workflows/zdev-audit.js` with matching `meta.name` values | `agents/zdev-routine-implementer.md`, `agents/zdev-implementer.md`, `agents/zdev-verifier.md`, and `agents/zdev-advanced-implementer.md`. Each `agent()` call selects the scoped native agent type; the existing zdev skill remains shared guidance and an ordinary-agent fallback. |
| OpenCode | `commands/zdev-implement.md`, `commands/zdev-verify.md`, `commands/zdev-audit.md` under the selected OpenCode scope | `agents/zdev-routine-implementer.md`, `agents/zdev-implementer.md`, `agents/zdev-verifier.md`, and `agents/zdev-advanced-implementer.md`; commands use the native task tool. The documented directory is plural `commands/`. |
| Pi | `prompts/zdev-implement.md`, `prompts/zdev-verify.md`, `prompts/zdev-audit.md` | `extensions/zdev-subagent.ts` exposes `routine-implementer`, `implementer`, `verifier`, and `advanced-implementer` with their resolved model and thinking controls; the installed zdev skill supplies the shared contract. |
| Oh My Pi | `prompts/zdev-implement.md`, `prompts/zdev-verify.md`, `prompts/zdev-audit.md` | `agents/zdev-routine-implementer.md`, `agents/zdev-implementer.md`, `agents/zdev-verifier.md`, and `agents/zdev-advanced-implementer.md`, invoked through native `task`; `hub` may resume an implementer for rework. |

These are renderable files, not a new runtime. Install and check must render the
same bytes through the existing integration renderer. The worker model and
effort come from the contract in [Worker profiles](worker-profiles.md).

The installed bundle includes all three entry points. Codex installation
targets the shared `skills/` root so the existing `zdev/` skill and the three
sibling workflow skills form one managed bundle while unrelated skills remain
untouched.

Forced migration has three hard-coded removals: `command/zdev-audit.md` and
`command/zdev-task.md` under an OpenCode shared root, and
`prompts/zdev-task.md` under a Pi shared root. They are previous zdev-owned
entry points replaced by the common names. A non-forced install and readiness
check fail closed while any of them remains, so old and new commands cannot be
reported as one ready installation. No other shared-root file is removed.

## Common execution contract

### Deterministic task selection

`zdev-implement` runs `zdev goal <area> --format json` first. A validated closed
goal returns no work before Git or task-work gates. For an open goal it runs
`zdev status <area> --format json` in the coordinating session and requires
`branch_status.task_work.safe`, reports `stale_advisory` once while continuing,
records status with untracked files plus staged and unstaged diffs, and uses
only the `ready` goal's task as its subject. Structurally unsafe branch state
still blocks. The complete JSON goal is passed unchanged to every worker
together with the brief, repository guidance, baseline, and task-owned paths.

An implement goal that is `open` / `empty`, `open` / `exhausted`, or `closed`
is a successful no-work result. Closed requires no branch or Git evidence;
open no-work retains the open-work gates. No worker is started, and no state
changes. A malformed graph, unsafe open branch, changed focus
task, or other validation error is a blocker. Before verification and before
each rework handoff, the coordinator reruns the status and goal reads and
requires the same task ID. This makes a stale long-running conversation fail
closed instead of implementing a newly selected task.

`zdev-verify` requires the explicit task ID to equal the current ready focus
task. Any no-work goal is a blocker and starts no worker. It uses the same
preflight and baseline comparison but does not invoke an implementer.
`zdev-audit` has no area selection and does not call `zdev goal`.

The native goal behavior described in [Deterministic goals across
harnesses](harness-goals.md) remains optional. Codex and Oh My Pi apply the
short native condition only when the user explicitly asks for a continuing
harness goal and no conflicting native goal exists. Claude Code, OpenCode, and
Pi always use the deterministic zdev goal as ordinary prompt context. Native
goal support is never a prerequisite.

### Ownership and delegation

The coordinating session owns:

- area and task selection, branch gates, the Git baseline, and overlap checks;
- user questions and product decisions;
- choosing the dated or configured worker profile;
- dispatch, result validation, diff inspection, and rework routing;
- `zdev task done`, `zdev commit`, and the final user report.

The `implementer` owns only task-approved source and test changes plus the
recorded validation. It does not edit `.zdev`, delegate, change task lifecycle,
or commit. The `verifier` is a fresh worker for every verdict. It reads the
brief, task, goal, baseline, full diff, and relevant source; runs required
checks; and makes no intentional edits. A validation command that writes files
is reported as `REWORK` and attributed before work continues.

Role selection is fixed by intent. `zdev-implement` uses one `implementer` and
then a fresh `verifier` for each verdict. `zdev-verify` uses only a fresh
`verifier`. `zdev-audit` uses a verifier for the audit and a different fresh
verifier to vet findings; when the boundary is small, the coordinator may omit
the first delegation and ask one fresh verifier for the checked audit directly.
An explicit multi-lens audit uses one verifier per lens and one more for final
vetting. These workers use the resolved profiles; zdev does not invent an
`auditor` model role.

The harness runtime owns child execution, cancellation, and delivery only. A
child session, job, transcript, or agent ID is not zdev state. Audit reviewers
are read-only. For a large boundary or an explicit swarm request, the
coordinator may fan out independent audit lenses, but one fresh verifier checks
and deduplicates their findings before they reach the user.

### Result envelopes

The first line is the machine-checkable envelope. The remainder is concise
Markdown evidence; harness-native structured output may carry the same fields,
but must render this public form.

| Workflow | Allowed first line | Required body |
| --- | --- | --- |
| `zdev-implement` | `PASS zdev-implement <area> <task-id>` or `BLOCKER zdev-implement <area> <task-id>` | Exact repeated area/task fields, summary, changed files, validation, verifier evidence, and commit ID on pass; stage, reason, and preserved state on blocker. |
| `zdev-verify` | One strict verifier JSON object with verdict `pass`, `rework`, or `blocker` | Exact schema version, kind, area, task ID, summary, evidence, findings, and constrained escalation. |
| `zdev-audit` | `PASS zdev-audit`, `FINDINGS zdev-audit`, or `BLOCKER zdev-audit` | Boundary, what was inspected and omitted, and checked findings with location, impact, and confidence. |

Implementer and verifier handoffs are single strict JSON objects with the nine
keys in the canonical task-workflow contract. They contain identity once and
no sentinel line or free-form body. Missing output, malformed JSON, duplicate,
unknown, or missing keys, mismatched identity, contradictory verdict or
escalation, extra text, or an unavailable required artifact becomes a
coordinator-generated `BLOCKER`. It is never interpreted as success.

### Rework, retry, and completion

Each verifier `rework` result returns the concrete findings to the same
implementer when the harness can preserve that worker context, or to a
replacement implementer with the current goal, baseline, and full findings.
Oh My Pi may message the existing implementer through `hub`; OpenCode may
resume its implementer child; Codex and Claude Code may use their native
follow-up mechanism. Pi starts a fresh child. A fresh verifier then checks the
whole task again, not only the repaired lines.

There is no fixed retry or correction count. Concrete task-owned findings keep
returning through implementation and fresh verification until the verifier
reports `pass`. The coordinator stops only for a genuine blocker, unsafe scope
expansion, or a required user-owned decision. Transport errors, unavailable
models, permission failures, timeouts, and invalid envelopes are blockers when
they prevent safe progress; zdev does not hide them behind blind transport
retries or turn the adapter into a scheduler.

Only a verified `pass` allows `zdev-implement` to complete the task and run
`zdev commit`. A completion or commit failure changes the public result to
`BLOCKER`; the report names the successful verification and exact remaining
state. `zdev-verify` and `zdev-audit` never change lifecycle state or commit.
An audit also never creates tasks automatically; the user decides whether a
checked finding should become durable work.

### Unsupported facilities and fallback

If a named worker artifact is unavailable but the harness can start a fresh
generic child with the required tools and resolved model controls, the
coordinator supplies the same role prompt and records that fallback. If it
cannot create a separate verification context, the workflow stops with
`BLOCKER`; the coordinating model must not verify its own work and call that
independent.

If a harness lacks a native goal, the JSON projection remains ordinary prompt
context. If a model or effort is unavailable, the explicit worker-profile
rules apply; there is no model search or silent substitution by zdev. If an
entry-point artifact is absent, direct use of the installed zdev skill can
explain the equivalent manual steps, but it must not claim that the named
workflow ran.

## What stays common and what stays Claude-specific

The current Claude task and audit workflows contain portable ideas that every
adapter keeps:

- a read-only preflight before delegation;
- one coordinator owning the baseline and task lifecycle;
- a separate implementation handoff and fresh evidence-based verification;
- a small verdict vocabulary with invalid output failing closed;
- task-owned rework followed by full verification;
- an audit fan-out followed by one evidence-vetting pass.

The JavaScript execution details stay Claude-specific: the literal `meta`
block, global `args`, top-level `await`, `agent()`, `pipeline()`, labels,
workflow progress rendering, and the plugin workflow namespace. Other adapters
express the same sequence through their own skills, commands, prompts, and
tools; the common contract does not imitate the JavaScript runtime.

Claude should retain and adapt the canonical scripts. Rename `zdev-task.js` to
`zdev-implement.js`, retain `zdev-audit.js`, and add a focused
`zdev-verify.js`. The implementation workflow keeps its deterministic
preflight, implementation, fresh-verification, and `while (REWORK)` control
flow, but routes `agent()` calls through the scoped named role, refreshes the
zdev goal before each handoff, validates the common envelopes, and returns the
coordinator enough evidence to complete and commit. The audit workflow keeps
its review-and-vet pipeline. These are native advantages, not behavior other
harnesses must reimplement in JavaScript.

## Implementation seams

No shared product decision remains. The implementation can stay within the
existing integration renderer and five harness adapters:

1. Define the three common workflow prompts once as canonical template input,
   including goal refresh, ownership, envelopes, and rework rules.
2. Render the exact entry-point files in the table above. Reuse the existing
   named agents and Pi extension, adding only the role controls settled in
   [Worker profiles](worker-profiles.md).
3. Update each adapter's file manifest and destination layout. In particular,
   use OpenCode's documented `commands/` directory; for Claude, declare the
   plugin workflow path and retain/adapt the canonical JavaScript artifacts
   under the three common names.
4. Keep install and check on the same all-or-nothing rendering path. Parse and
   render every artifact before replacing any destination.

The work is complete when:

- all five harnesses install exactly one invocable entry point for each common
  name, and install/check agree byte for byte;
- Claude's manifest exposes all three plugin-root JavaScript workflows under
  the `zdev:` namespace, and each script selects the named worker role while
  preserving the common goal, envelope, and lifecycle contract;
- `zdev-implement` selects the deterministic ready focus, preserves its task ID
  across every handoff, uses the configured implementer and fresh verifier,
  and completes and commits only after `PASS`;
- open empty, open exhausted, and closed areas return an implementation
  no-work pass without delegation or mutation; explicit verification returns
  a blocker for those states; invalid or unsafe state fails before a worker starts;
- every harness accepts and rejects the same result first lines and routes
  concrete task-owned `REWORK` findings through implementation and fresh
  verification with no fixed correction count;
- `zdev-verify` and `zdev-audit` leave the worktree and `.zdev` lifecycle
  unchanged apart from files written by declared validation, which are
  reported as rework;
- a missing native goal falls back to prompt context, while a missing
  independent-worker facility produces a blocker;
- focused integration coverage proves rendered artifact discovery, role
  selection, a pass, a rework cycle, one pre-publication failure, and
  deterministic install/check output without adding a scheduler or harness
  simulator.

This change does not add a scheduler, process manager, cross-harness session
database, benchmark runner, automatic model selection, or durable workflow
record. Area, slice, task, Git, and commit records remain the only durable zdev
state.

## Confidence and limitations

Confidence is high in the common boundary and installed surfaces documented
above: they follow current official documentation and pinned source where the
runtime is open. Confidence is moderate for compatibility over time. All five
harnesses are moving quickly, and several current docs describe experimental
or recently changed behavior.

This investigation did not execute the five harnesses against live provider
accounts. Claude Code's main runtime is distributed as a compiled package, so
its plugin documentation and public changelog provide stronger evidence than
source inspection for plugin discovery. The current official workflow and
plugin references explicitly document the plugin-root `workflows/` directory,
manifest field, and namespaced invocation, and issue 66032 is completed; those
sources support retaining the canonical JavaScript route in 2.1.237. OpenCode
background agents, Oh My Pi background jobs, and native session-goal behavior
were not selected because the proposed sequential workflow does not need them.

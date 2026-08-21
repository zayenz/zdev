# Explicit area continuation across harnesses

> **Status: design only.** Zdev does not install or run this continuation
> contract.

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

This user-facing route does not change the binary. Each iteration still runs
`zdev goal <area> --format json` to obtain the deterministic projection of one
ready task. The CLI name `zdev goal` is an internal projection command here,
not the meaning of the routed user intent.

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

Current Claude guidance uses `zdev goal` as ordinary workflow context and does
not inspect or apply Claude Code's separate `/goal`. Codex and Oh My Pi retain
their supported native-goal conflict behavior. The continuation design keeps
that division.

The current task workflow classifies a validated closed goal before status,
Git, and task-work checks. Open work still requires task-work safety and
complete Git evidence. The continuation implementation must preserve those
rules.

## One-task iteration

An iteration follows the installed `zdev-implement <area>` contract unchanged.
That contract owns implementation, fresh independent verification, unlimited
concrete REWORK, task completion, exact staging, and one zdev commit. An
adapter may invoke that contract directly or, for Claude, execute the same
strict cycle inline. It never pretends to call or import another workflow. The
area loop never combines two tasks into one verification or commit.

Before each iteration, the coordinator first runs complete
`zdev goal <area> --format json`. After a commit, the mandatory reconciliation
below obtains that fresh goal before the outer loop refreshes. These calls
validate the records and task graph and classify lifecycle and queue before a
Git branch gate.

For `open`, it then obtains complete `zdev status <area> --format json` plus
`git status --short --untracked-files=all`, `git diff --cached`, and `git diff`,
retaining explicit empty results. Status and goal area, lifecycle, queue, and
selected task must agree, and `branch_status.task_work.safe` must be true even
for open empty or exhausted queues. A true `stale_advisory` is reported once
and does not stop work.

For validated `closed`, it returns no work immediately. It does not run status
or Git evidence commands, require `task_work.safe`, or emit branch status or
advisory. The closed goal must contain the requested area, its actual empty or
exhausted queue, and `task: null`; malformed records still fail. This replaces
the current Claude `zdev-implement` preflight/parser rule that requires
task-work safety and Git evidence for every no-work result. It is the
branch-independent read contract from [Area lifecycle](area-lifecycle.md).

The loop deliberately repeats the applicable preflight instead of introducing
a shared cache or weaker round trip. Invalid records, dependencies, status,
goal, or unexplained Git changes fail before a worker.

One successful iteration must return an exact matching
`PASS zdev-implement <area> <task-id>` with the verified task, full commit ID,
change ID, commit subject, and completion result. Its changed-path report is
descriptive, never authorization. That PASS is provisional. The outer workflow
must independently reconcile the committed repository state against the
retained verifier-approved snapshot described below before it counts the task
or refreshes the queue. A result mismatch, an uncommitted pass, or failed
reconciliation is a blocker.

## Area condition and stop matrix

Codex and Oh My Pi native continuation use this fixed area-sized condition,
with `<area>` replaced by the validated tag. Claude's JavaScript workflow
encodes the same predicate directly in its loop:

```text
For zdev area <area>, repeatedly run the installed zdev one-task implementation contract. After each exact PASS and commit, refresh the complete goal; for open lifecycle also refresh status and complete Git evidence. Continue only while lifecycle is open, queue is ready, task work is safe, and no blocker or user-owned decision exists. Finish when a fresh goal is open/empty, open/exhausted, or closed. Stop and report any blocker. Complete and commit exactly one independently verified task per iteration.
```

This condition never replaces the selected task's `native_goal`. The area
condition governs continuation; every `zdev-implement` handoff still receives
the unchanged task-sized projection and `native_goal` from `zdev goal`.

| Fresh state or event | Result |
| --- | --- |
| `open` / `ready`, safe | Run exactly one task iteration. After PASS and commit, refresh before deciding again. |
| `open` / `empty` | `PASS`; no worker. The area objective remains open. |
| `open` / `exhausted` | `PASS`; no worker. The area objective remains open. |
| `closed` / `empty` or `closed` / `exhausted` | `PASS`; no worker. Report the explicit closed lifecycle without branch status or advisory. The result works off-branch, detached, or during an unrelated Git operation. |
| Unsafe task-work state or unexplained Git state | `BLOCKER`; no worker. Preserve and report exact state. |
| Malformed records, missing blockers, or a dependency cycle | Existing validation error, surfaced as `BLOCKER`; no worker or mutation. |
| Implementer or verifier `BLOCKER`, completion or commit failure, or failed post-completion reconciliation | `BLOCKER`; stop with the one-task workflow's preserved state. Do not count the task or retry it as new work. |
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
post-completion reconciled pairs. Lifecycle or queue is
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

The canonical Claude loop source is a standalone workflow, not a wrapper
around `/zdev:zdev-implement`. Claude workflows cannot import modules or invoke
another workflow through the documented JavaScript API. The new source must
therefore adapt the existing strict one-task control flow inline:

1. Run one coordinating preflight agent. It obtains and parses goal first. A
   validated closed goal returns the branch-independent closed envelope below.
   An open goal proceeds to complete status and Git baseline evidence.
2. For `open` / `ready`, retain the exact selected task ID and run the existing
   implement, refresh, fresh-verify, unlimited-REWORK, complete, stage, and
   commit sequence. Every current subject, area, task, evidence-key, advisory,
   and first-line parser rule still applies except for the deliberate closed
   no-work split. Retain the complete evidence snapshot supplied to the final
   verifier. On PASS, freeze that approved snapshot and the exact completion
   result derived from the verifier fields, then pass both unchanged to
   completion and reconciliation.
3. Treat an exact `PASS zdev-implement <area> <task-id>` as provisional. Parse
   its full commit ID, change ID, subject, and completion result, then always
   call the read-only post-completion reconciliation agent. Do not use its
   changed-path report as an allowlist.
   Record the task/commit pair only after its exact PASS. Accumulate stale
   advisory as one boolean used by every later return path. Then start the next
   iteration with a new full preflight, which first rechecks the latest
   reconciled commit and completed record before selecting or returning no
   work; never carry a task projection across commits.
4. Return the canonical loop `PASS` on fresh open empty/exhausted or closed
   state. Return `BLOCKER` immediately on unsafe or malformed state, an invalid
   inner envelope, completion or commit failure, failed reconciliation, or a
   user-owned decision.
   Concrete `REWORK` remains inside the selected iteration.

Both aliases use the current exact internal subjects:
`READY zdev-implement <area> <task-id>` or
`NO-WORK zdev-implement <area> <lifecycle> <queue>` from preflight,
`DONE implementer`, `PASS|REWORK|BLOCKER zdev-verify`, and finally
`PASS|BLOCKER zdev-implement`. Post-completion reconciliation uses
`PASS|BLOCKER zdev-reconcile`. Ready and open no-work payloads retain the
current exact status, goal, and three Git evidence strings. Closed no-work has
exactly `area`, `goal_json`, `lifecycle`, and `queue`; nested goal must agree,
have `task: null`, and omit `native_goal`. Status, Git evidence,
`branch_status`, and `advisory` are absent, not null.

All other existing exact area, task, field, JSON-key, and first-line validation
continues. A missing field, extra key, suffixed first line, mismatched area or
task, changed ready selection, or malformed nested status or goal is a loop
blocker. `meta.name` never changes these subjects or the canonical public
`zdev-loop` result.

### Replay-safe mutation

Claude resume replays completed agent results and restarts the first unfinished
agent. [Claude workflow resume](https://code.claude.com/docs/en/workflows#resume-after-a-pause)
(accessed 2026-08-20). A completed preflight result is therefore historical
evidence, not fresh authority for a restarted mutating agent. Every
implementation, REWORK repair, and completion/commit agent must perform this
check itself before its first edit, lifecycle mutation, staging action, or
commit:

1. rerun goal, then status and all three Git evidence commands for the exact
   area, following the closed short-circuit above; closed returns a blocker for
   a selected mutating agent without running the remaining checks;
2. require `open` / `ready`, the unchanged selected task ID, and
   `branch_status.task_work.safe: true`;
3. compare current staged, unstaged, and untracked state with the supplied
   baseline and inspect every difference against the task-owned boundary;
4. treat the supplied plan, baseline, and any replayed implementer or verifier
   text only as context to reconcile, never as proof that current state is
   safe.

An initial or restarted implementer may continue already-present work only
when the selected task is unchanged and every current difference is clearly
task-owned and consistent with the supplied plan. A repair agent additionally
requires the exact current findings subject. A completion agent treats the
supplied verifier PASS as historical: it requires the exact retained
pre-verification snapshot and proves that fresh status, goal, staged,
unstaged, and untracked bytes still match that snapshot before using the PASS.
Any difference requires a new outer refresh and fresh verifier, so the
restarted completion blocks rather than staging or committing.

If the task advanced, a prior completion already committed, evidence cannot be
refreshed, or ownership is unexplained or overlapping, the agent performs no
further mutation and returns its exact existing blocker envelope. The outer
workflow converts that to `BLOCKER zdev-loop <area>` with failed stage
`replay safety`, the current task or `none`, all earlier committed task/commit
pairs, and the preserved checkout. Replayed completed evidence is never
accepted as a fresh verifier, safety check, or permission to commit again.

### Post-completion reconciliation

Completion is the only agent step that changes both the task record and Git
history, so its PASS is not authority to advance the outer loop. Immediately
after every completion PASS, whether the runtime produced it live or replayed
it from the workflow cache, the outer workflow calls a separate read-only
verifier agent with the expected area, task ID, full commit ID, change ID,
commit subject, completion result, retained verifier-approved snapshot, and
pre-task Git baseline. The completion agent's path list is supplied only so a
mismatch can be reported; it grants no path or byte permission. No task/commit
pair is appended before reconciliation passes.
The agent uses ordinary zdev and Git commands directly; the workflow itself
does not need shell access. The concrete seam is this one call followed by the
strict parser; it does not reuse the completion agent:

```javascript
await agent(reconciliationPrompt, {
  agentType: 'zdev:zdev-verifier',
  label: 'zdev post-completion reconciliation',
})
```

Before completion starts, the outer workflow freezes one approval manifest.
It contains the full base commit and tree; the exact verifier PASS and
pre-verification status, cached diff, unstaged diff, and untracked-file blob
IDs; and a bytewise path-sorted task delta. Each task-delta entry contains path,
add/modify/delete operation, old and approved file modes, old blob ID, and
approved blob ID or a deletion marker. `git hash-object --path=<path> <path>`
computes the exact repository blob after configured clean filters without
changing the object database, so approved untracked source, test, or generated
files are included by committed bytes and mode. The task delta excludes all
unrelated baseline entries and excludes the exact task file and
`.zdev/<area>/TASKS.md`, which are reserved for deterministic completion.
Overlapping task and unrelated paths block before completion. The exact
task-done summary and validation strings are derived from the accepted verifier
fields and frozen in the same manifest; the completion agent cannot substitute
them.

The reconciliation agent obtains new command output and requires all of these
facts at once:

1. `git rev-parse --verify <commit>^{commit}` resolves to the reported full
   commit, `git rev-list --parents -n 1 <commit>` shows exactly the retained
   base commit as its sole parent, `git rev-parse HEAD` is exactly that commit,
   and
   `git merge-base --is-ancestor <commit> HEAD` succeeds. The loop contract
   deliberately requires the task commit to remain the current tip; merely
   finding the object somewhere in the repository is insufficient.
2. `zdev change inspect <commit> --format json` reports that exact commit,
   exact change ID, and exact subject.
3. In a disposable clone checked out at the retained base commit on the task
   branch, the agent runs the installed `zdev task done` with the frozen
   summary and validation. It requires that this changes only the exact task
   file and `.zdev/<area>/TASKS.md`, then records those two resulting mode and
   blob-ID entries. The clone is removed afterward and never shares an index or
   worktree with the user's checkout. This independently derives the lifecycle
   bytes through zdev's canonical writer instead of reimplementing Markdown or
   trusting the completion report.
4. The agent obtains the actual commit delta with
   `git diff-tree --raw --no-renames -r -z <base> <commit>` and compares its
   bytewise path-sorted `(path, operation, old mode/blob, new mode/blob)` map
   with exactly the union of the frozen verifier-approved task delta and the
   two independently derived task-done entries. Equality of Git blob IDs and
   modes is the byte and file-identity comparison. No report from completion
   can expand this union. A file or byte first introduced, removed, or altered
   during completion therefore fails even when completion listed it and the
   working tree is clean afterward.
5. `zdev task show <area> <task-id> --format json` identifies the same area and
   task, reports it done, and contains the exact frozen summary and validation
   result approved before completion. A missing task, a
   reopened task, or another done task does not match.
6. Fresh goal and status identify the same requested area, the lifecycle is
   still `open`, and the completed task is no longer the selected ready task.
   The refreshed queue may be ready, empty, or exhausted. A completion does
   not implicitly close an area.
7. Fresh `git status --short --untracked-files=all`, `git diff --cached`, and
   `git diff` show every task-owned path clean. Every unrelated staged,
   unstaged, or untracked baseline entry is still present with byte-identical
   content, and there is no new unexplained entry. This compares full retained
   evidence, not just path names, so the task commit cannot absorb, discard, or
   rewrite unrelated work. The pre-task snapshot therefore retains a Git blob
   ID from `git hash-object` for each unrelated untracked file in addition to
   the three ordinary Git evidence strings; reconciliation recomputes those
   IDs.

Its first line is exactly `PASS zdev-reconcile <area> <task-id>` or
`BLOCKER zdev-reconcile <area> <task-id>`. A PASS includes exact `Area`,
`Task`, `Commit ID`, `Change ID`, `Commit subject`, `Task result`, `HEAD`,
`Base commit`, `Approved task delta`, `Expected task-done delta`,
`Actual commit delta`, `Git state`, `Area state`, and `Located evidence`
fields. The three delta fields are canonical compact JSON arrays in bytewise
path order and contain modes and blob IDs, not prose path lists. A blocker
includes the same available identity and delta fields plus `Failed check`,
`Reason`, and `Preserved state`.
The outer parser rejects omitted, extra, malformed, or mismatched fields and
returns `BLOCKER zdev-loop <area>` with failed stage
`post-completion reconciliation`. It preserves earlier reconciled pairs but
does not count this task or retry it as newly selected work.

This call sits directly after completion in the JavaScript control flow, with
no intervening agent or mutation. Under Claude's resume rule, a cached
completion is therefore followed either by the first unfinished reconciliation
agent, which reruns, or by a reconciliation that already completed before a
later agent paused. In the latter case the next unfinished outer preflight is
fresh and repeats the latest pair's commit, exact approval-manifest delta,
record, area, and Git checks before it selects another task or returns a
terminal result; every restarted mutating agent also performs its own
replay-safety check. The manifest and verifier snapshot stay in the outer trace
and are not reconstructed from cached completion text. The script
unconditionally evaluates the reconciliation call when it reconstructs the
trace; it never uses a cached completion PASS alone to append a pair, start
another task, or return success. A reset, missing or replaced commit, changed
HEAD, reopened task, mismatched subject or change ID, changed result, changed
commit delta, closed or different area, or unexpected Git state invalidates the
latest pair, removes it from the reported task and commit counts, and blocks at
the first fresh check. It is not retried as new work; older independently
reconciled pairs remain in the trace.

Copying the mature cycle into two separately maintained sources would invite
parser drift, while refactoring the working `zdev-implement.js` in the same
change would add avoidable risk. The narrow implementation is one canonical
loop template rendered twice for the two public names. It intentionally
duplicates the one-task cycle from `zdev-implement.js` once. Focused behavior
fixtures exercise the canonical loop source; one alias-equivalence assertion
proves the two rendered artifacts differ only in `meta.name`. A later shared
source fragment is justified only if the implement and loop cycles need to
change together often. This respects the workflow runtime's documented lack
of module loading without building a JavaScript framework. [Workflow behavior and
limits](https://code.claude.com/docs/en/workflows#behavior-and-limits)
(accessed 2026-08-20).

## Native-goal conflict, restart, and resume

On Codex and Oh My Pi, an active, paused, budget-limited, or otherwise
unfinished native goal wins. Starting either alias reports the conflict and
does nothing. It never silently falls back to an ordinary prompt because that
would layer work over the unfinished goal. Resuming the exact same zdev native
goal in its existing session is continuation, not replacement.

Claude's zdev workflows are a separate native workflow mechanism. They neither
inspect nor mutate `/goal`, and `/goal` is not an input or fallback. The
workflow runtime owns pause and same-session resume. Because repository effects
outlive workflow variables, a resumed or restarted loop still performs the
full outer preflight before its next worker, every restarted mutating agent
performs its own fresh replay-safety check immediately before acting, and a
completion PASS cannot enter the completed-pair trace until post-completion
reconciliation passes. If that reconciliation was completed before a later
pause, the first unfinished preflight rechecks the reconciled tip before any
new work. Cached completed agent output is context only.

Zdev writes no loop execution record. On a new invocation or a session whose
native continuation cannot resume, the coordinator re-runs goal-first
classification and, for open state, status and all three Git evidence commands.
Completed task records and commits are the durable checkpoint. A fresh ready
state may start the next task; empty, exhausted, or closed stops successfully;
partial or unexplained state blocks and uses existing recovery guidance. A
harness transcript or native goal may help the user resume, but it is never
authoritative over repository state.

## Required scenario behavior

- **Ready to ready:** one task passes, completes, and commits. Codex and Oh My
  Pi native adapters and the Claude workflow refresh and start the next task;
  OpenCode and Pi return `CONTINUE` naming it.
- **Ready to exhausted:** the task commits, refresh reports `open` /
  `exhausted`, and the loop returns `PASS` without closing the area.
- **Closed:** record validation succeeds, then the loop returns `PASS` without
  status, Git evidence, task-work branch gating, advisory, or a worker. The same
  result holds off-branch and on detached HEAD.
- **Unsafe:** preflight returns `BLOCKER` before a worker and preserves the
  checkout.
- **REWORK:** the same selected task remains inside its one-task workflow until
  a fresh verifier passes or a genuine blocker or user decision occurs.
- **Active native goal:** Codex and Oh My Pi return `BLOCKER` without changing
  either goal or applying ordinary zdev context. Claude's workflow does not use
  `/goal`; it leaves that separate state untouched and follows its own strict
  workflow stop rules.
- **Claude resume during mutation:** the restarted implement, repair, or
  completion agent refreshes and reconciles its own evidence before acting.
  Changed selection, foreign changes, a changed verified snapshot, or an
  already committed task returns replay-safety `BLOCKER` without another edit
  or commit.
- **Claude completion replay:** a live or cached completion PASS is followed
  by post-completion reconciliation before the pair is counted. A missing or
  reset commit, changed HEAD, reopened task, mismatched subject, change ID, or
  result, different or closed area, or altered unrelated baseline returns
  `BLOCKER` and does not become another iteration.
- **Completion adds a clean file or byte:** reconciliation compares the actual
  commit delta with the frozen verifier-approved delta plus independently
  derived task-done bytes. A completion-only path or content change blocks even
  if it was reported and left no working-tree change.

## Implementation seams and acceptance

The follow-up should remain three narrow tasks:

1. Establish the common continuing intent, canonical envelopes, active-context
   routing, and alias manifests. Correct `docs/harness-goals.md`,
   `templates/zdev/shared-contract.md`, and `templates/zdev/claude-skill.md` so
   Claude is never told to inspect or apply `/goal`; retain Codex and Oh My Pi
   conflict behavior. Correct `templates/zdev/task-workflows.md` and related
   orchestration documentation so closed is classified before branch safety
   while every open state retains the full gate. Add the paired OpenCode
   commands and Pi prompts, whose two names render the same bounded one-task
   contract, then regenerate all managed copies.
2. Add the canonical Claude JavaScript loop template and render it as both
   `workflows/zdev-loop.js` and `workflows/zdev-goal.js`. Adapt the strict
   one-task cycle inline and add the outer fresh-preflight loop. First split the
   canonical `zdev-implement.js` no-work parser into branch-independent closed
   and fully gated open forms; apply the same parser to both loop aliases and
   regenerate their fixtures. Validate that the existing plugin
   workflow-directory manifest discovers both. Focused tests cover alias
   equivalence, ready-to-ready, ready-to-stop, stale advisory, REWORK, commit
   failure, and representative strict failures for missing evidence,
   mismatched area/task, suffixed first lines, and malformed nested status or
   goal. Add direct closed empty/exhausted cases off-branch and on detached HEAD
   proving no branch status, advisory, Git baseline, or worker, plus an unsafe
   open empty/exhausted rejection. Add replay cases for a partially edited
   implementer, repair with changed task, changed post-verification diff, and
   already-committed completion. Exercise the strict reconciliation parser and
   call ordering with live and cached completion fixtures, a valid current
   task/commit pair, and failures for a missing/reset/replaced commit, changed
   HEAD, reopened task, subject/change-ID/result mismatch, wrong area, and
   changed unrelated baseline. Cover an approved untracked file by exact blob,
   a completion-only file, a completion-time byte change to an approved file,
   unexpected task-file or `TASKS.md` bytes, and an unrelated staged,
   unstaged, or untracked baseline entry entering the commit. Assert exact
   accepted delta-map equality and that every mismatch leaves the pair count
   unchanged and cannot select it as new work. These tests inspect workflow
   behavior without executing Claude or introducing a workflow simulator.
3. Add the paired Codex skills and Oh My Pi prompts using their native goal
   mechanisms. Test unfinished-goal preservation, native unavailability's
   bounded `CONTINUE`, both aliases, and the common stop envelopes as artifact
   contracts.

Across those tasks, install and check must render identical bytes, validate
all artifacts before publication, preserve unrelated shared-root files, and
regenerate checked-in fixtures through zdev. They must prove that both names
select identical behavior and canonical `zdev-loop` envelopes, with no
canonical or generated Claude guidance retaining an inspect/create/apply
`/goal` claim. Codex and Oh My Pi must still preserve unfinished goals before
native application. Acceptance also requires the exact paths and invocations
above, explicit-only activation, every stop-matrix row, fresh evidence before
and after each commit, one verifier and commit boundary per task, no
replacement of unfinished native goals, and honest OpenCode/Pi bounded
fallbacks. Every restartable mutating Claude agent must fail closed before its
first mutation unless its own fresh evidence passes the replay-safety contract;
outer or replayed preflight evidence alone is insufficient. Closed no-work
must pass off-branch and detached without branch or advisory fields, while open
no-work continues to require `task_work.safe` and complete Git evidence. Every
completion PASS, including one replayed on resume, must take the unconditional
post-completion reconciliation path; only an exact reconciliation PASS may add
its task and full commit to the public trace. The accepted commit must be the
sole child of the frozen base and current HEAD. Its exact mode/blob delta must
equal the frozen verifier-approved task delta plus independently generated
`zdev task done` changes to only the task file and `TASKS.md`; the completion
report is never an allowlist. The done record/result and change identity must
match, the area must remain the same and open, and current Git evidence must
preserve every unrelated pre-task baseline entry outside the commit. Any path
or byte mismatch returns the fail-closed envelope, does not count or retry the
task, and does not start the next worker.

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

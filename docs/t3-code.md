# Coordinate zdev in T3 Code

Use T3 Code to organize zdev workers across Codex and Claude Code. The main
thread can run either harness; zdev retains task selection, branch checks,
verification, and commits while T3 owns worker execution and completion delivery.

## Install and select

Install the updated skill for whichever harness runs the T3 main thread:

```sh
zdev skill install codex --force
zdev skill install claude --force
```

Install both if you use both as coordinators. User scope is the default. Existing
project-scope and explicit-destination options work as usual. For Claude, enable
the installed zdev plugin as you do for its native integration. T3 provides its
own authenticated orchestration MCP tools to supported provider sessions.

Invoke the root zdev skill and select T3 explicitly:

> Use zdev with T3 orchestration to implement the next task in area scheduling,
> using Codex for implementation and Claude Code for verification.

To continue through the area's ready work, use T3's composer `/goal` command.
Check `/goal` first if this thread might already have an unfinished goal;
setting a new condition replaces it. Generate the condition with
`zdev goal scheduling --format json`, adding `--focus <text>` when needed.
Prefix its `native_goal` with "Use zdev with T3 orchestration." and supply that
to `/goal`. Retain worker choices in the conversation. The installed skill
prepares this condition and uses exposed native goal tools when available;
otherwise it returns the ready-to-send command before starting work.

“Goal” and “loop” mean the same operation. Optional focus text guides selection
from the complete ready frontier. The route also supports explicit verification,
plan-only work, and audits. Ordinary zdev requests retain native orchestration.
T3 tools being available does not change the default.

Use the natural-language request through the root skill. Claude's packaged
`zdev-loop` and `zdev-goal` workflow commands still execute their native scripts;
adding “T3” to their arguments does not select this route.

## Workers and profiles

An omitted worker target uses the main thread's inherited T3 provider instance.
A general implementation target covers routine, standard, and advanced work;
planner targeting follows the advanced implementer unless separately chosen.
Several available instances of a requested harness require an account choice.

Zdev resolves each role's existing profile against that worker's harness, with
the same area, run, and one-off profile precedence. The selected instance, model,
and effort form the frozen run baseline. One-off choices last through retries
and replacements of their logical step, then expire. Models and effort options
must match T3's live catalog; missing or ambiguous settings report a blocker.
Choose a supported concrete model in the request or update the existing zdev
profile when needed.
There is no separate T3 worker-profile namespace.

T3's role tags describe the work. Zdev's supplied role contracts define the
actual worker responsibilities and result fields. Workers receive task and
snapshot locators rather than the main thread's conversation. The coordinator
keeps lifecycle, staging, and commits; every verification round is a fresh child.

## Goal and loop

One visible parent goal continues the T3 coordinator. It selects a task and
delegates its first worker. For long steps, it yields while T3 runs the child.
Native goal continuations and child-completion wakes enter the same coordinator,
which checks its pending handle and phase before dispatch. An active worker
stays active; another goal turn never creates a duplicate worker. A completed
result is validated before implementation, verification, or rework follows.
After a verifier pass and snapshot comparison, the coordinator completes and
commits exactly one task.

It then reads fresh context and dispatches the next ready task's first worker
before yielding. The parent's native goal can also continue across this boundary.
T3 mirrors the provider's goal state; it does not evaluate zdev's area condition
itself. If goal continuation is lost after a committed task, the coordinator
returns `CONTINUE` with the fresh next task and the lost continuation.

The loop stops on empty/exhausted work, explicit area closure, a genuine blocker,
unsafe state, a required user decision, or the user's stop request. Queue
exhaustion leaves the area open. Repairs stay within the current task and get
fresh verification. Terminal blockers cancel owned pending work and confirm
termination before the final report and goal settlement. Unconfirmed execution
is reported as pending and preserved. A goal shown as complete says that the
run ended; the canonical zdev result still distinguishes `PASS` from `BLOCKER`. It never
substitutes for an independent verifier or snapshot comparison.

T3's Stop also stops delegated descendants. Codex Stop pauses its goal; Claude
Stop ends the turn but keeps the goal set and may check it after your next
message. The zdev route retains the stop and requires an explicit request to
resume, even after a late wake or unrelated message. To remove a stopped Claude
goal, use T3's Clear control or `/goal clear`. Unfinished changes stay preserved.
Pause, usage limits, budget limits, or unexpected goal clearing also prevent
new dispatch until execution is reconciled and you explicitly resume.

T3 documents these provider-specific controls in its
[goal guide](https://github.com/pingdotgg/t3code/blob/d720210996a514368ba99f4860063110033d93fa/docs/user/composer.md#goals).

Wait timeouts leave children running. Recovery inspects existing executions,
task records, and Git state before replacing work. Restart behavior depends on
T3's execution-recovery settings; recovered sessions still pass zdev's ordinary
admission, ownership, verification, and completion checks.

## Current limits

This integration packages coordinator instructions; it adds no server, scheduler,
or durable loop record. Codex and Claude Code are the supported coordinator and
worker targets. Delegated workers share the main thread's checkout, so this route
supports sequential work. Parallel implementation and assigned source worktrees
stop before mutation until workspace isolation is supported.

The contracts were checked against T3 nightly
`v0.0.46-nightly.20261008.2801`. Repository checks validate installation and
generated artifacts; live provider execution remains a separate qualification.
In a disposable repository with authenticated providers, check:

- One task through implementation, independent verification, and one commit.
- A repair followed by a fresh verifier, and changed-state rejection.
- A two-task visible goal in both Codex and Claude coordinator threads, including
  a goal continuation while a worker is pending and duplicate completion wakes.
- Empty/exhausted success and a confirmed blocker that ends the goal while
  retaining the correct `PASS` or `BLOCKER` result.
- A timeout, Stop, Claude's next unrelated message, usage/budget limits, and
  server restart without duplicate work, unwanted resume, or stale verification.

See the [research and rollout plan](t3-code-plan.md) for the source evidence and
later integration stages.

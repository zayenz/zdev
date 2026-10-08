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

To continue through the area's ready work:

> Use zdev with T3 orchestration to loop through area scheduling, using Codex
> for implementation and Claude Code for verification.

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
and replacements of their logical step, then expire. Models and effort options must match T3's
live catalog; missing or ambiguous settings report a blocker. Choose a supported
concrete model in the request or update the existing zdev profile when needed.
There is no separate T3 worker-profile namespace.

T3's role tags describe the work. Zdev's supplied role contracts define the
actual worker responsibilities and result fields. Workers receive task and
snapshot locators rather than the main thread's conversation. The coordinator
keeps lifecycle, staging, and commits; every verification round is a fresh child.

## Goal and loop

The coordinator selects a task and delegates its first worker. For long steps,
it yields while T3 runs the child. Completion wakes the main thread, which
validates the result and dispatches implementation, verification, or rework.
After a verifier pass and snapshot comparison, the coordinator completes and
commits exactly one task.

It then reads fresh context and dispatches the next ready task's first worker
before yielding. This keeps the continuation chain alive. T3 does not itself
evaluate zdev's area condition or wake an idle coordinator just because work is
ready. If continuation is unavailable after a committed task, the coordinator
returns `CONTINUE` with the next task instead of claiming a background loop.

The loop stops on empty/exhausted work, explicit area closure, a genuine blocker,
unsafe state, a required user decision, or the user's stop request. Queue
exhaustion leaves the area open. Repairs stay within the current task and get
fresh verification. Explicit stops cancel owned child work and preserve any
unfinished changes; late completion events do not restart the run.

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
- A two-task loop in both Codex and Claude coordinator threads, with the next
  worker dispatched at each task boundary.
- A timeout, explicit stop, and server restart without duplicate implementation
  or accepting a stale verification result.

See the [research and rollout plan](t3-code-plan.md) for the source evidence and
later integration stages.

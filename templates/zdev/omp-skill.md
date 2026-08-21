---
name: zdev
description: Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use only when the user invokes zdev or $zdev; asks to work through an existing .zdev area; or unmistakably refers to zdev's stored areas or tasks.
---

# Zdev for Oh My Pi

{{shared_contract}}

## Oh My Pi orchestration

Use `/zdev-implement` for one complete task cycle, `/zdev-verify` for explicit
read-only task verification, and `/zdev-audit` for a read-only audit.

Use `zdev-routine-implementer` only for authored routine,
`zdev-implementer` for standard/default, and `zdev-advanced-implementer` for
advanced. Advanced work first uses one blocking read-only `zdev-planner`.
Always verify with a fresh `zdev-verifier`. Return ordinary rework to the
selected profile with `hub` when possible; one valid standard-work escalation
starts an advanced replacement without replanning. The coordinator retains
task completion and commits.

For an active-zdev goal or loop request, inspect `/goal show` first. If no
unfinished goal exists, use the shared area continuation condition as the
native goal; the selected task's `native_goal` remains task context and is not
the area goal. If native continuation is unavailable, complete at most one task
and report the fresh next state. Never drop, replace, or layer work over an
unfinished Oh My Pi goal.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

---
name: zdev
description: Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use only when the user invokes zdev or $zdev; asks to work through an existing .zdev area; or unmistakably refers to zdev's stored areas or tasks.
---

# Zdev for Oh My Pi

{{shared_contract}}

## Oh My Pi orchestration

Use `/zdev-implement` for one complete task cycle, `/zdev-verify` for explicit
read-only task verification, and `/zdev-audit` for a read-only audit. Use
`/zdev-loop <area>` for native area continuation; `/zdev-goal <area>` is its
exact alias.

Use `zdev-routine-implementer` only for authored routine,
`zdev-implementer` for standard/default, and `zdev-advanced-implementer` for
advanced. Advanced work first uses one blocking read-only `zdev-planner`.
Always verify with a fresh `zdev-verifier`. Return ordinary rework to the
selected profile with `hub` when possible; one valid standard-work escalation
starts an advanced replacement without replanning. The coordinator retains
task completion and commits.

For an active-zdev goal or loop request, use either paired prompt. It calls the
model-facing `goal` tool with `op: "get"` before repository work, never drops,
replaces, or layers over an unfinished goal, and calls `op: "create"` with the
shared condition only when native goal state is clear. Native unavailability falls back to at most one verified committed task
and returns canonical `CONTINUE zdev-loop <area>` only when fresh ready work
remains.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

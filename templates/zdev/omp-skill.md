---
name: zdev
description: Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use when the user invokes zdev or $zdev, names an existing .zdev area or task, or asks to continue stored zdev work.
---

# Zdev for Oh My Pi

{{shared_contract}}

## Oh My Pi orchestration

Use `/zdev-implement` for one complete task cycle, `/zdev-verify` for explicit
read-only task verification, and `/zdev-audit` for a read-only audit. Use
`/zdev-loop <area>` for native area continuation; `/zdev-goal <area>` is its
exact alias.

Route authored routine, standard/default, and advanced work to
`zdev-routine-implementer`, `zdev-implementer`, or
`zdev-advanced-implementer`. Advanced work first uses one blocking read-only
`zdev-planner`.
Always verify with a fresh `zdev-verifier`. Return ordinary rework to the
selected profile with `hub` when possible; one valid standard-work escalation
starts an advanced replacement without replanning. The coordinator retains
task completion and commits.

Each agent starts with its role definition. Give it the complete rendered
task-workflow contract and a compact task payload: file paths for the brief,
task, guidance, and relevant source; the applicable snapshot IDs; and the
short result from the preceding role. Let the agent read those files instead
of copying their contents into the prompt.

For an active-zdev goal or loop request, use either paired prompt. It calls the
model-facing `goal` tool with `op: "get"` before repository work, never drops,
replaces, or layers over an unfinished goal, and calls `op: "create"` with the
shared condition only when native goal state is clear. Native unavailability falls back to at most one verified committed task
and returns canonical `CONTINUE zdev-loop <area>` only when fresh ready work
remains.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

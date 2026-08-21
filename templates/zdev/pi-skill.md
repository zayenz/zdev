---
name: zdev-pi
description: Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use only when the user invokes zdev or $zdev; asks to work through an existing .zdev area; or unmistakably refers to zdev's stored areas or tasks.
---

# Zdev for Pi

{{shared_contract}}

## Pi orchestration

Call `zdev_subagent` with role `routine-implementer` only for authored routine,
`implementer` for standard/default, or `advanced-implementer` for advanced.
Advanced work first uses one read-only `planner` call. Always use a fresh
`verifier` call for verification. Ordinary rework keeps the selected profile;
one valid standard-work escalation uses an advanced replacement without
replanning. Each child receives the selected brief, task, repository guidance,
work-context, and relevant source.

Use `/zdev-implement` for one complete task cycle, `/zdev-verify` for explicit
read-only task verification, and `/zdev-audit` for a read-only audit. Child Pi
processes cannot load extensions or delegate.

Stock Pi has no native continuation surface. For an active-zdev goal or loop
request, complete at most one task using the ordinary route, report the fresh
next state, and state that no continuing loop was started.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

---
name: zdev-opencode
description: "Zdev manages durable software work through briefs, tasks, implementation, independent verification, and commits. Use only when the user invokes zdev or $zdev; asks to work through an existing .zdev area; or unmistakably refers to zdev's stored areas or tasks."
compatibility: opencode
---

# Zdev for OpenCode

{{shared_contract}}

## OpenCode orchestration

Route authored routine, standard/default, and advanced work to
`@zdev-routine-implementer`, `@zdev-implementer`, or
`@zdev-advanced-implementer`. Advanced work first uses one read-only
`@zdev-planner`. Always verify with a fresh `@zdev-verifier`. Ordinary rework
stays on the selected profile; one valid standard-work escalation uses an
advanced replacement without replanning. Include rendered repository guidance
and applicable instructions in every prompt.

Use `/zdev-implement` for one complete task cycle, `/zdev-verify` for explicit
read-only task verification, and `/zdev-audit` for a read-only audit.

OpenCode has no required native continuation surface. For an active-zdev goal
or loop request, complete at most one task using the ordinary route, report the
fresh next state, and state that no continuing loop was started.

{{repository_guidance}}

The [task format](references/task-format.md) defines imported task bundles.

+++
schema_version = 1
id = "improvements-026"
key = "design-continuing-area-loop"
area = "improvements"
status = "done"
blocked_by = []
+++
# Define an explicit zdev area loop across harnesses

## Outcome

Settle a portable skill and workflow contract that repeatedly selects and completes one approved task at a time while an area remains open with ready work, using native continuation only where safely available.

## Context

zdev goal intentionally projects one task. Codex, Claude Code, and Oh My Pi expose different native goal mechanisms; OpenCode and Pi do not currently expose an equivalent lifecycle. Existing zdev-implement performs exactly one task cycle.

## Boundaries

- Research and design only.
- A bare English word goal or loop must not activate zdev outside an already-active zdev context; settle an explicit skill or command route.
- Keep one-task verification, completion, and commit boundaries.
- Do not build a scheduler, daemon, cross-harness runtime, persistent duplicate queue, or automatic branch switch or rebase.
- Never replace, clear, or layer over an unfinished native goal.
- Use the lifecycle and queue vocabulary settled in docs/area-lifecycle.md.

## Done when

- [x] The contract defines exact invocation names and activation phrases without hijacking generic goal requests.
- [x] It defines the loop condition and stop matrix for ready, empty, exhausted, closed, unsafe, malformed, blocker, REWORK, user decision, and native-goal conflict.
- [x] It defines per-harness continuation through a native mechanism where supported and an honest bounded fallback elsewhere.
- [x] It specifies how each iteration obtains fresh status, goal, and Git evidence while preserving one-task commits.
- [x] It defines restart and resume behavior without new durable execution state.
- [x] It reconciles task-sized native_goal with area-level continuation.
- [x] It produces exact per-harness installation forms and follow-up implementation tasks.

## Validation

- Build a dated, linked current source matrix for all five harnesses.
- Walk through ready-to-ready, ready-to-exhausted, closed, unsafe, REWORK, and active-native-goal scenarios.
- Compare proposed prompts and workflows with templates/zdev/task-workflows.md and the strict Claude JavaScript parsers.
- Do not implement runtime behavior or run an autonomous end-to-end campaign.

## Result

Defined explicit zdev goal and loop routes with safe native continuation and honest bounded fallbacks across all five harnesses.

Validation:

- Independent design and source review passed after correcting unsupported Claude native-goal claims and the existing documentation authority gap.
- Current primary-source link checks, documentation contract, relative-link, whitespace, and git diff checks passed.

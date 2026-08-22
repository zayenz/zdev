---
description: Continue a zdev area through native goals, one independently verified task and commit at a time
---

Call `goal({ op: "get" })` to inspect native goal state. With no unfinished
goal, call `goal({ op: "create", objective: condition })` with the validated
area condition and no invented token budget. If the exact same condition is
paused, call `goal({ op: "resume" })`; if it is active, continue without
creating another goal. Never call `replace` or `drop`. After the same active
zdev goal reaches a terminal PASS, call `goal({ op: "complete" })`.

__ZDEV_NATIVE_AREA_LOOP_BODY__

Use `zdev-routine-implementer` for routine, `zdev-implementer` for
standard/default, and `zdev-advanced-implementer` for advanced implementation.
Advanced work first uses one blocking read-only `zdev-planner`. Use a fresh
blocking `zdev-verifier` for every verdict. Ordinary rework may resume the
selected profile with `hub`; a valid one-time standard escalation starts an
advanced replacement without replanning. The current Oh My Pi session remains
the coordinator.

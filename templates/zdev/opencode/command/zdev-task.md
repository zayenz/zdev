---
description: Implement and verify one zdev task
---

Run `zd status $ARGUMENTS --format json` and `zd next $ARGUMENTS --format json`.
Confirm the four area gates, then read the brief, selected task, and repository
guidance. Record status with untracked files, the staged diff, and the unstaged
diff. Establish ownership for every overlapping path.

Give the task, baseline, and task-owned paths to `@zdev-implementer`. Inspect
the resulting changes, then ask a different `@zdev-verifier` to check every
task requirement, inspect the touched code, run required validation, and
compare Git state before and after validation. Return each task-owned `REWORK`
finding to an implementer and verify the correction with a different agent.
Continue until `PASS` or `BLOCKER`.

Review commits made after the baseline. The task-addition-only exception is
defined in `implement.md`. Leave `zd task done` and `zd commit` to the
coordinating agent.

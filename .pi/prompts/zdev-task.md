---
description: Implement and verify one zdev task
---

Run `zdev status $ARGUMENTS --format json` and select the next ready task. Confirm
the four area gates, then read the brief, task, repository guidance, and relevant
source. Record status with untracked files, the staged diff, and the unstaged
diff. Establish ownership for every overlapping path.

Call `zdev_subagent` with role `implementer`, the task, baseline, and task-owned
paths. Inspect the resulting changes, then use a different verifier to check
every task requirement, inspect the touched code, run required validation, and
compare Git state before and after validation. Return each task-owned `REWORK`
finding to an implementer and verify the correction with a different agent.
Continue until `PASS` or `BLOCKER`.

Review commits made after the baseline. The task-addition-only exception is
defined in `implement.md`. Leave `zdev task done` and `zdev commit` to the
coordinating agent.

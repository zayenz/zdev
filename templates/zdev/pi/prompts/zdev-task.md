---
description: Run one bounded zdev implementation and independent verification cycle
---

Orient with `zd status $ARGUMENTS --format json`, require the four area gates,
select the next ready task, and read its task file, area brief, repository
guidance, and relevant source. Record the three-part Git baseline: status
including untracked files, the staged diff, and the unstaged diff. Stop on
ambiguous overlap. Call `zdev_subagent` with role `implementer`, the baseline,
task-owned paths, and complete context. Inspect the resulting evidence, then
call a fresh verifier for separate Spec and Standards passes and a
pre/post-validation Git comparison. Treat summaries as context, not evidence.
Required validation that is unsafe or unavailable is `BLOCKER`; only optional
checks may be limitations. Send every task-owned `REWORK` to a fresh implementer
and then a fresh verifier; repeat without a fixed retry limit. Ignore an
intervening commit only when its complete diff adds new
`.zd/<area>/tasks/*.md` files, regenerates `.zd/<area>/TASKS.md`, and changes no
other path. Do not run `zd task done` or `zd commit` in this prompt.

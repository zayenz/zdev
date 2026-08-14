---
description: Run one bounded zdev implementation and verification cycle
---

Orient with `zd status $ARGUMENTS --format json` and select the next ready task
with `zd next $ARGUMENTS --format json`. Read its task file, area brief, and
repository guidance. Require the four area gates. Before delegation, record the
three-part Git baseline: status including untracked files, the staged diff, and
the unstaged diff. Stop on ambiguous overlap. Give that baseline and task-owned
paths to `@zdev-implementer`, inspect the complete resulting evidence, then
invoke a fresh `@zdev-verifier` for separate Spec and Standards passes and a
pre/post-validation Git comparison. Treat summaries as context, not evidence.
Required validation that is unsafe or unavailable is `BLOCKER`; only optional
checks may be limitations. Return `PASS`, `REWORK`, or `BLOCKER`. Send every
task-owned `REWORK` to an implementer, then use a fresh verifier; repeat without
a fixed retry limit. Ignore an intervening commit only when its complete diff
adds new `.zd/<area>/tasks/*.md` files, regenerates `.zd/<area>/TASKS.md`, and
changes no other path. The command does not run `zd task done` or `zd commit`.

---
description: Implement, independently verify, complete, and commit one ready zdev task
---

The coordinating session owns task selection, branch safety, Git ownership,
lifecycle changes, and commits. Workers never edit `.zdev`, complete tasks,
commit, delegate, or change the selected task.

Before starting an implementer or verifier, run
`zdev goal <area> --format json`. A validated closed goal is classified before
Git or task-work gates: implement returns successful no-work, while explicit
verify returns `BLOCKER zdev-verify`; neither starts a worker. For every open
goal, run `zdev status <area> --format json` and require
`branch_status.task_work.safe` to be true. When
`branch_status.task_work.stale_advisory` is true, report the advisory once and
continue without requesting a rebase. Staleness alone is not a blocker. A
false `safe` value blocks structurally unsafe branch, anchor, ancestry, linear
history, or active Git-operation state. Capture the complete Git baseline with
`git status --short --untracked-files=all`, `git diff --cached`, and `git diff`.
Keep explicit evidence for all three results, including empty results, and
inspect relevant untracked files. Stop on unexplained or overlapping changes
or any user-owned decision.

For implement, open/empty and open/exhausted are successful no-work results
after the open-work gates above and start no worker. Explicit verify requires
open/ready and returns `BLOCKER zdev-verify` without starting a verifier for
every no-work result. Invalid records, task graphs, or goal output are
blockers. For open/ready, retain the complete goal JSON
unchanged and its task ID as the subject. Before verification and every rework
handoff, rerun status, the complete Git evidence, and goal; require the same
ready task ID.

`zdev-implement <area>` gives the goal JSON, brief, task, repository guidance,
baseline, and task-owned paths to the configured `implementer`. Its internal
first line is `DONE implementer <area> <task-id>` or
`BLOCKER implementer <area> <task-id>`. Inspect the checkout,
then use a fresh configured `verifier` for every verdict. A verifier returns
exactly `PASS zdev-verify <area> <task-id>`,
`REWORK zdev-verify <area> <task-id>`, or
`BLOCKER zdev-verify <area> <task-id>` and includes exact `Area` and `Task`
fields, the stale advisory once when present, summary, validation, and located
evidence. Omit the advisory field when there is no stale advisory. Missing
output, a mismatched subject, a suffixed first line, or any other first line is
a blocker.

Every concrete task-owned `REWORK` goes to the same implementer when the
harness can resume it, or a replacement implementer with the unchanged goal,
baseline, current checkout, and full findings. There is no fixed rework count.
After each correction, a fresh verifier checks the whole task again. Stop only
on `PASS`, a genuine blocker, unsafe scope expansion, or a required user-owned
decision.

Only after the exact matching `PASS zdev-verify` envelope, the coordinator runs
`zdev task done`, stages only the attributed task-owned files and exact
generated task records, inspects the staged diff, and runs `zdev commit`.
Completion or commit failure is a blocker that preserves and reports the exact
state. Public output begins with
`PASS zdev-implement <area> <task-id>` or
`BLOCKER zdev-implement <area> <task-id>`; its body repeats the exact area and
task, reports the stale advisory once when present, and names summary, changed
files, validation, verifier evidence, and commit ID on pass, or the failed
stage, reason, and preserved state on blocker. It omits the advisory field when
no stale advisory was observed.

`zdev-verify <area> <task-id>` performs the same read-only preflight and requires
the explicit ID to equal the current ready goal task before starting one fresh
configured verifier. It never invokes an implementer, changes lifecycle state,
stages, or commits. Its public result is the verifier envelope above. Empty,
exhausted, or closed goals, a different ready task, unsafe state, unavailable
independent verification, or an invalid worker envelope returns `BLOCKER zdev-verify`
without mutation.

Use `$ARGUMENTS` as the area. The current Pi session is the coordinator. After
preflight, call `zdev_subagent` with role `implementer`, the unchanged goal JSON,
and baseline. Use a fresh call with role `verifier` for every full verification.
Pi children have no resumed session, so each rework uses a replacement
implementer with the complete current context and findings.

<!-- zdev:generated-repository-guidance:start -->
## Repository guidance discovery

Before planning or changing code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.
<!-- zdev:generated-repository-guidance:end -->

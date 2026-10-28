# Workflow

Zdev has one normal loop:

```text
next task
  → implementation sub-agent
  → independent review and verification sub-agent
  → rework and recheck when needed
  → mark done
  → commit with a stable change ID
  → next task
```

The harness coordinates execution. Zdev stores the area brief, task files,
generated task index, and branch metadata. Git stores accepted source
history and any rebase in progress.

The [user guide](user-guide.md) contains the complete installation and first-run
path. This reference explains what the workflow means and how to recover when
it stops.

## What zdev records

Your coding harness implements and verifies the work. Zdev records durable task
files, dependency selection, branch relationships, and stable Git change IDs.

Harness integrations render the same task contract and repository guidance in
their native formats. During implementation, source and tests may change while
`.zdev` remains unchanged. Verification runs in a fresh, read-only context. After
both specification and repository-standards checks pass, update the task and
commit the accepted files.

## Coding and review guidance

The [Coding guidelines](coding-guidelines.md) introduce the common
expectations. [Review and verification](review-and-verification.md) explains
the agent requests and their stopping points.

Installed integrations share one [coding guide](../templates/zdev/references/coding.md).
Planners describe responsibilities and changed contracts in the existing plan;
implementers reassess those boundaries and check names, effects, structure, and
completion evidence. The guide keeps design proportional to the change and
preserves the brief's testing level. It requires neither a fixed implementation
order nor a report for every identifier.

Verifiers assess the same standards alongside the task requirements. Before
substantive findings drive rework or reporting, someone other than their author
checks the locations, applicable rules, consequences, and severity against the
same code state. An independent coordinator can inspect that evidence;
otherwise a fresh verifier uses the retained execution profile. Zero findings
and mechanical validation-write bookkeeping need no additional pass. Open
questions and unavailable required evidence prevent a passing result.

## Branch orientation

An area either follows the configured project trunk or records its own Git
branch in isolated mode. With personal and project records, an area created on
the checked-out trunk without a mode flag is a trunk area under the default
`workspace.branching` value `on-request`. On another branch, under `isolated`,
or with pull-request records, the area binds the checked-out branch. `--trunk`
and `--branch` always choose the mode explicitly. An independent isolated
area's effective base is project trunk; a dependent area names one parent area,
whose branch becomes its effective base.
One thread per area is the default. Several areas may share a thread, worktree,
and branch, while task ownership remains area- and task-specific.

Initialization records trunk when HEAD names a branch; configure it before
managed work when initialization occurred on detached HEAD. Isolated area
creation records a branch and an initial base anchor when available. A
trunk area stores no branch or anchor and resolves current configured trunk
dynamically.

Before the first initialization, the harness asks whether `.zdev` is a personal,
project, or pull-request record. Personal state uses the exact clone-local
`.git/info/exclude` entry `/.zdev/`. Project state remains visible to Git as
lasting shared state. Pull-request state is committed for branch review but is
removed with `zdev cleanup squash` before squash merge. This record-policy
decision is separate from the user or project scope of a harness integration.
Repositories that already have `.zdev` keep their existing treatment without
another question.

Correct an existing binding or choose trunk mode explicitly:

```text
zdev config trunk <trunk-branch>
zdev area bind <area> <area-branch>
zdev area bind <area> --trunk
```

Read `branch_status` from `zdev status <area> --format json` during orientation:

- `branch_matches` says whether the area's branch is checked out.
- `mode` distinguishes a stored isolated branch from an explicit trunk area;
  `branch` is the resolved required branch in either case.
- `fresh` says whether the current effective-base tip is in the area branch.
- `anchor_valid` says whether the durable boundary between inherited and
  area-owned commits still exists.
- `finalized` says whether that boundary has advanced to the current base tip.
- `task_work.safe` says whether ordinary task selection, implementation,
  verification, and completion may continue. `stale_advisory` distinguishes a
  valid but stale relationship from an unsafe state.

A rewritten parent can leave the old anchor valid while the link is stale.
Current-base ancestry, not anchor containment, determines freshness.
Staleness is advisory while the recorded branch is checked out, both branches
and ancestry remain inspectable, the anchor is contained, child history after
it is linear, and no Git recovery operation is active. Status reports one
explicit rebase advisory in that case. Wrong or detached branches, missing or
invalid facts, nonlinear history, and active Git operations remain blockers.
Trunk areas require resolved trunk to be checked out and inspectable, but never
require an anchor, freshness check, or rebase advisory.

For an occasional two-area stack, create the parent branch and area first.
Create the child branch from the parent, then record the link:

```text
git switch -c <parent-branch>
zdev area create <parent> --title <title> --objective <objective>
git switch -c <child-branch>
zdev area create <child> --title <title> --objective <objective>
zdev area parent <child> <parent>
```

Area dependencies model branch ancestry. `blocked_by` models task order, and
may name a full task ID from another area, such as `platform-009`; bundle keys
stay local to their bundle. A cross-area blocker does not change branch
parents, rebase order or freshness checks. A task becomes ready when each
blocker's task file in the current checkout is `done`. When every open task in
an area waits on another area, the area's queue is `blocked`, and `zdev area
close` refuses to close an area whose open tasks block tasks elsewhere.

## Managed rebases

Rebasing is the supported way to incorporate effective-base changes. Run this
on the area's branch with a clean worktree:

```text
zdev area rebase <area>
```

For an explicit trunk area, this command returns `unchanged`. The area already
uses the configured trunk branch.

Zdev uses the stored anchor as the old boundary and the current effective-base
tip as the new boundary. It never merges, changes branches, rebases another
worktree, force-pushes, resolves conflicts, or recursively updates descendants.
A fresh and finalized link is a successful no-op.

If Git stops for conflicts, resolve and stage them, then continue or abort:

```text
zdev area rebase <area> --continue
zdev area rebase <area> --abort
```

Zdev advances the anchor only after a successful rebase. If you finish with
`git rebase --continue`, run `zdev area rebase <area>` afterward so zdev can
verify the result and finalize the anchor.

For a longer chain, update one link at a time from parent to child. Parent
completion is unnecessary. Child work may continue on a stale-but-safe link;
rebase when it needs newer parent changes or approaches integration.

## Planning and task creation

An area represents one coherent objective. `brief.md` is the concise source of
truth for shared conclusions. Task-specific details belong in the task file.

When an area contains several related increments, optional slice briefs under
`.zdev/<area>/slices/` give each increment a name, objective, and boundaries:

```sh
zdev slice create <area> <key> --title <title> --objective <objective> \
  --boundary <text> --boundary <text>
zdev slice list <area>
zdev slice show <area> <key>
```

Slices have no stored status and tasks do not have to belong to one. A task may
name an existing slice in its routing frontmatter. Zdev derives per-slice
ready, blocked, and done counts from those tasks, including zero counts for an
empty slice; unsliced tasks appear only in area totals. Task list, show, next,
status, and generated index output carry the applicable slice context. The area
brief remains authoritative for shared decisions and testing.

During approved area shaping or an authorized investigation task, retain
research under `.zdev/<area>/background/` only when later tasks will reuse
readable, stable, source-backed material. Index every retained file from the
brief, and link only relevant files from each task. Do not retain transcripts,
raw tool or search dumps, repository source copies, temporary prototypes, or
lifecycle metadata. The brief remains the authoritative synthesis; background
files do not override it or a task's outcome, boundaries, and done conditions.
Standalone investigations remain report-only unless the user asks to preserve
their result. Unfinished plans, future ideas, and collected research documents
may be saved under `.zdev/_notes/` as individual files or folders, including
nested folders. Notes need no metadata or index and remain outside the task
queue. The skill routes add note, list notes, and make area from note through
ordinary filesystem tools and the existing area workflow. See
[Notes for future work](user-guide.md#keep-notes-for-future-work).

Review the initial task split before import. This human checkpoint determines
scope, sequencing, and dependencies. Later execution continues until it
reaches a decision that changes that boundary.

Planning, audits, research, diagnosis, and prototypes are harness methods, not
zdev state types. They refine the brief or propose tasks. Only reviewed,
agent-ready implementation work enters the task queue.

### General one-off work

Use `general` as a conventional standing area when small, unrelated
improvements do not justify a new area each time. It has no special lifecycle
rules. Run the command on the checked-out trunk to create a trunk area under
the default `workspace.branching` value `on-request`:

```sh
zdev area create general \
  --title "General work" \
  --objective "Keep concrete one-off improvements as reviewed tasks."
```

For an isolated area, create or switch to the desired branch first; the same
command then binds the area to that branch. Pull-request records always use
the isolated form.

Keep its brief short and reusable: shared engineering boundaries, the agreed
testing level, and repository validation. Put each one-off outcome, context,
boundaries, completion conditions, and validation in its task. Unsliced tasks are the
default; optional slices help only when several tasks share one narrower
objective.

Every area starts open. An empty or exhausted queue leaves its objective open;
after reviewing the outcome, close it explicitly with `zdev area close <area>`.
Use `zdev area reopen <area>` before importing or reopening tasks. Both
mutations use the area's ordinary task-work branch-safety gate.

Discussion may proceed directly to exact task-bundle review when the request
explicitly asks for task creation and settles the product and testing choices.
This route skips unnecessary research or a new full brief. Task import requires exact bundle approval or existing delegated authority
covering its scope. The route retains safe branch state, proportionate testing, independent
verification, recorded completion, and a commit. Zdev never creates or switches
the `general` branch. The harness asks before making that Git change unless the
user already chose it.

Creating tasks stores a valid candidate before independent challenge. A fresh
reviewer reads the returned Markdown path, and concrete revisions replace the
stored candidate. Focused corrections are checked by the same reviewer against
the prior findings and the complete revised document. Material changes to
scope, boundaries, dependencies, task splitting, or testing strategy receive a
fresh full challenge. Present the final challenged document for approval unless
the user already delegated task creation and import within its scope. Storage
and independent challenge alone do not grant that authority.

Zdev must be active before intent routing. A `zdev` or `$zdev` cue, a
request to work through an existing `.zdev` area, or an unmistakable reference to
stored zdev work activates it; the mere presence of `.zdev` does not. Generic
words such as “audit,” “explore,” or “discuss” also do not activate zdev. Once
active, the harness selects one direct interaction:

- **Explore an objective** builds or revises the area brief
  (`wayfind` and `shape` are aliases).
- **Discuss the brief** identifies consequential choices, tests them
  in breadth-first rounds, and records decisions (`grill` is an alias).
- **Improve**, **Investigate**, **Create tasks**, **Implement**, and **Verify**
  remain separate actions.

Every interaction uses the same question policy. The harness asks one focused
question when the answer determines the next step, batches only independent
questions, recommends an answer with its trade-off, and treats no answer as no
consent. It reuses choices already stated for the work. Discussion and read-only
orientation begin in the current checkout. Implementation reuses an explicit
workspace choice. Otherwise the harness reads `workspace.branching`: under the
default `on-request`, it implements in the current checkout and creates a branch
or worktree only when you ask or the route needs one, as parallel batches,
rolling research, and an assigned source worktree do. Under `isolated`, it
creates a worktree as its authoritative checkout and reports that location. A
branch switch in the existing checkout still needs a user choice when it was
not already authorized.

An explicitly chosen assigned-worktree implementation keeps this admitted
checkout as the destination. It stores context for the exact task, gives the
worker an ordinary linked worktree at that baseline, and transports only an
inspected source commit back through a no-commit three-way Git operation.
Destination drift, deletion, binary data, and executable modes use Git's normal
merge semantics. Verification snapshots, completion records, staging, and the
final zdev commit remain destination-only.

An explicit **Parallel** interaction may overlap one finite, user-approved
batch of compatible ready tasks in separate assigned worktrees. The user also
chooses the destination, total delegated-role limit, and cleanup preference.
Codex, Claude Code, and Pi support this route. OpenCode checks its visible Task
surface before mutation, then offers sequential Implement because current
foreground batches do not return coordinator control per completed child.
Oh My Pi supports parallel work when its live background-worker capabilities
qualify. T3 orchestration currently supports sequential work only.
Parallel workers may overlap, but destination integration,
independent verification, task completion, and zdev commits remain serial.

After each interaction, zdev reports its result and relevant next actions. A
single user message may explicitly order several interactions. For example,
approving the exact task bundle and asking for implementation requests import
followed by the next ready task, provided the import and implementation gates
pass. Discussion remains the normal optional checkpoint before drafting tasks.
Creating tasks still requires an explicit request, and importing the split
still requires approval of the exact displayed bundle.

## Implementation

Run `zdev work-context <area> --format json` before dispatch. For open work,
require matching nested status and goal projections and
`branch_status.task_work.safe`; retain its HEAD and exact Git evidence. Report a
stale-but-safe rebase advisory once and continue without asking for rebase
consent. Stop on an unsafe branch, anchor, ancestry, history, or Git-operation
state. Use the managed rebase flow when the task needs newer base changes or
reaches an integration boundary, then rerun work-context.

For trunk mode, use dynamically resolved `project.trunk`, require it to be
checked out and safe, and skip freshness and rebase steps. Sharing trunk never
grants ownership of another area or unrelated changes.

The implementation agent receives the brief, one task, and relevant repository
context. It reads the brief first, then selectively loads task-relevant sources.
It edits source and tests but does not change zdev state or commit.

Inspect the diff before verification. Unrelated changes remain outside the
task.

New task-only commits are expected and do not interrupt the selected task. The
coordinator considers those tasks after it finishes the selected task and runs
`zdev next` again. Review any intervening commit that changes an existing task,
the brief, area metadata, lifecycle state, or source.

## Verification

A fresh agent reads the brief first, then the task and its relevant sources. It
performs separate Spec and Standards passes. The Spec pass checks every done
condition, boundary, and area decision. The Standards pass checks repository
conventions, maintainability, unrelated changes, and risks at touched
interfaces. It runs the task's validation and returns either a pass on both
axes or concrete, classified findings.

Verification includes code review of design, naming, and readability against
the shared coding guidelines. Passing tests alone does not establish acceptance.

Independence comes from the fresh, read-only context, not from storing an
execution transcript.

## Rework

Required corrections have a location, consequence, and proportionate
remedy; optional suggestions remain feedback and never block acceptance.

Use one full review, one correction pass, and one focused recheck. The recheck
covers accepted corrections, affected contracts, and repair regressions while
retaining conclusions for unchanged code. If required corrections remain, keep
the task open and preserve the work. Report a blocker requiring an explicit user
continuation decision. Recovery and goal/loop continuation retain this stage.
The installed coding reference links the authoritative review procedure and
conditionally loaded language and framework guidance.

## Completion and commit

Collect fresh work-context again before completion. `zdev task done` permits a
stale-but-safe relationship with one advisory, and refuses unsafe branch,
anchor, ancestry, history, or Git-operation state.

After a pass, run `zdev task done`; it updates the task file and regenerates
`TASKS.md`. Stage the intended source and area files, then run `zdev commit`. The
commit command adds a stable `Zdev-Change-Id` trailer.

Task completion records a short result and validation summary. It does not
copy the prompt, response, diff, commit hash, or agent tree into another store.

An active authorized task may need a routine technical clarification after
implementation begins. Coordination may edit the active task or brief when a
necessary helper or path conflicts only with a planner-written restriction. It
checks the agreed outcome, explicit constraints, compatibility promises,
original Git baseline, and ownership; explains the adjustment; captures fresh
work-context; and resumes the same task. Verification uses the updated records
and the original baseline, and completion includes the clarified records in the
scoped commit. Pending reviewed bundles still require exact renewed approval
after revision.

## Recovery

To resume interrupted work, inspect the task, checkout, and Git history:

- the task says whether work is open or done;
- the working tree contains unfinished edits;
- Git contains committed work;
- `zdev status` reports branch, effective-base, anchor, and finalization state;
  and
- the stable change ID finds a logical change after a rebase.

If Git has a rebase in progress, use `zdev area rebase <area> --continue` or
`--abort`. Otherwise rerun `zdev area rebase <area>` to finalize a manually
completed rebase. Refresh a stale-but-safe link when work needs current base
changes or approaches integration; ordinary task work can resume without it.
There is no separate zdev execution record to recover.

## One local hypothesis

For a recorded ready hypothesis, configure named `correctness_commands` alongside
the evaluator and commit the area records. Use `zdev research experiment <area>
<task> begin --path <code-file>` before editing, then `evaluate` to run commands
and record samples and invocation costs. A starting revision may precede HEAD
when intervening commits contain only development records.

`finalize --decision keep --lesson <lesson> --verification <verifier.json>` requires
a fresh independent verifier bound to the reported candidate snapshot. It commits
accepted code and advances the area baseline. `finalize --decision discard --lesson
<lesson>` retains the candidate reference, restores owned code and commits the
measured result. Exhausted attempts can finish as `failed-to-run` or `inconclusive`.
Unrelated staged, unstaged and untracked files remain preserved.

Read the [experiment contract](../skills/zdev/references/task-format.md#complete-one-local-experiment)
for the verifier envelope and retained-code prerequisite. After interruption,
`inspect` shows the checkpoint and `recover` resumes pending steps without
repeating recorded measurements, completion or a successful commit. Unresolved
measurements remain open; recovery does not launch them again.


For isolated hypotheses, start an authorized run with separate worker and
measurement limits. Create a source branch/worktree from admitted HEAD, then use
`research experiment <area> <task> admit --source <root> --path <file>`. Its context
records the authoritative paths and required measurements. Source workers edit
and validate only there. The coordinator confirms stop with `return
--worker-stopped`, measures with `evaluate`, and applies current measured Git
transport with `integrate`. A fresh independent verifier checks the destination
snapshot before keep.

If another candidate or supporting task changes accepted code, reconcile the
stopped source and measure again against the new baseline. Changed evaluator or
workload conditions pause candidates until an actual counted run rebaseline.
Rejected isolated candidates retain their code, observations and lessons while
leaving current accepted destination code in place. Recover pending effects before
new dispatch; unknown workers/processes retain capacity, and the binary never
removes their worktrees. The installed task-format reference defines this common
candidate contract for the coding harnesses.

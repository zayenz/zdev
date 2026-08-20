# Workflow

Zdev has one normal loop:

```text
next task
  → implementation sub-agent
  → verification sub-agent
  → rework and verify again when needed
  → mark done
  → commit with a stable change ID
  → next task
```

The harness keeps live execution state. Zdev persists the area brief, task
files, generated task index, and branch metadata. Git persists accepted source
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

## Branch orientation

One area owns one Git branch. An independent area's effective base is project
trunk. A dependent area names one parent area, whose branch becomes its
effective base.

Initialization records trunk when HEAD names a branch; configure it before
managed work when initialization occurred on detached HEAD. Area creation
requires an owning branch and records an initial base anchor when its effective
base is available. Area metadata without its required `branch` field is
invalid. Correct an existing binding or establish its base anchor explicitly:

Before the first initialization, the harness asks whether `.zdev` is a personal,
project, or pull-request record. Personal state uses the exact clone-local
`.git/info/exclude` entry `/.zdev/`. Project state remains visible to Git as
lasting shared state. Pull-request state is committed for branch review but is
removed with `zdev cleanup squash` before squash merge. This record-policy
decision is separate from the user or project scope of a harness integration.
Repositories that already have `.zdev` keep their existing treatment without
another question.

```text
zdev config trunk <trunk-branch>
zdev area bind <area> <area-branch>
```

Read `branch_status` from `zdev status <area> --format json` during orientation:

- `branch_matches` says whether the area's branch is checked out.
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

For an occasional two-area stack, create the parent branch and area first.
Create the child branch from the parent, then record the link:

```text
git switch -c <parent-branch>
zdev area create <parent> --title <title> --objective <objective>
git switch -c <child-branch>
zdev area create <child> --title <title> --objective <objective>
zdev area parent <child> <parent>
```

Area dependencies model branch ancestry. `blocked_by` models task order inside
one area.

## Managed rebases

Rebasing is the supported way to incorporate effective-base changes. Run this
on the area's branch with a clean worktree:

```text
zdev area rebase <area>
```

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

Keep a large source corpus as individual files under
`.zdev/<area>/background/`. Link them from the brief, and link only relevant
sources from each task. Background documents provide detail and source context;
they do not override the brief or a task's outcome, boundaries, and done
conditions.

Review the initial task split before import. This human checkpoint determines
scope, sequencing, and dependencies. Later execution continues until it
reaches a decision that changes that boundary.

Planning, audits, research, diagnosis, and prototypes are harness methods, not
zdev state types. They refine the brief or propose tasks. Only reviewed,
agent-ready implementation work enters the task queue.

### General one-off work

Use `general` as a conventional standing area when small, unrelated
improvements do not justify a new area each time. It has no special lifecycle
rules. Create or switch to its ordinary persistent branch, then use the
existing command:

```sh
zdev area create general \
  --title "General work" \
  --objective "Keep concrete one-off improvements as reviewed tasks."
```

Keep its brief short and reusable: shared engineering boundaries, the agreed
testing level, and repository validation. Put each one-off outcome, context,
boundaries, done proof, and validation in its task. Unsliced tasks are the
default; optional slices help only when several tasks share one narrower
objective.

Every area starts open. An empty or exhausted queue leaves its objective open;
after reviewing the outcome, close it explicitly with `zdev area close <area>`.
Use `zdev area reopen <area>` before importing or reopening tasks. Both
mutations use the area's ordinary task-work branch-safety gate.

Discussion may proceed directly to exact task-bundle review when the request
settles the product and testing choices. This route skips unnecessary research
or a new full brief. It still requires explicit bundle approval, safe branch
state, proportionate testing, independent verification, recorded completion,
and a commit. Zdev never creates or switches the `general` branch.

Zdev must be active before intent routing. A `zdev` or `$zdev` cue, a
request to work through an existing `.zdev` area, or an unmistakable reference to
stored zdev work activates it; the mere presence of `.zdev` does not. Generic
words such as “audit,” “explore,” or “discuss” also do not activate zdev. Once
active, the harness selects one direct interaction:

- **Explore an objective** builds or revises the area brief
  (`wayfind` and `shape` are aliases).
- **Discuss the brief** surveys material decision branches, challenges them in
  breadth-first rounds, and updates settled synthesis (`grill` is an alias).
- **Improve**, **Investigate**, **Create tasks**, **Implement**, and **Verify**
  remain separate actions.

After each interaction, zdev reports its result and relevant next actions. A
single user message may explicitly order several interactions. For example,
approving the exact task bundle and asking for implementation requests import
followed by the next ready task, provided the import and implementation gates
pass. Discussion remains the normal optional checkpoint before drafting tasks.
Creating tasks still requires an explicit request, and importing the split
still requires approval of the exact displayed bundle.

## Implementation

Run `zdev status <area> --format json` before dispatch and require
`branch_status.task_work.safe`. Report a stale-but-safe rebase advisory once and
continue without asking for rebase consent. Stop on an unsafe branch, anchor,
ancestry, history, or Git-operation state. Use the managed rebase flow when the
task needs newer base changes or reaches an integration boundary, then rerun
`zdev next <area> --format json`.

The implementation agent receives the brief, one task, and relevant repository
context. It reads the brief first, then selectively loads task-relevant sources.
It edits source and tests but does not change zdev state or commit.

Inspect the diff before verification. Unrelated changes remain outside the
task.

New task-only commits are expected and do not interrupt the selected task. The
processor considers those tasks after it finishes the selected task and runs
`zdev next` again. Review any intervening commit that changes an existing task,
the brief, area metadata, lifecycle state, or source.

## Verification

A fresh agent reads the brief first, then the task and its relevant sources. It
performs separate Spec and Standards passes. The Spec pass checks every done
condition, boundary, and area decision. The Standards pass checks repository
conventions, maintainability, unrelated changes, and risks at touched
interfaces. It runs the task's validation and returns either a pass on both
axes or concrete, classified findings.

Independence comes from the fresh, read-only context, not from storing an
execution transcript.

## Rework

Failed verification returns to implementation with exact findings. The loop
repeats until the change passes or reaches a real blocker. There is no fixed
retry count.

## Completion and commit

Check area status again before completion. `zdev task done` permits a
stale-but-safe relationship with one advisory, and refuses unsafe branch,
anchor, ancestry, history, or Git-operation state.

After a pass, run `zdev task done`; it updates the task file and regenerates
`TASKS.md`. Stage the intended source and area files, then run `zdev commit`. The
commit command adds a stable `Zdev-Change-Id` trailer.

Task completion records a short result and validation summary. It does not
copy the prompt, response, diff, commit hash, or agent tree into another store.

## Recovery

The repository contains enough recovery state:

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
Zdev has no execution claim, abandonment, or transaction recovery protocol.

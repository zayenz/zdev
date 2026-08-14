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
`.zd` remains unchanged. Verification runs in a fresh, read-only context. After
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

Before the first initialization, the harness asks whether `.zd` is a personal,
project, or pull-request record. Personal state uses the exact clone-local
`.git/info/exclude` entry `/.zd/`. Project state remains visible to Git as
lasting shared state. Pull-request state is committed for branch review but is
removed with `zd cleanup squash` before squash merge. This record-policy
decision is separate from the user or project scope of a harness integration.
Repositories that already have `.zd` keep their existing treatment without
another question.

```text
zd config trunk <trunk-branch>
zd area bind <area> <area-branch>
```

Read `branch_status` from `zd status <area> --format json` during orientation:

- `branch_matches` says whether the area's branch is checked out.
- `fresh` says whether the current effective-base tip is in the area branch.
- `anchor_valid` says whether the durable boundary between inherited and
  area-owned commits still exists.
- `finalized` says whether that boundary has advanced to the current base tip.

A rewritten parent can leave the old anchor valid while the link is stale.
Current-base ancestry, not anchor containment, determines freshness.

For an occasional two-area stack, create the parent branch and area first.
Create the child branch from the parent, then record the link:

```text
git switch -c <parent-branch>
zd area create <parent> --title <title> --objective <objective>
git switch -c <child-branch>
zd area create <child> --title <title> --objective <objective>
zd area parent <child> <parent>
```

Area dependencies model branch ancestry. `blocked_by` models task order inside
one area.

## Managed rebases

Rebasing is the supported way to incorporate effective-base changes. Run this
on the area's branch with a clean worktree:

```text
zd area rebase <area>
```

Zdev uses the stored anchor as the old boundary and the current effective-base
tip as the new boundary. It never merges, changes branches, rebases another
worktree, force-pushes, resolves conflicts, or recursively updates descendants.
A fresh and finalized link is a successful no-op.

If Git stops for conflicts, resolve and stage them, then continue or abort:

```text
zd area rebase <area> --continue
zd area rebase <area> --abort
```

Zdev advances the anchor only after a successful rebase. If you finish with
`git rebase --continue`, run `zd area rebase <area>` afterward so zdev can
verify the result and finalize the anchor.

For a longer chain, update one link at a time from parent to child. Parent
completion is unnecessary; each link only needs to be fresh before child work
continues.

## Planning and task creation

An area represents one coherent objective. `brief.md` is the concise source of
truth for shared conclusions. Task-specific details belong in the task file.

Keep a large source corpus as individual files under
`.zd/<area>/background/`. Link them from the brief, and link only relevant
sources from each task. Background documents provide detail and provenance;
they do not override the brief or a task's outcome, boundaries, and done
conditions.

Review the initial task split before import. This human checkpoint determines
scope, sequencing, and dependencies. Later execution continues until it
reaches a decision that changes that boundary.

Planning, audits, research, diagnosis, and prototypes are harness methods, not
zdev state types. They refine the brief or propose tasks. Only reviewed,
agent-ready implementation work enters the task queue.

Zdev must be active before intent routing. A `zdev`, `zd`, or `$zdev` cue, a
request to work through an existing `.zd` area, or an unmistakable reference to
stored zdev work activates it; the mere presence of `.zd` does not. Generic
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

Run `zd status <area> --format json` before dispatch. Stop on a branch mismatch,
stale link, invalid anchor, or pending anchor finalization. Use the managed
rebase flow, then rerun `zd next <area> --format json`.

The implementation agent receives the brief, one task, and relevant repository
context. It reads the brief first, then selectively loads task-relevant sources.
It edits source and tests but does not change zdev state or commit.

Inspect the diff before verification. Unrelated changes remain outside the
task.

New task-only commits are expected and do not interrupt the selected task. The
processor considers those tasks after it finishes the selected task and runs
`zd next` again. Review any intervening commit that changes an existing task,
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

Check area status again before completion. `zd task done` refuses the wrong
branch and stale or unfinalized base relationships.

After a pass, run `zd task done`; it updates the task file and regenerates
`TASKS.md`. Stage the intended source and area files, then run `zd commit`. The
commit command adds a stable `Zdev-Change-Id` trailer.

Task completion records a short result and validation summary. It does not
copy the prompt, response, diff, commit hash, or agent tree into another store.

## Recovery

The repository contains enough recovery state:

- the task says whether work is open or done;
- the working tree contains unfinished edits;
- Git contains committed work;
- `zd status` reports branch, effective-base, anchor, and finalization state;
  and
- the stable change ID finds a logical change after a rebase.

If Git has a rebase in progress, use `zd area rebase <area> --continue` or
`--abort`. Otherwise rerun `zd area rebase <area>` to finalize a manually
completed rebase or refresh a stale link. Inspect those facts, then restart or
resume the task. Zdev has no execution claim, abandonment, or transaction
recovery protocol.

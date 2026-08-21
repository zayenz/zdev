# Workflow round-trip audit

This audit counts the fixed orchestration in zdev's implement, verify, audit,
and task-import workflows. It does not estimate latency. Task-specific file
reads, validation commands, and provider-internal model turns vary with the
work and are outside the count.

## Counting method

The counts use four units:

- **C** — a minimum sequential coordinator turn: output must be observed and a
  semantic decision made before the next step can start. A harness may split a
  turn further, but cannot safely merge two turns separated by a listed gate.
- **Z** — one external `zdev` process, whether the coordinator or verifier runs
  it.
- **G** — one explicit local Git command. These are cheap, but keeping them
  separate makes repeated evidence collection visible. Internal Git child
  processes inside one zdev command are not separate orchestration calls.
- **W** — one agent dispatch. For Claude this includes generic agents used for
  preflight and completion, not only named implementers and verifiers.

The fixed command groups are:

```text
S  = zdev status + git status + git diff --cached + git diff + zdev goal
     Z2 G3
V  = three Git reads + validation + the same three Git reads
     Z0 G6, excluding validation
F  = zdev task done + git add + git diff --cached + zdev commit
     Z2 G2
```

`S` is a point-in-time snapshot. Its status, goal, and Git evidence must agree
at the dispatch boundary. `V` is the evidence required by the installed worker
prompts; its second Git read proves that validation did not leave an
unattributed write. The fuller `verify.md` contract also requires the verifier
to run `zdev status`, but that reference is not inlined into the installed
worker prompt. The executable baseline therefore cannot count that process.
`F` keeps task completion, explicit staging, staged-diff inspection, and commit
as separate failure boundaries. `implement.md` additionally asks for full Git
status before commit, but the installed task-workflow contract explicitly names
only staging and cached-diff inspection. F counts the installed instructions.

These groups come from `templates/zdev/task-workflows.md`,
`templates/zdev/references/implement.md`, and
`templates/zdev/references/verify.md`. The command implementations confirm
that status and goal independently load task state, `task done` checks the
selected task and branch safety again under the state lock, and `zdev commit`
delegates only the final Git commit (`src/lib.rs`, `src/goal.rs`,
`src/tasks.rs`, and `src/project.rs`).

## Baseline traces

For Codex, OpenCode, Pi, and Oh My Pi, the prompt-driven coordinator runs the
command groups directly:

```text
ordinary PASS:       S -> implementer -> S -> V(PASS) -> F
explicit verify:     S -> V(verdict)
one REWORK cycle:    S -> implementer -> S -> V(REWORK)
                     -> S -> implementer -> S -> V(PASS) -> F
small audit:         checking verifier
reviewed import:     tasks review -> user approval -> tasks import --commit
                     -> check -> tasks list
```

OpenCode and Oh My Pi may resume the implementer for rework. Codex may use a
follow-up. Pi starts a replacement process. That changes context reuse, not the
number of handoffs. Every post-rework verdict still comes from a fresh
verifier.

Claude's executable JavaScript has the same command trace, but shell work is
itself delegated because the workflow API exposes `agent()` and `pipeline()`,
not a direct shell primitive:

```text
ordinary PASS:       agent(S) -> named implementer -> agent(S)
                     -> named verifier(V) -> agent(F)
explicit verify:     agent(S) -> named verifier(V)
one REWORK cycle:    agent(S) -> named implementer -> agent(S)
                     -> named verifier(V: REWORK) -> agent(S)
                     -> named implementer -> agent(S)
                     -> named verifier(V: PASS) -> agent(F)
small audit:         named reviewer -> different named evidence vetter
```

The Claude completion agent receives the original preflight envelope,
implementer text, and verifier PASS. It is told to recheck that supplied
envelope and Git ownership, run `task done`, stage attributed paths, inspect
the cached diff, and commit. It is not explicitly told to collect another S,
and the script does not pass the latest `current.raw`. The counts therefore do
not invent a completion snapshot. `task done` still checks current task and
branch safety, but current three-part Git ownership evidence is underspecified
at this last handoff. That is an integration-contract gap, not a round-trip
saving.

The JavaScript parsers and routing gates are deterministic, so C is a semantic
workflow lower bound rather than a claim about Claude's internal inference
count. W is directly observable from the `agent()` calls.

| Harness | Ordinary PASS C/Z/G/W | Explicit verify C/Z/G/W | One REWORK C/Z/G/W | Small audit C/Z/G/W | Reviewed import C/Z/G/W |
| --- | --- | --- | --- | --- | --- |
| Codex | 5 / 6 / 14 / 2 | 2 / 2 / 9 / 1 | 7 / 10 / 26 / 4 | 2 / 0 / 0 / 1 | 4 / 4 / 0 / 0 |
| Claude | 5 / 6 / 14 / 5 | 2 / 2 / 9 / 2 | 7 / 10 / 26 / 9 | 2 / 0 / 0 / 2 | 4 / 4 / 0 / 0 |
| OpenCode | 5 / 6 / 14 / 2 | 2 / 2 / 9 / 1 | 7 / 10 / 26 / 4 | 2 / 0 / 0 / 1 | 4 / 4 / 0 / 0 |
| Pi | 5 / 6 / 14 / 2 | 2 / 2 / 9 / 1 | 7 / 10 / 26 / 4 | 2 / 0 / 0 / 1 | 4 / 4 / 0 / 0 |
| Oh My Pi | 5 / 6 / 14 / 2 | 2 / 2 / 9 / 1 | 7 / 10 / 26 / 4 | 2 / 0 / 0 / 1 | 4 / 4 / 0 / 0 |

The five ordinary PASS coordinator turns are: admit the initial snapshot,
accept implementation plus the refreshed snapshot, accept verifier PASS,
inspect completion and staged evidence, and accept the commit result. A REWORK
adds one turn to accept findings and dispatch correction after a fresh
snapshot, and one to accept the correction and dispatch a fresh verifier after
another snapshot. Import's four turns are review and approval request, import,
check, and list/report; the user's approval is an additional user turn, not C.

An audit has no fixed zdev or Git command. Its file and search calls depend on
the requested boundary. A multi-lens audit uses one worker per lens and one
different final vetter, so its W count is `lenses + 1` in every harness.

## Which repetition is load-bearing

| Repeated work | Classification | Reason |
| --- | --- | --- |
| Status, goal, and Git evidence before each implementation, verification, or rework handoff | **Required, but combinable** | The checkout and selected task may have changed. One fresh read-only command can collect the same point-in-time evidence without caching it. |
| Verifier's own zdev status | **Required by `verify.md`, missing from installed worker prompts** | Independent verification should inspect branch safety itself. The current prompts require Git state but do not explicitly invoke zdev status. A shared context command can close the gap. |
| Verifier's own pre-validation Git evidence | **Required, but combinable** | Independent verification must inspect the checkout itself. Coordinator evidence is context, not proof. |
| Verifier's post-validation Git evidence | **Required** | It detects generated or otherwise unexpected validation writes. It cannot be reused from before validation. |
| Goal task ID at every refreshed handoff | **Required** | It prevents a long-running workflow from acting on a newly selected task. |
| Brief, task, baseline, and prior findings passed to a replacement implementer | **Safely reusable** | These are context, provided refreshed status and goal still name the same ready task and authoritative files have not changed. |
| Stale advisory text after a safe status result | **Safely reusable within the run** | Staleness does not add a command. Claude already accumulates the advisory and emits it once. Every later safety gate still runs. |
| Implementer summary passed to the verifier | **Safely reusable only as a locator** | The verifier must open and check the cited evidence rather than trust the summary. |
| Envelope parsing after every worker | **Required** | Missing, malformed, or mismatched subjects fail closed. Sharing parser code could reduce source duplication, but would save no round trip. |
| `task done`, staged-diff inspection, and commit | **Required as separate gates** | Completion changes durable task state; explicit staging establishes ownership; inspection authorizes the exact commit. Failure must leave inspectable state. |
| Bundle parsing at review and import | **Required** | Import must fingerprint the bytes supplied after approval. Reusing an in-memory review would make approval session-dependent. |
| `check` after import | **Required under the current contract** | It checks the published area beyond the returned task IDs. Removing it would need equivalent pre-commit validation and more complicated rollback. |
| `tasks list` after successful import | **Redundant presentation** | Import already has the validated hypothetical graph and allocated IDs. It can return the ready frontier directly. |
| A second verifier for a small Claude audit | **Redundant** | The shared audit contract permits one fresh verifier to inspect and check a small boundary. Separate final vetting remains required after fan-out. |
| Fresh Git evidence in Claude's completion agent | **Required, currently underspecified** | The agent receives earlier evidence and an attestation, not the latest structured snapshot. Closing this gap may add a call; it must not be counted as an existing call or optimized away. |

## Representative failure traces

The clean path is the ordinary PASS trace above. A stale-but-safe path has the
same counts: `status.branch_status.task_work.safe` remains true, the advisory
is retained once, and no rebase or extra read is inserted. The black-box
`stale_independent_base_is_advisory_for_task_work` test exercises status,
selection, and completion on that path.

One concrete REWORK follows the expanded trace above. It adds `Z4 G12 W2` to a
prompt-driven PASS path, or `Z4 G12 W4` to Claude's path. Both added snapshots
are necessary: one precedes correction and one follows writes before the new
verifier starts.

An invalid verifier envelope follows:

```text
S -> implementer -> S -> V(invalid subject or fields) -> BLOCKER
```

It stops at `C3 Z4 G12 W2` in the prompt-driven harnesses and W4 in Claude.
There is no completion, staging, retry, or interpretation as success. Claude's
`parseReady`, `exactWorkerEnvelope`, and final field checks are exercised by
`claude_task_workflows_reject_incomplete_or_mismatched_structured_envelopes`.

A task-work commit failure follows the full PASS trace through the final
`zdev commit`. It has the same counts as PASS, but returns `BLOCKER` with the
task marked done and the intended index preserved for inspection; zdev does
not reset or rearrange the user's index. A committed task-import failure stops
after review and import (`Z2` externally). Its internal transaction unstages
only managed paths, removes created tasks, restores `TASKS.md`, preserves the
brief's exact prior index/worktree state, and leaves unrelated staged work
alone. `failed_committed_task_import_rolls_back_planning_changes_and_preserves_index`
exercises that recovery boundary.

## Ranked reductions

### 1. Add one fresh work-context command

Add the narrow read-only command
`zdev work-context <area> --format json`. It classifies the complete goal
projection first. A closed area returns this branch-independent object and
does not collect status or Git evidence:

```json
{
  "area": "<area>",
  "goal": {"<complete goal projection>": "<nested JSON value>"},
  "lifecycle": "closed",
  "queue": "empty|exhausted",
  "schema_version": 1,
  "task_id": null
}
```

An open area then obtains status and returns exactly these keys in stable
order:

```json
{
  "area": "<area>",
  "git_diff": "<complete stdout, possibly empty>",
  "git_diff_cached": "<complete stdout, possibly empty>",
  "git_status": "<complete stdout, possibly empty>",
  "goal": {"<complete goal projection>": "<nested JSON value>"},
  "lifecycle": "open",
  "queue": "empty|ready|exhausted",
  "schema_version": 1,
  "stale_advisory": false,
  "status": {"<complete status projection>": "<nested JSON value>"},
  "task_id": null
}
```

`task_id` is a JSON string for `ready` and JSON null otherwise. The nested
values are the existing projections, not JSON encoded inside strings. Open
work fails closed unless area, lifecycle, queue, task ID, and branch safety
agree exactly. Ready work requires `task_work.safe == true`,
`status.next == goal.task.id == task_id`, and a boolean `stale_advisory`.
Open no-work also requires safe task work and both task projections to be null.
Invalid or blocked task graphs remain errors. Closed validation still checks
the task records and lifecycle through goal, but deliberately does not require
a checkout, branch status, or Git cleanliness.

Collect goal, status, and the three Git results freshly and sequentially in
one zdev process. This is not an atomic filesystem snapshot: its value is that
one command owns a complete fail-closed collection, not that concurrent writes
become impossible. It must not cache or persist any result. Run Git directly
with process argv—`git status --short --untracked-files=all`, `git diff
--cached`, and `git diff`—without a shell. A spawn failure, nonzero exit,
non-UTF-8 output, or status/goal/Git parsing failure fails the whole command
without partial JSON. Successful empty Git stdout is the explicit empty
string. Existing `status` and `goal` output remain unchanged.

Use it for coordinator snapshots. Require the verifier to invoke it
independently for its pre-validation snapshot, closing the installed-prompt
status gap; the verifier still runs the three Git reads after validation. In
Claude, also require the completion agent to invoke it immediately before
completion and use that fresh envelope instead of the original `prepared.raw`.
This closes the observed completion gap but consumes one of Claude's process
savings.

```text
before PASS: S -> implementer -> S -> V -> F       C5 Z6 G14 W2
after PASS:  K -> implementer -> K -> VK -> F      C5 Z5 G5  W2

before verify: S -> V                              C2 Z2 G9 W1
after verify:  K -> VK                             C2 Z2 G3 W1

before one REWORK: S -> I -> S -> V -> S -> I -> S -> V -> F
                                                    C7 Z10 G26 W4
after one REWORK:  K -> I -> K -> VK -> K -> I -> K -> VK -> F
                                                    C7 Z8  G8  W4
```

Here K is one work-context process and VK is K plus validation and the required
three post-validation Git reads. These exact traces apply to Codex, OpenCode,
Pi, and Oh My Pi. Their savings are `1Z + 9G` for ordinary PASS, `0Z + 6G` for
explicit verify, and `2Z + 18G` for one REWORK cycle.

Claude keeps its current W counts because its workflow still needs agents to
run each command. With the added completion K, Claude's after counts are
`C5 Z6 G5 W5` for PASS and `C7 Z9 G8 W9` for one REWORK; explicit verify is
`C2 Z2 G3 W2`. It saves `0Z + 9G` for PASS, `0Z + 6G` for verify, and
`1Z + 18G` for one REWORK while adding the missing freshness gate.

Implementation size is medium: one command, the exact schema above, shared use
of existing status/goal renderers, template updates, generated integrations,
and focused black-box and parser coverage. Risk is medium because snapshot
completeness, subprocess errors, exact empty output, and byte preservation are
safety properties.

### 2. Use one checking verifier for a small Claude audit

When `lenses` is absent or empty, Claude can call one fresh verifier with the
public audit contract and validate its final envelope directly. Keep the
current review pipeline plus different final vetter whenever explicit lenses
are supplied.

```text
before small Claude audit: reviewer -> evidence vetter    W2
after small Claude audit:  checking verifier              W1
fan-out before and after:  lens workers -> fresh vetter   W(lenses + 1)
```

This saves one expensive worker handoff for the common small audit. It affects
Claude only, is a small JavaScript/template change, and has low risk because it
implements an option already stated in the shared contract. Focused coverage
should prove the one-worker empty-lens path, the unchanged fan-out path, and
fail-closed envelope validation.

### 3. Return the ready frontier from task import

Add a deterministic `ready` task-ID array to successful import JSON. It is the
complete post-import area's ready frontier, not merely ready IDs from the
bundle, ordered by the existing stable numeric task order. Compute it from the
same validated hypothetical graph used to render `TASKS.md`. Keep `zdev check`
after publication; remove only the separate `tasks list` command from canonical
guidance.

```text
before: tasks review -> approval -> tasks import --commit -> check -> tasks list
        C4 Z4
after:  tasks review -> approval -> tasks import --commit -> check
        C3 Z3
```

This saves one zdev process and one coordinator turn for every import in all
harnesses. The implementation is small and low risk. Existing import output is
extended, approval remains stateless, import still rereads and fingerprints
the bundle, `check` still validates the published area, and commit rollback is
unchanged. Focused coverage should import a new task blocked by an existing
open task and require the frontier to contain that existing task. This proves
the projection is area-wide and sufficient to replace the list call. A
successful valid import cannot have an empty frontier: it adds open work to a
finite acyclic graph, so at least one task is ready. No new transaction
abstraction is needed.

## Rejected shortcuts

- Do not reuse the coordinator snapshot as the verifier's own evidence. That
  removes the independent checkout check and misses changes between dispatch
  and worker execution.
- Do not reuse a pre-write goal or Git snapshot after implementation or
  rework. Fresh task identity and ownership are the reason those gates exist.
- Do not combine `task done`, staging, staged-diff approval, and commit into an
  opaque finish command. It would hide the exact mutation boundary and make
  recovery or user-owned staged changes harder to preserve.
- Do not skip post-validation Git evidence. A passing validation command can
  still write generated files.
- Do not retry malformed worker envelopes or commit failures automatically.
  Both are fail-closed states that require inspection, not transport smoothing.
- Do not cache status, goal, branch ancestry, approval, or Git evidence across
  turns. The saved process calls are not worth hidden mutable workflow state.
- Do not remove post-import `check` merely because import validates its own
  writes. The broader published-area check is currently observable behavior.

These three follow-up changes are independent. Implement the work-context
command first because it removes the most repeated calls across the widest
surface. The Claude audit fast path and import frontier can land in either
order.

## Follow-up task split

1. **Add a fresh work-context snapshot.** Implement the read-only command and
   switch canonical implement/verify guidance plus generated integrations to
   it. Give it the exact JSON and subprocess contract above. Preserve
   standalone status/goal output, exact task and branch gates, three-part Git
   evidence, verifier post-validation evidence, and invalid-envelope blockers.
   Require independent verifier collection and a fresh Claude completion
   collection. Prove clean, empty-output, command-error, stale-safe,
   changed-focus, and validation-write behavior with focused black-box
   coverage and all-harness generation checks.
2. **Shorten small Claude audits.** Route an empty `lenses` input to one fresh
   checking verifier, while retaining reviewer-plus-vetter fan-out for explicit
   lenses. Prove both dispatch counts and the existing invalid-envelope
   blocker in the executable workflow test.
3. **Report import's ready frontier.** Return ready task IDs from the validated
   post-import area graph in stable numeric order and remove only the
   guidance's `tasks list` follow-up. Preserve approval fingerprinting,
   `check`, commit path ordering, locks, and rollback. Cover an imported task
   blocked by an existing ready task in the existing import tests.

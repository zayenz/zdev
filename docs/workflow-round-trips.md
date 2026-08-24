# Workflow round-trip audit

> **Status: implemented reductions with retained baseline.** Work-context,
> coordinator-owned compact verifier handoffs, the small Claude audit path, and the import ready
> frontier have shipped. The opening counts preserve the pre-change baseline;
> the realized counts below describe current workflows.

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

## Baseline traces before the reductions

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

This table classifies the pre-change baseline. The implemented reductions that
follow preserve the required boundaries while combining the repeated calls.

| Repeated work | Classification | Reason |
| --- | --- | --- |
| Status, goal, and Git evidence before each implementation, verification, or rework handoff | **Required, but combinable** | The checkout and selected task may have changed. One fresh read-only command can collect the same point-in-time evidence without caching it. |
| Verifier's own zdev status | **Required by `verify.md`, missing from the baseline worker prompts** | Independent verification should inspect branch safety itself. Those prompts required Git state but did not explicitly invoke zdev status. Work-context closed the gap. |
| Verifier's own pre-validation Git evidence | **Required, but combinable** | Independent verification must inspect the checkout itself. Coordinator evidence is context, not proof. |
| Verifier's post-validation Git evidence | **Required** | It detects generated or otherwise unexpected validation writes. It cannot be reused from before validation. |
| Goal task ID at every refreshed handoff | **Required** | It prevents a long-running workflow from acting on a newly selected task. |
| Brief, task, baseline, and prior findings passed to a replacement implementer | **Safely reusable** | These are context, provided refreshed status and goal still name the same ready task and authoritative files have not changed. |
| Stale advisory text after a safe status result | **Safely reusable within the run** | Staleness does not add a command. Claude already accumulates the advisory and emits it once. Every later safety gate still runs. |
| Implementer summary passed to the verifier | **Safely reusable only as a locator** | The verifier must open and check the cited evidence rather than trust the summary. |
| Envelope parsing after every worker | **Required** | Missing, malformed, or mismatched subjects fail closed. Sharing parser code could reduce source duplication, but would save no round trip. |
| `task done`, staged-diff inspection, and commit | **Required as separate gates** | Completion changes durable task state; explicit staging establishes ownership; inspection authorizes the exact commit. Failure must leave inspectable state. |
| Bundle parsing at review and import | **Required** | Review stores the canonical bundle and internal fingerprint outside tracked state. Import rereads and validates that artifact and requires its opaque review identity to remain current, so the drift check is not session-dependent. The fingerprint is not security authorization. |
| `check` after import | **Required under the current contract** | It checks the published area beyond the returned task IDs. Removing it would need equivalent pre-commit validation and more complicated rollback. |
| `tasks list` after successful import | **Redundant presentation** | Import already has the validated hypothetical graph and allocated IDs. It can return the ready frontier directly. |
| A second verifier for a small Claude audit | **Redundant** | The shared audit contract permits one fresh verifier to inspect and check a small boundary. Separate final vetting remains required after fan-out. |
| Fresh Git evidence in Claude's completion agent | **Required, underspecified at baseline** | The agent received earlier evidence and an attestation, not the latest structured snapshot. Work-context added the missing fresh evidence. |

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
`parseReady`, strict worker-result parser, and final field checks are exercised by
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

## Implemented reductions

### 1. Fresh work-context command (implemented)

The narrow read-only command is
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
  "head": "<full lowercase commit ID>",
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
work fails closed unless area, lifecycle, queue, task ID, HEAD, and branch
safety agree exactly. Ready work requires `task_work.safe == true`,
`status.next == goal.task.id == task_id`, and a boolean `stale_advisory`.
Open no-work also requires safe task work and both task projections to be null.
Invalid or blocked task graphs remain errors. Closed validation still checks
the task records and lifecycle through goal, but deliberately does not require
a checkout, branch status, or Git cleanliness.

Collect goal, status, HEAD, and the three Git results freshly and sequentially
in one zdev process. This is not an atomic filesystem snapshot: its value is
that one command owns a complete fail-closed collection, not that concurrent
writes become impossible. The ordinary command does not persist its result. Run Git
directly with process argv—`git rev-parse HEAD`, `git status --short
--untracked-files=all`, `git diff --cached`, and `git diff`—without a shell.
HEAD must be one full lowercase commit ID; preserve the other stdout strings
exactly. A spawn failure, nonzero exit, non-UTF-8 output, or status/goal/Git
parsing failure fails the whole command without partial JSON. Successful empty
Git stdout is the explicit empty string. Existing `status` and `goal` output
remain unchanged.

An optional transport stores those exact pretty-JSON stdout bytes, including
the terminal newline, under the path resolved by `git rev-parse --git-path
zdev/work-context/<area>/<snapshot>.json`:

```sh
zdev work-context <area> --store --format json
zdev work-context <area> --show <snapshot> --format json
zdev work-context <area> --compare <snapshot> --format json
```

Store returns only the area, opaque ID, path, lifecycle, queue, task ID, and
HEAD when present. Show validates and reproduces the stored document. Compare
validates it, collects fresh ordinary work-context, and returns a compact
boolean without echoing either document. It is a successful comparison when
the values differ. Missing or expired, corrupt, and cross-area files are
errors. Files are immutable and content-addressed; each area retains eight
distinct snapshots using first-publication time and filename as the stable
tie-break. There is no current pointer, history, approval, or cleanup UI.

This storage lowers handoff context, not freshness requirements. A later
decision still needs a new collection or `--compare`; no stored snapshot is
current authority. Coordinator-to-implementer handoffs still use complete
inline work-context when the worker needs it. Verifier PASS and completion use
the stored transport.

Coordinators collect ordinary inline work-context for selection, ownership,
implementation, and rework. Immediately before each verifier, coordination
stores and shows the pre-validation context, validates it against the admitted
checkout, and supplies only its locator. The verifier shows that snapshot and
runs validation. Coordination parses the four semantic response fields and
uses compact compare afterward. Completion receives only that opaque ID and
performs one more compact compare before mutation. Neither handoff carries raw
Git strings, and no worker-supplied locator or identity is trusted.

```text
before PASS: S -> implementer -> S -> V -> F              C5 Z6 G14 W2
after PASS:  K -> implementer -> K -> CS -> V -> C -> FS  C5 Z9 G2  W2

before verify: S -> V                                     C2 Z2 G9 W1
after verify:  K -> CS -> V -> C                          C2 Z5 G0 W1

before one REWORK: S -> I -> S -> V -> S -> I -> S -> V -> F
                                                    C7 Z10 G26 W4
after one REWORK:  K -> I -> K -> CS -> V -> C
                   -> K -> I -> K -> CS -> V -> C -> FS   C7 Z15 G2 W4
```

Here K is one inline work-context process, CS is coordinator snapshot store and
show, V is verifier show plus validation, C is the coordinator's compact
post-response compare, and FS is the final compact compare followed by
completion. These exact traces apply to Codex, OpenCode, Pi, and Oh My Pi.

Claude's workflow needs generic agents to run each coordinator command.
Snapshot admission and post-response comparison add explicit calls around the
named verifier. This deliberately optimizes responsibility and deterministic
resolution rather than process count.

The realized fixed counts are therefore:

| Harness route | Ordinary PASS C/Z/G/W | Explicit verify C/Z/G/W | One REWORK C/Z/G/W |
| --- | --- | --- | --- |
| Codex, OpenCode, Pi, Oh My Pi | 5 / 9 / 2 / 2 | 2 / 5 / 0 / 1 | 7 / 15 / 2 / 4 |
| Claude | 5 / 9 / 2 / 7 | 2 / 5 / 0 / 4 | 7 / 15 / 2 / 13 |

A closed or open no-work implementation stops after one K: `C1 Z1 G0 W0`
for prompt-driven harnesses and `C1 Z1 G0 W1` for Claude. Closed K performs no
status or Git inspection. External orchestration counts stay lower than the
audited baseline even though every verifier now shows coordination's snapshot
and Claude uses separate agents for snapshot admission and comparison. Its
existing completion agent performs the final comparison after PASS.

The ordinary PASS counts end at the verified commit. A one-task command does
not run an unused post-commit `next` or K. An explicit continuation or area
goal/loop pays for a fresh K only when it will use that result to decide whether
to dispatch another task.

Implementation size was medium: one command, the exact schema above, shared use
of existing status/goal renderers, template updates, generated integrations,
and focused black-box and parser coverage. Risk is medium because inline
context completeness, subprocess errors, exact empty output, and byte
preservation are safety properties.

### 2. Use one checking verifier for a small Claude audit (implemented)

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
Claude only and is a small JavaScript/template change. Focused coverage proves
the one-worker empty-lens path, the unchanged fan-out path, and fail-closed
envelope validation.

### 3. Return the ready frontier from task import (implemented)

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
harnesses. The implementation is small and low risk. Import output includes
the frontier, approval remains outside durable `.zdev` state, import still
rereads the stored canonical bundle and checks its internal fingerprint,
`check` still validates the published area, and commit rollback is
unchanged. Focused coverage imports a new task blocked by an existing open task
and requires the frontier to contain that existing task. This proves
the projection is area-wide and sufficient to replace the list call. A
successful valid import cannot have an empty frontier: it adds open work to a
finite acyclic graph, so at least one task is ready. No new transaction
abstraction is needed.

### 4. Keep only the current Claude worker handoff (implemented)

Claude's task workflow no longer accumulates every accepted implementation
envelope. Each fresh verifier receives the expected area, task, and HEAD plus
only the latest accepted implementation or rework envelope as a locator. It
receives the coordinator-stored snapshot rather than raw Git payload. After
rework, the replacement envelope supersedes the earlier one.

The completion agent receives one verifier-approved snapshot ID. It does not
receive implementation envelopes, a second copy of the verifier PASS, the
latest inline coordinator context, or raw Git evidence. Every independent
verification, post-validation comparison, completion, and commit gate remains
in place.

### 5. Route verifier bookkeeping through coordination (implemented)

Immediately before dispatch, coordination creates and validates one immutable
pre-validation work-context snapshot. The verifier uses `--show` to inspect it,
runs validation, and returns exactly verdict, summary, findings, and
escalation. Afterward coordination runs `--compare`, rejects changed-state
PASS, and constructs the compatible nine-key public result. Its generated
evidence is `work_context_snapshot: W<16-lowercase-hex>` plus the optional
stale advisory; the summary carries validation conclusions.

Completion receives that ID only. It runs exactly one fresh `--compare` before
`task done`, staging, or commit. False, missing, expired, corrupt, cross-area,
or malformed evidence blocks before mutation. Direct inline Git evidence is no
longer serialized through verifier or completion prompts.

## Rejected shortcuts

- Do not let the verifier choose or echo the snapshot identity. Coordination
  admits it at the dispatch boundary, the verifier independently resolves it,
  and coordination compares it after the response.
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

These changes landed independently. Work-context provides the shared
collection boundary; the Claude audit fast path, import frontier, and trimmed
Claude worker handoffs do not depend on one another.

## Implemented task split

1. **Fresh work-context snapshot.** The read-only command and canonical
   implement/verify guidance use the exact JSON and subprocess contract above. They preserve
   standalone status/goal output, exact task and branch gates, three-part Git
   evidence, verifier post-validation evidence, and invalid-envelope blockers.
   Independent verifier collection and fresh Claude completion collection are
   covered by focused black-box and all-harness generation checks.
2. **Shorter small Claude audits.** An empty `lenses` input routes to one fresh
   checking verifier, while explicit lenses retain reviewer-plus-vetter fan-out.
   Executable workflow tests cover both dispatch counts and invalid envelopes.
3. **Import ready frontier.** Import returns ready task IDs from the validated
   post-import area graph in stable numeric order, and canonical guidance removes only the
   guidance's `tasks list` follow-up. Preserve the opaque review-fingerprint drift check,
   `check`, commit path ordering, locks, and rollback. Existing tests cover an
   imported task blocked by an existing ready task.
4. **Trimmed Claude handoffs.** Verification receives only the latest accepted
   implementation locator. Completion receives one normalized verifier
   evidence object and no implementation payload. Executable workflow probes
   cover initial PASS, rework replacement, advanced escalation, completion,
   and invalid envelopes while retaining the existing safety gates.

# Work-context snapshots and workflow calls

`zdev work-context` collects the selected task, branch status, and Git changes
in one read-only command. Workflows can store that result for worker handoffs
and compare it with fresh state before acting.

## Work-context output

The read-only command is
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
  "git_untracked": {"<path>": {"kind": "file", "hash": "<Git blob hash>"}},
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

The command collects goal, status, HEAD, and Git changes sequentially.
Concurrent writes are still possible; this is not an atomic filesystem
snapshot. Unless `--store` is supplied, the result is not saved. Zdev runs Git
directly with process arguments—`git rev-parse HEAD`, `git status --short
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

Store returns compact routing fields: area, opaque ID, path, lifecycle, queue,
task ID, HEAD, complexity, and stale advisory when present. Show validates and
reproduces the stored document. Compare validates it, collects fresh ordinary
work-context, and returns a compact boolean without echoing either document.
It is a successful comparison when the values differ. Missing, corrupt, and
cross-area files are errors. Files are immutable and content-addressed, and
remain available for active baseline and verification handoffs. There is no
current pointer, approval, or cleanup UI.

The `git_untracked` map uses unfiltered Git blob hashes for regular files and
`{"kind": "symlink", "target": "<link target>"}` for symlinks, including dangling
links. Capturing it does not stage files or write Git objects. Older open
snapshots without this map remain readable and compare unequal to fresh state.
Untracked nested repositories record their HEAD (null before the first commit),
status, binary-capable diffs, and their own `git_untracked` map. Ignored files
remain outside the snapshot.

Stored context reduces the data passed between workers. Before a later
decision, collect fresh context or use `--compare` to check for changes.
Workers receive the locator and load the immutable context with `--show` only
when needed. Verifier PASS and completion use the stored transport.

Coordinators store work-context for selection, ownership, implementation, and
rework. Immediately before each verifier, coordination stores the
pre-validation context, validates its compact routing fields, and supplies
only its locator. The verifier shows that snapshot and runs validation.
Coordination parses the four semantic response fields and uses compact compare
afterward. Completion receives only that opaque ID and performs one more
compact compare before mutation. Neither handoff carries raw Git strings, and
no worker-supplied locator or identity is trusted.

## Workflow call counts

These counts cover fixed orchestration calls in the audited routes, excluding
task-specific reads, validation, and provider-internal turns. They are not
latency measurements.

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

The audited counts are:

| Harness route | Ordinary PASS C/Z/G/W | Explicit verify C/Z/G/W | One REWORK C/Z/G/W |
| --- | --- | --- | --- |
| Codex, OpenCode, Pi, Oh My Pi | 5 / 9 / 2 / 2 | 2 / 5 / 0 / 1 | 7 / 15 / 2 / 4 |
| Claude | 5 / 10 / 2 / 6 | 2 / 4 / 0 / 3 | 7 / 17 / 2 / 11 |

A closed or open no-work implementation stops after one work-context call: `C1
Z1 G0 W0` for prompt-driven harnesses and `C1 Z1 G0 W1` for Claude. A closed
area requires no status or Git inspection. Each verifier reads the
coordinator's snapshot. Claude uses separate agents for snapshot collection
and comparison. Its existing completion agent performs the distinct final
comparison after PASS.

The ordinary PASS counts end at the verified commit. A one-task command does
not run an unused post-commit `next` or work-context call. An explicit
continuation or area goal/loop collects fresh work-context only when it will
use that result to decide whether to dispatch another task.

## Small Claude audits

When `lenses` is absent or empty, Claude can call one fresh verifier with the
public audit contract and validate its final result directly.

A small audit uses one worker. An audit with explicit lenses uses one worker
per lens and one fresh verifier to check the combined findings.

## Ready tasks after import

Successful import JSON includes a deterministic `ready` task-ID array. It is
the complete post-import area's ready frontier, not merely ready IDs from the
bundle, ordered by the existing stable numeric task order. Zdev computes it
from the same validated graph used to render `TASKS.md`. The workflow still
runs `zdev check` after publication, but no longer needs a separate `tasks
list` call.

The workflow is review, approval, import, then `zdev check`: three zdev
processes and three coordinator turns. Approval is a separate user turn.

A successful
import cannot have an empty ready list: it adds open work to a finite acyclic
graph, so at least one task is ready. Review validation, commit rollback, and
the post-import check still apply.

## Worker handoffs

Claude's task workflow keeps only the current implementation result. Each
fresh verifier receives the expected area, task, and HEAD plus only the latest
accepted implementation or rework envelope as a locator. It receives the
coordinator-stored snapshot rather than raw Git payload. After rework, the
replacement envelope supersedes the earlier one.

The completion agent receives one verifier-approved snapshot ID. It does not
receive implementation envelopes, a second copy of the verifier PASS, the
latest inline coordinator context, or raw Git evidence. Every independent
verification, post-validation comparison, completion, and commit gate remains
in place.

## Verification snapshots

Immediately before dispatch, coordination creates and validates one immutable
pre-validation work-context snapshot. The verifier uses `--show` to inspect it,
runs validation, and returns exactly verdict, summary, findings, and
escalation. Afterward coordination runs `--compare`, rejects changed-state
PASS, and constructs the compatible nine-key public result. Its generated
evidence is `work_context_snapshot: W<16-lowercase-hex>` plus the optional
stale advisory; the summary carries validation conclusions.

Completion receives that ID only. It runs exactly one fresh `--compare` before
`task done`, staging, or commit. False, missing, corrupt, cross-area, or
malformed evidence blocks before mutation. Direct inline Git evidence is no
longer serialized through verifier or completion prompts.

## Checks that remain separate

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
  Both require inspection before work can continue.
- Do not cache status, goal, branch ancestry, approval, or Git evidence across
  turns. The saved process calls are not worth hidden mutable workflow state.
- Do not remove post-import `check` merely because import validates its own
  writes. The broader published-area check is currently observable behavior.

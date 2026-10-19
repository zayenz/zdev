# Zdev 2.0 release readiness


Repair status, 19 October 2026: R1–R8 are implemented. The historical findings
below describe the audited `97b73df` baseline. The repair also adds explicit
research allowance extension and clarifies ordinary-loop versus research goal
completion. Existing record schema and configuration remain compatible.

Verification for the repairs: seven focused Rust tests passed, including the
existing two scripted rolling scenarios with interruption at the experiment
attempt limit, mixed added/edited files during reconciliation, parallel
checkpoint/profile/advisory handling, retained-plan dispatch, task approval, and
extension retry/accounting. The slowest existing check took about two minutes;
the full suite was not run. Tests used local commands and scripted workers with
no LLM calls. Formatting and Clippy passed. The documentation check/build and
70-page internal link/anchor check passed. Generated integrations were refreshed.
Publication, release dating/version notices, and importing the final site remain
release steps; this repair does not publish a release or the hosted site.


Reviewed 19 October 2026, from `v1.6.0` through `97b73df` on local `main`:
51 commits. The initial source snapshot was `ff4744e`; the pending guidance,
standalone review, generated integration, and field-feedback changes were then
reviewed, committed as `97b73df`, and integrated with a fast-forward. Local
`main` has no merge commits and is one commit ahead of `origin/main`. Nothing
was published or pushed during this review.

I recommend fixing the recovery and acceptance defects below before releasing
2.0. The ordinary task system, coding guidance, installation, and documentation
structure look coherent. Another broad redesign or record migration would add
release risk without addressing the problems found here.

## Required corrections

Findings are checked against the final committed source. P1 means fix before
release; P2 means a concrete correction to include in release preparation.
S means hours; M means roughly a day including focused verification. These are
estimates, not commitments. All findings below have high confidence from source
inspection; executable evidence is distinguished in the validation section.

| ID | Priority | Finding | Effort | Fix risk |
| --- | --- | --- | --- | --- |
| R1 | P1 | Rolling reconciliation can drop part of a candidate | M | Medium |
| R2 | P1 | Claude rolling resume needlessly measures completed candidates again | S | Low |
| R3 | P1 | Claude parallel acceptance continues past an unresolved destination checkpoint | S | Low |
| R4 | P2 | Continuing Claude parallel batches resolve supposedly frozen profiles again | S | Low |
| R5 | P2 | Claude parallel rejects a stale advisory even when task work is safe | S | Low |
| R6 | P2 | Task approval omits retained-code prerequisites | S | Low |
| R7 | P2 | Dispatch captures the same working tree two or three times | S | Low |
| R8 | P2 | Public references still describe older approvals, worktrees, and harness support | S | Low |

### R1: Preserve the full candidate during rolling reconciliation

[experiment.rs:2645](/Users/zayenz/projects/zdev/src/research/experiment.rs:2645)
resets a stopped source to the accepted baseline. At
[2652](/Users/zayenz/projects/zdev/src/research/experiment.rs:2652), the retained
candidate is replayed only when the source is completely clean.

A candidate can legally modify an existing file and add an untracked owned
file. The hard reset removes the existing-file edit but leaves the addition.
The resulting dirty status skips replay, and `return_candidate` records only
the surviving addition. The retained original remains recoverable, but the
active candidate silently loses part of its implementation.

Apply the retained candidate deterministically, including additions. Distinguish
an unapplied patch from an applied or conflicted patch; cleanliness cannot make
that distinction. Check one candidate containing both kinds of change across
baseline drift and interrupted refresh. Avoid general changes to candidate
storage or ownership.

### R2: Resume measurements by code identity

[zdev-research.js:587](/Users/zayenz/projects/zdev/templates/zdev/claude/workflows/zdev-research.js:587)
compares the last measured commit ID to the returned transport commit ID.
Measurement and transport intentionally create different commits:
[experiment.rs:959](/Users/zayenz/projects/zdev/src/research/experiment.rs:959)
and [2443](/Users/zayenz/projects/zdev/src/research/experiment.rs:2443) use
different messages, even when the trees and parents match.

Consequently, an unfinished measured candidate is scheduled for another
measurement on resume. This spends allowance unnecessarily and cannot proceed
when the experiment has reached its attempt limit. Assessment and finalization
of an already completed measurement should remain possible.

Use the existing code comparison and current-policy checks. Check one resume
with an unchanged completed measurement at its attempt limit. No new retry
framework is needed.

### R3: Pause acceptance at unresolved destination state

After transporting source changes, the Claude parallel workflow can return
`blocked` from verification at
[zdev-parallel.js:414](/Users/zayenz/projects/zdev/templates/zdev/claude/workflows/zdev-parallel.js:414).
At [474](/Users/zayenz/projects/zdev/templates/zdev/claude/workflows/zdev-parallel.js:474),
only a shared decision prevents another arriving candidate from entering the
destination gate. A `preserved` result stops new source dispatch, but already
running siblings can still reach destination integration.

This can combine task B's transport with unresolved task A changes, contrary to
the explicit pause rule in
[parallel.md:91](/Users/zayenz/projects/zdev/templates/zdev/references/parallel.md:91).
It risks conflicts and mixed attribution during verification or commit.

Track whether transport left an unresolved destination checkpoint. Preserve
later source results until that checkpoint is recovered. A task-local blocker
before destination mutation may still allow unaffected tasks to continue.
This defect also exists in v1.6.0; it was not introduced by this release.

### R4: Keep concrete worker settings across parallel batches

[claude-area-loop.md:7](/Users/zayenz/projects/zdev/templates/zdev/claude-area-loop.md:7)
promises retained concrete settings across finite batches of one goal. The
parallel workflow accepts profile names at
[zdev-parallel.js:113](/Users/zayenz/projects/zdev/templates/zdev/claude/workflows/zdev-parallel.js:113)
and resolves them anew at
[191](/Users/zayenz/projects/zdev/templates/zdev/claude/workflows/zdev-parallel.js:191).
It has no input for the retained concrete role map.

If saved preferences change during a continuing goal, the next batch silently
changes model or effort. Accept and validate the same concrete role records
already supported by sequential implementation, and carry them between batches.
This gap becomes relevant with the new continued parallel-goal behavior.

### R5: Treat stale but safe work consistently

[zdev-parallel.js:363](/Users/zayenz/projects/zdev/templates/zdev/claude/workflows/zdev-parallel.js:363)
requires `stale_advisory === false` after transport. The shared contract at
[task-workflows.md:99](/Users/zayenz/projects/zdev/templates/zdev/task-workflows.md:99)
says to report the advisory and continue safe task work.

A safe batch can therefore stop after integration and before verification,
unlike its sequential equivalent. Accept a boolean advisory when `safe` is
true, and report it once. Preserve every actual safety gate. This inconsistency
also exists in v1.6.0.

### R6: Include retained-code dependencies in approval Markdown

[tasks.rs:472](/Users/zayenz/projects/zdev/src/tasks.rs:472) renders `blocked_by`
but omits `requires_code`. The latter is included in the approval fingerprint
and imported task header, and changes admission behavior.

Two bundles with different execution prerequisites can display the same review
Markdown. Render a nonempty "Requires retained code" section containing the
task IDs. Existing fingerprints and record schemas can remain unchanged.

### R7: Reuse context within one dispatch decision

[lib.rs:1614](/Users/zayenz/projects/zdev/src/lib.rs:1614) calls snapshot
comparison, which captures fresh working-tree evidence at
[2178](/Users/zayenz/projects/zdev/src/lib.rs:2178). Implementation dispatch
captures it again at [1690](/Users/zayenz/projects/zdev/src/lib.rs:1690), and
retained-plan comparison at [1703](/Users/zayenz/projects/zdev/src/lib.rs:1703)
can trigger a third capture.

The intervening profile resolution is read-only. Each capture includes Git
state and untracked contents, so batching hash commands still leaves two or
three full reads. The field-feedback note reports expensive captures in a
large output tree; its duration is an observation, not this review's benchmark.

Capture fresh context once for the dispatch decision. Compare stored admission
and plan snapshots against those bytes and use the same task data for routing.
Preserve freshness at later worker and completion boundaries; add no cache
across calls.

### R8: Reconcile the public workflow contract

The public references contain consequential contradictions:

- [workflow.md:230](/Users/zayenz/projects/zdev/docs/workflow.md:230) makes
  exact bundle approval unconditional despite delegated task import.
- [workflow.md:261](/Users/zayenz/projects/zdev/docs/workflow.md:261) says
  unrequested worktrees wait for a choice; current implementation defaults to
  an isolated worktree when the user has not selected otherwise.
- [workflow.md:278](/Users/zayenz/projects/zdev/docs/workflow.md:278) says
  Oh My Pi only offers sequential work; the shipped route supports parallel
  work when its asynchronous capabilities qualify.
- [task-format.md:17](/Users/zayenz/projects/zdev/docs/task-format.md:17)
  says Claude rolling research follows later, although it ships now.
- [commands/parallel.md:26](/Users/zayenz/projects/zdev/docs/commands/parallel.md:26)
  describes overlap without identifying OpenCode's refusal and sequential
  fallback or T3's sequential limitation.
- [README.md:139](/Users/zayenz/projects/zdev/README.md:139) and
  [146](/Users/zayenz/projects/zdev/README.md:146) link to research sections
  removed from the user guide. Point them to `docs/research.md`.
- [cli/tasks.md:46](/Users/zayenz/projects/zdev/docs/cli/tasks.md:46) promises
  a complete task-bundle JSON example in the user guide that is no longer there.
- [tasks.rs:3030](/Users/zayenz/projects/zdev/src/tasks.rs:3030) still says
  automatic hypothesis execution is unsupported. Direct users to the explicit
  research route while preserving ordinary dispatch's hypothesis rejection.

Update these passages and add the changed defaults to the upgrade guide.
Also qualify the blocker-completion paragraph at
[codex-skill.md:112](/Users/zayenz/projects/zdev/templates/zdev/codex-skill.md:112)
as ordinary Area loop behavior. Research already has its own lifecycle and
must not complete an unfinished objective for a blocker or exhausted allowance.
The existing surrounding scope makes this an instruction ambiguity, rather
than a proved unconditional lifecycle defect.

## Breaking changes and missing capability

I found no reason to break existing records or worker configuration. Keep
schema version 1 and the existing goal/loop aliases. The 2.0 change in workflow
and acceptance expectations is substantial enough without inventing a data
migration. Another broad module reorganization would make these repairs harder
to assess; fix the local lifecycle and decision boundaries first.

The upgrade notes should explicitly name the defaults users will notice:
implementation worktrees, reuse of delegated task/import authority, continuing
parallel batches and their four-slot default, shared-area branches, stronger
readability review, and Sol/high-effort defaults. These affect workspace
behavior and model usage even though old records still load.

The strongest missing capability is extending research authorization. The
run API has Start, Stop, Resume, and End, but no extension. Resume at
[run.rs:316](/Users/zayenz/projects/zdev/src/research/run.rs:316) clears the stop
state while retaining the original deadline and allowances. The new
[field-feedback note](/Users/zayenz/projects/zdev/.zdev/_notes/autoresearch-field-feedback-2026-10-17.md:129)
describes an actual request for another day that required ending and replacing
the run with manual accounting notes.

An explicit additive extension could preserve run identity, spent allowances,
unresolved work, and experimental policy while changing only what the user
authorized. Define those semantics before implementing it. This is useful
follow-up work; the correctness repairs should take priority, and this feature
does not require a breaking release.

## Documentation publication

The [published site](https://zayenz.se/zdev/) is substantially ready as a reading
experience. A browser crawl checked 68 reachable documentation pages: all
returned success, and their internal anchors resolved. Search for "rolling
research" returned seven results; following a result opened the correct deep
anchor. A 390-pixel viewport had visible content and navigation with no horizontal
overflow. These checks do not establish a complete accessibility audit.

`feature/zdev-site` at `13e9e17` is the older site, already ported and superseded
by main's guides, command/CLI pages, and navigation. Do not integrate that branch
wholesale. Its useful remaining change is the direct README documentation link;
its public-listing instructions can inform promotion of the host project entry.

Before listing and releasing:

1. Correct R8 and finish the code repairs. Refresh the changelog for delegated
   approvals, worktrees, notes, shared-area branches, and declared research
   outputs; its current 2.0 entry omits several later changes.
2. After the release is available, update the accurate-for-now 1.6.0 status in
   [landing.mdx:75](/Users/zayenz/projects/zdev/site/src/landing.mdx:75) and
   [getting-started.md:25](/Users/zayenz/projects/zdev/docs/getting-started.md:25),
   date the changelog, and point installation advice at the released version.
3. Rebuild and import the selected final documentation revision into zayenz.se.
   Publication is manual: a zdev tag or push does not update the hosted copy.
4. Add the documentation link to the README and promote the host's project entry
   and guide links. Keep the existing source repository link available.

The new guide already explains installation prerequisites, the distinction
between the CLI and agent workflow, a complete small task, feature work,
research, longer runs, and coding/review expectations. It does not need another
tutorial framework or a second set of source documentation.

## Verification and limits

No tests were added. Tests invoked neither an LLM nor a live coding harness.
The independent audit agents performed source review, not model-driven tests.

- Offline Cargo build passed on the initial snapshot and on final `97b73df`.
- Fifteen focused Rust tests passed: twelve work-context tests, shared-branch
  behavior, declared untracked research outputs, and the existing scripted
  Claude parallel wiring scenario. Execution took about 17.3 seconds.
- The report generator's existing suite passed in about 3.2 seconds using its
  installed dependencies. Its optional browser suite was not run.
- The standalone release smoke passed: six integrations installed and checked,
  and task import, completion, commit/change ID, and record checking succeeded.
- The final portable Codex fixture check passed after the guidance commit.
- Formatting and diff checks passed for the committed pending changes.
- Two disposable Git probes confirmed the Git behavior underlying R1 and R2.
  The full failing zdev recovery paths were established by source inspection;
  they were not exercised end to end in this review.

The Rust tests and release smoke ran on `ff4744e`. The final commit only changes
instructions, their generated copies, one integration guidance string, and the
field note; it was reviewed separately, rebuilt, and checked for portable-bundle
consistency. The runtime paths implicated by the findings are unchanged.

The full Rust suite, full release/package gate, dependency advisory scan,
cross-platform builds, and live harness acceptance were not rerun. `cargo audit`
is not installed. The GitHub status wrapper returned no commit status entries;
that is not evidence that current CI passed. Live Claude research acceptance
was explicitly deferred in the approved auto-area task, so it is recorded as
unverified rather than added as a new requirement.

## Change-by-change checklist

"No new finding" means the changed contract and relevant callers were inspected
without a concrete defect being established; it does not mean exhaustive
runtime certification. Findings describe the final state, including repairs
that supersede earlier intermediate implementations.

| Commit | Change considered | Assessment |
| --- | --- | --- |
| `94b7b93` | Stable crates.io publication | Tag, release, package version, retry, and pinned authentication checks are coherent. Remote publication not exercised. |
| `826ff43` | GPT-6 built-in profiles | No new finding; current defaults supersede these values. |
| `531a189` | GPT-6.1 Sol settings | No new finding in resolved settings and compatibility. |
| `efe0f8e` | Codex worker completion and cleanup | Result ownership and cleanup instructions are coherent. |
| `f8ea3fd` | Task-oriented coding guidance | No new finding; later guidance retains task boundaries. |
| `4e6c992` | Tighter execution guidance | No new finding in the final guidance. |
| `90719c9` | Stop native loops on a confirmed blocker | Ordinary stop/report contract is coherent; clarify Research exclusion under R8. |
| `145ad59` | Guidance organized by role | Role loading and disclosure are coherent. |
| `77436f8` | Distributed guidance and checked findings | Independent claim checking is bounded; no recursive reviewer loop. |
| `66b5c5b` | Standalone area reports | Existing generator suite passes; editable source and offline artifact contract are clear. |
| `5480f74` | T3 orchestration | Explicit selection and sequential restriction are intentional. |
| `32c95d2` | Visible T3 parent goals | Parent continuation and worker completion ownership are coherent. |
| `ef8feae` | T3 fixture refresh | Consistent with the final canonical contracts on inspection. |
| `8e4fea2` | Linear integration policy | Applied to the pending commit; main has no merge commits. |
| `1a31d75` | Research exploration records | Intent inspected; no runtime promise introduced by the records. |
| `0ab1e46` | Research design decisions | Settled comparison, allowance, and lifecycle choices used as audit context. |
| `ad124a1` | Research task bundle | Inspected as implementation intent; no separate defect. |
| `c6d1e3e` | Research settings and task evidence | Optional record additions remain schema-1 compatible; R6 affects approval display. |
| `d56d47a` | Measurements and significance comparison | Direction, thresholds, and comparison contract are coherent on inspection. |
| `b518312` | Local experiments and recovery | No new defect found in ordinary local keep/discard handling. |
| `87d2376` | Authorized runs and durable accounting | Existing limits preserve costs; explicit extension is an additive gap. |
| `5a0cc09` | Codex serial research | Settled serial contract is coherent on inspection. |
| `1e7398d` | Claude serial research | No concrete serial regression found; no live harness execution. |
| `24acdce` | Isolated candidates and reconciliation | R1: mixed existing-file edits and additions are not safely replayed. |
| `a836f94` | Codex rolling research | Shared reconciliation inherits R1; separate role/measurement limits are coherent. |
| `a6bcafc` | Claude rolling research | R1 and R2 affect reconciliation and measured-candidate recovery. |
| `5ccb83e` | Research qualification records | Codex evidence and explicit Claude live deferral are represented honestly. |
| `609f649` | Auto integration refresh | No separate defect beyond the current workflow findings. |
| `063eeb6` | General-area tasks | Record additions considered; later completion records inspected. |
| `a76ea4e` | Contextual guidance and bounded review | Correction allowance and focused recheck appear coherent. |
| `78162fd` | Language/framework supplements | Contextual loading and project-instruction priority are clear. |
| `706111e` | Initial release repairs | Packaging probe, submodule rejection, locks, and reduced scripted checks inspected; no repeat finding for repaired defects. |
| `4b7af32` | Parallel authorization repair record | Consistent with blanket authority; concrete profile retention still needs R4. |
| `9000c4a` | Task/research state preservation repairs | Created-file rollback and retained research-state repairs look coherent. |
| `51708d1` | Parallel authorization completion record | No separate defect in completion bookkeeping. |
| `6872fc2` | Dependency-graph logo | Assets present and rendered by the published site. |
| `04e4e58` | README branding | Current branding retained. |
| `9fa91ed` | Delegated approvals, notes, and loop safeguards | Intent is coherent; reconcile old public statements under R8. |
| `93c4dc0` | Sol/high-effort ordinary defaults | Concrete defaults are coherent; explain the change in upgrade notes. |
| `8a6ad3c` | Shared areas/threads/branches | Exclusivity removal is consistent; focused shared-branch test passed. |
| `8d52783` | Continuing goals and execution defaults | R4 breaks profile retention across batches; R3/R5 are inherited parallel defects worth repairing now. |
| `244a01b` | Optional-area task listing | Existing area summary now succeeds, including an empty project. |
| `308f5b2` | Declared untracked research outputs | Focused check passed; undeclared source still remains protected. |
| `7f175e0` | Batched untracked hashing | Path/hash mapping is preserved; R7 identifies remaining repeated captures. |
| `5549c3e` | Documentation site and command/CLI guides | Site inventory and links are coherent; R8 covers stale public contracts. |
| `76949da` | Clearer site and persistent navigation | Live navigation and narrow viewport checked. |
| `f53882d` | Tutorial prerequisites | Git, harness/model access, and example-specific Python are distinguished. |
| `55491f0` | Areas, tasks, research, and longer-run guides | Guide structure is useful; update stale references under R8. |
| `d40d3f0` | Ordinary feature walkthrough and links | Main walkthrough is coherent; README research anchors still need repair. |
| `ff4744e` | Coding/review/report documentation | Public guide scope is adequate; final release version notices need promotion. |
| `97b73df` | Standalone review, naming/decomposition, field feedback | Independently reviewed, rebuilt, portable fixture checked, and committed; no new draft-specific blocker found. |

## Considered and rejected

- A new record-schema version or forced migration: optional additions preserve
  existing records; no concrete incompatibility requires it.
- A broad architecture rewrite before release: the findings have local fixes;
  extra layers or arbitrary function-size rules would add risk.
- Reporting declared output handling and batched hashing as unfixed: both
  changes already address the original failures; R7 is the distinct remaining
  duplicate-read issue.
- Reporting deferred live Claude testing as a newly unmet task requirement:
  the approved task explicitly removed that requirement.
- Merging `feature/zdev-site` wholesale: it would restore older content and
  navigation already superseded on main.
- Claiming a public-site outage because the web-fetch tool could not access it:
  the browser loaded it and checked its pages successfully.
- Treating every field-note suggestion as a release blocker: authorization
  extension is grounded direction; broader budget and data-reuse redesigns
  require separate decisions and were not established as code defects here.

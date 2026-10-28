# Autoresearch feedback from the Gecode autoport campaign

17 October 2026. Codex, macOS, local zdev 2.0.0. Source checked at
`7f175e0`, including the untracked-output and batched-hashing fixes.

The research objective was to improve Gecode's configured portfolio through
small measured wins, initially at eight workers. The user delegated supporting
edits, task creation and commits, and later requested at least another 24 hours
of work. This note draws on that campaign's recorded experience and a focused
read of zdev's current contracts and implementation. It is not a controlled
measurement of workflow overhead or a complete code audit.

My main concern is that running the workflow became a substantial part of the
research project. Zdev preserved useful evidence, but continuing safely required
too much manual accounting, repeated inspection and interpretation of competing
instructions. The remedy should remove work, not introduce another framework.

## What to keep

Independent review, exact-path commits and retained raw measurements were
valuable. The workflow correctly kept a CPU/RSS-invalid trajectory invocation
invalid rather than turning it into a successful comparison after a parser fix.
Native checks also exposed a real integration error: an explicitly requested
all-searchable LNS pool still inherited a smaller driving group.

Keep those checks. Keep real baselines, condition-specific results, negative
findings and separation of research retention from product-default promotion.
The existing `--untracked-output` rebaseline support and batched hashing are
already useful improvements; the original failures they address should not be
reported again as unfixed bugs.

## 1. Give the outer research objective one lifecycle

The user had to correct the agent for treating completion of one experiment as
completion of the research goal. Later, exhausting worker-dispatch allowance
became another stopping boundary even though the area objective was unfinished.

There is a concrete instruction hazard. `templates/zdev/codex-skill.md` tells
the adapter to complete a matching goal on the first terminal blocker, under a
stop-and-report interpretation. `templates/zdev/references/research.md` says
not to complete an unfinished objective for a blocker or exhausted allowance.
These can describe different kinds of goals, but the distinction must be made
before dispatch, not reconstructed by the coordinator at a stopping point.

Keep the user's objective separate from its current campaign, task and compute
authorization. An ended campaign should report its stop reason without implying
that the area objective was achieved. For research, an empty queue should lead
to another justified question while authorization remains, not automatic
success. Follow the harness's actual lifecycle rules; do not prescribe a
completion operation that conflicts with the objective or tool contract.

**First fix:** make the selected research route's lifecycle authoritative and
remove contradictory unconditional instructions from the generated adapter.
Check a task completion, an empty queue and an exhausted campaign against the
same unfinished outer objective.

## 2. Make an ordinary iteration cheap

The worktree held roughly 52,000-53,000 untracked capture files. Originally,
snapshot collection launched a Git process per file. Batching is now implemented,
but repeated collection still reads unrelated historical captures. After that
fix, one `dispatch-spec` invocation was observed still running after 4 minutes
24 seconds while hashing old campaign files. That is one observation, not an
average latency estimate.

There is also avoidable work inside a single command: `dispatch_spec` in
`src/lib.rs` calls `compare_work_context`, then the implement branch calls
`work_context_output` again. Retained-plan comparison can collect it again.
This follows an already completed `work-context --store` call.

Start with two small changes:

- Reuse the fresh context already collected inside a command. Keep fresh checks
  at real edit/verification boundaries; do not collect identical state several
  times within one dispatch decision.
- Let source ownership checks use explicitly declared generated-output
  boundaries. Preserve those files and keep strict checks for any capture that
  an experiment actually consumes. Never silently ignore tracked changes or
  unexpected untracked source, and do not assume every `results/` directory is
  disposable.

The compact snapshot locator is useful. Extend that approach to routine run
inspection: the coordinator usually needs the current task, baseline validity,
remaining limits, active work and next action, not every historical checkpoint
and observation. Full details should remain available on demand. Large JSON
responses repeatedly caused output truncation and unnecessary context use.

Also make immutable snapshot inspection genuinely usable read-only.
`work-context --show` required opening the writable zdev state lock in this
environment; a permission error then required escalation merely to read it.
Review whether that lock is needed for immutable reads.

## 3. Separate authorization, reservations and actual expenditure

The default 20 work attempts count planning, implementation and verification
dispatches, not 20 scientific experiments. Supporting work can exhaust that
allowance well before an overnight wall-clock window ends. The user should not
have to learn this distinction by asking why research stopped.

The coordinator also introduced several independent CPU, process and phase
ceilings that the user had not specified numerically. Some were sensible safety
estimates, but I then treated them as hard user authorization and repeatedly
asked for extensions. That was partly an agent mistake, not a zdev defect.

Show each limit's origin: explicit user constraint, product default or
coordinator estimate. A time-based request should not unexpectedly become a
short dispatch-count run. Use a generous or advisory dispatch limit for that
mode, while retaining explicit safety and resource constraints.

Reservations and spending also need different names and behavior. Task036
charged 1,700 CPU seconds in nonrefundable envelopes; its recorded actual
implementation and verification CPU totaled about 51.9 seconds. Those are
different quantities. Calling both a consumed compute budget made cheap
engineering work exhaust practical research capacity on paper.

Reserve the maximum while work is live; after trustworthy resource accounting,
charge actual successful and failed work and release unused capacity. Unknown
or interrupted work can retain its conservative ceiling until recovered. If a
study deliberately uses prepaid whole-attempt charging, make that an explicit
policy, not an assumed meaning of CPU expenditure. Never erase failed work or
accept an invalid measurement to recover allowance.

State accounting coverage too: native execution and its wrappers were counted,
but coordinator Git/harness work was not covered by the same receipts. Do not
present that narrower ledger as whole-session resource use.

## 4. Support an explicit extension without dismantling the run

The current run commands provide stop/resume and end/start, but no extension
operation. To honor the user's additional-day request, the coordinator ended
the settled exhausted run, started another, carried old costs in its scope,
and wrote separate budget notes. The native goal text still contained the old
deadline because its available API could not edit an unfinished objective.

Provide one explicit authorization-extension operation. Retain prior costs,
failures, comparison state and original limits, and record which allowance the
user changed. Do not reset counters or resume unresolved effects. The stable
outer objective should refer to current authorization rather than embed a
deadline that becomes stale on renewal.

Likewise, extending permission alone should not change the scientific study.
`policy_matches` in `src/research/run.rs` freezes correctness-command strings;
those commands included literal admission/preparation paths. Distinguish an
administrative path or authorization change from a changed horizon, workload,
solver or acceptance rule. A material experimental change still requires a
declared transition and an actual new baseline.

## 5. Make data reuse and baseline invalidation understandable

A bounded-memory collector/parser correction changed host-file hashes. Existing
reference checks then refused saved starts even though native binaries, model
artifacts and assignments were unchanged. A separate supporting task built an
explicit replay-validated reuse path. That was safer than weakening checks,
but routine reader maintenance should not require repeatedly inventing this
transition in each research area.

An evaluator should identify the inputs that determine search/replay separately
from collection and analysis code. Reuse validated assignments when their
relevant contract remains unchanged. Reanalyze original raw records with an
identified corrected reader without pretending they were collected by it.
Require fresh replay or measurement when the changed component can affect that
claim. Explain the reason for invalidation, not just that a file hash differs.

At zdev's level, `accepted_code_matches` also compares every non-`.zdev` path
against the baseline revision. By construction, unrelated documentation can
therefore invalidate it too. Where an experiment declares its relevant inputs,
use that boundary; remain conservative when the dependency is unknown.

Keep this distinction narrow and explicit in the existing evaluator contract;
it does not need a general compatibility service. In particular, reanalysis
must not retroactively accept the CPU/RSS-invalid task032 invocation.

## 6. Detect a comparison policy that can no longer improve

The group screen retained a source-declared profile with mean restricted time
to improvement near 0.012736 seconds. Its original absolute minimum-gain gate
remained about 0.125006 seconds. For this nonnegative metric, no subsequent
candidate could satisfy that gate, even by reaching zero.

Freezing the gate during the comparison was correct. Silently lowering it
after seeing candidate results would not be a remedy. Warn when a declared
metric bound makes further acceptance impossible, and surface that as the next
research decision before more candidates are prepared.

For later studies, allow a predeclared relative-gain rule or a justified
absolute floor that fits the expected scale and noise. Keep the original study
and threshold intact; a new condition needs its own actual baseline. The
workflow should help identify that transition rather than leave the agent to
infer it from repeated rejection.

## 7. Reduce handoff and execution bookkeeping

Workers received long, overlapping instruction files, multiple budget records
and repeated summaries of the same state. The coordinator reconstructed public
envelopes from semantic results and copied plans into several representations.
This took attention away from the implementation and research question.

Use one canonical task packet and result. Let zdev attach/check task identity,
snapshot and resolved profile mechanically. If compatibility requires another
envelope, generate it inside the tool instead of asking the model to rebuild it.
Keep full evidence at file locators and return a short completion summary.

I also sent additional concerns while a planner was working. Its response
covered only the latest amendment and referred to an earlier full plan that the
coordinator had not received. Another planning dispatch was needed to obtain a
self-contained answer. This was an orchestration mistake. Update one task
packet deliberately, and require the final response to cover the complete task;
do not let successive messages accidentally replace its scope.

Move repeated execution mechanics into existing zdev support: resolve the
interpreter and known tool locations once, run a command under a bounded
capture, and report its actual exit/resource/cleanup result. The campaign wrote
many temporary Python supervisors to do this manually. One narrow reusable
command runner is enough; do not build another scheduler. Do not confuse a
sandbox counter denial, a PATH miss or a wrong interpreter with a solver defect.

## 8. Optimize for research progress, not completed ceremony

Supporting code and verification are necessary, but they are not measured
portfolio improvements. Report the question answered, valid comparisons,
negative findings, current accepted result and the next useful experiment.
Keep engineering progress separate. A large task count or a retained isolated
screen win does not establish an eight-worker gain.

Make the default loop small: choose one informative contrast, run the relevant
preflight, measure a right-sized cohort, analyze saved results, retain the
finding, then select the next question. One or two suitable instances per model
often suffice for screening. Expand when ambiguity or confirmation warrants it,
not to fill a Cartesian product. Holdout separation and consequential native
validation remain essential.

Independent review should check the changed claim and reuse adequate unchanged
build/test receipts. It should not automatically repeat expensive setup or
invent a larger test suite. Ask the user about an actual scope, safety or
resource decision, not every incidental technical obstacle already delegated.

## Suggested first changes

1. Reconcile outer-goal lifecycle instructions and add explicit authorization
   extension, preserving costs and unfinished work.
2. Remove redundant context collection inside dispatch; support declared output
   boundaries and compact routine inspection without weakening used-input checks.
3. Separate reserved capacity from measured spending and distinguish dispatch
   counts from experiment attempts. Reuse one supported execution wrapper.

Then exercise one complete research iteration and one recovery on a fixture
with a realistically large retained-output tree. Measure coordination latency
and work before the next valid comparison, not just command counts. Add focused
regressions for these boundaries, not an exhaustive new harness test matrix.

Relevant starting points are `src/lib.rs` (`dispatch_spec`, work-context
collection and snapshot reads), `src/research/run.rs` (limits, renewal and
policy matching), `templates/zdev/codex-skill.md`,
`templates/zdev/references/research.md`, `docs/workflow-round-trips.md` and
`docs/harness-goals.md`. The earlier specific report, "Rebaseline rejects
unrelated untracked result captures", was fixed by `--untracked-output` in
308f5b2 and its note removed.

This is feedback for later design work. It creates no tasks, changes no active
research authorization and requests no interruption of the portfolio campaign.

+++
schema_version = 1
id = "auto-010"
key = "live-acceptance"
area = "auto"
status = "done"
complexity = "standard"
priority = "low"
blocked_by = ["auto-008", "auto-009"]
+++
# Verify Codex research and Claude Code readiness

## Outcome

Live Codex walkthroughs demonstrate serial and rolling research. Local validation and review against current primary documentation establish Claude Code readiness as far as possible without running Claude Code; the user will try it elsewhere later.

## Context

Read the brief's Testing section and the documented installed entry points. Codex live acceptance uses a disposable demo repository, native workers, authorized worktrees, deterministic decision fixtures and an existing meaningful workload. Claude readiness uses the committed workflows, existing deterministic scenarios and read-only integration checks. The user explicitly removed live Claude execution as a completion requirement on 2026-10-08.

## Boundaries

- Do not rely on finding a stochastic performance gain; demonstrate decisions with reproducible candidate fixtures.
- Keep demo artifacts outside this area's durable records apart from concise acceptance evidence; make only focused fixes or follow-up tasks for observed defects.
- Do not invoke Claude Code here. Record live Claude execution as unverified and leave it to the user's later testing elsewhere; it does not block completion.

## Done when

- [x] Live Codex demonstrates mixed tasks, a kept and rejected hypothesis, terminal failed/inconclusive evidence, and fresh-session serial recovery.
- [x] Live Codex demonstrates rolling refill, separate worker/measurement limits, changed-baseline reconciliation, and interruption/resume with retained candidate code.
- [x] Live Codex demonstrates research on an existing meaningful benchmark/workload with correctness checks and comparable baseline/final measurements; a measured gain is not required.
- [x] Claude serial and rolling workflows have passing local scenario and fixture evidence, are reviewed against current primary workflow documentation, and any observed fixable readiness defects are resolved. Live execution remains explicitly unverified for the user's later testing.
- [x] Finite stops and explicit unlimited authorization behave as documented; an empty queue does not complete the research objective or close its area.
- [x] Record harness prerequisites, executed scenarios, observable results, and any remaining blockers in the task result.

## Validation

- Reuse the independently reviewed Codex serial and rolling walkthroughs while their source is unchanged, including the meaningful workload and deterministic decisions. Inspect task evidence, Git retention/restore and preserved ambiguous work.
- Review Claude workflow APIs and packaging against current primary documentation. Inspect existing deterministic serial/rolling scenarios and their passing evidence, and run a fresh read-only Claude fixture check. Do not rerun unchanged long scenarios or add tests merely for this requirement change; run focused validation for any observed fixes.
- For these planning-record changes, run `zdev check auto --format json` and `git diff --check`. Live Claude execution is not required.

## Acceptance evidence

Codex live acceptance ran on 2026-10-08 against integrations from commit
`2ae004b49d26f8269d7d870415a7b3138318de58`. Authenticated Codex CLI 0.160.1
exposed native goal and collaboration controls. Managed worktree APIs were
unavailable; the documented, explicitly authorized local linked Git worktree
fallback supplied isolated sources. Frozen worker profiles were retained.

- Serial execution generated mixed tasks, independently verified supporting
  work and keeps, rejected a regression, and recorded terminal failed-to-run
  evidence. Explicit unlimited authorization retained its command limit and
  deadline. Fresh-session recovery preserved candidate code, run identity and
  costs, then measured and independently verified the interrupted candidate.
- Rolling execution demonstrated refill while a sibling remained active,
  two worker slots with one measurement slot, a refused concurrent evaluator,
  retained decisions and fresh-session recovery. A supplementary run corrected
  an initially late supporting dispatch check and demonstrated previously
  measured stale code: B measured 45 against 60, A was independently kept at
  50, then counted reconciliation and native source conflict repair preceded
  B's new measurement of 45 against 50 and fresh acceptance verification. Both
  observations and candidate references remain retained.
- Serial and isolated rolling runs exercised the existing pyperformance JSON
  dumps inputs with actual timing samples and JSON roundtrip correctness.
  Each finite run consumed three work attempts and eight managed commands,
  rejected its candidate and restored accepted code. Comparable final timing
  audits ran outside the ended authorizations. No stochastic gain or pyperf
  calibration is claimed.
- Completed runs ended with settled reservations and open, exhausted areas;
  queue exhaustion did not establish the optimization objective. All source
  worktrees remain retained. Disposable setup errors were corrected and all
  four area checks pass in each demo. No product fixes or source changes were
  needed, so repository-wide tests were not repeated.

The concise evidence index and exact runtime, measurement and Git evidence are
outside this area at `/tmp/zdev-auto-010-CBNgAa/acceptance-index.md`.

Claude readiness review on 2026-10-08 found no fixable integration defect.
The installed workflow's metadata, structured arguments, mediated agent calls,
fresh-invocation recovery and conservative native call limits match the current
[workflow documentation](https://code.claude.com/docs/en/workflows). Plugin
version 1.6.0 and its workflows directory match the
[plugin reference](https://code.claude.com/docs/en/plugins-reference).
The unchanged source retains auto-009's independently passing 23 serial and
22 rolling deterministic scenarios, within its full 185 integration and
three unit tests. A fresh isolated read-only Claude fixture check returned
`status: ok` and `warnings: []`; its output is retained at
`/tmp/zdev-auto-010-claude-ready-_d517puv/fixture-check.json`.
Historical demo reports describe the former live-Claude completion gate;
the revised requirements above supersede that gate.

Claude serial and rolling live execution remain **UNVERIFIED**. Claude Code
was not run. The user will test it elsewhere later; this follow-up does not
block completion after the required local readiness review and checks.

## Result

Verified live Codex serial and rolling research and local Claude Code readiness; user live Claude testing is deferred and non-gating.

Validation:

- Independent whole-task PASS at W915af43566617890; retained live Codex evidence covers all acceptance scenarios.
- Current Claude workflow and plugin documentation review, unchanged passing 23 serial and 22 rolling scenarios, and fresh isolated fixture check status ok with warnings []. No Claude CLI invoked.
- zdev check auto and git diff --check passed; exact pre-completion snapshot comparison equal true.

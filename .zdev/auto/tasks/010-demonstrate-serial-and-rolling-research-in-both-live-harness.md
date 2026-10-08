+++
schema_version = 1
id = "auto-010"
key = "live-acceptance"
area = "auto"
status = "open"
complexity = "standard"
priority = "low"
blocked_by = ["auto-008", "auto-009"]
+++
# Demonstrate serial and rolling research in both live harnesses

## Outcome

An attended walkthrough demonstrates the complete initial feature with Codex and Claude Code and records any concrete remaining defects.

## Context

Read the brief's Testing section and the documented installed entry points. This late acceptance requires installed, authenticated Codex and Claude Code with native worker/worktree controls and enough concurrent capacity for two candidates; use a disposable demo repository with deterministic decision fixtures and an existing meaningful benchmark/workload.

## Boundaries

- Do not rely on finding a stochastic performance gain; demonstrate decisions with reproducible candidate fixtures.
- Keep demo artifacts outside this area's durable records apart from concise acceptance evidence; make only focused fixes or follow-up tasks for observed defects.

## Done when

- [ ] Both live harnesses demonstrate mixed tasks, a kept and rejected hypothesis, terminal failed/inconclusive evidence, and fresh-session serial recovery.
- [ ] Both demonstrate rolling refill, separate worker/measurement limits, changed-baseline reconciliation, and interruption/resume with retained candidate code.
- [ ] Both harnesses demonstrate research on an existing meaningful benchmark/workload with correctness checks and comparable baseline/final measurements; a measured gain is not required.
- [ ] Finite stops and explicit unlimited authorization behave as documented; an empty queue does not complete the research objective or close its area.
- [ ] Record harness prerequisites, executed scenarios, observable results, and any remaining blockers in the task result.

## Validation

- Run the four documented live walkthroughs against committed integrations, including an existing benchmark/workload with correctness and comparable baseline/final measurements as well as deterministic decision fixtures. Inspect task evidence and Git retention/restore, and verify no ambiguous worker is cleaned up. Run repository checks only for any attributable fixes.

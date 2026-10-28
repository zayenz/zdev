+++
schema_version = 1
id = "general-030"
key = "widen-research-measure-test-deadlines"
area = "general"
status = "open"
complexity = "routine"
afk = true
priority = "high"
blocked_by = []
+++
# Give non-timeout cases in the research measure test a generous deadline

## Outcome

research_measure_collects_samples_and_retains_execution_failures passes under parallel load: only its timeout case uses a short deadline.

## Context

During general-029 this test failed in both of two concurrent full suites with outcome `timed_out` where `invalid` was expected; it passes sequentially. A GPT-6 Astra review found the cause: tests/lean.rs around line 15779 applies a 50 ms evaluator timeout to ordinary and invalid-output cases as well as the timeout case, so a trivial command can exceed it under CPU load.

## Boundaries

- Change only that test's settings; keep every outcome assertion, including the timeout case with a short deadline.

## Done when

- [ ] Ordinary and invalid-output cases use a deadline of several seconds; only the case that checks `timed_out` keeps a short deadline.
- [ ] The test passes when run under two concurrent full lean suites with separate CARGO_TARGET_DIRs.

## Validation

- Report the concurrent run.
- cargo fmt --all -- --check; cargo clippy --locked --all-targets --all-features -- -D warnings; cargo test --locked; git diff --check

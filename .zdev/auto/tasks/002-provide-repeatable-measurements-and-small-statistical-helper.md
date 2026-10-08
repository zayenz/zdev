+++
schema_version = 1
id = "auto-002"
key = "measurement-tools"
area = "auto"
status = "open"
complexity = "standard"
afk = true
priority = "normal"
blocked_by = ["auto-001"]
+++
# Provide repeatable measurements and small statistical helpers

## Outcome

An experiment can execute its evaluator, retain numeric samples, and compare them with its baseline under explicit area-defined rules.

## Context

Use the public research contract established by research-records. Read the brief's Measurement policy and common utilities and Testing sections and [Experiment mechanics](../background/experiment-mechanics.md). Follow existing subprocess/CLI and black-box test patterns; select a small documented set of optional confidence/significance methods with explicit assumptions.

## Boundaries

- One primary scalar metric; no general statistics framework or automatic choice of acceptance policy.
- Deterministic evaluators need neither repeated measurements nor statistical inference; invalid evidence cannot count as improvement.

## Done when

- [ ] Public measurement utilities repeat a command with timeout/error handling, preserve raw numeric samples, and report mean, median, and spread.
- [ ] Comparisons respect direction, threshold, comparable baseline conditions, and optional documented confidence/significance methods.
- [ ] Missing, nonfinite, malformed, failed, and timed-out measurements produce explicit invalid evidence.
- [ ] Collected observations feed the task-owned result contract and expose evaluation cost for run accounting.

## Validation

- Use deterministic evaluator fixtures for collection/failure behavior and known small samples checked against a trusted reference for statistics. Run focused tests and matching Rust checks; document commands and assumptions.

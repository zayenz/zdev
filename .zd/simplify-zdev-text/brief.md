# Simplify zdev language

## Objective

Make zdev instructions shorter and more reliable by replacing organizational prose with concrete trigger, action, and stop rules while preserving safety invariants.

Success means a model can choose the right zdev action from observable state
without classifying itself as a main, primary, or side conversation. Canonical
rules use concrete triggers, commands, allowed actions, and stop conditions.
User documentation describes visible behavior instead of harness governance.

## Why simplify

The current text often explains safety through organizational metaphors rather
than the state a model can observe. Phrases such as "return control to the main
conversation", "retain authority", and "committed additive task-set extension"
make the model infer which chat owns the work. That inference is fragile: adding
a task in a side chat previously made an implementation chat treat the new
commit as unexpected drift and stop.

The intended replacement is operational. For example:

- Instead of asking whether this is a side conversation, say: when adding tasks
  to an existing queue, run `zd tasks import <area> --from - --commit --format json`.
- Instead of saying that a primary conversation owns transitions, say: continue
  only when the user already requested the next interaction; otherwise report
  the result and stop.
- Instead of naming a "committed additive task-set extension", say: a commit
  containing only new task files and regenerated `TASKS.md` does not interrupt
  the selected task; consider it at the next `zd next`.
- Instead of requiring "full branch coverage" in Discuss, ask about unresolved
  choices that could materially change behavior, scope, task splitting, or
  validation, and stop when none remain.
- Instead of exposing reviewer authority and model-selection policy, state the
  review action directly: ask a fresh read-only reviewer to challenge the draft,
  then reconcile its suggestions against repository evidence.

This is not a request to weaken safeguards. Exact approvals, branch freshness,
Git baselines, edit boundaries, validation duties, verdict meanings, recovery
commands, and machine-readable schemas remain precise. The goal is to make each
safeguard easier to trigger correctly.

## Boundaries

- Preserve the activation boundary, personal/project choice, exact `/.zd/`
  exclusion, branch/base gates, explicit rebase consent, Git baseline, exact
  task-only concurrent-drift allowlist, testing level, verification duties,
  verdict meanings, explicit-path staging, task schema, JSON fields, commands,
  and recovery behavior.
- Preserve the fenced Markdown task approval contract and the breadth-first
  intent of Discuss, but remove exhaustive or organizational wording.
- Do not redesign zdev storage, task semantics, Git behavior, or harness
  capabilities as part of the prose simplification.
- Treat the development binary's rejection of this repository's legacy `.zd`
  areas as a separate compatibility issue; do not hide a migration in this
  language area.

## Settled decisions

- Use one shared transition rule: continue to another interaction only when
  the user already requested it; otherwise report, offer next actions, and
  stop. Approval applies only to the artifact shown.
- Use one task-intake rule: initial split uses ordinary import; additions to an
  existing list use `--commit --format json` unless the user explicitly wants
  uncommitted additions.
- Describe concurrent intake by its exact diff allowlist, not by chat role.
- Replace main/primary/side-conversation authority with command and agent-role
  boundaries. Implementers and verifiers leave `.zd`, lifecycle changes, and
  commits to their caller.
- Canonicalize the Git baseline, branch/base gate, rework loop, verification
  duties, and `PASS`/`REWORK`/`BLOCKER` meanings across harnesses.
- Required validation that cannot run safely is `BLOCKER`; only missing
  optional checks may be residual limitations.
- Discuss surveys high-impact decisions breadth first without requiring every
  source or an unknowable "full branch coverage" gate.
- User docs teach commands and observable behavior. Model policy lives in one
  canonical contract; harness templates add only harness-specific mechanics.
- CLI success and error text should give the next useful action without
  exposing internal policy vocabulary.

## Validation

- Canonical and checked-in rendered skill/reference copies remain in sync.
- Harness behavior tests assert safety invariants and required parser tokens,
  not incidental prose where possible.
- Focused tests cover changed CLI help, success, and recovery messages.
- Full formatting, strict locked Clippy, locked tests and build, release smoke,
  and diff checks pass.

## Testing

Focused coverage. Update existing skill-bundle, harness-rendering, CLI, and
documentation contract tests where behavior changes. Add a focused test only
when an important invariant lacks a stable existing seam. Do not introduce a
new test harness or broaden unrelated runtime coverage.

# Research workflow friction

## Objective

Fix the remaining problems reported from the Gecode autoport campaign that
made running zdev's research workflow a large share of the research effort.
Each fix removes work or false invalidation. None weakens a check on inputs an
experiment actually uses.

Source: [Gecode field feedback](../_notes/autoresearch-field-feedback-2026-10-17.md).
Read the section a task names before planning it.

## Already done (do not redo)

- `research run <area> extend` (feedback section 4, first half).
- Dispatch collects working-tree evidence once per decision (section 2, first
  bullet).
- Lifecycle wording in `templates/zdev/codex-skill.md` separates ordinary area
  loops from research objectives (section 1).
- `--untracked-output` for managed rebaseline (the linked bug note).

## Scope

Each item is a candidate task. Order is the recommended priority.

1. **Read-only snapshot inspection.** `work-context --show` must not need the
   writable zdev state lock to read an immutable stored snapshot (section 2).
2. **Unreachable acceptance gate.** Warn when a frozen `minimum_gain` cannot be
   met even by a perfect candidate (for a nonnegative lower-is-better metric:
   baseline below the absolute gain). Surface it as the next research decision.
   Optionally allow a predeclared relative-gain rule for new studies (section 6).
3. **Administrative vs material policy change.** `policy_matches` in
   `src/research/run.rs` compares `correctness_commands` as exact strings, so a
   moved admission path invalidates the run. Separate administrative changes
   from changes to horizon, workload, solver or acceptance rule (section 4).
4. **Relevant-input boundary for accepted code.** `accepted_code_matches`
   compares every path outside `.zdev` with the baseline revision, so a docs
   edit invalidates it. Use the experiment's declared relevant inputs when it
   declares them; stay conservative otherwise (section 5).
5. **Declared output boundaries and compact inspection.** Let source-ownership
   checks honor declared generated-output paths. Give routine run inspection a
   short default view, with full detail on request (section 2).
6. **Limit provenance.** Report each limit's origin (explicit user constraint,
   product default, coordinator estimate). A time-based authorization must not
   silently become a short dispatch-count run; count dispatches separately from
   experiments (section 3).
7. **Report research progress.** Templates report questions answered, valid
   comparisons, negative findings and the current accepted result separately
   from engineering progress (section 8).

## Boundaries and non-goals

- Never silently ignore tracked changes or undeclared untracked source. Never
  treat a directory as disposable because of its name.
- Never accept an invalid measurement, erase a failed attempt or reset counts to
  recover allowance. Reanalysis must not retroactively accept an invalid run.
- Freezing the comparison policy during a study stays. A material experimental
  change still needs a declared transition and a new measured baseline.
- Out of scope for now: a shared task packet or generated envelopes and a
  reusable bounded command runner (section 7). Revisit after these items land.
- Reservation accounting: zdev's `settle` already charges actual elapsed time
  and charges the reserved ceiling only for unknown cost after a crash. The
  1,700 vs 51.9 CPU-second gap in section 3 likely came from campaign-side
  envelopes. Confirm before changing settlement; do not change it on the
  report's word alone.

## Testing

Focused coverage. Research tests live in `tests/lean.rs` and use scripted
workers; they never invoke an LLM or a live harness. Add one regression per
changed boundary, following the three existing representative research
scenarios. Do not add a fault-injection matrix. Template-only changes (item 7)
need no new tests beyond the existing template and generated-fixture checks.

## Validation

- `cargo fmt --all -- --check`
- `cargo clippy --locked --all-targets --all-features -- -D warnings`
- `cargo test --locked`
- `cargo build --locked`
- `git diff --check`

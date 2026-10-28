+++
schema_version = 1
id = "cross-area-007"
key = "research-publication-dependent-indexes"
area = "cross-area"
status = "open"
complexity = "advanced"
afk = true
priority = "normal"
blocked_by = []
+++
# Publish research completions that other areas depend on

## Outcome

Completing a hypothesis task through `zdev research experiment ... finalize` or `recover` succeeds when a task in another area lists it in `blocked_by`. The regenerated TASKS.md of each dependent area is published, committed and recovered together with the experiment's own records.

## Context

Found while planning `refresh-dependent-indexes` (cross-area-003). Since that task, `complete_with_checkpoint` adds every dependent area's TASKS.md to the checkpoint list. Research publication only accepts records the experiment owns: `prepare_publication` in `src/research/experiment.rs` (around line 739) rejects any path outside `checkpoint.records`, which holds exactly the area's task file, TASKS.md and area.toml (enforced around line 387). So a hypothesis task that another area depends on fails at finalization with 'Publication includes an unowned experiment record'. This happens before anything is written, and every retry or recovery fails the same way. Before cross-area-003 the same case completed but left the dependent index stale.

The verifier of cross-area-003 confirmed the failure writes nothing. Supporting the case means extending record ownership to the dependent TASKS.md paths and keeping them consistent through `record_hashes` in `recover_publication`, the commit-tree paths, and the isolated-area TASKS.md handling. Start by reading `prepare_publication`, `recover_publication` and the publication commit path in `src/research/experiment.rs`, plus `read_dependent_areas` and `render_dependent_indexes` in `src/tasks.rs`.

## Boundaries

- Do not loosen ownership for any other path: only the dependent TASKS.md files that `render_dependent_indexes` reports for this status change may join the publication.
- Interrupted publication must stay recoverable: a crash after some writes recovers to either the old or the new state of every owned record, including dependent indexes, as today's records do.
- No change to ordinary task done/reopen behavior from cross-area-003.

## Done when

- [ ] Finalizing a hypothesis whose task is a cross-area blocker succeeds, and its commit contains the dependent area's regenerated TASKS.md; `zdev check` passes afterwards.
- [ ] Recovering an interrupted publication of such a task completes or rolls back the dependent index together with the experiment's own records.
- [ ] A publication that would include any other unowned path is still refused with the existing error.

## Validation

- Add one focused research test in tests/lean.rs that creates a cross-area dependent on a hypothesis task, finalizes it, and checks the commit and `zdev check`; extend an existing recovery test or add one interrupted-publication case. Use the scripted research fixtures; no live harness.
- cargo fmt --all -- --check; cargo clippy --locked --all-targets --all-features -- -D warnings; cargo test --locked; cargo build --locked; git diff --check

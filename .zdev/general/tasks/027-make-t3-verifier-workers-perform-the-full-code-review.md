+++
schema_version = 1
id = "general-027"
key = "t3-verifier-performs-full-review"
area = "general"
status = "done"
complexity = "routine"
afk = true
priority = "high"
blocked_by = []
+++
# Make T3 verifier workers perform the full code review

## Outcome

Under T3 orchestration, every task verifier runs the same full code review as the native route: the verifier payload and the T3 Verifier role require **Full review** in review.md, including the explicit naming and decomposition check, in addition to checking the task's requirements and validation.

## Context

On 2026-10-28 the maintainer asked that the T3 reviewer (by default Codex Sol at high effort, from general-024) be tasked with review, not just verification, as zdev originally intended. The native route requires it: templates/zdev/references/verify.md (around line 60) and templates/zdev/task-workflows.md (around line 59) require the explicit check in **Full review**, and the Claude verifier agent template says the same. The shared contract applies under T3 too (templates/zdev/task-workflows.md lines 53-60 require **Full review**, and src/integrations.rs around line 1033 injects the review guidance), but the T3 reference never says so for its own role: the **Verifier** bullet in **Worker roles** in templates/zdev/references/t3-code.md (around line 332) says only to load the snapshot, 'inspect the entire task and diff on initial review' and run checks, and points to review.md only for a focused recheck. A Codex verifier dispatched under T3 that follows its role section can therefore stop at checking done conditions. This task makes the requirement explicit in the T3 role and payload.

## Boundaries

- Change only T3 wording; the native verify contract already requires the full review.
- Keep the four-field verifier result schema and the focused-recheck rules unchanged.
- Edit templates and regenerate checked-in copies with the repo-built binary; never hand-edit generated files.

## Done when

- [x] The T3 Verifier role and the T3 verifier dispatch payload description require reading verify.md and performing **Full review** in review.md, with the explicit naming and decomposition check, on every initial task review, and say that verification of done conditions alone is not a pass.
- [x] Generated copies match the templates and `zdev skill check` reports every harness ready.

## Validation

- Extend the existing T3 reference rendering assertion in tests/lean.rs to check the Full review requirement. No live harness run.
- cargo fmt --all -- --check; cargo clippy --locked --all-targets --all-features -- -D warnings; cargo test --locked; cargo build --locked; git diff --check

## Result

The T3 Verifier role and the verifier dispatch payload now require verify.md and Full review in review.md, including the explicit naming and decomposition check, on every initial task review; verification of done conditions alone is not a pass. Focused rechecks, the result schema, audit and standalone review are unchanged. Verified by GPT-6 Astra (high).

Validation:

- cargo fmt --all -- --check: pass
- cargo clippy --locked --all-targets --all-features -- -D warnings: pass
- cargo test --locked: pass (lean 208)
- cargo build --locked: pass
- git diff --check: pass
- zdev skill check (codex portable, claude, opencode, pi, omp): ready

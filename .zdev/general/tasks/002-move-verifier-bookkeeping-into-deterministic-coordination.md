+++
schema_version = 1
id = "general-002"
key = "move-verifier-bookkeeping-to-coordinator"
area = "general"
status = "done"
complexity = "advanced"
blocked_by = ["general-003"]
+++
# Move verifier bookkeeping into deterministic coordination

## Outcome

Verification workers spend their response on semantic findings while zdev coordinators own snapshot identity, state comparison, advisory attachment, and strict transport validation.

## Context

The current verifier contract asks the model to produce and count a nine-key envelope, one snapshot evidence item, an optional advisory exactly once, and exact compact comparison shapes. Refactor the canonical task workflow and the Claude, Codex, OpenCode, Pi, and Oh My Pi adapters around the existing work-context snapshot commands: immediately before each fresh verifier dispatch, the coordinator stores and validates the immutable snapshot; the verifier reads that supplied snapshot, checks the complete task, and runs validation; after the verifier returns, the coordinator compares the checkout with the snapshot, refuses PASS on mismatch, routes attributable validation writes to rework and ambiguous writes to blocker, and reconstructs the existing public result. Relevant seams include templates/zdev/references/task-workflows.md, verify.md, implement.md, area-loop.md, the harness verifier prompts and Claude workflows, generated skills, and workflow fixtures in tests/lean.rs.

## Boundaries

- Preserve the existing public nine-key verifier envelope for compatibility: coordination supplies schema_version, kind, area, task_id, and coordinator-generated snapshot/advisory evidence around validated semantic fields.
- The verifier returns exactly the JSON shape {"verdict":"<verdict>","summary":"<non-empty summary>","findings":[],"escalation":"<escalation>"}. The serialized verdict literals are "pass", "rework", and "blocker"; escalation is always present and is "none" except that "rework" may use "advanced-implementer". Findings is an array of non-empty strings: pass has none, rework has at least one, and blocker may have findings.
- Preserve fresh independent semantic verification, whole-task checking, required validation, Git ownership gates, rework routing, escalation limits, and completion-before-commit safety.
- Keep malformed model output fail-closed, but enforce shapes and generated metadata in code or harness structured-output facilities rather than asking the verifier to count them.
- Do not add a new workflow engine, durable verifier state, provenance mechanism, retry policy, or broader envelope redesign for planners and implementers.

## Done when

- [x] The verifier-facing response is the exact four-field semantic object, while deterministic coordination reconstructs the compatible nine-key public envelope and checks every generated field.
- [x] Standalone verify, implementation verification, rework, and goal-loop routes store and validate the snapshot immediately before every verifier dispatch, supply it to the verifier, and require the coordinator's unchanged post-validation comparison before accepting PASS.
- [x] Post-validation changes cannot pass: attributable task-owned writes route to REWORK and ambiguous writes route to BLOCKER under the existing ownership rules.
- [x] All five installed harness integrations express the same responsibility split and generated fixtures match their canonical templates.
- [x] Focused workflow coverage proves PASS, REWORK, BLOCKER, escalation, malformed semantic output, validation-written state, snapshot mismatch, public-envelope reconstruction, and advisory handling without relying on verifier item counts.

## Validation

- Regenerate the checked-in integrations with `cargo run --locked -- skill install codex --to skills --force`, `cargo run --locked -- skill install claude --to .claude/skills/zdev --force`, and the corresponding `opencode --to .opencode`, `pi --to .pi`, and `omp --to .omp` commands.
- Run `cargo test --locked --test lean all_harness_task_workflows_are_discoverable_and_keep_coordinator_boundaries`, `cargo test --locked --test lean claude_task_workflows_reject_incomplete_or_mismatched_structured_envelopes`, `cargo test --locked --test lean claude_standalone_verify_returns_only_valid_snapshot_locators`, `cargo test --locked --test lean work_context_round_trip_counts_match_realized_routes`, and `cargo test --locked --test lean executable_templates_realize_deterministically_and_match_generated_fixtures`.
- Run the area-wide validation from brief.md.

## Result

Moved verifier bookkeeping into deterministic coordination while preserving the compatible public envelope and fail-closed state ownership.

Validation:

- Fresh independent verification passed. All five focused workflow checks and full area validation passed: cargo fmt --check, clippy with -D warnings, cargo test --locked (140 tests), cargo build --locked, fixture regeneration equality, and git diff --check. Snapshot W1d4d88ca65468a63 compared equal before completion.

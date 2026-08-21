+++
schema_version = 1
id = "improvements-033"
key = "typed-worker-results"
area = "improvements"
status = "done"
blocked_by = []
+++
# Simplify worker results to one typed envelope

## Outcome

Implementer and verifier handoffs use one strict machine-readable result instead of sentinel lines plus duplicated identity fields.

## Context

Replace the current sentinel-plus-body protocol with one strict discriminated JSON object. Required keys are `schema_version`, `kind`, `area`, `task_id`, `verdict`, `summary`, `evidence`, `findings`, and `escalation`. `kind` is implementer or verifier; implementer verdict is ready or blocker; verifier verdict is pass, rework, or blocker. Arrays are present even when empty. `escalation` is none except that verifier rework may request advanced-implementer.

## Boundaries

- Require schema_version 1 and exact area/task identity; reject missing, unknown, or duplicate keys and all text outside the single JSON object.
- Evidence and findings are arrays of non-empty strings; summary is non-empty.
- Only verifier REWORK may set escalation to advanced-implementer; every other combination requires none.
- Keep user decisions, lifecycle mutation, staging, and commit authority with the coordinator.
- Do not add provenance, signatures, session identifiers, retries, or durable worker state.

## Done when

- [x] Canonical implement and verify guidance defines and uses the exact discriminated object without sentinel lines or repeated free-form identity fields.
- [x] All five harness adapters parse or instruct the same semantic envelope using their native facilities.
- [x] Fresh-verifier identity checks and PASS/REWORK/BLOCKER meaning are unchanged.
- [x] Malformed, mismatched, duplicate-key, extra-text, and contradictory results are rejected deterministically.

## Validation

- Add focused parser/contract fixtures for PASS, REWORK, escalation, BLOCKER, unknown fields, and identity mismatch.
- Regenerate and check all harness artifacts.
- Run the area-wide validation from brief.md.

## Result

Replaced task-worker sentinel handoffs with one strict nine-key typed JSON result across all five harnesses, including Claude parsing and in-memory rework evidence.

Validation:

- Focused typed-envelope, history, escalation, harness, and generation tests passed; all five harness checks, full 104-test suite, formatting, strict Clippy, build, diff check, and fresh independent verification passed.

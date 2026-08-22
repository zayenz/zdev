+++
schema_version = 1
id = "improvements-050"
key = "trunk-area-schema"
area = "improvements"
status = "done"
blocked_by = []
+++
# Add explicit trunk area metadata

## Outcome

An area can explicitly use the configured trunk branch without storing a duplicate branch name, while all legacy areas remain isolated.

## Context

Implement the schema foundation from docs/trunk-area-mode.md. Optional `mode = "trunk"` follows project.trunk dynamically and forbids branch, parent, and base_commit. Missing mode or `isolated` preserves current branch-owned behavior.

## Boundaries

- Legacy records remain isolated and byte-stable.
- Multiple areas may share a branch only when every sharer is explicitly trunk mode and the branch equals configured project.trunk.
- Do not infer trunk mode from a stored branch matching trunk.
- Keep schema version 1 and reject contradictory fields strictly.

## Done when

- [x] Area parsing, creation, show, list, status, and check support explicit trunk mode and legacy isolated mode.
- [x] Trunk areas resolve their current branch from project.trunk.
- [x] Invalid trunk metadata and isolated/trunk ownership collisions fail with actionable errors.
- [x] Multiple explicit trunk areas coexist deterministically.

## Validation

- Add focused parser, legacy compatibility, creation, collision, and multi-area tests.
- Run the area-wide validation from brief.md.

## Result

Added explicit schema-v1 trunk area metadata with dynamic project.trunk resolution, legacy isolated compatibility, and strict ownership validation.

Validation:

- Independent verifier PASS; focused parser/create/collision/projection tests and full fmt, clippy, test, build, and diff checks passed.

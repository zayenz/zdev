+++
schema_version = 1
id = "improvements-055"
key = "filesystem-review-artifacts"
area = "improvements"
status = "done"
complexity = "standard"
blocked_by = []
+++
# Persist and present task review artifacts

## Outcome

Task-bundle review lives in repository-local filesystem state and can be shown and imported through zdev without reconstructing large Markdown or transporting its fingerprint.

## Context

Zdev 1.0 and 1.1 return the complete review Markdown through command output. Large bundles can be truncated by an agent tool, causing wasteful byte-for-byte reconstruction. Store one current review per area in Git administrative state so it remains untracked, atomically replaceable, and directly presentable by zdev.

## Boundaries

- Store the canonical bundle, rendered Markdown, and fingerprint under repository-local Git administrative state, not tracked .zdev state.
- Keep one current review per area; a new review atomically replaces it.
- Preserve direct bundle import for compatibility.
- Do not require the user or coordinator to copy, compare, reconstruct, or reason about the fingerprint.
- Keep initial approval readable; corrected bundles may be presented from the stored artifact without replaying bytes through model context.

## Done when

- [x] zdev tasks review writes the current area review artifact and returns small identity/path metadata instead of embedding the full Markdown in JSON.
- [x] zdev can show the stored review Markdown on demand.
- [x] zdev can import the stored reviewed bundle directly while retaining drift validation and committed-import behavior.
- [x] Missing, mismatched, corrupt, replaced, and cross-area artifacts fail clearly without task publication.
- [x] Canonical guidance and user documentation use the filesystem artifact and no longer instruct agents to reconstruct or transport the review document or fingerprint.

## Validation

- Add focused black-box tests for review storage, show, replacement, reviewed import, failure cases, and linked-worktree-safe Git paths.
- Run the area-wide validation from brief.md and regenerate affected harness artifacts.

## Result

Persisted task-bundle reviews as repository-local Markdown and canonical bundle artifacts that zdev can show and import without reconstruction or fingerprint transport.

Validation:

- Independent verifier PASS; focused review-store, replacement, linked-worktree, reviewed-import, corruption, compatibility, generation, package, and full validation passed.

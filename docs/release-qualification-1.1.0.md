# Zdev 1.1.0 release qualification

Date: 2026-08-22

This record qualifies the prepared source tree. It does not claim provider-wide
compatibility or replace the automated release checks. No command published,
pushed, tagged, or contacted a repository remote.

## Environment

- macOS, Rust 1.97.1, Cargo 1.97.1, cargo-dist 0.32.0
- Codex CLI 0.145.0 using GPT-5.6 Sol
- Claude Code 2.1.227, installed but not logged in
- OpenCode 1.17.2 using MiniMax-M2.7
- Pi and Oh My Pi were not installed

Each runnable harness received a fresh temporary Git repository containing
four explicit trunk areas: one ready implementation, one known defect for
read-only verification, one closed empty area, and one ready loop area. The
installed 1.1.0 integration and release binary were used. Both repositories
were clean after qualification.

## Automated qualification

A clean committed copy of the prepared diff passed:

```text
scripts/check-release.sh v1.1.0
```

That gate ran locked formatting, strict Clippy, all tests, source packaging and
verification, release installation, the standalone binary round trip,
five-harness project install/check, release-workflow synchronization, and
`dist plan --tag=v1.1.0`. The package contained 109 files, including the Codex
manifest, changelog, this record, canonical templates, and generated skills.
Cargo metadata, lockfile, release binary, Codex manifest, rendered Claude
manifest, package ID, and dist plan all reported 1.1.0. Generated-fixture
equality passed in the locked test suite.

## Actual harness results

| Harness | Ready / PASS | REWORK and verify-only | Closed no-work | Goal / loop | Small audit |
| --- | --- | --- | --- | --- | --- |
| Codex CLI 0.145.0 | PASS. A fresh implementer created the one owned file, a fresh verifier passed, and the coordinator completed and committed exactly the source file, task record, and `TASKS.md`. | PASS. Explicit verify returned a valid typed `rework` envelope for the deliberately absent file and did not mutate the repository. | PASS before Git gates; no worker or commit. | BLOCKER before repository work. This `codex exec` surface exposed no model-callable `get_goal`, so the adapter could not prove native goal state was clear. | PASS. One narrow structural lens and a different fresh final vetter found no task-record defect. |
| OpenCode 1.17.2 | Not reached. | Not reached. | Preflight projection was read successfully, but the route was not completed. | Not reached. | Not reached. |
| Claude Code 2.1.227 | Unavailable: the installed CLI reported `Not logged in`. | Unavailable | Unavailable | Unavailable | Unavailable |
| Pi | Unavailable: executable not installed. | Unavailable | Unavailable | Unavailable | Unavailable |
| Oh My Pi | Unavailable: executable not installed. | Unavailable | Unavailable | Unavailable | Unavailable |

The OpenCode bounded run loaded the installed zdev skill, but an embedded
`/zdev-verify` request was not expanded as a custom command by noninteractive
`opencode run`. The model then tried the nonexistent binary subcommand
`zdev verify`. The run was stopped after this one attempt; it changed no files.
This is evidence about that noninteractive invocation, not an assertion that
the interactive custom-command UI is broken.

Codex initially rejected one inherited-thread verifier spawn in ephemeral mode
with `no thread with id`. The contract's context-free retry succeeded, after
which REWORK, PASS, closed no-work, and audit completed. The CLI also emitted a
local model-cache warning. Neither issue changed the source repository.

## Reproduction boundary

The manual fixtures used the release binary on `PATH`, installed the harness at
project scope, and invoked the five routes in this order:

```text
zdev-verify rework rework-001
zdev-implement pass
zdev-implement closed
zdev-loop loop
zdev-audit <one task-record lens>
```

The known-defect task required a missing `VERIFIED.txt`, so a correct read-only
verifier had a concrete REWORK finding. The PASS task owned only `PASS.txt` and
required the exact bytes `pass\n`. The closed area had no tasks. The audit was
limited to four briefs, four `TASKS.md` files, and three task records.

The unavailable and blocked cells should be revisited when those authenticated
harness surfaces are available. They are not silently counted as passes.

Two narrow follow-ups are warranted: verify whether Codex CLI sessions are
expected to expose the model-callable goal operations used by the adapter, and
document or add a supported noninteractive OpenCode custom-command invocation
for future release qualification. This release task does not infer runtime
changes from either single-surface observation.

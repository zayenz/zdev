+++
schema_version = 1
id = "improvements-037"
key = "config-refresh-hint"
area = "improvements"
status = "done"
blocked_by = []
+++
# Report integration refresh after worker config changes

## Outcome

Worker-profile config mutations tell the caller exactly when installed integrations need refresh.

## Context

Config set and unset change the worker values used to realize harness artifacts, but the current result gives no next action. Add a concise result field and human hint that points to the existing install/check command without performing publication.

## Boundaries

- Only worker-profile mutations report the refresh requirement.
- Do not make config mutation install, check, or rewrite integrations.
- Keep the existing lock, validation, atomic write, and output fields.

## Done when

- [x] Successful worker-profile set and unset results include a stable integration_refresh_required value and the appropriate existing refresh command.
- [x] Unrelated config mutations do not claim a refresh is needed.
- [x] Human output communicates the same fact in one concise line.

## Validation

- Add focused set/unset JSON and human-output coverage without duplicating config persistence tests.
- Run the area-wide validation from brief.md.

## Result

Added scope- and harness-specific integration refresh hints to successful worker-profile set and unset results without publishing integrations.

Validation:

- Focused worker/project mutation and lock-failure checks, full 106-test suite, formatting, strict Clippy, build, diff check, and fresh independent verification passed.

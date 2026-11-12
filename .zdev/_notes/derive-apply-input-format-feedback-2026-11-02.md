# `tasks derive apply --from`: input format feedback

2 November 2026. Claude Code (Opus 5.5) inside T3 Code, macOS. Gecode
repository, `feature/breaking-7`, boost area, investigation boost-001.

## What happened

The implementer returned the investigation_follow_up proposal as the contract
says: one evidence string, `PROPOSE zdev-derived boost boost-001\n` followed by
one JSON object. The coordinator saved the JSON object alone, pretty-printed,
to a file and ran `zdev tasks derive apply boost --from <file> --format json`.

It failed with:

    Invalid derived proposal: invalid type: string "schema_version",
    expected struct DerivedProposal at line 1 column 18

The same object, prefixed with the `PROPOSE` header line and compacted to one
line, applied cleanly.

## Suggestions

- Say in `derive apply --help` and `derive review --help` that `--from`
  expects the complete evidence string, header line included.
- Or accept a bare JSON object as well, since the header only repeats the
  area and source task that the command already receives or can read from
  the object.
- When the header is missing, report "expected `PROPOSE zdev-derived <area>
  <task>` on the first line" instead of a serde type error.

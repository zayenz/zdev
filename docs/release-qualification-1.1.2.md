# Zdev 1.1.2 release qualification

Date: 2026-08-24

This record qualifies candidate revision
`46ba90b1e69c8b79a5cbdab5fedc3156c82d1ee6`. The checkout was clean before
and after the automated gate. No command in this qualification tagged, pushed,
published, or contacted a repository remote.

## Environment

- macOS 26.5.2 (build 25F84), arm64
- Rust 1.97.1 and Cargo 1.97.1
- cargo-dist 0.32.0, invoked through the `dist` executable pinned by
  `dist-workspace.toml`

## Automated qualification

The clean committed candidate passed:

```text
scripts/check-release.sh v1.1.2
```

The gate confirmed that Cargo metadata reports 1.1.2 and ran formatting,
Clippy with warnings denied across all targets and features, and the locked test
suite. The suite passed 3 library tests, 1 documentation contract test, and 137
black-box tests, with no failures. It then packaged and verified 104 source
files, installed the release build, and passed the standalone release smoke
round trip.

The smoke check installed and checked the generated project integrations for
Codex, Claude Code, OpenCode, Pi, and Oh My Pi. Release workflow synchronization
passed, and cargo-dist successfully planned `v1.1.2` with local paths disabled.
The gate ended with `release check passed: v1.1.2` and left the checkout clean.

## Focused workflow evidence

The locked suite exercised the 1.1.2 worker-contract changes. It confirmed
that:

- Claude Code and Oh My Pi planners accept one constrained four-field semantic
  result without a formatting retry, and malformed, unknown, or duplicate keys
  fail closed;
- all harnesses render the same semantic planner contract and reconstruct the
  compatible public planner envelope in coordination;
- verifier workers do not own snapshot bookkeeping, while coordinators store,
  compare, and reconstruct the public verifier result; and
- Claude Code uses its inline rendered contract when `CLAUDE_PLUGIN_ROOT` is
  absent or unusable and avoids redundant full-context verification calls.

Generated fixtures matched their canonical templates after these changes.

## Limitations

No external interactive harness session was run for this patch release. The
integration evidence is limited to generated installation, synchronization,
and executable workflow tests in the automated suite and release smoke check.

No release archive was built on a release runner, no tag was created, and
nothing was pushed or published. The coordinator will repeat the clean-tree
release gate after this documentation commit before treating the final release
revision as ready to tag.

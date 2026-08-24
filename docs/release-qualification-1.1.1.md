# Zdev 1.1.1 release qualification

Date: 2026-08-24

This record qualifies candidate revision
`979399f21a9108ac53dad2f1b99d0d56c4cc389b`. The checkout was clean before
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
scripts/check-release.sh v1.1.1
```

The gate confirmed that Cargo metadata reports 1.1.1 and ran formatting,
Clippy with warnings denied across all targets and features, and the locked test
suite. The suite passed 3 library tests, 1 documentation contract test, and 136
black-box tests, with no failures. It then packaged and verified 103 source
files, installed the release build, and passed the standalone release smoke
round trip.

The smoke check installed and checked the generated project integrations for
Codex, Claude Code, OpenCode, Pi, and Oh My Pi. Release workflow synchronization
passed, and cargo-dist successfully planned `v1.1.1` with local paths disabled.
The gate ended with `release check passed: v1.1.1` and left the checkout clean.

## Focused initial-import evidence

The locked suite exercised the improvements-064 import policy through focused
black-box tests. It confirmed that:

- project records publish the complete first planning record, including the
  exact config, area, brief, slice, task, and generated index paths;
- tracked and untracked project configuration layouts and pull-request records
  are accepted, while personal records remain local and reject explicit managed
  commits before publication;
- unexpected or partially tracked area state is rejected without publishing a
  task; and
- a rejected Git commit removes the created task, restores the previous
  `TASKS.md` and index state, preserves an unrelated staged change, and creates
  no partial commit.

The candidate contains the v1.1.0 commit
`dc20cf154fef07596676f2ee9965a6ad0a7c255d`, the improvements-064 task record
commit `3d729ffb0a4975fbb51fa77949bf73f73b4352d1`, and its implementation commit
`dbf61f09c890e0c41ce53f104b2897e2408a1a0d` in its ancestry.

## Limitations

No external harness session was run for this patch release. In particular, this
record does not repeat the Codex, Claude Code, OpenCode, Pi, or Oh My Pi manual
observations from the 1.1.0 qualification. The integration evidence above is
limited to generated project installation and synchronization checks performed
by the automated release smoke test.

No release archive was built on a release runner, no tag was created, and
nothing was pushed or published. The coordinator will repeat the clean-tree
release gate after the documentation commit before treating final main as ready
to tag.

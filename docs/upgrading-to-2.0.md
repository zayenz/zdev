# Upgrade to zdev 2.0

Zdev 2.0 makes coding guidance part of planning, implementation and independent
review and verification. Readability and repository standards contribute to
acceptance alongside the task's required behavior. Language and framework supplements apply to the
affected code, within explicit user requirements and project instructions.

Existing 1.x areas, briefs, tasks and worker configuration remain valid. The
record schema stays at version 1; there is no record migration or reinitialization.

## Refresh the binary and integrations

Install the 2.0 binary using your existing installation method. For Cargo:

```sh
cargo install zdev --version 2.0.0 --locked
```

Then refresh each binary-installed harness integration with the same scope and
destination:

```sh
zdev skill install codex --force
zdev skill check codex

# For an existing project installation:
zdev skill install codex --scope project --guidance auto --force
zdev skill check codex --scope project
```

Replace `codex` with your harness. Preserve an existing custom `--to` destination
and guidance selection. Updating the binary alone does not update installed
instructions, prompts or workflows.

The Codex plugin contains a portable skill bundle. Update the plugin through
its normal installation path. Plugin references resolve from the loaded skill
directory; they do not depend on the maintainer's checkout.

## Review and continuation changes

A separate agent reviews the implementation and verifies it against the task
requirements and repository standards. Reviews separate required corrections
from optional suggestions. An ordinary task receives one full review, one correction pass and one focused recheck.
Recovery carries the review stage and prior findings forward. If required
corrections remain after the recheck, the task stays open and the harness asks
for an explicit continuation decision. Research retains its own attempt budgets.

Explicit blanket approval for compatible parallel batches continues within the
approved area, work, worker limit, destination, worktree and cleanup choices.
Every batch still selects from fresh tasks and rechecks admission. Approval of
one batch alone does not authorize another; a choice outside blanket authority
still needs a decision. Harness continuation limits remain in effect.

## Execution defaults and delegated work

Implementation reuses the workspace you explicitly selected. In 2.0, the
harness otherwise used an isolated worktree as its authoritative checkout and
reported where to inspect the result. Later releases implement in the current
checkout by default; set `workspace.branching` to `isolated` to restore the
worktree default (see the [configuration reference](config-command.md)). Read-only exploration starts in the current
checkout. Parallel work defaults to four task slots within available capacity;
destination acceptance and commits remain serial.

An explicit delegation to create and import tasks remains valid within its
scope. Without that authority, the harness presents the final reviewed bundle
for approval. Notes under `.zdev/_notes/` capture work before it becomes an
area. Several areas may share a branch; task dependencies still stay within
their own area.

The normal OpenAI worker profile now uses GPT-6.1 Sol, with high reasoning for
standard implementation and verification. This can change model usage from
older defaults. The explicit simple profile and worker overrides remain
available. Existing saved choices continue to take precedence as documented
in [worker profiles](worker-profiles.md).

Research runs can extend explicitly granted allowances with `research run
<area> extend`, preserving the run and its costs. Stop/resume alone retains the
existing limits; see [research](research.md#extend-an-authorized-run).

## Research records after cloning

Task inspection, status and record validation remain available when local
experiment refs are absent. Before admitting work with `requires_code`, zdev
checks the prerequisite's verified candidate. It can resolve the recorded commit
directly when ordinary branch history already contains it.

Isolated candidate commits may be retained only in the originating clone's
`refs/zdev/experiments/` namespace. If admission reports missing retained code,
fetch the named ref from that clone into the same local namespace:

```sh
git fetch /path/to/originating-clone \
  refs/zdev/experiments/work/work-001/1:refs/zdev/experiments/work/work-001/1
```

Use the ref named in the error. Zdev preserves inspection and gives this recovery
instruction rather than silently treating unavailable code as admitted work.

Older research commits that used `Change-Id` remain ordinary Git history. New
research commits use `Zdev-Change-Id` and work with `zdev change inspect` and
`zdev change lookup`. The upgrade does not rewrite existing commits.

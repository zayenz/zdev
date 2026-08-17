# Zdev user guide

Zdev keeps development work in small Markdown tasks and leaves agent execution
to your coding harness. This guide takes one repository from installation to a
completed zdev task.

## 1. Install zdev

Versioned binaries are available from the
[latest GitHub release](https://github.com/zayenz/zdev/releases/latest). Follow
the release notes for the binary installation steps.

For installation from source with a Rust toolchain, use Cargo:

```sh
cargo install --git https://github.com/zayenz/zdev --locked
```

From a local zdev checkout, use:

```sh
cargo install --path . --locked
```

Check the installation:

```sh
zdev --version
```

## 2. Check or install a user integration

Check the user-scoped integration for your harness:

```sh
zdev skill check codex --scope user
```

Replace `codex` with `claude`, `opencode`, `pi`, or `omp`. Status `ok` means the
integration is current. For `missing` or `conflict`, install or refresh it:

```sh
zdev skill install codex
```

User-scoped integrations work across repositories and do not require zdev to be
initialized in the current project.

## 3. Initialize a repository

On the trunk branch, choose one zdev planning-record policy:

- Choose **personal** for a parallel record used only in this clone. Add the
  exact entry `/.zdev/` to `.git/info/exclude`; it stays local and will not
  travel to another clone or collaborator.
- Choose **project** when `.zdev` should be portable, reviewed, and shared. Leave
  it visible to Git and commit it with the repository.
- Choose **pull-request** when `.zdev` should be tracked and reviewed on the
  feature branch but omitted from the final squash-merged tree. Leave it
  visible to Git, commit it during review, and clean it before squash merge.

Integration scope is separate from record policy. After making the record
choice, run:

```sh
zdev init --record personal # or: project, pull-request
```

Zdev records the current branch as trunk. If you need to correct it, run
`zdev config trunk <branch>`.

If you prefer a checked-in integration, install it now that the repository is
initialized:

```sh
zdev skill install codex --scope project --guidance auto
```

Replace `codex` with `claude`, `opencode`, `pi`, or `omp`. Project installation
puts harness-native files under `.codex`, `.claude`, `.opencode`, `.pi`, or
`.omp`. The `--guidance auto` option uses a root `AGENTS.md` or creates
`.zdev/guidance.md`; you can instead pass `agents`, `zdev`, or a
repository-relative Markdown path. Edit that source, then refresh and check the
integration:

```sh
zdev skill install codex --scope project --force
zdev skill check codex --scope project
```

For a pull-request record, run `zdev cleanup squash` on the clean feature branch
immediately before squash merge. It deletes only tracked `.zdev` files and makes
one plain Git commit without a `Zdev-Change-Id`. It refuses missing or different
record policies, configured trunk, detached HEAD, in-progress Git operations,
local changes, and branches with no tracked `.zdev` files.
This prepares only the final tree. A normal merge or rebase that retains the
feature commits also retains `.zdev` in reachable history; `cleanup squash` does
not implement history-preserving cleanup.

## 4. Create an area

An area groups one objective and its tasks. Create its feature branch first:

```sh
git switch -c scheduling
zdev area create scheduling \
  --title "Scheduling support" \
  --objective "Add a tested scheduling API."
```

Zdev creates this structure:

```text
.zdev/scheduling/
  area.toml
  brief.md
  TASKS.md
  tasks/
```

Write shared decisions in `brief.md`. If the work starts with substantial
source material, keep it as separate files under `background/` and link them
from the brief. Each task remains the source of truth for its own outcome,
boundaries, and done conditions.

## 5. Explore and discuss the objective

Zdev is the top-level trigger for its harness workflow. Mention `zdev`,
`$zdev`, or the existing `.zdev` area when asking for help. Generic requests such as
“explore this idea” or “review this repository” do not recruit zdev by
themselves.

Start by asking the harness to use zdev to explore the objective:

```text
Use zdev to explore the scheduling objective and build up its brief.
```

**Explore an objective** inspects the repository, compares useful paths, and
builds or revises `brief.md`. `wayfind` and `shape` are aliases after zdev is
active. For non-trivial work, the normal next step is:

```text
Use zdev to discuss the scheduling brief before we create tasks.
```

**Discuss the brief** reads the brief and relevant indexed sources, then
identifies choices that could materially change behavior, scope, task splitting,
or validation. It resolves repository facts directly and works breadth first
across the highest-impact choices. Each round asks up to three independent
questions, using the harness's structured question tool when available. It asks
one question when that answer determines what to ask next or when you need to
explain freely. Discussion tests settled decisions against concrete scenarios,
updates the brief after each round, and stops when no unresolved choice could
materially change the work. `grill` is an alias.

Other active zdev intents route directly to **Improve**, **Investigate**,
**Create tasks**, **Implement**, or **Verify**. After each interaction, zdev
reports the result and relevant next actions. You can explicitly order several
interactions in one message—for example, “approve this exact bundle and then
implement.” Zdev then imports the unchanged approved bundle, validates it, and
starts the next ready task if every implementation gate passes. Approval applies
only to the artifact shown and does not imply another action.

## 6. Import reviewed tasks

Your harness sends the proposed Task Bundle JSON to zdev for deterministic
rendering:

```sh
zdev tasks review scheduling --from - --format json
```

The result contains the complete readable Markdown and an approval fingerprint.
After you approve the Markdown, the harness sends the unchanged JSON and that
fingerprint to import:

```sh
zdev tasks import scheduling --from - --approval <approval-id>
```

Zdev rejects content that differs from the reviewed bundle. For manual input,
paste the same JSON into each command and press Ctrl-D. `--from
path/to/tasks.json` also works and leaves the source file in place. Zdev writes
one Markdown file per task and regenerates `TASKS.md`. Edit task files, not the
generated index.

When adding tasks to an existing task list, commit them directly:

```sh
zdev tasks import scheduling --from - --approval <approval-id> --commit --format json
```

Use ordinary import for the initial task split or when you explicitly want the
additions left uncommitted. A committed import contains only the new task files
and regenerated `TASKS.md`; unrelated staged and unstaged changes are
preserved. The JSON result includes task IDs, paths, the commit hash, and the
stable change ID.

## 7. Run the task loop

Check the branch relationship and select the next ready task:

```sh
zdev status scheduling --format json
zdev next scheduling --format json
```

Ask the harness to work on the returned task with zdev. It should:

1. implement against the task, area brief, and repository guidance;
2. verify the result in a fresh, read-only context with separate specification
   and repository-standards checks;
3. fix concrete failures and verify again; and
4. stop for a real blocker or mark the task ready for completion after both
   checks pass.

After both checks pass, mark the task done and commit:

```sh
zdev task done scheduling scheduling-001 \
  --summary "Implemented and independently verified the scheduling model." \
  --validation "Focused model tests passed."

git add <changed-files> .zdev/scheduling
zdev commit -m "feat: add scheduling model"
```

`zdev commit` adds a stable `Zdev-Change-Id` trailer. Repeat `zdev next` until the
area is complete. Inspect or find a logical change after a rebase with:

```sh
zdev change inspect HEAD
zdev change lookup Z0123456789abcdef...
```

You can add tasks while this loop is running with the committed-import command
from step 6. New task-only commits do not interrupt the selected task. Finish
that task, then consider the additions at the next `zdev next`.

## 8. Keep the area current

Before selecting or completing work, `zdev status` should report a matching,
fresh, anchor-valid, and finalized branch relationship. If trunk advanced,
run this from the area branch:

```sh
zdev area rebase scheduling
```

If Git stops on a conflict, resolve and stage the files, then continue or
abort:

```sh
zdev area rebase scheduling --continue
zdev area rebase scheduling --abort
```

See the [workflow reference](workflow.md) for dependent areas, base anchors,
verification responsibilities, and recovery.

## Harness notes

### Codex

Codex uses collaboration agents and a fresh verification context. A project
integration lives under `.codex/skills/zdev`.

### Claude Code

Start Claude Code from the repository root and accept workspace trust for a
project installation. The integration provides scoped implementer and verifier
agents. Claude Code 2.1.154 or later can also use `/zdev:zdev-task` and
`/zdev:zdev-audit`.

### OpenCode

OpenCode installs its skill, agents, and `/zdev-task` and `/zdev-audit`
commands under `.opencode`. It discovers project skills when started from a
subdirectory in the worktree.

### Pi

Pi installs a skill, prompt templates, and the `zdev_subagent` extension under
`.pi`. The extension starts a fresh child Pi process for each implementation or
verification handoff. A user installation goes to `$PI_CODING_AGENT_DIR`, or
`~/.pi/agent` when the variable is unset.

### Oh My Pi

Oh My Pi is separate from plain Pi. It installs a skill and constrained native
task agents under `.omp` and uses OMP's built-in `task` and `hub` facilities.
A user installation goes to `$PI_CODING_AGENT_DIR`, or `~/.omp/agent` when the
variable is unset.

OMP 17.2.15 may find the skill but miss user task agents when
`PI_CODING_AGENT_DIR` relocates the user root. The install and check commands
warn about this. Unset the variable or use a project install under `.omp` until
upstream discovery is fixed.

## Get help

Run `zdev --help` or `zdev <command> --help`. The [task format](task-format.md)
documents every task field.

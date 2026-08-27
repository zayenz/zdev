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

Inspect the fixed project and worker registry with `zdev config show` or
`zdev config get <key>`. Use typed `config set` and `config unset` mutations;
worker changes report the exact integration refresh command instead of
rewriting installed files automatically. Run `zdev config --help` for the
supported keys, scopes, and value grammar.

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

An area groups one objective and its tasks. By default it owns an isolated
feature branch, which you create first:

```sh
git switch -c scheduling
zdev area create scheduling \
  --title "Scheduling support" \
  --objective "Add a tested scheduling API."
```

For a personal or project record, use `--trunk` when the area should explicitly
share configured project trunk with other trunk areas:

```sh
git switch main
zdev area create scheduling \
  --title "Scheduling support" \
  --objective "Add a tested scheduling API." \
  --trunk
```

Trunk mode follows later `project.trunk` configuration changes dynamically. It
does not weaken task ownership or stage unrelated trunk changes. Pull-request
records remain isolated.

Zdev creates this structure:

```text
.zdev/scheduling/
  area.toml
  brief.md
  TASKS.md
  tasks/
```

Write shared decisions in `brief.md`. During approved area shaping or an
authorized investigation task, reusable research may live as separate files
under `background/`. Keep only readable, stable, source-backed material that
later tasks need; index every file from the brief and link only relevant files
from tasks. Do not retain transcripts, raw tool or search dumps, repository
source copies, temporary prototypes, or lifecycle metadata. The brief remains
the authoritative synthesis, and each task remains the source of truth for its
own outcome, boundaries, and done conditions. A standalone investigation stays
report-only unless you ask to preserve its result.

For a larger area with several related increments, add lightweight slice
briefs as needed:

```sh
zdev slice create scheduling api \
  --title "Scheduling API" \
  --objective "Expose the scheduling model through a stable API." \
  --boundary "Keep persistence out of this slice."
zdev slice list scheduling
zdev slice show scheduling api
```

Slice files live under `.zdev/scheduling/slices/`. They contain an objective
and boundaries but no stored status. Tasks may name a slice, and zdev derives
its ready, blocked, and done counts from those tasks, including zeros for an
empty slice. Unsliced tasks remain valid and count only in area totals. Task
selection reports the slice brief to read after the authoritative area brief.

### Keep one-off work in a general area

If you often have small, unrelated improvements, keep them in an ordinary area
with the conventional tag `general`. Create its isolated branch yourself, then
create the area with the existing command:

```sh
git switch -c general
zdev area create general \
  --title "General work" \
  --objective "Keep concrete one-off improvements as reviewed tasks."
```

For a personal/project record that deliberately shares configured trunk, omit
the new branch and add `--trunk` to the area command.

Maintain a short standing `brief.md` with the shared engineering boundaries,
testing level, and validation commands. Each one-off task still needs its own
useful outcome, context, boundaries, done proof, and validation. Most remain
unsliced; add a slice only when several related tasks need one shared objective.

For example:

```markdown
# General work

## Objective

Keep concrete one-off improvements as reviewed tasks.

## Boundaries

- Give every task a useful outcome, clear scope, and observable done proof.
- Preserve compatibility unless an approved task says otherwise.

## Testing

Focused coverage. Match test work to the behavior or risk in each task.

## Validation

- Run the repository's standard validation before completion.
```

You can ask the harness to discuss a concrete one-off request and draft its task
bundle in the same interaction. When no product or testing choice remains, it
may proceed directly to the exact bundle review without a separate research or
full-brief pass. You still approve that exact bundle, work on the recorded
branch, run proportionate tests, verify independently, complete the task, and
commit the accepted changes. Zdev does not create or switch the branch for you.

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

Zdev stores the canonical bundle, an internal fingerprint, and an actual
Markdown review file under repository-local Git administrative state. Its small
JSON result names that file and an opaque review identity. For non-trivial
work, a fresh reviewer reads that exact Markdown file. If it suggests concrete
revisions, the harness replaces the stored candidate. A focused correction is
checked by the same reviewer against the prior findings and the complete
revised document. A material change to scope, boundaries, dependencies, task
splitting, or testing strategy receives a fresh full challenge. Storing or
challenging a candidate does not approve it. The harness presents only the
final challenged document for explicit approval with:

```sh
zdev tasks review scheduling --show
```

You approve the Markdown once. The harness retains the review identity
automatically; you never read, copy, compare, or diagnose it or the internal
fingerprint. It then imports the exact current artifact with `zdev tasks import
scheduling --reviewed <review-id>`.

Zdev rejects content that differs from the reviewed bundle. If it changed, the
harness replaces the stored review, shows the new Markdown, and asks for
approval again. It does not reconstruct or resend the approved document. For
manual input, direct import remains available: paste the JSON into `zdev tasks import
scheduling --from -` and press Ctrl-D. `--from path/to/tasks.json` also works
and leaves the source file in place. Zdev writes one Markdown file per task and
regenerates `TASKS.md`. Edit task files, not the generated index.

For project and pull-request records, the harness adds `--commit --format json`
to its stored-review import, including for the initial task split. Personal
records keep using ordinary import. A manual direct import can use `zdev tasks
import scheduling --from - --commit --format json`. Use ordinary import under
any policy when you explicitly want the additions left uncommitted.

An initial managed commit includes the config, area metadata, brief, referenced
slice briefs, new task files, and regenerated `TASKS.md`. If later approved work modified the owning area's
tracked `brief.md`, leave it unstaged. The committed import validates and
includes the brief with the new task files and regenerated `TASKS.md`; no
separate brief commit is needed. Unrelated staged and unstaged changes are
preserved. The JSON result includes task IDs, paths, the commit hash, and the
stable change ID. It also includes the complete ready frontier in stable task
order.

## 7. Run the task loop

Inspect the branch relationship and next ready task directly when needed:

```sh
zdev status scheduling --format json
zdev next scheduling --format json
```

The installed implementation workflow uses one read-only collection instead:

```sh
zdev work-context scheduling --format json
```

For open work this returns matching status and goal projections, HEAD, and the
exact staged, unstaged, and untracked evidence. A validated closed area returns
before branch and Git collection.

When that complete JSON would be expensive to carry between workers, use its
optional filesystem transport:

```sh
zdev work-context scheduling --store --format json
zdev work-context scheduling --show <snapshot-id> --format json
zdev work-context scheduling --compare <snapshot-id> --format json
```

Store returns a compact reference and writes the exact ordinary JSON under
repository-local Git administrative state. Show emits those bytes exactly.
Compare collects a new ordinary work-context and returns only whether it is
equal. Always compare or collect fresh state at a new decision boundary: the
stored file is an immutable handoff, not permission to act on later. Snapshots
remain available so an active workflow can keep loading its original baseline.

With only an area, zdev ranks ready tasks by AFK suitability, priority, then
numeric task ID. When you give a fuzzy loop focus, every supported harness
reads the full ready frontier and chooses the best fit. The focus is guidance,
not an exact filter, and is applied again after each commit.

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

git add <explicit-implementation-path>... \
  .zdev/scheduling/tasks/001-add-the-scheduling-model.md \
  .zdev/scheduling/TASKS.md
zdev commit -m "feat: add scheduling model"
```

`zdev commit` adds a stable `Zdev-Change-Id` trailer. Repeat `zdev next` until
the open queue is exhausted. Queue exhaustion does not close the objective;
close it explicitly after reviewing the result:

```sh
zdev area close scheduling
# If more approved work appears later:
zdev area reopen scheduling
```

Task bundles may author `complexity` as `routine`, `standard`, or `advanced`;
omission means `standard`. The harness never infers routine work. Advanced work
gets one read-only plan before its first edit, while every route keeps a fresh
independent standard verifier.

Activate zdev and ask it to loop or set a goal for an area when you explicitly
want continuation across tasks. Both words select the same route. The harness
reports each selected task and completed verified commit. Codex, Claude Code,
and Oh My Pi can continue natively. OpenCode and Pi complete at most one task
per invocation and return `CONTINUE` only after a commit and a fresh ready
work-context.

Inspect or find a logical change after a rebase with:

```sh
zdev change inspect HEAD
zdev change lookup Z0123456789abcdef...
```

You can add tasks while this loop is running with the committed-import command
from step 6. New task-only commits do not interrupt the selected task. Finish
that task, then consider the additions at the next `zdev next`.

## 8. Keep the area current

Before selecting or completing work, require
`branch_status.task_work.safe` in JSON status. If trunk advances while the
checked-out branch, anchor, ancestry, and linear history remain valid, zdev
reports one rebase advisory and allows ordinary task work to continue. Run the
explicit rebase when you need current trunk changes or are preparing to
integrate:

```sh
zdev area rebase scheduling
```

An explicit trunk area instead resolves current `project.trunk` on every
operation. Require that branch to be checked out and safe, but do not request a
rebase or freshness step. Keep exact area/task attribution when inspecting and
staging changes shared on trunk.

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

Codex uses one `$zdev` skill, collaboration agents, and a fresh verification
context. A project integration lives under `.codex/skills/zdev`. Asking zdev to
loop or set a goal uses Codex's native goal when clear and falls back to one
verified task only when inspection proved clear but native creation is unavailable.

### Claude Code

Start Claude Code from the repository root and accept workspace trust for a
project installation. The integration provides one zdev skill, scoped
implementer and verifier agents, and packaged workflows used by the skill.

### OpenCode

OpenCode installs one skill plus agents and route commands under `.opencode`.
Asking zdev to loop or set a goal completes at most one task and returns `CONTINUE` only
after a verified commit when ready work remains. OpenCode discovers project
skills when started from a subdirectory in the worktree.

### Pi

Pi installs a skill, prompt templates, and the `zdev_subagent` extension under
`.pi`. The extension starts a fresh child Pi process for each implementation or
verification handoff. Goal and loop are the same bounded one-task continuation
route. A user installation goes to
`$PI_CODING_AGENT_DIR`, or `~/.pi/agent` when the variable is unset.

### Oh My Pi

Oh My Pi is separate from plain Pi. It installs a skill and constrained native
task agents under `.omp` and uses OMP's built-in `task` and `hub` facilities.
The root skill's goal and loop route uses OMP's native goal when clear and falls
back to one verified task when creation is
unavailable after that clear inspection.
A fallback requires a successful clear-goal inspection; ambiguous native goal
state blocks.
A user installation goes to `$PI_CODING_AGENT_DIR`, or `~/.omp/agent` when the
variable is unset.

OMP 17.2.15 may find the skill but miss user task agents when
`PI_CODING_AGENT_DIR` relocates the user root. The install and check commands
warn about this. Unset the variable or use a project install under `.omp` until
upstream discovery is fixed.

## Get help

Run `zdev --help` or `zdev <command> --help`. The [task format](task-format.md)
documents every task field.

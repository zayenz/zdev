# zdev

Zdev gives coding agents a small, durable development loop. It stores plans as
plain Markdown tasks, selects the next unblocked task, and gives each accepted
change a stable Git identifier. Your coding harness still decides how to
implement and verify the work.

Zdev is a personal tool maintained in public. It has no broad support promise
or large compatibility matrix. Releases target macOS and Linux on x86-64 and
Arm64. Zdev integrates with Codex, Claude Code, OpenCode, Pi, and Oh My Pi.

## Install

Versioned binaries and `zdev-installer.sh` are available from the
[latest GitHub release](https://github.com/zayenz/zdev/releases/latest). Follow
the release notes for the binary installation steps.

For installation from source with a Rust toolchain, use Cargo:

```sh
cargo install --git https://github.com/zayenz/zdev --locked
```

A Rust toolchain is required for source installation. From a local checkout, use
`cargo install --path . --locked`.

Install the integration for your coding harness:

```sh
zd skill install codex
# or: claude, opencode, pi, omp
```

## Smallest useful workflow

On the project's trunk branch, first choose the planning-record policy. Use
`personal` for work that stays in this clone and add `/.zd/` to
`.git/info/exclude`. Use `project` for durable collaborative state. Use
`pull-request` when `.zd` should be committed for branch review but omitted
from the squash-merged tree. Then initialize zdev and create an area on a
feature branch:

```sh
zd init --record personal # or: project, pull-request
git switch -c scheduling
zd area create scheduling \
  --title "Scheduling support" \
  --objective "Add a tested scheduling API."
```

Ask your coding harness to plan the objective with zdev. Review its proposed
task split before it imports the tasks. The normal loop is then:

```text
select next task
  → implementation agent
  → fresh verification agent
  → mark done
  → commit with a stable change ID
```

After verification passes, mark the task done and commit it:

```sh
zd next scheduling --format json
zd task done scheduling scheduling-001 \
  --summary "Implemented and independently verified the scheduling model." \
  --validation "Focused model tests passed."
git add <changed-files> .zd/scheduling
zd commit -m "feat: add scheduling model"
```

You can add work while another task is in progress. For an existing task list,
review the bundle, then import and commit that exact bundle:

```sh
zd tasks review scheduling --from - --format json
zd tasks import scheduling --from - --approval <approval-id> --commit --format json
```

A commit containing only new task files and the regenerated `TASKS.md` does not
interrupt the selected task. Zdev considers the additions the next time you run
`zd next`.

For a `pull-request` record, commit `.zd` normally during review. Immediately
before squash merge, require a clean feature branch and run:

```sh
zd cleanup squash
```

This creates one plain Git commit deleting only tracked `.zd` files. The commit
has no `Zdev-Change-Id`; checked-in harness integrations outside `.zd` remain.
It prepares the final tree only: a normal merge or rebase that retains feature
commits still keeps `.zd` in reachable history, and this command does not
implement history-preserving cleanup.

Zdev keeps the area brief, individual task files, a generated task index, and
branch metadata under `.zd/`. It does not keep model transcripts or a second
execution database.

## Learn more

- [User guide](docs/user-guide.md): the complete installation and first-run path
- [Workflow reference](docs/workflow.md): branch, verification, rebase, and
  recovery semantics
- [Task format](docs/task-format.md): the task-file contract
- [Contributing](CONTRIBUTING.md): local checks and generated-file guidance
- [MIT license](LICENSE)

Run `zd --help` or `zd <command> --help` for command details.

## Development

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked
cargo build --locked
git diff --check
```

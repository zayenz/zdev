# Your first zdev task

Zdev keeps an agreed plan and its remaining work in your Git repository. Your
coding agent reads that plan, implements one task, and asks a separate agent to
verify the result before committing it. You can leave the conversation and pick
up the same work in another session.

This tutorial adds a `--name` option to a small Python program. The change is
deliberately small so you can follow the whole process, including what you
approve and what the agent does next.

## Before you start

You need Git, Python 3, and a coding harness with an authenticated model account.
A harness is the app or command-line agent you use to work on code, such as
Codex or Claude Code. Zdev uses its model access and delegation tools. Installing
zdev alone does not start agents or provide a model subscription.

The binary supports macOS and Linux on x86-64 and Arm64. This guide describes
zdev 2.0 from source. The latest versioned binary release is still 1.6.0.
For the version used by this guide, install with a
[Rust toolchain](https://rustup.rs/):

```sh
cargo install --git https://github.com/zayenz/zdev --locked
zdev --version
```

[GitHub releases](https://github.com/zayenz/zdev/releases) also provide versioned
binaries. Check the version when following this guide.

Install the integration for your harness:

```sh
zdev skill install codex
zdev skill check codex
```

Replace `codex` with `claude`, `opencode`, `pi`, `omp`, or `agy` as appropriate.
The default installation is for your user and works across repositories.
The check should report `ok`. After upgrading zdev, refresh it with
`zdev skill install codex --force`. The
[harness reference](user-guide.md#harness-notes) describes differences in support.

## Make a small repository

Run these commands in a terminal, in a directory where you keep experiments.
Choose a directory name that does not already exist:

```sh
mkdir zdev-greeting
cd zdev-greeting
git init -b main
cat > hello.py <<'PY'
print("Hello, world!")
PY
python3 hello.py
git add hello.py
git commit -m "Add greeting program"
```

The program prints `Hello, world!`. Git needs your usual author name and email
configured before the commit.

Initialize zdev while you are on `main`:

```sh
zdev init --record personal
printf '\n/.zdev/\n' >> .git/info/exclude
git switch -c greeting
zdev area create greeting \
  --title "Named greetings" \
  --objective "Let hello.py greet a named person with --name."
```

`personal` keeps planning files local to this clone. The exclude entry keeps
Git from staging them. Accepted source changes will still be committed.
Use `project` instead when you want to share the planning record through Git;
the [record-policy reference](user-guide.md#3-initialize-a-repository) explains
that choice and the `pull-request` policy.

An *area* groups work toward one objective. This one is named `greeting` and is
bound to the branch you just created. Its brief and tasks live in
`.zdev/greeting/`. The brief holds the agreed outcome and boundaries; each task
has its own Markdown file. `TASKS.md` is a generated index.

## Agree on the change

Open your coding harness in this repository. The following examples are
messages to the agent, not terminal commands:

```text
Use zdev to explore the greeting area. Add --name NAME to hello.py.
Without the option it should still print Hello, world! With --name Ada
it should print Hello, Ada! Use Python's standard library. Checking
those two invocations is enough validation for this example.
Use this checkout for the tutorial.
```

The agent reads the repository and updates `.zdev/greeting/brief.md`. Review
that brief. It should describe the two outputs, the standard-library boundary,
and the agreed validation. If it introduces unnecessary work, ask it to revise
the brief before making tasks.

For a larger change, [discuss](commands/discuss.md) the unresolved design choices
at this point. Here the requirements are small enough to continue directly.

## Review the task

Ask the agent:

```text
Use zdev to create tasks for greeting. Show me the proposed bundle
before importing it.
```

The agent proposes one or more tasks and renders them for review. Each task
should say what will change and how completion will be checked. For this
example, one task is enough. Ask for a smaller split if the proposal adds work
you did not request.

Once the displayed bundle matches the brief, reply:

```text
Import this exact task bundle for greeting.
```

Import records the approved work. It does not start implementation. You can
stop the conversation here; a new session can read the same brief and tasks.

## Implement and verify

Ask for one task:

```text
Use zdev to implement the next task in greeting in this checkout.
```

The agent selects a task whose dependencies are satisfied, implements it, and
runs the agreed checks. A fresh verifier checks the result against the task
and repository rules. Required corrections get a correction pass and a focused
recheck. Unresolved corrections or a missing prerequisite leave the task open.

After a pass, the coordinating agent records completion and commits the accepted
source changes. The commit has a `Zdev-Change-Id` trailer, which identifies the
logical change across rebases. A single implementation request stops after
that task and its commit.

Inspect the result in the checkout where the agent reports the commit:

```sh
python3 hello.py
python3 hello.py --name Ada
zdev status greeting
zdev next greeting
git log -1 --format=full
```

The program should print `Hello, world!` and `Hello, Ada!`. Status should show
the completed task. If you imported only one task, `next` should have no ready
task left. The source change should be in the reported commit; the personal
planning record remains local.

## Continue or finish

For a larger area, [implement](commands/implement.md) handles one ready task.
[Goal and loop](commands/loop.md) continue through a named area's approved work,
subject to the harness's continuation support. A [parallel batch](commands/parallel.md)
can handle compatible tasks in separate worktrees.

An empty queue does not close the area or merge the feature branch. Once you
have inspected the result, close this tutorial area:

```sh
zdev area close greeting
```

Merge the branch using your usual Git workflow when you are ready. If work was
interrupted, use [recovery](commands/recover.md) before starting another task.
The [command guide](commands/index.md) covers the other things you can ask
zdev to do; the [workflow reference](workflow.md) explains selection,
verification, branch handling, and recovery in detail.

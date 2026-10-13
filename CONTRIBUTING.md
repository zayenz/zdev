# Contributing

Zdev is a personal tool maintained in public. Small bug reports and focused
changes are welcome, but response times and long-term support are not
guaranteed.

Use ordinary [GitHub issues](https://github.com/zayenz/zdev/issues) for bugs or
questions. Include the zdev version, operating system, harness, command, and a
small reproduction where relevant. Never include credentials or private
repository content.

## Making a change

Keep changes narrow and preserve unrelated work. Run the checks that match the
change; the full set is:

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked
cargo build --locked
git diff --check
```

`src/lib.rs` and `src/main.rs` contain the Rust CLI. Black-box behavior tests
live in `tests/lean.rs`.

Tests run locally with temporary repositories and scripted worker responses.
They must not invoke an LLM, start a live coding harness, or depend on provider
credentials. Keep expensive workflow checks to representative wiring scenarios;
CLI lifecycle checks own detailed failure and recovery behavior. Extend a
relevant existing check for a regression before adding another scenario matrix.

The report generator has its own small local suite. Install its pinned
dependencies and run it when changing the generator or qualifying a release:

```sh
npm ci --prefix skills/zdev-area-report
npm test --prefix skills/zdev-area-report
```

Browser checks remain optional; these commands do not launch a coding harness.

Canonical harness sources live under `templates/zdev/`. The checked-in harness
integrations are generated fixtures. Change the templates, then regenerate the
fixtures; do not patch generated copies independently. Likewise, edit
individual task files under `.zdev/<area>/tasks/`, not the generated `TASKS.md`
index.

The checked-in Codex plugin bundle is relocatable. Regenerate it with
`zdev skill install codex --to skills --portable --force`; check it with the
same `--to skills --portable` flags. Ordinary user and project installations
keep their resolved absolute locators. Refresh the other generated fixtures
with their existing `skill install <harness> --to <directory> --force` commands.

For release and packaging changes, also run the relevant release scripts and
package checks under `scripts/`.

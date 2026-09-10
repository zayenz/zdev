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

Canonical harness sources live under `templates/zdev/`. The checked-in harness
integrations are generated fixtures. Change the templates, then regenerate the
fixtures; do not patch generated copies independently. Likewise, edit
individual task files under `.zdev/<area>/tasks/`, not the generated `TASKS.md`
index.

For release and packaging changes, also run the relevant release scripts and
package checks under `scripts/`.

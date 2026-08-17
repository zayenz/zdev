# Contributing

Zdev is a personal tool maintained in public. Small bug reports and focused
changes are welcome, but response times and long-term support are not
guaranteed.

Use ordinary [GitHub issues](https://github.com/zayenz/zdev/issues) for bugs or
questions. Include the zdev version, operating system, harness, command, and a
small reproduction when they matter. Never include credentials or private
repository content.

## Making a change

Keep changes narrow and preserve unrelated work. Run the checks that match the
change; the normal full set is:

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked
cargo build --locked
git diff --check
```

`src/lib.rs` and `src/main.rs` contain the Rust CLI. Black-box behavior tests
live in `tests/lean.rs`.

Several checked-in harness integrations are generated from templates under
`templates/zdev/` and source workflow references under `skills/zdev/`. Change
the source templates or references, then regenerate the integrations; do not
patch generated copies independently. Likewise, edit individual task files
under `.zdev/<area>/tasks/`, not the generated `TASKS.md` index.

Release and packaging changes should also follow the repository's existing
release scripts and package checks.

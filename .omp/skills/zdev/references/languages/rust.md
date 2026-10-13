# Rust

Apply these rules with [coding guidance](../coding.md) when planning, changing,
or reviewing Rust code.

## Ownership and contracts

Use structs for records and enums with associated data for alternatives. Use
`Option` for absence, `Result` for expected failure, and `?` for propagation.
Reserve panics for justified invariant failures; an `expect` message explains
why the invariant should hold. Tests may use direct assertions and unwraps.

Borrow when an operation needs access; take ownership when it stores or consumes
a value. Pass inexpensive `Copy` values directly. Prefer slices and `&str` when
only contents matter. Clone or collect when ownership, reuse, or clarity earns
the allocation; avoid intricate lifetimes merely to save a harmless clone.

Use explicit annotations for non-obvious parsed or collected results. Follow
the usual `as_`/`to_`/`into_` and `iter`/`iter_mut`/`into_iter` conventions to make
conversion and consumption apparent. Add traits and generics for current
variation rather than hypothetical extensibility. Keep unsafe operations narrow
and document the invariants that make them sound.

## Validation

Always run rustfmt and Clippy for affected Rust work. Format with the repository
configuration; verify formatting without rewriting, normally with
`cargo fmt --all -- --check`. Run Clippy over affected packages, targets, and
supported feature combinations; `cargo clippy --workspace --all-targets` is a
useful workspace default. Select features deliberately: `--all-features` can
enable incompatible combinations. Respect the project's warning policy and use
narrow, justified lint allowances instead of enabling or suppressing every lint.

References: [Rust API guidelines](https://rust-lang.github.io/api-guidelines/),
[Clippy usage](https://doc.rust-lang.org/clippy/usage.html).

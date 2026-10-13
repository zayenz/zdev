# Rust

Apply these rules with [coding guidance](../coding.md) when planning, changing,
or reviewing Rust code.

- Use structs for records and enums with associated data for alternatives.
- Use `Option` for absence, `Result` for failure, and `?` for propagation.
- Borrow for reading; take ownership when storing or consuming values.
- Prefer slices and `&str` when a function only needs their contents.
- Clone or collect when ownership, reuse, or clarity warrants it.
- Prefer named aggregates such as `sum` to an equivalent fold.
- Add traits and generics for current needs.
- Keep ownership and error paths visible.

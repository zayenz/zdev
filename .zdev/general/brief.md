# General work

## Objective

Keep concrete one-off improvements as reviewed tasks.

## Boundaries

- Keep each task narrow and avoid new abstractions or process machinery unless
  the task demonstrates a concrete need.
- Preserve documented behavior and compatibility unless an approved task says
  otherwise.
- Change canonical harness templates first and regenerate checked-in artifacts.

## Testing

Use focused tests when behavior changes. Do not add tests that merely restate
the implementation. Every task runs the checks relevant to its change and the
standard repository validation before completion.

## Validation

- `cargo fmt --all -- --check`
- `cargo clippy --locked --all-targets --all-features -- -D warnings`
- `cargo test --locked`
- `cargo build --locked`
- `git diff --check`

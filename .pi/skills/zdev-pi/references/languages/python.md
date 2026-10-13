# Python

Apply these rules with [coding guidance](../coding.md), including for scripts.

## Types and explicit behavior

Type function parameters and results, including `-> None`, and meaningful fields.
Annotate ambiguous containers; infer obvious local values. Use collection
interfaces that describe the required operations. Choose ordinary classes,
dataclasses, `TypedDict`, or `Protocol` for an actual distinction in the contract.
Keep `Any`, casts, and suppressions narrow and justified. Annotations do not
validate external data at runtime.

Prefer direct calls, normal imports, explicit configuration, and visible
registration. Avoid reflection-based dispatch, dynamic attributes, monkey-patching,
metaclasses, and import-time registration when functions and data suffice. Keep
dynamic behavior required by an established framework localized. Comprehensions,
generators, context managers, dataclasses, and ordinary properties are useful
idioms; use them when their behavior is clear.

## Execution and validation

Use uv for dependency management and execution. Run project commands through
`uv run`, using the project environment and lockfile. Executable scripts start
with `#!/usr/bin/env -S uv run --script`; set executable permission when direct
execution is intended.

Keep standalone script dependencies in PEP 723 inline metadata instead of
creating a package merely to run one file. Include `dependencies`, even when
empty, when using this metadata. Metadata isolates the script from surrounding
project dependencies; scripts requiring project imports must retain the project
execution environment. Add Python requirements or a script lockfile when needed
for compatibility or reproducibility.

Use Ruff for formatting and linting, and ty for type checking. Typical project
checks are `uv run ruff format --check`, `uv run ruff check`, and `uv run ty check`.
Use existing project commands where available. Check standalone scripts explicitly
and ensure the type checker resolves their actual dependency environment. Keep
syntax compatible with the supported Python baseline.

References: [Python typing](https://docs.python.org/3/library/typing.html),
[uv scripts](https://docs.astral.sh/uv/guides/scripts/),
[ty](https://docs.astral.sh/ty/type-checking/).

# Kotlin

Apply these rules with [coding guidance](../coding.md). Read Java guidance too
when changing Java code or the Java/Kotlin boundary.

## Contracts and expressions

Declare return and property types where they expose a contract or clarify a
non-obvious expression. Infer obvious locals. Prefer constructor properties,
ordinary properties, top-level functions, default arguments, and named arguments
when they make the operation clear. Add interfaces and builders for concrete
variation or construction needs. Use expression bodies for short computations
and blocks when steps need names.

Represent meaningful absence with nullable types. Handle it through checks,
smart casts, safe calls, or Elvis expressions. Reserve `!!` and `lateinit` for
established invariants or framework lifecycle requirements. Establish nullability
explicitly at Java boundaries: platform types can bypass Kotlin's guarantees.

Use data classes when generated equality and copying match the data's meaning.
Constructor properties determine generated equality; `copy()` is shallow.
Prefer `val` and read-only collection contracts while keeping mutation ownership
clear; neither guarantees immutable referenced objects. Use enums for simple
alternatives and sealed types for alternatives carrying different data, with
exhaustive `when` handling where applicable.

Name ambiguous lambda parameters. Keep scope functions and transformation chains
short enough that receivers, intermediate meanings, and effects remain apparent.
Choose `let`, `apply`, and related functions for their purpose rather than merely
removing temporary variables.

When coroutines are needed, give work an owned lifecycle, preserve cancellation,
and make blocking boundaries explicit. Follow the existing concurrency model.

## Validation

Discover compiler, platform, and interoperability targets from the build. Run
existing formatting, static checks, compilation, and relevant behavioral checks.
Load platform guidance only for affected responsibilities; Kotlin alone does not
imply Android, Compose, coroutines, or a particular lint tool.

References: [Kotlin conventions](https://kotlinlang.org/docs/coding-conventions.html),
[null safety](https://kotlinlang.org/docs/null-safety.html).

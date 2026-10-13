# Java

Apply these rules with [coding guidance](../coding.md). Read Kotlin guidance too
when changing Kotlin code or the Java/Kotlin boundary.

## Values and behavior

Use modern stable language features supported by the project's compiler and
runtime. Use records for transparent values whose generated accessors and value
equality fit the contract; use ordinary classes for identity, mutable lifecycle,
or hidden representation. Records are only shallowly immutable. Preserve public
accessor, equality, and serialization contracts when converting existing types.

Use enums for simple finite alternatives and sealed types for alternatives with
different data. Prefer pattern matching and exhaustive switches where supported;
avoid a default branch that accidentally hides newly added alternatives.

Use explicit local types when a factory or calculation has an unclear result,
or when the abstraction or numeric width matters. `var` is useful when the
initializer already explains the type. Check inference for empty collections
and generic factories.

Choose collection mutability deliberately. Factory/copy methods and
`Stream.toList()` produce unmodifiable collections; replacing a mutable result
can change callers' behavior. Use streams for clear transformations and loops
when state or intermediate steps need names.

Own resources with try-with-resources where applicable. Preserve exception causes
and use value equality deliberately. Required input validation must run without
relying on Java assertions being enabled.

Prefer direct functions and concrete classes. Introduce interfaces, builders,
factories, service layers, and dependency injection machinery only when a current
requirement earns them.

## Validation

Discover the actual source/runtime baseline and run the project's build, format,
lint, and relevant behavioral checks. Respect existing Maven, Gradle, or Bazel
workflows. Treat version upgrades and preview features as explicit decisions.

References: [OpenJDK local type inference](https://github.com/openjdk/amber-docs/blob/master/site/guides/lvti-style-guide.md),
[records](https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/lang/Record.html).

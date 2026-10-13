# C++

Apply these rules with [coding guidance](../coding.md). Use the project's supported
standard, error handling, naming, and dependency conventions.

## Ownership and interfaces

Prefer values and scoped objects. Use RAII for memory, files, locks, and other
resources. Prefer `std::unique_ptr` for dynamic exclusive ownership; use shared
ownership for a concrete shared lifetime. Raw pointers and references ordinarily
borrow. Confine manual allocation and external ownership protocols to their
boundaries.

Prefer the rule of zero: members should provide correct copy, move, and destruction
behavior. When implementing ownership yourself, deliberately define or disable
the relevant special operations. Pass inexpensive inputs by value and larger
read-only objects by `const` reference where appropriate.

Use `std::span` and `std::string_view` for borrowed sequences and text when the
standard permits. Check lifetimes, invalidation, stored/returned views, and
asynchronous captures. A string view does not promise null termination.

Use explicit types when they reveal meaning, ownership, or an unclear result.
Use `auto` when the initializer explains the type or deduction improves clarity;
choose value/reference and constness deliberately. Keep public contracts apparent.
Prefer concrete functions and types until generic behavior earns its complexity.

Make headers self-contained and include dependencies directly. Prefer named C++
casts, checking narrowing conversions. Reinterpretation and removal of constness
require a specific correctness argument.

## Validation

Run clang-format with project configuration and clang-tidy with actual compilation
flags, preferably through `compile_commands.json`. Use selected checks and review
fixes; enabling every check is not a default.

Build and link affected code with ASan and UBSan, then execute relevant tests or
representative scenarios. Use project sanitizer settings; a common Clang setup is
`-fsanitize=address,undefined -g -fno-omit-frame-pointer`. Check sanitizer reports
even when the process exits successfully: some UBSan checks recover by default.
Report unsupported platforms or incomplete instrumentation. Passing runs cover
executed paths, not every lifetime error, race, or undefined behavior.

References: [Google C++ guide](https://google.github.io/styleguide/cppguide.html),
[Core Guidelines](https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines),
[Clang sanitizers](https://clang.llvm.org/docs/UndefinedBehaviorSanitizer.html).

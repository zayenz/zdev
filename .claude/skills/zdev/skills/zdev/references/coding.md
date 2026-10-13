# Coding guidance

Planning agents read **Design** and **Code standards**; implementation agents
also read **Completion**. Review agents read **Code standards** and follow
**Review**. Apply the rules within the approved outcome and testing level,
using existing plans, handoffs, and results.

## Design

1. Trace the affected behavior through existing code, callers, data, dependencies,
   and relevant tests. Read applicable project instructions and design decisions.
   Expand the investigation when evidence reveals another affected contract or
   invariant. Planning is ready when you can explain where the behavior belongs,
   why that boundary fits, and which callers and invariants are affected.
2. Choose the smallest coherent change that fits this context. Include adjacent
   changes necessary for a sound solution; separate independent improvements.
   In `plan.approach`, or the existing handoff, state the observable result,
   owning function or module, affected contracts, and completion evidence.
3. Sketch a multi-step operation's signature and call sequence before its
   supporting functions. Make inputs, results, and effects clear enough to
   understand the operation without opening every function. A local correction
   needs only its affected step identified.
4. Reassess the plan against the code. Define changed contracts and update the
   plan or handoff before another agent builds against them. Keep decisions
   within the approved scope; surface changes to requirements or boundaries.

## Code standards

### Context and simplicity

Reduce the reader's cognitive load. Prefer the simplest design that handles
current requirements. Add abstractions when their benefit earns their cost.
Readability is part of correctness and acceptance. Use judgment: each rule
should improve the code in its context.

Use the project's formatter and established style. Evaluate existing patterns
against their purpose before copying them. Refactor for readability when it
helps explain or safely change affected logic. Preserve behavior and public
contracts unless changing them is part of the task. Keep unrelated cleanup
separate and remove leftovers from the change. Add features, configuration,
defensive code, and interfaces for concrete needs.

### Names

Name actions for what they do, predicates and booleans as questions, and other
queries for the answer they return, using the language's idiom. Use nouns for
types, plurals for collections, and singulars for elements.

Use the fewest words that remain clear in context. Omit context supplied by the
receiver or enclosing code. Use established abbreviations and corresponding
names for paired operations. Name conditions and constants for their meaning.
Check names at their callers. Explain consequential interface choices when
needed; ordinary identifiers need careful review, not individual check lines.

### Data and contracts

Give data with distinct field meanings a named structure. Prefer named fields
to tuple positions and lists used as records. Keep conventional pairs and
uniform nested collections simple. Use algebraic data types for alternatives
where supported; make meaningless combinations of flags and optional fields
unrepresentable where practical.

Represent absence and failure explicitly, preserving meaningful distinctions.
Prefer immutable transformations to shared mutable bookkeeping. Pass only the
data a function needs; keep results as data until rendering.
When mutation is intrinsic to the algorithm, make ownership, lifetime, and
invalidation rules explicit.

Make inputs, results, effects, and prerequisites clear. Express guarantees in
types; test doubles must honor them too. Validate external data at entry.
Handle failures where a meaningful response is possible and propagate the rest.
Use project error handling, logging, and configuration facilities.

Write explicit types when they reveal a contract, domain meaning, ownership, or
a result that is otherwise difficult to infer. Use inference when the expression
makes the type clear. Judge readability at the declaration and its uses.

Keep reads free of observable mutation and make writes apparent. Return what
the next operation needs; combine inseparable operations. Sort only when order
matters, with explicit keys and stable tie-breaking where required. Keep
pagination consistent with that ordering.

### Structure and control flow

Read top down from the entry point: the code should tell a story. Put the main
operation before its details where project conventions permit. Split code into
manageable parts without excessive indirection. Extract a function when its
name and boundary make the code easier to understand. A long, cohesive function
is fine. Keep related calculations and invariants together.

Separate domain decisions from storage and presentation where useful. Distinct
responsibilities do not automatically need separate layers or files. Make
dependencies explicit and give shared mutable state an owner. Keep declarations
near use and checks beside the values they validate.
Use the project's actual dependency direction. A cohesive state machine may
combine control flow and updates when its contract and invariants stay clear.

Use early returns to discharge cases and expose the main path. Prefer switch or
match to chains of alternatives on the same value. Handle every typed
alternative deliberately. Prefer clear, idiomatic map and filter operations.
Use simple reduce operations for obvious aggregates; be more cautious with
fold. Use a loop when a transformation obscures state, effects, or intermediate
meanings. Preserve cleanup and resource-lifetime guarantees.

Improve names and structure before adding explanatory comments. Comment
constraints, invariants, and necessary exceptions; link relevant specifications.
Keep examples focused on teaching the interface. Put necessary regression
assertions in the existing tests rather than turning examples into test harnesses.

### Language and framework guidance

Before planning, changing, or reviewing the following code or its build and
validation configuration, read the corresponding supplement:

- **Rust:** [languages/rust.md](languages/rust.md).
- **TypeScript:** [languages/typescript.md](languages/typescript.md), including TSX.
- **MiniZinc:** [languages/minizinc.md](languages/minizinc.md), including data and solver variants.
- **Java:** [languages/java.md](languages/java.md).
- **Kotlin:** [languages/kotlin.md](languages/kotlin.md).
- **Python:** [languages/python.md](languages/python.md), including executable scripts.
- **C++:** [languages/cpp.md](languages/cpp.md).

Also load the supplements for the affected responsibilities:

- **Web:** [web.md](web.md) for browser behavior, URLs, forms, or HTTP responses.
- **React:** [frameworks/react.md](frameworks/react.md) for components, hooks, or component state.
- **React Router framework mode:** [frameworks/react-router.md](frameworks/react-router.md)
  for route modules, data loading, mutations, or navigation.

Load project and installed language or framework guidance when the affected
code uses it. Scope framework guidance to the responsibility being changed or
reviewed; a dependency's presence alone does not make all its guidance relevant.
Resolve conflicts using explicit user requirements and applicable project
constraints; explain material exceptions.

Keep project-specific guidance with the project. Add shared supplements for
explicit preferences or recurring problems, with a pointer here stating what
work triggers each one. Each supplement owns additional rules or justified
exceptions; link to common rules. Discover versions, commands, and configuration
from the environment. Keep each rule in one place.

Supplement checks use the existing completion and review procedure; loading
several supplements adds no review rounds. Use project tool versions and
configuration. Report unavailable required tools or unsupported checks as
missing evidence; do not silently replace the toolchain or claim a pass.
Before declaring a tool unavailable, check PATH and known project or installed
toolchain locations once. Use its actual project configuration and build flags.

## Completion

1. Check relevant tests before a refactor. Keep test effort proportional to
   useful confidence and maintenance cost. Test behavior and relevant failures;
   a regression test must catch the original bug.
   A bug fix does not automatically need a permanent regression test. Use nearby
   test style when one adds useful confidence; existing checks or a temporary
   reproduction may be enough at the agreed testing level.
2. Read the finished change from its entry point. Check requirements, names,
   structure, callers, and both sides of changed contracts against this guidance.
3. Run agreed formatting, linting, and validation, including relevant tests after
   a refactor. Finish when the requested behavior, readability, and required
   validation are demonstrated. Report missing evidence and limitations honestly
   in the existing result; leave unsupported claims unverified.

## Review

**Reviewing code or routing corrections:** Read [review.md](review.md) for the
review procedure, feedback standard, and bounded correction cycle. Give each
planning, implementation, and review agent this coding guidance and its
applicable supplements. Use ordinary role names and call code parts functions,
types, or modules according to what they are.

# TypeScript

Apply these rules with [coding guidance](../coding.md). Load framework guidance
for the responsibilities being changed, including React when working on its TSX.

## Types and narrowing

Declare meaningful API and return contracts; infer obvious locals and contextual
callback parameters. Use `: Type` to establish an intended contract and
`satisfies Type` to check a value while retaining useful specificity. Use
`as const` for literal information and readonly types; it neither validates data
nor freezes it at runtime. Keep `const`/`let` choices about reassignment separate
from decisions about type annotations.

Avoid casts. Build complete, correctly typed values, narrow unions, and repair
inaccurate declarations instead of using `as`, postfix `!`, double assertions,
or `any` to silence errors. A necessary assertion belongs at a narrow boundary
with an explanation of the guarantee the compiler cannot establish.

Treat untrusted input as `unknown` until validated. Type predicates and assertion
functions must perform the checks their signatures promise. Use discriminated
unions for alternatives with different data and handle closed alternatives
exhaustively. Keep absence meaningful instead of making every property optional.

Prefer straightforward named types to clever conditional or mapped types unless
the transformation removes a real source of inconsistency. Follow the project's
type/interface convention; neither choice is a universal design rule.

## Validation

Run the project's type checker, formatter, and linter. Use strict checking for
new configurations; consider unchecked indexed access when indexing can miss.
Respect the existing runtime, module settings, and package manager. Changing
repository-wide compiler policy is a separate decision from fixing affected code.

References: [Matt Pocock on annotations and assertions](https://www.totaltypescript.com/books/total-typescript-essentials/annotations-and-assertions),
[TypeScript narrowing](https://www.typescriptlang.org/docs/handbook/2/narrowing.html).

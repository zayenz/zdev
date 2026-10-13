# MiniZinc

Apply these rules with [coding guidance](../coding.md). These portable defaults
adapt `z-minizinc-style`; load that skill when available for detailed solver work.

## Model and data

Start models with `include "globals.mzn";`. Group declarations into the sections
the model needs: data, validation, derived data, decisions, constraints, objective,
and output. Use `snake_case` for values, fields, functions, and predicates;
`UpperCamelCase` for types, enum constructors, and named sets or domains.

Use enums, records, and optional values when they express the domain directly.
Preserve array index sets and enum identities instead of assuming integer indexes
start at one. Compute data-derived parameters without introducing decision
variables. Give decisions tight input-derived domains. Prefer globals when they
express the intended relation clearly.

Assert input assumptions with useful messages. Explain each non-trivial constraint
block's intent and label each top-level logical constraint with a short string
annotation. Check implication versus equivalence, empty generators, optional
values, and partial expressions. Justify redundant strengthening and symmetry
breaking; use `implied_constraint` and `symmetry_breaking_constraint` wrappers.
Symmetry breaking must preserve representatives and the intended objective.

Keep shared formulations solver-independent. Isolate deliberate objective or
search variants in thin include-based wrappers.

## Validation

Use zincite to format and lint (`zincite-fmt`, `zincite-lint`); discover installed
options and honor project configuration. Review fixes before applying them.
Follow with MiniZinc model/instance checks and representative solves: linting
cannot establish semantic correctness or performance.

Select solver comparisons for the task and discover identifiers with
`minizinc --solvers`. Apply a project or loaded skill's solver matrix, reporting
unavailable entries. Compare the same data, limits, and output expectations.
Check solutions against intended constraints and distinguish feasible solutions,
proved optimality, infeasibility, and limits. Use an independent solution checker
when model complexity warrants one. For `relax_and_reconstruct`, choose meaningful
decision variables and a restart policy; relaxation is triggered by restarts.

References: [MiniZinc modelling](https://docs.minizinc.dev/en/stable/efficient.html),
[zincite](https://github.com/zayenz/zincite).

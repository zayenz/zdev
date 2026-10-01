# Coding guidance

Use this reference to make the assigned change understandable and safe to
maintain. Someone reading a call should be able to tell what it accomplishes,
which values it needs, and what it can change. Clear code reduces mistakes when
the next change arrives.

The task determines the scope and testing level. These rules guide decisions
inside that scope; they grant no additional work, tests, abstractions, approval
steps, planning workers, or cleanup.

- Planners use **Design** and **Code standards**.
- Implementers also use **Completion**.
- Verifiers, auditors, and improvement reviewers use **Code standards** and
  **Review**.

## Design

Describe the intended result in `plan.approach`: what will be observable, where
the behavior belongs, and how completion will be demonstrated. Read the
repository instructions and relevant existing code before choosing the shape
of the change.

For a change spanning several calls, sketch the controlling function's inputs,
result, and sequence first. Specify what each called unit receives, returns,
and changes before filling in its implementation. Reading this sequence should
be enough to understand the operation. An existing function may already provide
the right home; use a plain function when that suffices.

For a local correction, identifying the affected operation may be all the
design needed. Use **Code standards** to assess the proposed split and data
types, including suggestions in an existing plan. Each collaborator and
interface needs a reason to exist in this change.

Implementation can reveal that the planned contract is unsuitable. Settle its
replacement before building against it, and update the plan or handoff if
another worker relies on that contract. Such an adjustment within the task
does not call for another planner. Arrange the work into useful increments
that can be validated as the repository expects, including changes that cross
layers.

## Code standards

### Change size and placement

Keep the edit tied to the requested result. Preserve adjacent code and remove
unused material left by your own change. When a rewrite is authorized, revisit
the names and divisions inside the unit being rewritten; adjacent units retain
their existing scope.

New features and configuration need an expressed requirement. Defensive code
needs a failure that can occur. Share a utility to resolve duplication that
exists, and introduce an interface only for current variation or a need to test
the behavior independently.

Place domain decisions where they can be understood without knowledge of
storage, rendering, or the framework. Entry points pass requests into that
logic, and persistence code handles data access. A coordinating function
assembles the operation from named calls. Its collaborators receive their
dependencies explicitly, with ownership and lifetime accounted for. An
interface to infrastructure expresses what the consuming code requires.

Check tool-enforced requirements and adopted standards before borrowing a
neighboring idiom. Preserve semantics that ecosystem callers depend on; use
local spelling and layout to settle choices left open by those requirements.
When making an exception, state the applicable rule or practical constraint.
An existing defect gives no reason to repeat it or expand the task to fix it.

### Calls and values

A function should receive the data its operation needs and return a typed
result for the next operation. Keep that result as data until the output is
rendered. Prefer transformations of immutable values to shared bookkeeping.

To decide whether a block belongs in a separate function, try describing its
single job at the call site. Extract it, even for one caller, if that description
works as a specific name, the signature and name explain the result and effects,
and a few typed arguments replace access to the enclosing function's working
state. If any of those requirements fails, keep the block inline or choose a
different split. Long or deeply nested code deserves inspection, but neither
length nor depth decides where to cut.

Some computations belong together to preserve an invariant or avoid repeating
substantial algorithmic work. Keep them together and briefly document the
constraint that requires it.

Choose types that carry the meaning of the values being passed. Use named
domain types where they clarify a contract, and declare public return types as
the language expects. Make optional results and failures explicit. If the
domain distinguishes a missing entry, a null value, and an empty value, retain
those distinctions.

### What callers can rely on

Express guarantees through the API and its types wherever practical. All
implementations, including testing substitutes, must provide them. Calls that
omit a prerequisite or use outdated state should be impossible or explicitly
rejected. Supply the next operation with the value it needs, or make inseparable
operations one call. If callers need separate dependent operations, their
names and contracts must disclose that dependency.

Keep reads free of observable mutation and identify writes as such. Using an
API correctly should require no investigation of hidden changes to state.
Assign shared mutable state an owner and specify how access is synchronized.

Check incoming external data at its entry point so internal operations can rely
on the accepted values. Respond to a particular failure where a useful response
is available; otherwise pass it to the caller responsible for handling it. Use
the project's error handling, configuration, and logging facilities.

Ordering is also a contract. Inspect how consumers use a sequence: sort only
when its order affects behavior. Choose the domain's comparison keys, including
a unique tie-breaker, and make pagination use that same comparison. An ordering
test must begin with data arranged differently from its expected output.

### Reading the code

Give identifiers accurate meanings and a grammatical form suited to their use.
Functions start with a specific verb, predicates and boolean values ask a
question, and types use noun phrases. Flags, counters, and cached values deserve
the same scrutiny as public names. Match accessors and conversions to the
expectations callers bring from the language and API.

Use as few words as clarity allows across the name's whole scope. Context
already apparent from the receiver or enclosing code need not be repeated;
abbreviations should be recognizable in the project or ecosystem. Name different
implementations for their behavior, and make the names of paired operations
recognizable as a pair.

Let the ordinary execution path remain visible. Exit early for guard cases,
give compound conditions names stating the fact under examination, and choose
straightforward control flow. Name constants for their domain meaning and
represent alternatives with appropriate types. Keep a declaration close to
where it is used and a check beside the value it validates. Arrange related
statements together; where permitted, show the public API before its private
support.

When understanding a passage needs an explanation, first improve the names or
structure. Comments supply information the code cannot convey: a necessary
constraint, an invariant, an exception's justification, or a specification
behind the behavior. Link the relevant specification. A class or coordinating
function may need a short statement of purpose beyond its name. Repair defects
in the code itself; explaining a defect does not resolve it.

## Completion

Before running validation, compare the diff with the intended design. Inspect
each added or renamed identifier for meaning, grammatical form, and clarity in
its scope. Reconsider the split of each new or substantially changed function
using **Calls and values**, and correct what this inspection reveals. Mention
exceptions that matter and issues still open; no list of routine identifier
checks is required.

Perform the applicable required formatting, linting, and tests. Respect the
agreed testing level, which can include adding no tests. Where tests are called
for, exercise behavior available to callers and relevant failures. Confirm that
a regression test detects the original bug. For a refactor, check the relevant
tests both before and after the edit.

Report completion only when the requested result and required checks are
demonstrated. If required evidence cannot be obtained, the result cannot pass;
say what remains unverified. A gap in optional evidence is a limitation to
report.

## Review

### Coverage and evidence

Assess the final implementation in context: its callers, the other side of any
changed contract, the task requirements, and the chosen testing level. Work
through all applicable **Code standards** within the review's scope, including
after finding a problem. Look for regressions and unsafe changes to touched
interfaces, as well as missing required validation. Explain which checks do
not apply or lack evidence without adding a reporting checklist.

A review judges the code available to inspect. Absent design history leaves the
process unknown. Judge test coverage against the agreed level and the confidence
it provides; additional tests have a maintenance cost as well as a benefit.

For a substantive issue, identify the inspected location, explain its practical
consequence, and propose a correction inside the task's scope. Read and search
before reporting an omission, and look for evidence that would disprove the
claim. Confusion caused by a name or structure can warrant a finding: describe
what the reader would misunderstand or have to reconstruct. Keep actual
failures, possible hazards, and appearance preferences distinct. Repairs, when
requested, follow **Design** and **Completion**.

### Checking and reporting conclusions

Substantive candidate findings need a second reader before they are treated as
settled or used to direct repairs. That reader examines the reviewed code and
the claim together, confirming that the cited location supports the applicable
rule, impact, and severity. A coordinator who did not author the finding can
perform this check; otherwise dispatch a fresh verifier using the settings
retained for the run. Check the candidates only. This adds neither a full
review nor a further check of the checking result. With no findings, or only
valid mechanical `validation_write:` markers, no second reader is needed.

Revise or withdraw claims that fail this check and explain the correction in
the existing summary. Retain those supported by evidence. A claim still in doubt
is an open question and may be reported as such without another worker. If the
route requires verified findings, unresolved checking prevents completion or
repair instructions; identify the claims awaiting a check.

Keep the reviewed state available for comparison. For task verification, use
the stored snapshot and compare it again after finding checks; retain and
compare the inspected state for other reviews too. Changes to that state
require a new review.

The review is complete when coverage, evidence, and second-reader checks are
resolved and remaining limitations are stated. An evidence-backed result of no
findings is valid. Report through the route's existing fields and summary; no
additional format or durable record is needed.

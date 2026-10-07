# Coding guidance

Read the sections for your assigned role before starting:

- Planning: **Design** and **Code standards**.
- Implementation or repair: **Design**, **Completion**, and **Code standards**.
- Verification, audit, or improvement review: **Review** and **Code standards**.

Apply these instructions within the assigned scope and agreed testing level.
Preserve the active route's approval requirements and worker selection. Use
existing plans, handoffs, result fields, and summaries.

## Design

1. Read repository instructions and relevant existing code. In `plan.approach`,
   state the observable result, the unit that owns the behavior, and the
   evidence needed for completion.
2. For an operation spanning several calls, sketch the controlling function's
   signature and call sequence before implementing its collaborators. Specify
   each called unit's inputs, result, and effects. The sequence must explain the
   operation without opening those implementations. For a local correction,
   identify the affected step in the existing function. Reuse an existing
   function where suitable; a plain function is enough.
3. Check proposed boundaries and data types against **Code standards**, including
   those supplied by a plan. Every collaborator and interface needs a current
   purpose. Choose useful, working increments that fit the repository's
   validation approach, including across layers.

If implementation needs a different contract, define the replacement before
building against it. Update the existing plan or handoff before another worker
relies on that contract. Adjustments within scope stay with the implementer.

## Completion

1. For a refactor, establish that relevant tests pass before changing the code.
2. Implement the design within scope and apply **Code standards**. Handle
   contract changes as specified in **Design**.
3. Before final validation, compare the diff with the intended design. Check
   every added or renamed identifier against **Names and readability**, and each
   new or substantially changed function against **Functions and values**.
   Correct the problems found.
4. Run required formatting, linting, and tests at the agreed testing level,
   including when no additional tests are agreed. Tests exercise public behavior
   and relevant failures. A regression test must detect the unfixed bug. After a
   refactor, rerun the relevant baseline tests.
5. Report the observable result, validation evidence, useful exceptions, and
   unresolved issues through the existing result fields. Routine identifier
   checks need no itemized report.

A pass requires the requested behavior and all required validation to be
demonstrated. Identify unavailable required evidence and leave the corresponding
claims unverified. Report unavailable optional evidence as a limitation.

## Review

1. Retain the inspected code state. For task verification, use the stored
   snapshot; for other reviews, retain comparable state.
2. Read the final code, its callers, and both sides of changed contracts. Cover
   every applicable **Code standards** rule within scope, task requirements, the
   agreed testing level, regressions, the safety of touched interfaces, and required
   validation. Continue through the full scope after finding a defect. Account
   for gaps and inapplicable rules in the existing summary. Missing design
   history leaves the authoring process unknown. Judge tests against the agreed
   level and relevant confidence, accounting for their maintenance cost.
3. Support each substantive candidate finding with a checked location, a concrete
   consequence, and an in-scope correction. Search before claiming an omission
   and seek counterevidence. Distinguish actual failures, possible hazards, and
   cosmetic preferences. A misleading name or difficult structure can warrant a
   finding when you explain the misunderstanding or reasoning burden it causes.
4. Before treating a substantive finding as settled or directing repairs, have a
   reader other than its author check the location, rule applicability,
   consequence, and severity against the retained state. An independent
   coordinator may do this; otherwise use a fresh verifier with the run's
   retained settings. Limit this check to candidate findings: it requires neither
   a full review nor a recursive check of its own result. Zero findings and valid
   mechanical `validation_write:` markers need no second reader.
5. Keep supported findings. Correct or withdraw overturned claims and explain
   the revision in the existing summary. Report unresolved claims as open
   questions; they require no additional checking worker. When the route requires
   verified findings, identify unchecked claims and block completion or repair
   instructions if their check is unavailable.
6. Compare the reviewed state again after checking findings. Any change requires
   a fresh review.
7. Report through the existing result fields and summary. Finish only when all
   applicable checks are covered, findings have evidence, independent checks are
   resolved, and remaining gaps are stated. An evidence-backed result of no
   findings is valid.

Requested repairs follow **Design** and **Completion**.

## Code standards

### Scope and conventions

Each edit must contribute to the requested result. Preserve adjacent code and
remove leftovers created by the change. An authorized rewrite includes
reconsidering names and boundaries throughout its unit; adjacent units remain
outside that rewrite.

Features and configuration need an expressed requirement. Defensive code needs
a reachable failure. Shared utilities resolve actual duplication. Interfaces
serve current variation or a need to test behavior independently.

Choose conventions in this order: tool-enforced requirements, adopted standards,
then ecosystem semantics callers depend on. Use local spelling and layout for
remaining choices. State the rule or practical constraint behind an exception.
Keep nearby defects outside the change rather than copying or repairing them.

### Responsibilities and dependencies

Keep domain decisions independent of storage, rendering, and framework details.
Entry points pass requests to domain logic. Persistence code handles data
access. Coordination assembles named calls.

Supply dependencies explicitly and choose their ownership and lifetime. An
infrastructure interface expresses the consuming code's needs. Give shared
mutable state an owner and a synchronization rule.

### Functions and values

A function receives only the data it uses and returns a typed result. Retain
results as data until rendering. Prefer transformations of immutable values to
shared bookkeeping.

Extract a block, even for one caller, when all of these conditions hold:

- A specific name describes its single job.
- Its name and signature predict the result and effects.
- A few typed arguments suffice without passing the caller's mutable bookkeeping.

Otherwise, keep it inline or choose another boundary. Inspect long or deeply
nested code for useful boundaries rather than splitting to meet a size limit.
Keep computations together when separation would break an invariant or repeat
substantial algorithmic work, and briefly comment the constraint.

Use named domain types where they clarify meaning or a contract, and explicit
public return types as the language expects. Represent optional results and
failures explicitly. Preserve distinctions between missing, null, and empty
values when the domain depends on them.

### API contracts and errors

Express guarantees through the API and its types where practical. Every
implementation, including test doubles, must honor them. Callers must be able to
use the API from its declared contract.

Prevent or explicitly reject calls that omit prerequisites or use stale state.
Return the value the next operation needs, or combine inseparable operations.
Offer dependent operations separately only when callers need that separation;
make the dependency explicit in their names and contracts.

Reads leave observable state unchanged. Writes disclose their mutation.

Validate external data on entry so internal calls can rely on the accepted
values. Handle specific failures where a meaningful response is available;
propagate the rest. Use the project's error handling, configuration, and logging
facilities.

### Sequence order

Inspect consumers before sorting a sequence. Where order affects behavior,
specify domain comparison keys and a unique tie-breaker. Use the same comparison
for pagination. An ordering test starts with input arranged differently from the
expected output. Leave order-insensitive consumers unsorted.

### Names and readability

Give every identifier accurate meaning and appropriate grammar, including
flags, counters, and cached values. Use specific verbs for functions, questions
for predicates and booleans, and noun phrases for types. Accessors and conversions
preserve the expectations callers bring from the language and API.

Use the fewest words that remain unambiguous and readable throughout the name's
scope. Omit context supplied by the receiver or enclosing code. Use recognized
project or ecosystem abbreviations. Distinguish implementations by behavior, and
give paired operations corresponding names.

Keep the main path visible through early guard exits and direct control flow.
Name compound conditions for the fact being checked, and constants for their
domain meaning. Express alternatives through types. Place declarations near use
and checks beside the values they validate. Group related statements. Show the
public API before private support where the language permits.

Improve names and structure before explaining code in comments. Comment
necessary constraints, invariants, justified exceptions, and behavior derived
from specifications; link the specification. Add a short purpose comment to a
class or coordinating function when its name needs that support. Repair a defect
within scope rather than justifying it in a comment.

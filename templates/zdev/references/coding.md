# Coding guidance

Read the sections for your assigned role before starting:

- Planning: **Design** and **Code standards**.
- Implementation or repair: **Design**, **Code standards**, and **Completion**.
- Verification, audit, or improvement review: **Code standards** and **Review**.

Apply the rules within the assigned scope and agreed testing level. The active
route retains its approval requirements and worker selection. Use existing
plans, handoffs, and result fields.

## Design

1. Read repository instructions and relevant existing code. Put the observable
   result, the unit that owns the behavior, and the evidence needed for
   completion in `plan.approach`.
2. For an operation spanning several calls, sketch the controlling function's
   signature and call sequence before implementing its collaborators. Specify
   each called unit's inputs, result, and effects. The sequence must explain the
   operation without opening those implementations. For a local correction,
   identify the affected step in the existing function. Reuse an existing
   function where suitable; a plain function is enough.
3. Assess proposed boundaries and data types against **Code standards**, even
   when supplied by a plan. Each collaborator and interface needs a present
   purpose. Choose useful, working increments that fit the repository's
   validation approach, including across layers.

If implementation requires a different contract, settle it before building
against it. Update the existing plan or handoff before another worker relies
on the replacement. Adjustments within scope stay with the implementer.

## Code standards

### Change size and placement

Each edit must contribute to the requested result. Preserve adjacent code and
remove leftovers created by the change. An authorized rewrite includes
reconsidering names and boundaries throughout its unit; adjacent units remain
outside that rewrite.

Features and configuration need an expressed requirement; defensive code needs
a reachable failure. Shared utilities resolve actual duplication. Interfaces
serve current variation or a need to test behavior independently.

Keep domain decisions independent of storage, rendering, and framework details.
Entry points pass requests to domain logic; persistence code handles data
access. Coordination assembles named calls. Supply dependencies explicitly
and choose their ownership and lifetime. An infrastructure interface expresses
the consuming code's needs.

Resolve convention choices through tool-enforced requirements, then adopted
standards, then ecosystem semantics callers depend on. Local spelling and
layout settle remaining choices. State the rule or practical constraint behind
an exception. Keep nearby defects outside the change rather than copying or
repairing them.

### Calls and values

A function receives only the data it uses and returns a typed result. Retain
results as data until rendering. Prefer transformations of immutable values
to shared bookkeeping.

Extract a block, even for one caller, when a specific name describes its single
job, its name and signature predict the result and effects, and a few typed
arguments suffice without passing the caller's mutable bookkeeping. All of
these requirements must hold; otherwise keep it inline or choose another
boundary. Inspect long or deeply nested code for such boundaries rather than
splitting to meet a size limit.

Keep computations together when separation would break an invariant or repeat
substantial algorithmic work. Briefly comment the constraint.

Use named domain types where they clarify meaning or a contract, and explicit
public return types as the language expects. Represent optional results and
failures explicitly. Preserve distinctions between missing, null, and empty
values when the domain depends on them.

### Caller contracts

Express guarantees through the API and its types where practical; every
implementation, including test doubles, must honor them. Prevent or explicitly
reject calls that omit prerequisites or use stale state. Return the value the
next operation needs, or combine inseparable operations. Offer dependent
operations separately only when callers need that separation, with the
dependency explicit in their names and contracts.

Reads leave observable state unchanged; writes disclose their mutation. Callers
must be able to use the API from its declared contract. Assign shared mutable
state an owner and a synchronization rule.

Validate external data on entry so internal calls can rely on the accepted
values. Handle specific failures where a meaningful response is available;
propagate the rest. Use the project's error handling, configuration, and
logging facilities.

Inspect sequence consumers before sorting. Where order affects behavior,
specify domain comparison keys and a unique tie-breaker; pagination uses the
same comparison. An ordering test starts with input arranged differently from
the expected output. Leave order-insensitive consumers unsorted.

### Reading the code

Give every identifier accurate meaning and appropriate grammar: specific verbs
for functions, questions for predicates and booleans, noun phrases for types.
Include flags, counters, and cached values. Accessors and conversions preserve
the expectations callers bring from the language and API.

Use the fewest words that remain unambiguous and readable throughout the name's
scope. Omit context supplied by the receiver or enclosing code. Use recognized
project or ecosystem abbreviations, distinguish implementations by behavior,
and give paired operations corresponding names.

Keep the main path visible through early guard exits and direct control flow.
Name compound conditions for the fact being checked and constants for their
domain meaning. Express alternatives through types. Place declarations near use
and checks beside the values they validate. Group related statements and show
the public API before private support where the language permits.

Improve names and structure before explaining a passage in comments. Comment
necessary constraints, invariants, justified exceptions, and behavior derived
from specifications; link the specification. Add a short purpose comment to a
class or coordinating function when its name needs that support. Repair a
defect in code rather than justifying it in a comment.

## Completion

1. For a refactor, establish passing relevant tests before editing.
2. Before final validation, compare the diff with the intended design. Check
   every added or renamed identifier under **Reading the code**, and each new
   or substantially changed function under **Calls and values**. Correct the
   problems found.
3. Run required formatting, linting, and tests at the agreed testing level,
   including an agreement to add no tests. Tests exercise public behavior and
   relevant failures. A regression test must detect the unfixed bug; rerun the
   relevant baseline tests after a refactor.
4. Report the observable result, validation evidence, useful exceptions, and
   unresolved issues through the existing result fields. A pass requires the
   requested behavior and all required validation to be demonstrated. Identify
   unavailable required evidence and leave the corresponding claims unverified;
   report unavailable optional evidence as a limitation.

Routine identifier checks need no itemized report.

## Review

### Coverage and evidence

Read the final code, its callers, and both sides of changed contracts. Assess
every applicable **Code standards** rule within scope, task requirements, the
agreed testing level, regressions, unsafe touched interfaces, and required
validation. Continue through the full scope after finding a defect. Account
for gaps and inapplicable rules in the existing summary.

Missing design history leaves the authoring process unknown. Judge tests
against the agreed level and relevant confidence, accounting for their
maintenance cost.

Each substantive candidate finding needs a checked location, a concrete
consequence, and an in-scope correction. Search before claiming an omission
and seek counterevidence. A misleading name or difficult structure can warrant
a finding: explain the misunderstanding or reasoning burden. Distinguish actual
failures, possible hazards, and cosmetic preferences. Requested repairs follow
**Design** and **Completion**.

### Checking and reporting conclusions

1. Retain the inspected code state. Use the stored snapshot for task
   verification and retain comparable state for other reviews.
2. Before treating a substantive finding as settled or directing repairs, have
   a reader other than its author check the location, rule applicability,
   consequence, and severity against that state. An independent coordinator
   may do this; otherwise use a fresh verifier with the run's retained settings.
   Limit this check to candidate findings. It requires neither a full review
   nor a recursive check of its own result. Zero findings and valid mechanical
   `validation_write:` markers require no second reader.
3. Keep supported findings. Correct or withdraw overturned claims and explain
   the revision in the existing summary. Report unresolved claims as open
   questions, which require no additional checking worker. When the route
   requires verified findings, identify unchecked claims and block completion
   or repair instructions if their check is unavailable.
4. Compare the reviewed state again after checking findings. Any change
   requires a fresh review.

Finish when all applicable checks are covered, findings have evidence,
independent checks are resolved, and remaining gaps are stated. Report through
the existing result fields and summary. An evidence-backed result of no
findings is valid.

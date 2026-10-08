# Choose questions for the area

The Model and portfolio reports used the same reading layout but answered
different questions. Keep that distinction. These prompts help select a
structure; they are not a questionnaire to fill mechanically.

## Questions common to an engineering report

- What problem does this work solve for its users, and what does ordinary use
  look like? Show one representative workflow before describing internals.
- What does the public interface promise? Which boundaries matter for
  correctness, ownership, failure, compatibility and performance?
- Where does work happen, what repeats, and what is shared? Use a small diagram
  when several stages or consumers make prose hard to follow.
- What do current source and existing results actually establish? Which older
  results still apply, and which repairs invalidate them?
- What are the exhibited defects, intentional tradeoffs, unsupported cases and
  unanswered questions? Give each its appropriate weight.
- What concrete work would change the assessment? How would we know it helped?

## Compiler, model or transformation area

Ask what is represented at each stage, when lowering occurs, how passes are
selected and scheduled, and whether they iterate to a fixpoint. Explain
observable values, temporary objects, copy semantics and lifetime separately.
Check whether a rewrite preserves solutions, multiplicity, readback and search
contracts. Distinguish fewer intermediate records from fewer runtime actors.

Measure construction, compilation, root creation and repeated execution at
comparable boundaries. Assess no-op inputs and rejected candidates as well as
successful rewrites. For borrowed ideas, identify the actual algorithm or data
structure and the solver assumptions needed for it to transfer.

The `compiler` fixture supplies a few headings to start this line of inquiry.

## Search, scheduling or portfolio area

Ask how workers coordinate, what state survives restarts, how stops and
exhaustion are distinguished, and when a result becomes valid globally.
Explain the first-solution phase separately from improvement or proof search.
Check what determines a neighbourhood or work partition, rather than relying
on its apparent size.

Compare configurations at equal resource budgets. Inspect trajectories,
accepted improvements, proof outcomes, setup and communication costs. Ask what
each added mechanism contributes and whether its interactions help. An oracle
that selects the best run afterwards is not a runtime policy.

The `portfolio` fixture supplies a different set of headings for these issues.

## Other areas

Use `blank` and derive questions from the public workflow. A storage area may
need durability and recovery; an API may need compatibility and adoption; a
frontend may need accessibility and error recovery. Keep only topics with a
real consequence for this area's users or maintainers.

# Discuss the brief

## When

Use **Discuss the brief** after zdev is active when the user wants to grill,
challenge, or sharpen an existing area brief before reviewing a task split.
Discussion updates `brief.md`; it does not create a transcript, second plan, or
tasks. Use this zdev interaction directly rather than invoking a separate
grilling skill.

## Do

1. Read `brief.md` first, then repository instructions and the source, tests,
   domain documentation, ADRs, and indexed background material relevant to the
   decisions under discussion. When the user asks about imported or background
   material, treat the relevant parts of that corpus as required input.
2. Identify unresolved choices that could materially change behavior, scope,
   task splitting, or validation. Do not assume the brief's open-question list
   is complete. Challenge a settled decision only with a concrete scenario,
   dependency, contradiction, or consequence.
3. Resolve factual questions from repository evidence. Ask the user about
   intent, trade-offs, scope, and hard-to-reverse choices.
4. Work breadth first. In each round, ask up to three independent, high-impact
   questions from different decision branches. Ask one question when its
   answer determines what follows or the user needs to explain freely. Never
   batch dependent questions.
5. Read the relevant evidence before asking. Give each question a recommended
   answer and its concrete trade-off; include alternatives only when they are
   viable. Test answers against specific scenarios, edges, or contradictions.

Use Oh My Pi's `ask` tool with its `questions` array so one call presents the whole round. Give each question concrete options and descriptions, put the recommended answer first, and reserve plain text for free-form explanation.

After each round, update the relevant decisions, boundaries, terms, open
questions, testing, or validation in `brief.md`. Record conclusions, not the
conversation. Preserve useful text and remove settled questions.

For the conventional `general` area, keep its standing brief focused on shared
rules. Carry one-off conclusions into the proposed task's context, outcome,
boundaries, done proof, and validation. Update the area brief only when a
standing rule changes. An optional slice brief may hold narrower shared context
for several related tasks.

If discussion suggests a project-document change outside `.zdev`, show its scope
and wait for confirmation unless the user already requested that edit.

## Stop

Stop when no unresolved choice would materially change the objective or task
split. Low-impact implementation details may remain for individual tasks.
Run `zdev check <area> --format json` once against the finished brief, correct
any reported issue, then summarize the resulting brief and remaining
non-blocking details. When the
user explicitly requested task creation and no material product or testing
choice remains, continue directly with **Create tasks** and show the exact
rendered task bundle for approval; no separate research interaction is needed.

Recommend one next interaction when the result points clearly to it. Then stop
unless the user already requested more work. Import tasks only after the user
approves the exact rendered bundle.

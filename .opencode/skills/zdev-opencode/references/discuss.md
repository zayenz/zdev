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

Use OpenCode's `question` tool with multiple questions in one call. Give each question concrete options, put the recommended answer first, and reserve plain text for free-form explanation.

After each round, update the relevant decisions, boundaries, terms, open
questions, testing, or validation in `brief.md`. Record conclusions, not the
conversation. Preserve useful text and remove settled questions. Run
`zd check <area> --format json` after meaningful updates.

If discussion suggests a project-document change outside `.zd`, show its scope
and wait for confirmation unless the user already requested that edit.

## Stop

Stop when no unresolved choice would materially change the objective or task
split. Low-impact implementation details may remain for individual tasks.
Summarize the resulting brief and any remaining non-blocking details.

If the user did not already request another interaction, offer the relevant
next actions and stop:

- **Create tasks** to draft a task split for separate review and approval.
- **Explore an objective** when discussion exposed a different path or boundary.
- **Investigate** when one named uncertainty still needs evidence.
- Continue **Discuss the brief** when the user identifies another material choice.

Do not import tasks or begin implementation unless the user explicitly
requested that next interaction. Task import still requires approval of the
exact rendered bundle.

+++
schema_version = 1
id = "general-007"
key = "ask-before-workflow-and-git-transitions"
area = "general"
status = "open"
complexity = "standard"
afk = true
priority = "high"
blocked_by = []
+++
# Ask before changing workflow phase or Git workspace

## Outcome

Exploration and discussion stop at the requested boundary, and every zdev interaction uses appropriate harness questions before an undecided Git or user-owned choice.

## Context

Read Discussion and task transitions and Git choices and questions in the [settled workflow brief](../../../plans/002-workflow-review-brief.md). The shaping route currently creates or switches branches before discussing the objective. Prototype guidance also prefers a temporary branch or worktree without asking. Only discuss.md receives question_tool_guidance, whose Codex wording assumes multiple questions. Start with templates/zdev/shared-contract.md, references/shape-work.md, discuss.md, setup.md, investigate.md, to-tasks.md, recovery.md, and src/integrations.rs. The settled behavior is to begin discussion in the current checkout, reuse prior authorization, and ask before an unrequested branch creation, branch switch, or worktree creation. A complete brief alone does not request task drafting; bundle import alone does not request implementation.

## Boundaries

- Preserve the direct path for explicitly requested, sufficiently specified tasks and implementation already authorized by the user.
- Keep CLI branch-binding defaults and existing branch-safety checks. Fix the general-area example to distinguish choosing a branch from binding the area.
- Use the existing harness adapters. Allow one question, batch only independent questions, respect tool restrictions on approvals, and treat unanswered questions as unanswered.

## Done when

- [ ] Rendered instructions across all five harnesses apply the question policy to setup, shaping, discussion, area selection, task approval, prototypes, recovery, and implementation decisions.
- [ ] Scenario review shows that an exploration request presents a brief for discussion; an explicit task request can proceed to drafting; and import proceeds to implementation only with authorization covering that work.
- [ ] Scenario review shows that an undecided branch or worktree choice prompts the user before mutation, while a previously stated choice is reused.
- [ ] Canonical sources, relevant user guidance, and regenerated integrations agree without retaining a conflicting Discuss-only question policy.

## Validation

- Regenerate the five checked-in harness integrations from canonical templates and run the existing rendering, discovery, and fixture-parity checks.
- Review the stated scenarios against the rendered instructions; avoid new tests that freeze incidental sentences.
- Run the standard validation in the area brief.

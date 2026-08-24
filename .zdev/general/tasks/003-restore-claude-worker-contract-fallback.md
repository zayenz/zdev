+++
schema_version = 1
id = "general-003"
key = "restore-claude-contract-fallback"
area = "general"
status = "open"
complexity = "standard"
blocked_by = []
+++
# Restore Claude worker-contract fallback

## Outcome

Claude task workflows load the installed task contract when resolvable and continue with the rendered inline contract when CLAUDE_PLUGIN_ROOT is absent or unusable.

## Context

The canonical Claude implement and verify workflows pass the literal `$CLAUDE_PLUGIN_ROOT/contracts/task-workflows.md` to child agents; loop and goal inherit the implement body through `claude_loop_workflow` composition. The user reports that, in Claude Code 2.1.227, a workflow-spawned child Bash has no `CLAUDE_PLUGIN_ROOT` and cannot load the contract. This user-observed runtime failure is the regression input; repository probes cover the generated child prompt rather than pretending to reproduce Claude's process environment. The Environment variables section of the official plugin reference at https://code.claude.com/docs/en/plugins-reference documents `${CLAUDE_PLUGIN_ROOT}` substitution in static skill and agent content and export to hooks, MCP, and LSP subprocesses, but does not establish availability in these dynamically spawned workflow children. Current workflow probes require the literal path and explicitly reject the inline fallback, contradicting the unconditional availability and fallback claims in docs/harness-orchestration.md. Restore a same-call rendered fallback through the existing `task_workflow_contract` template value.

## Boundaries

- Tell each child to load the installed contract only when `${CLAUDE_PLUGIN_ROOT:-}` is non-empty and the quoted `${CLAUDE_PLUGIN_ROOT}/contracts/task-workflows.md` is readable; otherwise use the rendered canonical contract included inline in the same worker prompt.
- Use the existing rendered `task_workflow_contract` value; do not add path discovery, installation-state probing, another worker call, or runtime state.
- Change the canonical implement and verify templates; cover loop and goal through their existing implement-workflow composition and regenerated fixtures.
- Preserve task selection, verification, rework, lifecycle, and commit semantics, and do not change other harness behavior.

## Done when

- [ ] Claude task workers receive usable canonical contract instructions when CLAUDE_PLUGIN_ROOT is unset, empty, or points to an unreadable contract.
- [ ] Generated Claude workflows prefer a readable installed contract but no longer turn an unavailable plugin-root path into an automatic task blocker when the rendered fallback is present.
- [ ] Focused workflow probes verify that generated worker prompts contain the guarded installed-path preference and rendered canonical fallback, and no longer require blocker-only behavior; they do not claim to execute a real Claude child Bash environment.
- [ ] docs/harness-orchestration.md removes the false unconditional availability statement, distinguishes the narrower documented static substitution/export guarantees from dynamically spawned workflow children, and records the prompt-size trade-off.
- [ ] The checked-in Claude fixture matches its canonical templates.

## Validation

- Regenerate the repository's checked-in Claude plugin fixture with `cargo run --locked -- skill install claude --to .claude/skills/zdev --force`.
- Run the focused Claude workflow probes and `cargo test --locked --test lean executable_templates_realize_deterministically_and_match_generated_fixtures`.
- Run the area-wide validation from brief.md.

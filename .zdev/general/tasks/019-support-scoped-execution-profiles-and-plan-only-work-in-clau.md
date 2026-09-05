+++
schema_version = 1
id = "general-019"
key = "select-scoped-execution-profiles-in-claude"
area = "general"
status = "done"
complexity = "advanced"
afk = true
priority = "normal"
blocked_by = ["general-018"]
+++
# Support scoped execution profiles and plan-only work in Claude Code

## Outcome

Claude Code can use Fable-based advanced workers or another named profile for a run or a one-off role while retaining zdev's phase and verification boundaries.

## Context

Read Selection and scope, Initial mappings, Plan-only requests, and Workflow coverage in the [execution profiles brief](../../../plans/004-execution-profiles-brief.md), then load the shared contract from the prerequisite. Claude's worker model and effort are currently rendered into templates/zdev/claude/agents/*.md; the planner references advanced-implementer variables. Its implementation, verify, audit, and loop workflow scripts dispatch fixed named agents. Extend these native seams and the ordinary-subagent fallback so selected profiles reach the intended roles. Start with src/integrations.rs and the controlled Claude workflow tests in tests/lean.rs; inspect actual Claude native model/effort controls before choosing dispatch overrides or prepared native definitions.

## Boundaries

- Use claude-fable-5-1 for the approved advanced seed and preserve existing normal settings and cheap coordination helpers. A planner-only override does not upgrade implementation or verification.
- Use supported native controls without rewriting shared installation or persistent settings to select each run. Preserve result validation, fresh independent verifiers, and existing fallback semantics.
- Do not silently substitute another model, expand native permissions, change task complexity, or start implementation after a plan-only request.

## Done when

- [x] The installed Claude skill and workflows accept and propagate shared run/role selections, retain their resolved concrete model/effort values across dispatches, and resolve the approved advanced Fable settings. Two runs with different selections do not overwrite each other's configuration, and mid-run preference edits do not change an already resolved run.
- [x] A plan-only request, including the exact advanced-planner wording, returns the selected task's read-only plan and then stops; subsequent implementation uses its own selected settings and the shared plan revalidation rule.
- [x] Implementation, required planning, verification, audits, task-draft challenge, and authorized continuation use the intended selected roles, with one-off choices ending at their specified boundary.
- [x] Configured model and effort reach real native dispatch controls or prepared definitions, and missing runtime capability is reported accurately through the existing fallback rather than treated as successful profile selection.
- [x] Existing normal role results, rework, escalation, blocker findings, and independent completion behavior remain intact, with updated canonical assets, documentation, and regenerated fixtures.

## Validation

- Extend existing controlled Claude workflows to exercise Fable model/effort selection, a planner-only override with no implementation dispatch, normal work afterward, and two differing run selections.
- Retain current planner/verifier parsing, rework, audit, loop, and fixture tests; review natural-language phase boundaries without sentence matching.
- Run integration discovery/parity and the standard area validation without requiring live Fable access or usage credits.

## Result

Added Claude scoped execution profiles and plan-only work with frozen five-role dispatch maps, native model/effort controls, strict deep admission, inheritance support, retained-plan validation, and continuation/parallel propagation.

Validation:

- Independent verification passed from exact snapshot W0e247c66ebc28d13: 8 focused Claude simulations, all 161 tests, deep malformed/inheritance cases, generated parity, fmt, clippy with warnings denied, build, and diff check.

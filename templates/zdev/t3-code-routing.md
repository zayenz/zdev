## Select orchestration before the workflow

An explicit request to use zdev **with T3 orchestration** selects
[T3 Code coordination](references/t3-code.md). Read that reference before
selecting native dispatch, worker profiles, or continuation. It supports
implementation, verification, plan-only work, audits, and goal/loop in a T3
main thread running Codex or Claude Code. Keep this selection for the logical
run, its completion wakes, and explicitly requested recovery. Ordinary requests
use the native harness route unless they continue that selected T3 run.

For the selected T3 route, the reference replaces the native dispatch, scoped
profile resolution, and goal/workflow instructions below. The shared zdev
ownership, safety, result, verification, and commit contracts still apply.
Availability of T3 tools alone does not select the route. An unavailable T3
route reports a blocker instead of substituting native orchestration.

Example: “Use zdev with T3 orchestration to loop through area scheduling,
using Codex for implementation and Claude Code for verification.”

## Select orchestration before the workflow

Select the orchestration route before any worker dispatch, packaged workflow,
or native goal. The native route is this harness's orchestration section below.

A session runs in T3 Code when T3's orchestration instructions are present in
it or T3's orchestration MCP server (`t3-code`) is connected. In that session,
[T3 Code coordination](references/t3-code.md) is the default route for
implementation, verification, plan-only work, audits, standalone review, and
goal/loop. An explicit request to use zdev **with T3 orchestration** also
selects the T3 route. An explicit request for **native orchestration** selects
the native route instead. A session with neither T3 signal keeps the native
route.

On the T3 route, read that reference and make its bounded
`orchestrator_capabilities` probe before any dispatch. When the probe fails,
report a blocker before dispatch and name the explicit request for native
orchestration as the way to opt out. Never fall back to the native route
silently.

On the T3 route, the reference replaces the native dispatch, scoped profile
resolution, packaged workflows, and goal/workflow instructions below. Its
verifier selection follows `t3.verifier-harness`. The shared zdev ownership,
safety, result, verification, and commit contracts still apply.

Keep the selected route, whether it is the default or was requested, for the
whole logical run. Every goal continuation, completion wake, and explicitly
requested recovery re-enters that route. The T3 goal condition keeps its
"Use zdev with T3 orchestration." prefix. Neither a goal continuation nor a
leading `/goal` switches a T3 run to native dispatch, the native area-loop, or
packaged workflows. An explicitly native run likewise stays native.

The T3 route does not support research runs yet. When the T3 route is selected,
a research-run request returns an unsupported-route result before any dispatch
and before any research-run start or resume. The result states that the T3
route does not support research runs, names the native research route
`references/research.md`, and states that an explicit request for native
orchestration selects it. Start no packaged or native research worker.

Example: “Use zdev with T3 orchestration to loop through area scheduling,
using Codex for implementation and Claude Code for verification.”

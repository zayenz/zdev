+++
schema_version = 1
id = "general-028"
key = "t3-default-route-visible-loop"
area = "general"
status = "done"
complexity = "advanced"
afk = true
priority = "high"
blocked_by = ["general-027"]
+++
# Default to the T3 route inside T3 Code and keep loop progress in the main thread

## Outcome

When a zdev coordinator runs inside T3 Code with T3's orchestration tools available, implement, verify, audit, review and goal/loop use the T3 route by default, so verifiers follow t3.verifier-harness (Codex Sol by default) and every worker appears as a T3 child task. A goal or loop is driven by the main-thread coordinator, which dispatches each phase as its own visible child and reports the selected task, each phase result and each commit in the main thread. It never runs the loop inside one opaque packaged workflow or a single delegated loop agent.

## Context

Maintainer feedback, 2026-10-28, with screenshots from several T3 Code threads: loops were run in inconsistent ways. A Gecode thread asked 'zdev loop model' and Claude launched the packaged `zdev-loop` Workflow as one background subagent; the main thread only said the loop was running in the background, and task phases were visible only inside the workflow view. Another thread (rostering) used a T3 goal. The maintainer wants the main thread to reflect what is happening.

Cause: templates/zdev/t3-code-routing.md (rendered into the Claude and Codex skills via `{{t3_code_routing}}`) selects the T3 route only on an explicit 'with T3 orchestration' request and says 'Availability of T3 tools alone does not select the route'. Ordinary requests in T3 Code therefore take the native route; for Claude that means templates/zdev/claude-skill.md's 'use the packaged continuation workflow when available' (around line 49), which is opaque in T3. It also means general-024's T3 verifier default (Codex Sol) never applies in practice. The T3 reference templates/zdev/references/t3-code.md already defines a main-thread coordinator loop with a visible coordinator goal (**Continue an area**, **Use the visible coordinator goal**, **Continue task phases**), so the needed behavior largely exists; it is not selected.

Decision (maintainer, 2026-10-28): inside T3 Code the T3 route is the default. Detect it by the T3 orchestration tools being available to the coordinator (for example a successful `orchestrator_capabilities` call); outside T3 Code nothing changes. An explicit request for native orchestration still selects the native route. The packaged Claude workflows remain the default for plain Claude Code.

Further decisions (coordinator, 2026-10-28, after review):
- Detection. A coordinator is in T3 Code when the session itself identifies T3 Code (T3's orchestration instructions or the t3-code MCP server are present). In that case it makes the bounded `orchestrator_capabilities` probe the T3 reference already describes, including the lazy-discovery and ACP fallbacks. A positively identified T3 session whose orchestration is unavailable reports a blocker before dispatch; it never silently restores native routing. A session with no T3 signal is ordinary and keeps the native route.
- Codex paths. The prohibition covers Codex too: templates/zdev/codex-skill.md (dispatch-spec and spawn_agent ordering around lines 66-79, native area-loop selection around lines 105-123) and templates/zdev/native-area-loop.md (around lines 61-66) apply only when the native route is selected.
- Research runs inside T3 Code are not supported by the T3 route yet. Before any dispatch they return an explicit unsupported-route result that names the native research route; the user can then ask for native orchestration explicitly. Do not silently launch packaged or native research workers.
- Goal text. src/goal.rs line 37 builds `native_goal` with 'Use worktrees by default', which contradicts the T3 bound-checkout rule (t3-code.md lines 35-41) and the workspace.branching on-request default from general-025. Change that sentence to keep the run's workspace choice and follow workspace.branching (current checkout by default), and update the tests and docs that pin it. This is the one allowed change to the goal contract.
- The dependency on t3-verifier-performs-full-review is rollout ordering: both tasks edit t3-code.md.

## Boundaries

- Outside T3 Code, routing and the packaged workflows are unchanged.
- Keep the T3 route's existing contracts (worker roles, dispatch keys, snapshots, verification, completion, commit, goal settlement, stop and recovery); add only what visibility and default selection need. The one exception is the native_goal workspace sentence named in Context.
- Parallel batches remain unsupported in the T3 route as today: an explicit parallel request inside T3 Code reports that and offers the sequential route or explicit native orchestration; do not silently fall back.
- Do not add new binary commands; this is routing and coordination guidance plus any existing-test updates.
- Edit templates and regenerate checked-in copies with the repo-built binary; never hand-edit generated files.

## Done when

- [x] t3-code-routing.md makes the T3 route the default when the coordinator is positively identified as running in T3 Code, keeps explicit 'with T3 orchestration' working, lets an explicit native request opt out, reports a blocker when a T3 session cannot reach orchestration, and no longer says tool availability alone does not select the route.
- [x] In the installed Claude and Codex instructions, native dispatch (dispatch-spec, spawn_agent, native area-loop) and the packaged Claude workflows (implement, verify, audit, loop/goal, parallel, research) are stated as conditional on the native route; a T3-selected coordinator is never directed to them, and the T3 selection persists across goal continuations and completion wakes.
- [x] t3-code.md states that the main-thread coordinator drives every goal/loop iteration itself: it dispatches each planner, implementer and verifier as its own T3 child task, posts the selected task, each phase result and each commit in the main thread around each dispatch, and never delegates a whole loop or task cycle to one child or workflow.
- [x] A research-run request inside T3 Code returns an explicit unsupported-route result naming the native research route before any dispatch.
- [x] native_goal no longer says 'Use worktrees by default'; it keeps the run's workspace choice and follows workspace.branching, with pinned tests and docs updated.
- [x] The verifier selection under this default follows t3.verifier-harness (general-024).
- [x] docs/t3-code.md (and any user-guide mention of T3 routing) and CHANGELOG.md Unreleased describe the new default, the native opt-out and the visible loop.
- [x] Generated copies match the templates and `zdev skill check` reports every harness ready.

## Validation

- Update the T3 routing and reference rendering assertions in tests/lean.rs (around lines 7525-7534) so they check, in the installed Claude and Codex instructions: the T3 default and native opt-out; that native dispatch and packaged workflows are conditional on the native route; persistence of the T3 selection; the main-thread loop rule; the research unsupported-route result; and the new native_goal wording. Check for the absence of contradictory unconditional instructions, not only for new phrases. No live T3 or harness run.
- cargo fmt --all -- --check; cargo clippy --locked --all-targets --all-features -- -D warnings; cargo test --locked; cargo build --locked; git diff --check

## Result

Inside T3 Code the Codex and Claude skills select the T3 route by default (T3 instructions or the t3-code MCP server present); 'with T3 orchestration' still selects it, an explicit native request opts out, and an unreachable orchestrator is a blocker naming the opt-out. Native dispatch, packaged Claude workflows and the native area-loop are explicitly native-route only, and the selection persists across continuations and wakes. The main-thread coordinator dispatches each planner, implementer and verifier as its own T3 child and posts the selected task, each phase result and each commit in the thread. Research runs in T3 return an unsupported-route result. native_goal follows workspace.branching instead of 'Use worktrees by default'. Verified by GPT-6 Astra (high), with one doc correction rechecked.

Validation:

- cargo fmt --all -- --check: pass
- cargo clippy --locked --all-targets --all-features -- -D warnings: pass
- cargo test --locked: pass (211)
- cargo build --locked: pass
- git diff --check: pass
- zdev skill check (codex portable, claude, opencode, pi, omp): ready

+++
schema_version = 1
id = "improvements-030"
key = "design-trunk-based-area-work"
area = "improvements"
status = "done"
blocked_by = []
+++
# Define first-class trunk-based area work

## Outcome

Produce an implementation-ready contract for explicitly running one or more areas directly on the configured project trunk, while retaining branch-isolated areas as the default and preserving Git ownership, verification, lifecycle, and commit safety.

## Context

AreaMetadata.branch is mandatory. Area create defaults it to the checked-out branch, area bind changes it, and validation permits only one area owner per branch. A single area whose branch equals configured trunk nevertheless works through the existing same-branch shortcut, bypassing anchor and child-history requirements. This is incidental rather than a complete mode: metadata and CLI do not express intent, multiple trunk areas are rejected, trunk reconfiguration is undefined, guidance assumes feature branches, and import, completion, rebase, and commit use different branch gates.

## Boundaries

- Research and design only; produce one design record and narrow follow-up tasks.
- Keep branch-isolated work as the default and preserve all existing area records without silently reinterpreting an area merely because its stored branch currently equals configured trunk.
- Do not create, switch, rename, delete, merge, rebase, or push branches automatically.
- Add no active-area lock, worktree or session state, concurrency service, or generic clean-worktree requirement.
- Do not weaken active Git-operation checks, three-part Git baselines, change attribution, independent verification, exact staging, rollback, or commit inspection.
- Preserve task lifecycle, area lifecycle, record policies, task ordering, and imports except where the contract identifies an unavoidable mode-specific rule.
- Do not redesign loops, derived-task authority, coordinator profiles, or round-trip optimization from tasks improvements-026 through improvements-029.
- Do not require multiple isolated areas to share a branch; any shared ownership rule applies only to explicitly represented trunk mode.

## Done when

- [x] A design record documents current behavior, including the same-branch shortcut and the differing gates for ordinary import, committed import, selection, completion, rebase, and commit.
- [x] It defines the exact durable representation, default, strict schema, human and JSON projection, and compatibility behavior for legacy records, including legacy areas stored on the current or former trunk.
- [x] It defines exact, unambiguous area create and area bind grammar for isolated versus trunk mode, conflicts and defaults, transition rules, and failures for detached HEAD or unconfigured or missing trunk.
- [x] It settles whether multiple explicit trunk areas may coexist, how ownership validation distinguishes them from isolated areas, and deterministic next --any behavior when several trunk areas are ready on checked-out trunk.
- [x] It defines what an explicit trunk area follows when project trunk is renamed or reconfigured, including open and closed areas, selected work, missing branches, atomicity, and recovery, without silently moving legacy isolated areas.
- [x] It defines every branch_status and task_work field for trunk mode, including checked-out branch, effective base, diagnostics, freshness, finalization, anchor validity, linear history, stale advisory, and active Git-operation behavior.
- [x] It settles base_commit behavior in trunk mode and defines create and bind migration accordingly.
- [x] It defines the legal parent matrix for trunk areas and exact no-op or rejection behavior for managed rebase.
- [x] It defines ordinary and committed task import, brief inclusion, task completion, reopen, and final commit behavior with unrelated staged, unstaged, and untracked trunk changes, including overlap and attribution rules.
- [x] It reconciles personal, project, and pull-request record policy and cleanup squash; unsupported combinations fail explicitly.
- [x] It maps required changes to project, task, config, status, selection, documentation, canonical guidance, executable workflow, and generated artifact seams without implementing them.
- [x] It traces isolated, explicit-trunk, mixed, and failure cases and produces thin follow-up implementation tasks with no remaining product decisions.

## Validation

- Inspect current create, bind, trunk configuration, relationship validation, status and task-work, parent and rebase, import and completion, selection, commit, and cleanup seams plus focused tests.
- Build a scenario matrix covering one and multiple trunk areas, mixed modes, legacy current or former trunk records, trunk reconfiguration, missing or wrong branches, detached HEAD, active Git operations, unrelated and overlapping changes, imports, parents, rebasing, record policies, cleanup, and deterministic selection.
- For each case record expected metadata, human and JSON status, allowed command or failure, and mutation boundary.
- Run documentation validation only; do not implement runtime behavior.

## Result

Defined an explicit trunk-area mode that preserves isolated defaults and legacy records while settling schema, transitions, shared-trunk ownership, status, Git safety, record policies, and implementation slices.

Validation:

- Independent verifier PASS after checking every task condition, current source contracts, trunk ancestry and override behavior, goal/status boundaries, scenario coverage, and implementation seams.
- cargo test --locked --test documentation-contract passed.
- cargo fmt, Markdown fence checks, state audit, and git diff --check passed.

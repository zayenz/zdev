# Worker profiles

This note records a design decision, not a permanent model ranking. Model
availability, aliases, and harness controls change. The evidence and suggested
defaults below were checked on 2026-08-20.

## Roles

Zdev exposes four worker roles. `implementer` and `verifier` are the standard
defaults; the other two are explicit implementation tiers rather than a role
matrix.

- `routine-implementer` handles authored routine tasks: tightly specified,
  low-risk mechanical work. It may edit only the selected task's exact
  implementation paths and collect narrow read-only evidence. It never performs
  final verification, coordinates lifecycle, stages, commits, or makes product
  decisions. Routine is never inferred or selected by default.
- `implementer` handles normal implementation and uses the standard profile.
- `verifier` independently reads the brief, task, baseline, diff, and relevant
  source; runs required checks; and returns `PASS`, `REWORK`, or `BLOCKER`. It
  needs careful requirement accounting, defect finding, and evidence handling.
  Its main risks are confirmation bias, trusting the implementer's summary, and
  accepting tests as a substitute for inspecting the change.
- `advanced-implementer` handles authored advanced implementation and explicit
  advanced rework. Advanced-task planning reuses it read-only; there is
  no separate planner or advanced-verifier key.

The coordinator is not another worker profile. It owns task selection, user
decisions, branch safety, dispatch, rework, completion, and commits. A verifier
must remain a fresh, read-only worker even when both roles use the same model.

## Observed evidence

These observations help choose initial defaults. They do not establish that a
model will have the same quality in another harness or repository.

DeepSWE v1 uses 113 original, long-horizon tasks across 91 repositories and five
languages. Its publication snapshot put GPT-5.5 at xhigh effort ahead of the
other reported configurations, with every model using mini-swe-agent. Its
trajectory analysis found family-specific instruction-following evidence:
GPT-5.5 had the lowest rate of missed stated behaviors and GPT-5.4 was close
behind, while the tested Claude configurations more often missed a parallel
requirement. The self-testing evidence crossed model families instead. GPT-5.4
and Claude Opus 4.7 wrote and ran new repository tests in more than 80% of
DeepSWE runs, while GPT-5.5 did so in 67%. These are observations about the
tested versions, prompts, and harness, not timeless family traits. [DeepSWE v1
methodology, publication snapshot, and trajectory
analysis](https://deepswe.datacurve.ai/blog/deepswe) and [the accompanying
paper](https://arxiv.org/abs/2607.07946) (accessed 2026-08-20).

DeepSWE v1.1 keeps those tasks but changes execution and grading. The agent now
works in a natural Git environment with no future history, commits its change,
and only that patch is applied and graded in a fresh verifier container. The
revision also adds structured test reports, fixes dependency drift, and removes
flaky tests. Its leaderboard, updated 2026-08-20, reports Claude Opus 5 at max
effort at 74% and GPT-5.6 Sol at max effort at 73%. Those close scores are still
results for specific configurations under DeepSWE's harness and isolated
grader, not a harness-independent ordering for zdev. [DeepSWE v1.1 methodology
and current results](https://deepswe.datacurve.ai/blog/deepswe-v1-1) (accessed
2026-08-20).

FrontierCode measures whether maintainers would merge a change. Its criteria
cover correctness, test quality, scope discipline, style, and repository
standards, using tests, rubrics, and other verifiers. Version 1.1 keeps the Main
and Extended sets and reports each model at its best tested reasoning effort.
The exported results retain the model's harness, so the scores are explicitly
model-and-harness observations. Current published results place Opus 5 and
GPT-5.6 Sol among the strong configurations, but do not isolate the model from
Claude Code, Codex, or Devin scaffolding. [FrontierCode leaderboard and
methodology](https://cognition.com/frontiercode), [FrontierCode 1.1
revision](https://cognition.com/blog/frontier-code-1.1), and [Epoch's
methodology record](https://epoch.ai/benchmarks/frontiercode) (accessed
2026-08-20).

Artificial Analysis Intelligence Index v4.1.1 combines agentic work, coding,
science, reasoning, and knowledge evaluations; it is not a software-maintainer
review score. Its current leaderboard places Claude Opus 5 and GPT-5.6 Sol in
the top group, and its GPT-5.6 report says Sol led its Coding Agent Index in the
Codex harness. These results support considering both models, but the aggregate
rank cannot select a zdev worker independently of role and harness.
[Intelligence Index methodology](https://artificialanalysis.ai/methodology/intelligence-benchmarking),
[v4.1.1 release](https://artificialanalysis.ai/articles/artificial-analysis-intelligence-index-v4-1-1),
and [GPT-5.6 report](https://artificialanalysis.ai/articles/gpt-5-6-has-landed)
(accessed 2026-08-20).

Confidence is moderate. The sources agree that the recommended models are
frontier-capable, and DeepSWE and FrontierCode exercise work close to zdev's
implementation contract. Confidence does not extend to a precise ordering:
benchmark tasks, prompts, graders, effort settings, and harnesses differ, and
the current releases have not all been compared under one zdev workflow.

## Harness controls

- Codex supports a spawned agent's explicit model and reasoning effort; these
  override the configured subagent defaults. Its current model guide describes
  `gpt-5.6-sol` as the flagship for complex coding and exposes high and higher
  effort settings. [Codex configuration
  reference](https://developers.openai.com/codex/config-reference/) and [Codex
  model guide](https://developers.openai.com/codex/models/) (accessed
  2026-08-20).
- Claude Code agent frontmatter accepts `model` and `effort`. A model can be an
  alias, a full ID, or `inherit`; effort supports `low` through `max` when the
  model supports it. Organization allowlists and provider-specific alias
  resolution can substitute or inherit another model with a warning.
  [Claude Code subagents](https://code.claude.com/docs/en/sub-agents) and [model
  configuration](https://code.claude.com/docs/en/model-config) (accessed
  2026-08-20).
- OpenCode agents accept a `provider/model-id`. With no agent model, a subagent
  inherits the invoking primary agent. Extra model options, including the
  documented OpenAI `reasoningEffort` example, are provider-specific; there is
  no portable effort field across providers. [OpenCode agent
  configuration](https://opencode.ai/docs/agents/) (accessed 2026-08-20).
- Pi accepts `--model provider/id` and `--thinking`, with levels from `off`
  through `max`. Its model metadata can omit, hide, or clamp unsupported
  levels. Zdev's Pi extension exposes all four worker profiles and passes each
  resolved model and thinking level to the isolated child process. [Pi CLI model options](https://github.com/earendil-works/pi/blob/main/packages/coding-agent/README.md)
  and [Pi model controls](https://github.com/earendil-works/pi/blob/main/packages/coding-agent/docs/models.md)
  (accessed 2026-08-20).
- Oh My Pi agent frontmatter accepts a prioritized model list and
  `thinking-level`. A settings override wins over frontmatter, then the parent
  model is the fallback. Unsupported effort is clamped, or the spawn fails when
  no supported level remains. [Oh My Pi task-agent
  discovery](https://github.com/can1357/oh-my-pi/blob/main/docs/task-agent-discovery.md)
  (accessed 2026-08-20).

## Suggested mappings

These are editable suggestions dated 2026-08-20, not automatic choices.
Standard is the normal default. Advanced uses the same frontier family at high
reasoning. Routine uses a cheaper documented model only when authored task
complexity explicitly requests it.
Where a harness can use more than one provider, using a different model family
for verification may reduce correlated misses. That is a zdev inference, not a
benchmark result.

| Harness | Routine implementer | Standard implementer | Standard verifier | Advanced implementer |
| --- | --- | --- | --- | --- |
| Codex | `gpt-5.6-luna`, `low` | `gpt-5.6-sol`, `low` | `gpt-5.6-sol`, `low` | `gpt-5.6-sol`, `high` |
| Claude Code | `haiku`, `low` | `claude-opus-5`, `low` | `claude-opus-5`, `low` | `claude-opus-5`, `high` |
| OpenCode | `openai/gpt-5.6-luna`, `low` | `openai/gpt-5.6-sol`, `low` | `anthropic/claude-opus-5`, effort inherited | `openai/gpt-5.6-sol`, `high` |
| Pi | `openai/gpt-5.6-luna`, `low` | `openai/gpt-5.6-sol`, `low` | `anthropic/claude-opus-5`, `low` | `openai/gpt-5.6-sol`, `high` |
| Oh My Pi | `openai/gpt-5.6-luna:low` | `openai/gpt-5.6-sol:low` | `anthropic/claude-opus-5:low` | `openai/gpt-5.6-sol:high` |

OpenAI documents [Luna](https://developers.openai.com/api/docs/models/gpt-5.6-luna)
as its cost-sensitive, high-volume model. Claude Code documents the
[`haiku` alias](https://code.claude.com/docs/en/model-config) as its fast,
efficient choice for simple tasks (accessed 2026-08-22). Those facts support
the routine defaults without creating a runtime catalog.

The mixed-provider rows assume both providers are configured. A project that
uses one provider should override both roles rather than depend on a hidden
substitution.

## Editable override contract

An optional repository file, `.zdev/workers.toml`, overrides the dated
suggestions for project-scoped integration installation. A global
`zdev/workers.toml` under the absolute configuration home supplies user
preferences and the fallback for projects without a local row. The worker file
is deliberately separate from `.zdev/config.toml`, whose schema describes
project and branch state rather than harness preferences. The exact global path
rules are in [Layered zdev configuration](config-command.md).

```toml
schema_version = 1

[codex.implementer]
model = "gpt-5.6-sol"
effort = "low"

[codex.verifier]
inherit = true

[codex.advanced-implementer]
model = "gpt-5.6-sol"
effort = "high"

[opencode.verifier]
model = "anthropic/claude-opus-5"
effort = "inherit"
```

Each optional table is named `<harness>.<role>`, using the five harness names
`codex`, `claude`, `opencode`, `pi`, and `omp` and the four roles above. A table
must contain either `inherit = true`, or both a non-empty `model` and an
`effort`. Effort is one of `inherit`, `low`, `medium`, `high`, `xhigh`, or
`max`. `inherit` as a whole table omits both controls; `effort = "inherit"`
sets the model but omits an effort control.

The file remains at schema version 1 because all four role tables are optional.
Existing files that contain only `implementer` and `verifier` keep their exact
whole-profile behavior.

Resolution is per harness and role:

1. A table in `.zdev/workers.toml` wins.
2. Otherwise a table in the global worker file wins.
3. Otherwise zdev uses the dated built-in suggestion above.
4. If the selected row says `inherit`, or zdev cannot express a built-in field
   in that harness, the generated integration omits that field and lets the
   harness inherit its native value.

Harness-native policy still applies after generation. For example, a Claude
Code environment override or an Oh My Pi settings override can supersede agent
frontmatter. Zdev should state that limitation; it should not claim an
effective runtime model that it cannot observe.

The parser rejects an unsupported schema version, unknown harness, role, key,
or effort; an empty model; `inherit` combined with `model` or `effort`; and a
model-effort pair the target adapter cannot express. `zdev skill install` and
`zdev skill check` report the file, table, and value and stop before publishing
any integration files. An explicit unsupported value is never silently
dropped.

Zdev can validate syntax and adapter capability without contacting a provider.
It cannot promise that a model is enabled for an account. If the harness later
rejects or substitutes the model, its native error or warning is authoritative;
zdev performs no automatic model search. The user can set that row to
`inherit = true` or choose an available model. Missing files and missing tables
are not errors: they use the built-in row, then native inheritance where the
table above records a gap.

## Implementation seam

Keep the implementation inside integration generation:

1. Use the strict parser for the optional local and global worker files. Resolve
   four complete role profiles for the requested harness before
   rendering.
2. Pass those resolved values into the existing canonical integration
   templates. Claude Code, OpenCode, and Oh My Pi write native agent metadata;
   Pi adds model and thinking arguments in its existing subagent extension;
   Codex supplies explicit model and effort when spawning each role.
3. Make install and check call the same resolver and renderer. Parse and render
   every artifact before replacing any destination, preserving the existing
   all-or-nothing publication rule.

No evaluator, benchmark runner, task corpus, telemetry, leaderboard sync,
automatic model selection, cost database, or new worker lifecycle belongs in
this change.

An implementation task is complete when:

- absent configuration produces the five mappings above, including the
  OpenCode verifier's omitted effort;
- every harness realizes the requested model and effort through its documented
  native control, and `inherit` omits both controls;
- repository overrides win over global profiles and defaults for project-scoped
  install and check, while user scope uses global profiles before defaults;
- invalid or explicitly unsupported configuration fails before any destination
  changes, with a useful location and value;
- install and check render identical deterministic bytes from the same inputs;
- focused integration tests cover one default, one override, one inherit or
  unsupported gap, and one pre-publication failure without building a model
  catalog or evaluation framework.

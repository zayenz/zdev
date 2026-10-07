# Autonomous research implementation landscape

Research date: 2026-10-07. This catalog compares 20 public repositories and
AlphaEvolve’s published design. Sources are primary repository documentation,
selected implementation files, official documentation, and the Arbor paper.
No external agent, training run, or benchmark was executed. “Borrow” identifies
a proposed ZDev adaptation, not an endorsed dependency or measured advantage.

Read the [brief](../brief.md) for the current scope. Read
[experiment mechanics](experiment-mechanics.md) for detailed measurement,
rollback, parallelism, and recovery implications.

## Coverage and shortlist

| System | Pattern | Relevance |
| --- | --- | --- |
| [karpathy/autoresearch](https://github.com/karpathy/autoresearch) | Minimal serial loop | Highest |
| [davebcn87/pi-autoresearch](https://github.com/davebcn87/pi-autoresearch) | General-purpose harness extension | Highest |
| [RUC-NLPIR/Arbor](https://github.com/RUC-NLPIR/Arbor) | Hypothesis coordination and worktrees | Highest |
| [WecoAI/aideml](https://github.com/WecoAI/aideml) | Draft/debug/improve search | Highest |
| [iii-experimental/n-autoresearch](https://github.com/iii-experimental/n-autoresearch) | Multi-GPU experiment infrastructure | High |
| [uditgoenka/autoresearch](https://github.com/uditgoenka/autoresearch) | Portable instruction-led optimization | High |
| [facebookresearch/spdl](https://github.com/facebookresearch/spdl) | Production pipeline experiment workflow | High |
| [WecoAI/SpecBench](https://github.com/WecoAI/SpecBench) | Search and evaluator comparison | High |
| [trevin-creator/autoresearch-mlx](https://github.com/trevin-creator/autoresearch-mlx) | Local hardware port and repeat measurements | High |
| [algorithmicsuperintelligence/openevolve](https://github.com/algorithmicsuperintelligence/openevolve) | Evolutionary program archive | Medium |
| [SakanaAI/ShinkaEvolve](https://github.com/SakanaAI/ShinkaEvolve) | Sample-efficient evolutionary search | Medium |
| [google-deepmind/funsearch](https://github.com/google-deepmind/funsearch) | Evaluator-guided program search | Medium |
| [microsoft/RD-Agent](https://github.com/microsoft/RD-Agent) | Research/development feedback loop | Medium |
| [SakanaAI/AI-Scientist](https://github.com/SakanaAI/AI-Scientist) | Template-based end-to-end science | Context |
| [SakanaAI/AI-Scientist-v2](https://github.com/SakanaAI/AI-Scientist-v2) | Managed exploratory experiment trees | Context |
| [EvoScientist/EvoScientist](https://github.com/EvoScientist/EvoScientist) | Long-lived research assistant memory | Context |
| [AutoResearch/autora](https://github.com/AutoResearch/autora) | Closed-loop empirical science | Context |
| [rbudnar/open-autoresearch](https://github.com/rbudnar/open-autoresearch) | Protocol and adoption scaffold | Context |
| [facebookresearch/aira-dojo](https://github.com/facebookresearch/aira-dojo) | Research-agent evaluation framework | Context |
| [facebookresearch/llm-speedrunner](https://github.com/facebookresearch/llm-speedrunner) | Reproduction and discovery benchmark | Context |
| [AlphaEvolve](https://deepmind.google/blog/alphaevolve-a-gemini-powered-coding-agent-for-designing-advanced-algorithms/) | Published evaluator-guided evolutionary design | Conceptual |

Start implementation study with **Karpathy, pi-autoresearch, Arbor, and AIDE**.
Together they cover a minimal metric loop, durable local evidence, hypothesis
coordination/worktrees, and bounded repair versus exploration. Add SPDL and
n-autoresearch when settling long-running experiments and resource limits.
OpenEvolve, ShinkaEvolve, and FunSearch inform future search policies; they do
not justify adding an evolutionary engine to ZDev now.

## Repository notes

### 1. karpathy/autoresearch

**Observed:** A fixed training budget, a protected evaluator, one editable training file, a baseline, and keep/discard/crash results make the optimization question explicit. The human edits the instructions; the agent edits the candidate.

**ZDev adaptation:** Borrow the narrow experiment contract and readable result. Replace the untracked TSV and whole-branch reset with ZDev-owned task results and scoped rollback. Hardware-local measurements are not automatically comparable across machines.

**Inspection:** README and program.md. Source: [README at 228791f](https://github.com/karpathy/autoresearch/blob/228791fb499afffb54b46200aca536f79142f117/README.md).
Selected files: [program.md](https://github.com/karpathy/autoresearch/blob/228791fb499afffb54b46200aca536f79142f117/program.md).

### 2. davebcn87/pi-autoresearch

**Observed:** Provides initialization, command execution, metric tracking, correctness checks, an ideas backlog, and disk-based session reconstruction. Hypothesis and rollback explanations survive reverted code; changed assumptions can justify revisiting an earlier discard.

**ZDev adaptation:** Closest lightweight reference for general-purpose local work and memory. Its broad staging/cleanup and advisory noise score need adaptation. ZDev already owns task state and worker orchestration, so another extension runtime is unnecessary.

**Inspection:** README, experiment tools, JSONL reconstruction, compaction, and revisit example. Source: [README at 04b8a50](https://github.com/davebcn87/pi-autoresearch/blob/04b8a50dac7efe80ffc91648bd00ed9b022e24c2/README.md).
Selected files: [extensions/pi-autoresearch/index.ts](https://github.com/davebcn87/pi-autoresearch/blob/04b8a50dac7efe80ffc91648bd00ed9b022e24c2/extensions/pi-autoresearch/index.ts), [extensions/pi-autoresearch/jsonl.ts](https://github.com/davebcn87/pi-autoresearch/blob/04b8a50dac7efe80ffc91648bd00ed9b022e24c2/extensions/pi-autoresearch/jsonl.ts), [extensions/pi-autoresearch/compaction.ts](https://github.com/davebcn87/pi-autoresearch/blob/04b8a50dac7efe80ffc91648bd00ed9b022e24c2/extensions/pi-autoresearch/compaction.ts), [site/content/revisiting-discards.md](https://github.com/davebcn87/pi-autoresearch/blob/04b8a50dac7efe80ffc91648bd00ed9b022e24c2/site/content/revisiting-discards.md).

### 3. RUC-NLPIR/Arbor

**Observed:** A long-lived coordinator maintains hypotheses and insights; short-lived executors implement candidates in isolated worktrees. The records separate development and test scores, code references, attempts, and experiment validity.

**ZDev adaptation:** Closest overall structural match. Borrow durable questions, lessons, assigned worktrees, and current-baseline acceptance. A full hypothesis-tree store and autonomous research runtime would duplicate ZDev. Check source enforcement rather than relying on README acceptance claims.

**Inspection:** README, hypothesis records, worktree tools, acceptance code, and convergence excerpts. Source: [README at 7cdaf1f](https://github.com/RUC-NLPIR/Arbor/blob/7cdaf1fa6d779b3d5e340052357bf3fe55dfed93/README.md).
Selected files: [src/coordinator/idea_tree.py](https://github.com/RUC-NLPIR/Arbor/blob/7cdaf1fa6d779b3d5e340052357bf3fe55dfed93/src/coordinator/idea_tree.py), [src/coordinator/tools/worktree.py](https://github.com/RUC-NLPIR/Arbor/blob/7cdaf1fa6d779b3d5e340052357bf3fe55dfed93/src/coordinator/tools/worktree.py), [src/coordinator/tools/git_ops.py](https://github.com/RUC-NLPIR/Arbor/blob/7cdaf1fa6d779b3d5e340052357bf3fe55dfed93/src/coordinator/tools/git_ops.py), [src/coordinator/convergence.py](https://github.com/RUC-NLPIR/Arbor/blob/7cdaf1fa6d779b3d5e340052357bf3fe55dfed93/src/coordinator/convergence.py).

### 4. WecoAI/aideml

**Observed:** AIDE represents candidate programs as journal nodes. Its policy creates initial drafts, sometimes repairs buggy leaves, then improves the best valid node. The journal carries parent, code, execution, analysis, and metric information.

**ZDev adaptation:** Borrow the separation of a new hypothesis, experiment repair, and improvement. Explicitly separate unusable evaluation from valid poor performance. Start with task links and simple selection; tree search is a later policy choice.

**Inspection:** README and agent/journal implementations. Source: [README at 60b3978](https://github.com/WecoAI/aideml/blob/60b3978ddf65b71f86eb7c64506965048a1398cf/README.md).
Selected files: [aide/agent.py](https://github.com/WecoAI/aideml/blob/60b3978ddf65b71f86eb7c64506965048a1398cf/aide/agent.py), [aide/journal.py](https://github.com/WecoAI/aideml/blob/60b3978ddf65b71f86eb7c64506965048a1398cf/aide/journal.py).

### 5. iii-experimental/n-autoresearch

**Observed:** Registers a hypothesis before training, records results centrally, manages a GPU pool, and suggests search directions from keep rates, crashes, underexplored categories, and near misses.

**ZDev adaptation:** Borrow resource acquisition, central acceptance, and evidence-driven next-idea prompts. GPU availability and agent availability are separate. Its iii workers, HTTP functions, KV state, and category heuristics are infrastructure-specific, not requirements for ZDev.

**Inspection:** README, Python orchestrator, and GPU state. Source: [README at 26f239c](https://github.com/iii-experimental/n-autoresearch/blob/26f239c4f0587b1a83c3b27e7b35f061242da9ee/README.md).
Selected files: [workers/orchestrator/orchestrator.py](https://github.com/iii-experimental/n-autoresearch/blob/26f239c4f0587b1a83c3b27e7b35f061242da9ee/workers/orchestrator/orchestrator.py), [workers/gpu/src/state.rs](https://github.com/iii-experimental/n-autoresearch/blob/26f239c4f0587b1a83c3b27e7b35f061242da9ee/workers/gpu/src/state.rs).

### 6. uditgoenka/autoresearch

**Observed:** Generalizes the loop around a goal, scope, metric, verification command, baseline, and iteration limit. It supports several coding harnesses and emphasizes keeping simpler implementations when the result is comparable.

**ZDev adaptation:** Borrow intake questions and a concise metric-oriented route. Many surrounding commands cover debugging, security, content, and shipping; they are broader than this feature. Do not import that entire command suite.

**Inspection:** README and core autoresearch command instructions. Source: [README at 050e30d](https://github.com/uditgoenka/autoresearch/blob/050e30dc4ba0974b03f2873111b9901ec3211390/README.md).
Selected files: [.agents/skills/autoresearch/autoresearch.md](https://github.com/uditgoenka/autoresearch/blob/050e30dc4ba0974b03f2873111b9901ec3211390/.agents/skills/autoresearch/autoresearch.md).

### 7. facebookresearch/spdl

**Observed:** Separates a bounded asynchronous engine from a pipeline-specific workflow. Experiment phases and job identity support resume. Follow-up experiments form a hypothesis tree; domain logic owns stopping and cancellation.

**ZDev adaptation:** Borrow phase-aware recovery and compute-aware concurrency, especially for long experiments. The full platform/capability architecture serves remote pipeline builds and training jobs; a local first version needs much less structure.

**Inspection:** Official autoresearch documentation, architecture, and core type/workflow excerpts. Source: [README at 9beff53](https://github.com/facebookresearch/spdl/blob/9beff53052004b289a24f22f9934dc1793e0b0f7/README.md).
Selected files: [docs/source/autoresearch/autoresearch_architecture.rst](https://github.com/facebookresearch/spdl/blob/9beff53052004b289a24f22f9934dc1793e0b0f7/docs/source/autoresearch/autoresearch_architecture.rst), [src/spdl/autoresearch/core/_types.py](https://github.com/facebookresearch/spdl/blob/9beff53052004b289a24f22f9934dc1793e0b0f7/src/spdl/autoresearch/core/_types.py), [src/spdl/autoresearch/core/_workflow.py](https://github.com/facebookresearch/spdl/blob/9beff53052004b289a24f22f9934dc1793e0b0f7/src/spdl/autoresearch/core/_workflow.py).

### 8. WecoAI/SpecBench

**Observed:** Offers AIDE, linear, and autoresearch modes using coding harnesses. Public feedback guides the agent; private test results are recorded separately. Runs track step/time/cost limits and trajectories.

**ZDev adaptation:** Borrow an honest comparison between measured optimization and independent correctness/generalization. The loop is a useful reference for explicit budgets. Its benchmark results do not establish which policy is best for ZDev workloads.

**Inspection:** README and benchmark loop implementation. Source: [README at 0860735](https://github.com/WecoAI/SpecBench/blob/08607352adc8abd78be2193dd9f725f1f032b8f0/README.md).
Selected files: [benchmarks/spec_bench/run_loop.py](https://github.com/WecoAI/SpecBench/blob/08607352adc8abd78be2193dd9f725f1f032b8f0/benchmarks/spec_bench/run_loop.py).

### 9. trevin-creator/autoresearch-mlx

**Observed:** Ports the fixed-budget loop to Apple Silicon MLX. A helper retains repeated observations, avoids identical code proposals, and compares candidate and accepted means with bootstrap resampling.

**ZDev adaptation:** Demonstrates that hardware belongs in the comparison conditions and that small gains need confirmation. The helper labels repeats as seeds without passing a seed per invocation; borrow the measurement concern, not an assumed statistical guarantee.

**Inspection:** README, program.md, and rigor.py. Source: [README at 766a25f](https://github.com/trevin-creator/autoresearch-mlx/blob/766a25ff22afa799efd8d0aa450a4348e4749df2/README.md).
Selected files: [program.md](https://github.com/trevin-creator/autoresearch-mlx/blob/766a25ff22afa799efd8d0aa450a4348e4749df2/program.md), [rigor.py](https://github.com/trevin-creator/autoresearch-mlx/blob/766a25ff22afa799efd8d0aa450a4348e4749df2/rigor.py).

### 10. algorithmicsuperintelligence/openevolve

**Observed:** Maintains a program population and islands, selects parents and inspirations, evaluates generated candidates, and supports checkpoints and staged evaluation. Configuration exposes stopping and parallel evaluation choices.

**ZDev adaptation:** Borrow cheap validity checks before expensive measurements and retaining useful alternatives. Population archives, islands, ensembles, and diversity machinery are optional search strategies, not a minimal task workflow. Configuration marks some infrastructure features as unimplemented.

**Inspection:** README, default configuration, and selected database/evaluator source. Source: [README at 9196d87](https://github.com/algorithmicsuperintelligence/openevolve/blob/9196d8763300d1e46cc8b48cb0dc987966db3d48/README.md).
Selected files: [configs/default_config.yaml](https://github.com/algorithmicsuperintelligence/openevolve/blob/9196d8763300d1e46cc8b48cb0dc987966db3d48/configs/default_config.yaml), [openevolve/evaluator.py](https://github.com/algorithmicsuperintelligence/openevolve/blob/9196d8763300d1e46cc8b48cb0dc987966db3d48/openevolve/evaluator.py), [openevolve/database.py](https://github.com/algorithmicsuperintelligence/openevolve/blob/9196d8763300d1e46cc8b48cb0dc987966db3d48/openevolve/database.py).

### 11. SakanaAI/ShinkaEvolve

**Observed:** Combines model-generated mutations with program evolution, candidate evaluation, archives, and parallel execution. It includes model-selection and proposal-budget controls alongside island-based search.

**ZDev adaptation:** Borrow the distinction between preparing proposals and consuming evaluator capacity, and account for proposal cost. Adaptive model selection, crossover, novelty embeddings, and evolutionary databases can wait until simpler research tasks demonstrate a need.

**Inspection:** README, core configuration, and parent selection excerpts. Source: [README at 8adc053](https://github.com/SakanaAI/ShinkaEvolve/blob/8adc053a2ce4511ad2ac310e004c530a73fb974a/README.md).
Selected files: [shinka/core/config.py](https://github.com/SakanaAI/ShinkaEvolve/blob/8adc053a2ce4511ad2ac310e004c530a73fb974a/shinka/core/config.py), [shinka/database/parents.py](https://github.com/SakanaAI/ShinkaEvolve/blob/8adc053a2ce4511ad2ac310e004c530a73fb974a/shinka/database/parents.py).

### 12. google-deepmind/funsearch

**Observed:** Pairs generated functions with evaluation and a program database that preserves useful search diversity. The public implementation explains the evolutionary method and supplies example domains.

**ZDev adaptation:** Borrow objective feedback and keeping diverse promising alternatives. The repository explicitly omits the generating language model, execution sandbox, and distributed infrastructure, so it is a method reference rather than a ready autonomous worker system.

**Inspection:** README and evaluator/program database implementations. Source: [README at cc53f27](https://github.com/google-deepmind/funsearch/blob/cc53f274237d7ab05c19df939edbc1f9616a7c19/README.md).
Selected files: [implementation/evaluator.py](https://github.com/google-deepmind/funsearch/blob/cc53f274237d7ab05c19df939edbc1f9616a7c19/implementation/evaluator.py), [implementation/programs_database.py](https://github.com/google-deepmind/funsearch/blob/cc53f274237d7ab05c19df939edbc1f9616a7c19/implementation/programs_database.py).

### 13. microsoft/RD-Agent

**Observed:** Separates proposing a hypothesis from developing and running its experiment, then feeds results into the next research iteration. Its scenarios apply that loop to data and model work.

**ZDev adaptation:** Borrow the separation of reasoning about an experimental question from engineering the measurement. ZDev already has coordinator, planner, implementation, and verification roles; another general agent framework would add little for the first version.

**Inspection:** README and framework design documentation. Source: [README at 484776c](https://github.com/microsoft/RD-Agent/blob/484776c211e4fbbeef03e0ec00d6bbee7362a4f4/README.md).
Selected files: [docs/project_framework_introduction.rst](https://github.com/microsoft/RD-Agent/blob/484776c211e4fbbeef03e0ec00d6bbee7362a4f4/docs/project_framework_introduction.rst).

### 14. SakanaAI/AI-Scientist

**Observed:** Starts from experiment templates and baseline runs, generates ideas, executes studies, analyzes results, and writes papers. Templates give the system a concrete experimental starting point.

**ZDev adaptation:** Borrow runnable starting conditions and baseline establishment. Literature novelty, paper writing, plots, and publication review solve a wider research problem than optimizing a repository metric.

**Inspection:** README and template/baseline workflow documentation. Source: [README at 1de1dbc](https://github.com/SakanaAI/AI-Scientist/blob/1de1dbc1f4ee2c5f61e9c94348d55eb51d7fa2eb/README.md).

### 15. SakanaAI/AI-Scientist-v2

**Observed:** Uses an experiment manager and progressive agentic tree search with worker, step, seed, draft, and debugging controls. It broadens exploration beyond the first version’s hand-authored templates.

**ZDev adaptation:** Borrow bounded experimentation and handling repair depth separately from fresh exploration. The README itself distinguishes broad exploration from reliable work on a strong template; do not assume a more elaborate agent organization always helps.

**Inspection:** README and best-first-tree configuration. Source: [README at 96bd516](https://github.com/SakanaAI/AI-Scientist-v2/blob/96bd51617cfdbb494a9fc283af00fe090edfae48/README.md).
Selected files: [bfts_config.yaml](https://github.com/SakanaAI/AI-Scientist-v2/blob/96bd51617cfdbb494a9fc283af00fe090edfae48/bfts_config.yaml).

### 16. EvoScientist/EvoScientist

**Observed:** Combines specialized research roles with persistent, distilled observations across sessions and tools for resuming research work. Recurring patterns can become reusable skills.

**ZDev adaptation:** Borrow compact reusable lessons and retrieval before proposing another experiment. Cross-session memory graphs, automatic skill synthesis, channels, and a living research assistant are wider than area-local hypothesis tasks.

**Inspection:** README feature and memory/resume documentation. Source: [README at 3b382b9](https://github.com/EvoScientist/EvoScientist/blob/3b382b92b8fdad6e1f13467c382dcdd91d9b19d2/README.md).

### 17. AutoResearch/autora

**Observed:** Separates experimentalists that choose conditions, runners that collect observations, and theorists that infer models. Shared state contains variables, conditions, experimental data, and models.

**ZDev adaptation:** Useful conceptual check: an experiment changes conditions and measures observations; code is only one possible intervention. Keep ZDev’s initial implementation grounded in repository experiments without claiming every research process reduces to code mutation.

**Inspection:** README and terminology documentation. Source: [README at fc5cc3e](https://github.com/AutoResearch/autora/blob/fc5cc3ea213a5f097c72eeaf3a16016f8065acd5/README.md).
Selected files: [docs/terminology.md](https://github.com/AutoResearch/autora/blob/fc5cc3ea213a5f097c72eeaf3a16016f8065acd5/docs/terminology.md).

### 18. rbudnar/open-autoresearch

**Observed:** Supplies a protocol, copyable configuration/templates, reference verification scripts, and example campaigns. It distinguishes advisory in-agent rules from controls enforced outside the experimenting agent.

**ZDev adaptation:** Borrow explicit evaluator ownership and accurate claims about what is checked. It is a protocol template, not a working research agent; its extensive specification and promotion tiers would be excessive for the initial ZDev feature.

**Inspection:** README and adoption/threat-model positioning. Source: [README at d65b331](https://github.com/rbudnar/open-autoresearch/blob/d65b33171604edb57e41e16fdb1e472a1736b34c/README.md).

### 19. facebookresearch/aira-dojo

**Observed:** Separates a task’s environment, action space, and evaluator from solver operators and search policy. A runner can execute many solver-task runs with isolated execution and scheduler integration.

**ZDev adaptation:** Borrow the distinction between an optimization problem and a policy that searches it. ZDev should permit different future selection strategies without building a multi-solver benchmark platform or Slurm integration now.

**Inspection:** README and task/solver/runner descriptions. Source: [README at c795d86](https://github.com/facebookresearch/aira-dojo/blob/c795d8649d30b77fa9b8c011b978bba7127507ad/README.md).

### 20. facebookresearch/llm-speedrunner

**Observed:** Separates ideation, implementation, execution, and analysis. Tasks can supply hints for reproducing known improvements or omit them for discovery; workspace templates and task configuration define starting conditions.

**ZDev adaptation:** Borrow a reproducible starting workspace and a controlled demonstration of the feature. Given hypotheses and autonomous discovery exercise different capabilities; evaluate those separately before crediting the new loop with useful research.

**Inspection:** README and scientist scaffold description. Source: [README at ec3e4af](https://github.com/facebookresearch/llm-speedrunner/blob/ec3e4afd6dcf7c99f7b0be35898e40df6e1257c9/README.md).

## 21. AlphaEvolve — published design, separate evidence level

DeepMind describes combining model-generated code changes with automated
evaluators and evolutionary selection to improve algorithms. It provides an
important generalization of function-level search to larger code artifacts.
The useful lesson is that an evaluator and measurable objective can drive code
search across domains. This study inspected the official design description,
not AlphaEvolve’s internal production implementation; OpenEvolve and ShinkaEvolve
are independent projects, not evidence that the original system is reproduced.

Source: [DeepMind’s AlphaEvolve announcement](https://deepmind.google/blog/alphaevolve-a-gemini-powered-coding-agent-for-designing-advanced-algorithms/).

## Research papers and documentation to revisit

- [Arbor / Hypothesis-Tree Refinement](https://arxiv.org/abs/2606.11926):
  explains the long-lived coordinator, short-lived executors, and cumulative
  hypothesis/insight organization. Preprint and authors’ implementation; reported
  benchmark improvements are not independently reproduced here.
- [AIDE](https://arxiv.org/abs/2502.13138): read when comparing candidate selection
  and draft/debug/improve policies. This study used the public implementation
  as the concrete reference rather than adopting performance claims.
- [AI Scientist v2](https://arxiv.org/abs/2504.08066): read only if the feature
  later needs broader scientific exploration and experimental managers.
- [SPDL autoresearch documentation](https://facebookresearch.github.io/spdl/main/autoresearch/index.html):
  domain-specific optimization examples and architecture; source inspection
  complements the documentation.
- [FunSearch paper](https://www.nature.com/articles/s41586-023-06924-6): read when
  considering diversity-preserving program search. The public implementation
  is sufficient for the first comparison.

These are follow-up reading pointers. Repository and paper claims about speed,
quality, or benchmark success were not used to rank expected ZDev performance.

## What the comparison does and does not establish

The close implementations repeatedly separate the objective, allowed changes,
evaluation, and remembered results. Several preserve alternative ideas or
support parallel evaluation, but they differ substantially in task structure,
resource accounting, candidate acceptance, and interruption behavior. Those
differences are the useful design material.

README-level coverage is labeled in each entry; only the selected files listed
were inspected in code. This is a comparative source study, not an audit or
qualification of these repositories. Search hits, stars, dashboards, advertised
“autonomy,” and claimed benchmark gains are not evidence that a workflow will
improve a ZDev user’s metric. No external implementation was copied or installed.

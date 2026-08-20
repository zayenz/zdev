# Adapted methods

Zdev maps methods from external agent skills onto one storage model: an area
brief, individual task files, a generated summary, and Git history. The method
mapping preserves their judgment while zdev tasks provide the durable record.

## Source mapping

| Source method | Zdev adaptation |
| --- | --- |
| Matt Pocock's grilling and domain-modeling skills | `discuss.md`: identify high-impact decisions, challenge independent branches breadth first, and keep settled synthesis in `brief.md`. |
| Matt Pocock's `to-issues` and `to-tickets` | `to-tasks.md`: create reviewed tracer-bullet task files with real blocking edges. |
| Matt Pocock's wayfinder | `shape-work.md`: directly explore an objective and compare plausible paths while building the area brief. |
| Matt Pocock's planning and specification skills | `shape-work.md`, `discuss.md`, and the task format: shared decisions live in the brief; each task is its own implementation specification. |
| Matt Pocock's TDD and implementation skills | `implement.md`: test behavior at a stable seam, then enter independent verification. |
| Matt Pocock's research, diagnosis, and prototype skills | `investigate.md`: answer one question without creating another durable state system. |
| Matt Pocock's code-review and codebase-design skills | `improve.md` and `verify.md`: demand evidence, vet findings, and inspect task boundaries. |
| Explicit multi-agent review | `improve.md`: use ephemeral focused reviewers and report vetted candidate work with visible next choices. |
| shadcn's Improve skill | `improve.md`: reconnoitre, audit, vet, prioritize, and report findings without creating tasks. |
| Poteto Noodle's `unslop` skill | Shared zdev guidance: edit zdev-authored human-facing prose for plain language, specificity, natural rhythm, and fewer formulaic AI patterns. |

## Poteto Noodle skill audit

This audit covers all 29 `SKILL.md` files under
[`poteto/noodle/.agents/skills`](https://github.com/poteto/noodle/tree/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills)
at commit `82d2921c52370f23f29086de81ccfb600939c037`. It also checks the
references needed to distinguish portable review or execution guidance from
Noodle's brain vault, scheduler, event protocol, provider routing, and worktree
manager.

For this inventory, **adopt** means adding a skill to zdev substantially as-is,
**adapt** means incorporating a portable method into an existing zdev method,
and **skip** means making no zdev change. A skipped domain skill may still be
useful to a coding harness. It does not belong in zdev's general workflow.

| Skill | Decision | Evidence and rationale |
| --- | --- | --- |
| [`adversarial-review`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/adversarial-review/SKILL.md) | Skip | The architect, skeptic, and minimalist lenses are portable, as is the lead's duty to reject reviewer overreach. Zdev `improve` already uses bounded reviewers by category and vets every finding; `verify` already requires a fresh independent reviewer. Its mandatory opposite-model CLI, brain principles, scheduled trigger, and temporary output protocol add Noodle machinery rather than a missing review guarantee. |
| [`ast-grep`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/ast-grep/SKILL.md) | Skip | The test-snippet-first structural search workflow is useful tool guidance. Zdev should not prescribe a repository search tool, and the method adds nothing to area, task, or verification behavior. |
| [`brain`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/brain/SKILL.md) | Skip | It requires an Obsidian-style `brain/` vault, wikilinks, index maintenance, and a separate memory taxonomy. Zdev deliberately keeps durable state to briefs, tasks, metadata, generated summaries, and Git while leaving domain documents authoritative. |
| [`codex`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/codex/SKILL.md) | Skip | It is project-specific Codex CLI, profile, model, background-process, and worktree guidance. Zdev's harness integrations and worker profiles own cross-harness dispatch without importing one repository's model pins or CLI assumptions. |
| [`commit`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/commit/SKILL.md) | Skip | Logical commits and explicit ownership are portable, but zdev already records the baseline, stages exact task-owned paths, inspects the index, and adds a stable change ID. Requiring Noodle's conventional-commit and worktree conventions would weaken that repository-sensitive contract. |
| [`debugging`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/debugging/SKILL.md) | Skip | Reproduce, read the error, isolate, test hypotheses, fix the cause, and rerun the original failure already define zdev `investigate`. The remaining checks target `.noodle`, tmux, and the brain vault. |
| [`execute`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/execute/SKILL.md) | Skip | Scope discipline, decomposition, verification, and attribution already exist in zdev `implement` and `verify`. This skill assumes Noodle tasks, mandatory worktrees, autonomous commits, teams, and a `stage_yield` event, all outside zdev's coordinator-owned lifecycle. |
| [`find-skills`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/find-skills/SKILL.md) | Skip | It searches a marketplace and installs project-local skills. Zdev is not a skill marketplace, and installing capabilities is unrelated to durable task work. |
| [`frontend-design`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/frontend-design/SKILL.md) | Skip | This is a frontend design domain skill, not a software-work method. Repositories that need it can select it through their harness guidance. |
| [`go-best-practices`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/go-best-practices/SKILL.md) | Skip | Its Go lifecycle, concurrency, configuration, testing, and CI rules are domain policy. Zdev defers such choices to repository instructions and the task's testing contract. |
| [`interaction-design`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/interaction-design/SKILL.md) | Skip | Framer Motion, timing, animation, and accessibility guidance is useful only for relevant interface work. It adds no general zdev behavior. |
| [`make-interfaces-feel-better`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/make-interfaces-feel-better/SKILL.md) | Skip | The typography, surface, animation, and hit-area checklist is another interface domain guide. Zdev should pass applicable repository or harness skills to a task, not absorb their design opinions. |
| [`meditate`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/meditate/SKILL.md) | Skip | Its useful filter—keep only high-signal, frequent, or high-impact knowledge—matches zdev's preference for small authoritative briefs. The actual method snapshots and rewrites the brain, auto-memory, skills, and `.noodle` state through scheduled agents and commits, creating a second maintenance lifecycle. |
| [`noodle`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/noodle/SKILL.md) | Skip | This skill documents Noodle itself: `.noodle.toml`, mise, scheduled orders, stages, providers, events, runtimes, and automatic worktree merging. Copying it would amount to building the orchestration runtime excluded by the area brief. |
| [`oops`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/oops/SKILL.md) | Skip | Its reproduce-diagnose-fix-verify loop duplicates zdev `investigate`; interrupted Git and task-state recovery already lives in zdev `recovery`. The rest assumes scheduled repair, Noodle state, tmux, and autonomous commits. |
| [`plan`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/plan/SKILL.md) | Skip | Scope questions, repository exploration, alternatives, and bounded phases overlap zdev `shape`, `discuss`, and task creation. Brain plans, automatic skill installation, rigid file-count phase sizing, autonomous commits, and `stage_yield` conflict with zdev's approved brief and task bundle. |
| [`quality`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/quality/SKILL.md) | Skip | Scope, code, tests, runtime evidence, and independent judgment already form zdev `verify`. Its fixed Go checks, brain principles, scheduler-facing `stage_message`, and automatic backlog filing are Noodle-specific. |
| [`react-best-practices`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/react-best-practices/SKILL.md) | Skip | These 47 client-side React performance rules are domain guidance. They belong in applicable repository or harness instructions, not the zdev lifecycle. |
| [`refine`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/refine/SKILL.md) | Skip | Turning vague work into a self-contained prompt overlaps zdev `shape`, `discuss`, and exact task-bundle review. It is coupled to `brain/todos.md` and its separate backlog format. |
| [`reflect`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/reflect/SKILL.md) | Skip | Routing lessons to durable documentation or structural checks is sensible, but zdev already updates briefs or project documents when a settled decision warrants it. A scheduled post-task brain, skill, and backlog mutation would expand zdev's record and silently create work. |
| [`review`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/review/SKILL.md) | Skip | Architecture, code, tests, performance, evidence, and bounded findings overlap zdev `improve` and `verify`; material choices belong in `discuss`. Its brain principles, marketplace search, automatic audit files, todos, and commits introduce a parallel review record. |
| [`ruminate`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/ruminate/SKILL.md) | Skip | It mines complete Claude and Codex conversation archives into the brain using provider-specific paths and a team of analysis agents. Zdev keeps transcripts ephemeral and does not need bulk history extraction, implicit memory, or the associated privacy and context cost. |
| [`schedule`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/schedule/SKILL.md) | Skip | The one-plan-at-a-time heuristic is less useful than zdev's explicit ready-task selection and dependency graph. The skill otherwise depends on mise, orders, events, task-type schedules, provider routing, process runtimes, and autonomous dispatch. |
| [`skill-creator`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/skill-creator/SKILL.md) | Skip | Context economy and progressive disclosure are good skill-authoring advice, but zdev renders one canonical workflow into supported harnesses; it is not a general skill creation or packaging tool. |
| [`testing`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/testing/SKILL.md) | Skip | Reproduction and behavioral regression tests already fit zdev `investigate` and `implement`. Mandatory TDD, separate failing-test commits, Noodle fixtures, and fixed commands conflict with each area's proportionate testing decision. |
| [`todo`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/todo/SKILL.md) | Skip | `brain/todos.md` is a second backlog and lifecycle. Zdev tasks already provide stable IDs, dependencies, status, summaries, and commits. |
| [`ts-best-practices`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/ts-best-practices/SKILL.md) | Skip | TypeScript narrowing and modeling rules are domain policy. Zdev should use repository guidance rather than impose them on every task. |
| [`unslop`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/unslop/SKILL.md) | Adapt | This decision predates the audit. Shared zdev guidance already paraphrases its portable preference for plain, specific prose, natural rhythm, restrained formatting, and a final check for formulaic writing. Zdev does not install or invoke the upstream skill. |
| [`worktree`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/worktree/SKILL.md) | Skip | CWD-safe `git -C` use and explicit isolation are sound, but the method requires `noodle worktree`, ephemeral branches, automatic rebasing, merging, cleanup, dependency installation, and shared brain state. Zdev binds an area to a persistent branch and keeps rebasing explicit; it must not switch, merge, or clean worktrees automatically. |

The result is 0 adopted skills, 1 existing adaptation, and 28 skips. No new
follow-up is justified. Every portable method with clear workflow value already
has a home in `shape-work.md`, `investigate.md`, `improve.md`, `implement.md`,
`verify.md`, or `recovery.md`. The remaining skills either supply optional
domain expertise or require the Noodle runtime. Reconsider a skipped skill only
after a concrete zdev failure demonstrates value that those methods do not
provide.

Zdev embeds the grilling method in `discuss.md`; it does not invoke a separate
`grill-me` skill. Discussion reads the area's brief and relevant indexed
sources, identifies decisions that could materially change the work, and
challenges independent high-impact branches breadth first. It writes settled
conclusions back to the brief and stops when no unresolved choice could
materially change behavior, scope, task splitting, or validation. Task creation
still requires an explicit request.

The method mapping uses these pinned source revisions:

- Matt Pocock's skills at
  [`d574778f94cf620fcc8ce741584093bc650a61d3`](https://github.com/mattpocock/skills/tree/d574778f94cf620fcc8ce741584093bc650a61d3)
  (v1.1.0), with the frontier-grilling update at
  [`b8fd9afa42a6eebcfdcfc5007c42ef2367911000`](https://github.com/mattpocock/skills/commit/b8fd9afa42a6eebcfdcfc5007c42ef2367911000).
- shadcn's Improve skill at
  [`03369ee6d7cafbfcecc4346539b05b3dc0a603bb`](https://github.com/shadcn/improve/tree/03369ee6d7cafbfcecc4346539b05b3dc0a603bb).
- Poteto Noodle's complete `.agents/skills` tree at
  [`82d2921c52370f23f29086de81ccfb600939c037`](https://github.com/poteto/noodle/tree/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills),
  including the canonical
  [`unslop`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/unslop/SKILL.md)
  source already adapted into shared guidance.

These links document the source material; they are not runtime dependencies.
Zdev does not load an installed upstream skill while following its own workflow.
Changes to the mapping require explicit review against a pinned upstream
revision.

Each adaptation is a self-contained zdev reference. It reports relevant next
actions but does not call an upstream skill or silently invoke another zdev
method.

The Noodle adaptation keeps the upstream method's preference for plain,
specific prose, natural rhythm, restrained formatting, and a final check for
formulaic AI writing. Zdev's workflow, terminology, scope rules, exclusions,
and precedence for user instructions and approved area, slice, and task
contracts remain original zdev guidance. The adaptation is documentation, not
a runtime rewriting step or a separately installed skill. Upstream changes are
adopted only after a manual review against a newly pinned revision.

## Licensing and attribution

The source materials actually adapted by zdev are MIT-licensed. The [Matt Pocock source
license](https://github.com/mattpocock/skills/blob/d574778f94cf620fcc8ce741584093bc650a61d3/LICENSE)
credits Matt Pocock. The [Improve license
statement](https://github.com/shadcn/improve/blob/03369ee6d7cafbfcecc4346539b05b3dc0a603bb/README.md#license)
identifies shadcn's skill as MIT © shadcn.
Poteto Noodle's root
[MIT license](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/LICENSE)
states `Copyright (c) 2026 Lauren Tan`. The audit paraphrases methods and keeps
the pinned source and copyright attribution. Copying substantial Noodle text in
a future adaptation would require retaining that MIT notice.

Two skipped directories, `frontend-design` and `skill-creator`, carry their own
Apache License 2.0 files inside the Noodle tree. The root MIT grant should not
be treated as replacing those narrower notices. Zdev copied no material from
either skill. Any future reuse would need a fresh license review and the
applicable Apache license, attribution, notice, and modification requirements.

Zdev does not load these upstream skills as runtime dependencies. Zdev's own
code and documentation are covered by the [MIT license](../LICENSE).

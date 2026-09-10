# Adapted methods

Zdev adapts planning, implementation, and review methods from external agent
skills. The adaptations use zdev briefs, task files, and Git history to record
the work.

## Source mapping

| Source method | Zdev adaptation |
| --- | --- |
| Matt Pocock's grilling and domain-modeling skills | `discuss.md`: identify high-impact decisions, challenge independent branches breadth first, and record decisions in `brief.md`. |
| Matt Pocock's `to-issues`, `to-tickets`, and `writing-for-agents` | `shape-work.md`, `to-tasks.md`, and workflow contracts: keep agent-readable briefs, tasks, context pointers, and worker instructions focused on their branch, authoritative context, and checkable completion. |
| Matt Pocock's wayfinder | `shape-work.md`: directly explore an objective and compare plausible paths while building the area brief. |
| Matt Pocock's planning and specification skills | `shape-work.md`, `discuss.md`, and the task format: shared decisions live in the brief; each task is its own implementation specification. |
| Matt Pocock's TDD and implementation skills | `implement.md`: test observable behavior, then enter independent verification. |
| Matt Pocock's research, diagnosis, and prototype skills | `investigate.md`: answer one question without creating another durable state system. |
| Matt Pocock's code-review and codebase-design skills | `improve.md` and `verify.md`: demand evidence, vet findings, and inspect task boundaries. |
| Explicit multi-agent review | `improve.md`: use focused reviewers, check their findings, and report proposed work. |
| shadcn's Improve skill | `improve.md`: inspect, audit, check, prioritize, and report findings without creating tasks. |
| Cursor pstack's `unslop` skill by Lauren Tan | Shared zdev guidance: preserve meaning and tone while editing zdev-authored prose for concrete active language, readable rhythm, stable terms, and fewer formulaic AI patterns. |

Each adaptation lives in a self-contained reference under `skills/zdev/`.
Zdev does not load upstream skills at runtime. Discussion records decisions in
the brief; creating tasks remains a separate, explicitly requested action.

The method mapping uses these pinned source revisions:

- Cursor pstack's `unslop` skill at
  [`93b00b89ef425a9c1bac0d0b317dfc49c930ac99`](https://github.com/cursor/plugins/blob/93b00b89ef425a9c1bac0d0b317dfc49c930ac99/pstack/skills/unslop/SKILL.md).
- Matt Pocock's `writing-for-agents` skill at
  [`3cca18b368ae95cdbdebbff572ccafa662551015`](https://github.com/mattpocock/skills/blob/3cca18b368ae95cdbdebbff572ccafa662551015/skills/productivity/writing-for-agents/SKILL.md).
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

Changes to an adaptation require review against a pinned upstream revision.
Repository instructions and approved area, slice, and task requirements still
govern the work.

## Licensing and attribution

The source materials adapted by zdev are MIT-licensed. The [Matt Pocock source
license](https://github.com/mattpocock/skills/blob/3cca18b368ae95cdbdebbff572ccafa662551015/LICENSE)
credits Matt Pocock. Cursor pstack's [MIT
license](https://github.com/cursor/plugins/blob/93b00b89ef425a9c1bac0d0b317dfc49c930ac99/pstack/LICENSE)
credits Lauren Tan. The [Improve license
statement](https://github.com/shadcn/improve/blob/03369ee6d7cafbfcecc4346539b05b3dc0a603bb/README.md#license)
identifies shadcn's skill as MIT © shadcn. Poteto Noodle's root [MIT
license](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/LICENSE)
states `Copyright (c) 2026 Lauren Tan`. Zdev's own code and documentation are
covered by the [MIT license](../LICENSE).

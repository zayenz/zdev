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
- Poteto Noodle's canonical `unslop` skill at
  [`82d2921c52370f23f29086de81ccfb600939c037`](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/.agents/skills/unslop/SKILL.md).

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

The adapted source materials are MIT-licensed. The [Matt Pocock source
license](https://github.com/mattpocock/skills/blob/d574778f94cf620fcc8ce741584093bc650a61d3/LICENSE)
credits Matt Pocock. The [Improve license
statement](https://github.com/shadcn/improve/blob/03369ee6d7cafbfcecc4346539b05b3dc0a603bb/README.md#license)
identifies shadcn's skill as MIT © shadcn.
Poteto Noodle's root
[MIT license](https://github.com/poteto/noodle/blob/82d2921c52370f23f29086de81ccfb600939c037/LICENSE)
states `Copyright (c) 2026 Lauren Tan`.

Zdev does not load these upstream skills as runtime dependencies. Zdev's own
code and documentation are covered by the [MIT license](../LICENSE).

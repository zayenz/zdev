# Method provenance

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

These sources provide provenance, not runtime dependencies. Zdev does not load
an installed upstream skill while following its own workflow. Changes to the
mapping require explicit review against a pinned upstream revision.

Each adaptation is a self-contained zdev reference. It reports relevant next
actions but does not call an upstream skill or silently invoke another zdev
method.

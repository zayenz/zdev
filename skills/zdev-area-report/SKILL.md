---
name: zdev-area-report
description: Research and write a reader-oriented engineering report for a zdev area, then generate a self-contained HTML reading copy. Use for area reviews, readiness assessments, or reports combining implementation, experiments and next steps.
---

# Zdev area report

Produce an evidence-backed report that a reader can understand without the
development conversation. Use the bundled Astro renderer for presentation;
spend the research effort on the area's actual questions. Deliver one static,
self-contained HTML file that the user can share directly. Astro is a build
tool, not a server or a requirement for the reader.

## Establish the review boundary

Find the requested area's checkout and read its brief, tasks, applicable
repository guidance, implementation and existing measurements. Record the
source revision, working-tree changes relevant to the review, and the reader's
likely background. Ask only about choices that materially change the report.

A report request authorizes research and report artifacts. Keep product fixes,
task imports, release approval, commits and expensive new measurement campaigns
as separate actions unless the user also requested them. Reuse decisions the
user has already made. A completed task is evidence of completed work, not an
automatic release verdict.

## Research questions before sections

Choose questions that would help this area's reader assess its design and
results. Start with [question prompts](references/questions.md), then replace
or omit prompts that do not fit. A compiler and a search portfolio need
different questions even when their reports share a layout.

Follow the important public workflows through implementation and callers.
Check substantive findings against source and seek evidence that would disprove
them. Where delegation is permitted, simpler sub-agents are useful for bounded
research: tracing one public workflow, extracting a benchmark comparison, or
checking one design claim. Give each a specific question, relevant sources,
scope boundaries and a separate notes file to write. Ask for evidence, limits
and unresolved questions, not just a verdict. Prefer a smaller or faster model
when its capabilities fit the question and model selection is available.
The report author checks consequential claims and reconciles the findings.
Research delegation is not independent verification; use a separate review
when appropriate and state which conclusions received only your own review.

Read the underlying results behind decision-changing numbers. Distinguish the
measured revision from current source, supported results from inference, and
completed solves from timeouts. Research related systems through primary
sources where a comparison helps explain a concrete design choice.

Research is sufficient when each proposed conclusion has supporting evidence
or an explicit uncertainty, and the next steps follow from those conclusions.
A fixed section count or a large bibliography is not a completion criterion.

## Keep research on disk

Initialize the report directory using [generation](references/generation.md)
before substantial research. Keep supporting Markdown documents in `notes/`
beside `report.mdx`. Start with one working note; split by question or topic
when it helps, especially for parallel research. Save useful tables, small
result files and command output there when needed, or link to existing evidence
instead of copying large datasets.

Write down findings as you go: the question, source locations or URLs, relevant
revision and measurement conditions, observations, interpretation and remaining
uncertainty. Record accepted decisions and changes to earlier conclusions.
After an interruption or context reset, reread these documents. Draft from the
saved evidence rather than relying on conversation history or model memory.
Keep this lightweight; no additional tracking framework is needed.

Supporting documents are working material, not reading prerequisites. Bring
the evidence and explanation needed for the report's conclusions into the HTML;
the reader should not need the notes directory to understand the assessment.

For a refresh, use the saved notes to identify which source, measurements or
decisions changed. Reuse findings whose supporting evidence still applies;
revisit affected questions and record that choice in a short `notes/refresh.md`.
Reassess the overall conclusion even when most notes remain valid. An older
report can point to evidence, but its conclusion is not new supporting evidence.

## Write the reading copy

Read [writing and evidence](references/writing.md) before drafting. If available,
use the user's `unslop` and `zayenz-writing-style` skills; the bundled guidance
also makes the workflow usable without those personal installations.

Lead with the reason for the report and the assessment. Explain the user-facing
problem before implementation details. Introduce concepts through a small real
example, then develop architecture, semantics, measurements and recommendations
in the order needed by this reader. Use paragraphs and tables by default.

Keep citations next to claims and include the review boundary. Preserve older
reports as dated evidence; write a coherent current assessment instead of
appending another contradictory verdict. Separate proposed follow-ups from
tasks that already exist. Do not turn every limit into a blocking defect.

## Generate the artifact

Read [generation](references/generation.md) for commands, metadata, links and
optional widgets. Reports live in the reviewed repository:

```text
.zdev/reports/<area>/<YYYY-MM-DD>-<name>/
  report.mdx       # editable prose, metadata and citations
  notes/           # supporting Markdown research and small evidence files
  widgets/         # optional report-specific Astro components
  index.html       # generated, standalone reading copy
```

If not already initialized, use `scripts/report.mjs init` with an explicit
local date and a relevant fixture. The fixtures are starting points, not claims
about the project.
Replace their prompts with researched prose. Use `build` to generate HTML;
edit the MDX rather than the generated file. The renderer supplies navigation,
responsive tables, typography and printing. Add a widget only when changing
an input makes an important relationship easier to understand; keep the
essential facts readable without JavaScript.

## Review and deliver

Reread as someone who knows less than you do. Check definitions, examples,
units, comparison boundaries, source links and the distinction between facts
and recommendations. Apply the writing pass before changing `status` to
`final`. A final report is finished writing, not an approval of the area.

Build, inspect the actual HTML at desktop and narrow widths, check tables and
navigation, and exercise any widget. Use the recipient-style browser check in
[generation](references/generation.md) when browser execution is available.
The build checks local links, anchors and single-file output; those checks do
not establish the truth of the prose.
Report visual checks that were unavailable rather than claiming them.

Return a link to `index.html`, its editable source, the principal conclusion
and material limits on verification. State whether code, tasks or defaults
changed. Leave unrelated work in the checkout intact.

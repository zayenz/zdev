# Generate a report

Run the helper from any directory. Replace `SKILL_DIR` below with the absolute
directory containing this skill's `SKILL.md`. Node 22.12 or newer is required;
the package and lockfile pin the Astro/MDX dependencies. Install them once in
the skill directory with `npm ci --prefix "$SKILL_DIR"`. This does not change
the reviewed project's dependencies.

```sh
node "$SKILL_DIR/scripts/report.mjs" init \
  --project /absolute/repository --area model \
  --date 2026-10-02 --name review --fixture compiler

node "$SKILL_DIR/scripts/report.mjs" build \
  /absolute/repository/.zdev/reports/model/2026-10-02-review
```

Choose `blank`, `compiler`, `portfolio`, or `example`. The first three contain
writing prompts; `example` is a complete, explicitly fictional format sample
with an optional widget. Init requires an existing zdev area, refuses an
existing report directory, and prints the created path. Use the user's local
date explicitly. A same-day second report can use a different name.

The report follows the repository's existing `.zdev` tracking policy. No area
or task is created for reports, and no automatic commit or publication occurs.
This companion skill is available in the source checkout and Codex plugin;
`zdev skill install` still installs the existing task-workflow skill only.

## Editable source

`report.mdx` has ordinary YAML frontmatter:

```yaml
title: Review of the model interface
area: model
date: "2026-10-02"
revision: "8d0fd37d4c, with relevant uncommitted changes described below"
scope: "Source inspection and recorded experiments; no new solver campaign."
status: draft
```

All six fields are required. Set `status: final` after reviewing the report.
The title is the page's H1. Use `##` for report sections; the renderer derives
navigation from them. Use `###` for subsections. Regular Markdown paragraphs,
lists, tables, links and fenced code work in MDX. Escape raw braces or angle
brackets in prose when MDX would interpret them as code or markup.

Links are relative to the report directory, not the shared renderer. For this
directory depth, the area brief is `../../../<area>/brief.md` and a repository
file is `../../../../path/to/file`. Prefer stable source URLs with commit and
line anchors when the report will leave the checkout. The builder checks local
file existence and in-report anchors; it does not fetch external links or check
Markdown heading fragments. Keep enough evidence in the text for an offline
reader, even when source links are unavailable.

## Components and optional widgets

Most reports need no component imports. Markdown tables are made horizontally
scrollable automatically. Use `<aside className="callout">...</aside>` for a
short interpretation that deserves separation from the main argument.

For a meaningful repeated-cost illustration:

```mdx
import CostComparison from '@report-components/CostComparison.astro';

<CostComparison baselineMs={0.1} candidateMs={0.16} repetitions={1000} />
```

This component shows the per-call values in a static table and lets the reader
change the call count. The total is arithmetic, not a throughput prediction.
Label illustrative values or cite measured ones in adjacent prose.

For another interaction, create `widgets/Name.astro` inside this report and
import it from `./widgets/Name.astro`. Use scoped styles and small
`<script is:inline>` scripts. Keep each widget's state local to its element,
support keyboard input, and keep a useful static/printed explanation. Inline
SVG and data-URI images are suitable for a single-file artifact. The build
rejects runtime asset dependencies, including bundled JS imports: use inline
scripts/styles or explicitly revise the delivery requirement before adopting
a multi-file island/framework. It does not add React or a charting library.

## Output and verification

The deliverable is a single static `index.html`, ready to copy or send on its
own. Embed styles, optional widget scripts and their data in that file. Readers
need neither Astro nor Node, a server, adjacent assets or the research notes.
Keep essential evidence in the report itself; source links are supplementary.

Build uses a temporary output directory and writes only `index.html` beside
the source after validation. A rendering or validation failure leaves the last
generated report intact. Building MDX executes its imports: use only report
source and components you have reviewed, not arbitrary downloaded MDX.

### Recipient-style browser check

When browser execution is permitted, run:

```sh
node "$SKILL_DIR/scripts/check-browser.mjs" /absolute/path/to/index.html \
  --screenshots /absolute/path/to/new-review-directory
```

This optional check uses the skill's Playwright development dependency and an
installed Chrome browser. Set `REPORT_BROWSER_EXECUTABLE` to use another installed
Chromium executable. It does not download a browser or use your normal profile.
The screenshot directory must be new; omit it for checks without screenshots.

The helper copies only the HTML into a temporary directory, blocks requests for
external resources, and checks desktop/mobile overflow, section links, keyboard
access to the skip link, runtime errors, print navigation and the built-in cost
widget. It also opens the report with JavaScript disabled. Supplementary source
links intentionally remain outside this test because the recipient may not have
the checkout. Exercise custom widgets separately and inspect the screenshots:
the automated checks do not judge readability or verify every interaction.

For generator development, `ZDEV_REPORT_BROWSER=1 npm test` in the skill
directory includes the fixture's browser check and checks that a widget's
attempt to fetch external data fails the offline check. Ordinary `npm test`
requires no browser and explicitly reports that these checks were not run.

If browser permissions or an executable are unavailable, report that limitation.
Do not work around access restrictions. A permitted HTTP-only preview may help
with layout, but does not verify the required offline `file://` behavior.

When changing the renderer, compare before/after output using the same report
source and data. The `example` fixture is a small starting point; include an
unchanged real report if the layout issue needs more content. Record concrete
improvements or regressions in the working notes rather than building a separate
comparison-tracking system.

Implementation references: [Astro MDX](https://docs.astro.build/en/guides/integrations-guide/mdx/),
[programmatic build](https://docs.astro.build/en/reference/programmatic-reference/),
and [inline stylesheet configuration](https://docs.astro.build/en/reference/configuration-reference/#buildinlinestylesheets).

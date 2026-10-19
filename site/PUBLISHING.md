# Publish zdev at zayenz.se/zdev/

Build the selected source revision with Node 22.12 or newer:

```sh
cd site
npm ci
npm run check
SITE_URL=https://zayenz.se SITE_BASE=/zdev npm run build
npm run preview
```

Review the homepage, a deep guide, search and a result, mobile layout,
light/dark themes, keyboard navigation and the link back to all projects.
`dist/` contains pages, assets, the Pagefind index and sitemaps directly,
without an extra `zdev/` wrapper.

## First publication

In zayenz.se, register `zdev` in `src/data/projects.ts` with
`importedSite: true`. For this publication, keep its existing project-directory
entry pointing to GitHub. Do not set `site` or add documentation guide links
until the site is ready to be linked from `/projects/`.

The import flag registers the destination, inherited host analytics, and sitemap
discovery independently of the optional directory link. Add the flag and
imported output together. Do not add a documentation link to the blog post as
part of this publication.

## Public listing

Before listing the site, reconcile the workflow reference and upgrade notes,
then build and import the selected documentation revision below. Check the
first-task guide's release-version notice against the release actually available;
update it when 2.0 is published. A release tag does not update the hosted docs.

Once the final import is reviewed, set the zdev project entry's site link to
`/zdev/` and add its getting-started and workflow guide links in zayenz.se. Keep
its GitHub source link. The README now links to the documentation; update the
introductory blog post's documentation link as part of the host publication.
Run the host's required checks and publish through its normal route.

## Import or update

Use Node 24 in the zayenz.se repository. Replace the source path with the
actual absolute path to this build:

```sh
npm run import:project -- --project zdev --source /absolute/path/to/zdev/site/dist --dry-run
npm run import:project -- --project zdev --source /absolute/path/to/zdev/site/dist
```

For an existing import, add `--replace` to both commands. It removes obsolete
assets and search fragments along with the old output. Review any existing
uncommitted import changes before replacement.

Run zayenz.se's required `npm run release:check`, review the combined site and
record the zdev source commit in the import commit message. Publish through
its normal Git-based Netlify route: a push to production `main`. The hosted
build serves the committed static output; it does not build zdev or run Rust.
Check the actual HTTPS pages and search after deployment.

Keep guide edits in this repository. Documentation publication is explicit:
a zdev push or release does not update zayenz.se. Rebuild and import when
publishing documentation changes. Revert the relevant zayenz.se import commit
and publish through the same route to roll back.

# Zdev website

The homepage and documentation are built for `https://zayenz.se/zdev/`.
Astro and Starlight provide navigation, search and light/dark themes.

`docs/`, `CHANGELOG.md` and `CONTRIBUTING.md` remain authoritative. The content
preparation script selects published pages explicitly, turns guide links into
website links and sends repository-file links to GitHub. Historical release
qualification notes and design probes remain in the repository.
`src/landing.mdx` owns the homepage prose.
The wordmark under `assets/` uses a green `z` with neutral lettering for `dev`;
the light variant keeps the word readable on a dark background.

Use Node 22.12 or newer:

```sh
cd site
npm ci
npm run dev
npm run check
npm run build
npm run preview
```

The default origin is `https://zayenz.se` and base path is `/zdev`.
`SITE_URL` and `SITE_BASE` override them. Generated content, `.astro/`,
dependencies and `dist/` are ignored. The package lock fixes website
dependencies independently of the Rust release.

The site separates a worked tutorial, chat-command guides, and detailed reference.
Agent-command pages are authored under `docs/commands/` and terminal-command
pages under `docs/cli/`; the other public pages live
directly under `docs/`.

To publish another guide, add its name to `scripts/prepare-content.mjs` and its
sidebar entry to `astro.config.mjs`. Edit the Markdown source, then rebuild;
do not edit generated content or imported HTML.

See [PUBLISHING.md](PUBLISHING.md) for importing and publishing through zayenz.se.
Website checks do not run Rust tests or release qualification.

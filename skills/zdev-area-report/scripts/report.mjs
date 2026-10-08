#!/usr/bin/env node
import { access, cp, mkdir, mkdtemp, readFile, realpath, rm, symlink, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseArgs } from 'node:util';

const skillDirectory = fileURLToPath(new URL('../', import.meta.url));
const fixtures = ['blank', 'compiler', 'portfolio', 'example'];
process.env.ASTRO_TELEMETRY_DISABLED = '1';

/** Create editable report source without replacing an existing report. */
export async function initializeReport({ project, area, date, name, fixture = 'blank' }) {
  for (const [label, value] of Object.entries({ area, name })) {
    if (!value || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(value))
      throw new Error(`${label} must be a lowercase hyphenated name`);
  }
  if (!/^\d{4}-\d{2}-\d{2}$/.test(date ?? '') ||
      !Number.isFinite(Date.parse(date)) || new Date(date).toISOString().slice(0, 10) !== date)
    throw new Error('date must be an existing calendar date in YYYY-MM-DD form');
  if (!fixtures.includes(fixture)) throw new Error(`fixture must be one of: ${fixtures.join(', ')}`);
  if (!project) throw new Error('project is required');
  const root = await realpath(project);
  await access(path.join(root, '.zdev', area, 'area.toml'));
  const parent = path.join(root, '.zdev', 'reports', area);
  await mkdir(parent, { recursive: true });
  const directory = path.join(parent, `${date}-${name}`);
  const template = await readFile(path.join(skillDirectory, 'fixtures', `${fixture}.mdx`), 'utf8');
  await mkdir(directory);
  await writeFile(path.join(directory, 'report.mdx'), template
    .replaceAll('{{AREA}}', area).replaceAll('{{DATE}}', date), { flag: 'wx' });
  return directory;
}

/** Check the rendered artifact, not a particular heading or prose template. */
export async function validateHtml(html, directory) {
  const { parse } = await import('parse5');
  const ids = new Set();
  const links = [];
  const errors = [];
  function inspect(node) {
    const attributes = Object.fromEntries((node.attrs ?? []).map(({ name, value }) => [name, value]));
    if (attributes.id) {
      if (ids.has(attributes.id)) errors.push(`duplicate id: ${attributes.id}`);
      ids.add(attributes.id);
    }
    if (node.tagName === 'a' && attributes.href) links.push(attributes.href);
    if (node.tagName === 'script' && (attributes.src || attributes.type === 'module'))
      errors.push('Use inline classic scripts so the report works through file://');
    if (node.tagName === 'link' && attributes.rel?.split(/\s+/).includes('stylesheet'))
      errors.push('Stylesheets must be embedded');
    if (['img', 'source', 'video', 'audio', 'iframe', 'object', 'embed'].includes(node.tagName)) {
      for (const key of ['src', 'srcset', 'poster', 'data']) {
        if (attributes[key] && !attributes[key].startsWith('data:'))
          errors.push(`${node.tagName} ${key} must be embedded as a data URI`);
      }
    }
    if (node.tagName === 'style') {
      const css = (node.childNodes ?? []).map(child => child.value ?? '').join('');
      const urls = [...css.matchAll(/url\(\s*['"]?([^)'"\s]+)/gi)].map(match => match[1]);
      if (/@import\b/i.test(css) || urls.some(url => !/^(data:|#)/i.test(url)))
        errors.push('CSS must not load external assets');
    }
    for (const child of node.childNodes ?? []) inspect(child);
    if (node.content) inspect(node.content);
  }
  inspect(parse(html));
  for (const link of links) {
    if (link.startsWith('#')) {
      if (!ids.has(decodeURIComponent(link.slice(1)))) errors.push(`missing anchor: ${link}`);
    } else if (!/^[a-z][a-z0-9+.-]*:|^\/\//i.test(link)) {
      const target = decodeURIComponent(link.split(/[?#]/)[0]);
      if (!target) continue;
      try { await access(path.resolve(directory, target)); }
      catch { errors.push(`missing local link: ${link}`); }
    }
  }
  if (errors.length) throw new Error(errors.join('\n'));
}

/** Render one report in scratch space; publish HTML only after its checks pass. */
export async function buildReport(reportDirectory) {
  const directory = await realpath(reportDirectory);
  await access(path.join(directory, 'report.mdx'));
  const [{ build }, { default: mdx }] = await Promise.all([import('astro'), import('@astrojs/mdx')]);
  const scratch = await realpath(await mkdtemp(path.join(tmpdir(), 'zdev-area-report-')));
  const originalDirectory = process.cwd();
  try {
    await cp(path.join(skillDirectory, 'assets', 'renderer', 'src'), path.join(scratch, 'src'), { recursive: true });
    await mkdir(path.join(scratch, 'report'));
    await cp(path.join(directory, 'report.mdx'), path.join(scratch, 'report', 'report.mdx'));
    try {
      await cp(path.join(directory, 'widgets'), path.join(scratch, 'report', 'widgets'), { recursive: true });
    } catch (error) {
      if (error.code !== 'ENOENT') throw error;
    }
    await symlink(path.join(skillDirectory, 'node_modules'), path.join(scratch, 'node_modules'), 'dir');
    // Astro keeps prerender output under the process working directory.
    process.chdir(scratch);
    await build({
      root: scratch,
      configFile: false,
      outDir: path.join(scratch, 'output'),
      cacheDir: path.join(scratch, 'cache'),
      output: 'static',
      publicDir: path.join(scratch, 'public'),
      integrations: [mdx()],
      build: { inlineStylesheets: 'always' },
      markdown: { syntaxHighlight: false },
      vite: { cacheDir: path.join(scratch, 'vite'), resolve: { alias: {
        '@report': path.join(scratch, 'report', 'report.mdx'),
        '@report-components': path.join(scratch, 'src', 'components'),
      } } },
    });
    const html = await readFile(path.join(scratch, 'output', 'index.html'), 'utf8');
    await validateHtml(html, directory);
    const output = path.join(directory, 'index.html');
    await writeFile(output, html);
    return output;
  } finally {
    process.chdir(originalDirectory);
    await rm(scratch, { recursive: true, force: true });
  }
}

async function main() {
  const { values, positionals } = parseArgs({
    allowPositionals: true,
    options: Object.fromEntries(['project', 'area', 'date', 'name', 'fixture'].map(key => [key, { type: 'string' }])),
  });
  const [command, directory] = positionals;
  if (command === 'init' && positionals.length === 1) console.log(await initializeReport(values));
  else if (command === 'build' && positionals.length === 2) console.log(await buildReport(directory));
  else throw new Error('Usage: report.mjs init --project PATH --area TAG --date YYYY-MM-DD --name NAME [--fixture blank|compiler|portfolio|example]\n       report.mjs build REPORT_DIRECTORY');
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  main().catch(error => { console.error(error.message); process.exitCode = 1; });
}

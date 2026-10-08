import assert from 'node:assert/strict';
import { mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { buildReport, initializeReport, validateHtml } from '../scripts/report.mjs';

test('fixtures generate standalone reports, preserve source, and reject broken output', async (t) => {
  const project = await mkdtemp(path.join(tmpdir(), 'zdev report test '));
  try {
    const area = path.join(project, '.zdev', 'demo');
    await mkdir(area, { recursive: true });
    await writeFile(path.join(area, 'area.toml'), 'title = "Report fixture"\n');
    await writeFile(path.join(area, 'brief.md'), '# Demonstration area\n');
    const options = { project, area: 'demo', date: '2026-10-02', name: 'sample', fixture: 'example' };
    const directory = await initializeReport(options);
    await assert.rejects(initializeReport(options), { code: 'EEXIST' });
    await assert.rejects(initializeReport({ ...options, name: '../escape' }), /hyphenated/);
    await assert.rejects(initializeReport({ ...options, date: '2026-02-30' }), /calendar date/);
    const source = await readFile(path.join(directory, 'report.mdx'), 'utf8');
    const output = await buildReport(directory);
    const html = await readFile(output, 'utf8');
    assert.match(html, /All measurements below are invented/);
    assert.match(html, /class="table-wrap"/);
    assert.match(html, /0\.060 seconds difference/);
    assert.match(html, /addEventListener\('input'/);
    assert.equal(await readFile(path.join(directory, 'report.mdx'), 'utf8'), source);
    await validateHtml(html, directory);
    if (process.env.ZDEV_REPORT_BROWSER === '1') {
      const { checkBrowser } = await import('../scripts/check-browser.mjs');
      await checkBrowser(output);
      const dynamicRequest = path.join(directory, 'dynamic-request.html');
      await writeFile(dynamicRequest, html.replace('</body>',
        '<script>fetch("https://example.invalid/report-data").catch(() => {});</script></body>'));
      await assert.rejects(checkBrowser(dynamicRequest), /Offline report/);
    } else {
      t.diagnostic('Browser checks not run; set ZDEV_REPORT_BROWSER=1 to include them.');
    }
    await assert.rejects(validateHtml('<a href="#missing">Missing</a>', directory), /missing anchor/);
    await assert.rejects(validateHtml('<script src="widget.js"></script>', directory), /inline classic/);
    await writeFile(path.join(directory, 'report.mdx'), `${source}\n[Broken](missing-source.md)\n`);
    await assert.rejects(buildReport(directory), /missing local link/);
    assert.equal(await readFile(output, 'utf8'), html);

    for (const fixture of ['blank', 'compiler', 'portfolio']) {
      const draft = await initializeReport({ ...options, name: fixture, fixture });
      const draftHtml = await readFile(await buildReport(draft), 'utf8');
      assert.match(draftHtml, /Writing and evidence review are not complete/);
    }

    const custom = await initializeReport({ ...options, name: 'custom', fixture: 'blank' });
    await mkdir(path.join(custom, 'widgets'));
    await writeFile(path.join(custom, 'widgets', 'Note.astro'), '<aside>Local component rendered</aside>\n');
    const customSource = await readFile(path.join(custom, 'report.mdx'), 'utf8');
    await writeFile(path.join(custom, 'report.mdx'), `${customSource}\nimport Note from './widgets/Note.astro';\n\n<Note />\n`);
    assert.match(await readFile(await buildReport(custom), 'utf8'), /Local component rendered/);
  } finally {
    await rm(project, { recursive: true, force: true });
  }
});

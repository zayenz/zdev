#!/usr/bin/env node
import assert from 'node:assert/strict';
import { copyFile, mkdir, mkdtemp, realpath, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { parseArgs } from 'node:util';

/** Check the reading copy without its source tree or a normal browser profile. */
export async function checkBrowser(report, screenshots) {
  const source = await realpath(report);
  const { chromium } = await import('playwright');
  const temporary = await mkdtemp(path.join(tmpdir(), 'zdev-report-reader-'));
  let browser;
  try {
    if (screenshots) await mkdir(screenshots);
    const copy = path.join(temporary, 'index.html');
    await copyFile(source, copy);
    const url = pathToFileURL(copy).href;
    browser = await chromium.launch(process.env.REPORT_BROWSER_EXECUTABLE
      ? { executablePath: process.env.REPORT_BROWSER_EXECUTABLE }
      : { channel: 'chrome' });
    const failures = [];
    for (const javaScriptEnabled of [true, false]) {
      const context = await browser.newContext({ javaScriptEnabled, reducedMotion: 'reduce' });
      const embedded = resource => resource.split('#')[0] === url || /^(data|blob):/.test(resource);
      context.on('request', request => {
        if (!embedded(request.url())) failures.push(`Resource outside the shared HTML: ${request.url()}`);
      });
      await context.route('**/*', route => {
        const resource = route.request().url();
        return embedded(resource) ? route.continue() : route.abort();
      });
      const page = await context.newPage();
      page.setDefaultTimeout(5000);
      page.on('pageerror', error => failures.push(error.message));
      page.on('requestfailed', request => failures.push(`Load failed: ${request.url()}`));
      for (const width of [1440, 390]) {
        await page.setViewportSize({ width, height: 900 });
        await page.goto(url, { waitUntil: 'load' });
        assert.equal(await page.locator('main h1').count(), 1, 'Missing report title');
        assert.equal(await page.locator('article').isVisible(), true, 'Report body is hidden');
        assert.ok(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth),
          `Page overflows at ${width}px (JavaScript ${javaScriptEnabled})`);
        if (screenshots) await page.screenshot({
          path: path.join(screenshots, `${width}-${javaScriptEnabled ? 'interactive' : 'no-js'}.png`),
          fullPage: true,
        });
        await page.keyboard.press('Tab');
        assert.equal(await page.locator('.skip').evaluate(node => node === document.activeElement),
          true, 'Skip link is not first in keyboard order');
        for (const link of await page.locator('nav a[href^="#"]').all()) {
          const fragment = await link.getAttribute('href');
          await link.click();
          assert.equal(new URL(page.url()).hash, fragment, 'Section link did not navigate');
          assert.ok(await page.evaluate(hash => {
            const target = document.getElementById(decodeURIComponent(hash.slice(1)));
            const box = target?.getBoundingClientRect();
            return box && box.height > 0 && box.bottom > 0 && box.top < innerHeight;
          }, fragment), `Section is not visible: ${fragment}`);
        }
        if (javaScriptEnabled) {
          for (const widget of await page.locator('.cost-comparison').all()) {
            await widget.locator('input').fill('2000');
            const difference = Number(await widget.getAttribute('data-difference'));
            assert.equal(await widget.locator('output').innerText(),
              `${(difference * 2).toFixed(3)} seconds difference`, 'Cost widget did not update');
          }
        }
        await page.emulateMedia({ media: 'print' });
        assert.equal(await page.locator('.sidebar').isVisible(), false, 'Print includes navigation');
        await page.emulateMedia({ media: 'screen' });
      }
      await context.close();
    }
    assert.deepEqual(failures, [], 'Offline report raised errors or requested outside resources');
  } finally {
    try { await browser?.close(); }
    finally { await rm(temporary, { recursive: true, force: true }); }
  }
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const { values, positionals } = parseArgs({
    allowPositionals: true, options: { screenshots: { type: 'string' } },
  });
  if (positionals.length !== 1) {
    console.error('Usage: check-browser.mjs INDEX_HTML [--screenshots NEW_DIRECTORY]');
    process.exitCode = 1;
  } else {
    checkBrowser(positionals[0], values.screenshots).then(
      () => console.log('Recipient-style browser checks passed. Visual and custom-widget review remain manual.'),
      error => { console.error(error.message); process.exitCode = 1; },
    );
  }
}

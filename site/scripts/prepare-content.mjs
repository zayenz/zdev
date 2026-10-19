import { mkdir, readFile, writeFile, copyFile, rm } from 'node:fs/promises';
import { resolve, relative, dirname, posix } from 'node:path';
import { fileURLToPath } from 'node:url';

const site = fileURLToPath(new URL('../', import.meta.url));
const root = resolve(site, '..');
const guides = [
  'areas-and-briefs', 'working-with-tasks', 'discussion-and-research',
  'feature-walkthrough', 'research-walkthrough', 'longer-runs',
  'coding-guidelines', 'review-and-verification',
  'getting-started', 'user-guide', 'workflow', 'task-format', 'area-lifecycle',
  'area-loop', 'trunk-area-mode', 'derived-work-handoffs', 'config-command',
  'worker-profiles', 'task-complexity-routing', 'harness-orchestration',
  'harness-goals', 't3-code', 'workflow-round-trips', 'area-reports',
  'adapted-methods', 'releasing', 'upgrading-to-2.0', 'research', 'reference',
];
const commands = [
  'notes', 'explore', 'discuss', 'improve', 'audit', 'investigate', 'tasks',
  'plan', 'implement', 'review', 'verify', 'parallel', 'loop', 'recover',
  'research', 'configure', 'setup',
];
const cli = [
  'instructions', 'init', 'cleanup', 'config', 'area', 'research', 'slice',
  'tasks', 'task', 'next', 'goal', 'work-context', 'status', 'check', 'skill',
  'commit', 'change-id', 'change',
];
const pages = new Map([
  ['docs/cli/index.md', 'cli'],
  ...cli.map((name) => [`docs/cli/${name}.md`, `cli/${name}`]),
  ['docs/commands/index.md', 'commands'],
  ...commands.map((name) => [`docs/commands/${name}.md`, `commands/${name}`]),
  ...guides.map((name) => [`docs/${name}.md`, `guides/${name === 'upgrading-to-2.0' ? 'upgrading-to-2' : name}`]),
  ['CHANGELOG.md', 'changelog'],
  ['CONTRIBUTING.md', 'contributing'],
]);
const output = resolve(site, 'src/content/docs');
await rm(output, { recursive: true, force: true });
await mkdir(output, { recursive: true });

for (const [file, slug] of pages) {
  const source = resolve(root, file);
  const markdown = await readFile(source, 'utf8');
  const title = markdown.match(/^# (.+)\r?\n/);
  if (!title) throw new Error(`Missing page title in ${source}`);
  const body = markdown.slice(title[0].length).replace(/\]\(([^\s)]+)\)/g, (link, target) => {
    if (/^(?:[a-z]+:|#|\/)/i.test(target)) return link;
    const [path, anchor = ''] = target.split('#');
    const resolved = relative(root, resolve(dirname(source), path));
    const destination = pages.get(resolved);
    const suffix = anchor ? `#${anchor}` : '';
    if (destination) {
      return `](${posix.relative(slug, destination) || '.'}/${suffix})`;
    }
    return `](https://github.com/zayenz/zdev/blob/main/${resolved}${suffix})`;
  });
  const target = resolve(output, `${slug}.md`);
  await mkdir(dirname(target), { recursive: true });
  await writeFile(target, `---\ntitle: ${JSON.stringify(title[1])}\neditUrl: https://github.com/zayenz/zdev/edit/main/${file}\n---\n${body}`);
}

await copyFile(resolve(site, 'src/landing.mdx'), resolve(output, 'index.mdx'));

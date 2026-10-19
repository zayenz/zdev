import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

const base = process.env.SITE_BASE || '/zdev';
const site = process.env.SITE_URL || 'https://zayenz.se';
if (!/^\/(?:[a-zA-Z0-9_-]+\/)*[a-zA-Z0-9_-]*$/.test(base)) {
  throw new Error('SITE_BASE must be / or a path such as /zdev');
}
const url = new URL(site);
if (!['https:', 'http:'].includes(url.protocol) || url.pathname !== '/' || url.search || url.hash) {
  throw new Error('SITE_URL must be an HTTP(S) origin; set its path with SITE_BASE');
}

export default defineConfig({
  site, base, trailingSlash: 'always',
  integrations: [starlight({
    title: 'zdev',
    logo: { light: '../assets/zdev-logo.svg', dark: '../assets/zdev-logo-light.svg', replacesTitle: true, alt: 'zdev' },
    description: 'Organize development work in repository briefs and tasks, then use coding agents to carry it out.',
    favicon: '/favicon.svg',
    customCss: ['./src/styles/custom.css'],
    components: { Footer: './src/components/Footer.astro' },
    social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/zayenz/zdev' }],
    sidebar: [
      { label: 'Start here', items: [
        { label: 'What zdev does', slug: '' },
        { label: 'Your first task', slug: 'guides/getting-started' },
        { label: 'Upgrade to 2.0', slug: 'guides/upgrading-to-2' },
      ] },
      { label: 'Working with zdev', items: [
        'guides/areas-and-briefs', 'guides/working-with-tasks',
        'guides/discussion-and-research', 'guides/feature-walkthrough',
        'guides/research-walkthrough',
        'guides/longer-runs',
      ] },
      { label: 'Agent commands', items: [
        { label: 'Choose an agent command', slug: 'commands' },
        'commands/setup', 'commands/notes', 'commands/explore', 'commands/discuss',
        'commands/improve', 'commands/audit', 'commands/investigate', 'commands/tasks',
        'commands/plan', 'commands/implement', 'commands/review', 'commands/verify',
        'commands/parallel', 'commands/loop', 'commands/recover', 'commands/configure',
        'commands/research',
      ], collapsed: true },
      { label: 'CLI reference', items: [
        { label: 'CLI overview', slug: 'cli' },
        ...['init', 'skill', 'instructions', 'area', 'slice', 'tasks', 'task',
          'next', 'status', 'check', 'config', 'goal', 'work-context', 'research',
          'commit', 'change', 'change-id', 'cleanup'].map((name) => ({
            label: `zdev ${name}`, slug: `cli/${name}`,
          })),
      ], collapsed: true },
      { label: 'Detailed reference', items: [
        { label: 'Reference overview', slug: 'guides/reference' },
        { label: 'Setup and task operations', slug: 'guides/user-guide' },
        'guides/workflow', 'guides/task-format', 'guides/area-lifecycle',
        'guides/area-loop', 'guides/trunk-area-mode', 'guides/derived-work-handoffs',
        'guides/config-command', 'guides/worker-profiles', 'guides/task-complexity-routing',
        'guides/harness-orchestration', 'guides/harness-goals', 'guides/t3-code',
        'guides/research', 'guides/area-reports',
      ], collapsed: true },
      { label: 'Project', items: ['changelog', 'contributing', 'guides/adapted-methods', 'guides/releasing'], collapsed: true },
      { label: 'All projects', link: 'https://zayenz.se/projects/' },
    ],
  })],
});

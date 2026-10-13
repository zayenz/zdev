# React Router framework mode

Apply these rules with [coding guidance](../coding.md), [React](react.md), and
[web guidance](../web.md). Use APIs supported by the installed version and route
rendering mode.

## Routes and data

Use route loaders for reads and actions for mutations instead of effect-driven
requests or duplicate component caches. Keep secrets, authorization, and
server-only work on the server; client loaders and actions are client code.
Treat action inputs as untrusted even when a form supplied them. Use generated
route types instead of duplicating inferred loader and action result contracts.

Use links for navigation, GET forms for shareable queries, and mutation forms
when submission should navigate. Use fetchers when an interaction should stay
on the current page. Read shareable state from URL parameters instead of
synchronizing another authoritative copy in component state.

Use navigation and fetcher state for pending UI. Add optimistic behavior when
the result is predictable from the submission and failure recovery is clear.
Keep submitted values and field errors available after failed validation; use
route error boundaries for failures that prevent the route from rendering.

Preserve default loader revalidation unless a specific correctness argument
justifies changing it. Distinguish application data refresh from HTTP conditional
revalidation: neither alone coordinates every browser, server, and CDN cache.
Set response headers at the framework boundary that actually produces the
response and account for parent/child route header handling.

## Validation

Run route type generation before the project type check when required, plus
existing formatting and linting. Check affected routes through direct entry and
client navigation, including submissions, pending states, field failures, error
boundaries, and subsequent data refresh. Apply the web checks when URLs or HTTP
behavior change. Use current framework APIs rather than copying older examples.

References: [route modules](https://reactrouter.com/start/framework/route-module),
[pending UI](https://reactrouter.com/start/framework/pending-ui),
[state management](https://reactrouter.com/explanation/state-management).

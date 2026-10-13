# Web

Apply these rules with [coding guidance](coding.md) for browser behavior, URLs,
forms, and HTTP responses. Framework supplements supply their API choices.

## URLs and native behavior

Use the URL as the authority for shareable resource identity, committed searches,
filters, sorting, pagination, and meaningful selected views. Direct entry, reload,
and copied URLs must restore that state. Keep unfinished input, transient UI, and
private data elsewhere. Shareable state must not depend on navigation history
state. Preserve unrelated parameters unless the interaction deliberately resets
them.

Push history entries for meaningful navigation; replace for normalization or
intermediate changes that should not fill the Back history. Use links with real
destinations, buttons for actions, and forms for submission. Use GET for
non-sensitive queries and an appropriate unsafe HTTP method for writes.

Start with semantic HTML: headings, landmarks, labels, and native controls.
Preserve keyboard operation, accessible names, visible focus, and input-associated
validation feedback. Custom controls need a concrete benefit that earns their
additional interaction and validation work.

## HTTP caching

Choose cache policy from the response's audience and acceptable age. `public`
permits shared caching; `private` confines storage to private caches; `no-cache`
allows storage but requires validation before reuse; `no-store` prevents storage.
Cookies alone do not isolate personalized responses. Use `Vary` for headers that
select representations and validators such as `ETag` when conditional requests
help. Prefer long freshness for fingerprinted assets and deliberate freshness
for mutable content.

Use `stale-while-revalidate` when temporary staleness is acceptable, with a bounded
window chosen for the data. Check actual CDN behavior and invalidate application
caches across identity changes. Adding caching libraries or service workers needs
a separate benefit.

## Validation

For affected behavior, check direct entry, reload, copied URLs, Back/Forward,
keyboard use, and mutation results. When caching changes, check response headers,
relevant conditional responses, acceptable staleness, and isolation between users.
These are evidence choices under common completion guidance, not a new test suite.

References: [HTTP caching](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Caching),
[stale-while-revalidate](https://www.rfc-editor.org/rfc/rfc5861.html),
[HTML accessibility](https://developer.mozilla.org/en-US/docs/Learn_web_development/Core/Accessibility/HTML).

export const meta = {
  name: 'zdev-audit',
  description: 'Audit a bounded codebase and return independently checked proposals',
}

const input = args ?? {}
const boundary = input.boundary ?? 'the current repository'
const requestedLenses = Array.isArray(input.lenses)
  ? input.lenses.map(String).map(lens => lens.trim()).filter(Boolean)
  : []
const reviewScopes = requestedLenses.length > 0
  ? requestedLenses.map(lens => `${lens} lens`)
  : ['broad review']
const reviews = await pipeline(reviewScopes, scope =>
  agent(
    `Perform one bounded read-only ${scope} of ${boundary}. Inspect repository evidence and return only material findings with locations, impact, and confidence. Do not edit files or .zd, create tasks, commit, open a pull request, or create durable run state.`,
    { label: `${scope} audit` },
  ),
)

if (reviews.filter(Boolean).length === 0) {
  return 'BLOCKER: the audit reviewer returned no evidence to vet.'
}

return agent(
  `Act as a fresh read-only evidence vetter for this audit of ${boundary}. Open every cited location, reject weak, speculative, or duplicate claims, and rank only evidence-backed candidate work for human selection. State the inspected boundary and omissions. Do not edit files or .zd, create zdev tasks, commit, open a pull request, or modify project state.\n\nReviewer output:\n${reviews.filter(Boolean).join('\n\n')}`,
  { label: 'audit evidence vetter' },
)

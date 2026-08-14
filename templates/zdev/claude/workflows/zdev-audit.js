export const meta = {
  name: 'zdev-audit',
  description: 'Review a codebase and return checked findings',
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
    `Review ${boundary} from the ${scope}. Keep the repository unchanged. Return concrete findings with locations, impact, and confidence.`,
    { label: `${scope} audit` },
  ),
)

if (reviews.filter(Boolean).length === 0) {
  return 'BLOCKER: the audit reviewer returned no evidence to vet.'
}

return agent(
  `Check these findings for ${boundary}. Keep the repository unchanged. Open every cited location, remove weak, speculative, and duplicate claims, then rank the remaining findings by impact. State what was inspected and omitted. Return locations, impact, confidence, and a recommended next action.\n\nReviewer output:\n${reviews.filter(Boolean).join('\n\n')}`,
  { label: 'audit evidence vetter' },
)

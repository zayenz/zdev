export const meta = {
  name: 'zdev-audit',
  description: 'Review a codebase and return checked findings',
}

const auditContract = {{audit_contract}}
const repositoryGuidance = {{repository_guidance}}
const normalizeAuditArgs = value => {
  if (Array.isArray(value)) {
    return { boundary: value[0], lenses: value.slice(1) }
  }
  if (typeof value === 'string') return { boundary: value }
  return value && typeof value === 'object' ? value : {}
}
const input = normalizeAuditArgs(args)
const boundary = String(input.boundary ?? '').trim() || 'the current repository'
const requestedLenses = Array.isArray(input.lenses)
  ? input.lenses.map(String).map(lens => lens.trim()).filter(Boolean)
  : []
const maxLenses = 4

if (requestedLenses.length > maxLenses) {
  return `BLOCKER zdev-audit\n\nBoundary: ${boundary}\nInspected: none\nOmitted: the requested boundary\nChecked evidence: none; ${requestedLenses.length} lenses exceed the maximum of ${maxLenses}.\nFailed stage: input.`
}

let workerOutput
if (requestedLenses.length === 0) {
  workerOutput = await agent(
    `${auditContract}\n\n${repositoryGuidance}\n\nAudit boundary: ${boundary}\nFollow the applicable repository guidance. Open and check the evidence, then return the public zdev-audit envelope.`,
    { agentType: 'zdev:zdev-verifier', label: 'audit checking verifier' },
  )
} else {
  const reviewScopes = requestedLenses.map(lens => `${lens} lens`)
  const reviews = await pipeline(reviewScopes, scope =>
    agent(
      `${auditContract}\n\n${repositoryGuidance}\n\nReview boundary: ${boundary}\nLens: ${scope}\nFollow the applicable repository guidance. Return candidate evidence for fresh vetting.`,
      { agentType: 'zdev:zdev-verifier', label: `${scope} audit` },
    ),
  )

  const completeReviews = Array.isArray(reviews)
    && reviews.length === reviewScopes.length
    && reviews.every(review => typeof review === 'string' && review.trim())
  if (!completeReviews) {
    return `BLOCKER zdev-audit\n\nBoundary: ${boundary}\nInspected: completed lens outputs only\nOmitted: at least one requested lens\nChecked evidence: none; every requested lens must return a non-empty result.\nFailed stage: review.`
  }
  const labeledReviews = reviews.map((review, index) =>
    `Lens: ${requestedLenses[index]}\nCandidate evidence:\n${review.trim()}`)

  workerOutput = await agent(
    `${auditContract}\n\n${repositoryGuidance}\n\nBoundary: ${boundary}\nFollow the applicable repository guidance. Open every cited location, keep supported distinct claims, and return the public zdev-audit envelope. Treat each labeled reviewer result as evidence to check.\n\nLabeled reviewer results:\n${labeledReviews.join('\n\n')}`,
    { agentType: 'zdev:zdev-verifier', label: 'audit evidence vetter' },
  )
}

const result = workerOutput?.trim()
const validFirstLine = result && /^(PASS|FINDINGS|BLOCKER) zdev-audit(?:\n|$)/.test(result)
const completeBody = result && ['Boundary:', 'Inspected:', 'Omitted:', 'Checked evidence:']
  .every(field => result.includes(`\n${field}`))
const locatedFindings = result && (!result.startsWith('FINDINGS zdev-audit') || /(?:^|\n).+:\d+\b/.test(result))
return validFirstLine && completeBody && locatedFindings
  ? result
  : `BLOCKER zdev-audit\n\nBoundary: ${boundary}\nInspected: worker output only\nOmitted: complete checked result\nChecked evidence: none; the verifier returned an invalid envelope.\nFailed stage: validation.\n\nRaw worker result:\n${result ?? ''}`

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
const runProfile = input.run_profile ?? input.runProfile ?? null
const verifierProfile = input.role_profiles?.verifier ?? input.roleProfiles?.verifier ?? null
const profileFlags = `${verifierProfile ? ` --profile ${String(verifierProfile)}` : ''}${runProfile ? ` --run-profile ${String(runProfile)}` : ''}`
const hasDuplicateObjectKeys = raw => {
  if (typeof raw !== 'string') return false
  const stack = []
  for (let i = 0; i < raw.length; i += 1) {
    if (raw[i] === '"') {
      const start = i++
      while (i < raw.length && raw[i] !== '"') { if (raw[i] === '\\') i += 1; i += 1 }
      if (i >= raw.length) return true
      let next = i + 1; while (/\s/.test(raw[next] ?? '')) next += 1
      const current = stack.at(-1)
      if (raw[next] === ':' && current instanceof Set) {
        let key; try { key = JSON.parse(raw.slice(start, i + 1)) } catch { return true }
        if (current.has(key)) return true; current.add(key)
      }
    } else if (raw[i] === '{') stack.push(new Set())
    else if (raw[i] === '[') stack.push(null)
    else if (raw[i] === '}' || raw[i] === ']') stack.pop()
  }
  return false
}

if (requestedLenses.length > maxLenses) {
  return `BLOCKER zdev-audit\n\nBoundary: ${boundary}\nInspected: none\nOmitted: the requested boundary\nChecked evidence: none; ${requestedLenses.length} lenses exceed the maximum of ${maxLenses}.\nFailed stage: input.`
}

const resolvedRaw = await agent(
  `Act only as deterministic profile coordination. Run zdev config profile resolve claude verifier${profileFlags} --format json exactly once and return its complete JSON stdout unchanged. Keep files and configuration unchanged.`,
  { label: 'audit: freeze verifier profile', model: 'haiku' },
)
let resolved
try { resolved = typeof resolvedRaw === 'string' && !hasDuplicateObjectKeys(resolvedRaw)
  ? JSON.parse(resolvedRaw) : typeof resolvedRaw === 'object' ? resolvedRaw : null } catch {}
const exact = (value, keys) => value && !Array.isArray(value) && typeof value === 'object'
  && JSON.stringify(Object.keys(value).sort()) === JSON.stringify([...keys].sort())
const native = exact(resolved, ['schema_version', 'profile', 'harness', 'role', 'value', 'origin', 'fallback'])
  && resolved.schema_version === 1 && resolved.harness === 'claude' && resolved.role === 'verifier'
  && typeof resolved.profile === 'string' && resolved.profile
  && ((exact(resolved.value, ['inherit']) && resolved.value.inherit === true)
    || (exact(resolved.value, ['model', 'effort']) && typeof resolved.value.model === 'string'
      && resolved.value.model && ['inherit', 'low', 'medium', 'high', 'xhigh', 'max'].includes(resolved.value.effort)))
  ? resolved.value : null
if (!native) return `BLOCKER zdev-audit\n\nBoundary: ${boundary}\nInspected: none\nOmitted: the requested boundary\nChecked evidence: profile resolution failed.\nFailed stage: profile selection.`
const verifierOptions = (label) => native.inherit === true
  ? { agentType: 'zdev:zdev-verifier', label }
  : native.effort === 'inherit' ? { agentType: 'zdev:zdev-verifier', label, model: native.model }
    : { agentType: 'zdev:zdev-verifier', label, model: native.model, effort: native.effort }

let workerOutput
if (requestedLenses.length === 0) {
  workerOutput = await agent(
    `${auditContract}\n\n${repositoryGuidance}\n\nAudit boundary: ${boundary}\nFollow the applicable repository guidance. Open and check the evidence, then return the public zdev-audit envelope.`,
    verifierOptions('audit checking verifier'),
  )
} else {
  const reviewScopes = requestedLenses.map(lens => `${lens} lens`)
  const reviews = await pipeline(reviewScopes, scope =>
    agent(
      `${auditContract}\n\n${repositoryGuidance}\n\nReview boundary: ${boundary}\nLens: ${scope}\nFollow the applicable repository guidance. Return candidate evidence for fresh vetting.`,
      verifierOptions(`${scope} audit`),
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
    verifierOptions('audit evidence vetter'),
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

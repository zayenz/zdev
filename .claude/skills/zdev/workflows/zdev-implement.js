export const meta = {
  name: 'zdev-implement',
  description: 'Implement, independently verify, complete, and commit one ready zdev task',
}

const repositoryGuidance = "<!-- zdev:generated-repository-guidance:start -->\n## Repository guidance discovery\n\nBefore inspecting, planning, changing, or validating code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.\n<!-- zdev:generated-repository-guidance:end -->"
const workerContract = repositoryGuidance
const taskWorkflowContractPath = "/Users/zayenz/projects/zdev/.claude/skills/zdev/contracts/task-workflows.md"
const normalizeAreaArg = value => {
  if (Array.isArray(value)) return value[0]
  if (typeof value === 'string') return value
  return value && typeof value === 'object' ? value.area : ''
}
const input = {
  area: normalizeAreaArg(args),
  taskId: args && typeof args === 'object' && !Array.isArray(args)
    ? args.task_id ?? args.taskId ?? null
    : null,
}
const area = String(input.area ?? '').trim()
const selectedTaskId = input.taskId === null ? null : String(input.taskId).trim()

const field = (text, name) => {
  const lines = text.split('\n')
  const matches = lines.flatMap((line, index) =>
    line.startsWith(`${name}: `) || line.trimEnd() === `${name}:` ? [index] : [])
  if (matches.length !== 1) return null
  const index = matches[0]
  const inline = lines[index].slice(name.length + 1).trim()
  if (inline) return inline
  const values = []
  for (const line of lines.slice(index + 1)) {
    if (!line.trim() || /^[A-Z][A-Za-z ]*:(?: |$)/.test(line)) break
    values.push(line.trim().replace(/^[-*]\s*/, ''))
  }
  return values.length > 0 ? values.join(', ') : null
}
const fromExactLine = (text, expected) => {
  const lines = text.split('\n')
  const results = lines.flatMap((line, index) =>
    /^(?:PASS|BLOCKER) zdev-implement /.test(line.trim()) ? [index] : [])
  return results.length === 1 && lines[results[0]].trim() === expected
    ? lines.slice(results[0]).join('\n').trim()
    : null
}
const advisoryText = 'stale effective-base link; managed rebase remains optional.'
const blocker = (subjectArea, taskId, stage, reason, state, staleAdvisory = false) =>
  `BLOCKER zdev-implement ${subjectArea} ${taskId}\n\nArea: ${subjectArea}\nTask: ${taskId}\n${staleAdvisory ? `Advisory: ${advisoryText}\n` : ''}Failed stage: ${stage}\nReason: ${reason}\nPreserved state: ${state}`
const decodeJsonObject = raw => {
  if (raw && !Array.isArray(raw) && typeof raw === 'object') {
    return { value: raw, raw: JSON.stringify(raw) }
  }
  if (typeof raw !== 'string') return null
  const candidates = []
  let start = -1
  let depth = 0
  let inString = false
  for (let index = 0; index < raw.length; index += 1) {
    const character = raw[index]
    if (inString) {
      if (character === '\\') index += 1
      else if (character === '"') inString = false
      continue
    }
    if (character === '"' && depth > 0) {
      inString = true
    } else if (character === '{') {
      if (depth === 0) start = index
      depth += 1
    } else if (character === '}' && depth > 0) {
      depth -= 1
      if (depth === 0 && start >= 0) {
        const candidate = raw.slice(start, index + 1)
        try {
          const value = JSON.parse(candidate)
          if (value && !Array.isArray(value) && typeof value === 'object') {
            candidates.push({ value, raw: candidate })
          }
        } catch {}
        start = -1
      }
    }
  }
  return depth === 0 && candidates.length === 1 ? candidates[0] : null
}
const parseStoredContext = (raw, expectedArea, expected = null) => {
  const decoded = decodeJsonObject(raw)
  if (!decoded) return null
  const stored = decoded.value
  const required = ['area', 'lifecycle', 'path', 'queue', 'schema_version', 'snapshot', 'task_id']
  if (!required.every(key => Object.hasOwn(stored, key))) return null
  if (stored.schema_version !== 1 || stored.area !== expectedArea) return null
  if (!/^W[0-9a-f]{16}$/.test(stored.snapshot ?? '')) return null
  if (!['open', 'closed'].includes(stored.lifecycle)
    || !['ready', 'empty', 'exhausted'].includes(stored.queue)) return null
  if (stored.task_id === null) {
    if (stored.queue === 'ready' || expected) return null
    return { lifecycle: stored.lifecycle, queue: stored.queue, taskId: null, complexity: null,
      staleAdvisory: Boolean(stored.stale_advisory), head: stored.head ?? null,
      baselineSnapshot: stored.snapshot }
  }
  if (typeof stored.task_id !== 'string' || stored.queue !== 'ready'
    || !['routine', 'standard', 'advanced'].includes(stored.complexity)
    || !/^[0-9a-f]{40}$/.test(stored.head ?? '')
    || typeof stored.stale_advisory !== 'boolean') return null
  if (expected && (stored.task_id !== expected.taskId || stored.head !== expected.head
    || stored.complexity !== expected.complexity)) return null
  return { lifecycle: stored.lifecycle, queue: stored.queue, taskId: stored.task_id,
    complexity: stored.complexity, staleAdvisory: stored.stale_advisory, head: stored.head,
    baselineSnapshot: stored.snapshot }
}
const workerResultKeys = [
  'area',
  'escalation',
  'evidence',
  'findings',
  'kind',
  'schema_version',
  'summary',
  'task_id',
  'verdict',
]
const scanTopLevelObject = raw => {
  let index = 0
  const keys = []
  const rawValues = new Map()
  const skipWhitespace = () => {
    while (/\s/.test(raw[index] ?? '')) index += 1
  }
  const scanString = () => {
    if (raw[index] !== '"') return null
    const start = index
    index += 1
    while (index < raw.length) {
      if (raw[index] === '\\') {
        index += 2
      } else if (raw[index] === '"') {
        index += 1
        try {
          return JSON.parse(raw.slice(start, index))
        } catch {
          return null
        }
      } else {
        index += 1
      }
    }
    return null
  }
  skipWhitespace()
  if (raw[index] !== '{') return null
  index += 1
  while (true) {
    skipWhitespace()
    if (raw[index] === '}') {
      index += 1
      break
    }
    const key = scanString()
    if (key === null) return null
    keys.push(key)
    skipWhitespace()
    if (raw[index] !== ':') return null
    index += 1
    skipWhitespace()
    const valueStart = index
    let depth = 0
    let inString = false
    while (index < raw.length) {
      const character = raw[index]
      if (inString) {
        if (character === '\\') index += 1
        else if (character === '"') inString = false
      } else if (character === '"') {
        inString = true
      } else if (character === '[' || character === '{') {
        depth += 1
      } else if (character === ']' || (character === '}' && depth > 0)) {
        depth -= 1
      } else if (depth === 0 && (character === ',' || character === '}')) {
        break
      }
      index += 1
    }
    if (index === valueStart || inString || depth !== 0) return null
    rawValues.set(key, raw.slice(valueStart, index))
    if (raw[index] === ',') {
      index += 1
      continue
    }
    if (raw[index] === '}') {
      index += 1
      break
    }
    return null
  }
  skipWhitespace()
  if (index !== raw.length) return null
  return { keys, rawValues }
}
const topLevelKeys = raw => scanTopLevelObject(raw)?.keys ?? null
const validateWorkerResult = (result, expectedKind, expectedArea, expectedTask) => {
  if (!result || Array.isArray(result) || typeof result !== 'object') return null
  if (JSON.stringify(Object.keys(result).sort()) !== JSON.stringify(workerResultKeys)) return null
  if (result.schema_version !== 1 || result.kind !== expectedKind) return null
  if (result.area !== expectedArea || result.task_id !== expectedTask) return null
  if (typeof result.summary !== 'string' || !result.summary.trim()) return null
  for (const name of ['evidence', 'findings']) {
    if (!Array.isArray(result[name])) return null
    if (!result[name].every(item => typeof item === 'string' && item.trim())) return null
  }
  const validVerdict = expectedKind === 'planner'
    ? ['plan', 'blocker'].includes(result.verdict)
    : expectedKind === 'implementer' ? ['ready', 'blocker'].includes(result.verdict) : false
  if (!validVerdict) return null
  if (expectedKind === 'planner' && result.verdict === 'plan') {
    if (result.evidence.length !== 3) return null
    if (!['Approach: ', 'Paths: ', 'Validation: '].every((prefix, index) =>
      result.evidence[index].startsWith(prefix) && result.evidence[index].length > prefix.length)) return null
  }
  if (expectedKind === 'planner' && result.verdict === 'blocker'
    && (result.evidence.length !== 0 || result.findings.length === 0)) return null
  return result.escalation === 'none' ? result : null
}
const parseWorkerResult = (raw, expectedKind, expectedArea, expectedTask) => {
  const decoded = decodeJsonObject(raw)
  if (!decoded) return null
  const keys = topLevelKeys(decoded.raw)
  if (!keys || new Set(keys).size !== keys.length) return null
  if (JSON.stringify([...keys].sort()) !== JSON.stringify(workerResultKeys)) return null
  return validateWorkerResult(decoded.value, expectedKind, expectedArea, expectedTask)
}
const plannerSchema = {
  type: 'object',
  additionalProperties: false,
  required: ['verdict', 'summary', 'plan', 'findings'],
  properties: {
    verdict: { type: 'string', enum: ['plan', 'blocker'] },
    summary: { type: 'string', minLength: 1 },
    plan: { anyOf: [
      { type: 'null' },
      { type: 'object', additionalProperties: false,
        required: ['approach', 'paths', 'validation'], properties: {
          approach: { type: 'string', minLength: 1 },
          paths: { type: 'array', minItems: 1, items: { type: 'string', minLength: 1 } },
          validation: { type: 'array', minItems: 1, items: { type: 'string', minLength: 1 } },
        } },
    ] },
    findings: { type: 'array', items: { type: 'string', minLength: 1 } },
  },
}
const semanticPlannerKeys = ['findings', 'plan', 'summary', 'verdict']
const semanticPlanKeys = ['approach', 'paths', 'validation']
const normalizedRepositoryPath = path => typeof path === 'string' && path.trim() === path
  && path.length > 0 && !path.includes('\\')
  && (path.startsWith('/') ? path.slice(1) : path)
    .split('/').every(part => part && part !== '.' && part !== '..')
const validateSemanticPlannerResult = result => {
  if (!result || Array.isArray(result) || typeof result !== 'object') return null
  if (JSON.stringify(Object.keys(result).sort()) !== JSON.stringify(semanticPlannerKeys)) return null
  if (typeof result.summary !== 'string' || !result.summary.trim()) return null
  if (!Array.isArray(result.findings)
    || !result.findings.every(item => typeof item === 'string' && item.trim())) return null
  if (result.verdict === 'blocker') return result.plan === null && result.findings.length > 0 ? result : null
  if (result.verdict !== 'plan'
    || !result.plan || Array.isArray(result.plan) || typeof result.plan !== 'object') return null
  if (JSON.stringify(Object.keys(result.plan).sort()) !== JSON.stringify(semanticPlanKeys)) return null
  if (typeof result.plan.approach !== 'string' || !result.plan.approach.trim()) return null
  if (!Array.isArray(result.plan.paths) || result.plan.paths.length === 0
    || !result.plan.paths.every(normalizedRepositoryPath)) return null
  if (!Array.isArray(result.plan.validation) || result.plan.validation.length === 0
    || !result.plan.validation.every(item => typeof item === 'string' && item.trim())) return null
  return result
}
const parsePlannerResult = raw => {
  const decoded = decodeJsonObject(raw)
  if (!decoded) return null
  const scanned = scanTopLevelObject(decoded.raw)
  const keys = scanned?.keys
  if (!keys || new Set(keys).size !== keys.length
    || JSON.stringify([...keys].sort()) !== JSON.stringify(semanticPlannerKeys)) return null
  try {
    const result = decoded.value
    if (result?.verdict === 'plan') {
      const planKeys = topLevelKeys(scanned.rawValues.get('plan'))
      if (!planKeys || new Set(planKeys).size !== planKeys.length
        || JSON.stringify([...planKeys].sort()) !== JSON.stringify(semanticPlanKeys)) return null
    }
    return validateSemanticPlannerResult(result)
  } catch { return null }
}
const reconstructPlannerResult = (semantic, area, taskId) => validateWorkerResult({
  schema_version: 1, kind: 'planner', area, task_id: taskId,
  verdict: semantic.verdict, summary: semantic.summary,
  evidence: semantic.verdict === 'plan' ? [
    `Approach: ${semantic.plan.approach}`,
    `Paths: ${semantic.plan.paths.join(', ')}`,
    `Validation: ${semantic.plan.validation.join('; ')}`,
  ] : [],
  findings: semantic.findings, escalation: 'none',
}, 'planner', area, taskId)
const verifierResultKeys = ['escalation', 'findings', 'summary', 'verdict']
const validationWriteMarker = 'validation_write:'
const validationWritePrefix = 'validation_write: '
const reportsValidationWrite = result => {
  const marked = result.findings.filter(item => item.startsWith(validationWriteMarker))
  return result.verdict === 'rework' && marked.length > 0
    && marked.every(item => {
      if (!item.startsWith(validationWritePrefix)) return false
      const path = item.slice(validationWritePrefix.length)
      return !path.startsWith('/') && !path.includes('\\')
        && path.split('/').every(part => part && part !== '.' && part !== '..')
    })
}
const parseVerifierResult = raw => {
  const decoded = decodeJsonObject(raw)
  if (!decoded) return null
  const keys = topLevelKeys(decoded.raw)
  if (!keys || new Set(keys).size !== keys.length) return null
  if (JSON.stringify([...keys].sort()) !== JSON.stringify(verifierResultKeys)) return null
  const result = decoded.value
  if (!result || Array.isArray(result) || typeof result !== 'object') return null
  if (['schema_version', 'kind', 'area', 'task_id', 'evidence'].some(key => Object.hasOwn(result, key))) return null
  if (!['pass', 'rework', 'blocker'].includes(result.verdict)) return null
  if (typeof result.summary !== 'string' || !result.summary.trim()) return null
  if (!Array.isArray(result.findings)
    || !result.findings.every(item => typeof item === 'string' && item.trim())) return null
  if (result.verdict === 'pass' && result.findings.length !== 0) return null
  if (result.verdict === 'rework' && result.findings.length === 0) return null
  const validEscalation = result.escalation === 'none'
    || (result.verdict === 'rework' && result.escalation === 'advanced-implementer')
  return validEscalation ? result : null
}
const parseComparison = (raw, expectedArea, expectedSnapshot) => {
  const decoded = decodeJsonObject(raw)
  if (!decoded) return null
  const result = decoded.value
  if (!result || Array.isArray(result) || typeof result !== 'object') return null
  if (JSON.stringify(Object.keys(result).sort()) !== JSON.stringify(['area', 'equal', 'schema_version', 'snapshot'])) return null
  return result.schema_version === 1 && result.area === expectedArea
    && result.snapshot === expectedSnapshot && typeof result.equal === 'boolean'
    ? result : null
}
const publicVerifier = (semantic, snapshot, advisory) => {
  const result = {
    schema_version: 1,
    kind: 'verifier',
    area,
    task_id: taskId,
    verdict: semantic.verdict,
    summary: semantic.summary,
    evidence: [`work_context_snapshot: ${snapshot}`, ...(advisory ? [advisory] : [])],
    findings: semantic.findings,
    escalation: semantic.escalation,
  }
  return JSON.stringify(Object.keys(result).sort()) === JSON.stringify(workerResultKeys)
    ? result : null
}
const derivedSplitFrom = (result, expectedArea, expectedTask) => {
  if (result?.kind !== 'implementer' || result.verdict !== 'blocker'
    || result.escalation !== 'none' || result.findings.length !== 0
    || result.evidence.length !== 1) return null
  const proposal = result.evidence[0]
  const first = `PROPOSE zdev-derived ${expectedArea} ${expectedTask}\n`
  if (!proposal.startsWith(first)) return null
  try {
    const payload = JSON.parse(proposal.slice(first.length))
    return payload?.proposal === 'implementation_split'
      && payload.area === expectedArea && payload.source_task === expectedTask
      ? proposal
      : null
  } catch {
    return null
  }
}

if (!/^[a-z0-9][a-z0-9-]*$/.test(area)) {
  return blocker('unknown', 'unknown', 'input', 'a lowercase area is required.', 'no preflight or worker was started.')
}
if (selectedTaskId !== null && !/^[a-z0-9][a-z0-9-]*-[0-9]+$/.test(selectedTaskId)) {
  return blocker(area, 'unknown', 'input', 'task_id must be a zdev task ID.', 'no preflight or worker was started.')
}

const preflight = (label, selected = selectedTaskId) => agent(
  `Act only as read-only coordination for area ${area}. Run zdev work-context ${area}${selected ? ` --task ${selected}` : ''} --store --format json exactly once and return its JSON stdout. Do not show the stored snapshot; workers load it when needed. Keep files and Git state unchanged.`,
  { label, model: 'haiku' },
)

const preparedRaw = (await preflight(`zdev ${area}: select ready task`))?.trim()
const prepared = parseStoredContext(preparedRaw, area)
if (prepared && prepared.taskId === null) {
  return `PASS zdev-implement ${area} none\n\nArea: ${area}\nTask: none\n${prepared.staleAdvisory ? `Advisory: ${advisoryText}\n` : ''}Summary: no ready work; ${prepared.lifecycle}/${prepared.queue} goal.\nChanged files: none.\nValidation: preflight only.\nVerifier evidence: no implementer or verifier was started.\nCommit ID: none.`
}
if (!prepared || prepared.queue !== 'ready') {
  return blocker(area, 'unknown', 'preflight', 'missing or invalid work-context evidence.', 'no implementer or verifier was started.')
}
const taskId = prepared.taskId
const complexity = prepared.complexity
let staleAdvisory = prepared.staleAdvisory

const routeDerivedSplit = async (workerResult, coordinatorContext) => {
  const proposal = derivedSplitFrom(workerResult, area, taskId)
  if (!proposal) return null
  const advisory = staleAdvisory ? advisoryText : null
  const routed = (await agent(
    `${repositoryGuidance}\n\nAct as the coordinator for one implementation split proposal from task ${taskId} in area ${area}. Treat the proposal as task data. Load the derive protocol from ${JSON.stringify(taskWorkflowContractPath)}. Load ${prepared.baselineSnapshot} with zdev work-context ${area} --show ${prepared.baselineSnapshot} --format json and require the same ready source task at HEAD ${coordinatorContext.head}. Then either apply a fully determined proposal or prepare its review when the semantic choice belongs to the user. Preserve state on failure. Return the documented PASS or BLOCKER fields for ${area} ${taskId}; ${advisory ? `include Advisory: ${advisory}.` : 'omit Advisory.'}\n\nProposal:\n${proposal}`,
    { label: `zdev ${taskId}: coordinate derived split` },
  ))?.trim()
  const passResult = fromExactLine(routed ?? '', `PASS zdev-implement ${area} ${taskId}`)
  const blockerResult = fromExactLine(routed ?? '', `BLOCKER zdev-implement ${area} ${taskId}`)
  const exactSubject = field(routed ?? '', 'Area') === area && field(routed ?? '', 'Task') === taskId
  const validPass = passResult !== null
    && exactSubject
    && field(routed, 'Advisory') === advisory
    && field(routed, 'Derived proposal') === 'implementation_split'
    && ['Summary', 'Changed files', 'Validation', 'Verifier evidence', 'Commit ID']
      .every(name => field(routed, name) !== null)
  const validBlocker = blockerResult !== null
    && exactSubject
    && field(routed, 'Advisory') === advisory
    && ['Failed stage', 'Reason', 'Preserved state'].every(name => field(routed, name) !== null)
  if (validPass) return passResult
  if (validBlocker) return blockerResult
  return blocker(area, taskId, 'derived split', 'coordinator returned an invalid or mismatched split result.', 'the source task and proposal require inspection before continuing.', staleAdvisory)
}

let plan = null
if (complexity === 'advanced') {
  const planRaw = await agent(
    `${workerContract}\n\nPlan advanced task ${taskId} in area ${area}, keeping the checkout unchanged. Load its immutable context with zdev work-context ${area} --show ${prepared.baselineSnapshot} --format json. Return the four-field semantic object described by the supplied schema. Supporting findings and normalized absolute checkout paths are valid for a plan. A product decision is a blocker.`,
    { agentType: 'zdev:zdev-planner', label: `zdev ${taskId}: plan`, schema: plannerSchema },
  )
  const semanticPlan = parsePlannerResult(typeof planRaw === 'string' ? planRaw.trim() : planRaw)
  plan = semanticPlan && reconstructPlannerResult(semanticPlan, area, taskId)
  if (!semanticPlan || !plan) {
    return blocker(area, taskId, 'planning', 'planner returned an invalid or mismatched envelope.', 'no implementation, lifecycle, or commit change was started.', staleAdvisory)
  }
  if (plan.verdict === 'blocker') {
    return blocker(area, taskId, 'planning', plan.summary, `Evidence: none. Findings: ${plan.findings.join('; ')}`, staleAdvisory)
  }
  plan = semanticPlan
}
const implementationAgentType = complexity === 'routine'
  ? 'zdev:zdev-routine-implementer'
  : complexity === 'advanced'
    ? 'zdev:zdev-advanced-implementer'
    : 'zdev:zdev-implementer'
const implementationRaw = (await agent(
  `${workerContract}\n\nImplement ${complexity} task ${taskId} in area ${area}. Load its immutable context with zdev work-context ${area} --show ${prepared.baselineSnapshot} --format json.${plan ? ` Follow this validated plan: ${JSON.stringify(plan)}.` : ''} Treat named and planned paths as expected seams rather than an allowlist. Change every attributable path directly needed by the task's semantic boundaries, validate the result, and return the implementer envelope from your role prompt. Do not block merely for partial progress or another file. If direct work must split, load ${JSON.stringify(taskWorkflowContractPath)}, use its typed implementation_split blocker, and leave derive commands to the coordinator.`,
  { agentType: implementationAgentType, label: `zdev ${taskId}: implement (${complexity})` },
))?.trim()
const implementation = parseWorkerResult(implementationRaw, 'implementer', area, taskId)
let latestImplementation = implementation
let activeAgentType = implementationAgentType
let escalated = false
const compactWorkerSummary = result => JSON.stringify({
  summary: result.summary,
  evidence: result.evidence,
})

const refresh = async label => {
  const current = parseStoredContext((await preflight(label, taskId))?.trim(), area, {
    taskId, head: prepared.head, complexity,
  })
  if (current?.staleAdvisory) staleAdvisory = true
  return current?.queue === 'ready' && current.complexity === complexity ? current : blocker(area, taskId, 'context refresh', `expected ready task ${taskId} with unchanged complexity ${complexity} and complete work-context evidence.`, 'lifecycle and commit were not changed.', staleAdvisory)
}
const verify = async () => {
  const storedRaw = (await agent(
    `Act only as read-only verification coordination. Run zdev work-context ${area} --task ${taskId} --store --format json exactly once and return its JSON stdout. Do not show the stored snapshot.`,
    { label: `zdev ${taskId}: capture verification snapshot`, model: 'haiku' },
  ))?.trim()
  const stored = parseStoredContext(storedRaw, area, {
    taskId, head: prepared.head, complexity,
  })
  if (!stored) return null
  if (stored.staleAdvisory) staleAdvisory = true
  const currentAdvisory = staleAdvisory ? advisoryText : null
  const current = stored
  const snapshot = stored.baselineSnapshot
  const raw = (await agent(
    `${workerContract}\n\nIndependently verify task ${taskId} in area ${area}. Load the original baseline with zdev work-context ${area} --show ${prepared.baselineSnapshot} --format json and the verification snapshot with zdev work-context ${area} --show ${snapshot} --format json; require task ${taskId} at HEAD ${current.head}. Use the implementer summary only to locate evidence. Check the whole task and run required validation. Return exactly one JSON object with exactly these four keys and no others: verdict, summary, findings, escalation. Pass requires an empty findings array; rework requires at least one finding. Report each validation-written task-owned file as a validation_write: <repository-relative path> finding with verdict rework. Never add validation_writes or another fifth key. Do not repair or discard validation writes.\n\nImplementer summary: ${compactWorkerSummary(latestImplementation)}`,
    { agentType: 'zdev:zdev-verifier', label: `zdev ${taskId}: verify` },
  ))?.trim()
  const semantic = parseVerifierResult(raw)
  const comparedRaw = (await agent(
    `Act only as deterministic post-verification coordination. Run zdev work-context ${area} --compare ${snapshot} --format json exactly once and return its complete JSON stdout unchanged, with no fence or other text. Keep files and Git state unchanged.`,
    { label: `zdev ${taskId}: confirm verifier left snapshot unchanged`, model: 'haiku' },
  ))?.trim()
  const compared = parseComparison(comparedRaw, area, snapshot)
  if (!semantic || !compared) return null
  if (!compared.equal && !reportsValidationWrite(semantic)) return null
  const result = publicVerifier(semantic, snapshot, currentAdvisory)
  if (!result) return null
  return {
    raw: JSON.stringify(result),
    result,
    approved: result.verdict === 'pass' ? snapshot : true,
  }
}

if (!implementation) {
  return blocker(area, taskId, 'implementation', 'implementer returned an invalid or mismatched envelope.', 'lifecycle and commit were not changed.', staleAdvisory)
}
const initialSplit = await routeDerivedSplit(implementation, prepared)
if (initialSplit) return initialSplit
if (implementation.verdict === 'blocker') {
  return blocker(area, taskId, 'implementation', implementation.summary, `Evidence: ${implementation.evidence.join('; ') || 'none.'} Findings: ${implementation.findings.join('; ') || 'none.'}`, staleAdvisory)
}
let current = null
let verdict = await verify()
if (!verdict) {
  return blocker(area, taskId, 'verification', 'verifier returned an invalid or mismatched envelope.', 'lifecycle and commit were not changed.', staleAdvisory)
}
while (verdict.result.verdict === 'rework') {
  if (verdict.result.escalation === 'advanced-implementer') {
    if (complexity !== 'standard' || escalated) {
      return blocker(area, taskId, 'rework', 'verifier requested an inapplicable or repeated advanced escalation.', 'lifecycle and commit were not changed.', staleAdvisory)
    }
    escalated = true
    activeAgentType = 'zdev:zdev-advanced-implementer'
  }
  current = await refresh(`zdev ${taskId}: refresh before rework`)
  if (typeof current === 'string') return current
  const reworkRaw = (await agent(
    `${workerContract}\n\nCorrect every concrete task-owned finding for ${taskId}. Load the original baseline with zdev work-context ${area} --show ${prepared.baselineSnapshot} --format json and require current HEAD ${current.head}. Return the implementer envelope from your role prompt. If direct work must split, load ${JSON.stringify(taskWorkflowContractPath)} and use its typed implementation_split blocker.\n\nVerifier findings:\n${verdict.raw}`,
    { agentType: activeAgentType, label: `zdev ${taskId}: ${escalated ? 'advanced ' : ''}rework` },
  ))?.trim()
  const rework = parseWorkerResult(reworkRaw, 'implementer', area, taskId)
  if (!rework) {
    return blocker(area, taskId, 'rework', 'implementer returned an invalid or mismatched envelope.', 'lifecycle and commit were not changed.', staleAdvisory)
  }
  const reworkSplit = await routeDerivedSplit(rework, current)
  if (reworkSplit) return reworkSplit
  if (rework.verdict === 'blocker') {
    return blocker(area, taskId, 'rework', rework.summary, `Evidence: ${rework.evidence.join('; ') || 'none.'} Findings: ${rework.findings.join('; ') || 'none.'}`, staleAdvisory)
  }
  latestImplementation = rework
  verdict = await verify()
  if (!verdict) {
    return blocker(area, taskId, 'verification', 'verifier returned an invalid or mismatched envelope.', 'lifecycle and commit were not changed.', staleAdvisory)
  }
}
if (verdict.result.verdict !== 'pass') {
  return blocker(area, taskId, 'verification', verdict.result.summary, verdict.result.evidence.join('; ') || 'lifecycle and commit were not changed.', staleAdvisory)
}

const advisory = staleAdvisory ? advisoryText : null
const completed = await agent(
  `${repositoryGuidance}\n\nAct as the existing completion coordinator for verified task ${taskId} in area ${area}. Whether this completion is live or resumed, before mutation run exactly one zdev work-context ${area} --compare ${verdict.approved} --format json. Accept the exact four-key JSON object {"schema_version":1,"area":"${area}","snapshot":"${verdict.approved}","equal":true}. On an exact match, run zdev task done, stage the attributed task-owned paths and exact task records, inspect the cached diff, and run zdev commit. Preserve the task-done and index state if staging, cached-diff inspection, or commit needs recovery. Return PASS zdev-implement ${area} ${taskId} or BLOCKER zdev-implement ${area} ${taskId} as the exact first line. Repeat exact Area: ${area} and Task: ${taskId} fields. ${advisory ? `Include Advisory: ${advisory} exactly once, ` : 'Omit Advisory, '}plus Summary, Changed files, Validation, Verifier evidence, and Commit ID on pass, or Failed stage, Reason, and Preserved state on blocker.\n\nCompletion handoff: ${JSON.stringify({ snapshot: verdict.approved, implementation: latestImplementation.summary, verification: verdict.result.summary })}`,
  { label: `zdev ${taskId}: complete and commit` },
)
const result = completed?.trim()
const passResult = fromExactLine(result ?? '', `PASS zdev-implement ${area} ${taskId}`)
const blockerResult = fromExactLine(result ?? '', `BLOCKER zdev-implement ${area} ${taskId}`)
const exactSubject = field(result ?? '', 'Area') === area && field(result ?? '', 'Task') === taskId
const validPass = passResult !== null
  && exactSubject
  && field(result, 'Advisory') === advisory
  && ['Summary', 'Changed files', 'Validation', 'Verifier evidence', 'Commit ID']
    .every(name => field(result, name) !== null)
const validBlocker = blockerResult !== null
  && exactSubject
  && field(result, 'Advisory') === advisory
  && ['Failed stage', 'Reason', 'Preserved state'].every(name => field(result, name) !== null)
if (validPass) return passResult
if (validBlocker) return blockerResult
return blocker(area, taskId, 'completion and commit', 'coordinator returned an invalid or mismatched envelope.', 'inspect the checkout and zdev task record before continuing.', staleAdvisory)

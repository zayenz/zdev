export const meta = {
  name: 'zdev-research',
  description: 'Authorize or resume serial or rolling research with counted workers and durable recovery',
}

const runOneTask = async (args, agent) => {
const repositoryGuidance = "<!-- zdev:generated-repository-guidance:start -->\n## Repository guidance discovery\n\nBefore inspecting, planning, changing, or validating code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.\n<!-- zdev:generated-repository-guidance:end -->\n\n## Coding guidance\n\nCoding guidance file: \"/Users/zayenz/projects/zdev/.claude/skills/zdev/skills/zdev/references/coding.md\"\n\nBefore planning code, read Design and Code standards from this file. Before implementing or repairing code, also read Completion. Before verification, audit, or code review, read Code standards and Review. Pass this exact location to each relevant worker; read it directly even when the root skill is not loaded. Routine coordination and non-code tasks do not need it."
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
  runProfile: args && typeof args === 'object' && !Array.isArray(args)
    ? args.run_profile ?? args.runProfile ?? null : null,
  roleProfiles: args && typeof args === 'object' && !Array.isArray(args)
    ? args.role_profiles ?? args.roleProfiles ?? {} : {},
  planOnly: Boolean(args && typeof args === 'object' && !Array.isArray(args)
    && (args.plan_only ?? args.planOnly)),
  retainedPlan: args && typeof args === 'object' && !Array.isArray(args)
    ? args.retained_plan ?? args.retainedPlan ?? 'none' : 'none',
  planSnapshot: args && typeof args === 'object' && !Array.isArray(args)
    ? args.plan_snapshot ?? args.planSnapshot ?? null : null,
  retainedPlanResult: args && typeof args === 'object' && !Array.isArray(args)
    ? args.retained_plan_result ?? args.retainedPlanResult ?? null : null,
  frozenProfiles: args && typeof args === 'object' && !Array.isArray(args)
    ? args.frozen_profiles ?? args.frozenProfiles ?? null : null,
}
const area = String(input.area ?? '').trim()
const selectedTaskId = input.taskId === null ? null : String(input.taskId).trim()
const profileName = value => value === null ? null : String(value).trim()
const runProfile = profileName(input.runProfile)
const roleProfiles = input.roleProfiles && !Array.isArray(input.roleProfiles)
  && typeof input.roleProfiles === 'object' ? input.roleProfiles : null

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
const hasDuplicateObjectKeys = raw => {
  if (typeof raw !== 'string') return false
  const stack = []
  for (let index = 0; index < raw.length; index += 1) {
    const c = raw[index]
    if (c === '"') {
      const start = index++
      while (index < raw.length && raw[index] !== '"') {
        if (raw[index] === '\\') index += 1
        index += 1
      }
      if (index >= raw.length) return true
      let next = index + 1
      while (/\s/.test(raw[next] ?? '')) next += 1
      const current = stack.at(-1)
      if (raw[next] === ':' && current instanceof Set) {
        let key
        try { key = JSON.parse(raw.slice(start, index + 1)) } catch { return true }
        if (current.has(key)) return true
        current.add(key)
      }
    } else if (c === '{') stack.push(new Set())
    else if (c === '[') stack.push(null)
    else if (c === '}' || c === ']') stack.pop()
  }
  return false
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
    || typeof stored.stale_advisory !== 'boolean'
    || typeof stored.task_path !== 'string'
    || !stored.task_path.startsWith(`.zdev/${expectedArea}/tasks/`)
    || !stored.task_path.endsWith('.md')) return null
  if (expected && (stored.task_id !== expected.taskId || stored.head !== expected.head
    || stored.complexity !== expected.complexity
    || (expected.taskPath && stored.task_path !== expected.taskPath))) return null
  return { lifecycle: stored.lifecycle, queue: stored.queue, taskId: stored.task_id,
    complexity: stored.complexity, staleAdvisory: stored.stale_advisory, head: stored.head,
    baselineSnapshot: stored.snapshot, taskPath: stored.task_path,
    hypothesis: stored.hypothesis }
}
const parseDispatchSpec = (raw, expectedRoute, expectedTask, expectedSnapshot) => {
  const decoded = decodeJsonObject(raw)
  const value = decoded?.value
  const roles = ['routine-implementer', 'implementer', 'advanced-implementer', 'planner', 'verifier']
  const exact = (object, keys) => object && !Array.isArray(object) && typeof object === 'object'
    && JSON.stringify(Object.keys(object).sort()) === JSON.stringify([...keys].sort())
  const validValue = value => (exact(value, ['inherit']) && value.inherit === true)
    || (exact(value, ['model', 'effort'])
      && typeof value.model === 'string' && value.model.trim() === value.model && value.model
      && typeof value.effort === 'string' && ['inherit', 'low', 'medium', 'high', 'xhigh', 'max'].includes(value.effort))
  const validRole = (record, role) => exact(record,
    ['schema_version', 'profile', 'harness', 'role', 'value', 'origin', 'fallback'])
    && record.schema_version === 1 && record.harness === 'claude' && record.role === role
    && typeof record.profile === 'string' && record.profile.trim() === record.profile && record.profile
    && validValue(record.value)
  const scanned = decoded && scanTopLevelObject(decoded.raw)
  if (!value || hasDuplicateObjectKeys(decoded.raw) || !scanned || new Set(scanned.keys).size !== scanned.keys.length
    || !exact(value, ['schema_version', 'kind', 'harness', 'route', 'area', 'task_id', 'snapshot', 'dispatches', 'profiles', 'stop'])
    || value.schema_version !== 1 || value.kind !== 'dispatch-spec'
    || value.harness !== 'claude' || value.route !== expectedRoute
    || value.area !== area || value.task_id !== expectedTask || value.snapshot !== expectedSnapshot
    || !exact(value.profiles, roles) || !roles.every(role => validRole(value.profiles[role], role))
    || !Array.isArray(value.dispatches)) return null
  const dispatchKeys = ['role', 'profile', 'model', 'reasoning_effort', 'task_id', 'snapshot', 'next_phase']
  if (!value.dispatches.every(item => exact(item, dispatchKeys)
    && roles.includes(item.role) && typeof item.profile === 'string' && item.profile
    && (item.model === null || typeof item.model === 'string' && item.model)
    && (item.reasoning_effort === null || ['inherit', 'low', 'medium', 'high', 'xhigh', 'max'].includes(item.reasoning_effort))
    && item.task_id === expectedTask && item.snapshot === expectedSnapshot
    && typeof item.next_phase === 'string' && item.next_phase)) return null
  if (expectedRoute === 'plan-next-task'
    && (value.stop !== 'plan-only' || value.dispatches.length !== 1
      || value.dispatches[0].role !== 'planner' || value.dispatches[0].next_phase !== 'plan-only-stop'
      || value.dispatches[0].profile !== value.profiles.planner.profile
      || value.dispatches[0].model !== (value.profiles.planner.value.model ?? null)
      || value.dispatches[0].reasoning_effort !== (value.profiles.planner.value.effort ?? null))) return null
  if (expectedRoute === 'implement' && value.stop !== 'completion') return null
  return value
}
const nativeWorkerOptions = (spec, role, extra = {}) => {
  const resolved = spec.profiles[role]
  const value = resolved?.value
  if (!value || value.inherit === true) return { ...extra }
  if (typeof value.model !== 'string' || typeof value.effort !== 'string') return null
  return value.effort === 'inherit' ? { ...extra, model: value.model }
    : { ...extra, model: value.model, effort: value.effort }
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
const blockerDispositionSchema = {
  type: 'object',
  additionalProperties: false,
  required: ['action', 'reason'],
  properties: {
    action: { type: 'string', enum: ['continue', 'stop'] },
    reason: { type: 'string', minLength: 1 },
  },
}
const technicalAdjustmentSchema = {
  type: 'object', additionalProperties: false,
  required: ['action', 'reason', 'record_paths'],
  properties: {
    action: { type: 'string', enum: ['adjusted', 'stop'] },
    reason: { type: 'string', minLength: 1 },
    record_paths: { type: 'array', items: { type: 'string', minLength: 1 } },
  },
}
const parseBlockerDisposition = raw => {
  const decoded = decodeJsonObject(raw)
  const result = decoded?.value
  return result && !Array.isArray(result) && typeof result === 'object'
    && JSON.stringify(Object.keys(result).sort()) === JSON.stringify(['action', 'reason'])
    && ['continue', 'stop'].includes(result.action)
    && typeof result.reason === 'string' && result.reason.trim()
    ? result : null
}
const parseTechnicalAdjustmentResult = raw => {
  const result = decodeJsonObject(raw)?.value
  if (!result || Array.isArray(result) || typeof result !== 'object'
    || JSON.stringify(Object.keys(result).sort()) !== JSON.stringify(['action', 'reason', 'record_paths'])
    || !['adjusted', 'stop'].includes(result.action)
    || typeof result.reason !== 'string' || !result.reason.trim()
    || !Array.isArray(result.record_paths)
    || !result.record_paths.every(path => typeof path === 'string' && path.trim())) return null
  if (result.action === 'adjusted' && result.record_paths.length === 0) return null
  if (result.action === 'stop' && result.record_paths.length !== 0) return null
  return result
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
const onlyValidationWrites = result => reportsValidationWrite(result)
  && result.findings.every(item => item.startsWith(validationWritePrefix))
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
const technicalAdjustmentFrom = (result, expectedArea, expectedTask, expectedTaskPath) => {
  if (result?.kind !== 'implementer' || result.verdict !== 'blocker'
    || result.escalation !== 'none' || result.findings.length !== 0
    || result.evidence.length !== 1) return null
  const first = `PROPOSE zdev-adjustment ${expectedArea} ${expectedTask}\n`
  if (!result.evidence[0].startsWith(first)) return null
  let proposal
  try { proposal = JSON.parse(result.evidence[0].slice(first.length)) } catch { return null }
  if (!proposal || Array.isArray(proposal) || typeof proposal !== 'object'
    || JSON.stringify(Object.keys(proposal).sort()) !== JSON.stringify(['explanation', 'record_paths'])
    || typeof proposal.explanation !== 'string' || !proposal.explanation.trim()
    || !Array.isArray(proposal.record_paths) || proposal.record_paths.length === 0
    || new Set(proposal.record_paths).size !== proposal.record_paths.length
    || !proposal.record_paths.every(path => path === `.zdev/${expectedArea}/brief.md`
      || path === expectedTaskPath)) return null
  return proposal
}

if (!/^[a-z0-9][a-z0-9-]*$/.test(area)) {
  return blocker('unknown', 'unknown', 'input', 'a lowercase area is required.', 'no preflight or worker was started.')
}
if (selectedTaskId !== null && !/^[a-z0-9][a-z0-9-]*-[0-9]+$/.test(selectedTaskId)) {
  return blocker(area, 'unknown', 'input', 'task_id must be a zdev task ID.', 'no preflight or worker was started.')
}
if ((runProfile !== null && !/^[A-Za-z0-9][A-Za-z0-9_-]*$/.test(runProfile))
  || !roleProfiles
  || !Object.entries(roleProfiles).every(([role, profile]) =>
    ['routine-implementer', 'implementer', 'advanced-implementer', 'planner', 'verifier'].includes(role)
    && /^[A-Za-z0-9][A-Za-z0-9_-]*$/.test(String(profile)))) {
  return blocker(area, 'unknown', 'input', 'profile selections are invalid.', 'no preflight or worker was started.')
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
const dispatchRoute = input.planOnly ? 'plan-next-task' : 'implement'
const profileFlags = [
  runProfile ? ` --run-profile ${runProfile}` : '',
  ...Object.entries(roleProfiles).map(([role, profile]) => ` --role-profile ${role}=${profile}`),
  input.planSnapshot ? ` --plan-snapshot ${String(input.planSnapshot)}` : '',
  input.retainedPlan !== 'none' ? ` --retained-plan ${String(input.retainedPlan)}` : '',
].join('')
const frozenRoles = ['routine-implementer', 'implementer', 'advanced-implementer', 'planner', 'verifier']
const validFrozenRole = (record, role) => record && !Array.isArray(record)
  && JSON.stringify(Object.keys(record).sort()) === JSON.stringify(['fallback', 'harness', 'origin', 'profile', 'role', 'schema_version', 'value'])
  && record.schema_version === 1 && record.role === role && record.harness === 'claude'
  && typeof record.profile === 'string' && record.profile
  && record.value && ((JSON.stringify(Object.keys(record.value).sort()) === JSON.stringify(['inherit'])
      && record.value.inherit === true)
    || (JSON.stringify(Object.keys(record.value).sort()) === JSON.stringify(['effort', 'model'])
      && typeof record.value.model === 'string' && record.value.model
      && ['inherit', 'low', 'medium', 'high', 'xhigh', 'max'].includes(record.value.effort)))
const suppliedFrozen = input.frozenProfiles
  && JSON.stringify(Object.keys(input.frozenProfiles).sort()) === JSON.stringify([...frozenRoles].sort())
  && frozenRoles.every(role => validFrozenRole(input.frozenProfiles[role], role))
if (suppliedFrozen && prepared.hypothesis !== false) {
  return blocker(area, taskId, 'task kind', 'frozen ordinary dispatch requires confirmed supporting work; hypotheses use the authorized research route.', 'no task worker was started.', staleAdvisory)
}
const dispatchRaw = suppliedFrozen ? null : await agent(
  `Act only as deterministic profile coordination. Run zdev config profile dispatch-spec ${dispatchRoute} --harness claude --area ${area} --task ${taskId} --snapshot ${prepared.baselineSnapshot}${profileFlags} --format json exactly once and return its complete JSON stdout unchanged. Keep files and Git state unchanged.`,
  { label: `zdev ${taskId}: freeze worker profiles`, model: 'haiku' },
)
const dispatchSpec = suppliedFrozen
  ? { profiles: input.frozenProfiles, route: dispatchRoute, task_id: taskId,
    snapshot: prepared.baselineSnapshot, stop: 'completion', dispatches: [] }
  : parseDispatchSpec(dispatchRaw, dispatchRoute, taskId, prepared.baselineSnapshot)
if (!dispatchSpec) {
  return blocker(area, taskId, 'profile selection', 'profile resolution or native dispatch capability failed.', 'no task worker was started and configuration was not changed.', staleAdvisory)
}
if (!suppliedFrozen && dispatchRoute === 'implement') {
  const implementationRole = complexity === 'routine' ? 'routine-implementer'
    : complexity === 'advanced' ? 'advanced-implementer' : 'implementer'
  const expectedRoles = complexity === 'advanced' && input.retainedPlan !== 'applicable'
    ? ['planner', implementationRole, 'verifier'] : [implementationRole, 'verifier']
  const expectedPhases = expectedRoles.map((_, index) => index === expectedRoles.length - 1
    ? 'completion' : index === 0 && expectedRoles[0] === 'planner' ? 'implementation' : 'verification')
  if (JSON.stringify(dispatchSpec.dispatches.map(item => item.role)) !== JSON.stringify(expectedRoles)
    || JSON.stringify(dispatchSpec.dispatches.map(item => item.next_phase)) !== JSON.stringify(expectedPhases)
    || dispatchSpec.dispatches.some(item => {
      const resolved = dispatchSpec.profiles[item.role]
      return item.profile !== resolved.profile || item.model !== (resolved.value.model ?? null)
        || item.reasoning_effort !== (resolved.value.effort ?? null)
    })) {
    return blocker(area, taskId, 'profile selection', 'dispatch sequence was incomplete or inconsistent with the frozen role map.', 'no task worker was started.', staleAdvisory)
  }
}
const workerOptions = (role, extra = {}) => nativeWorkerOptions(dispatchSpec, role, extra)
if (input.planOnly) {
  const selectedPlanner = dispatchSpec.dispatches?.[0]
  if (dispatchSpec.stop !== 'plan-only' || dispatchSpec.dispatches?.length !== 1
    || selectedPlanner?.role !== 'planner') {
    return blocker(area, taskId, 'plan-only dispatch', 'dispatch specification did not admit exactly one planner.', 'no planner was started.', staleAdvisory)
  }
  const options = selectedPlanner.model === null
    ? { agentType: 'zdev:zdev-planner', label: `zdev ${taskId}: plan only` }
    : selectedPlanner.reasoning_effort === null || selectedPlanner.reasoning_effort === 'inherit'
      ? { agentType: 'zdev:zdev-planner', label: `zdev ${taskId}: plan only`, model: selectedPlanner.model }
      : { agentType: 'zdev:zdev-planner', label: `zdev ${taskId}: plan only`,
        model: selectedPlanner.model, effort: selectedPlanner.reasoning_effort }
  const raw = await agent(
    `${workerContract}\n\nPlan task ${taskId} in area ${area}, keeping the checkout unchanged. Load its immutable context with zdev work-context ${area} --show ${prepared.baselineSnapshot} --format json. Return the four-field semantic planner object and stop.`,
    options,
  )
  const semantic = parsePlannerResult(typeof raw === 'string' ? raw.trim() : raw)
  if (!semantic) return blocker(area, taskId, 'plan-only dispatch', 'planner returned an invalid result.', 'no implementation, verification, lifecycle, or commit work was started.', staleAdvisory)
  return JSON.stringify({ task_id: taskId, snapshot: prepared.baselineSnapshot,
    selected_profile: selectedPlanner.profile, model: selectedPlanner.model,
    effort: selectedPlanner.reasoning_effort, ...semantic })
}

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

let plan = input.retainedPlan === 'applicable'
  ? validateSemanticPlannerResult(input.retainedPlanResult) : null
if (input.retainedPlan === 'applicable' && (!plan || plan.verdict !== 'plan')) {
  return blocker(area, taskId, 'retained plan', 'applicable retained plan content is missing or invalid.', 'no implementation was started.', staleAdvisory)
}
if (complexity === 'advanced' && input.retainedPlan !== 'applicable') {
  const planRaw = await agent(
    `${workerContract}\n\nPlan advanced task ${taskId} in area ${area}, keeping the checkout unchanged. Load its immutable context with zdev work-context ${area} --show ${prepared.baselineSnapshot} --format json. Return the four-field semantic object described by the supplied schema. Supporting findings and normalized absolute checkout paths are valid for a plan. A product decision is a blocker.`,
    workerOptions('planner', { agentType: 'zdev:zdev-planner', label: `zdev ${taskId}: plan`, schema: plannerSchema }),
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
  `${workerContract}\n\nImplement ${complexity} task ${taskId} in area ${area}. Load its immutable context with zdev work-context ${area} --show ${prepared.baselineSnapshot} --format json.${plan ? ` Follow this validated plan: ${JSON.stringify(plan)}.` : ''} Treat named and planned paths as expected seams rather than an allowlist. Change every attributable path directly needed by the task's semantic boundaries, validate the result, and return the implementer envelope from your role prompt. Do not block merely for partial progress or another file. If a planner-written technical restriction itself must be corrected, load ${JSON.stringify(taskWorkflowContractPath)} and use its transient adjustment blocker so coordination can check and edit the record. When smaller tasks would help implementation or verification, use its typed implementation_split blocker and leave derive commands to the coordinator.`,
  workerOptions(complexity === 'routine' ? 'routine-implementer' : complexity === 'advanced' ? 'advanced-implementer' : 'implementer',
    { agentType: implementationAgentType, label: `zdev ${taskId}: implement (${complexity})` }),
))?.trim()
const implementation = parseWorkerResult(implementationRaw, 'implementer', area, taskId)
let latestImplementation = implementation
let activeAgentType = implementationAgentType
let escalated = false
const technicalAdjustments = []
const compactWorkerSummary = result => JSON.stringify({
  summary: result.summary,
  evidence: result.evidence,
})

const refresh = async label => {
  const current = parseStoredContext((await preflight(label, taskId))?.trim(), area, {
    taskId, head: prepared.head, complexity, taskPath: prepared.taskPath,
  })
  if (current?.staleAdvisory) staleAdvisory = true
  return current?.queue === 'ready' && current.complexity === complexity ? current : blocker(area, taskId, 'context refresh', `expected ready task ${taskId} with unchanged complexity ${complexity} and complete work-context evidence.`, 'lifecycle and commit were not changed.', staleAdvisory)
}
const compareSnapshot = async (snapshot, label) => parseComparison((await agent(
  `Act only as deterministic progress coordination. Run zdev work-context ${area} --compare ${snapshot} --format json exactly once and return its complete JSON stdout unchanged, with no fence or other text. Keep files and Git state unchanged.`,
  { label, model: 'haiku' },
))?.trim(), area, snapshot)
const classifyImplementerBlocker = async (result, current, madeProgress, priorResult) => {
  const raw = await agent(
    `${repositoryGuidance}\n\nClassify an implementer blocker for task ${taskId} in area ${area}. Load current snapshot ${current.baselineSnapshot} with zdev work-context ${area} --show ${current.baselineSnapshot} --format json and inspect the authoritative task and attributable diff. Return action continue when directly actionable task work remains, including partial implementation, an underestimated file count, another necessary in-scope path, or new investigation or validation evidence that establishes a concrete next step. Return action stop only for unavailable external state, unsafe or ambiguous ownership, a user-owned product or scope decision, or another concrete impasse. ${madeProgress === null ? 'This is the first ordinary blocker.' : `The replacement ${madeProgress ? 'made attributable checkout progress' : 'left the checkout unchanged'}. Compare the prior and current blocker results. When the checkout is unchanged, stop if the same obstacle remains unresolved without a concrete new next step; continue if new evidence makes remaining task work directly actionable.`}\n\n${priorResult ? `Prior worker result: ${JSON.stringify(priorResult)}\n` : ''}Current worker result: ${JSON.stringify(result)}`,
    { label: `zdev ${taskId}: classify implementer blocker`, model: 'haiku', schema: blockerDispositionSchema },
  )
  return parseBlockerDisposition(typeof raw === 'string' ? raw.trim() : raw)
}
const coordinateTechnicalAdjustment = async (proposal, current) => {
  const raw = await agent(
    `${repositoryGuidance}\n\nAct as coordination for one proposed incidental technical adjustment in active task ${taskId} in area ${area}. Load the original baseline with zdev work-context ${area} --show ${prepared.baselineSnapshot} --format json and current context with zdev work-context ${area} --show ${current.baselineSnapshot} --format json. Check the proposed helper or path against the agreed outcome, acceptance criteria, explicit user constraints, compatibility promises, original Git baseline, and ownership. If it changes only a planner-written technical restriction and ownership is clear, edit only the proposed active task or brief records, then return action adjusted with the exact edited record_paths and a concise explanation. Return action stop with no record paths for an explicit constraint, changed outcome or acceptance criterion, compatibility change, ambiguous ownership, or unresolved material choice. Do not edit a pending reviewed bundle or create state.\n\nProposal: ${JSON.stringify(proposal)}`,
    { label: `zdev ${taskId}: coordinate technical adjustment`, model: 'haiku', schema: technicalAdjustmentSchema },
  )
  const result = parseTechnicalAdjustmentResult(typeof raw === 'string' ? raw.trim() : raw)
  return result?.action === 'adjusted'
    && JSON.stringify(result.record_paths) !== JSON.stringify(proposal.record_paths)
    ? null : result
}
const resolveImplementerResult = async (initial, initialContext, phase) => {
  let result = initial
  let context = initialContext
  let priorBlockerSnapshot = null
  let priorBlockerResult = null
  while (result.verdict === 'blocker') {
    const split = await routeDerivedSplit(result, context)
    if (split) return { terminal: split }
    const proposedAdjustment = technicalAdjustmentFrom(result, area, taskId, prepared.taskPath)
    if (proposedAdjustment) {
      const adjusted = await coordinateTechnicalAdjustment(proposedAdjustment, context)
      if (!adjusted) {
        return { terminal: blocker(area, taskId, phase, 'coordinator returned an invalid technical-adjustment result.', 'lifecycle and commit were not changed.', staleAdvisory) }
      }
      if (adjusted.action === 'stop') {
        return { terminal: blocker(area, taskId, phase, adjusted.reason, 'the proposed record adjustment was not applied.', staleAdvisory) }
      }
      const refreshed = await refresh(`zdev ${taskId}: refresh after technical adjustment`)
      if (typeof refreshed === 'string') return { terminal: refreshed }
      technicalAdjustments.push({ record_paths: adjusted.record_paths, explanation: adjusted.reason })
      context = refreshed
      const retryRaw = (await agent(
        `${workerContract}\n\nResume task ${taskId} in area ${area} after coordination clarified an incidental technical restriction. Load the original baseline with zdev work-context ${area} --show ${prepared.baselineSnapshot} --format json and fresh context with zdev work-context ${area} --show ${refreshed.baselineSnapshot} --format json. Read the updated task and brief, complete the directly necessary work, and run validation. Return the implementer envelope from your role prompt.\n\nAdjustment: ${JSON.stringify(technicalAdjustments.at(-1))}`,
        workerOptions(activeAgentType.endsWith('advanced-implementer') ? 'advanced-implementer'
          : activeAgentType.endsWith('routine-implementer') ? 'routine-implementer' : 'implementer',
        { agentType: activeAgentType, label: `zdev ${taskId}: resume after technical adjustment` }),
      ))?.trim()
      const retry = parseWorkerResult(retryRaw, 'implementer', area, taskId)
      if (!retry) {
        return { terminal: blocker(area, taskId, phase, 'replacement implementer returned an invalid or mismatched envelope.', 'the clarified records remain in the checkout.', staleAdvisory) }
      }
      result = retry
      continue
    }
    const refreshed = await refresh(`zdev ${taskId}: refresh after ${phase} blocker`)
    if (typeof refreshed === 'string') return { terminal: refreshed }
    let madeProgress = null
    if (priorBlockerSnapshot) {
      const comparison = await compareSnapshot(
        priorBlockerSnapshot,
        `zdev ${taskId}: compare ${phase} blocker progress`,
      )
      if (!comparison) {
        return { terminal: blocker(area, taskId, phase, 'could not compare progress after an implementer blocker.', 'lifecycle and commit were not changed.', staleAdvisory) }
      }
      madeProgress = !comparison.equal
    }
    const disposition = await classifyImplementerBlocker(result, refreshed, madeProgress, priorBlockerResult)
    if (!disposition) {
      return { terminal: blocker(area, taskId, phase, 'coordinator returned an invalid blocker classification.', 'lifecycle and commit were not changed.', staleAdvisory) }
    }
    if (disposition.action === 'stop') {
      return { terminal: blocker(area, taskId, phase, disposition.reason, `Evidence: ${result.evidence.join('; ') || 'none.'} Findings: ${result.findings.join('; ') || 'none.'}`, staleAdvisory) }
    }
    priorBlockerSnapshot = refreshed.baselineSnapshot
    priorBlockerResult = result
    context = refreshed
    const retryRaw = (await agent(
      `${workerContract}\n\nContinue task ${taskId} in area ${area} from current snapshot ${refreshed.baselineSnapshot}. Load it with zdev work-context ${area} --show ${refreshed.baselineSnapshot} --format json. The coordinator classified the prior blocker as directly actionable: ${disposition.reason}. Complete the remaining task work, including every directly necessary in-scope path, and run validation. Return the implementer envelope from your role prompt. If a genuine new impasse remains, report it precisely.\n\nPrior worker result: ${JSON.stringify(result)}`,
      workerOptions(activeAgentType.endsWith('advanced-implementer') ? 'advanced-implementer'
        : activeAgentType.endsWith('routine-implementer') ? 'routine-implementer' : 'implementer',
      { agentType: activeAgentType, label: `zdev ${taskId}: continue after ${phase} blocker` }),
    ))?.trim()
    const retry = parseWorkerResult(retryRaw, 'implementer', area, taskId)
    if (!retry) {
      return { terminal: blocker(area, taskId, phase, 'replacement implementer returned an invalid or mismatched envelope.', 'lifecycle and commit were not changed.', staleAdvisory) }
    }
    result = retry
  }
  return { result }
}
const verify = async (priorReview = null) => {
  const storedRaw = (await agent(
    `Act only as read-only verification coordination. Run zdev work-context ${area} --task ${taskId} --store --format json exactly once and return its JSON stdout. Do not show the stored snapshot.`,
    { label: `zdev ${taskId}: capture verification snapshot`, model: 'haiku' },
  ))?.trim()
  const stored = parseStoredContext(storedRaw, area, {
    taskId, head: prepared.head, complexity, taskPath: prepared.taskPath,
  })
  if (!stored) return null
  if (stored.staleAdvisory) staleAdvisory = true
  const currentAdvisory = staleAdvisory ? advisoryText : null
  const current = stored
  const snapshot = stored.baselineSnapshot
  const raw = (await agent(
    `${workerContract}\n\nIndependently verify task ${taskId} in area ${area}. Load the original baseline with zdev work-context ${area} --show ${prepared.baselineSnapshot} --format json and the verification snapshot with zdev work-context ${area} --show ${snapshot} --format json; require task ${taskId} at HEAD ${current.head}. Read any clarified records from the verification snapshot and check their updated requirements while retaining the original baseline for attribution. Use the implementer summary only to locate evidence. ${priorReview ? `Perform the focused recheck under the coding guidance Review reference: check accepted corrections, affected contracts, and repair regressions; retain conclusions for unchanged code. Prior review and accepted corrections: ${JSON.stringify(priorReview)}.` : "Check the whole task against requirements and coding guidance."} Run required validation. Put optional suggestions in summary, never findings. Keep verification read-only: use a check or dry-run form for generators and other commands expected to rewrite tracked files. Return exactly one JSON object with exactly these four keys and no others: verdict, summary, findings, escalation. Pass requires an empty findings array; rework requires at least one finding. Report each unexpected validation-written task-owned file as a validation_write: <repository-relative path> finding with verdict rework. Never add validation_writes or another fifth key. Do not repair or discard validation writes.\n\nTechnical adjustments: ${JSON.stringify(technicalAdjustments)}\nImplementer summary: ${compactWorkerSummary(latestImplementation)}`,
    workerOptions('verifier', { agentType: 'zdev:zdev-verifier', label: `zdev ${taskId}: verify` }),
  ))?.trim()
  let semantic = parseVerifierResult(raw)
  const comparedRaw = (await agent(
    `Act only as deterministic post-verification coordination. Run zdev work-context ${area} --compare ${snapshot} --format json exactly once and return its complete JSON stdout unchanged, with no fence or other text. Keep files and Git state unchanged.`,
    { label: `zdev ${taskId}: confirm verifier left snapshot unchanged`, model: 'haiku' },
  ))?.trim()
  const compared = parseComparison(comparedRaw, area, snapshot)
  if (!semantic || !compared) return null
  if (!compared.equal && !reportsValidationWrite(semantic)) return null
  if (semantic.findings.some(item => item.startsWith(validationWriteMarker)) && !reportsValidationWrite(semantic)) {
    return null
  }
  if (compared.equal && reportsValidationWrite(semantic)) {
    semantic = { ...semantic, findings: semantic.findings.filter(item => !item.startsWith(validationWritePrefix)) }
    if (semantic.findings.length === 0) semantic = { ...semantic, verdict: 'pass', escalation: 'none' }
  }
  if (semantic.verdict === 'rework' && !onlyValidationWrites(semantic)) {
    if (!compared.equal) return null
    const checked = await agent(
      `${workerContract}\n\nCheck the substantive findings in this candidate review of task ${taskId}, independently of their author. Load context with zdev work-context ${area} --show ${snapshot} --format json. Follow Review in the supplied coding guidance. Inspect the cited code, callers, applicable rules, consequences, severity, and counterevidence; do not repeat the whole review or rerun validation. Preserve the original validation conclusions. Return the reconciled four-field semantic verifier object: rework for supported corrections, pass only when all substantive findings are disproved and the original review otherwise established completion, or blocker for unresolved claims or missing evidence. Explain removed or corrected claims in summary. Do not add new findings, validation_write markers, or escalate beyond the candidate. Do not delegate or edit.\n\nCandidate review: ${JSON.stringify(semantic)}`,
      workerOptions('verifier', { agentType: 'zdev:zdev-verifier', label: `zdev ${taskId}: check findings` }),
    )
    const vetted = parseVerifierResult(typeof checked === 'string' ? checked.trim() : checked)
    const unchanged = await compareSnapshot(snapshot, `zdev ${taskId}: confirm findings snapshot unchanged`)
    if (!vetted || !unchanged?.equal || vetted.findings.some(item => item.startsWith(validationWriteMarker))
      || (vetted.escalation !== 'none' && vetted.escalation !== semantic.escalation)) return null
    semantic = vetted
  }
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
const resolvedImplementation = await resolveImplementerResult(implementation, prepared, 'implementation')
if (resolvedImplementation.terminal) return resolvedImplementation.terminal
latestImplementation = resolvedImplementation.result
let current = null
let verdict = await verify()
if (!verdict) {
  return blocker(area, taskId, 'verification', 'verifier returned an invalid or mismatched envelope.', 'lifecycle and commit were not changed.', staleAdvisory)
}
if (verdict.result.verdict === 'rework') {
  const priorReview = verdict.result
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
    `${workerContract}\n\nCorrect every concrete task-owned finding for ${taskId}. Load the original baseline with zdev work-context ${area} --show ${prepared.baselineSnapshot} --format json and require current HEAD ${current.head}. Return the implementer envelope from your role prompt. When smaller tasks would help implementation or verification, load ${JSON.stringify(taskWorkflowContractPath)} and use its typed implementation_split blocker.\n\nVerifier findings:\n${verdict.raw}`,
    workerOptions(activeAgentType.endsWith('advanced-implementer') ? 'advanced-implementer'
      : activeAgentType.endsWith('routine-implementer') ? 'routine-implementer' : 'implementer',
    { agentType: activeAgentType, label: `zdev ${taskId}: ${escalated ? 'advanced ' : ''}rework` }),
  ))?.trim()
  const rework = parseWorkerResult(reworkRaw, 'implementer', area, taskId)
  if (!rework) {
    return blocker(area, taskId, 'rework', 'implementer returned an invalid or mismatched envelope.', 'lifecycle and commit were not changed.', staleAdvisory)
  }
  const resolvedRework = await resolveImplementerResult(rework, current, 'rework')
  if (resolvedRework.terminal) return resolvedRework.terminal
  latestImplementation = resolvedRework.result
  verdict = await verify(priorReview)
  if (!verdict) {
    return blocker(area, taskId, 'verification', 'verifier returned an invalid or mismatched envelope.', 'lifecycle and commit were not changed.', staleAdvisory)
  }
}
if (verdict.result.verdict === 'rework') {
  return blocker(area, taskId, 'focused recheck', 'required corrections remain after one correction pass; explicit user continuation is needed.', `Review stage: correction allowance exhausted. Work preserved; task remains open. ${verdict.result.summary} Findings: ${verdict.result.findings.join('; ')}`, staleAdvisory)
}
if (verdict.result.verdict !== 'pass') {
  return blocker(area, taskId, 'verification', verdict.result.summary, `Evidence: ${verdict.result.evidence.join('; ') || 'none.'} Findings: ${verdict.result.findings.join('; ') || 'none.'}`, staleAdvisory)
}

const advisory = staleAdvisory ? advisoryText : null
const completed = await agent(
  `${repositoryGuidance}\n\nAct as the existing completion coordinator for verified task ${taskId} in area ${area}. Whether this completion is live or resumed, before mutation run exactly one zdev work-context ${area} --compare ${verdict.approved} --format json. Accept the exact four-key JSON object {"schema_version":1,"area":"${area}","snapshot":"${verdict.approved}","equal":true}. On an exact match, run zdev task done, stage the attributed task-owned paths, every clarified record path in the handoff, and exact task records, inspect the cached diff, and run zdev commit. Preserve the task-done and index state if staging, cached-diff inspection, or commit needs recovery. Return PASS zdev-implement ${area} ${taskId} or BLOCKER zdev-implement ${area} ${taskId} as the exact first line. Repeat exact Area: ${area} and Task: ${taskId} fields. ${advisory ? `Include Advisory: ${advisory} exactly once, ` : 'Omit Advisory, '}plus Summary, Changed files, Validation, Verifier evidence, and Commit ID on pass, or Failed stage, Reason, and Preserved state on blocker.\n\nCompletion handoff: ${JSON.stringify({ snapshot: verdict.approved, implementation: latestImplementation.summary, verification: verdict.result.summary, technical_adjustments: technicalAdjustments })}`,
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

}

const repositoryGuidance = "<!-- zdev:generated-repository-guidance:start -->\n## Repository guidance discovery\n\nBefore inspecting, planning, changing, or validating code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.\n<!-- zdev:generated-repository-guidance:end -->\n\n## Coding guidance\n\nCoding guidance file: \"/Users/zayenz/projects/zdev/.claude/skills/zdev/skills/zdev/references/coding.md\"\n\nBefore planning code, read Design and Code standards from this file. Before implementing or repairing code, also read Completion. Before verification, audit, or code review, read Code standards and Review. Pass this exact location to each relevant worker; read it directly even when the root skill is not loaded. Routine coordination and non-code tasks do not need it."
const taskWorkflowContractPath = "/Users/zayenz/projects/zdev/.claude/skills/zdev/contracts/task-workflows.md"
const researchReferencePath = "/Users/zayenz/projects/zdev/.claude/skills/zdev/skills/zdev/references/research.md"
const codingReferencePath = "/Users/zayenz/projects/zdev/.claude/skills/zdev/skills/zdev/references/coding.md"
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
const hasDuplicateObjectKeys = raw => {
  if (typeof raw !== 'string') return false
  const stack = []
  for (let index = 0; index < raw.length; index += 1) {
    const c = raw[index]
    if (c === '"') {
      const start = index++
      while (index < raw.length && raw[index] !== '"') {
        if (raw[index] === '\\') index += 1
        index += 1
      }
      if (index >= raw.length) return true
      let next = index + 1
      while (/\s/.test(raw[next] ?? '')) next += 1
      const current = stack.at(-1)
      if (raw[next] === ':' && current instanceof Set) {
        let key
        try { key = JSON.parse(raw.slice(start, index + 1)) } catch { return true }
        if (current.has(key)) return true
        current.add(key)
      }
    } else if (c === '{') stack.push(new Set())
    else if (c === '[') stack.push(null)
    else if (c === '}' || c === ']') stack.pop()
  }
  return false
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
    || typeof stored.stale_advisory !== 'boolean'
    || typeof stored.task_path !== 'string'
    || !stored.task_path.startsWith(`.zdev/${expectedArea}/tasks/`)
    || !stored.task_path.endsWith('.md')) return null
  if (expected && (stored.task_id !== expected.taskId || stored.head !== expected.head
    || stored.complexity !== expected.complexity
    || (expected.taskPath && stored.task_path !== expected.taskPath))) return null
  return { lifecycle: stored.lifecycle, queue: stored.queue, taskId: stored.task_id,
    complexity: stored.complexity, staleAdvisory: stored.stale_advisory, head: stored.head,
    baselineSnapshot: stored.snapshot, taskPath: stored.task_path,
    hypothesis: stored.hypothesis }
}
const parseDispatchSpec = (raw, expectedRoute, expectedTask, expectedSnapshot) => {
  const decoded = decodeJsonObject(raw)
  const value = decoded?.value
  const roles = ['routine-implementer', 'implementer', 'advanced-implementer', 'planner', 'verifier']
  const exact = (object, keys) => object && !Array.isArray(object) && typeof object === 'object'
    && JSON.stringify(Object.keys(object).sort()) === JSON.stringify([...keys].sort())
  const validValue = value => (exact(value, ['inherit']) && value.inherit === true)
    || (exact(value, ['model', 'effort'])
      && typeof value.model === 'string' && value.model.trim() === value.model && value.model
      && typeof value.effort === 'string' && ['inherit', 'low', 'medium', 'high', 'xhigh', 'max'].includes(value.effort))
  const validRole = (record, role) => exact(record,
    ['schema_version', 'profile', 'harness', 'role', 'value', 'origin', 'fallback'])
    && record.schema_version === 1 && record.harness === 'claude' && record.role === role
    && typeof record.profile === 'string' && record.profile.trim() === record.profile && record.profile
    && validValue(record.value)
  const scanned = decoded && scanTopLevelObject(decoded.raw)
  if (!value || hasDuplicateObjectKeys(decoded.raw) || !scanned || new Set(scanned.keys).size !== scanned.keys.length
    || !exact(value, ['schema_version', 'kind', 'harness', 'route', 'area', 'task_id', 'snapshot', 'dispatches', 'profiles', 'stop'])
    || value.schema_version !== 1 || value.kind !== 'dispatch-spec'
    || value.harness !== 'claude' || value.route !== expectedRoute
    || value.area !== area || value.task_id !== expectedTask || value.snapshot !== expectedSnapshot
    || !exact(value.profiles, roles) || !roles.every(role => validRole(value.profiles[role], role))
    || !Array.isArray(value.dispatches)) return null
  const dispatchKeys = ['role', 'profile', 'model', 'reasoning_effort', 'task_id', 'snapshot', 'next_phase']
  if (!value.dispatches.every(item => exact(item, dispatchKeys)
    && roles.includes(item.role) && typeof item.profile === 'string' && item.profile
    && (item.model === null || typeof item.model === 'string' && item.model)
    && (item.reasoning_effort === null || ['inherit', 'low', 'medium', 'high', 'xhigh', 'max'].includes(item.reasoning_effort))
    && item.task_id === expectedTask && item.snapshot === expectedSnapshot
    && typeof item.next_phase === 'string' && item.next_phase)) return null
  if (expectedRoute === 'plan-next-task'
    && (value.stop !== 'plan-only' || value.dispatches.length !== 1
      || value.dispatches[0].role !== 'planner' || value.dispatches[0].next_phase !== 'plan-only-stop'
      || value.dispatches[0].profile !== value.profiles.planner.profile
      || value.dispatches[0].model !== (value.profiles.planner.value.model ?? null)
      || value.dispatches[0].reasoning_effort !== (value.profiles.planner.value.effort ?? null))) return null
  if (expectedRoute === 'implement' && value.stop !== 'completion') return null
  return value
}
const nativeWorkerOptions = (spec, role, extra = {}) => {
  const resolved = spec.profiles[role]
  const value = resolved?.value
  if (!value || value.inherit === true) return { ...extra }
  if (typeof value.model !== 'string' || typeof value.effort !== 'string') return null
  return value.effort === 'inherit' ? { ...extra, model: value.model }
    : { ...extra, model: value.model, effort: value.effort }
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
const blockerDispositionSchema = {
  type: 'object',
  additionalProperties: false,
  required: ['action', 'reason'],
  properties: {
    action: { type: 'string', enum: ['continue', 'stop'] },
    reason: { type: 'string', minLength: 1 },
  },
}
const technicalAdjustmentSchema = {
  type: 'object', additionalProperties: false,
  required: ['action', 'reason', 'record_paths'],
  properties: {
    action: { type: 'string', enum: ['adjusted', 'stop'] },
    reason: { type: 'string', minLength: 1 },
    record_paths: { type: 'array', items: { type: 'string', minLength: 1 } },
  },
}
const parseBlockerDisposition = raw => {
  const decoded = decodeJsonObject(raw)
  const result = decoded?.value
  return result && !Array.isArray(result) && typeof result === 'object'
    && JSON.stringify(Object.keys(result).sort()) === JSON.stringify(['action', 'reason'])
    && ['continue', 'stop'].includes(result.action)
    && typeof result.reason === 'string' && result.reason.trim()
    ? result : null
}
const parseTechnicalAdjustmentResult = raw => {
  const result = decodeJsonObject(raw)?.value
  if (!result || Array.isArray(result) || typeof result !== 'object'
    || JSON.stringify(Object.keys(result).sort()) !== JSON.stringify(['action', 'reason', 'record_paths'])
    || !['adjusted', 'stop'].includes(result.action)
    || typeof result.reason !== 'string' || !result.reason.trim()
    || !Array.isArray(result.record_paths)
    || !result.record_paths.every(path => typeof path === 'string' && path.trim())) return null
  if (result.action === 'adjusted' && result.record_paths.length === 0) return null
  if (result.action === 'stop' && result.record_paths.length !== 0) return null
  return result
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
const onlyValidationWrites = result => reportsValidationWrite(result)
  && result.findings.every(item => item.startsWith(validationWritePrefix))
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


const roles = ['routine-implementer', 'implementer', 'advanced-implementer', 'planner', 'verifier']
const exactObject = (value, keys) => value && !Array.isArray(value) && typeof value === 'object'
  && JSON.stringify(Object.keys(value).sort()) === JSON.stringify([...keys].sort())
const validText = value => typeof value === 'string' && value.trim() === value && value.length > 0
const json = raw => {
  const decoded = decodeJsonObject(raw)
  return decoded && !hasDuplicateObjectKeys(decoded.raw) ? decoded.value : null
}
const validRoleRecord = (record, role) => exactObject(record,
  ['schema_version', 'profile', 'harness', 'role', 'value', 'origin', 'fallback'])
  && record.schema_version === 1 && record.harness === 'claude' && record.role === role
  && validText(record.profile) && ((exactObject(record.value, ['inherit']) && record.value.inherit === true)
    || (exactObject(record.value, ['model', 'effort']) && validText(record.value.model)
      && ['inherit', 'low', 'medium', 'high', 'xhigh', 'max'].includes(record.value.effort)))
const validProfiles = profiles => exactObject(profiles, roles)
  && roles.every(role => validRoleRecord(profiles[role], role))
const input = args && !Array.isArray(args) && typeof args === 'object' ? args : null
const area = input?.area
const invocation = input?.invocation_id
const mode = input?.authorization
const execution = input?.execution ?? 'serial'
const capacities = ['worker_limit', 'measurement_limit']
const limits = ['attempt_limit', 'command_limit', 'command_seconds', 'time_limit_seconds', 'target']
let nativeCalls = 0
let reporting = false
// Claude documents 1,000 agents per workflow. Keep room for draining and live-state reporting.
const callAgent = (prompt, options) => {
  if (nativeCalls >= (reporting ? 1_000 : 980)) throw new Error('Workflow call boundary reached; preserve effects and resume in a fresh invocation')
  nativeCalls++
  return agent(prompt, options)
}
let root = null
let runId = null
let startAttempted = false
let profiles = input?.frozen_profiles ?? null
const retainedPlans = input?.retained_plans ?? {}
const retainedCandidates = input?.retained_candidates ?? {}
let context = null
let requestNumber = 0
let pendingRecoveryVerification = null
const completed = []
const recovery = []
const nextRequest = kind => `${invocation}-${kind}-${++requestNumber}`
const report = (verdict, reason) => JSON.stringify({
  schema_version: 1, kind: 'research', area: area ?? 'unknown', run_id: runId,
  scope: context?.run?.scope ?? input?.scope ?? null,
  completed: [...completed, ...(context?.checkpoints ?? [])
    .filter(checkpoint => checkpoint.index_updated && !completed.some(item => item.task_id === checkpoint.task))
    .map(checkpoint => ({ task_id: checkpoint.task, decision: checkpoint.final_record.decision, commit: checkpoint.commit }))],
  accepted_baseline: context?.accepted_baseline ?? null,
  remaining: context?.remaining ?? null, stop_reason: reason,
  execution, cleanup: input?.cleanup === true, native_calls: nativeCalls, recovery_actions: recovery, frozen_profiles: profiles, retained_plans: retainedPlans, retained_candidates: retainedCandidates, verdict,
})
if (!input || !/^[a-z0-9][a-z0-9-]*$/.test(area ?? '')
  || !/^[A-Za-z0-9][A-Za-z0-9_-]*$/.test(invocation ?? '')
  || !['start', 'resume'].includes(mode)
  || !['serial', 'rolling'].includes(execution)
  || (Object.hasOwn(input, 'cleanup') && typeof input.cleanup !== 'boolean')
  || capacities.some(key => Object.hasOwn(input, key) && (!Number.isInteger(input[key]) || input[key] <= 0))
  || (mode === 'start' && !validText(input.scope))
  || (mode === 'resume' && !validText(input.run_id))
  || (Object.hasOwn(input, 'until_interrupted') && typeof input.until_interrupted !== 'boolean')
  || limits.some(key => Object.hasOwn(input, key)
    && (typeof input[key] !== 'number' || !Number.isFinite(input[key])
      || (key !== 'target' && input[key] <= 0)
      || (['attempt_limit', 'command_limit'].includes(key) && !Number.isInteger(input[key]))))
  || (mode === 'resume' && ['scope', 'until_interrupted', ...limits, ...capacities].some(key => Object.hasOwn(input, key)))
  || (input.run_profile && !/^[A-Za-z0-9][A-Za-z0-9_-]*$/.test(input.run_profile))
  || (input.role_profiles && (!exactObject(input.role_profiles, Object.keys(input.role_profiles))
    || Object.entries(input.role_profiles).some(([role, profile]) => !roles.includes(role)
      || !/^[A-Za-z0-9][A-Za-z0-9_-]*$/.test(profile))))
  || (profiles !== null && !validProfiles(profiles))
  || !retainedPlans || Array.isArray(retainedPlans) || typeof retainedPlans !== 'object'
  || !retainedCandidates || Array.isArray(retainedCandidates) || typeof retainedCandidates !== 'object'
  || Object.entries(retainedCandidates).some(([task, handoff]) => !task.startsWith(`${area}-`)
    || !exactObject(handoff, ['findings', 'escalated', 'needs_repair']) || !Array.isArray(handoff.findings)
    || !handoff.findings.every(validText) || typeof handoff.escalated !== 'boolean' || typeof handoff.needs_repair !== 'boolean')
  || Object.entries(retainedPlans).some(([task, plan]) => !task.startsWith(`${area}-`)
    || validateSemanticPlannerResult(plan)?.verdict !== 'plan')) {
  recovery.push('Supply structured area, authorization start/resume, fresh invocation_id, scope for start or run_id for resume, and valid limits/profiles.')
  return report('blocker', 'input')
}

const control = async (label, prompt, schema = null) => {
  const raw = await callAgent(`${repositoryGuidance}\n\n${root ? `Use absolute repository cwd ${JSON.stringify(root)} for every tool. ` : ''}Read installed research reference ${JSON.stringify(researchReferencePath)} and coding guidance ${JSON.stringify(codingReferencePath)}. Task contract: ${JSON.stringify(taskWorkflowContractPath)}. ${prompt}`,
    { label: `zdev research ${area}: ${label}`, model: 'haiku', ...(schema ? { schema } : {}) })
  const value = json(raw)
  if (!value || value.error) throw new Error(value?.error ?? `${label} returned invalid JSON; inspect effects before retrying`)
  return value
}
const validRunContext = value => value?.schema_version === 1 && value.area === area
  && validText(value.run?.id) && validText(value.run.scope)
  && typeof value.run.comparison_valid === 'boolean' && typeof value.run.ended === 'boolean'
  && value.run.reservations && value.run.admissions
  && !Array.isArray(value.run.reservations) && !Array.isArray(value.run.admissions)
  && typeof value.run.reservations === 'object' && typeof value.run.admissions === 'object'
  && [null, 'target', 'budget', 'interruption', 'blocker', 'ended'].includes(value.stop_reason)
  && Array.isArray(value.checkpoints) && Array.isArray(value.tasks?.tasks)
  && value.accepted_baseline && value.consumed && value.remaining
const command = async (argv, label) => control(label,
  `Act only as command mediation. Execute the following argument array exactly once, preserving argument boundaries, then return its complete JSON stdout unchanged. On failure return only {"error":"concrete command failure and preserved effects"}. Do not launch workers, retry commands or invent successful state. Research command: ${JSON.stringify(['zdev', ...argv, '--format', 'json'])}`)
const runCommand = (action, flags = [], label = action) => command(
  ['research', 'run', area, action, '--run', runId, ...flags], label)
const refresh = async () => {
  const value = await command(['research', 'run', area, 'context'], 'fresh run context')
  if (!validRunContext(value) || (runId && value.run.id !== runId)) throw new Error('Fresh run context did not preserve the authorized identity')
  context = value
  return value
}
const reserve = async (task, kind) => {
  const request = nextRequest(kind)
  const value = await runCommand('reserve', ['--request', request, '--task', task, '--kind', kind, ...(execution === 'rolling' ? ['--worker-slot'] : [])], `reserve ${kind} ${task}`)
  const reservation = value.run?.reservations?.[request]
  // The last attempt authorizes this dispatch even though the returned context now says budget.
  if (!validRunContext(value) || value.run.id !== runId || !reservation
    || reservation.task !== task || reservation.kind !== kind || reservation.settled || reservation.bound
    || (execution === 'rolling' && (value.dispatch_authorized !== true || !reservation.worker || !reservation.requested_worker_slot))) {
    throw new Error('Reservation did not authorize the requested fresh dispatch')
  }
  context = value
  return request
}
const dispatch = async (task, kind, role, prompt, extra = {}) => {
  const request = await reserve(task, kind)
  const options = nativeWorkerOptions({ profiles }, role,
    { agentType: `zdev:zdev-${role}`, label: `zdev research ${task}: ${role}`, ...extra })
  const raw = await callAgent(prompt, options)
  if (role === 'planner') {
    const plan = parsePlannerResult(raw)
    // Preserve completed planning even when settlement's outcome needs recovery.
    if (plan?.verdict === 'plan' && reconstructPlannerResult(plan, area, task)) retainedPlans[task] = plan
  }
  if (execution === 'rolling') {
    const valid = role === 'planner' ? parsePlannerResult(raw)
      : role === 'verifier' ? parseVerifierResult(raw) : parseWorkerResult(raw, 'implementer', area, task)
    if (!valid) throw new Error(`${role} returned invalid or interrupted evidence; preserve its worker claim`)
    await checkedControl('confirm standalone role stopped', `Confirm actual role ${role} for ${task}, request ${request}, has stopped, including owned subprocesses. A returned Promise alone is insufficient. Preserve an ambiguous claim. Result: ${JSON.stringify(valid)}`)
  }
  await runCommand('settle', ['--request', request], `settle ${request}`)
  return raw
}
const workerPrompt = (selected, action) => `${repositoryGuidance}\n\nUse absolute repository cwd ${JSON.stringify(root)} for every file and command tool. Read task contract ${JSON.stringify(taskWorkflowContractPath)}, coding guidance ${JSON.stringify(codingReferencePath)}, and research reference ${JSON.stringify(researchReferencePath)}. Read .zdev/${area}/brief.md first, complete task ${selected.task_id} and applicable repository guidance. Load zdev work-context ${area} --show ${selected.context.snapshot} --format json. ${action} Never edit .zdev, stage, finalize, complete a task or commit. Return only your strict role result.`
const select = async (unfinishedTask = null, excluded = []) => {
  const selected = await control('select ready work',
    `Read fresh work-context and tasks list/show. Exclude these already assigned or unfinished tasks: ${JSON.stringify(excluded)}. ${unfinishedTask ? `Select only unfinished checkpoint task ${unfinishedTask} for recording/restoration, not new dispatch.` : 'Select from the entire ready frontier using ordinary eligibility and dependencies, preferring afk true. Under shared blocker choose only supporting work relevant to the recorded blocker; otherwise return task_id null when no eligible work remains.'} Read all prior rejected, inconclusive and failed-to-run lessons. For a selected task store explicit work-context --task <task> --store. Read full task/policy and declare exact clean candidate files before any hypothesis edit, including named new files; exclude managed records, directories, symlinks and unrelated changes. Return exactly {"task_id":string|null,"kind":"supporting"|"hypothesis"|null,"complexity":"routine"|"standard"|"advanced"|null,"context":compact-work-context|null,"paths":string[],"reason":string}. Keep files and Git unchanged. Fresh run data: ${JSON.stringify(context)}`)
  if (!exactObject(selected, ['task_id', 'kind', 'complexity', 'context', 'paths', 'reason'])
    || !validText(selected.reason) || !Array.isArray(selected.paths)) throw new Error('Invalid ready-work selection')
  if (selected.task_id === null) {
    if (selected.kind !== null || selected.complexity !== null || selected.context !== null || selected.paths.length) throw new Error('Ambiguous empty selection')
    return selected
  }
  const stored = parseStoredContext(selected.context, area)
  if (!stored || stored.taskId !== selected.task_id || stored.lifecycle !== 'open' || stored.queue !== 'ready'
    || selected.complexity !== stored.complexity || !['supporting', 'hypothesis'].includes(selected.kind)
    || stored.hypothesis !== (selected.kind === 'hypothesis')
    || (unfinishedTask && selected.task_id !== unfinishedTask)
    || excluded.includes(selected.task_id)
    || (!unfinishedTask && context.stop_reason === 'blocker' && selected.kind !== 'supporting')
    || (selected.kind === 'hypothesis' && (!selected.paths.length
      || new Set(selected.paths).size !== selected.paths.length
      || selected.paths.some(path => !normalizedRepositoryPath(path) || path.startsWith('/')
        || path === '.zdev' || path.startsWith('.zdev/'))))) throw new Error('Selection did not admit safe explicit task and declared files')
  return selected
}
const checkedControl = async (label, prompt) => {
  const result = await control(label, `${prompt} Return exactly {"safe":boolean,"reason":string}.`)
  if (!exactObject(result, ['safe', 'reason']) || result.safe !== true || !validText(result.reason)) {
    throw new Error(result.reason ?? `${label} could not establish safe continuation`)
  }
}
const recoverEffects = async (allowDispatch = true) => {
  const recovered = await control('recover pending effects',
    `Recover live durable effects before selection. For rolling execution confirm actual native workers and evaluator processes stopped before settling claims or Return --worker-stopped. A null, thrown, invalid or missing role response does not establish stop. Preserve unresolved capacity and sources; never redispatch a checkpoint through recovery. Inspect admissions, attributable uncommitted records, outstanding reservations, rebaseline requests and experiments. Retry only identical admission content under its original request; finish exact publication/staging/cached-diff/commit checkpoints through zdev commit. Inspect/recover experiments and interrupted rebaseline using the original request; never relaunch unknown measurements. Publish/commit an actual rebaseline's exact area metadata before new work. Reconcile reservations from actual worker effects without refunds or duplicate dispatch. Resume an already verified ordinary completion with its retained verification evidence and existing completion coordinator. When that evidence is unavailable but original baseline and ownership are established, safely reopen an uncommitted done ordinary task with zdev task reopen, store explicit ready work-context, and return reverify {"task_id":string,"context":compact-work-context,"baseline_snapshot":original-snapshot,"implementation":nonempty-summary}; otherwise preserve ambiguous state and report its concrete recovery action. Return exactly {"safe":boolean,"reason":string,"reverify":object|null}. Do not launch workers in this helper. Pending candidate code must be restored through authorized lifecycle before accepted supporting edits. For stopped isolated reconciliation conflicts, preserve the source and establish exact paths/base for a newly counted repair; do not launch or perform that repair in recovery. Preserve unresolved measurement/process ownership. Context: ${JSON.stringify(context)}`)
  if (!exactObject(recovered, ['safe', 'reason', 'reverify']) || recovered.safe !== true || !validText(recovered.reason)) throw new Error(recovered.reason ?? 'Unresolved recovery effects')
  pendingRecoveryVerification = recovered.reverify ?? pendingRecoveryVerification
  if (pendingRecoveryVerification !== null && allowDispatch) {
    const handoff = pendingRecoveryVerification
    const stored = parseStoredContext(handoff?.context, area)
    if (!exactObject(handoff, ['task_id', 'context', 'baseline_snapshot', 'implementation'])
      || !stored || stored.taskId !== handoff.task_id || stored.hypothesis !== false
      || !/^W[0-9a-f]{16}$/.test(handoff.baseline_snapshot ?? '') || !validText(handoff.implementation)) throw new Error('Invalid supporting completion recovery handoff')
    const selected = { task_id: handoff.task_id, context: handoff.context, complexity: stored.complexity }
    const raw = await dispatch(handoff.task_id, 'supporting', 'verifier', workerPrompt(selected,
      `Independently verify this interrupted ordinary completion against original baseline zdev work-context ${area} --show ${handoff.baseline_snapshot} --format json and supplied snapshot ${stored.baselineSnapshot}. Check the whole task and coding Review/Code standards. Implementation summary: ${handoff.implementation}. Return exactly verdict, summary, findings, escalation.`))
    const semantic = parseVerifierResult(raw)
    const comparison = parseComparison(await command(['work-context', area, '--compare', stored.baselineSnapshot], 'compare recovered completion'), area, stored.baselineSnapshot)
    if (!semantic || !comparison?.equal || semantic.verdict === 'blocker') throw new Error('Recovered completion lacks valid independent verification and stable snapshot')
    if (semantic.verdict === 'rework') {
      if (execution === 'rolling') await checkRollingFindings(selected, semantic, 'supporting', stored.baselineSnapshot)
      else await checkedControl('check recovered completion findings', `Independently check substantive task-owned findings against snapshot ${stored.baselineSnapshot}: ${JSON.stringify(semantic.findings)}`)
      if (stored.complexity === 'advanced' && !retainedPlans[handoff.task_id]) throw new Error('Advanced completion rework requires the retained original semantic plan')
      await support(selected)
    } else {
      const result = await callAgent(`${repositoryGuidance}\n\nUse absolute cwd ${JSON.stringify(root)}. Act as the existing ordinary completion coordinator for ${handoff.task_id}. Compare zdev work-context ${area} --compare ${stored.baselineSnapshot} --format json and require exact equal true before mutation. Read ${JSON.stringify(taskWorkflowContractPath)}. Run task done, stage exact attributed source paths and exact task/TASKS.md records, inspect full cached diff and zdev commit. Preserve unfinished checkpoint on failure. Return the standard PASS/BLOCKER zdev-implement ${area} ${handoff.task_id} result and required fields. Independent verification: ${JSON.stringify(semantic)}. Original baseline: ${handoff.baseline_snapshot}.`,
        { label: `zdev ${handoff.task_id}: complete and commit` })
      recordSupportingCompletion(result, handoff.task_id)
    }
    pendingRecoveryVerification = null
  }
  await refresh()
}
const publish = async admitted => checkedControl('publish admitted tasks',
  `Inspect allocated tasks in this admission and its exact delta. Stage only admitted task files and .zdev/${area}/TASKS.md, inspect full cached diff and Git status, then use zdev commit. Keep source, policy and unrelated records unchanged. Retain any interrupted publication checkpoint. Admission: ${JSON.stringify(admitted)}`)
const generate = async () => {
  const proposal = await control('propose scoped work',
    `Propose a small nonempty mixed TaskBundle as data. Read brief, accepted baseline, task evidence and prior rejected/inconclusive/failed-to-run lessons. Respect original scope, permitted changes, graph links and requires_code (only verified retained acceptance establishes code). Under a shared blocker propose only relevant ordinary repairs. Return exactly {"bundle":TaskBundle,"reason":string}. Do not admit, edit, stage or commit. Fresh authorized context: ${JSON.stringify(context)}`)
  if (!exactObject(proposal, ['bundle', 'reason']) || !validText(proposal.reason)
    || proposal.bundle?.schema_version !== 1 || proposal.bundle.area !== area
    || !Array.isArray(proposal.bundle.tasks) || !proposal.bundle.tasks.length) throw new Error('Proposal was empty, malformed or outside the selected area')
  const request = nextRequest('generation')
  const admitted = await control('admit scoped proposal',
    `Check the exact proposal against brief, scope, graph and requires_code. Write only a temporary bundle file outside tracked paths, then run zdev research run ${area} admit --run ${runId} --request ${request} --from <that-file> --format json once. Return complete JSON stdout or {"error":"concrete failure"}. No worker or code edits. Proposal: ${JSON.stringify(proposal.bundle)}`)
  if (!validRunContext(admitted) || admitted.run.id !== runId || !admitted.run.admissions?.[request]?.tasks?.length) throw new Error('Admission did not allocate tasks')
  await publish(admitted)
}
const recordSupportingCompletion = (result, task) => {
  const pass = typeof result === 'string' && fromExactLine(result, `PASS zdev-implement ${area} ${task}`)
  const commit = pass && field(pass, 'Commit ID')?.replace(/^`|`$/g, '')
  if (!pass || field(pass, 'Area') !== area || field(pass, 'Task') !== task
    || !['Summary', 'Changed files', 'Validation', 'Verifier evidence'].every(name => field(pass, name))
    || !/^[0-9a-f]{40}$/.test(commit ?? '')) throw new Error(typeof result === 'string' ? result : 'Supporting workflow returned no valid completion')
  completed.push({ task_id: task,
    decision: field(pass, 'Derived proposal') === 'implementation_split' ? 'implementation_split' : 'supporting', commit })
}
const support = async selected => {
  const retained = retainedPlans[selected.task_id]
  const result = await runOneTask({ area, task_id: selected.task_id, frozen_profiles: profiles,
    ...(retained ? { retained_plan: 'applicable', retained_plan_result: retained } : {}) }, async (prompt, options) => {
    if (options?.agentType) {
      const role = options.agentType.replace('zdev:zdev-', '')
      if (!roles.includes(role)) throw new Error('Unknown supporting role')
      const raw = await dispatch(selected.task_id, 'supporting', role,
        `${workerPrompt(selected, 'Follow the ordinary task contract and this exact dispatch.')}\n\n${prompt}`, options)
      if (role === 'planner') {
        const plan = parsePlannerResult(raw)
        if (plan?.verdict === 'plan') retainedPlans[selected.task_id] = plan
      }
      return raw
    }
    return callAgent(`${root ? `Use absolute repository cwd ${JSON.stringify(root)} for every tool.\n` : ''}${prompt}`, options)
  })
  recordSupportingCompletion(result, selected.task_id)
}
const experiment = (task, action, flags = []) => command(['research', 'experiment', area, task, action, ...flags], `${task}: ${action}`)
const inspectExperiment = (value, task) => {
  if (value?.schema_version !== 1 || value.area !== area || value.task !== task
    || !value.checkpoint || !Array.isArray(value.checkpoint.attempts)
    || typeof value.comparison_valid !== 'boolean') throw new Error('Invalid experiment checkpoint')
  return value
}
const hypothesis = async selected => {
  const task = selected.task_id
  let measured = context.checkpoints.find(checkpoint => checkpoint.task === task && !checkpoint.index_updated)
  let plan = retainedPlans[task] ?? null
  let evaluationReservation = null
  if (measured) {
    measured = inspectExperiment(await experiment(task, 'inspect'), task)
    if (!measured.checkpoint.attempts.length) {
      if (selected.complexity === 'advanced' && !plan) throw new Error('Interrupted advanced preparation requires its retained semantic plan; preserve checkpoint without replanning')
      evaluationReservation = await reserve(task, 'repair')
    }
  } else {
    if (selected.complexity === 'advanced' && !plan) {
      const raw = await dispatch(task, 'preparation', 'planner', workerPrompt(selected,
        'Plan advanced hypothesis without edits. Return the strict four-field semantic planner object.'), { schema: plannerSchema })
      plan = parsePlannerResult(raw)
      if (!plan || !reconstructPlannerResult(plan, area, task) || plan.verdict !== 'plan') throw new Error('Advanced hypothesis planning did not return a valid plan')
    }
    const preparation = await reserve(task, 'preparation')
    measured = inspectExperiment(await experiment(task, 'begin', [
      ...selected.paths.flatMap(path => ['--path', path]), '--reservation', preparation,
    ]), task)
  }
  if (!measured.checkpoint.attempts.length) {
    const admitted = await command(['work-context', area, '--task', task, '--store'], `${task}: refresh hypothesis admission`)
    const current = parseStoredContext(admitted, area)
    if (!current || current.taskId !== task || current.hypothesis !== true
      || current.head !== measured.checkpoint.head || current.complexity !== selected.complexity
      || JSON.stringify(measured.checkpoint.paths) !== JSON.stringify(selected.paths)) throw new Error('Begin did not preserve explicit hypothesis admission and ownership')
    const role = selected.complexity === 'routine' ? 'routine-implementer'
      : selected.complexity === 'advanced' ? 'advanced-implementer' : 'implementer'
    // Begin has already charged this preparation; a second reserve would double-charge it.
    let raw = await callAgent(workerPrompt({ ...selected, context: admitted },
      `Implement only these declared files: ${JSON.stringify(selected.paths)}. Report another necessary file without editing it. ${plan ? `Follow this validated semantic plan unchanged: ${JSON.stringify(plan)}.` : ''}`),
      nativeWorkerOptions({ profiles }, role, { agentType: `zdev:zdev-${role}`, label: `zdev research ${task}: implement hypothesis` }))
    let implementation = parseWorkerResult(raw, 'implementer', area, task)
    let priorBlocker = null
    while (implementation?.verdict === 'blocker') {
      const disposition = parseBlockerDisposition(await control('classify hypothesis implementation blocker',
        `Inspect actual candidate delta against declared ownership. Return exactly {"action":"continue"|"stop","reason":string}. Continue only directly actionable work within declared files and original task/policy; another necessary file requires lifecycle resolution before a revised hypothesis. Stop for unsafe state, external/user-owned decisions or the same unresolved obstacle without new actionable evidence. No edits, workers or records. Prior blocker: ${JSON.stringify(priorBlocker)}. Current blocker: ${JSON.stringify(implementation)}. Checkpoint: ${JSON.stringify(measured)}`))
      if (!disposition || disposition.action !== 'continue') throw new Error(disposition?.reason ?? 'Invalid implementation blocker disposition')
      priorBlocker = implementation
      if (evaluationReservation) await runCommand('settle', ['--request', evaluationReservation])
      evaluationReservation = await reserve(task, 'repair')
      const refreshed = await command(['work-context', area, '--task', task, '--store'], `${task}: refresh replacement admission`)
      const replacement = parseStoredContext(refreshed, area)
      if (!replacement || replacement.taskId !== task || replacement.hypothesis !== true
        || replacement.head !== measured.checkpoint.head || replacement.complexity !== selected.complexity) throw new Error('Replacement lost explicit hypothesis admission')
      raw = await callAgent(workerPrompt({ ...selected, context: refreshed },
        `Finish directly actionable work only in declared files ${JSON.stringify(selected.paths)}. Prior blocker: ${JSON.stringify(implementation)}. ${plan ? `Follow the unchanged semantic plan: ${JSON.stringify(plan)}.` : ''}`),
        nativeWorkerOptions({ profiles }, role, { agentType: `zdev:zdev-${role}`, label: `zdev research ${task}: replacement` }))
      implementation = parseWorkerResult(raw, 'implementer', area, task)
    }
    if (!implementation || implementation.verdict !== 'ready') throw new Error('Hypothesis implementer did not return a valid ready result; preserve effects before replacement')
    await checkedControl('inspect candidate delta',
      `Inspect the whole staged/unstaged/untracked delta against checkpoint baseline. Require edits only to declared exact candidate files with unchanged records, HEAD, policy and unrelated work. Implementation: ${JSON.stringify(implementation)}. Checkpoint: ${JSON.stringify(measured)}`)
    measured = inspectExperiment(await experiment(task, 'evaluate', evaluationReservation ? ['--reservation', evaluationReservation] : []), task)
  }
  let escalated = false
  while (true) {
    const checkpoint = measured.checkpoint
    if (checkpoint.attempts.some(attempt => attempt.measuring || !attempt.published)) throw new Error('Unresolved measurement must recover without relaunch')
    if (checkpoint.final_record || checkpoint.index_updated) {
      const recovered = inspectExperiment(await experiment(task, 'recover'), task)
      if (!recovered.checkpoint.index_updated) throw new Error('Finalization still requires recovery')
      completed.push({ task_id: task, decision: recovered.checkpoint.final_record.decision, commit: recovered.checkpoint.commit })
      return
    }
    const decision = await control('assess measured hypothesis',
      `Check actual requirements, samples, checks, assessment, comparison validity and retained candidate references. Return exactly {"action":"keep"|"discard"|"failed-to-run"|"inconclusive"|"repair"|"rerun"|"block","lesson":string,"reason":string}. Keep requires measured improvement and all configured checks; discard needs usable evidence. Exhausted attempts without usable observations permit failed-to-run; with observations but no answer permit inconclusive. Never fabricate terminal evidence to clear interruption. Shared environment failure returns block. A changed comparison condition cannot keep; recover safe restoration before actual rebaseline. No worker/measurement or record edits. Evidence: ${JSON.stringify(measured)}`)
    if (!exactObject(decision, ['action', 'lesson', 'reason'])
      || !['keep', 'discard', 'failed-to-run', 'inconclusive', 'repair', 'rerun', 'block'].includes(decision.action)
      || !validText(decision.lesson) || !validText(decision.reason)) throw new Error('Invalid measured disposition')
    if (decision.action === 'block') {
      await runCommand('block', ['--reason', decision.reason])
      throw new Error(`Shared blocker: ${decision.reason}; recover candidate before supporting repairs`)
    }
    let verification = null
    let repairFindings = null
    if (decision.action === 'keep') {
      if (!measured.comparison_valid || measured.assessment?.improved !== true
        || !/^W[0-9a-f]{16}$/.test(checkpoint.verification_snapshot ?? '')) throw new Error('Keep lacks valid improving measurements and verification snapshot')
      const snapshot = checkpoint.verification_snapshot
      const raw = await dispatch(task, 'confirmation', 'verifier', workerPrompt(selected,
        `Independently review all acceptance requirements and declared code, read coding Review and Code standards. Show supplied zdev work-context ${area} --show ${snapshot} --format json. Starting baseline, owned paths, measured samples and full policy: ${JSON.stringify(measured)}. Return exactly verdict, summary, findings, escalation as the ordinary strict semantic verifier object.`))
      const semantic = parseVerifierResult(raw)
      const comparison = parseComparison(await command(['work-context', area, '--compare', snapshot], `${task}: compare acceptance snapshot`), area, snapshot)
      if (!semantic || !comparison?.equal) throw new Error('Independent acceptance result or stable snapshot comparison failed')
      if (semantic.verdict === 'blocker') throw new Error(semantic.summary)
      if (semantic.verdict === 'rework') {
        await checkedControl('check acceptance findings', `Independently check every substantive verifier finding against snapshot ${snapshot}; require concrete task-owned repairs. Findings: ${JSON.stringify(semantic.findings)}`)
        if (semantic.escalation === 'advanced-implementer') {
          if (selected.complexity !== 'standard' || escalated) throw new Error('Inapplicable or repeated acceptance escalation')
          escalated = true
        }
        repairFindings = semantic.findings
      } else verification = { schema_version: 1, kind: 'verifier', area, task_id: task,
        ...semantic, evidence: [`work_context_snapshot: ${snapshot}`] }
    }
    if (repairFindings || ['repair', 'rerun'].includes(decision.action)) {
      if (selected.complexity === 'advanced' && !plan && (repairFindings || decision.action === 'repair')) throw new Error('Advanced repair requires its retained semantic plan; preserve checkpoint without replanning')
      const kind = repairFindings || decision.action === 'repair' ? 'repair' : 'rerun'
      const request = await reserve(task, kind)
      if (kind === 'repair') {
        const refreshed = await command(['work-context', area, '--task', task, '--store'], `${task}: refresh repair admission`)
        const admission = parseStoredContext(refreshed, area)
        if (!admission || admission.taskId !== task || admission.hypothesis !== true
          || admission.head !== checkpoint.head || admission.complexity !== selected.complexity) throw new Error('Repair lost explicit hypothesis admission and baseline HEAD')
        const role = escalated || selected.complexity === 'advanced' ? 'advanced-implementer'
          : selected.complexity === 'routine' ? 'routine-implementer' : 'implementer'
        const raw = await callAgent(workerPrompt({ ...selected, context: refreshed },
          `Repair only declared files ${JSON.stringify(checkpoint.paths)}. Load original baseline zdev work-context ${area} --show ${checkpoint.starting_snapshot} --format json. Findings: ${JSON.stringify(repairFindings ?? [decision.reason])}. ${plan ? `Retain this unchanged semantic plan: ${JSON.stringify(plan)}.` : ''}`),
          nativeWorkerOptions({ profiles }, role, { agentType: `zdev:zdev-${role}`, label: `zdev research ${task}: repair` }))
        if (parseWorkerResult(raw, 'implementer', area, task)?.verdict !== 'ready') throw new Error('Repair did not return a valid ready result')
        await checkedControl('inspect repaired delta', `Require whole candidate delta within declared files and unchanged records/HEAD/policy. Checkpoint: ${JSON.stringify(checkpoint)}`)
      }
      measured = inspectExperiment(await experiment(task, 'evaluate', ['--reservation', request]), task)
      continue
    }
    const finalized = await control('finalize measured hypothesis',
      `Use managed experiment finalize for ${task} exactly once with --decision ${decision.action} --lesson ${JSON.stringify(decision.lesson)}${verification ? ` and --verification <temporary-envelope-file> containing ${JSON.stringify(verification)}` : ''}. Keep temporary files outside tracked paths. Return complete experiment JSON stdout or {"error":"concrete failure"}. Finalize alone owns candidate retention/restoration, records, completion and exact commit. Never repeat ordinary task done/staging/commit.`)
    inspectExperiment(finalized, task)
    if (!finalized.checkpoint.index_updated || !/^[0-9a-f]{40}$/.test(finalized.checkpoint.commit ?? '')) throw new Error('Managed finalization did not finish its exact commit')
    completed.push({ task_id: task, decision: finalized.checkpoint.final_record.decision, commit: finalized.checkpoint.commit })
    return
  }
}

const checkRollingFindings = async (selected, semantic, kind = 'confirmation', snapshot = null) => {
  const marked = semantic.findings.filter(finding => finding.startsWith(validationWriteMarker))
  if (marked.length && !reportsValidationWrite(semantic)) throw new Error('Malformed validation-write findings refuse rework')
  const substantive = semantic.findings.filter(finding => !finding.startsWith(validationWritePrefix))
  if (!substantive.length) return
  const request = await reserve(selected.task_id, kind)
  const raw = await callAgent(workerPrompt(selected,
    `Independently check only these substantive findings, their checked location, consequence and applicability: ${JSON.stringify(substantive)}. ${snapshot ? `Use exact snapshot ${snapshot}.` : 'Inspect the current supporting verification state.'} Keep files and records unchanged. Return exactly {"safe":boolean,"reason":string}; safe true means every finding supports the requested in-scope correction.`),
    nativeWorkerOptions({ profiles }, 'verifier', { label: `zdev research ${selected.task_id}: check findings` }))
  const checked = json(raw)
  if (!exactObject(checked, ['safe', 'reason']) || checked.safe !== true || !validText(checked.reason)) throw new Error('Finding check unavailable or unsupported; preserve claim and candidate')
  await checkedControl('confirm standalone role stopped', `Confirm actual finding checker ${request} and its subprocesses stopped. Result: ${JSON.stringify(checked)}`)
  await runCommand('settle', ['--request', request])
  if (snapshot) {
    const compared = parseComparison(await command(['work-context', area, '--compare', snapshot], 'compare finding check'), area, snapshot)
    if (!compared?.equal) throw new Error('Candidate changed during independent finding check')
  }
}

// Only promises and arrival identities are transient. Run claims/checkpoints own recovery.
const runRollingResearch = async () => {
  const candidates = new Map()
  const active = new Map()
  const measuring = new Map()
  const arrivals = []
  let wake = null
  let generatedWithoutWork = false
  let supportWaiting = null
  let stopping = false
  const signal = event => { arrivals.push(event); if (wake) { wake(); wake = null } }
  const launch = (kind, candidate, operation) => {
    const calls = kind === 'source' ? active : measuring
    const promise = Promise.resolve().then(operation)
      .then(value => ({ kind, candidate, value }), error => ({ kind, candidate, error: String(error?.message ?? error) }))
      .then(event => { signal(event) })
    calls.set(candidate.selected.task_id, promise)
  }
  const sourcePrompt = (candidate, action) => {
    const checkpoint = candidate.measured.checkpoint
    const isolated = checkpoint.isolated
    return `${repositoryGuidance}\n\nRead task contract ${JSON.stringify(taskWorkflowContractPath)}, coding Design/Completion/Code standards ${JSON.stringify(codingReferencePath)}, and research reference ${JSON.stringify(researchReferencePath)}. Complete task ${candidate.selected.task_id}. Read authoritative brief ${JSON.stringify(isolated.brief)} FIRST, complete task record ${JSON.stringify(isolated.task_record)} and repository instructions ${JSON.stringify(isolated.repository_instructions)}. Read immutable admitted snapshot ${isolated.admitted_snapshot} through zdev work-context ${area} --show in authoritative cwd ${JSON.stringify(root)}. Use absolute assigned source cwd ${JSON.stringify(isolated.source_root)} for EVERY source edit and validation command. Never read or edit source .zdev, edit authoritative records, integrate, stage, complete, finalize or commit. Edit only exact files ${JSON.stringify(checkpoint.paths)}; report any additional file before editing. ${action} ${candidate.plan ? `Follow this semantic plan unchanged: ${JSON.stringify(candidate.plan)}.` : ''} Return only the strict implementer result.`
  }
  const launchSource = candidate => {
    retainedCandidates[candidate.selected.task_id] = { findings: candidate.findings ?? [], escalated: candidate.escalated === true, needs_repair: true }
    const role = candidate.escalated || candidate.selected.complexity === 'advanced' ? 'advanced-implementer'
      : candidate.selected.complexity === 'routine' ? 'routine-implementer' : 'implementer'
    launch('source', candidate, () => callAgent(sourcePrompt(candidate,
      `Implement the complete hypothesis. Repair findings: ${JSON.stringify(candidate.findings ?? [])}. Never integrate, stage or commit.`),
      nativeWorkerOptions({ profiles }, role, { agentType: `zdev:zdev-${role}`, label: `zdev research ${candidate.selected.task_id}: source implementation` })))
  }
  const collect = async event => {
    const candidate = event.candidate
    const task = candidate.selected.task_id
    const pendingCalls = event.kind === 'source' ? active : measuring
    pendingCalls.delete(task)
    if (event.error) throw new Error(`${task} ${event.kind}: ${event.error}; confirm actual worker/process stop before recovery`)
    if (event.kind === 'source') {
      const result = parseWorkerResult(event.value, 'implementer', area, task)
      if (!result) throw new Error(`${task}: invalid or interrupted source result; preserve worker claim and source`)
      await checkedControl('confirm source stopped', `Inspect actual native source role and owned subprocesses for task ${task}, request ${candidate.request}, source ${candidate.measured.checkpoint.isolated.source_root}. Confirm completion/stop and exact owned delta before Return. For a source reconciliation whose conflicts are fully resolved in owned files, inspect and stage only those resolved source paths to clear its unmerged index; never stage destination records. Promise completion alone is insufficient. Worker result: ${JSON.stringify(result)}`)
      candidate.measured = inspectExperiment(await experiment(task, 'return', ['--worker-stopped']), task)
      candidate.needsMeasurement = true
      if (result.verdict === 'ready') retainedCandidates[task] = { findings: [], escalated: candidate.escalated === true, needs_repair: false }
      if (result.verdict === 'blocker') {
        const disposition = parseBlockerDisposition(await control('classify hypothesis implementation blocker',
          `Inspect stopped source effects. Continue only directly actionable work inside exact paths, original scope and policy. Return exactly {"action":"continue"|"stop","reason":string}. Result: ${JSON.stringify(result)}. Checkpoint: ${JSON.stringify(candidate.measured)}`))
        if (disposition?.action !== 'continue') throw new Error(disposition?.reason ?? 'Source has a concrete unresolved blocker')
        candidate.findings = [result.summary, ...result.findings]
        candidate.needsRepair = true
        retainedCandidates[task] = { findings: candidate.findings, escalated: candidate.escalated === true, needs_repair: true }
      }
    } else {
      candidate.measured = inspectExperiment(event.value, task)
      if (candidate.measured.checkpoint.attempts.some(attempt => attempt.measuring || !attempt.published)) throw new Error(`${task}: unresolved measurement; preserve process ownership without relaunch`)
      candidate.needsMeasurement = false
    }
  }
  const nextArrival = async () => {
    if (!arrivals.length && (active.size || measuring.size)) await new Promise(resolve => { wake = resolve })
    if (arrivals.length) await collect(arrivals.shift())
  }
  const roleCapacity = async () => {
    await refresh()
    while (context.remaining.workers === 0 && active.size) { await nextArrival(); await refresh() }
    if (context.remaining.workers === 0) throw new Error('Native role capacity remains occupied by unresolved durable claims')
  }
  const prepareRepair = async candidate => {
    if (candidate.selected.complexity === 'advanced' && !candidate.plan) throw new Error('Advanced repair requires its retained original semantic plan')
    await roleCapacity()
    candidate.request = await reserve(candidate.selected.task_id, 'repair')
    candidate.measured = inspectExperiment(await experiment(candidate.selected.task_id, 'prepare', ['--reservation', candidate.request]), candidate.selected.task_id)
    if (candidate.measured.dispatch_authorized !== true || candidate.measured.checkpoint.preparation?.request !== candidate.request) throw new Error('Prepare replay or changed request refused source dispatch')
    candidate.needsRepair = false
    launchSource(candidate)
  }
  const synchronize = async candidate => {
    const task = candidate.selected.task_id
    candidate.measured = inspectExperiment(await experiment(task, 'inspect'), task)
    if (candidate.measured.checkpoint.isolated.integrated) return
    const checked = await control('check candidate baseline',
      `Read fresh destination HEAD, accepted code and comparison policy, and this stopped returned source. Return exactly {"refresh":boolean,"remeasure":boolean,"reason":string}. Refresh whenever destination HEAD or comparison conditions differ. Remeasure when accepted CODE or conditions changed; record-only HEAD drift may refresh retained measurements. Do not mutate. Checkpoint: ${JSON.stringify(candidate.measured)}. Fresh run: ${JSON.stringify(context)}`)
    if (!exactObject(checked, ['refresh', 'remeasure', 'reason']) || typeof checked.refresh !== 'boolean' || typeof checked.remeasure !== 'boolean'
      || (checked.remeasure && !checked.refresh) || !validText(checked.reason)) throw new Error('Invalid current-baseline check')
    if (checked.refresh) {
      try {
        candidate.measured = inspectExperiment(await experiment(task, 'reconcile'), task)
      } catch (error) {
        const preserved = inspectExperiment(await experiment(task, 'inspect'), task)
        if (!preserved.checkpoint.isolated.refreshing || !preserved.checkpoint.isolated.worker_stopped
          || preserved.checkpoint.attempts.some(attempt => attempt.measuring)) throw error
        candidate.measured = preserved
        candidate.findings = [`Resolve the retained source reconciliation conflicts against current accepted code: ${String(error?.message ?? error)}`]
        candidate.needsRepair = true
        retainedCandidates[task] = { findings: candidate.findings, escalated: candidate.escalated === true, needs_repair: true }
      }
      candidate.needsMeasurement ||= checked.remeasure
    }
  }
  const finalizeCandidate = async (candidate, decision, verification = null) => {
    const task = candidate.selected.task_id
    const finalized = await control('finalize measured hypothesis',
      `Use managed experiment finalize for ${task} exactly once with --decision ${decision.action} --lesson ${JSON.stringify(decision.lesson)}${verification ? ` and --verification <temporary-envelope-file> containing ${JSON.stringify(verification)}` : ''}. Keep temporary files outside tracked paths. Return complete experiment JSON stdout or {"error":"concrete failure"}. Finalize alone owns candidate retention/restoration, records, completion and exact commit. Never repeat ordinary task done/staging/commit.`)
    inspectExperiment(finalized, task)
    if (!finalized.checkpoint.index_updated || !/^[0-9a-f]{40}$/.test(finalized.checkpoint.commit ?? '')) throw new Error('Managed finalization requires recovery of its exact commit')
    completed.push({ task_id: task, decision: finalized.checkpoint.final_record.decision, commit: finalized.checkpoint.commit })
    candidates.delete(task)
    delete retainedCandidates[task]
    generatedWithoutWork = false
    if (input.cleanup === true && finalized.cleanup_eligible === true) {
      await checkedControl('cleanup completed source', `Remove only this exclusively run-owned local linked worktree after actual worker/process stop, durable completion and retained history. Never remove pre-existing or ambiguous worktrees. Context: ${JSON.stringify(finalized)}`)
    }
  }
  const accept = async candidate => {
    const task = candidate.selected.task_id
    const decision = await control('assess measured hypothesis',
      `Check requirements, samples, checks, current assessment, comparison validity and retained history. Return exactly {"action":"keep"|"discard"|"failed-to-run"|"inconclusive"|"repair"|"rerun"|"block","lesson":string,"reason":string}. Only measured improvement with all checks permits keep. Exhausted unusable attempts permit failed-to-run; usable observations without an answer permit inconclusive. Interruption stays unfinished. Shared failure returns block. Evidence: ${JSON.stringify(candidate.measured)}`)
    if (!exactObject(decision, ['action', 'lesson', 'reason']) || !['keep', 'discard', 'failed-to-run', 'inconclusive', 'repair', 'rerun', 'block'].includes(decision.action)
      || !validText(decision.lesson) || !validText(decision.reason)) throw new Error('Invalid measured disposition')
    if (decision.action === 'block') {
      await runCommand('block', ['--reason', decision.reason])
      return
    }
    if (decision.action === 'repair') {
      candidate.findings = [decision.reason]
      retainedCandidates[task] = { findings: candidate.findings, escalated: candidate.escalated === true, needs_repair: true }
      await prepareRepair(candidate); return
    }
    if (decision.action === 'rerun') { candidate.measurementRequest = await reserveMeasurement(task, 'rerun'); candidate.needsMeasurement = true; return }
    if (decision.action !== 'keep') { await finalizeCandidate(candidate, decision); return }
    if (!candidate.measured.comparison_valid || candidate.measured.assessment?.improved !== true) throw new Error('Keep lacks current valid improving measurement')
    candidate.measured = inspectExperiment(await experiment(task, 'integrate'), task)
    const snapshot = candidate.measured.checkpoint.verification_snapshot
    if (!/^W[0-9a-f]{16}$/.test(snapshot ?? '')) throw new Error('Integration did not supply fresh destination verification')
    await roleCapacity() // This acceptance owner has priority over another source preparation.
    const raw = await dispatch(task, 'confirmation', 'verifier', workerPrompt(candidate.selected,
      `Independently verify the entire integrated task in authoritative destination ${root}. Read coding Review and Code standards. Show exact supplied snapshot ${snapshot}, original baseline ${candidate.measured.checkpoint.starting_snapshot}, declared paths, full samples and policy: ${JSON.stringify(candidate.measured)}. Return exactly verdict, summary, findings, escalation.`))
    let semantic = parseVerifierResult(raw)
    const compared = parseComparison(await command(['work-context', area, '--compare', snapshot], `${task}: compare acceptance snapshot`), area, snapshot)
    if (!semantic || !compared?.equal) throw new Error('Acceptance verification or immutable destination snapshot failed')
    if (onlyValidationWrites(semantic)) semantic = { ...semantic, verdict: 'pass', findings: [], escalation: 'none' }
    if (semantic.verdict === 'blocker') throw new Error(semantic.summary)
    if (semantic.verdict === 'rework') {
      await checkRollingFindings(candidate.selected, semantic, 'confirmation', snapshot)
      if (semantic.escalation === 'advanced-implementer') {
        if (candidate.selected.complexity !== 'standard' || candidate.escalated) throw new Error('Inapplicable or repeated acceptance escalation')
        candidate.escalated = true
      }
      candidate.findings = semantic.findings
      retainedCandidates[task] = { findings: semantic.findings, escalated: candidate.escalated === true, needs_repair: true }
      await prepareRepair(candidate) // Prepare withdraws integrated owned paths before source edits.
      return
    }
    await finalizeCandidate(candidate, decision, { schema_version: 1, kind: 'verifier', area, task_id: task,
      ...semantic, evidence: [`work_context_snapshot: ${snapshot}`] })
  }
  const reserveMeasurement = async (task, kind) => {
    const request = nextRequest(kind)
    const value = await runCommand('reserve', ['--request', request, '--task', task, '--kind', kind])
    const reservation = value.run?.reservations?.[request]
    if (!validRunContext(value) || value.run.id !== runId || !reservation || reservation.task !== task
      || reservation.kind !== kind || reservation.settled || reservation.bound || reservation.requested_worker_slot) throw new Error('Measurement reservation did not authorize a fresh attempt')
    context = value
    return request
  }
  try {
    for (const checkpoint of context.checkpoints.filter(item => !item.index_updated)) {
      if (!checkpoint.isolated) throw new Error('Rolling resume found local candidate; resume its serial route')
      const measured = inspectExperiment(await experiment(checkpoint.task, checkpoint.isolated.refreshing ? 'inspect' : 'recover'), checkpoint.task)
      if (measured.checkpoint.index_updated) continue
      if (!measured.checkpoint.isolated.worker_stopped || measured.checkpoint.attempts.some(attempt => attempt.measuring || !attempt.published)) throw new Error('Unresolved worker or measurement must remain owned before rolling resume')
      const selected = await select(checkpoint.task)
      if (selected.complexity === 'advanced' && !retainedPlans[checkpoint.task]) throw new Error('Advanced rolling resume requires the unchanged original semantic plan before dispatch')
      candidates.set(checkpoint.task, { selected, measured, plan: retainedPlans[checkpoint.task] ?? null,
        needsMeasurement: !measured.checkpoint.attempts.length || measured.checkpoint.attempts.at(-1).candidate !== measured.checkpoint.isolated.returned_commit,
        ...(retainedCandidates[checkpoint.task] ? { findings: retainedCandidates[checkpoint.task].findings,
          escalated: retainedCandidates[checkpoint.task].escalated, needsRepair: retainedCandidates[checkpoint.task].needs_repair || measured.checkpoint.isolated.refreshing }
          : !measured.checkpoint.attempts.length || measured.checkpoint.isolated.refreshing ? { needsRepair: true } : {}) })
    }
    while (true) {
      await refresh()
      if (nativeCalls >= 900) stopping = true
      while (arrivals.length) await collect(arrivals.shift())
      await refresh()
      if (context.stop_reason && context.stop_reason !== 'blocker') stopping = true
      if (stopping) {
        if (active.size || measuring.size) { await nextArrival(); continue }
        if (!context.stop_reason) { await runCommand('stop'); await refresh() }
        recovery.push(...[...candidates.keys()].map(task => `Resume unfinished ${task} from its retained source and checkpoint; do not replay dispatch.`))
        return report('stopped', nativeCalls >= 900 ? 'workflow-call-boundary' : context.stop_reason)
      }
      if (pendingRecoveryVerification) {
        if (active.size || measuring.size) { await nextArrival(); continue }
        if (context.run.acceptance) throw new Error('Recover candidate acceptance before ordinary completion verification')
        await recoverEffects()
        continue
      }
      // Shared support and rebaseline require a quiet destination and stopped measurements.
      if (context.stop_reason === 'blocker' || !context.run.comparison_valid || supportWaiting) {
        if (active.size || measuring.size) { await nextArrival(); continue }
        if (context.run.acceptance) throw new Error('Recover integrated candidate withdrawal before shared supporting work')
        const selected = supportWaiting ?? (context.stop_reason === 'blocker' ? await select(null, [...candidates.keys()]) : null)
        supportWaiting = null
        if (selected?.task_id === null) {
          if (generatedWithoutWork) throw new Error('No relevant supporting repair was admitted for shared blocker')
          await generate(); generatedWithoutWork = true; continue
        }
        if (selected) {
          await support(selected); generatedWithoutWork = false; await refresh()
          if (context.stop_reason === 'blocker') {
            await checkedControl('confirm shared repair', `Confirm committed supporting work resolved the recorded shared failure without changing frozen requirements. Context: ${JSON.stringify(context)}`)
            await runCommand('resolve')
          }
        }
        await checkedControl('prepare accepted rebaseline', `Require clean accepted HEAD, stopped measurements and unchanged metric/direction/correctness/acceptance requirements. Preserve stopped isolated sources. Run actual commands rather than supplied samples. Context: ${JSON.stringify(context)}`)
        await runCommand('rebaseline', ['--request', nextRequest('rebaseline')])
        await recoverEffects(false) // Publish the actual area baseline before comparison/refill.
        continue
      }
      const acceptance = [...candidates.values()].find(candidate => candidate.measured.checkpoint.isolated.integrated)
      const measured = acceptance ?? [...candidates.values()].find(candidate => !active.has(candidate.selected.task_id)
        && !measuring.has(candidate.selected.task_id) && !candidate.needsMeasurement && !candidate.needsRepair)
      if (measured && measuring.size === 0) {
        await synchronize(measured)
        if (!measured.needsMeasurement && !measured.needsRepair) { await accept(measured); continue }
      }
      // Refill independently of the evaluator. Never wait for every sibling first.
      if (context.remaining.workers > 0 && !context.run.acceptance) {
        const repair = [...candidates.values()].find(candidate => candidate.needsRepair && !active.has(candidate.selected.task_id))
        if (repair) { await prepareRepair(repair); continue }
        const selected = await select(null, [...candidates.keys()])
        if (selected.task_id === null) {
          if (!generatedWithoutWork && measuring.size === 0 && context.remaining.measurements === context.run.measurement_limit) {
            await generate(); generatedWithoutWork = true; continue
          }
          if (!candidates.size) throw new Error('Proposal made no eligible progress')
        } else if (selected.kind === 'supporting') { supportWaiting = selected; continue }
        else {
          generatedWithoutWork = false
          let plan = retainedPlans[selected.task_id] ?? null
          if (selected.complexity === 'advanced' && !plan) {
            const raw = await dispatch(selected.task_id, 'preparation', 'planner', workerPrompt(selected,
              'Plan advanced hypothesis without edits. Return the strict four-field semantic planner object.'), { schema: plannerSchema })
            plan = parsePlannerResult(raw)
            if (!plan || plan.verdict !== 'plan' || !reconstructPlannerResult(plan, area, selected.task_id)) throw new Error('Advanced source requires a valid retained semantic plan')
          }
          const admitted = await command(['work-context', area, '--task', selected.task_id, '--store'], `${selected.task_id}: fresh source admission`)
          const stored = parseStoredContext(admitted, area)
          if (!stored || stored.taskId !== selected.task_id || !stored.hypothesis) throw new Error('Source dispatch lost its fresh explicit task')
          const source = await control('create current source',
            `Create an exclusively run-owned linked Git worktree outside the authoritative checkout and source branch from authoritative destination HEAD ${stored.head} current at THIS dispatch, verifying its code matches accepted baseline. Do not reuse a sibling or change destination. Keep records unchanged. Return exactly {"source_root":canonical-absolute-path,"head":full-commit}. Run ${runId}, task ${selected.task_id}, request namespace ${invocation}. Authoritative root: ${root}`)
          if (!exactObject(source, ['source_root', 'head']) || !validText(source.source_root) || !source.source_root.startsWith('/')
            || source.source_root === root || source.source_root.startsWith(`${root}/`) || source.head !== stored.head) throw new Error('Source was not created from current dispatch HEAD')
          const request = await reserve(selected.task_id, 'preparation')
          const value = inspectExperiment(await experiment(selected.task_id, 'admit', ['--source', source.source_root,
            ...selected.paths.flatMap(path => ['--path', path]), '--reservation', request]), selected.task_id)
          if (value.dispatch_authorized !== true || value.checkpoint.preparation?.request !== request
            || value.checkpoint.isolated?.source_root !== source.source_root || value.checkpoint.isolated.dispatch_head !== stored.head) throw new Error('Admit replay or changed source refused implementation dispatch')
          const candidate = { selected: { ...selected, context: admitted }, measured: value, plan, request, needsMeasurement: false }
          candidates.set(selected.task_id, candidate)
          launchSource(candidate)
          continue
        }
      }
      await refresh()
      if (!context.run.acceptance && context.remaining.measurements > 0 && measuring.size < context.run.measurement_limit) {
        const candidate = [...candidates.values()].find(item => item.needsMeasurement && !item.needsRepair
          && !active.has(item.selected.task_id) && !measuring.has(item.selected.task_id))
        if (candidate) {
          await synchronize(candidate)
          if (candidate.needsRepair) continue
          const measurementRequest = candidate.measurementRequest
          launch('measurement', candidate, () => experiment(candidate.selected.task_id, 'evaluate',
            measurementRequest ? ['--reservation', measurementRequest] : []))
          candidate.measurementRequest = null
          continue
        }
      }
      if (active.size || measuring.size || arrivals.length) { await nextArrival(); continue }
      throw new Error('No safe dispatch, measurement or acceptance progress; inspect retained capacity and candidates')
    }
  } catch (error) {
    // No cancellation API is assumed. Drain known calls, retaining ambiguous effects for fresh recovery.
    await Promise.all([...active.values(), ...measuring.values()])
    recovery.push(...arrivals.map(event => `${event.candidate.selected.task_id}: inspect stopped ${event.kind} effects and original request before recovery`))
    throw error
  }
}

try {
  const initial = await control('inspect authorization',
    `This is fresh invocation ${JSON.stringify(invocation)}, authorization ${mode}. Inspect live durable state; never use stopped-script replay as fresh dispatch authority. Read brief first, research settings, complete work-context and existing run context. Return exactly {"repository_root":absolute-path,"safe":boolean,"reason":string,"run_context":run-context|null}. Require open area, safe branch and attributable staged/unstaged/untracked work. Do not mutate. No existing run is expected for first authorization.`)
  if (!exactObject(initial, ['repository_root', 'safe', 'reason', 'run_context'])
    || !validText(initial.repository_root) || !initial.repository_root.startsWith('/') || initial.safe !== true
    || !validText(initial.reason)) throw new Error(initial.reason ?? 'Unsafe authorization context')
  root = initial.repository_root
  if (initial.run_context !== null && !validRunContext(initial.run_context)) throw new Error('Invalid original run context')
  context = initial.run_context
  if (context && [...Object.keys(context.run.reservations), ...Object.keys(context.run.admissions)]
    .some(request => request.startsWith(`${invocation}-`))) {
    throw new Error('Invocation identity was already used; resume with a fresh invocation_id and inspect live effects')
  }
  if (mode === 'start') {
    if (context && !context.run.ended) throw new Error('An unfinished run must resume its existing identity')
    const flags = ['--scope', input.scope]
    if (input.until_interrupted === true) flags.push('--until-interrupted')
    for (const key of [...limits, ...capacities]) if (Object.hasOwn(input, key)) flags.push(`--${key.replaceAll('_', '-')}`, String(input[key]))
    startAttempted = true
    context = await command(['research', 'run', area, 'start', ...flags], 'start authorized run')
    if (!validRunContext(context)) throw new Error('Start did not establish run context')
    runId = context.run.id
  } else {
    if (!context || context.run.id !== input.run_id || context.run.ended) throw new Error('Resume must name the same unfinished authorization')
    runId = input.run_id
  }
  if (!profiles) {
    if (mode === 'resume') recovery.push('Concrete worker handoff unavailable; resolve all five Claude roles afresh before the next dispatch.')
    const flags = [...(input.run_profile ? ['--run-profile', input.run_profile] : []),
      ...Object.entries(input.role_profiles ?? {}).flatMap(([role, profile]) => ['--role-profile', `${role}=${profile}`])]
    const resolved = {}
    for (const role of roles) resolved[role] = await command(['config', 'profile', 'resolve', 'claude', role, '--area', area, ...flags], `resolve ${role}${mode === 'resume' ? ' afresh; concrete handoff unavailable' : ''}`)
    if (!validProfiles(resolved)) throw new Error('Could not freeze all five concrete Claude roles')
    profiles = resolved
  }
  await recoverEffects(false)
  if (mode === 'resume' && context.stop_reason === 'interruption') await runCommand('resume')
  if (execution === 'rolling') return await runRollingResearch()
  let generatedWithoutWork = false
  while (true) {
    await refresh()
    await recoverEffects()
    const unfinished = context.checkpoints.find(checkpoint => !checkpoint.index_updated)
    if (unfinished) {
      const selected = await select(unfinished.task)
      await hypothesis(selected)
      continue
    }
    if (context.stop_reason && context.stop_reason !== 'blocker') return report('stopped', context.stop_reason)
    if (!context.run.comparison_valid && context.stop_reason !== 'blocker') {
      await checkedControl('prepare accepted rebaseline', `Require clean accepted HEAD and unchanged metric/direction/correctness policy; recover or safely restore unfinished candidates before rebaseline. Do not replace supplied samples to clear invalidation. Context: ${JSON.stringify(context)}`)
      await runCommand('rebaseline', ['--request', nextRequest('rebaseline')])
      continue
    }
    const selected = await select()
    if (selected.task_id === null) {
      if (generatedWithoutWork) throw new Error('Proposal made no eligible progress; no further generation authorized by this invocation')
      await generate()
      generatedWithoutWork = true
      continue
    }
    generatedWithoutWork = false
    if (selected.kind === 'supporting') {
      await support(selected)
      await refresh()
      if (context.stop_reason === 'blocker') {
        await checkedControl('confirm shared repair', `Check the recorded shared environment failure is actually resolved by committed supporting work. Preserve original policy. Context: ${JSON.stringify(context)}`)
        await runCommand('resolve')
      }
    } else await hypothesis(selected)
  }
} catch (error) {
  reporting = true
  recovery.push(String(error?.message ?? error))
  if (root && startAttempted && !runId) {
    try {
      const live = await command(['research', 'run', area, 'context'], 'inspect uncertain start effects')
      if (validRunContext(live) && !live.run.ended) { context = live; runId = live.run.id }
    } catch (inspectionError) { recovery.push(`Inspect uncertain start effects: ${String(inspectionError?.message ?? inspectionError)}`) }
  }
  if (runId) {
    try {
      await refresh()
      if (!context.stop_reason) await runCommand('stop')
      await refresh()
    } catch (inspectionError) { recovery.push(`Inspect live run and effects: ${String(inspectionError?.message ?? inspectionError)}`) }
  }
  recovery.push('After stop/failure use a new zdev-research resume invocation_id with the same run_id; recover live effects before a newly reserved dispatch.')
  return report('blocker', context?.stop_reason ?? 'unsafe state')
}

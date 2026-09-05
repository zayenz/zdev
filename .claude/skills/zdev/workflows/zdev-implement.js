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
    baselineSnapshot: stored.snapshot, taskPath: stored.task_path }
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
  `${workerContract}\n\nImplement ${complexity} task ${taskId} in area ${area}. Load its immutable context with zdev work-context ${area} --show ${prepared.baselineSnapshot} --format json.${plan ? ` Follow this validated plan: ${JSON.stringify(plan)}.` : ''} Treat named and planned paths as expected seams rather than an allowlist. Change every attributable path directly needed by the task's semantic boundaries, validate the result, and return the implementer envelope from your role prompt. Do not block merely for partial progress or another file. If a planner-written technical restriction itself must be corrected, load ${JSON.stringify(taskWorkflowContractPath)} and use its transient adjustment blocker so coordination can check and edit the record. If direct work must split, use its typed implementation_split blocker and leave derive commands to the coordinator.`,
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
const verify = async () => {
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
    `${workerContract}\n\nIndependently verify task ${taskId} in area ${area}. Load the original baseline with zdev work-context ${area} --show ${prepared.baselineSnapshot} --format json and the verification snapshot with zdev work-context ${area} --show ${snapshot} --format json; require task ${taskId} at HEAD ${current.head}. Read any clarified records from the verification snapshot and check their updated requirements while retaining the original baseline for attribution. Use the implementer summary only to locate evidence. Check the whole task and run required validation. Keep verification read-only: use a check or dry-run form for generators and other commands expected to rewrite tracked files. Return exactly one JSON object with exactly these four keys and no others: verdict, summary, findings, escalation. Pass requires an empty findings array; rework requires at least one finding. Report each unexpected validation-written task-owned file as a validation_write: <repository-relative path> finding with verdict rework. Never add validation_writes or another fifth key. Do not repair or discard validation writes.\n\nTechnical adjustments: ${JSON.stringify(technicalAdjustments)}\nImplementer summary: ${compactWorkerSummary(latestImplementation)}`,
    workerOptions('verifier', { agentType: 'zdev:zdev-verifier', label: `zdev ${taskId}: verify` }),
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
const resolvedImplementation = await resolveImplementerResult(implementation, prepared, 'implementation')
if (resolvedImplementation.terminal) return resolvedImplementation.terminal
latestImplementation = resolvedImplementation.result
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
  verdict = await verify()
  if (!verdict) {
    return blocker(area, taskId, 'verification', 'verifier returned an invalid or mismatched envelope.', 'lifecycle and commit were not changed.', staleAdvisory)
  }
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

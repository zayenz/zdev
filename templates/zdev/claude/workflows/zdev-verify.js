export const meta = {
  name: 'zdev-verify',
  description: 'Independently verify the explicit current ready zdev task',
}

const repositoryGuidance = {{repository_guidance}}
const taskWorkflowContract = {{task_workflow_contract}}
const workerContract = [
  'Before acting, use the canonical zdev task-workflow contract. In Bash, when `${CLAUDE_PLUGIN_ROOT:-}` is non-empty and `"${CLAUDE_PLUGIN_ROOT}/contracts/task-workflows.md"` is readable, load that installed file. Otherwise use the rendered canonical contract included inline below in this same prompt.',
  taskWorkflowContract,
  repositoryGuidance,
].join('\n\n')
const normalizeVerifyArgs = value => {
  if (Array.isArray(value)) return { area: value[0], task_id: value[1] }
  if (typeof value === 'string') {
    const [area, task_id] = value.trim().split(/\s+/, 2)
    return { area, task_id }
  }
  return value && typeof value === 'object' ? value : {}
}
const input = normalizeVerifyArgs(args)
const area = String(input.area ?? '').trim()
const taskId = String(input.task_id ?? input.taskId ?? '').trim()
const advisoryText = 'stale effective-base link; managed rebase remains optional.'
const blocker = (subjectArea, subjectTask, reason, staleAdvisory = false) =>
  `BLOCKER zdev-verify ${subjectArea} ${subjectTask}\n\nArea: ${subjectArea}\nTask: ${subjectTask}\n${staleAdvisory ? `Advisory: ${advisoryText}\n` : ''}Summary: ${reason}\nValidation: not accepted.\nLocated evidence: no verifier result was accepted.`
const expectedContextKeys = [
  'area',
  'git_diff',
  'git_diff_cached',
  'git_status',
  'goal',
  'head',
  'lifecycle',
  'queue',
  'schema_version',
  'stale_advisory',
  'status',
  'task_id',
]
const parseReady = raw => {
  if (typeof raw !== 'string') return null
  let payload
  try {
    payload = JSON.parse(raw)
  } catch {
    return null
  }
  if (!payload || Array.isArray(payload) || typeof payload !== 'object') return null
  if (JSON.stringify(Object.keys(payload).sort()) !== JSON.stringify(expectedContextKeys)) return null
  if (payload.schema_version !== 1 || payload.area !== area || payload.task_id !== taskId) return null
  if (payload.lifecycle !== 'open' || payload.queue !== 'ready' || !/^[0-9a-f]{40}$/.test(payload.head ?? '')) return null
  for (const key of ['git_status', 'git_diff_cached', 'git_diff']) {
    if (typeof payload[key] !== 'string') return null
  }
  const status = payload.status
  const goal = payload.goal
  const taskWork = status?.branch_status?.task_work
  if (taskWork?.safe !== true || typeof taskWork.stale_advisory !== 'boolean' || payload.stale_advisory !== taskWork.stale_advisory) return null
  if (status?.area?.tag !== area || status?.lifecycle !== 'open' || status?.queue !== 'ready' || status?.next !== taskId) return null
  if (goal?.lifecycle !== 'open' || goal?.queue !== 'ready' || goal?.area?.tag !== area || goal?.task?.id !== taskId) return null
  return { raw, head: payload.head, staleAdvisory: taskWork.stale_advisory, payload }
}
const publicResultKeys = ['area', 'escalation', 'evidence', 'findings', 'kind', 'schema_version', 'summary', 'task_id', 'verdict']
const topLevelKeys = raw => {
  let index = 0
  const keys = []
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
  return index === raw.length ? keys : null
}
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
  if (typeof raw !== 'string') return null
  const keys = topLevelKeys(raw)
  if (!keys || new Set(keys).size !== keys.length) return null
  if (JSON.stringify([...keys].sort()) !== JSON.stringify(verifierResultKeys)) return null
  let result
  try {
    result = JSON.parse(raw)
  } catch {
    return null
  }
  if (!result || Array.isArray(result) || typeof result !== 'object') return null
  if (typeof result.summary !== 'string' || !result.summary.trim()) return null
  if (!Array.isArray(result.findings)
    || !result.findings.every(item => typeof item === 'string' && item.trim())) return null
  if (!['pass', 'rework', 'blocker'].includes(result.verdict)) return null
  const validEscalation = result.escalation === 'none'
    || (result.verdict === 'rework' && result.escalation === 'advanced-implementer')
  if (result.verdict === 'pass' && result.findings.length !== 0) return null
  if (result.verdict === 'rework' && result.findings.length === 0) return null
  return validEscalation ? result : null
}
const parseStoredContext = (raw, expected) => {
  if (typeof raw !== 'string') return null
  let stored
  try {
    stored = JSON.parse(raw)
  } catch {
    return null
  }
  if (!stored || Array.isArray(stored) || typeof stored !== 'object') return null
  if (JSON.stringify(Object.keys(stored).sort()) !== JSON.stringify(['context', 'snapshot'])) return null
  if (!/^W[0-9a-f]{16}$/.test(stored.snapshot ?? '')) return null
  const context = parseReady(JSON.stringify(stored.context))
  if (!context
    || context.payload.head !== expected.payload.head
    || context.payload.git_status !== expected.payload.git_status
    || context.payload.git_diff_cached !== expected.payload.git_diff_cached
    || context.payload.git_diff !== expected.payload.git_diff) return null
  return { snapshot: stored.snapshot, context }
}
const parseComparison = (raw, expectedSnapshot) => {
  if (typeof raw !== 'string') return null
  let result
  try {
    result = JSON.parse(raw)
  } catch {
    return null
  }
  if (!result || Array.isArray(result) || typeof result !== 'object') return null
  if (JSON.stringify(Object.keys(result).sort()) !== JSON.stringify(['area', 'equal', 'schema_version', 'snapshot'])) return null
  return result.schema_version === 1 && result.area === area
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
  return JSON.stringify(Object.keys(result).sort()) === JSON.stringify(publicResultKeys)
    ? result : null
}

if (!/^[a-z0-9][a-z0-9-]*$/.test(area) || !/^[a-z0-9][a-z0-9-]*$/.test(taskId)) {
  return blocker('unknown', 'unknown', 'a lowercase area and explicit task ID are required.')
}

const preflight = await agent(
  `Act only as the coordinating read-only preflight. Run zdev work-context ${area} --format json exactly once and return its complete JSON stdout unchanged, with no fence or other text. Keep files and Git state unchanged.`,
  { label: 'zdev verify preflight' },
)
const prepared = parseReady(preflight?.trim())
if (!prepared) {
  return blocker(area, taskId, 'missing or invalid ready goal, requested task match, branch safety, or complete Git baseline evidence.')
}
const advisory = prepared.staleAdvisory ? advisoryText : null

const storedRaw = await agent(
  `Act only as deterministic verification coordination for task ${taskId} in area ${area}. Immediately before verifier dispatch, run zdev work-context ${area} --store --format json, then show that snapshot with zdev work-context ${area} --show <snapshot> --format json. Return only {"snapshot":"<snapshot>","context":<shown JSON object>}. Keep files and Git state unchanged.`,
  { label: 'zdev verification snapshot' },
)
const stored = parseStoredContext(storedRaw?.trim(), prepared)
if (!stored) {
  return blocker(area, taskId, 'coordinator could not store and validate the admitted verification snapshot.', prepared.staleAdvisory)
}

const verified = await agent(
  `${workerContract}\n\nIndependently verify task ${taskId} in area ${area} from the current checkout. Show the coordinator-supplied immutable context with zdev work-context ${area} --show ${stored.snapshot} --format json and require the same open, ready, safe task and HEAD ${prepared.head}. Check the whole task and run required validation. Report validation-written task-owned files as rework and ambiguous writes as blocker; never repair or discard them. For every concrete task-owned file written by validation, include the exact finding validation_write: <normalized repository-relative path>; never use that prefix for an ordinary implementation defect. Put checked locations and validation conclusions in summary. Return only the exact four-field JSON object {"verdict":"pass|rework|blocker","summary":"<non-empty summary>","findings":[],"escalation":"none|advanced-implementer"} with no identity, evidence, or surrounding text. Keep lifecycle and coordination-owned state unchanged.\n\nVerification snapshot: ${stored.snapshot}`,
  { agentType: 'zdev:zdev-verifier', label: 'zdev fresh verification' },
)
const semantic = parseVerifierResult(verified?.trim())
const comparedRaw = await agent(
  `Act only as deterministic post-verification coordination. Run zdev work-context ${area} --compare ${stored.snapshot} --format json exactly once and return its complete JSON stdout unchanged, with no fence or other text. Keep files and Git state unchanged.`,
  { label: 'zdev post-verification compare' },
)
const compared = parseComparison(comparedRaw?.trim(), stored.snapshot)
if (!semantic || !compared || (!compared.equal && !reportsValidationWrite(semantic))) {
  return blocker(area, taskId, 'verifier output or post-validation comparison was invalid, contradictory, or changed ambiguously.', prepared.staleAdvisory)
}
const result = publicVerifier(semantic, stored.snapshot, advisory)
return result
  ? JSON.stringify(result)
  : blocker(area, taskId, 'coordinator could not construct the strict public verifier envelope.', prepared.staleAdvisory)

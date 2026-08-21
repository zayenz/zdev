export const meta = {
  name: 'zdev-verify',
  description: 'Independently verify the explicit current ready zdev task',
}

const taskContract = {{task_workflow_contract}}
const repositoryGuidance = {{repository_guidance}}
const workflowContract = [taskContract, repositoryGuidance].join('\n\n')
const input = args ?? {}
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
  return { raw, head: payload.head, staleAdvisory: taskWork.stale_advisory }
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
const parseWorkerResult = (raw, expectedKind, expectedArea, expectedTask) => {
  if (typeof raw !== 'string') return null
  const keys = topLevelKeys(raw)
  if (!keys || new Set(keys).size !== keys.length) return null
  if (JSON.stringify([...keys].sort()) !== JSON.stringify(workerResultKeys)) return null
  let result
  try {
    result = JSON.parse(raw)
  } catch {
    return null
  }
  if (!result || Array.isArray(result) || typeof result !== 'object') return null
  if (result.schema_version !== 1 || result.kind !== expectedKind) return null
  if (result.area !== expectedArea || result.task_id !== expectedTask) return null
  if (typeof result.summary !== 'string' || !result.summary.trim()) return null
  for (const name of ['evidence', 'findings']) {
    if (!Array.isArray(result[name])) return null
    if (!result[name].every(item => typeof item === 'string' && item.trim())) return null
  }
  const validVerdict = expectedKind === 'implementer'
    ? ['ready', 'blocker'].includes(result.verdict)
    : ['pass', 'rework', 'blocker'].includes(result.verdict)
  if (!validVerdict) return null
  const validEscalation = result.escalation === 'none'
    || (expectedKind === 'verifier' && result.verdict === 'rework' && result.escalation === 'advanced-implementer')
  return validEscalation ? result : null
}
const approvedPostValidation = result => {
  const one = prefix => {
    const matches = result.evidence.filter(item => item.startsWith(prefix))
    return matches.length === 1 ? matches[0].slice(prefix.length) : null
  }
  const head = one('HEAD: ')
  if (!/^[0-9a-f]{40}$/.test(head ?? '')) return null
  const approved = { head }
  for (const name of ['git_status', 'git_diff_cached', 'git_diff']) {
    const encoded = one(`${name}: `)
    if (encoded === null) return null
    try {
      approved[name] = JSON.parse(encoded)
    } catch {
      return null
    }
    if (typeof approved[name] !== 'string') return null
  }
  return approved
}

if (!/^[a-z0-9][a-z0-9-]*$/.test(area) || !/^[a-z0-9][a-z0-9-]*$/.test(taskId)) {
  return blocker('unknown', 'unknown', 'a lowercase area and explicit task ID are required.')
}

const preflight = await agent(
  `${workflowContract}\n\nAct only as the coordinating read-only preflight. Run zdev work-context ${area} --format json exactly once and return its complete JSON stdout unchanged, with no fence or other text. Do not run separate status, goal, or Git evidence commands, change files, or start another worker.`,
  { label: 'zdev verify preflight' },
)
const prepared = parseReady(preflight?.trim())
if (!prepared) {
  return blocker(area, taskId, 'missing or invalid ready goal, requested task match, branch safety, or complete Git baseline evidence.')
}
const advisory = prepared.staleAdvisory ? advisoryText : null

const verified = await agent(
  `${workflowContract}\n\nIndependently verify task ${taskId} in area ${area} from the current checkout. First run zdev work-context ${area} --format json yourself and require the same open, ready, safe task and HEAD ${prepared.head}; coordinator context is not your evidence. Check the whole task and run required validation, then rerun git status --short --untracked-files=all, git diff --cached, and git diff and compare them with your independent pre-validation context. A pass evidence array must contain exactly one HEAD: entry copied from your independent work-context head and exactly one git_status:, git_diff_cached:, and git_diff: entry whose remainder is the JSON encoding of that exact post-validation stdout string. Return only the required strict JSON object with kind "verifier", area "${area}", and task_id "${taskId}". ${advisory ? `Include ${advisory} exactly once in evidence.` : `Do not include ${advisoryText} in evidence.`} Make no intentional edits and never change lifecycle or Git state.\n\nCoordinator context for comparison:\n${prepared.raw}`,
  { agentType: 'zdev:zdev-verifier', label: 'zdev fresh verification' },
)
const result = verified?.trim()
const parsed = parseWorkerResult(result, 'verifier', area, taskId)
const approved = parsed?.verdict === 'pass' ? approvedPostValidation(parsed) : {}
const advisoryCount = parsed?.evidence.filter(item => item === advisoryText).length
return parsed && approved && (parsed.verdict !== 'pass' || approved.head === prepared.head) && advisoryCount === (advisory ? 1 : 0)
  ? result
  : blocker(area, taskId, 'verifier returned invalid, extra, contradictory, or mismatched JSON.', prepared.staleAdvisory)

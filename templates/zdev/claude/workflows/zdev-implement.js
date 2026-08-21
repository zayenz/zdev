export const meta = {
  name: 'zdev-implement',
  description: 'Implement, independently verify, complete, and commit one ready zdev task',
}

const taskContract = {{task_workflow_contract}}
const repositoryGuidance = {{repository_guidance}}
const workflowContract = [taskContract, repositoryGuidance].join('\n\n')
const input = args ?? {}
const area = String(input.area ?? '').trim()

const field = (text, name) => {
  const matches = text.split('\n').filter(line => line.startsWith(`${name}: `))
  return matches.length === 1 ? matches[0].slice(name.length + 2) : null
}
const advisoryText = 'stale effective-base link; managed rebase remains optional.'
const blocker = (subjectArea, taskId, stage, reason, state, staleAdvisory = false) =>
  `BLOCKER zdev-implement ${subjectArea} ${taskId}\n\nArea: ${subjectArea}\nTask: ${taskId}\n${staleAdvisory ? `Advisory: ${advisoryText}\n` : ''}Failed stage: ${stage}\nReason: ${reason}\nPreserved state: ${state}`
const expectedOpenContextKeys = [
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
const expectedClosedContextKeys = [
  'area',
  'goal',
  'lifecycle',
  'queue',
  'schema_version',
  'task_id',
]
const parseContext = (raw, expectedArea, expectedTask = null) => {
  if (typeof raw !== 'string') return null
  let payload
  try {
    payload = JSON.parse(raw)
  } catch {
    return null
  }
  if (!payload || Array.isArray(payload) || typeof payload !== 'object') return null
  if (payload.schema_version !== 1 || payload.area !== expectedArea) return null
  if (payload.lifecycle === 'closed') {
    if (JSON.stringify(Object.keys(payload).sort()) !== JSON.stringify(expectedClosedContextKeys)) return null
    if (!['empty', 'exhausted'].includes(payload.queue) || payload.task_id !== null) return null
    if (payload.goal?.lifecycle !== 'closed' || payload.goal?.queue !== payload.queue || payload.goal?.area?.tag !== expectedArea || payload.goal?.task !== null) return null
    return expectedTask ? null : { raw, lifecycle: 'closed', queue: payload.queue, taskId: null, staleAdvisory: false, payload }
  }
  if (payload.lifecycle !== 'open') return null
  if (JSON.stringify(Object.keys(payload).sort()) !== JSON.stringify(expectedOpenContextKeys)) return null
  if (!/^[0-9a-f]{40}$/.test(payload.head ?? '')) return null
  for (const key of ['git_status', 'git_diff_cached', 'git_diff']) {
    if (typeof payload[key] !== 'string') return null
  }
  const status = payload.status
  const goal = payload.goal
  const taskWork = status?.branch_status?.task_work
  if (taskWork?.safe !== true || typeof taskWork.stale_advisory !== 'boolean' || payload.stale_advisory !== taskWork.stale_advisory) return null
  if (status?.area?.tag !== expectedArea || status?.lifecycle !== 'open' || status?.queue !== payload.queue || status?.next !== payload.task_id) return null
  if (goal?.area?.tag !== expectedArea || goal?.lifecycle !== 'open' || goal?.queue !== payload.queue) return null
  if (payload.queue === 'ready') {
    if (typeof payload.task_id !== 'string' || goal?.task?.id !== payload.task_id) return null
    if (expectedTask && payload.task_id !== expectedTask) return null
  } else {
    if (!['empty', 'exhausted'].includes(payload.queue) || payload.task_id !== null || goal?.task !== null || expectedTask) return null
  }
  return { raw, lifecycle: 'open', queue: payload.queue, taskId: payload.task_id, staleAdvisory: payload.stale_advisory, payload }
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

if (!/^[a-z0-9][a-z0-9-]*$/.test(area)) {
  return blocker('unknown', 'unknown', 'input', 'a lowercase area is required.', 'no preflight or worker was started.')
}

const preflight = async label => agent(
  `${workflowContract}\n\nAct only as the coordinating preflight for area ${area}. Run zdev work-context ${area} --format json exactly once. Return its complete JSON stdout unchanged, with no fence or other text. Do not run separate status, goal, or Git evidence commands, change files, or start another worker. If the command fails, return only its error.`,
  { label },
)

const preparedRaw = (await preflight('zdev implement preflight'))?.trim()
const prepared = parseContext(preparedRaw, area)
if (prepared && prepared.taskId === null) {
  return `PASS zdev-implement ${area} none\n\nArea: ${area}\nTask: none\n${prepared.staleAdvisory ? `Advisory: ${advisoryText}\n` : ''}Summary: no ready work; ${prepared.lifecycle}/${prepared.queue} goal.\nChanged files: none.\nValidation: preflight only.\nVerifier evidence: no implementer or verifier was started.\nCommit ID: none.`
}
if (!prepared || prepared.queue !== 'ready') {
  return blocker(area, 'unknown', 'preflight', 'missing or invalid work-context evidence.', 'no implementer or verifier was started.')
}
const taskId = prepared.taskId
let staleAdvisory = prepared.staleAdvisory

const implementationRaw = (await agent(
  `${workflowContract}\n\nImplement the ready task ${taskId} in area ${area}. Use the complete coordinator context below. Change only task-owned source and tests, run required validation, and return only the required strict JSON object with kind "implementer", area "${area}", and task_id "${taskId}".\n\nCoordinator context:\n${prepared.raw}`,
  { agentType: 'zdev:zdev-implementer', label: 'zdev implementation' },
))?.trim()
const implementation = parseWorkerResult(implementationRaw, 'implementer', area, taskId)
const implementationHistory = []

const refresh = async label => {
  const current = parseContext((await preflight(label))?.trim(), area, taskId)
  if (current?.staleAdvisory) staleAdvisory = true
  return current?.queue === 'ready' ? current : blocker(area, taskId, 'context refresh', `expected ready task ${taskId} with complete work-context evidence.`, 'lifecycle and commit were not changed.', staleAdvisory)
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
const verify = async current => {
  const currentAdvisory = current.staleAdvisory ? advisoryText : null
  const raw = (await agent(
    `${workflowContract}\n\nIndependently verify task ${taskId} in area ${area}. First run zdev work-context ${area} --format json yourself and require the same open, ready, safe task ${taskId}; do not reuse the coordinator context as evidence. Use the complete implementer history only to locate evidence. Check the whole task and run required validation. Then capture git status --short --untracked-files=all, git diff --cached, and git diff. A pass evidence array must contain exactly one HEAD: entry copied from your independent work-context head and exactly one git_status:, git_diff_cached:, and git_diff: entry whose remainder is the JSON encoding of that exact post-validation stdout string. Return only the required strict JSON object with kind "verifier", area "${area}", and task_id "${taskId}". ${currentAdvisory ? `Include ${currentAdvisory} exactly once in evidence.` : `Do not include ${advisoryText} in evidence.`} Make no intentional edits.\n\nCoordinator context for comparison:\n${current.raw}\n\nValidated implementer history:\n${JSON.stringify(implementationHistory)}`,
    { agentType: 'zdev:zdev-verifier', label: 'zdev fresh verification' },
  ))?.trim()
  const result = parseWorkerResult(raw, 'verifier', area, taskId)
  const advisoryCount = result?.evidence.filter(item => item === advisoryText).length
  const approved = result?.verdict === 'pass' ? approvedPostValidation(result) : {}
  return result && approved && advisoryCount === (currentAdvisory ? 1 : 0) ? { raw, result, approved } : null
}

if (!implementation) {
  return blocker(area, taskId, 'implementation', 'implementer returned an invalid or mismatched envelope.', 'lifecycle and commit were not changed.', staleAdvisory)
}
if (implementation.verdict === 'blocker') {
  return blocker(area, taskId, 'implementation', implementation.summary, `Evidence: ${implementation.evidence.join('; ') || 'none.'} Findings: ${implementation.findings.join('; ') || 'none.'}`, staleAdvisory)
}
implementationHistory.push(implementation)

let current = await refresh('zdev pre-verification refresh')
if (typeof current === 'string') return current
let verdict = await verify(current)
if (!verdict) {
  return blocker(area, taskId, 'verification', 'verifier returned an invalid or mismatched envelope.', 'lifecycle and commit were not changed.', staleAdvisory)
}
while (verdict.result.verdict === 'rework') {
  if (verdict.result.escalation === 'advanced-implementer') {
    return blocker(area, taskId, 'rework', 'verifier requested an unavailable advanced implementer.', 'lifecycle and commit were not changed.', staleAdvisory)
  }
  current = await refresh('zdev rework refresh')
  if (typeof current === 'string') return current
  const reworkRaw = (await agent(
    `${workflowContract}\n\nCorrect every concrete task-owned finding for ${taskId}. Use the unchanged goal, current checkout, baseline, and full findings below. Return only the required strict JSON object with kind "implementer", area "${area}", and task_id "${taskId}".\n\nCurrent coordinator context:\n${current.raw}\n\nFindings:\n${verdict.raw}`,
    { agentType: 'zdev:zdev-implementer', label: 'zdev native rework' },
  ))?.trim()
  const rework = parseWorkerResult(reworkRaw, 'implementer', area, taskId)
  if (!rework) {
    return blocker(area, taskId, 'rework', 'implementer returned an invalid or mismatched envelope.', 'lifecycle and commit were not changed.', staleAdvisory)
  }
  if (rework.verdict === 'blocker') {
    return blocker(area, taskId, 'rework', rework.summary, `Evidence: ${rework.evidence.join('; ') || 'none.'} Findings: ${rework.findings.join('; ') || 'none.'}`, staleAdvisory)
  }
  implementationHistory.push(rework)
  current = await refresh('zdev post-rework verification refresh')
  if (typeof current === 'string') return current
  verdict = await verify(current)
  if (!verdict) {
    return blocker(area, taskId, 'verification', 'verifier returned an invalid or mismatched envelope.', 'lifecycle and commit were not changed.', staleAdvisory)
  }
}
if (verdict.result.verdict !== 'pass') {
  return blocker(area, taskId, 'verification', verdict.result.summary, verdict.result.evidence.join('; ') || 'lifecycle and commit were not changed.', staleAdvisory)
}

const advisory = staleAdvisory ? advisoryText : null
const completed = await agent(
  `${workflowContract}\n\nAct as the existing completion coordinator for verified task ${taskId} in area ${area}. Whether this completion is live or resumed, first run zdev work-context ${area} --format json yourself. Require its exact area and task_id, open/ready lifecycle and queue, safe nested status, and full HEAD ${verdict.approved.head}. Require its git_status, git_diff_cached, and git_diff strings to equal the verifier-approved post-validation strings below byte for byte. Any mismatch or malformed context blocks before mutation. On an exact match, run zdev task done, stage only attributed task-owned paths and exact task records, inspect the cached diff, and run zdev commit. Preserve the task-done and index state if staging, cached-diff inspection, or commit fails. Return PASS zdev-implement ${area} ${taskId} or BLOCKER zdev-implement ${area} ${taskId} as the exact first line. Repeat exact Area: ${area} and Task: ${taskId} fields. ${advisory ? `Include Advisory: ${advisory} exactly once, ` : 'Omit Advisory, '}plus Summary, Changed files, Validation, Verifier evidence, and Commit ID on pass, or Failed stage, Reason, and Preserved state on blocker.\n\nLatest coordinator context:\n${current.raw}\n\nVerifier-approved post-validation evidence:\n${JSON.stringify(verdict.approved)}\n\nValidated implementer history:\n${JSON.stringify(implementationHistory)}\n\nVerifier pass:\n${verdict.raw}`,
  { label: 'zdev completion and commit' },
)
const result = completed?.trim()
const first = result?.split('\n', 1)[0]
const exactSubject = field(result ?? '', 'Area') === area && field(result ?? '', 'Task') === taskId
const validPass = first === `PASS zdev-implement ${area} ${taskId}`
  && exactSubject
  && field(result, 'Advisory') === advisory
  && ['Summary', 'Changed files', 'Validation', 'Verifier evidence', 'Commit ID']
    .every(name => field(result, name) !== null)
const validBlocker = first === `BLOCKER zdev-implement ${area} ${taskId}`
  && exactSubject
  && field(result, 'Advisory') === advisory
  && ['Failed stage', 'Reason', 'Preserved state'].every(name => field(result, name) !== null)
return validPass || validBlocker
  ? result
  : blocker(area, taskId, 'completion and commit', 'coordinator returned an invalid or mismatched envelope.', 'inspect the checkout and zdev task record before continuing.', staleAdvisory)

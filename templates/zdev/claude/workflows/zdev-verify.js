export const meta = {
  name: 'zdev-verify',
  description: 'Independently verify the explicit current ready zdev task',
}

const repositoryGuidance = {{repository_guidance}}
const workerContract = repositoryGuidance
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
    if (character === '"' && depth > 0) inString = true
    else if (character === '{') {
      if (depth === 0) start = index
      depth += 1
    } else if (character === '}' && depth > 0) {
      depth -= 1
      if (depth === 0) {
        try {
          const value = JSON.parse(raw.slice(start, index + 1))
          if (value && !Array.isArray(value) && typeof value === 'object') {
            candidates.push({ value, raw: raw.slice(start, index + 1) })
          }
        } catch {}
      }
    }
  }
  return depth === 0 && candidates.length === 1 ? candidates[0] : null
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
  const decoded = decodeJsonObject(raw)
  if (!decoded) return null
  const keys = topLevelKeys(decoded.raw)
  if (!keys || new Set(keys).size !== keys.length) return null
  if (JSON.stringify([...keys].sort()) !== JSON.stringify(verifierResultKeys)) return null
  const result = decoded.value
  if (!result || Array.isArray(result) || typeof result !== 'object') return null
  if (['schema_version', 'kind', 'area', 'task_id', 'evidence'].some(key => Object.hasOwn(result, key))) return null
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
const parseStoredContext = raw => {
  const decoded = decodeJsonObject(raw)
  if (!decoded) return null
  const stored = decoded.value
  if (!stored || Array.isArray(stored) || typeof stored !== 'object') return null
  if (!['area', 'complexity', 'head', 'lifecycle', 'path', 'queue', 'schema_version', 'snapshot', 'stale_advisory', 'task_id']
    .every(key => Object.hasOwn(stored, key))) return null
  if (stored.schema_version !== 1 || stored.area !== area || stored.task_id !== taskId
    || stored.lifecycle !== 'open' || stored.queue !== 'ready'
    || !['routine', 'standard', 'advanced'].includes(stored.complexity)
    || !/^[0-9a-f]{40}$/.test(stored.head ?? '')
    || typeof stored.stale_advisory !== 'boolean') return null
  if (!/^W[0-9a-f]{16}$/.test(stored.snapshot ?? '')) return null
  return { snapshot: stored.snapshot, head: stored.head, staleAdvisory: stored.stale_advisory }
}
const parseComparison = (raw, expectedSnapshot) => {
  const decoded = decodeJsonObject(raw)
  if (!decoded) return null
  const result = decoded.value
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

const storedRaw = await agent(
  `Act only as read-only admission for task ${taskId} in area ${area}. Run zdev work-context ${area} --task ${taskId} --store --format json exactly once and return its JSON stdout. Do not show the snapshot. Keep files and Git state unchanged.`,
  { label: `zdev ${taskId}: capture verification snapshot`, model: 'haiku' },
)
const stored = parseStoredContext(storedRaw?.trim())
if (!stored) {
  return blocker(area, taskId, 'coordinator could not store and validate the admitted verification snapshot.')
}
const advisory = stored.staleAdvisory ? advisoryText : null

const verified = await agent(
  `${workerContract}\n\nIndependently verify task ${taskId} in area ${area}. Load immutable context with zdev work-context ${area} --show ${stored.snapshot} --format json and require the same ready task at HEAD ${stored.head}. Check the whole task and run required validation. Return exactly one JSON object with exactly these four keys and no others: verdict, summary, findings, escalation. Pass requires an empty findings array; rework requires at least one finding. Report each validation-written task-owned file as a validation_write: <repository-relative path> finding with verdict rework. Never add validation_writes or another fifth key. Never repair or discard validation writes.`,
  { agentType: 'zdev:zdev-verifier', label: `zdev ${taskId}: verify` },
)
const semantic = parseVerifierResult(verified?.trim())
const comparedRaw = await agent(
  `Act only as deterministic post-verification coordination. Run zdev work-context ${area} --compare ${stored.snapshot} --format json exactly once and return its complete JSON stdout unchanged, with no fence or other text. Keep files and Git state unchanged.`,
  { label: `zdev ${taskId}: confirm verifier left snapshot unchanged`, model: 'haiku' },
)
const compared = parseComparison(comparedRaw?.trim(), stored.snapshot)
if (!semantic || !compared || (!compared.equal && !reportsValidationWrite(semantic))) {
  return blocker(area, taskId, 'verifier output or post-validation comparison was invalid, contradictory, or changed ambiguously.', stored.staleAdvisory)
}
const result = publicVerifier(semantic, stored.snapshot, advisory)
return result
  ? JSON.stringify(result)
  : blocker(area, taskId, 'coordinator could not construct the public verifier envelope.', stored.staleAdvisory)

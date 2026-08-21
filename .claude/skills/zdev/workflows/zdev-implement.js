export const meta = {
  name: 'zdev-implement',
  description: 'Implement, independently verify, complete, and commit one ready zdev task',
}

const taskContract = "The coordinating session owns task selection, branch safety, Git ownership,\nlifecycle changes, and commits. Workers never edit `.zdev`, complete tasks,\ncommit, delegate, or change the selected task.\n\nBefore starting an implementer or verifier, run\n`zdev goal <area> --format json`. A validated closed goal is classified before\nGit or task-work gates: implement returns successful no-work, while explicit\nverify returns `BLOCKER zdev-verify`; neither starts a worker. For every open\ngoal, run `zdev status <area> --format json` and require\n`branch_status.task_work.safe` to be true. When\n`branch_status.task_work.stale_advisory` is true, report the advisory once and\ncontinue without requesting a rebase. Staleness alone is not a blocker. A\nfalse `safe` value blocks structurally unsafe branch, anchor, ancestry, linear\nhistory, or active Git-operation state. Capture the complete Git baseline with\n`git status --short --untracked-files=all`, `git diff --cached`, and `git diff`.\nKeep explicit evidence for all three results, including empty results, and\ninspect relevant untracked files. Stop on unexplained or overlapping changes\nor any user-owned decision.\n\nFor implement, open/empty and open/exhausted are successful no-work results\nafter the open-work gates above and start no worker. Explicit verify requires\nopen/ready and returns `BLOCKER zdev-verify` without starting a verifier for\nevery no-work result. Invalid records, task graphs, or goal output are\nblockers. For open/ready, retain the complete goal JSON\nunchanged and its task ID as the subject. Before verification and every rework\nhandoff, rerun status, the complete Git evidence, and goal; require the same\nready task ID.\n\n`zdev-implement <area>` gives the goal JSON, brief, task, repository guidance,\nbaseline, and task-owned paths to the configured `implementer`. Every\nimplementer and verifier returns only one JSON object, without a sentinel line,\nMarkdown fence, or other text. The object has exactly these keys:\n\n```json\n{\n  \"schema_version\": 1,\n  \"kind\": \"implementer\",\n  \"area\": \"<area>\",\n  \"task_id\": \"<task-id>\",\n  \"verdict\": \"ready\",\n  \"summary\": \"<non-empty summary>\",\n  \"evidence\": [],\n  \"findings\": [],\n  \"escalation\": \"none\"\n}\n```\n\n`kind` is `implementer` or `verifier`. Implementer verdict is `ready` or\n`blocker`; verifier verdict is `pass`, `rework`, or `blocker`. `summary` is a\nnon-empty string. `evidence` and `findings` are always arrays of non-empty\nstrings, including when empty. `escalation` is `none`, except that verifier\n`rework` may request `advanced-implementer`. Every other combination requires\n`none`. Schema version, kind, area, task ID, keys, types, and combinations must\nmatch exactly. Reject duplicate or unknown keys, missing keys, extra text, and\nmalformed JSON. Inspect the checkout after an implementer result, then use a\nfresh configured `verifier` for every verdict. When the stale advisory applies,\nthe verifier includes its exact text once in `evidence`; otherwise it omits it.\n\nEvery concrete task-owned verifier `rework` goes to the same implementer when the\nharness can resume it, or a replacement implementer with the unchanged goal,\nbaseline, current checkout, and full findings. There is no fixed rework count.\nAfter each correction, a fresh verifier checks the whole task again. Stop only\non verifier `pass`, a genuine blocker, unsafe scope expansion, or a required\nuser-owned decision. Do not silently send an `advanced-implementer` escalation\nto an ordinary implementer; stop if that role is unavailable.\n\nOnly after an exact matching verifier object with verdict `pass`, the coordinator runs\n`zdev task done`, stages only the attributed task-owned files and exact\ngenerated task records, inspects the staged diff, and runs `zdev commit`.\nCompletion or commit failure is a blocker that preserves and reports the exact\nstate. Public output begins with\n`PASS zdev-implement <area> <task-id>` or\n`BLOCKER zdev-implement <area> <task-id>`; its body repeats the exact area and\ntask, reports the stale advisory once when present, and names summary, changed\nfiles, validation, verifier evidence, and commit ID on pass, or the failed\nstage, reason, and preserved state on blocker. It omits the advisory field when\nno stale advisory was observed.\n\n`zdev-verify <area> <task-id>` performs the same read-only preflight and requires\nthe explicit ID to equal the current ready goal task before starting one fresh\nconfigured verifier. It never invokes an implementer, changes lifecycle state,\nstages, or commits. Its public result is the accepted verifier object above. Empty,\nexhausted, or closed goals, a different ready task, unsafe state, unavailable\nindependent verification, or an invalid worker envelope returns `BLOCKER zdev-verify`\nwithout mutation."
const repositoryGuidance = "<!-- zdev:generated-repository-guidance:start -->\n## Repository guidance discovery\n\nBefore planning or changing code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.\n<!-- zdev:generated-repository-guidance:end -->"
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
const expectedKeys = [
  'area',
  'git_diff',
  'git_diff_cached',
  'git_status',
  'goal_json',
  'status_json',
  'task_id',
]
const expectedNoWorkKeys = [
  'area',
  'git_diff',
  'git_diff_cached',
  'git_status',
  'goal_json',
  'lifecycle',
  'queue',
  'status_json',
]
const expectedClosedNoWorkKeys = [
  'area',
  'goal_json',
  'lifecycle',
  'queue',
]
const parseReady = (raw, workflow, expectedArea, expectedTask = null) => {
  if (typeof raw !== 'string') return null
  const newline = raw.indexOf('\n')
  if (newline < 0) return null
  const first = raw.slice(0, newline)
  const match = first.match(new RegExp(`^READY ${workflow} ([a-z0-9][a-z0-9-]*) ([a-z0-9][a-z0-9-]*)$`))
  if (!match || match[1] !== expectedArea || (expectedTask && match[2] !== expectedTask)) return null
  let payload
  try {
    payload = JSON.parse(raw.slice(newline + 1))
  } catch {
    return null
  }
  if (!payload || Array.isArray(payload) || typeof payload !== 'object') return null
  if (JSON.stringify(Object.keys(payload).sort()) !== JSON.stringify(expectedKeys)) return null
  if (payload.area !== match[1] || payload.task_id !== match[2]) return null
  for (const key of ['status_json', 'goal_json', 'git_status', 'git_diff_cached', 'git_diff']) {
    if (typeof payload[key] !== 'string') return null
  }
  if (!payload.status_json || !payload.goal_json) return null
  let status
  let goal
  try {
    status = JSON.parse(payload.status_json)
    goal = JSON.parse(payload.goal_json)
  } catch {
    return null
  }
  const taskWork = status?.branch_status?.task_work
  if (taskWork?.safe !== true || typeof taskWork.stale_advisory !== 'boolean') return null
  if (status?.area?.tag !== match[1] || status?.next !== match[2]) return null
  if (goal?.lifecycle !== 'open' || goal?.queue !== 'ready' || goal?.area?.tag !== match[1] || goal?.task?.id !== match[2]) return null
  return { raw, taskId: match[2], staleAdvisory: taskWork.stale_advisory }
}
const parseNoWork = raw => {
  if (typeof raw !== 'string') return null
  const newline = raw.indexOf('\n')
  if (newline < 0) return null
  const match = raw.slice(0, newline).match(new RegExp(`^NO-WORK zdev-implement ${area} (open|closed) (empty|exhausted)$`))
  if (!match) return null
  let payload
  try {
    payload = JSON.parse(raw.slice(newline + 1))
  } catch {
    return null
  }
  if (!payload || Array.isArray(payload) || typeof payload !== 'object') return null
  if (payload.area !== area || payload.lifecycle !== match[1] || payload.queue !== match[2]) return null
  if (match[1] === 'closed') {
    if (JSON.stringify(Object.keys(payload).sort()) !== JSON.stringify(expectedClosedNoWorkKeys)) return null
    if (typeof payload.goal_json !== 'string' || !payload.goal_json) return null
    let goal
    try {
      goal = JSON.parse(payload.goal_json)
    } catch {
      return null
    }
    if (goal?.lifecycle !== 'closed' || goal?.queue !== match[2] || goal?.area?.tag !== area || goal?.task !== null) return null
    return { lifecycle: 'closed', queue: match[2], staleAdvisory: false }
  }
  if (JSON.stringify(Object.keys(payload).sort()) !== JSON.stringify(expectedNoWorkKeys)) return null
  for (const key of ['status_json', 'goal_json', 'git_status', 'git_diff_cached', 'git_diff']) {
    if (typeof payload[key] !== 'string') return null
  }
  if (!payload.status_json || !payload.goal_json) return null
  let status
  let goal
  try {
    status = JSON.parse(payload.status_json)
    goal = JSON.parse(payload.goal_json)
  } catch {
    return null
  }
  const taskWork = status?.branch_status?.task_work
  if (taskWork?.safe !== true || typeof taskWork.stale_advisory !== 'boolean') return null
  if (status?.area?.tag !== area || status?.next !== null) return null
  if (status?.lifecycle !== match[1] || status?.queue !== match[2]) return null
  if (goal?.lifecycle !== match[1] || goal?.queue !== match[2] || goal?.area?.tag !== area || goal?.task !== null) return null
  return { lifecycle: match[1], queue: match[2], staleAdvisory: taskWork.stale_advisory }
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
  `${workflowContract}\n\nAct only as the coordinating preflight for area ${area}. Run zdev goal ${area} --format json first. For a validated closed goal return exactly, without inspecting Git or task-work status:\nNO-WORK zdev-implement ${area} closed <empty-or-exhausted>\n<one JSON object with exactly area, lifecycle, queue, and goal_json; goal_json is the complete command JSON bytes encoded as a string>.\nFor an open goal, run zdev status ${area} --format json and require branch_status.task_work.safe to be true. If stale_advisory is true, retain it and continue. Capture git status --short --untracked-files=all, git diff --cached, and git diff as explicit strings, including empty results. Do not change files or start another worker. For ready work return exactly:\nREADY zdev-implement ${area} <task-id>\n<one JSON object with exactly area, task_id, status_json, goal_json, git_status, git_diff_cached, and git_diff; status_json and goal_json are the complete command JSON bytes encoded as strings>.\nFor open no-work return exactly:\nNO-WORK zdev-implement ${area} open <empty-or-exhausted>\n<one JSON object with exactly area, lifecycle, queue, status_json, goal_json, git_status, git_diff_cached, and git_diff, with complete command JSON bytes encoded as strings>.\nOtherwise return a blocker explanation.`,
  { label },
)

const preparedRaw = (await preflight('zdev implement preflight'))?.trim()
const noWork = parseNoWork(preparedRaw)
if (noWork) {
  return `PASS zdev-implement ${area} none\n\nArea: ${area}\nTask: none\n${noWork.staleAdvisory ? `Advisory: ${advisoryText}\n` : ''}Summary: no ready work; ${noWork.lifecycle}/${noWork.queue} goal.\nChanged files: none.\nValidation: preflight only.\nVerifier evidence: no implementer or verifier was started.\nCommit ID: none.`
}
const prepared = parseReady(preparedRaw, 'zdev-implement', area)
if (!prepared) {
  return blocker(area, 'unknown', 'preflight', 'missing or invalid ready/no-work goal, branch safety, or complete Git baseline evidence.', 'no implementer or verifier was started.')
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
  const current = parseReady((await preflight(label))?.trim(), 'zdev-implement', area, taskId)
  if (current?.staleAdvisory) staleAdvisory = true
  return current ?? blocker(area, taskId, 'goal refresh', `expected ready task ${taskId} with complete status, goal, and Git evidence.`, 'lifecycle and commit were not changed.', staleAdvisory)
}
const verify = async current => {
  const currentAdvisory = current.staleAdvisory ? advisoryText : null
  const raw = (await agent(
    `${workflowContract}\n\nIndependently verify task ${taskId} in area ${area}. Use the complete implementer history only to locate evidence. Check the whole task and current checkout, run required validation, and return only the required strict JSON object with kind "verifier", area "${area}", and task_id "${taskId}". ${currentAdvisory ? `Include ${currentAdvisory} exactly once in evidence.` : `Do not include ${advisoryText} in evidence.`} Make no intentional edits.\n\nCurrent coordinator context:\n${current.raw}\n\nValidated implementer history:\n${JSON.stringify(implementationHistory)}`,
    { agentType: 'zdev:zdev-verifier', label: 'zdev fresh verification' },
  ))?.trim()
  const result = parseWorkerResult(raw, 'verifier', area, taskId)
  const advisoryCount = result?.evidence.filter(item => item === advisoryText).length
  return result && advisoryCount === (currentAdvisory ? 1 : 0) ? { raw, result } : null
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
  `${workflowContract}\n\nAct as the coordinator for verified task ${taskId} in area ${area}. Recheck the same structured ready envelope and Git ownership. Only if they match, run zdev task done, stage only attributed task-owned paths and exact task records, inspect the cached diff, and run zdev commit. Return PASS zdev-implement ${area} ${taskId} or BLOCKER zdev-implement ${area} ${taskId} as the exact first line. Repeat exact Area: ${area} and Task: ${taskId} fields. ${advisory ? `Include Advisory: ${advisory} exactly once, ` : 'Omit Advisory, '}plus Summary, Changed files, Validation, Verifier evidence, and Commit ID on pass, or Failed stage, Reason, and Preserved state on blocker.\n\nPreflight:\n${prepared.raw}\n\nValidated implementer history:\n${JSON.stringify(implementationHistory)}\n\nVerifier pass:\n${verdict.raw}`,
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

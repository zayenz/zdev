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
  if (JSON.stringify(Object.keys(payload).sort()) !== JSON.stringify(expectedNoWorkKeys)) return null
  if (payload.area !== area || payload.lifecycle !== match[1] || payload.queue !== match[2]) return null
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
const exactWorkerEnvelope = (text, verdicts, workflow, expectedArea, expectedTask) => {
  if (typeof text !== 'string') return false
  const first = text.split('\n', 1)[0]
  const validFirst = verdicts.some(verdict => first === `${verdict} ${workflow} ${expectedArea} ${expectedTask}`)
  return validFirst
    && field(text, 'Area') === expectedArea
    && field(text, 'Task') === expectedTask
}

if (!/^[a-z0-9][a-z0-9-]*$/.test(area)) {
  return blocker('unknown', 'unknown', 'input', 'a lowercase area is required.', 'no preflight or worker was started.')
}

const preflight = async label => agent(
  `${workflowContract}\n\nAct only as the coordinating preflight for area ${area}. Run zdev status ${area} --format json and require branch_status.task_work.safe to be true. If stale_advisory is true, retain it and continue. Capture git status --short --untracked-files=all, git diff --cached, and git diff as explicit strings, including empty results. Run zdev goal ${area} --format json. Do not change files or start another worker. For ready work require lifecycle open and queue ready, then return exactly:\nREADY zdev-implement ${area} <task-id>\n<one JSON object with exactly area, task_id, status_json, goal_json, git_status, git_diff_cached, and git_diff; status_json and goal_json are the complete command JSON bytes encoded as strings>.\nFor no work return exactly:\nNO-WORK zdev-implement ${area} <open-or-closed> <empty-or-exhausted>\n<one JSON object with exactly area, lifecycle, queue, status_json, goal_json, git_status, git_diff_cached, and git_diff, with complete command JSON bytes encoded as strings>.\nOtherwise return a blocker explanation.`,
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

let implementation = await agent(
  `${workflowContract}\n\nImplement the ready task ${taskId} in area ${area}. Use the complete coordinator context below. Change only task-owned source and tests, run required validation, and return DONE implementer ${area} ${taskId} or BLOCKER implementer ${area} ${taskId}, with exact Area and Task fields plus Changed files, Validation, and Blockers.\n\nCoordinator context:\n${prepared.raw}`,
  { agentType: 'zdev:zdev-implementer', label: 'zdev implementation' },
)

const refresh = async label => {
  const current = parseReady((await preflight(label))?.trim(), 'zdev-implement', area, taskId)
  if (current?.staleAdvisory) staleAdvisory = true
  return current ?? blocker(area, taskId, 'goal refresh', `expected ready task ${taskId} with complete status, goal, and Git evidence.`, 'lifecycle and commit were not changed.', staleAdvisory)
}
const verify = async (current, implementationContext) => {
  const currentAdvisory = current.staleAdvisory ? advisoryText : null
  const verdict = await agent(
    `${workflowContract}\n\nIndependently verify task ${taskId} in area ${area}. Use the implementer text only to locate evidence. Check the whole task and current checkout, run required validation, and return exactly PASS zdev-verify ${area} ${taskId}, REWORK zdev-verify ${area} ${taskId}, or BLOCKER zdev-verify ${area} ${taskId}, with exact Area and Task fields, ${currentAdvisory ? `Advisory: ${currentAdvisory} exactly once, ` : 'no Advisory field, '}Summary, Validation, and Located evidence. Make no intentional edits.\n\nCurrent coordinator context:\n${current.raw}\n\nImplementer result:\n${implementationContext}`,
    { agentType: 'zdev:zdev-verifier', label: 'zdev fresh verification' },
  )
  return exactWorkerEnvelope(verdict?.trim(), ['PASS', 'REWORK', 'BLOCKER'], 'zdev-verify', area, taskId)
    && ['Summary', 'Validation', 'Located evidence'].every(name => field(verdict.trim(), name) !== null)
    && field(verdict.trim(), 'Advisory') === currentAdvisory
    ? verdict.trim()
    : `BLOCKER zdev-verify ${area} ${taskId}\n\nArea: ${area}\nTask: ${taskId}\n${staleAdvisory ? `Advisory: ${advisoryText}\n` : ''}Summary: verifier returned an invalid or mismatched envelope.\nValidation: not accepted.\nLocated evidence: raw result follows.\n\n${verdict?.trim() ?? ''}`
}
const validImplementer = result =>
  exactWorkerEnvelope(result?.trim(), ['DONE'], 'implementer', area, taskId)
  && ['Changed files', 'Validation', 'Blockers'].every(name => field(result.trim(), name) !== null)

if (!validImplementer(implementation)) {
  return blocker(area, taskId, 'implementation', 'implementer returned an invalid or mismatched envelope.', 'lifecycle and commit were not changed.', staleAdvisory)
}

let current = await refresh('zdev pre-verification refresh')
if (typeof current === 'string') return current
let verdict = await verify(current, implementation)
while (verdict.split('\n', 1)[0] === `REWORK zdev-verify ${area} ${taskId}`) {
  current = await refresh('zdev rework refresh')
  if (typeof current === 'string') return current
  const rework = await agent(
    `${workflowContract}\n\nCorrect every concrete task-owned finding for ${taskId}. Use the unchanged goal, current checkout, baseline, and full findings below. Return DONE implementer ${area} ${taskId} with exact Area and Task fields plus Changed files, Validation, and Blockers.\n\nCurrent coordinator context:\n${current.raw}\n\nFindings:\n${verdict}`,
    { agentType: 'zdev:zdev-implementer', label: 'zdev native rework' },
  )
  if (!validImplementer(rework)) {
    return blocker(area, taskId, 'rework', 'implementer returned an invalid or mismatched envelope.', 'lifecycle and commit were not changed.', staleAdvisory)
  }
  implementation = `${implementation}\n\nRework:\n${rework}`
  current = await refresh('zdev post-rework verification refresh')
  if (typeof current === 'string') return current
  verdict = await verify(current, implementation)
}
if (verdict.split('\n', 1)[0] !== `PASS zdev-verify ${area} ${taskId}`) {
  return blocker(area, taskId, 'verification', 'independent verification did not pass.', 'lifecycle and commit were not changed.', staleAdvisory)
}

const advisory = staleAdvisory ? advisoryText : null
const completed = await agent(
  `${workflowContract}\n\nAct as the coordinator for verified task ${taskId} in area ${area}. Recheck the same structured ready envelope and Git ownership. Only if they match, run zdev task done, stage only attributed task-owned paths and exact task records, inspect the cached diff, and run zdev commit. Return PASS zdev-implement ${area} ${taskId} or BLOCKER zdev-implement ${area} ${taskId} as the exact first line. Repeat exact Area: ${area} and Task: ${taskId} fields. ${advisory ? `Include Advisory: ${advisory} exactly once, ` : 'Omit Advisory, '}plus Summary, Changed files, Validation, Verifier evidence, and Commit ID on pass, or Failed stage, Reason, and Preserved state on blocker.\n\nPreflight:\n${prepared.raw}\n\nImplementation evidence:\n${implementation}\n\nVerifier PASS:\n${verdict}`,
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

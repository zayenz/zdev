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
const field = (text, name) => {
  const matches = text.split('\n').filter(line => line.startsWith(`${name}: `))
  return matches.length === 1 ? matches[0].slice(name.length + 2) : null
}
const advisoryText = 'stale effective-base link; managed rebase remains optional.'
const blocker = (subjectArea, subjectTask, reason, staleAdvisory = false) =>
  `BLOCKER zdev-verify ${subjectArea} ${subjectTask}\n\nArea: ${subjectArea}\nTask: ${subjectTask}\n${staleAdvisory ? `Advisory: ${advisoryText}\n` : ''}Summary: ${reason}\nValidation: not accepted.\nLocated evidence: no verifier result was accepted.`
const expectedKeys = [
  'area',
  'git_diff',
  'git_diff_cached',
  'git_status',
  'goal_json',
  'status_json',
  'task_id',
]
const parseReady = raw => {
  if (typeof raw !== 'string') return null
  const newline = raw.indexOf('\n')
  if (newline < 0 || raw.slice(0, newline) !== `READY zdev-verify ${area} ${taskId}`) return null
  let payload
  try {
    payload = JSON.parse(raw.slice(newline + 1))
  } catch {
    return null
  }
  if (!payload || Array.isArray(payload) || typeof payload !== 'object') return null
  if (JSON.stringify(Object.keys(payload).sort()) !== JSON.stringify(expectedKeys)) return null
  if (payload.area !== area || payload.task_id !== taskId) return null
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
  if (status?.area?.tag !== area || status?.next !== taskId) return null
  if (goal?.lifecycle !== 'open' || goal?.queue !== 'ready' || goal?.area?.tag !== area || goal?.task?.id !== taskId) return null
  return { raw, staleAdvisory: taskWork.stale_advisory }
}

if (!/^[a-z0-9][a-z0-9-]*$/.test(area) || !/^[a-z0-9][a-z0-9-]*$/.test(taskId)) {
  return blocker('unknown', 'unknown', 'a lowercase area and explicit task ID are required.')
}

const preflight = await agent(
  `${workflowContract}\n\nAct only as the coordinating read-only preflight. Run zdev goal ${area} --format json first. If the validated lifecycle is closed, return a blocker explanation without inspecting Git or task-work status. For an open goal, run zdev status ${area} --format json and require branch_status.task_work.safe to be true; retain stale_advisory and continue when it is true. Capture git status --short --untracked-files=all, git diff --cached, and git diff as explicit strings, including empty results, then require ready task ${taskId} exactly. Do not change files or start another worker. Return exactly:\nREADY zdev-verify ${area} ${taskId}\n<one JSON object with exactly area, task_id, status_json, goal_json, git_status, git_diff_cached, and git_diff; status_json and goal_json are the complete command JSON bytes encoded as strings>.`,
  { label: 'zdev verify preflight' },
)
const prepared = parseReady(preflight?.trim())
if (!prepared) {
  return blocker(area, taskId, 'missing or invalid ready goal, requested task match, branch safety, or complete Git baseline evidence.')
}
const advisory = prepared.staleAdvisory ? advisoryText : null

const verified = await agent(
  `${workflowContract}\n\nIndependently verify task ${taskId} in area ${area} from the current checkout. Check the whole task, run required validation, compare Git state before and after, and return exactly PASS zdev-verify ${area} ${taskId}, REWORK zdev-verify ${area} ${taskId}, or BLOCKER zdev-verify ${area} ${taskId}. Repeat exact Area: ${area} and Task: ${taskId} fields and ${advisory ? `include Advisory: ${advisory} exactly once` : 'omit Advisory'}, plus Summary, Validation, and Located evidence. Make no intentional edits and never change lifecycle or Git state.\n\nCoordinator context:\n${prepared.raw}`,
  { agentType: 'zdev:zdev-verifier', label: 'zdev fresh verification' },
)
const result = verified?.trim()
const first = result?.split('\n', 1)[0]
const validFirst = ['PASS', 'REWORK', 'BLOCKER']
  .some(verdict => first === `${verdict} zdev-verify ${area} ${taskId}`)
const valid = validFirst
  && field(result, 'Area') === area
  && field(result, 'Task') === taskId
  && field(result, 'Advisory') === advisory
  && ['Summary', 'Validation', 'Located evidence'].every(name => field(result, name) !== null)
return valid
  ? result
  : blocker(area, taskId, 'verifier returned an invalid, suffixed, or mismatched envelope.', prepared.staleAdvisory)

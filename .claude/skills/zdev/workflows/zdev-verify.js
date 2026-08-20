export const meta = {
  name: 'zdev-verify',
  description: 'Independently verify the explicit current ready zdev task',
}

const taskContract = "The coordinating session owns task selection, branch safety, Git ownership,\nlifecycle changes, and commits. Workers never edit `.zdev`, complete tasks,\ncommit, delegate, or change the selected task.\n\nBefore starting an implementer or verifier, run\n`zdev status <area> --format json` and require\n`branch_status.task_work.safe` to be true. When\n`branch_status.task_work.stale_advisory` is true, report the advisory once and\ncontinue without requesting a rebase. Staleness alone is not a blocker. A\nfalse `safe` value blocks structurally unsafe branch, anchor, ancestry, linear\nhistory, or active Git-operation state. Capture the complete Git baseline with\n`git status --short --untracked-files=all`, `git diff --cached`, and `git diff`.\nKeep explicit evidence for all three results, including empty results, and\ninspect relevant untracked files. Stop on unexplained or overlapping changes\nor any user-owned decision.\n\nRun `zdev goal <area> --format json`. `empty` and `complete` are successful\nno-work results and start no worker. Invalid records, task graphs, or goal\noutput are blockers. For `ready`, retain the complete goal JSON unchanged and\nits task ID as the subject. Before verification and every rework handoff, rerun\nstatus, the complete Git evidence, and goal; require the same ready task ID.\n\n`zdev-implement <area>` gives the goal JSON, brief, task, repository guidance,\nbaseline, and task-owned paths to the configured `implementer`. Its internal\nfirst line is `DONE implementer <area> <task-id>` or\n`BLOCKER implementer <area> <task-id>`. Inspect the checkout,\nthen use a fresh configured `verifier` for every verdict. A verifier returns\nexactly `PASS zdev-verify <area> <task-id>`,\n`REWORK zdev-verify <area> <task-id>`, or\n`BLOCKER zdev-verify <area> <task-id>` and includes exact `Area` and `Task`\nfields, the stale advisory once when present, summary, validation, and located\nevidence. Omit the advisory field when there is no stale advisory. Missing\noutput, a mismatched subject, a suffixed first line, or any other first line is\na blocker.\n\nEvery concrete task-owned `REWORK` goes to the same implementer when the\nharness can resume it, or a replacement implementer with the unchanged goal,\nbaseline, current checkout, and full findings. There is no fixed rework count.\nAfter each correction, a fresh verifier checks the whole task again. Stop only\non `PASS`, a genuine blocker, unsafe scope expansion, or a required user-owned\ndecision.\n\nOnly after the exact matching `PASS zdev-verify` envelope, the coordinator runs\n`zdev task done`, stages only the attributed task-owned files and exact\ngenerated task records, inspects the staged diff, and runs `zdev commit`.\nCompletion or commit failure is a blocker that preserves and reports the exact\nstate. Public output begins with\n`PASS zdev-implement <area> <task-id>` or\n`BLOCKER zdev-implement <area> <task-id>`; its body repeats the exact area and\ntask, reports the stale advisory once when present, and names summary, changed\nfiles, validation, verifier evidence, and commit ID on pass, or the failed\nstage, reason, and preserved state on blocker. It omits the advisory field when\nno stale advisory was observed.\n\n`zdev-verify <area> <task-id>` performs the same read-only preflight and requires\nthe explicit ID to equal the current ready goal task before starting one fresh\nconfigured verifier. It never invokes an implementer, changes lifecycle state,\nstages, or commits. Its public result is the verifier envelope above. Empty or\ncomplete goals, a different ready task, unsafe state, unavailable independent\nverification, or an invalid worker envelope returns `BLOCKER zdev-verify`\nwithout mutation."
const repositoryGuidance = "<!-- zdev:generated-repository-guidance:start -->\n## Repository guidance discovery\n\nBefore planning or changing code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.\n<!-- zdev:generated-repository-guidance:end -->"
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
  if (goal?.state !== 'ready' || goal?.area?.tag !== area || goal?.task?.id !== taskId) return null
  return { raw, staleAdvisory: taskWork.stale_advisory }
}

if (!/^[a-z0-9][a-z0-9-]*$/.test(area) || !/^[a-z0-9][a-z0-9-]*$/.test(taskId)) {
  return blocker('unknown', 'unknown', 'a lowercase area and explicit task ID are required.')
}

const preflight = await agent(
  `${workflowContract}\n\nAct only as the coordinating read-only preflight. Run zdev status ${area} --format json and require branch_status.task_work.safe to be true; retain stale_advisory and continue when it is true. Capture git status --short --untracked-files=all, git diff --cached, and git diff as explicit strings, including empty results. Run zdev goal ${area} --format json and require ready task ${taskId} exactly. Do not change files or start another worker. Return exactly:\nREADY zdev-verify ${area} ${taskId}\n<one JSON object with exactly area, task_id, status_json, goal_json, git_status, git_diff_cached, and git_diff; status_json and goal_json are the complete command JSON bytes encoded as strings>.`,
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

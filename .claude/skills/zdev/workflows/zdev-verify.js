export const meta = {
  name: 'zdev-verify',
  description: 'Independently verify the explicit current ready zdev task',
}

const taskContract = "The coordinating session owns task selection, branch safety, Git ownership,\nlifecycle changes, and commits. Workers never edit `.zdev`, complete tasks,\ncommit, delegate, or change the selected task.\n\nBefore starting an implementer or verifier, run\n`zdev goal <area> --format json`. A validated closed goal is classified before\nGit or task-work gates: implement returns successful no-work, while explicit\nverify returns `BLOCKER zdev-verify`; neither starts a worker. For every open\ngoal, run `zdev status <area> --format json` and require\n`branch_status.task_work.safe` to be true. When\n`branch_status.task_work.stale_advisory` is true, report the advisory once and\ncontinue without requesting a rebase. Staleness alone is not a blocker. A\nfalse `safe` value blocks structurally unsafe branch, anchor, ancestry, linear\nhistory, or active Git-operation state. Capture the complete Git baseline with\n`git status --short --untracked-files=all`, `git diff --cached`, and `git diff`.\nKeep explicit evidence for all three results, including empty results, and\ninspect relevant untracked files. Stop on unexplained or overlapping changes\nor any user-owned decision.\n\nFor implement, open/empty and open/exhausted are successful no-work results\nafter the open-work gates above and start no worker. Explicit verify requires\nopen/ready and returns `BLOCKER zdev-verify` without starting a verifier for\nevery no-work result. Invalid records, task graphs, or goal output are\nblockers. For open/ready, retain the complete goal JSON\nunchanged and its task ID as the subject. Before verification and every rework\nhandoff, rerun status, the complete Git evidence, and goal; require the same\nready task ID.\n\n`zdev-implement <area>` gives the goal JSON, brief, task, repository guidance,\nbaseline, and task-owned paths to the configured `implementer`. Every\nimplementer and verifier returns only one JSON object, without a sentinel line,\nMarkdown fence, or other text. The object has exactly these keys:\n\n```json\n{\n  \"schema_version\": 1,\n  \"kind\": \"implementer\",\n  \"area\": \"<area>\",\n  \"task_id\": \"<task-id>\",\n  \"verdict\": \"ready\",\n  \"summary\": \"<non-empty summary>\",\n  \"evidence\": [],\n  \"findings\": [],\n  \"escalation\": \"none\"\n}\n```\n\n`kind` is `implementer` or `verifier`. Implementer verdict is `ready` or\n`blocker`; verifier verdict is `pass`, `rework`, or `blocker`. `summary` is a\nnon-empty string. `evidence` and `findings` are always arrays of non-empty\nstrings, including when empty. `escalation` is `none`, except that verifier\n`rework` may request `advanced-implementer`. Every other combination requires\n`none`. Schema version, kind, area, task ID, keys, types, and combinations must\nmatch exactly. Reject duplicate or unknown keys, missing keys, extra text, and\nmalformed JSON. Inspect the checkout after an implementer result, then use a\nfresh configured `verifier` for every verdict. When the stale advisory applies,\nthe verifier includes its exact text once in `evidence`; otherwise it omits it.\n\nEvery concrete task-owned verifier `rework` goes to the same implementer when the\nharness can resume it, or a replacement implementer with the unchanged goal,\nbaseline, current checkout, and full findings. There is no fixed rework count.\nAfter each correction, a fresh verifier checks the whole task again. Stop only\non verifier `pass`, a genuine blocker, unsafe scope expansion, or a required\nuser-owned decision. Do not silently send an `advanced-implementer` escalation\nto an ordinary implementer; stop if that role is unavailable.\n\nOnly after an exact matching verifier object with verdict `pass`, the coordinator runs\n`zdev task done`, stages only the attributed task-owned files and exact\ngenerated task records, inspects the staged diff, and runs `zdev commit`.\nCompletion or commit failure is a blocker that preserves and reports the exact\nstate. Public output begins with\n`PASS zdev-implement <area> <task-id>` or\n`BLOCKER zdev-implement <area> <task-id>`; its body repeats the exact area and\ntask, reports the stale advisory once when present, and names summary, changed\nfiles, validation, verifier evidence, and commit ID on pass, or the failed\nstage, reason, and preserved state on blocker. It omits the advisory field when\nno stale advisory was observed.\n\n`zdev-verify <area> <task-id>` performs the same read-only preflight and requires\nthe explicit ID to equal the current ready goal task before starting one fresh\nconfigured verifier. It never invokes an implementer, changes lifecycle state,\nstages, or commits. Its public result is the accepted verifier object above. Empty,\nexhausted, or closed goals, a different ready task, unsafe state, unavailable\nindependent verification, or an invalid worker envelope returns `BLOCKER zdev-verify`\nwithout mutation."
const repositoryGuidance = "<!-- zdev:generated-repository-guidance:start -->\n## Repository guidance discovery\n\nBefore planning or changing code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.\n<!-- zdev:generated-repository-guidance:end -->"
const workflowContract = [taskContract, repositoryGuidance].join('\n\n')
const input = args ?? {}
const area = String(input.area ?? '').trim()
const taskId = String(input.task_id ?? input.taskId ?? '').trim()
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
  `${workflowContract}\n\nIndependently verify task ${taskId} in area ${area} from the current checkout. Check the whole task, run required validation, compare Git state before and after, and return only the required strict JSON object with kind "verifier", area "${area}", and task_id "${taskId}". ${advisory ? `Include ${advisory} exactly once in evidence.` : `Do not include ${advisoryText} in evidence.`} Make no intentional edits and never change lifecycle or Git state.\n\nCoordinator context:\n${prepared.raw}`,
  { agentType: 'zdev:zdev-verifier', label: 'zdev fresh verification' },
)
const result = verified?.trim()
const parsed = parseWorkerResult(result, 'verifier', area, taskId)
const advisoryCount = parsed?.evidence.filter(item => item === advisoryText).length
return parsed && advisoryCount === (advisory ? 1 : 0)
  ? result
  : blocker(area, taskId, 'verifier returned invalid, extra, contradictory, or mismatched JSON.', prepared.staleAdvisory)

export const meta = {
  name: 'zdev-verify',
  description: 'Independently verify the explicit current ready zdev task',
}

const taskContract = "The coordinating session owns task selection, branch safety, Git ownership,\nlifecycle changes, and commits. Workers never edit `.zdev`, complete tasks,\ncommit, delegate, or change the selected task.\n\nBefore starting an implementer or verifier, run\n`zdev work-context <area> --format json` and retain the complete result. The\ncommand classifies goal lifecycle first. A validated closed context contains\nno status or Git evidence: implement returns successful no-work, while\nexplicit verify returns `BLOCKER zdev-verify`; neither starts a worker. Every\nopen context contains matching nested status and goal projections, a boolean\n`stale_advisory`, a full lowercase `head` commit ID, and exact `git_status`,\n`git_diff_cached`, and `git_diff` strings. Require the projected area,\nlifecycle, queue, and task ID to agree and task work to be safe. Report a true stale advisory once and continue without\nrequesting a rebase. Inspect relevant untracked files, and stop on unexplained\nor overlapping changes or any user-owned decision.\n\nFor implement, open/empty and open/exhausted are successful no-work results\nafter the open-work gates above and start no worker. Explicit verify requires\nopen/ready and returns `BLOCKER zdev-verify` without starting a verifier for\nevery no-work result. Invalid records, task graphs, or context output are\nblockers. For open/ready, retain the complete context unchanged and its task ID\nas the subject. Before verification and every rework handoff, rerun\n`work-context` and require the same ready task ID and an explainable exact Git\ndelta.\n\n`zdev-implement <area>` gives the complete work-context JSON, brief, task, repository guidance,\nbaseline, and task-owned paths to the configured `implementer`. Every\nimplementer and verifier returns only one JSON object, without a sentinel line,\nMarkdown fence, or other text. The object has exactly these keys:\n\n```json\n{\n  \"schema_version\": 1,\n  \"kind\": \"implementer\",\n  \"area\": \"<area>\",\n  \"task_id\": \"<task-id>\",\n  \"verdict\": \"ready\",\n  \"summary\": \"<non-empty summary>\",\n  \"evidence\": [],\n  \"findings\": [],\n  \"escalation\": \"none\"\n}\n```\n\n`kind` is `implementer` or `verifier`. Implementer verdict is `ready` or\n`blocker`; verifier verdict is `pass`, `rework`, or `blocker`. `summary` is a\nnon-empty string. `evidence` and `findings` are always arrays of non-empty\nstrings, including when empty. `escalation` is `none`, except that verifier\n`rework` may request `advanced-implementer`. Every other combination requires\n`none`. Schema version, kind, area, task ID, keys, types, and combinations must\nmatch exactly. Reject duplicate or unknown keys, missing keys, extra text, and\nmalformed JSON. Inspect the checkout after an implementer result, then use a\nfresh configured `verifier` for every verdict. When the stale advisory applies,\nthe verifier includes its exact text once in `evidence`; otherwise it omits it.\n\nEvery verifier independently runs\n`zdev work-context <area> --format json` before inspecting or validating. It\nrequires the same open, ready, safe area and task, compares that fresh context\nwith the coordinator context only to detect intervening state, then runs the\nrequired validation. After validation it reruns `git status\n--short --untracked-files=all`, `git diff --cached`, and `git diff` and reports\nany change. On `pass`, its evidence contains exactly one `HEAD: <full-lowercase-id>`\nentry copied from its independent context and exactly one `git_status:\n<json-string>`, `git_diff_cached: <json-string>`, and `git_diff:\n<json-string>` entry. Each JSON string encodes the exact post-validation\nstdout, including empty output. These four entries let the coordinator compare\nidentity, index, worktree, and untracked state before mutation. Coordinator\ncontext is a locator, never the verifier's evidence.\n\nEvery concrete task-owned verifier `rework` goes to the same implementer when the\nharness can resume it, or a replacement implementer with the unchanged goal,\nbaseline, current checkout, and full findings. There is no fixed rework count.\nAfter each correction, a fresh verifier checks the whole task again. Stop only\non verifier `pass`, a genuine blocker, unsafe scope expansion, or a required\nuser-owned decision. Do not silently send an `advanced-implementer` escalation\nto an ordinary implementer; stop if that role is unavailable.\n\nOnly after an exact matching verifier object with verdict `pass`, the\ncoordinator compares the accepted post-validation area, task, lifecycle,\nsafety, HEAD, staged diff, unstaged diff, and untracked evidence with the\nlatest context. Claude performs this comparison by running a fresh\n`work-context` inside its existing completion agent; no additional worker is\nstarted. On a match, the coordinator runs `zdev task done`, stages only the\nattributed task-owned files and exact generated task records, inspects the\nstaged diff, and runs `zdev commit`.\nCompletion or commit failure is a blocker that preserves and reports the exact\nstate. Public output begins with\n`PASS zdev-implement <area> <task-id>` or\n`BLOCKER zdev-implement <area> <task-id>`; its body repeats the exact area and\ntask, reports the stale advisory once when present, and names summary, changed\nfiles, validation, verifier evidence, and commit ID on pass, or the failed\nstage, reason, and preserved state on blocker. It omits the advisory field when\nno stale advisory was observed.\n\n`zdev-implement` completes one task. After reporting its verified commit, it\nstops without querying `zdev next` or another `work-context`. A goal, loop, or\nexplicit continuation owns the next iteration and must collect a fresh\n`zdev work-context <area> --format json` after the commit and before another\nworker dispatch. It never reuses the completed task's pre-commit selection.\n\n`zdev-verify <area> <task-id>` performs the same read-only preflight and requires\nthe explicit ID to equal the current ready goal task before starting one fresh\nconfigured verifier. It never invokes an implementer, changes lifecycle state,\nstages, or commits. Its public result is the accepted verifier object above. Empty,\nexhausted, or closed goals, a different ready task, unsafe state, unavailable\nindependent verification, or an invalid worker envelope returns `BLOCKER zdev-verify`\nwithout mutation."
const repositoryGuidance = "<!-- zdev:generated-repository-guidance:start -->\n## Repository guidance discovery\n\nBefore planning or changing code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.\n<!-- zdev:generated-repository-guidance:end -->"
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

export const meta = {
  name: 'zdev-goal',
  description: 'Continue one zdev area through independently verified task commits',
}

const runOneTask = async (args, agent) => {
const repositoryGuidance = "<!-- zdev:generated-repository-guidance:start -->\n## Repository guidance discovery\n\nBefore inspecting, planning, changing, or validating code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.\n<!-- zdev:generated-repository-guidance:end -->"
const taskWorkflowContract = "The coordinating session owns task selection, branch safety, Git ownership,\nlifecycle changes, staging, commits, and delegation. Workers stay within the\nselected task and return one role-specific result.\n\nAn isolated area uses its stored branch and managed base relationship. An\nexplicit trunk area dynamically uses configured `project.trunk`, may share it\nwith other explicit trunk areas, and never needs a rebase or freshness step.\nIn both modes, `task_work.safe` and the exact selected area/task govern work;\nsharing trunk never grants ownership of another area's or unrelated paths.\n\nBefore starting an implementer or verifier, run\n`zdev work-context <area> --format json` and retain the complete result. The\ncommand classifies goal lifecycle first. A validated closed context contains\nno status or Git evidence: implement returns successful no-work, while\nexplicit verify returns `BLOCKER zdev-verify`; neither starts a worker. Every\nopen context contains matching nested status and goal projections, a boolean\n`stale_advisory`, a full lowercase `head` commit ID, and exact `git_status`,\n`git_diff_cached`, and `git_diff` strings. Require the projected area,\nlifecycle, queue, and task ID to agree and task work to be safe. Report a true stale advisory once and continue without\nrequesting a rebase. Inspect relevant untracked files, and stop on unexplained\nor overlapping changes or any user-owned decision.\n\nFor implement, open/empty and open/exhausted are successful no-work results\nafter the open-work gates above and start no worker. Explicit verify requires\nopen/ready and returns `BLOCKER zdev-verify` without starting a verifier for\nevery no-work result. Invalid records, task graphs, or context output are\nblockers. For open/ready, retain the complete context unchanged and its task ID\nas the subject. Before verification and every rework handoff, rerun\n`work-context` and require the same ready task ID and an explainable exact Git\ndelta.\n\n`zdev-implement <area>` reads effective complexity from the selected task in\nwork-context.\nAuthored `routine` uses `routine-implementer`; `standard`, including an omitted\nlegacy value, uses `implementer`. Never infer routine work from files or diff\nsize. Before any edit for `advanced`, start one fresh read-only `planner` using\nthe `advanced-implementer` profile. Give it the complete work-context JSON,\nbrief, task, repository guidance, baseline, and task-owned paths. A valid plan\nis passed unchanged to a fresh `advanced-implementer`. A planner blocker,\nincluding any product decision, stops before edits. Resumption, verification,\nand rework never repeat planning.\n\nEvery planner, implementer, and verifier returns only one JSON object, without a\nsentinel line, Markdown fence, or other text. The object has exactly these keys:\n\n```json\n{\n  \"schema_version\": 1,\n  \"kind\": \"implementer\",\n  \"area\": \"<area>\",\n  \"task_id\": \"<task-id>\",\n  \"verdict\": \"ready\",\n  \"summary\": \"<non-empty summary>\",\n  \"evidence\": [],\n  \"findings\": [],\n  \"escalation\": \"none\"\n}\n```\n\n`kind` is `planner`, `implementer`, or `verifier`. Planner verdict is `plan` or\n`blocker`; implementer verdict is `ready` or `blocker`; verifier verdict is\n`pass`, `rework`, or `blocker`. A plan has no findings and puts exactly one\nnon-empty `Approach: `, `Paths: `, and `Validation: ` entry in `evidence`.\nVerifier `pass` has no findings; verifier `rework` has at least one concrete\nfinding. `summary` is a non-empty string. `evidence` and `findings` are always arrays of non-empty\nstrings, including when empty. `escalation` is `none`, except that verifier\n`rework` may request `advanced-implementer`. Every other combination requires\n`none`. Schema version, kind, area, task ID, keys, types, and combinations must\nmatch exactly. Reject duplicate or unknown keys, missing keys, extra text, and\nmalformed JSON. Inspect the checkout after an implementer result, then use a\nfresh configured `verifier` for every verdict. When the stale advisory applies,\nthe verifier includes its exact text once in `evidence`; otherwise it omits it.\n\n## Derived work handoff\n\nAn implementer that needs to split necessary direct work already covered by\nthe approved brief and task returns a valid implementer object with verdict `blocker`, escalation\n`none`, no findings, and one evidence item containing the complete transient\nproposal. That evidence string begins\n`PROPOSE zdev-derived <area> <source-task-id>\\n` and continues with exactly one\nJSON object. It proposes one through five ordinary TaskDraft children and no\nnested proposal. A pre-edit split has an empty `retained_parent_paths`; a\npost-edit split names the exact complete unstaged parent-owned path set and\nassigns every child exact, normalized, path-disjoint future paths. The worker\nnever runs derive review, apply, import, or any other `.zdev` mutation.\n\nThe coordinator recognizes this strict alternative before treating the worker\nresult as an ordinary blocker. It refreshes work-context and requires unchanged\narea, source task, HEAD, safety, and attributable Git state. Automatic authority\nrequires every child to be necessary direct work already covered by the brief\nand source task. When those semantic and retained-context checks pass, send the\nunchanged proposal directly to `zdev tasks derive apply\n<area> --from - --format json` with no approval; apply revalidates mechanical\nauthority under its lock.\n\nWhen the user must make a semantic choice and current state and path ownership\nare safe and mechanically eligible, send the proposal\nto `zdev tasks derive review <area> --from - --format json`. Require its\n`mechanically_eligible` result to remain true, present its stored Markdown with\n`zdev tasks derive review <area> --show`, and ask for ordinary approval. After\napproval, apply the returned opaque identity with `zdev tasks derive apply\n<area> --reviewed <review-id> --format json`. Do not reconstruct or resend the\nproposal. Approval resolves only the semantic choice.\n\nAn invalid proposal, unsafe or changed context, staged or incomplete ownership,\nor any mechanical apply failure stops without review or apply. Preserve and\nreport the state, follow recovery, and obtain fresh work-context before\nreconsidering it; a stored review cannot waive those gates. Never use ordinary\ntask import for a derived proposal.\n\nOne successful apply consumes this uninterrupted handoff. Do not accept a\nsecond or nested proposal from it. An investigation follow-up completes its\nsource and may expose ready children. A split keeps its source open and blocked\nby its children; retained parent edits stay with that source. Report the\nderived commit and stop the one-task interaction. A goal, loop, or explicit\ncontinuation obtains fresh work-context before selecting from the updated\nordinary graph. A later independently selected child or resumed source may\npropose once under the same current gates; no derivation count or lineage is\nstored.\n\nEvery verifier independently runs\n`zdev work-context <area> --store --format json` before inspecting or\nvalidating. It accepts only the compact locator for the same open, ready task\nand HEAD, then uses `zdev work-context <area> --show <snapshot> --format json`\nto inspect the complete immutable pre-validation context. It requires the same\nopen, ready, safe area and task and compares that context with the coordinator\nidentity only to detect intervening state. After validation it runs\n`zdev work-context <area> --compare <snapshot> --format json` and accepts only\nthe exact compact comparison schema for the selected area and snapshot with\n`equal: true`. A false comparison is `rework` for attributable task-owned\nwrites and otherwise `blocker`; missing, expired, corrupt, cross-area, or\nmalformed snapshot evidence is `blocker`. The verifier never repairs or\ndiscards validation writes.\n\nOn `pass`, its evidence contains exactly one\n`work_context_snapshot: W<16-lowercase-hex>` entry, apart from the existing\noptional stale advisory. Put checked locations and validation conclusions in\n`summary`, not additional evidence items. The snapshot is resolved only by\nzdev; coordinators accept the opaque ID and never a worker-supplied path. This\none immutable snapshot proves both the independently collected pre-validation\nstate and, through the successful comparison, the equal post-validation state.\n\nEvery concrete task-owned verifier `rework` with escalation `none` goes to the\nsame selected profile when the harness can resume it, or a same-profile\nreplacement with the unchanged goal, baseline, current checkout, and full\nfindings. A verifier may request `advanced-implementer` once, only after the\ninitial standard/default implementation. That starts a replacement advanced\nimplementer without planning and is followed by a fresh standard verifier.\nReject a second escalation, an escalation after routine or advanced\nimplementation, and every escalation attached to `pass` or `blocker`. There is\nno fixed ordinary-rework count. After each correction, a fresh standard\nverifier checks the whole task again. Stop only on verifier `pass`, a genuine\nblocker, unsafe scope expansion, or a required user-owned decision.\n\nAfter an exact matching verifier object with verdict `pass`, the coordinator\ngives completion the opaque snapshot ID plus the accepted implementation and\nverifier summaries. Completion derives paths from the verified checkout and runs\nexactly one `zdev work-context <area> --compare <snapshot> --format json`\nbefore mutation and accepts only the exact compact schema for that area and ID\nwith `equal: true`. This fresh binary comparison covers area, ready task,\nlifecycle, safety, HEAD, index, worktree, and untracked state because all are\npart of the stored canonical context. A false comparison or an unavailable,\nexpired, corrupt, cross-area, or malformed artifact blocks before mutation.\nOn an accepted comparison, the coordinator runs `zdev task done`, stages only\nthe attributed task-owned files and exact generated task records, inspects the\nstaged diff, and runs `zdev commit`.\nCompletion or commit failure is a blocker that preserves and reports the exact\nstate. Public output begins with\n`PASS zdev-implement <area> <task-id>` or\n`BLOCKER zdev-implement <area> <task-id>`; its body repeats the exact area and\ntask, reports the stale advisory once when present, and names summary, changed\nfiles, validation, verifier evidence, and commit ID on pass, or the failed\nstage, reason, and preserved state on blocker. It omits the advisory field when\nno stale advisory was observed.\n\nAn ordinary `zdev-implement` pass completes one task. A successful split uses\nthe derived exception above and leaves its source open. After reporting the\nordinary verified commit or derived managed commit, it stops without querying\n`zdev next` or another `work-context`. A goal, loop, or explicit continuation\nowns the next iteration and must collect a fresh\n`zdev work-context <area> --format json` after the commit and before another\nworker dispatch. It never reuses the completed task's pre-commit selection.\n\n`zdev-verify <area> <task-id>` performs the same read-only preflight and requires\nthe explicit ID to equal the current ready task before starting one fresh\nconfigured verifier. It never invokes an implementer, changes lifecycle state,\nstages, commits, or routes a derived proposal. Its public result is the accepted verifier object above. Empty,\nexhausted, or closed goals, a different ready task, unsafe state, unavailable\nindependent verification, or an invalid worker envelope returns `BLOCKER zdev-verify`\nwithout mutation."
const workerContract = [
  'Before acting, use the canonical zdev task-workflow contract. In Bash, when `${CLAUDE_PLUGIN_ROOT:-}` is non-empty and `"${CLAUDE_PLUGIN_ROOT}/contracts/task-workflows.md"` is readable, load that installed file. Otherwise use the rendered canonical contract included inline below in this same prompt.',
  taskWorkflowContract,
  repositoryGuidance,
].join('\n\n')
const normalizeAreaArg = value => {
  if (Array.isArray(value)) return value[0]
  if (typeof value === 'string') return value
  return value && typeof value === 'object' ? value.area : ''
}
const input = { area: normalizeAreaArg(args) }
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
    if (!['routine', 'standard', 'advanced'].includes(goal?.task?.complexity)) return null
    if (expectedTask && payload.task_id !== expectedTask) return null
  } else {
    if (!['empty', 'exhausted'].includes(payload.queue) || payload.task_id !== null || goal?.task !== null || expectedTask) return null
  }
  return { raw, lifecycle: 'open', queue: payload.queue, taskId: payload.task_id, complexity: goal?.task?.complexity ?? null, staleAdvisory: payload.stale_advisory, payload }
}
const parseStoredContext = (raw, expectedArea) => {
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
  if (!stored.context || Array.isArray(stored.context) || typeof stored.context !== 'object') return null
  const context = parseContext(JSON.stringify(stored.context), expectedArea)
  return context ? { ...context, baselineSnapshot: stored.snapshot } : null
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
  const validVerdict = expectedKind === 'planner'
    ? ['plan', 'blocker'].includes(result.verdict)
    : expectedKind === 'implementer'
      ? ['ready', 'blocker'].includes(result.verdict)
      : ['pass', 'rework', 'blocker'].includes(result.verdict)
  if (!validVerdict) return null
  if (expectedKind === 'planner' && result.verdict === 'plan') {
    if (result.findings.length !== 0) return null
    for (const prefix of ['Approach: ', 'Paths: ', 'Validation: ']) {
      if (result.evidence.filter(item => item.startsWith(prefix) && item.length > prefix.length).length !== 1) return null
    }
  }
  const validEscalation = result.escalation === 'none'
    || (expectedKind === 'verifier' && result.verdict === 'rework' && result.escalation === 'advanced-implementer')
  if (expectedKind === 'verifier' && result.verdict === 'pass' && result.findings.length !== 0) return null
  if (expectedKind === 'verifier' && result.verdict === 'rework' && result.findings.length === 0) return null
  return validEscalation ? result : null
}
const derivedSplitFrom = (result, expectedArea, expectedTask) => {
  if (result?.kind !== 'implementer' || result.verdict !== 'blocker'
    || result.escalation !== 'none' || result.findings.length !== 0
    || result.evidence.length !== 1) return null
  const proposal = result.evidence[0]
  const first = `PROPOSE zdev-derived ${expectedArea} ${expectedTask}\n`
  if (!proposal.startsWith(first)) return null
  try {
    const payload = JSON.parse(proposal.slice(first.length))
    return payload?.proposal === 'implementation_split'
      && payload.area === expectedArea && payload.source_task === expectedTask
      ? proposal
      : null
  } catch {
    return null
  }
}

if (!/^[a-z0-9][a-z0-9-]*$/.test(area)) {
  return blocker('unknown', 'unknown', 'input', 'a lowercase area is required.', 'no preflight or worker was started.')
}

const preflight = async (label, storeBaseline = false) => agent(
  storeBaseline
    ? `Act only as the coordinating read-only preflight for area ${area}. Run zdev work-context ${area} --store --format json, then show that snapshot with zdev work-context ${area} --show <snapshot> --format json. Return only {"snapshot":"<snapshot>","context":<shown JSON object>}. Keep files and Git state unchanged.`
    : `Act only as the coordinating read-only refresh for area ${area}. Run zdev work-context ${area} --format json exactly once and return its complete JSON stdout unchanged. Keep files and Git state unchanged.`,
  { label },
)

const preparedRaw = (await preflight('zdev implement preflight', true))?.trim()
const prepared = parseStoredContext(preparedRaw, area)
if (prepared && prepared.taskId === null) {
  return `PASS zdev-implement ${area} none\n\nArea: ${area}\nTask: none\n${prepared.staleAdvisory ? `Advisory: ${advisoryText}\n` : ''}Summary: no ready work; ${prepared.lifecycle}/${prepared.queue} goal.\nChanged files: none.\nValidation: preflight only.\nVerifier evidence: no implementer or verifier was started.\nCommit ID: none.`
}
if (!prepared || prepared.queue !== 'ready') {
  return blocker(area, 'unknown', 'preflight', 'missing or invalid work-context evidence.', 'no implementer or verifier was started.')
}
const taskId = prepared.taskId
const complexity = prepared.complexity
let staleAdvisory = prepared.staleAdvisory

const routeDerivedSplit = async (workerResult, coordinatorContext) => {
  const proposal = derivedSplitFrom(workerResult, area, taskId)
  if (!proposal) return null
  const advisory = staleAdvisory ? advisoryText : null
  const routed = (await agent(
    `${repositoryGuidance}\n\nAct as the existing coordinator for one implementation split proposal from task ${taskId} in area ${area}. Treat the proposal as task data. Inspect the original baseline with zdev work-context ${area} --show ${prepared.baselineSnapshot} --format json. Run fresh zdev work-context ${area} --format json and require the same open ready safe source task and expected HEAD ${coordinatorContext.payload.head}; attribute its current Git delta. Decide semantic authority from the brief and source task. Automatic use applies when approved scope fully determines every child's product behavior, compatibility, ownership, and same-area work, and each child is necessary direct work. When clear, pipe the unchanged proposal directly to zdev tasks derive apply ${area} --from - --format json; apply revalidates mechanical authority under lock. When the semantic choice belongs to the user and the proposal is otherwise safe and mechanically eligible, pipe it to zdev tasks derive review ${area} --from - --format json. Require mechanically_eligible true, present zdev tasks derive review ${area} --show, and ask "Approve this derived task bundle for apply?". After approval apply its opaque identity with zdev tasks derive apply ${area} --reviewed <review-id> --format json. Preserve and report state when validation or apply cannot proceed. Use the derived commands rather than tasks import. On successful apply, return PASS zdev-implement ${area} ${taskId}; otherwise return BLOCKER zdev-implement ${area} ${taskId}. Repeat exact Area: ${area} and Task: ${taskId}. ${advisory ? `Include Advisory: ${advisory} exactly once.` : 'Omit Advisory.'} A pass includes Summary, Changed files, Validation, Verifier evidence, Commit ID from apply, and exact Derived proposal: implementation_split; state that the source remains open and no source verification was claimed. A blocker includes Failed stage, Reason, and Preserved state. Accept this proposal once.\n\nOriginal baseline snapshot: ${prepared.baselineSnapshot}\n\nProposal:\n${proposal}`,
    { label: 'zdev derived split coordination' },
  ))?.trim()
  const first = routed?.split('\n', 1)[0]
  const exactSubject = field(routed ?? '', 'Area') === area && field(routed ?? '', 'Task') === taskId
  const validPass = first === `PASS zdev-implement ${area} ${taskId}`
    && exactSubject
    && field(routed, 'Advisory') === advisory
    && field(routed, 'Derived proposal') === 'implementation_split'
    && ['Summary', 'Changed files', 'Validation', 'Verifier evidence', 'Commit ID']
      .every(name => field(routed, name) !== null)
  const validBlocker = first === `BLOCKER zdev-implement ${area} ${taskId}`
    && exactSubject
    && field(routed, 'Advisory') === advisory
    && ['Failed stage', 'Reason', 'Preserved state'].every(name => field(routed, name) !== null)
  return validPass || validBlocker
    ? routed
    : blocker(area, taskId, 'derived split', 'coordinator returned an invalid or mismatched split result.', 'the source task and proposal require inspection before continuing.', staleAdvisory)
}

let plan = null
if (complexity === 'advanced') {
  const planRaw = (await agent(
    `${workerContract}\n\nPlan the ready advanced task ${taskId} in area ${area}, keeping the checkout unchanged. Use the complete coordinator context below. Return only the strict JSON object with kind "planner", area "${area}", task_id "${taskId}", verdict "plan" or "blocker", and escalation "none". A plan puts exactly one non-empty Approach:, Paths:, and Validation: entry in evidence and has no findings. A product decision is a blocker.\n\nCoordinator context:\n${prepared.raw}`,
    { agentType: 'zdev:zdev-planner', label: 'zdev advanced read-only plan' },
  ))?.trim()
  plan = parseWorkerResult(planRaw, 'planner', area, taskId)
  if (!plan) {
    return blocker(area, taskId, 'planning', 'planner returned an invalid or mismatched envelope.', 'no implementation, lifecycle, or commit change was started.', staleAdvisory)
  }
  if (plan.verdict === 'blocker') {
    return blocker(area, taskId, 'planning', plan.summary, `Evidence: ${plan.evidence.join('; ') || 'none.'} Findings: ${plan.findings.join('; ') || 'none.'}`, staleAdvisory)
  }
}
const implementationAgentType = complexity === 'routine'
  ? 'zdev:zdev-routine-implementer'
  : complexity === 'advanced'
    ? 'zdev:zdev-advanced-implementer'
    : 'zdev:zdev-implementer'
const implementationRaw = (await agent(
  `${workerContract}\n\nImplement the ready ${complexity} task ${taskId} in area ${area}. Use the complete coordinator context below.${plan ? ` Follow this validated plan unchanged: ${JSON.stringify(plan)}.` : ''} Change only task-owned source and tests, run required validation, and return only the required strict JSON object with kind "implementer", area "${area}", and task_id "${taskId}". If necessary direct work must split, use the valid typed blocker alternative with the exact implementation_split proposal as its sole evidence item; leave derive commands to the coordinator.\n\nCoordinator context:\n${prepared.raw}`,
  { agentType: implementationAgentType, label: `zdev ${complexity} implementation` },
))?.trim()
const implementation = parseWorkerResult(implementationRaw, 'implementer', area, taskId)
let latestImplementation = implementation
let activeAgentType = implementationAgentType
let escalated = false
const compactWorkerSummary = result => JSON.stringify({
  summary: result.summary,
  evidence: result.evidence,
})

const refresh = async label => {
  const current = parseContext((await preflight(label))?.trim(), area, taskId)
  if (current?.staleAdvisory) staleAdvisory = true
  return current?.queue === 'ready' && current.complexity === complexity ? current : blocker(area, taskId, 'context refresh', `expected ready task ${taskId} with unchanged complexity ${complexity} and complete work-context evidence.`, 'lifecycle and commit were not changed.', staleAdvisory)
}
const approvedSnapshot = result => {
  const matches = result.evidence
    .map(item => /^work_context_snapshot: (W[0-9a-f]{16})$/.exec(item))
    .filter(Boolean)
  return matches.length === 1 ? matches[0][1] : null
}
const verify = async current => {
  const currentAdvisory = current.staleAdvisory ? advisoryText : null
  const raw = (await agent(
    `${workerContract}\n\nIndependently verify task ${taskId} in area ${area}. Inspect the original implementation baseline through zdev work-context ${area} --show ${prepared.baselineSnapshot} --format json. Before validation, run zdev work-context ${area} --store --format json and accept its compact locator for the same open, ready, safe task ${taskId} and expected HEAD ${current.payload.head}. Inspect that current immutable context through zdev work-context ${area} --show <snapshot> --format json. Use the compact implementer summary to locate evidence. Check the whole task and run required validation, then run zdev work-context ${area} --compare <snapshot> --format json. Accept only the exact four-key compact result {"schema_version":1,"area":"${area}","snapshot":"<same-id>","equal":true}. Validation-written task-owned files are rework and ambiguous writes are blocker. A pass has empty findings and an evidence array containing exactly one work_context_snapshot: W<16-lowercase-hex> item${currentAdvisory ? ` and ${currentAdvisory} exactly once` : ''}. Rework has at least one concrete finding. Put checked locations and validation conclusions in summary. Return only the required strict JSON object with kind "verifier", area "${area}", and task_id "${taskId}". ${currentAdvisory ? '' : `Omit ${advisoryText} from evidence.`} Keep files unchanged. Return the locator rather than a snapshot path or raw Git evidence.\n\nOriginal baseline snapshot: ${prepared.baselineSnapshot}\nCompact implementer summary: ${compactWorkerSummary(latestImplementation)}`,
    { agentType: 'zdev:zdev-verifier', label: 'zdev fresh verification' },
  ))?.trim()
  const result = parseWorkerResult(raw, 'verifier', area, taskId)
  const advisoryCount = result?.evidence.filter(item => item === advisoryText).length
  const approved = result?.verdict === 'pass' ? approvedSnapshot(result) : true
  const passEvidenceCount = currentAdvisory ? 2 : 1
  return result && approved && (result.verdict !== 'pass' || result.evidence.length === passEvidenceCount) && advisoryCount === (currentAdvisory ? 1 : 0) ? { raw, result, approved } : null
}

if (!implementation) {
  return blocker(area, taskId, 'implementation', 'implementer returned an invalid or mismatched envelope.', 'lifecycle and commit were not changed.', staleAdvisory)
}
const initialSplit = await routeDerivedSplit(implementation, prepared)
if (initialSplit) return initialSplit
if (implementation.verdict === 'blocker') {
  return blocker(area, taskId, 'implementation', implementation.summary, `Evidence: ${implementation.evidence.join('; ') || 'none.'} Findings: ${implementation.findings.join('; ') || 'none.'}`, staleAdvisory)
}
let current = await refresh('zdev pre-verification refresh')
if (typeof current === 'string') return current
let verdict = await verify(current)
if (!verdict) {
  return blocker(area, taskId, 'verification', 'verifier returned an invalid or mismatched envelope.', 'lifecycle and commit were not changed.', staleAdvisory)
}
while (verdict.result.verdict === 'rework') {
  if (verdict.result.escalation === 'advanced-implementer') {
    if (complexity !== 'standard' || escalated) {
      return blocker(area, taskId, 'rework', 'verifier requested an inapplicable or repeated advanced escalation.', 'lifecycle and commit were not changed.', staleAdvisory)
    }
    escalated = true
    activeAgentType = 'zdev:zdev-advanced-implementer'
  }
  current = await refresh('zdev rework refresh')
  if (typeof current === 'string') return current
  const reworkRaw = (await agent(
    `${workerContract}\n\nContinue from the accepted plan and correct every concrete task-owned finding for ${taskId}. Inspect the original baseline through zdev work-context ${area} --show ${prepared.baselineSnapshot} --format json and use the current checkout at expected HEAD ${current.payload.head}. Return only the required strict JSON object with kind "implementer", area "${area}", and task_id "${taskId}". If necessary direct work must split, use the valid typed blocker alternative with the exact implementation_split proposal as its sole evidence item; leave derive commands to the coordinator.\n\nOriginal baseline snapshot: ${prepared.baselineSnapshot}\nVerifier findings:\n${verdict.raw}`,
    { agentType: activeAgentType, label: escalated ? 'zdev advanced escalation rework' : 'zdev native rework' },
  ))?.trim()
  const rework = parseWorkerResult(reworkRaw, 'implementer', area, taskId)
  if (!rework) {
    return blocker(area, taskId, 'rework', 'implementer returned an invalid or mismatched envelope.', 'lifecycle and commit were not changed.', staleAdvisory)
  }
  const reworkSplit = await routeDerivedSplit(rework, current)
  if (reworkSplit) return reworkSplit
  if (rework.verdict === 'blocker') {
    return blocker(area, taskId, 'rework', rework.summary, `Evidence: ${rework.evidence.join('; ') || 'none.'} Findings: ${rework.findings.join('; ') || 'none.'}`, staleAdvisory)
  }
  latestImplementation = rework
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
  `${repositoryGuidance}\n\nAct as the existing completion coordinator for verified task ${taskId} in area ${area}. Whether this completion is live or resumed, before mutation run exactly one zdev work-context ${area} --compare ${verdict.approved} --format json. Accept the exact four-key JSON object {"schema_version":1,"area":"${area}","snapshot":"${verdict.approved}","equal":true}. On an exact match, run zdev task done, stage the attributed task-owned paths and exact task records, inspect the cached diff, and run zdev commit. Preserve the task-done and index state if staging, cached-diff inspection, or commit needs recovery. Return PASS zdev-implement ${area} ${taskId} or BLOCKER zdev-implement ${area} ${taskId} as the exact first line. Repeat exact Area: ${area} and Task: ${taskId} fields. ${advisory ? `Include Advisory: ${advisory} exactly once, ` : 'Omit Advisory, '}plus Summary, Changed files, Validation, Verifier evidence, and Commit ID on pass, or Failed stage, Reason, and Preserved state on blocker.\n\nCompletion handoff: ${JSON.stringify({ snapshot: verdict.approved, implementation: latestImplementation.summary, verification: verdict.result.summary })}`,
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

}

const normalizeAreaArg = value => {
  if (Array.isArray(value)) return value[0]
  if (typeof value === 'string') return value
  return value && typeof value === 'object' ? value.area : ''
}
const loopArea = String(normalizeAreaArg(args) ?? '').trim()
const loopField = (text, name) => {
  const matches = text.split('\n').filter(line => line.startsWith(`${name}: `))
  return matches.length === 1 ? matches[0].slice(name.length + 2) : null
}
const loopAdvisory = 'stale effective-base link; managed rebase remains optional.'
const completedTasks = []
const commits = []
let sawAdvisory = false
let latestCompletedTask = null
let latestCommit = null

const stateFrom = raw => {
  try {
    const value = JSON.parse(raw)
    const context = value?.context ?? value
    return context?.area === loopArea
      && ['open', 'closed'].includes(context.lifecycle)
      && ['ready', 'empty', 'exhausted'].includes(context.queue)
      ? { lifecycle: context.lifecycle, queue: context.queue, taskId: context.task_id, head: context.head }
      : { lifecycle: 'unknown', queue: 'unknown', taskId: null, head: null }
  } catch {
    return { lifecycle: 'unknown', queue: 'unknown', taskId: null, head: null }
  }
}
const list = values => values.length === 0 ? 'none' : values.join(', ')
const advisoryLine = () => sawAdvisory ? `Advisory: ${loopAdvisory}\n` : ''
const pass = (state, reason) =>
  `PASS zdev-loop ${loopArea}\n\nArea: ${loopArea}\nLifecycle: ${state.lifecycle}\nQueue: ${state.queue}\n${advisoryLine()}Tasks completed: ${list(completedTasks)}\nCommits: ${list(commits)}\nStop reason: ${reason}`
const block = (state, task, stage, reason, preserved) =>
  `BLOCKER zdev-loop ${loopArea || 'unknown'}\n\nArea: ${loopArea || 'unknown'}\nLifecycle: ${state.lifecycle}\nQueue: ${state.queue}\n${advisoryLine()}Tasks completed: ${list(completedTasks)}\nCommits: ${list(commits)}\nStop reason: blocked.\nCurrent task: ${task}\nFailed stage: ${stage}\nReason: ${reason}\nPreserved state: ${preserved}`

if (!/^[a-z0-9][a-z0-9-]*$/.test(loopArea)) {
  return block({ lifecycle: 'unknown', queue: 'unknown' }, 'none', 'input', 'a lowercase area is required.', 'no preflight or worker was started.')
}

const freshContext = async () => agent(
  `Act only as the area-loop read-only preflight for area ${loopArea}. Run zdev work-context ${loopArea} --store --format json, then show that snapshot with zdev work-context ${loopArea} --show <snapshot> --format json. Return only {"snapshot":"<snapshot>","context":<shown JSON object>}. Keep files and Git state unchanged.`,
  { label: 'zdev loop continuation preflight' },
)

while (true) {
  const contextRaw = (await freshContext())?.trim() ?? ''
  const state = stateFrom(contextRaw)
  if (latestCompletedTask && state.lifecycle === 'open'
    && (state.head !== latestCommit || state.taskId === latestCompletedTask)) {
    return block(state, latestCompletedTask, 'continuation refresh', 'fresh work-context did not confirm the committed task advanced.', 'the committed pair remains recorded and no next worker was started.')
  }
  let supplied = false
  const result = (await runOneTask({ area: loopArea }, async (prompt, options) => {
    if (!supplied && options?.label === 'zdev implement preflight') {
      supplied = true
      return contextRaw
    }
    return agent(prompt, options)
  }))?.trim() ?? ''
  const first = result.split('\n', 1)[0]
  const task = loopField(result, 'Task')
  if (loopField(result, 'Advisory') === loopAdvisory) sawAdvisory = true

  if (first === `PASS zdev-implement ${loopArea} none` && task === 'none') {
    return pass(state, `no ready work; ${state.lifecycle}/${state.queue}.`)
  }
  if (task && task !== 'none' && first === `PASS zdev-implement ${loopArea} ${task}`) {
    const commit = loopField(result, 'Commit ID')
    if (!/^[0-9a-f]{40}$/.test(commit ?? '')) {
      return block(state, task, 'result validation', 'the one-task PASS omitted its commit ID.', 'the task result was not counted and no next task was started.')
    }
    if (loopField(result, 'Derived proposal') !== 'implementation_split') {
      completedTasks.push(task)
    }
    commits.push(commit)
    latestCompletedTask = task
    latestCommit = commit
    continue
  }
  if (task && first === `BLOCKER zdev-implement ${loopArea} ${task}`) {
    return block(
      state,
      task,
      loopField(result, 'Failed stage') ?? 'one-task iteration',
      loopField(result, 'Reason') ?? 'the one-task workflow stopped.',
      loopField(result, 'Preserved state') ?? 'inspect the one-task blocker before continuing.',
    )
  }
  return block(state, task ?? 'none', 'result validation', 'the one-task workflow returned an invalid or mismatched envelope.', 'the result was not counted and no next task was started.')
}

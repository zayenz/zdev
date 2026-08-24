export const meta = {
  name: 'zdev-goal',
  description: 'Continue one zdev area through independently verified task commits',
}

const runOneTask = async (args, agent) => {
const repositoryGuidance = "<!-- zdev:generated-repository-guidance:start -->\n## Repository guidance discovery\n\nBefore inspecting, planning, changing, or validating code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.\n<!-- zdev:generated-repository-guidance:end -->"
const taskWorkflowContract = "The coordinating session owns task selection, branch safety, Git ownership,\nlifecycle changes, staging, commits, and delegation. Workers stay within the\nselected task and return one role-specific result.\n\nAn isolated area uses its stored branch and managed base relationship. An\nexplicit trunk area dynamically uses configured `project.trunk`, may share it\nwith other explicit trunk areas, and never needs a rebase or freshness step.\nIn both modes, `task_work.safe` and the exact selected area/task govern work;\nsharing trunk never grants ownership of another area's or unrelated paths.\n\nBefore starting an implementer or verifier, collect fresh complete work-context\nthrough one of the admitted forms below and retain the complete result. The\ncommand classifies goal lifecycle first. A validated closed context contains\nno status or Git evidence: implement returns successful no-work, while\nexplicit verify returns `BLOCKER zdev-verify`; neither starts a worker. Every\nopen context contains matching nested status and goal projections, a boolean\n`stale_advisory`, a full lowercase `head` commit ID, and exact `git_status`,\n`git_diff_cached`, and `git_diff` strings. Require the projected area,\nlifecycle, queue, and task ID to agree and task work to be safe. Report a true stale advisory once and continue without\nrequesting a rebase. Inspect relevant untracked files, and stop on unexplained\nor overlapping changes or any user-owned decision.\n\nFor implement, open/empty and open/exhausted are successful no-work results\nafter the open-work gates above and start no worker. Explicit verify requires\nopen/ready and returns `BLOCKER zdev-verify` without starting a verifier for\nevery no-work result. Invalid records, task graphs, or context output are\nblockers. For open/ready, retain the complete context unchanged and its task ID\nas the subject. Every worker handoff requires fresh work-context admission and\nthe same ready task ID. Use either ordinary `--format json` or, when the same\nboundary also needs the immutable verifier snapshot, one `--store` plus\n`--show` collection. That store-and-show collection satisfies the fresh\npre-verifier admission without a preceding duplicate ordinary collection.\nBefore rework implementation, retain the ordinary refresh and require an\nexplainable exact Git delta.\n\n`zdev-implement <area>` reads effective complexity from the selected task in\nwork-context.\nAuthored `routine` uses `routine-implementer`; `standard`, including an omitted\nlegacy value, uses `implementer`. Never infer routine work from files or diff\nsize. Before any edit for `advanced`, start one fresh read-only `planner` using\nthe `advanced-implementer` profile. Give it the complete work-context JSON,\nbrief, task, repository guidance, baseline, and task-owned paths. The planner\nreturns exactly `verdict`, `summary`, `plan`, and `findings`. A plan uses\n`{\"verdict\":\"plan\",\"summary\":\"<non-empty>\",\"plan\":{\"approach\":\"<non-empty>\",\"paths\":[\"<normalized repository-relative path>\"],\"validation\":[\"<non-empty validation step>\"]},\"findings\":[]}`;\na blocker uses verdict `blocker`, `plan: null`, and at least one non-empty\nfinding. Reject unknown or duplicate keys, empty values, non-normalized paths,\ncontradictory variants, legacy nine-key output, extra text, and malformed JSON.\n\nThe coordinator reconstructs the compatible public nine-key planner envelope\nwith fixed `schema_version: 1`, `kind: \"planner\"`, selected area and task ID,\nand `escalation: \"none\"`. It copies summary and findings. For a plan, evidence\nis exactly `Approach: <approach>`, `Paths: <comma-joined paths>`, and\n`Validation: <semicolon-joined validation steps>` in that order; for a blocker,\nevidence is empty. Validate this complete public envelope before routing. Pass\nthe validated semantic plan object, with its approach and ordered arrays\nunchanged, to a fresh `advanced-implementer`. A planner blocker,\nincluding any product decision, stops before edits. Resumption, verification,\nand rework never repeat planning.\n\nEvery implementer returns only one JSON object, without a sentinel\nline, Markdown fence, or other text. The object has exactly these keys:\n\n```json\n{\n  \"schema_version\": 1,\n  \"kind\": \"implementer\",\n  \"area\": \"<area>\",\n  \"task_id\": \"<task-id>\",\n  \"verdict\": \"ready\",\n  \"summary\": \"<non-empty summary>\",\n  \"evidence\": [],\n  \"findings\": [],\n  \"escalation\": \"none\"\n}\n```\n\n`kind` is `implementer`; verdict is `ready` or `blocker`.\n`summary` is a non-empty string. `evidence` and `findings` are always arrays of non-empty\nstrings, including when empty. `escalation` is `none`. Schema version, kind, area, task ID,\nkeys, types, and combinations must\nmatch exactly. Reject duplicate or unknown keys, missing keys, extra text, and\nmalformed JSON. Inspect the checkout after an implementer result, then use a\nfresh configured `verifier` for every verdict.\n\n## Derived work handoff\n\nAn implementer that needs to split necessary direct work already covered by\nthe approved brief and task returns a valid implementer object with verdict `blocker`, escalation\n`none`, no findings, and one evidence item containing the complete transient\nproposal. That evidence string begins\n`PROPOSE zdev-derived <area> <source-task-id>\\n` and continues with exactly one\nJSON object. It proposes one through five ordinary TaskDraft children and no\nnested proposal. A pre-edit split has an empty `retained_parent_paths`; a\npost-edit split names the exact complete unstaged parent-owned path set and\nassigns every child exact, normalized, path-disjoint future paths. The worker\nnever runs derive review, apply, import, or any other `.zdev` mutation.\n\nThe coordinator recognizes this strict alternative before treating the worker\nresult as an ordinary blocker. It refreshes work-context and requires unchanged\narea, source task, HEAD, safety, and attributable Git state. Automatic authority\nrequires every child to be necessary direct work already covered by the brief\nand source task. When those semantic and retained-context checks pass, send the\nunchanged proposal directly to `zdev tasks derive apply\n<area> --from - --format json` with no approval; apply revalidates mechanical\nauthority under its lock.\n\nWhen the user must make a semantic choice and current state and path ownership\nare safe and mechanically eligible, send the proposal\nto `zdev tasks derive review <area> --from - --format json`. Require its\n`mechanically_eligible` result to remain true, present its stored Markdown with\n`zdev tasks derive review <area> --show`, and ask for ordinary approval. After\napproval, apply the returned opaque identity with `zdev tasks derive apply\n<area> --reviewed <review-id> --format json`. Do not reconstruct or resend the\nproposal. Approval resolves only the semantic choice.\n\nAn invalid proposal, unsafe or changed context, staged or incomplete ownership,\nor any mechanical apply failure stops without review or apply. Preserve and\nreport the state, follow recovery, and obtain fresh work-context before\nreconsidering it; a stored review cannot waive those gates. Never use ordinary\ntask import for a derived proposal.\n\nOne successful apply consumes this uninterrupted handoff. Do not accept a\nsecond or nested proposal from it. An investigation follow-up completes its\nsource and may expose ready children. A split keeps its source open and blocked\nby its children; retained parent edits stay with that source. Report the\nderived commit and stop the one-task interaction. A goal, loop, or explicit\ncontinuation obtains fresh work-context before selecting from the updated\nordinary graph. A later independently selected child or resumed source may\npropose once under the same current gates; no derivation count or lineage is\nstored.\n\nImmediately before every verifier dispatch, coordination runs\n`zdev work-context <area> --store --format json`, validates its compact result,\nand uses `zdev work-context <area> --show <snapshot> --format json` to require\nthe same open, ready, safe area, task, HEAD, and checkout as the admitted\nboundary. It supplies only the opaque `W<16-lowercase-hex>` locator and expected\nidentity to the verifier. The verifier resolves that immutable context with\n`--show`, checks the whole task, runs required validation, reports validation\nwrites, and never repairs or discards them.\n\nThe verifier returns only this semantic JSON object with no surrounding text:\n\n```json\n{\n  \"verdict\": \"pass\",\n  \"summary\": \"<non-empty summary>\",\n  \"findings\": [],\n  \"escalation\": \"none\"\n}\n```\n\nIt has exactly those four unique keys. `verdict` is `pass`, `rework`, or\n`blocker`; `summary` is non-empty; and `findings` is an array of non-empty\nstrings. `pass` has no findings, `rework` has at least one, and `blocker` may\nhave findings. `escalation` is `none`, except that `rework` may request\n`advanced-implementer`. Reject legacy nine-key verifier envelopes, duplicate\nor unknown keys, missing keys, extra text, malformed JSON, and contradictory\ncombinations.\n\nFor each concrete task-owned file written by validation, `rework` includes one\nexact `validation_write: <normalized repository-relative path>` finding. The\nverifier never uses that prefix for an ordinary implementation defect. An\nambiguous validation write is `blocker`, not a tagged finding.\nWhen any finding starts with `validation_write:`, every such finding must use\nthe exact valid form; a mixed valid and malformed marker set is a blocker.\n\nAfter the response, coordination runs\n`zdev work-context <area> --compare <snapshot> --format json` and accepts only\nthe exact compact schema for the selected area and snapshot. It never accepts\n`pass` unless `equal` is true. A false comparison preserves `rework` only when\nthe semantic result contains at least one tagged task-owned validation-write\npath and every marker-prefixed finding is valid;\nan ordinary implementation-defect rework plus unequal state is a coordinator\nblocker because the mismatch is not attributed. Missing, expired,\ncorrupt, cross-area, or malformed snapshot or comparison evidence is also a\nblocker.\n\nCoordination then constructs the compatible public verifier envelope with\ngenerated `schema_version: 1`, `kind: \"verifier\"`, selected `area`, selected\n`task_id`, and `evidence`. Evidence contains exactly\n`work_context_snapshot: <snapshot>` plus the exact stale advisory once when it\napplies. It copies only the validated four semantic fields into that envelope\nand validates the resulting nine keys and all combinations before routing or\nreturning it. Put checked locations and validation conclusions in `summary`.\nThe opaque snapshot is never accepted from worker output.\n\nEvery concrete task-owned verifier `rework` with escalation `none` goes to the\nsame selected profile when the harness can resume it, or a same-profile\nreplacement with the unchanged goal, baseline, current checkout, and full\nfindings. A verifier may request `advanced-implementer` once, only after the\ninitial standard/default implementation. That starts a replacement advanced\nimplementer without planning and is followed by a fresh standard verifier.\nReject a second escalation, an escalation after routine or advanced\nimplementation, and every escalation attached to `pass` or `blocker`. There is\nno fixed ordinary-rework count. After each correction, a fresh standard\nverifier checks the whole task again. Stop only on verifier `pass`, a genuine\nblocker, unsafe scope expansion, or a required user-owned decision.\n\nAfter an exact matching coordinator-constructed verifier object with verdict `pass`, the coordinator\ngives completion the opaque snapshot ID plus the accepted implementation and\nverifier summaries. Completion derives paths from the verified checkout and runs\nexactly one `zdev work-context <area> --compare <snapshot> --format json`\nbefore mutation and accepts only the exact compact schema for that area and ID\nwith `equal: true`. This fresh binary comparison covers area, ready task,\nlifecycle, safety, HEAD, index, worktree, and untracked state because all are\npart of the stored canonical context. A false comparison or an unavailable,\nexpired, corrupt, cross-area, or malformed artifact blocks before mutation.\nOn an accepted comparison, the coordinator runs `zdev task done`, stages only\nthe attributed task-owned files and exact generated task records, inspects the\nstaged diff, and runs `zdev commit`.\nCompletion or commit failure is a blocker that preserves and reports the exact\nstate. Public output begins with\n`PASS zdev-implement <area> <task-id>` or\n`BLOCKER zdev-implement <area> <task-id>`; its body repeats the exact area and\ntask, reports the stale advisory once when present, and names summary, changed\nfiles, validation, verifier evidence, and commit ID on pass, or the failed\nstage, reason, and preserved state on blocker. It omits the advisory field when\nno stale advisory was observed.\n\nAn ordinary `zdev-implement` pass completes one task. A successful split uses\nthe derived exception above and leaves its source open. After reporting the\nordinary verified commit or derived managed commit, it stops without querying\n`zdev next` or another `work-context`. A goal, loop, or explicit continuation\nowns the next iteration and must collect a fresh\n`zdev work-context <area> --format json` after the commit and before another\nworker dispatch. It never reuses the completed task's pre-commit selection.\n\n`zdev-verify <area> <task-id>` performs the same read-only preflight and requires\nthe explicit ID to equal the current ready task before starting one fresh\nconfigured verifier. It never invokes an implementer, changes lifecycle state,\nstages, commits, or routes a derived proposal. Its public result is the coordinator-constructed verifier object above. Empty,\nexhausted, or closed goals, a different ready task, unsafe state, unavailable\nindependent verification, or an invalid worker envelope returns `BLOCKER zdev-verify`\nwithout mutation."
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
const parseStoredContext = (raw, expectedArea, expected = null) => {
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
  const context = parseContext(JSON.stringify(stored.context), expectedArea, expected?.taskId ?? null)
  if (expected && context && (
    context.queue !== 'ready'
    || context.payload.head !== expected.payload.head
    || context.payload.git_status !== expected.payload.git_status
    || context.payload.git_diff_cached !== expected.payload.git_diff_cached
    || context.payload.git_diff !== expected.payload.git_diff
  )) return null
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
const scanTopLevelObject = raw => {
  let index = 0
  const keys = []
  const rawValues = new Map()
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
    rawValues.set(key, raw.slice(valueStart, index))
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
  if (index !== raw.length) return null
  return { keys, rawValues }
}
const topLevelKeys = raw => scanTopLevelObject(raw)?.keys ?? null
const validateWorkerResult = (result, expectedKind, expectedArea, expectedTask) => {
  if (!result || Array.isArray(result) || typeof result !== 'object') return null
  if (JSON.stringify(Object.keys(result).sort()) !== JSON.stringify(workerResultKeys)) return null
  if (result.schema_version !== 1 || result.kind !== expectedKind) return null
  if (result.area !== expectedArea || result.task_id !== expectedTask) return null
  if (typeof result.summary !== 'string' || !result.summary.trim()) return null
  for (const name of ['evidence', 'findings']) {
    if (!Array.isArray(result[name])) return null
    if (!result[name].every(item => typeof item === 'string' && item.trim())) return null
  }
  const validVerdict = expectedKind === 'planner'
    ? ['plan', 'blocker'].includes(result.verdict)
    : expectedKind === 'implementer' ? ['ready', 'blocker'].includes(result.verdict) : false
  if (!validVerdict) return null
  if (expectedKind === 'planner' && result.verdict === 'plan') {
    if (result.findings.length !== 0 || result.evidence.length !== 3) return null
    if (!['Approach: ', 'Paths: ', 'Validation: '].every((prefix, index) =>
      result.evidence[index].startsWith(prefix) && result.evidence[index].length > prefix.length)) return null
  }
  if (expectedKind === 'planner' && result.verdict === 'blocker'
    && (result.evidence.length !== 0 || result.findings.length === 0)) return null
  return result.escalation === 'none' ? result : null
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
  return validateWorkerResult(result, expectedKind, expectedArea, expectedTask)
}
const plannerSchema = {
  type: 'object',
  additionalProperties: false,
  required: ['verdict', 'summary', 'plan', 'findings'],
  properties: {
    verdict: { type: 'string', enum: ['plan', 'blocker'] },
    summary: { type: 'string', minLength: 1 },
    plan: { anyOf: [
      { type: 'null' },
      { type: 'object', additionalProperties: false,
        required: ['approach', 'paths', 'validation'], properties: {
          approach: { type: 'string', minLength: 1 },
          paths: { type: 'array', minItems: 1, items: { type: 'string', minLength: 1 } },
          validation: { type: 'array', minItems: 1, items: { type: 'string', minLength: 1 } },
        } },
    ] },
    findings: { type: 'array', items: { type: 'string', minLength: 1 } },
  },
}
const semanticPlannerKeys = ['findings', 'plan', 'summary', 'verdict']
const semanticPlanKeys = ['approach', 'paths', 'validation']
const normalizedRepositoryPath = path => typeof path === 'string' && path.trim() === path
  && path.length > 0 && !path.startsWith('/') && !path.includes('\\')
  && path.split('/').every(part => part && part !== '.' && part !== '..')
const validateSemanticPlannerResult = result => {
  if (!result || Array.isArray(result) || typeof result !== 'object') return null
  if (JSON.stringify(Object.keys(result).sort()) !== JSON.stringify(semanticPlannerKeys)) return null
  if (typeof result.summary !== 'string' || !result.summary.trim()) return null
  if (!Array.isArray(result.findings)
    || !result.findings.every(item => typeof item === 'string' && item.trim())) return null
  if (result.verdict === 'blocker') return result.plan === null && result.findings.length > 0 ? result : null
  if (result.verdict !== 'plan' || result.findings.length !== 0
    || !result.plan || Array.isArray(result.plan) || typeof result.plan !== 'object') return null
  if (JSON.stringify(Object.keys(result.plan).sort()) !== JSON.stringify(semanticPlanKeys)) return null
  if (typeof result.plan.approach !== 'string' || !result.plan.approach.trim()) return null
  if (!Array.isArray(result.plan.paths) || result.plan.paths.length === 0
    || !result.plan.paths.every(normalizedRepositoryPath)) return null
  if (!Array.isArray(result.plan.validation) || result.plan.validation.length === 0
    || !result.plan.validation.every(item => typeof item === 'string' && item.trim())) return null
  return result
}
const parsePlannerResult = raw => {
  if (typeof raw !== 'string') return validateSemanticPlannerResult(raw)
  const scanned = scanTopLevelObject(raw)
  const keys = scanned?.keys
  if (!keys || new Set(keys).size !== keys.length
    || JSON.stringify([...keys].sort()) !== JSON.stringify(semanticPlannerKeys)) return null
  try {
    const result = JSON.parse(raw)
    if (result?.verdict === 'plan') {
      const planKeys = topLevelKeys(scanned.rawValues.get('plan'))
      if (!planKeys || new Set(planKeys).size !== planKeys.length
        || JSON.stringify([...planKeys].sort()) !== JSON.stringify(semanticPlanKeys)) return null
    }
    return validateSemanticPlannerResult(result)
  } catch { return null }
}
const reconstructPlannerResult = (semantic, area, taskId) => validateWorkerResult({
  schema_version: 1, kind: 'planner', area, task_id: taskId,
  verdict: semantic.verdict, summary: semantic.summary,
  evidence: semantic.verdict === 'plan' ? [
    `Approach: ${semantic.plan.approach}`,
    `Paths: ${semantic.plan.paths.join(', ')}`,
    `Validation: ${semantic.plan.validation.join('; ')}`,
  ] : [],
  findings: semantic.findings, escalation: 'none',
}, 'planner', area, taskId)
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
  if (typeof raw !== 'string') return null
  const keys = topLevelKeys(raw)
  if (!keys || new Set(keys).size !== keys.length) return null
  if (JSON.stringify([...keys].sort()) !== JSON.stringify(verifierResultKeys)) return null
  let result
  try {
    result = JSON.parse(raw)
  } catch {
    return null
  }
  if (!result || Array.isArray(result) || typeof result !== 'object') return null
  if (!['pass', 'rework', 'blocker'].includes(result.verdict)) return null
  if (typeof result.summary !== 'string' || !result.summary.trim()) return null
  if (!Array.isArray(result.findings)
    || !result.findings.every(item => typeof item === 'string' && item.trim())) return null
  if (result.verdict === 'pass' && result.findings.length !== 0) return null
  if (result.verdict === 'rework' && result.findings.length === 0) return null
  const validEscalation = result.escalation === 'none'
    || (result.verdict === 'rework' && result.escalation === 'advanced-implementer')
  return validEscalation ? result : null
}
const parseComparison = (raw, expectedArea, expectedSnapshot) => {
  if (typeof raw !== 'string') return null
  let result
  try {
    result = JSON.parse(raw)
  } catch {
    return null
  }
  if (!result || Array.isArray(result) || typeof result !== 'object') return null
  if (JSON.stringify(Object.keys(result).sort()) !== JSON.stringify(['area', 'equal', 'schema_version', 'snapshot'])) return null
  return result.schema_version === 1 && result.area === expectedArea
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
  return JSON.stringify(Object.keys(result).sort()) === JSON.stringify(workerResultKeys)
    ? result : null
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
  const planRaw = await agent(
    `${workerContract}\n\nPlan the ready advanced task ${taskId} in area ${area}, keeping the checkout unchanged. Use the complete coordinator context below. Return only the exact four-field semantic JSON object {"verdict":"plan|blocker","summary":"<non-empty>","plan":{"approach":"<non-empty>","paths":["<normalized repository-relative path>"],"validation":["<non-empty validation step>"]}|null,"findings":[]}. A plan has an exact three-field plan object and no findings. A blocker has plan null and at least one finding. A product decision is a blocker.\n\nCoordinator context:\n${prepared.raw}`,
    { agentType: 'zdev:zdev-planner', label: 'zdev advanced read-only plan', schema: plannerSchema },
  )
  const semanticPlan = parsePlannerResult(typeof planRaw === 'string' ? planRaw.trim() : planRaw)
  plan = semanticPlan && reconstructPlannerResult(semanticPlan, area, taskId)
  if (!semanticPlan || !plan) {
    return blocker(area, taskId, 'planning', 'planner returned an invalid or mismatched envelope.', 'no implementation, lifecycle, or commit change was started.', staleAdvisory)
  }
  if (plan.verdict === 'blocker') {
    return blocker(area, taskId, 'planning', plan.summary, `Evidence: none. Findings: ${plan.findings.join('; ')}`, staleAdvisory)
  }
  plan = semanticPlan
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
const verify = async expected => {
  const storedRaw = (await agent(
    `Act only as deterministic verification coordination for task ${taskId} in area ${area}. Immediately before verifier dispatch, run zdev work-context ${area} --store --format json, then show that snapshot with zdev work-context ${area} --show <snapshot> --format json. Return only {"snapshot":"<snapshot>","context":<shown JSON object>}. Keep files and Git state unchanged.`,
    { label: 'zdev verification snapshot' },
  ))?.trim()
  const stored = parseStoredContext(storedRaw, area, expected)
  if (!stored) return null
  if (stored.taskId !== taskId || stored.complexity !== complexity
    || stored.payload.head !== prepared.payload.head) return null
  if (stored.staleAdvisory) staleAdvisory = true
  const currentAdvisory = staleAdvisory ? advisoryText : null
  const current = stored
  const snapshot = stored.baselineSnapshot
  const raw = (await agent(
    `${workerContract}\n\nIndependently verify task ${taskId} in area ${area}. Inspect the original implementation baseline through zdev work-context ${area} --show ${prepared.baselineSnapshot} --format json. Show the coordinator-supplied immutable verification context with zdev work-context ${area} --show ${snapshot} --format json and require open, ready, safe task ${taskId} at expected HEAD ${current.payload.head}. Use the compact implementer summary to locate evidence. Check the whole task and run required validation. Report any validation writes as rework when concretely task-owned or blocker when ownership is ambiguous; never repair or discard them. For every concrete task-owned file written by validation, include the exact finding validation_write: <normalized repository-relative path>; never use that prefix for an ordinary implementation defect. Put checked locations and validation conclusions in summary. Return only the exact four-field JSON object {"verdict":"pass|rework|blocker","summary":"<non-empty summary>","findings":[],"escalation":"none|advanced-implementer"} with no identity, evidence, or surrounding text. Keep lifecycle and coordination-owned state unchanged.\n\nVerification snapshot: ${snapshot}\nOriginal baseline snapshot: ${prepared.baselineSnapshot}\nCompact implementer summary: ${compactWorkerSummary(latestImplementation)}`,
    { agentType: 'zdev:zdev-verifier', label: 'zdev fresh verification' },
  ))?.trim()
  const semantic = parseVerifierResult(raw)
  const comparedRaw = (await agent(
    `Act only as deterministic post-verification coordination. Run zdev work-context ${area} --compare ${snapshot} --format json exactly once and return its complete JSON stdout unchanged, with no fence or other text. Keep files and Git state unchanged.`,
    { label: 'zdev post-verification compare' },
  ))?.trim()
  const compared = parseComparison(comparedRaw, area, snapshot)
  if (!semantic || !compared) return null
  if (!compared.equal && !reportsValidationWrite(semantic)) return null
  const result = publicVerifier(semantic, snapshot, currentAdvisory)
  if (!result) return null
  return {
    raw: JSON.stringify(result),
    result,
    approved: result.verdict === 'pass' ? snapshot : true,
  }
}

if (!implementation) {
  return blocker(area, taskId, 'implementation', 'implementer returned an invalid or mismatched envelope.', 'lifecycle and commit were not changed.', staleAdvisory)
}
const initialSplit = await routeDerivedSplit(implementation, prepared)
if (initialSplit) return initialSplit
if (implementation.verdict === 'blocker') {
  return blocker(area, taskId, 'implementation', implementation.summary, `Evidence: ${implementation.evidence.join('; ') || 'none.'} Findings: ${implementation.findings.join('; ') || 'none.'}`, staleAdvisory)
}
let current = null
let verdict = await verify()
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
  verdict = await verify()
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

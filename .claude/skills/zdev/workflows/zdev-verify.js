export const meta = {
  name: 'zdev-verify',
  description: 'Independently verify the explicit current ready zdev task',
}

const repositoryGuidance = "<!-- zdev:generated-repository-guidance:start -->\n## Repository guidance discovery\n\nBefore inspecting, planning, changing, or validating code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.\n<!-- zdev:generated-repository-guidance:end -->"
const taskWorkflowContract = "The coordinating session owns task selection, branch safety, Git ownership,\nlifecycle changes, staging, commits, and delegation. Workers stay within the\nselected task and return one role-specific result.\n\nAn isolated area uses its stored branch and managed base relationship. An\nexplicit trunk area dynamically uses configured `project.trunk`, may share it\nwith other explicit trunk areas, and never needs a rebase or freshness step.\nIn both modes, `task_work.safe` and the exact selected area/task govern work;\nsharing trunk never grants ownership of another area's or unrelated paths.\n\nBefore starting an implementer or verifier, run\n`zdev work-context <area> --format json` and retain the complete result. The\ncommand classifies goal lifecycle first. A validated closed context contains\nno status or Git evidence: implement returns successful no-work, while\nexplicit verify returns `BLOCKER zdev-verify`; neither starts a worker. Every\nopen context contains matching nested status and goal projections, a boolean\n`stale_advisory`, a full lowercase `head` commit ID, and exact `git_status`,\n`git_diff_cached`, and `git_diff` strings. Require the projected area,\nlifecycle, queue, and task ID to agree and task work to be safe. Report a true stale advisory once and continue without\nrequesting a rebase. Inspect relevant untracked files, and stop on unexplained\nor overlapping changes or any user-owned decision.\n\nFor implement, open/empty and open/exhausted are successful no-work results\nafter the open-work gates above and start no worker. Explicit verify requires\nopen/ready and returns `BLOCKER zdev-verify` without starting a verifier for\nevery no-work result. Invalid records, task graphs, or context output are\nblockers. For open/ready, retain the complete context unchanged and its task ID\nas the subject. Before verification and every rework handoff, rerun\n`work-context` and require the same ready task ID and an explainable exact Git\ndelta.\n\n`zdev-implement <area>` reads effective complexity from the selected task in\nwork-context.\nAuthored `routine` uses `routine-implementer`; `standard`, including an omitted\nlegacy value, uses `implementer`. Never infer routine work from files or diff\nsize. Before any edit for `advanced`, start one fresh read-only `planner` using\nthe `advanced-implementer` profile. Give it the complete work-context JSON,\nbrief, task, repository guidance, baseline, and task-owned paths. A valid plan\nis passed unchanged to a fresh `advanced-implementer`. A planner blocker,\nincluding any product decision, stops before edits. Resumption, verification,\nand rework never repeat planning.\n\nEvery planner and implementer returns only one JSON object, without a sentinel\nline, Markdown fence, or other text. The object has exactly these keys:\n\n```json\n{\n  \"schema_version\": 1,\n  \"kind\": \"implementer\",\n  \"area\": \"<area>\",\n  \"task_id\": \"<task-id>\",\n  \"verdict\": \"ready\",\n  \"summary\": \"<non-empty summary>\",\n  \"evidence\": [],\n  \"findings\": [],\n  \"escalation\": \"none\"\n}\n```\n\n`kind` is `planner` or `implementer`. Planner verdict is `plan` or `blocker`;\nimplementer verdict is `ready` or `blocker`. A plan has no findings and puts exactly one\nnon-empty `Approach: `, `Paths: `, and `Validation: ` entry in `evidence`.\n`summary` is a non-empty string. `evidence` and `findings` are always arrays of non-empty\nstrings, including when empty. `escalation` is `none`. Schema version, kind, area, task ID,\nkeys, types, and combinations must\nmatch exactly. Reject duplicate or unknown keys, missing keys, extra text, and\nmalformed JSON. Inspect the checkout after an implementer result, then use a\nfresh configured `verifier` for every verdict.\n\n## Derived work handoff\n\nAn implementer that needs to split necessary direct work already covered by\nthe approved brief and task returns a valid implementer object with verdict `blocker`, escalation\n`none`, no findings, and one evidence item containing the complete transient\nproposal. That evidence string begins\n`PROPOSE zdev-derived <area> <source-task-id>\\n` and continues with exactly one\nJSON object. It proposes one through five ordinary TaskDraft children and no\nnested proposal. A pre-edit split has an empty `retained_parent_paths`; a\npost-edit split names the exact complete unstaged parent-owned path set and\nassigns every child exact, normalized, path-disjoint future paths. The worker\nnever runs derive review, apply, import, or any other `.zdev` mutation.\n\nThe coordinator recognizes this strict alternative before treating the worker\nresult as an ordinary blocker. It refreshes work-context and requires unchanged\narea, source task, HEAD, safety, and attributable Git state. Automatic authority\nrequires every child to be necessary direct work already covered by the brief\nand source task. When those semantic and retained-context checks pass, send the\nunchanged proposal directly to `zdev tasks derive apply\n<area> --from - --format json` with no approval; apply revalidates mechanical\nauthority under its lock.\n\nWhen the user must make a semantic choice and current state and path ownership\nare safe and mechanically eligible, send the proposal\nto `zdev tasks derive review <area> --from - --format json`. Require its\n`mechanically_eligible` result to remain true, present its stored Markdown with\n`zdev tasks derive review <area> --show`, and ask for ordinary approval. After\napproval, apply the returned opaque identity with `zdev tasks derive apply\n<area> --reviewed <review-id> --format json`. Do not reconstruct or resend the\nproposal. Approval resolves only the semantic choice.\n\nAn invalid proposal, unsafe or changed context, staged or incomplete ownership,\nor any mechanical apply failure stops without review or apply. Preserve and\nreport the state, follow recovery, and obtain fresh work-context before\nreconsidering it; a stored review cannot waive those gates. Never use ordinary\ntask import for a derived proposal.\n\nOne successful apply consumes this uninterrupted handoff. Do not accept a\nsecond or nested proposal from it. An investigation follow-up completes its\nsource and may expose ready children. A split keeps its source open and blocked\nby its children; retained parent edits stay with that source. Report the\nderived commit and stop the one-task interaction. A goal, loop, or explicit\ncontinuation obtains fresh work-context before selecting from the updated\nordinary graph. A later independently selected child or resumed source may\npropose once under the same current gates; no derivation count or lineage is\nstored.\n\nImmediately before every verifier dispatch, coordination runs\n`zdev work-context <area> --store --format json`, validates its compact result,\nand uses `zdev work-context <area> --show <snapshot> --format json` to require\nthe same open, ready, safe area, task, HEAD, and checkout as the admitted\nrefresh. It supplies only the opaque `W<16-lowercase-hex>` locator and expected\nidentity to the verifier. The verifier resolves that immutable context with\n`--show`, checks the whole task, runs required validation, reports validation\nwrites, and never repairs or discards them.\n\nThe verifier returns only this semantic JSON object with no surrounding text:\n\n```json\n{\n  \"verdict\": \"pass\",\n  \"summary\": \"<non-empty summary>\",\n  \"findings\": [],\n  \"escalation\": \"none\"\n}\n```\n\nIt has exactly those four unique keys. `verdict` is `pass`, `rework`, or\n`blocker`; `summary` is non-empty; and `findings` is an array of non-empty\nstrings. `pass` has no findings, `rework` has at least one, and `blocker` may\nhave findings. `escalation` is `none`, except that `rework` may request\n`advanced-implementer`. Reject legacy nine-key verifier envelopes, duplicate\nor unknown keys, missing keys, extra text, malformed JSON, and contradictory\ncombinations.\n\nFor each concrete task-owned file written by validation, `rework` includes one\nexact `validation_write: <normalized repository-relative path>` finding. The\nverifier never uses that prefix for an ordinary implementation defect. An\nambiguous validation write is `blocker`, not a tagged finding.\nWhen any finding starts with `validation_write:`, every such finding must use\nthe exact valid form; a mixed valid and malformed marker set is a blocker.\n\nAfter the response, coordination runs\n`zdev work-context <area> --compare <snapshot> --format json` and accepts only\nthe exact compact schema for the selected area and snapshot. It never accepts\n`pass` unless `equal` is true. A false comparison preserves `rework` only when\nthe semantic result contains at least one tagged task-owned validation-write\npath and every marker-prefixed finding is valid;\nan ordinary implementation-defect rework plus unequal state is a coordinator\nblocker because the mismatch is not attributed. Missing, expired,\ncorrupt, cross-area, or malformed snapshot or comparison evidence is also a\nblocker.\n\nCoordination then constructs the compatible public verifier envelope with\ngenerated `schema_version: 1`, `kind: \"verifier\"`, selected `area`, selected\n`task_id`, and `evidence`. Evidence contains exactly\n`work_context_snapshot: <snapshot>` plus the exact stale advisory once when it\napplies. It copies only the validated four semantic fields into that envelope\nand validates the resulting nine keys and all combinations before routing or\nreturning it. Put checked locations and validation conclusions in `summary`.\nThe opaque snapshot is never accepted from worker output.\n\nEvery concrete task-owned verifier `rework` with escalation `none` goes to the\nsame selected profile when the harness can resume it, or a same-profile\nreplacement with the unchanged goal, baseline, current checkout, and full\nfindings. A verifier may request `advanced-implementer` once, only after the\ninitial standard/default implementation. That starts a replacement advanced\nimplementer without planning and is followed by a fresh standard verifier.\nReject a second escalation, an escalation after routine or advanced\nimplementation, and every escalation attached to `pass` or `blocker`. There is\nno fixed ordinary-rework count. After each correction, a fresh standard\nverifier checks the whole task again. Stop only on verifier `pass`, a genuine\nblocker, unsafe scope expansion, or a required user-owned decision.\n\nAfter an exact matching coordinator-constructed verifier object with verdict `pass`, the coordinator\ngives completion the opaque snapshot ID plus the accepted implementation and\nverifier summaries. Completion derives paths from the verified checkout and runs\nexactly one `zdev work-context <area> --compare <snapshot> --format json`\nbefore mutation and accepts only the exact compact schema for that area and ID\nwith `equal: true`. This fresh binary comparison covers area, ready task,\nlifecycle, safety, HEAD, index, worktree, and untracked state because all are\npart of the stored canonical context. A false comparison or an unavailable,\nexpired, corrupt, cross-area, or malformed artifact blocks before mutation.\nOn an accepted comparison, the coordinator runs `zdev task done`, stages only\nthe attributed task-owned files and exact generated task records, inspects the\nstaged diff, and runs `zdev commit`.\nCompletion or commit failure is a blocker that preserves and reports the exact\nstate. Public output begins with\n`PASS zdev-implement <area> <task-id>` or\n`BLOCKER zdev-implement <area> <task-id>`; its body repeats the exact area and\ntask, reports the stale advisory once when present, and names summary, changed\nfiles, validation, verifier evidence, and commit ID on pass, or the failed\nstage, reason, and preserved state on blocker. It omits the advisory field when\nno stale advisory was observed.\n\nAn ordinary `zdev-implement` pass completes one task. A successful split uses\nthe derived exception above and leaves its source open. After reporting the\nordinary verified commit or derived managed commit, it stops without querying\n`zdev next` or another `work-context`. A goal, loop, or explicit continuation\nowns the next iteration and must collect a fresh\n`zdev work-context <area> --format json` after the commit and before another\nworker dispatch. It never reuses the completed task's pre-commit selection.\n\n`zdev-verify <area> <task-id>` performs the same read-only preflight and requires\nthe explicit ID to equal the current ready task before starting one fresh\nconfigured verifier. It never invokes an implementer, changes lifecycle state,\nstages, commits, or routes a derived proposal. Its public result is the coordinator-constructed verifier object above. Empty,\nexhausted, or closed goals, a different ready task, unsafe state, unavailable\nindependent verification, or an invalid worker envelope returns `BLOCKER zdev-verify`\nwithout mutation."
const workerContract = [
  'Before acting, use the canonical zdev task-workflow contract. In Bash, when `${CLAUDE_PLUGIN_ROOT:-}` is non-empty and `"${CLAUDE_PLUGIN_ROOT}/contracts/task-workflows.md"` is readable, load that installed file. Otherwise use the rendered canonical contract included inline below in this same prompt.',
  taskWorkflowContract,
  repositoryGuidance,
].join('\n\n')
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
  return { raw, head: payload.head, staleAdvisory: taskWork.stale_advisory, payload }
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
const parseStoredContext = (raw, expected) => {
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
  const context = parseReady(JSON.stringify(stored.context))
  if (!context
    || context.payload.head !== expected.payload.head
    || context.payload.git_status !== expected.payload.git_status
    || context.payload.git_diff_cached !== expected.payload.git_diff_cached
    || context.payload.git_diff !== expected.payload.git_diff) return null
  return { snapshot: stored.snapshot, context }
}
const parseComparison = (raw, expectedSnapshot) => {
  if (typeof raw !== 'string') return null
  let result
  try {
    result = JSON.parse(raw)
  } catch {
    return null
  }
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

const preflight = await agent(
  `Act only as the coordinating read-only preflight. Run zdev work-context ${area} --format json exactly once and return its complete JSON stdout unchanged, with no fence or other text. Keep files and Git state unchanged.`,
  { label: 'zdev verify preflight' },
)
const prepared = parseReady(preflight?.trim())
if (!prepared) {
  return blocker(area, taskId, 'missing or invalid ready goal, requested task match, branch safety, or complete Git baseline evidence.')
}
const advisory = prepared.staleAdvisory ? advisoryText : null

const storedRaw = await agent(
  `Act only as deterministic verification coordination for task ${taskId} in area ${area}. Immediately before verifier dispatch, run zdev work-context ${area} --store --format json, then show that snapshot with zdev work-context ${area} --show <snapshot> --format json. Return only {"snapshot":"<snapshot>","context":<shown JSON object>}. Keep files and Git state unchanged.`,
  { label: 'zdev verification snapshot' },
)
const stored = parseStoredContext(storedRaw?.trim(), prepared)
if (!stored) {
  return blocker(area, taskId, 'coordinator could not store and validate the admitted verification snapshot.', prepared.staleAdvisory)
}

const verified = await agent(
  `${workerContract}\n\nIndependently verify task ${taskId} in area ${area} from the current checkout. Show the coordinator-supplied immutable context with zdev work-context ${area} --show ${stored.snapshot} --format json and require the same open, ready, safe task and HEAD ${prepared.head}. Check the whole task and run required validation. Report validation-written task-owned files as rework and ambiguous writes as blocker; never repair or discard them. For every concrete task-owned file written by validation, include the exact finding validation_write: <normalized repository-relative path>; never use that prefix for an ordinary implementation defect. Put checked locations and validation conclusions in summary. Return only the exact four-field JSON object {"verdict":"pass|rework|blocker","summary":"<non-empty summary>","findings":[],"escalation":"none|advanced-implementer"} with no identity, evidence, or surrounding text. Keep lifecycle and coordination-owned state unchanged.\n\nVerification snapshot: ${stored.snapshot}`,
  { agentType: 'zdev:zdev-verifier', label: 'zdev fresh verification' },
)
const semantic = parseVerifierResult(verified?.trim())
const comparedRaw = await agent(
  `Act only as deterministic post-verification coordination. Run zdev work-context ${area} --compare ${stored.snapshot} --format json exactly once and return its complete JSON stdout unchanged, with no fence or other text. Keep files and Git state unchanged.`,
  { label: 'zdev post-verification compare' },
)
const compared = parseComparison(comparedRaw?.trim(), stored.snapshot)
if (!semantic || !compared || (!compared.equal && !reportsValidationWrite(semantic))) {
  return blocker(area, taskId, 'verifier output or post-validation comparison was invalid, contradictory, or changed ambiguously.', prepared.staleAdvisory)
}
const result = publicVerifier(semantic, stored.snapshot, advisory)
return result
  ? JSON.stringify(result)
  : blocker(area, taskId, 'coordinator could not construct the strict public verifier envelope.', prepared.staleAdvisory)

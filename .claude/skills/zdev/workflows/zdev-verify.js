export const meta = {
  name: 'zdev-verify',
  description: 'Independently verify the explicit current ready zdev task',
}

const repositoryGuidance = "<!-- zdev:generated-repository-guidance:start -->\n## Repository guidance discovery\n\nBefore inspecting, planning, changing, or validating code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.\n<!-- zdev:generated-repository-guidance:end -->"
const taskWorkflowContract = "The coordinating session owns task selection, branch safety, Git ownership,\nlifecycle changes, staging, commits, and delegation. Workers stay within the\nselected task and return one role-specific result.\n\nAn isolated area uses its stored branch and managed base relationship. An\nexplicit trunk area dynamically uses configured `project.trunk`, may share it\nwith other explicit trunk areas, and never needs a rebase or freshness step.\nIn both modes, `task_work.safe` and the exact selected area/task govern work;\nsharing trunk never grants ownership of another area's or unrelated paths.\n\nBefore starting an implementer or verifier, run\n`zdev work-context <area> --format json` and retain the complete result. The\ncommand classifies goal lifecycle first. A validated closed context contains\nno status or Git evidence: implement returns successful no-work, while\nexplicit verify returns `BLOCKER zdev-verify`; neither starts a worker. Every\nopen context contains matching nested status and goal projections, a boolean\n`stale_advisory`, a full lowercase `head` commit ID, and exact `git_status`,\n`git_diff_cached`, and `git_diff` strings. Require the projected area,\nlifecycle, queue, and task ID to agree and task work to be safe. Report a true stale advisory once and continue without\nrequesting a rebase. Inspect relevant untracked files, and stop on unexplained\nor overlapping changes or any user-owned decision.\n\nFor implement, open/empty and open/exhausted are successful no-work results\nafter the open-work gates above and start no worker. Explicit verify requires\nopen/ready and returns `BLOCKER zdev-verify` without starting a verifier for\nevery no-work result. Invalid records, task graphs, or context output are\nblockers. For open/ready, retain the complete context unchanged and its task ID\nas the subject. Before verification and every rework handoff, rerun\n`work-context` and require the same ready task ID and an explainable exact Git\ndelta.\n\n`zdev-implement <area>` reads effective complexity from the selected task in\nwork-context.\nAuthored `routine` uses `routine-implementer`; `standard`, including an omitted\nlegacy value, uses `implementer`. Never infer routine work from files or diff\nsize. Before any edit for `advanced`, start one fresh read-only `planner` using\nthe `advanced-implementer` profile. Give it the complete work-context JSON,\nbrief, task, repository guidance, baseline, and task-owned paths. A valid plan\nis passed unchanged to a fresh `advanced-implementer`. A planner blocker,\nincluding any product decision, stops before edits. Resumption, verification,\nand rework never repeat planning.\n\nEvery planner, implementer, and verifier returns only one JSON object, without a\nsentinel line, Markdown fence, or other text. The object has exactly these keys:\n\n```json\n{\n  \"schema_version\": 1,\n  \"kind\": \"implementer\",\n  \"area\": \"<area>\",\n  \"task_id\": \"<task-id>\",\n  \"verdict\": \"ready\",\n  \"summary\": \"<non-empty summary>\",\n  \"evidence\": [],\n  \"findings\": [],\n  \"escalation\": \"none\"\n}\n```\n\n`kind` is `planner`, `implementer`, or `verifier`. Planner verdict is `plan` or\n`blocker`; implementer verdict is `ready` or `blocker`; verifier verdict is\n`pass`, `rework`, or `blocker`. A plan has no findings and puts exactly one\nnon-empty `Approach: `, `Paths: `, and `Validation: ` entry in `evidence`.\nVerifier `pass` has no findings; verifier `rework` has at least one concrete\nfinding. `summary` is a non-empty string. `evidence` and `findings` are always arrays of non-empty\nstrings, including when empty. `escalation` is `none`, except that verifier\n`rework` may request `advanced-implementer`. Every other combination requires\n`none`. Schema version, kind, area, task ID, keys, types, and combinations must\nmatch exactly. Reject duplicate or unknown keys, missing keys, extra text, and\nmalformed JSON. Inspect the checkout after an implementer result, then use a\nfresh configured `verifier` for every verdict. When the stale advisory applies,\nthe verifier includes its exact text once in `evidence`; otherwise it omits it.\n\n## Derived work handoff\n\nAn implementer that needs to split necessary direct work already covered by\nthe approved brief and task returns a valid implementer object with verdict `blocker`, escalation\n`none`, no findings, and one evidence item containing the complete transient\nproposal. That evidence string begins\n`PROPOSE zdev-derived <area> <source-task-id>\\n` and continues with exactly one\nJSON object. It proposes one through five ordinary TaskDraft children and no\nnested proposal. A pre-edit split has an empty `retained_parent_paths`; a\npost-edit split names the exact complete unstaged parent-owned path set and\nassigns every child exact, normalized, path-disjoint future paths. The worker\nnever runs derive review, apply, import, or any other `.zdev` mutation.\n\nThe coordinator recognizes this strict alternative before treating the worker\nresult as an ordinary blocker. It refreshes work-context and requires unchanged\narea, source task, HEAD, safety, and attributable Git state. Automatic authority\nrequires every child to be necessary direct work already covered by the brief\nand source task. When those semantic and retained-context checks pass, send the\nunchanged proposal directly to `zdev tasks derive apply\n<area> --from - --format json` with no approval; apply revalidates mechanical\nauthority under its lock.\n\nWhen the user must make a semantic choice and current state and path ownership\nare safe and mechanically eligible, send the proposal\nto `zdev tasks derive review <area> --from - --format json`. Require its\n`mechanically_eligible` result to remain true, present its stored Markdown with\n`zdev tasks derive review <area> --show`, and ask for ordinary approval. After\napproval, apply the returned opaque identity with `zdev tasks derive apply\n<area> --reviewed <review-id> --format json`. Do not reconstruct or resend the\nproposal. Approval resolves only the semantic choice.\n\nAn invalid proposal, unsafe or changed context, staged or incomplete ownership,\nor any mechanical apply failure stops without review or apply. Preserve and\nreport the state, follow recovery, and obtain fresh work-context before\nreconsidering it; a stored review cannot waive those gates. Never use ordinary\ntask import for a derived proposal.\n\nOne successful apply consumes this uninterrupted handoff. Do not accept a\nsecond or nested proposal from it. An investigation follow-up completes its\nsource and may expose ready children. A split keeps its source open and blocked\nby its children; retained parent edits stay with that source. Report the\nderived commit and stop the one-task interaction. A goal, loop, or explicit\ncontinuation obtains fresh work-context before selecting from the updated\nordinary graph. A later independently selected child or resumed source may\npropose once under the same current gates; no derivation count or lineage is\nstored.\n\nEvery verifier independently runs\n`zdev work-context <area> --store --format json` before inspecting or\nvalidating. It accepts only the compact locator for the same open, ready task\nand HEAD, then uses `zdev work-context <area> --show <snapshot> --format json`\nto inspect the complete immutable pre-validation context. It requires the same\nopen, ready, safe area and task and compares that context with the coordinator\nidentity only to detect intervening state. After validation it runs\n`zdev work-context <area> --compare <snapshot> --format json` and accepts only\nthe exact compact comparison schema for the selected area and snapshot with\n`equal: true`. A false comparison is `rework` for attributable task-owned\nwrites and otherwise `blocker`; missing, expired, corrupt, cross-area, or\nmalformed snapshot evidence is `blocker`. The verifier never repairs or\ndiscards validation writes.\n\nOn `pass`, its evidence contains exactly one\n`work_context_snapshot: W<16-lowercase-hex>` entry, apart from the existing\noptional stale advisory. Put checked locations and validation conclusions in\n`summary`, not additional evidence items. The snapshot is resolved only by\nzdev; coordinators accept the opaque ID and never a worker-supplied path. This\none immutable snapshot proves both the independently collected pre-validation\nstate and, through the successful comparison, the equal post-validation state.\n\nEvery concrete task-owned verifier `rework` with escalation `none` goes to the\nsame selected profile when the harness can resume it, or a same-profile\nreplacement with the unchanged goal, baseline, current checkout, and full\nfindings. A verifier may request `advanced-implementer` once, only after the\ninitial standard/default implementation. That starts a replacement advanced\nimplementer without planning and is followed by a fresh standard verifier.\nReject a second escalation, an escalation after routine or advanced\nimplementation, and every escalation attached to `pass` or `blocker`. There is\nno fixed ordinary-rework count. After each correction, a fresh standard\nverifier checks the whole task again. Stop only on verifier `pass`, a genuine\nblocker, unsafe scope expansion, or a required user-owned decision.\n\nAfter an exact matching verifier object with verdict `pass`, the coordinator\ngives completion the opaque snapshot ID plus the accepted implementation and\nverifier summaries. Completion derives paths from the verified checkout and runs\nexactly one `zdev work-context <area> --compare <snapshot> --format json`\nbefore mutation and accepts only the exact compact schema for that area and ID\nwith `equal: true`. This fresh binary comparison covers area, ready task,\nlifecycle, safety, HEAD, index, worktree, and untracked state because all are\npart of the stored canonical context. A false comparison or an unavailable,\nexpired, corrupt, cross-area, or malformed artifact blocks before mutation.\nOn an accepted comparison, the coordinator runs `zdev task done`, stages only\nthe attributed task-owned files and exact generated task records, inspects the\nstaged diff, and runs `zdev commit`.\nCompletion or commit failure is a blocker that preserves and reports the exact\nstate. Public output begins with\n`PASS zdev-implement <area> <task-id>` or\n`BLOCKER zdev-implement <area> <task-id>`; its body repeats the exact area and\ntask, reports the stale advisory once when present, and names summary, changed\nfiles, validation, verifier evidence, and commit ID on pass, or the failed\nstage, reason, and preserved state on blocker. It omits the advisory field when\nno stale advisory was observed.\n\nAn ordinary `zdev-implement` pass completes one task. A successful split uses\nthe derived exception above and leaves its source open. After reporting the\nordinary verified commit or derived managed commit, it stops without querying\n`zdev next` or another `work-context`. A goal, loop, or explicit continuation\nowns the next iteration and must collect a fresh\n`zdev work-context <area> --format json` after the commit and before another\nworker dispatch. It never reuses the completed task's pre-commit selection.\n\n`zdev-verify <area> <task-id>` performs the same read-only preflight and requires\nthe explicit ID to equal the current ready task before starting one fresh\nconfigured verifier. It never invokes an implementer, changes lifecycle state,\nstages, commits, or routes a derived proposal. Its public result is the accepted verifier object above. Empty,\nexhausted, or closed goals, a different ready task, unsafe state, unavailable\nindependent verification, or an invalid worker envelope returns `BLOCKER zdev-verify`\nwithout mutation."
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
  if (expectedKind === 'verifier' && result.verdict === 'pass' && result.findings.length !== 0) return null
  if (expectedKind === 'verifier' && result.verdict === 'rework' && result.findings.length === 0) return null
  return validEscalation ? result : null
}
const approvedSnapshot = result => {
  const matches = result.evidence
    .map(item => /^work_context_snapshot: (W[0-9a-f]{16})$/.exec(item))
    .filter(Boolean)
  return matches.length === 1 ? matches[0][1] : null
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

const verified = await agent(
  `${workerContract}\n\nIndependently verify task ${taskId} in area ${area} from the current checkout. Before inspection or validation, run zdev work-context ${area} --store --format json and accept only its compact locator for the same open, ready, safe task and HEAD ${prepared.head}. Inspect that immutable context only through zdev work-context ${area} --show <snapshot> --format json. Check the whole task and run required validation, then run zdev work-context ${area} --compare <snapshot> --format json. Accept only the exact four-key compact result {"schema_version":1,"area":"${area}","snapshot":"<same-id>","equal":true}. Validation-written task-owned files are rework and ambiguous writes are blocker. A pass has empty findings and an evidence array containing exactly one work_context_snapshot: W<16-lowercase-hex> item${advisory ? ` and ${advisory} exactly once` : ''}. Rework has at least one concrete finding. Put checked locations and validation conclusions in summary. Return only the required strict JSON object with kind "verifier", area "${area}", and task_id "${taskId}". ${advisory ? '' : `Omit ${advisoryText} from evidence.`} Keep files, lifecycle, and Git state unchanged. Return the locator rather than a snapshot path or raw Git evidence.`,
  { agentType: 'zdev:zdev-verifier', label: 'zdev fresh verification' },
)
const result = verified?.trim()
const parsed = parseWorkerResult(result, 'verifier', area, taskId)
const approved = parsed?.verdict === 'pass' ? approvedSnapshot(parsed) : true
const advisoryCount = parsed?.evidence.filter(item => item === advisoryText).length
const passEvidenceCount = advisory ? 2 : 1
return parsed && approved && (parsed.verdict !== 'pass' || parsed.evidence.length === passEvidenceCount) && advisoryCount === (advisory ? 1 : 0)
  ? result
  : blocker(area, taskId, 'verifier returned invalid, extra, contradictory, or mismatched JSON.', prepared.staleAdvisory)

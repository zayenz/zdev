export const meta = {
  name: 'zdev-parallel',
  description: 'Implement a finite approved zdev task batch in isolated worktrees',
}

const repositoryGuidance = "<!-- zdev:generated-repository-guidance:start -->\n## Repository guidance discovery\n\nBefore inspecting, planning, changing, or validating code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.\n<!-- zdev:generated-repository-guidance:end -->\n\n## Coding guidance\n\nCoding guidance file: \"/Users/zayenz/projects/zdev/.claude/skills/zdev/skills/zdev/references/coding.md\"\n\nBefore planning code, read Design and Code standards from this file. Before implementing or repairing code, also read Completion and finish its naming and decomposition check before claiming readiness. Before verification, audit, or code review, read Code standards and Review and perform the explicit naming and decomposition check before a pass. Pass this exact location to each relevant worker; read it directly even when the root skill is not loaded. Routine coordination and non-code tasks do not need it."
const workerContract = repositoryGuidance
const taskWorkflowContractPath = "/Users/zayenz/projects/zdev/.claude/skills/zdev/contracts/task-workflows.md"
const object = raw => {
  if (raw && !Array.isArray(raw) && typeof raw === 'object') return raw
  if (typeof raw !== 'string') return null
  let depth = 0
  let start = -1
  let string = false
  const values = []
  for (let i = 0; i < raw.length; i += 1) {
    const c = raw[i]
    if (string) {
      if (c === '\\') i += 1
      else if (c === '"') string = false
    } else if (c === '"' && depth > 0) string = true
    else if (c === '{') { if (depth === 0) start = i; depth += 1 }
    else if (c === '}' && depth > 0 && --depth === 0) {
      try { values.push(JSON.parse(raw.slice(start, i + 1))) } catch {}
    }
  }
  return depth === 0 && values.length === 1 ? values[0] : null
}
const decodedObject = raw => {
  if (raw && !Array.isArray(raw) && typeof raw === 'object') return { value: raw, raw: JSON.stringify(raw) }
  if (typeof raw !== 'string') return null
  let depth = 0, start = -1, string = false
  const values = []
  for (let i = 0; i < raw.length; i += 1) {
    const c = raw[i]
    if (string) { if (c === '\\') i += 1; else if (c === '"') string = false }
    else if (c === '"' && depth > 0) string = true
    else if (c === '{') { if (depth === 0) start = i; depth += 1 }
    else if (c === '}' && depth > 0 && --depth === 0) {
      const text = raw.slice(start, i + 1)
      try { const value = JSON.parse(text); if (value && !Array.isArray(value)) values.push({ value, raw: text }) } catch {}
    }
  }
  return depth === 0 && values.length === 1 ? values[0] : null
}
const hasDuplicateObjectKeys = raw => {
  if (typeof raw !== 'string') return false
  const stack = []
  for (let i = 0; i < raw.length; i += 1) {
    if (raw[i] === '"') {
      const start = i++
      while (i < raw.length && raw[i] !== '"') { if (raw[i] === '\\') i += 1; i += 1 }
      if (i >= raw.length) return true
      let next = i + 1; while (/\s/.test(raw[next] ?? '')) next += 1
      const current = stack.at(-1)
      if (raw[next] === ':' && current instanceof Set) {
        let key; try { key = JSON.parse(raw.slice(start, i + 1)) } catch { return true }
        if (current.has(key)) return true; current.add(key)
      }
    } else if (raw[i] === '{') stack.push(new Set())
    else if (raw[i] === '[') stack.push(null)
    else if (raw[i] === '}' || raw[i] === ']') stack.pop()
  }
  return false
}
const topLevelKeys = raw => {
  let i = 0
  const keys = []
  const skip = () => { while (/\s/.test(raw[i] ?? '')) i += 1 }
  const string = () => {
    if (raw[i] !== '"') return null
    const start = i++
    while (i < raw.length) {
      if (raw[i] === '\\') i += 2
      else if (raw[i++] === '"') { try { return JSON.parse(raw.slice(start, i)) } catch { return null } }
    }
    return null
  }
  skip(); if (raw[i++] !== '{') return null
  while (true) {
    skip(); if (raw[i] === '}') { i += 1; break }
    const key = string(); if (key === null) return null
    keys.push(key); skip(); if (raw[i++] !== ':') return null; skip()
    let depth = 0, quoted = false
    while (i < raw.length) {
      const c = raw[i]
      if (quoted) { if (c === '\\') i += 1; else if (c === '"') quoted = false }
      else if (c === '"') quoted = true
      else if (c === '[' || c === '{') depth += 1
      else if (c === ']' || (c === '}' && depth > 0)) depth -= 1
      else if (depth === 0 && (c === ',' || c === '}')) break
      i += 1
    }
    if (quoted || depth !== 0) return null
    if (raw[i] === ',') { i += 1; continue }
    if (raw[i] === '}') { i += 1; break }
    return null
  }
  skip()
  return i === raw.length ? keys : null
}
const strictObject = (raw, keys) => {
  if (hasDuplicateObjectKeys(typeof raw === 'string' ? raw : '')) return null
  const decoded = decodedObject(raw)
  const actual = decoded && topLevelKeys(decoded.raw)
  return actual && new Set(actual).size === actual.length
    && JSON.stringify([...actual].sort()) === JSON.stringify([...keys].sort()) ? decoded.value : null
}
const exactKeys = (value, keys) => value && !Array.isArray(value) && typeof value === 'object'
  && JSON.stringify(Object.keys(value).sort()) === JSON.stringify([...keys].sort())
const roles = ['routine-implementer', 'implementer', 'advanced-implementer', 'planner', 'verifier']
const validRole = (record, role) => exactKeys(record,
  ['schema_version', 'profile', 'harness', 'role', 'value', 'origin', 'fallback'])
  && record.schema_version === 1 && record.role === role && record.harness === 'claude'
  && typeof record.profile === 'string' && record.profile
  && ((exactKeys(record.value, ['inherit']) && record.value.inherit === true)
    || (exactKeys(record.value, ['model', 'effort']) && typeof record.value.model === 'string'
      && record.value.model && ['inherit', 'low', 'medium', 'high', 'xhigh', 'max'].includes(record.value.effort)))
const absolute = value => typeof value === 'string' && value.startsWith('/')
const taskId = value => typeof value === 'string' && /^[a-z0-9][a-z0-9-]*-[0-9]+$/.test(value)
const input = args && !Array.isArray(args) && typeof args === 'object' ? args : {}
const area = String(input.area ?? '').trim()
const tasks = Array.isArray(input.task_ids) ? input.task_ids.map(value => String(value).trim()) : []
const destinationRoot = String(input.destination_root ?? '').trim()
const destinationBranch = String(input.destination_branch ?? '').trim()
const limit = Number(input.worker_limit ?? 4)
const cleanup = input.cleanup === true
const consent = input.consent === true
const resume = Array.isArray(input.resume) ? input.resume : []
const runProfile = input.run_profile ?? input.runProfile ?? null
const frozenProfiles = input.frozen_profiles ?? input.frozenProfiles ?? null
const requestedRoleProfiles = input.role_profiles ?? input.roleProfiles ?? {}
const roleProfiles = requestedRoleProfiles && !Array.isArray(requestedRoleProfiles)
  && typeof requestedRoleProfiles === 'object' ? requestedRoleProfiles : {}
const resumeKeys = ['task_id', 'worktree', 'branch', 'baseline', 'status', 'commit', 'last_result']
const validResume = resume.every(item => exactKeys(item, resumeKeys) && tasks.includes(item.task_id)
  && absolute(item.worktree) && typeof item.branch === 'string' && item.branch && /^[0-9a-f]{40}$/.test(item.baseline)
  && ['committed', 'unfinished', 'completion'].includes(item.status)
  && (item.status === 'committed' ? /^[0-9a-f]{40}$/.test(item.commit) : item.commit === null)
  && (item.last_result === null || typeof item.last_result === 'string'))
const invalid = !/^[a-z0-9][a-z0-9-]*$/.test(area) || tasks.length < 2
  || new Set(tasks).size !== tasks.length || !tasks.every(taskId)
  || !absolute(destinationRoot) || !destinationBranch || !Number.isInteger(limit) || limit < 1 || !validResume
if (frozenProfiles !== null && (!exactKeys(frozenProfiles, roles) || !roles.every(role => validRole(frozenProfiles[role], role)))) {
  return `BLOCKER zdev-parallel ${area}\n\nReason: invalid frozen worker profiles; no admission or worker was started.`
}
if (invalid || !consent) {
  return `BLOCKER zdev-parallel ${area || 'unknown'}\n\nReason: require a finite unique task_ids batch, absolute destination_root, destination_branch, positive worker_limit, cleanup choice, and explicit consent.`
}
const unfinishedRecovery = resume.filter(item => item.status === 'unfinished')
if (unfinishedRecovery.length > 0) {
  return `BLOCKER zdev-parallel ${area}\n\nReason: use the installed Recover procedure for unfinished tasks; this executable cannot retain their review stage and correction allowance.\nPreserved state: ${unfinishedRecovery.map(item => `${item.task_id} ${item.worktree} ${item.branch} ${item.baseline}; last result=${item.last_result ?? 'unknown'}`).join(' | ')}`
}
const recoveredCompletions = []
const recoveryCompletionKeys = ['task_id', 'status', 'summary', 'commit', 'findings', 'preserved']
const recoveryCompletionSchema = { type: 'object', additionalProperties: false, required: recoveryCompletionKeys,
  properties: { task_id: { type: 'string' }, status: { enum: ['committed', 'preserved'] }, summary: { type: 'string' },
    commit: { anyOf: [{ type: 'null' }, { type: 'string' }] }, findings: { type: 'array', items: { type: 'string' } }, preserved: { type: 'string' } } }
for (const retained of resume.filter(item => item.status === 'completion')) {
  const recovered = strictObject(await agent(
    `${repositoryGuidance}\n\nRecover the existing completion checkpoint for task ${retained.task_id} first, before any new admission or dispatch. Authoritative checkout: ${destinationRoot}; retained worktree ${retained.worktree}, branch ${retained.branch}, baseline ${retained.baseline}, last result ${JSON.stringify(retained.last_result)}. Inspect current task and Git state, obtain fresh explicit-task admission, and finish only the normal snapshot comparison, task completion, staging, and one zdev commit when safe. Do not spawn an agent or depend on an old agent ID. Preserve the checkpoint otherwise.`,
    { label: `zdev ${retained.task_id}: recover completion checkpoint`, schema: recoveryCompletionSchema },
  ), recoveryCompletionKeys)
  if (!recovered || recovered.task_id !== retained.task_id || !['committed', 'preserved'].includes(recovered.status)
    || (recovered.status === 'committed' ? !/^[0-9a-f]{40}$/.test(recovered.commit) : recovered.commit !== null)) {
    return `BLOCKER zdev-parallel ${area}\n\nReason: malformed completion recovery result.\nPreserved state: ${retained.worktree} ${retained.branch} ${retained.baseline}`
  }
  if (recovered.status === 'preserved') {
    return `BLOCKER zdev-parallel ${area}\n\nReason: completion recovery remains unresolved; use the installed Recover procedure before any new admission or dispatch.\nPreserved state: ${JSON.stringify(recovered)}`
  }
  recoveredCompletions.push(recovered)
}
const remainingTasks = tasks.filter(id => !resume.some(item => item.task_id === id && item.status === 'committed')
  && !recoveredCompletions.some(item => item.task_id === id && item.status === 'committed'))
const preparationSchema = {
  type: 'object', additionalProperties: false,
  required: ['area', 'destination_branch', 'destination_root', 'assignments'],
  properties: {
    area: { type: 'string' }, destination_branch: { type: 'string' }, destination_root: { type: 'string' },
    assignments: { type: 'array', items: { type: 'object', additionalProperties: false,
      required: ['task_id', 'worktree', 'branch', 'baseline', 'snapshot', 'complexity', 'task_path'],
      properties: { task_id: { type: 'string' }, worktree: { type: 'string' }, branch: { type: 'string' },
        baseline: { type: 'string' }, snapshot: { type: 'string' }, complexity: { type: 'string' }, task_path: { type: 'string' } } } },
  },
}
const preparedRaw = await agent(
  `${repositoryGuidance}\n\nAct as the admission and assigned-worktree coordinator for the explicitly approved parallel batch ${JSON.stringify(tasks)} in area ${area}. The authoritative checkout is ${destinationRoot} on branch ${destinationBranch}. Read applicable instructions and ${JSON.stringify(taskWorkflowContractPath)}. Explicit recovery input: ${JSON.stringify(resume)}. On recovery, inspect retained worktree, branch, baseline, accepted commits, last result, and current task records; exclude committed tasks, finish a safe existing completion checkpoint before dispatch, and obtain fresh admission. Never rely on an old agent ID. Otherwise create one zdev-owned ordinary Git branch and linked worktree from the inspected destination HEAD per admitted task. Inspect every requested and resumed task for hypothesis and reject hypotheses before creating worktrees or starting workers: automatic experiment execution is not yet supported; use the research record contract in the installed task-format reference. Reject unsafe, dependent, overlapping, or resource-conflicting work. Do not copy or edit authoritative .zdev records. Return only the requested assignment object, preserving remaining input task order.`,
  { label: `zdev ${area}: admit parallel batch`, schema: preparationSchema },
)
const prepared = strictObject(preparedRaw, ['area', 'destination_branch', 'destination_root', 'assignments'])
const assignmentKeys = ['task_id', 'worktree', 'branch', 'baseline', 'snapshot', 'complexity', 'task_path']
const assignments = prepared?.assignments
const validPrepared = exactKeys(prepared, ['area', 'destination_branch', 'destination_root', 'assignments'])
  && prepared.area === area && prepared.destination_root === destinationRoot
  && prepared.destination_branch === destinationBranch && Array.isArray(assignments)
  && assignments.length === remainingTasks.length && assignments.every((item, index) => exactKeys(item, assignmentKeys)
    && item.task_id === remainingTasks[index] && absolute(item.worktree) && /^[0-9a-f]{40}$/.test(item.baseline)
    && /^W[0-9a-f]{16}$/.test(item.snapshot) && ['routine', 'standard', 'advanced'].includes(item.complexity)
    && typeof item.branch === 'string' && item.branch && typeof item.task_path === 'string')
if (!validPrepared) {
  return `BLOCKER zdev-parallel ${area}\n\nReason: admission did not return exact safe assignments.\nPreserved state: inspect any reported worktrees before retrying.`
}
let batchProfiles = frozenProfiles
if (assignments.length > 0 && !batchProfiles) {
  const first = assignments[0]
  const flags = `${runProfile ? ` --run-profile ${String(runProfile)}` : ''}${Object.entries(roleProfiles).map(([role, profile]) => ` --role-profile ${role}=${profile}`).join('')}`
  const raw = await agent(
    `${repositoryGuidance}\n\nAct only as deterministic batch profile coordination in ${destinationRoot}. Run zdev config profile dispatch-spec implement --harness claude --area ${area} --task ${first.task_id} --snapshot ${first.snapshot}${flags} --format json exactly once and return its complete JSON stdout unchanged. Keep files and configuration unchanged.`,
    { label: `zdev ${area}: freeze parallel worker profiles`, model: 'claude-opus-5-5', effort: 'low' },
  )
  const spec = strictObject(raw, ['schema_version', 'kind', 'harness', 'route', 'area', 'task_id', 'snapshot', 'dispatches', 'profiles', 'stop'])
  const exact = (value, keys) => value && !Array.isArray(value) && typeof value === 'object'
    && JSON.stringify(Object.keys(value).sort()) === JSON.stringify([...keys].sort())
  const implementationRole = first.complexity === 'routine' ? 'routine-implementer'
    : first.complexity === 'advanced' ? 'advanced-implementer' : 'implementer'
  const expectedRoles = first.complexity === 'advanced'
    ? ['planner', implementationRole, 'verifier'] : [implementationRole, 'verifier']
  const expectedPhases = first.complexity === 'advanced'
    ? ['implementation', 'verification', 'completion'] : ['verification', 'completion']
  if (!exact(spec, ['schema_version', 'kind', 'harness', 'route', 'area', 'task_id', 'snapshot', 'dispatches', 'profiles', 'stop'])
    || spec.schema_version !== 1 || spec.kind !== 'dispatch-spec' || spec.harness !== 'claude'
    || spec.route !== 'implement' || spec.area !== area || spec.task_id !== first.task_id
    || spec.snapshot !== first.snapshot || spec.stop !== 'completion'
    || !exact(spec.profiles, roles) || !roles.every(role => validRole(spec.profiles[role], role))
    || !Array.isArray(spec.dispatches)
    || JSON.stringify(spec.dispatches.map(item => item.role)) !== JSON.stringify(expectedRoles)
    || JSON.stringify(spec.dispatches.map(item => item.next_phase)) !== JSON.stringify(expectedPhases)
    || spec.dispatches.some(item => !exact(item, ['role', 'profile', 'model', 'reasoning_effort', 'task_id', 'snapshot', 'next_phase'])
      || item.task_id !== first.task_id || item.snapshot !== first.snapshot
      || item.profile !== spec.profiles[item.role]?.profile
      || item.model !== (spec.profiles[item.role]?.value.model ?? null)
      || item.reasoning_effort !== (spec.profiles[item.role]?.value.effort ?? null))) {
    return `BLOCKER zdev-parallel ${area}\n\nReason: profile resolution failed before worker dispatch.\nPreserved state: inspect admitted worktrees before retrying.`
  }
  batchProfiles = spec.profiles
}
const profiled = (role, options) => {
  if (!batchProfiles) return options
  const value = batchProfiles?.[role]?.value
  if (value?.inherit === true) return options
  return value?.effort === 'inherit' ? { ...options, model: value.model }
    : { ...options, model: value?.model, effort: value?.effort }
}

const implementerSchema = {
  type: 'object', additionalProperties: false,
  required: ['schema_version', 'kind', 'area', 'task_id', 'verdict', 'summary', 'evidence', 'findings', 'escalation'],
  properties: { schema_version: { const: 1 }, kind: { const: 'implementer' }, area: { const: area },
    task_id: { type: 'string' }, verdict: { enum: ['ready', 'blocker'] }, summary: { type: 'string' },
    evidence: { type: 'array', items: { type: 'string' } }, findings: { type: 'array', items: { type: 'string' } },
    escalation: { const: 'none' } },
}
const plannerKeys = ['verdict', 'summary', 'plan', 'findings']
const planKeys = ['approach', 'paths', 'validation']
const normalizedPath = path => typeof path === 'string' && path.trim() === path && path.length > 0
  && !path.includes('\\') && path.split('/').filter((_, index) => !(index === 0 && path.startsWith('/')))
    .every(part => part && part !== '.' && part !== '..')
const parsePlanner = raw => {
  const result = strictObject(raw, plannerKeys)
  if (!result || typeof result.summary !== 'string' || !result.summary.trim()
    || !Array.isArray(result.findings) || !result.findings.every(item => typeof item === 'string' && item.trim())) return null
  if (result.verdict === 'blocker') return result.plan === null && result.findings.length > 0 ? result : null
  if (result.verdict !== 'plan' || !exactKeys(result.plan, planKeys)
    || typeof result.plan.approach !== 'string' || !result.plan.approach.trim()
    || !Array.isArray(result.plan.paths) || result.plan.paths.length === 0 || !result.plan.paths.every(normalizedPath)
    || !Array.isArray(result.plan.validation) || result.plan.validation.length === 0
    || !result.plan.validation.every(item => typeof item === 'string' && item.trim())) return null
  return result
}
const validImplementer = (result, expectedTask) => exactKeys(result,
  ['schema_version', 'kind', 'area', 'task_id', 'verdict', 'summary', 'evidence', 'findings', 'escalation'])
  && result.schema_version === 1 && result.kind === 'implementer' && result.area === area
  && result.task_id === expectedTask && ['ready', 'blocker'].includes(result.verdict)
  && typeof result.summary === 'string' && result.summary.trim()
  && Array.isArray(result.evidence) && result.evidence.every(item => typeof item === 'string' && item.trim())
  && Array.isArray(result.findings) && result.findings.every(item => typeof item === 'string' && item.trim())
  && result.escalation === 'none'
const parseImplementer = (raw, expectedTask) => {
  const result = strictObject(raw, ['schema_version', 'kind', 'area', 'task_id', 'verdict', 'summary', 'evidence', 'findings', 'escalation'])
  return validImplementer(result, expectedTask) ? result : null
}
const runLane = async assignment => {
  let plan = null
  if (assignment.complexity === 'advanced') {
    plan = parsePlanner(await agent(
      `${repositoryGuidance}\n\nPlan task ${assignment.task_id} read-only. Load authoritative snapshot ${assignment.snapshot} from ${destinationRoot}; source work will occur at ${assignment.worktree} from baseline ${assignment.baseline}. Return the semantic planner object required by the installed task-workflows contract.`,
      profiled('planner', { agentType: 'zdev:zdev-planner', label: `zdev ${assignment.task_id}: plan` }),
    ))
    if (!plan || plan.verdict === 'blocker') {
      return { task_id: assignment.task_id, assignment, result: null,
        failure: plan?.verdict === 'blocker' ? `planner blocker: ${plan.summary}; ${plan.findings.join('; ')}` : 'invalid planner result' }
    }
  }
  const profile = assignment.complexity === 'routine' ? 'zdev:zdev-routine-implementer'
    : assignment.complexity === 'advanced' ? 'zdev:zdev-advanced-implementer' : 'zdev:zdev-implementer'
  const raw = await agent(
    `${workerContract}\n\nImplement only task ${assignment.task_id} in assigned source worktree ${assignment.worktree}. Use that absolute path as cwd for every source, Git, and validation command. The authoritative checkout is ${destinationRoot}; read task record ${assignment.task_path} and snapshot ${assignment.snapshot} there. Baseline is ${assignment.baseline}; branch is ${assignment.branch}. Never edit .zdev, stage, commit, complete, integrate, or use Claude native isolation. Return the implementer envelope with task_id ${assignment.task_id}. ${plan === null ? '' : `Validated planner result: ${JSON.stringify(plan)}`}`,
    profiled(assignment.complexity === 'routine' ? 'routine-implementer'
      : assignment.complexity === 'advanced' ? 'advanced-implementer' : 'implementer',
    { agentType: profile, label: `zdev ${assignment.task_id}: implement`, schema: implementerSchema }),
  )
  const result = parseImplementer(raw, assignment.task_id)
  return { task_id: assignment.task_id, assignment, result,
    failure: validImplementer(result, assignment.task_id) ? null : 'malformed or mismatched implementer result' }
}
const gateSchema = {
  type: 'object', additionalProperties: false,
  required: ['task_id', 'status', 'summary', 'commit', 'findings', 'preserved'],
  properties: { task_id: { type: 'string' }, status: { enum: ['committed', 'blocked', 'shared-decision', 'preserved'] },
    summary: { type: 'string' }, commit: { anyOf: [{ type: 'null' }, { type: 'string' }] },
    findings: { type: 'array', items: { type: 'string' } }, preserved: { type: 'string' } },
}
const transportSchema = {
  type: 'object', additionalProperties: false,
  required: ['task_id', 'status', 'head', 'summary', 'findings', 'preserved'],
  properties: { task_id: { type: 'string' }, status: { enum: ['integrated', 'blocked', 'shared-decision'] },
    head: { anyOf: [{ type: 'null' }, { type: 'string' }] }, summary: { type: 'string' },
    findings: { type: 'array', items: { type: 'string' } }, preserved: { type: 'string' } },
}
const verifierKeys = ['verdict', 'summary', 'findings', 'escalation']
const parseVerifier = raw => {
  const result = strictObject(raw, verifierKeys)
  if (!result || !['pass', 'rework', 'blocker'].includes(result.verdict)
    || typeof result.summary !== 'string' || !result.summary.trim() || !Array.isArray(result.findings)
    || !result.findings.every(item => typeof item === 'string' && item.trim())) return null
  if (result.verdict === 'pass' && (result.findings.length > 0 || result.escalation !== 'none')) return null
  if (result.verdict === 'rework' && (result.findings.length === 0
    || !['none', 'advanced-implementer'].includes(result.escalation))) return null
  if (result.verdict === 'blocker' && result.escalation !== 'none') return null
  return result
}
const comparisonSchema = { type: 'object', additionalProperties: false,
  required: ['schema_version', 'area', 'snapshot', 'equal'], properties: {
    schema_version: { const: 1 }, area: { const: area }, snapshot: { type: 'string' }, equal: { type: 'boolean' } } }
const admissionKeys = ['schema_version', 'area', 'task_id', 'head', 'snapshot', 'lifecycle', 'queue', 'safe', 'stale_advisory', 'checkout']
const admissionSchema = { type: 'object', additionalProperties: false, required: admissionKeys,
  properties: { schema_version: { const: 1 }, area: { const: area }, task_id: { type: 'string' },
    head: { type: 'string' }, snapshot: { type: 'string' }, lifecycle: { type: 'string' }, queue: { type: 'string' },
    safe: { type: 'boolean' }, stale_advisory: { type: 'boolean' }, checkout: { type: 'string' } } }
const preserved = (candidate, summary, findings = []) => ({ task_id: candidate.task_id, status: 'preserved',
  summary, commit: null, findings, preserved: `${candidate.assignment.worktree} ${candidate.assignment.branch} ${candidate.assignment.baseline}` })
let destinationPending = false
let staleAdvisory = false
const runDestinationGate = async candidate => {
  let implementation = candidate.result
  let escalated = false
  let priorReview = null
  while (true) {
    // A ready source may mutate the destination even if transport returns an invalid result.
    destinationPending ||= implementation.verdict === 'ready'
    const transported = strictObject(await agent(
      `${repositoryGuidance}\n\nAct only as serial source transport coordination for parallel task ${candidate.task_id}. Authoritative destination: ${destinationRoot} on ${destinationBranch}. Assigned source: ${candidate.assignment.worktree}, branch ${candidate.assignment.branch}, baseline ${candidate.assignment.baseline}. Worker result: ${JSON.stringify(implementation)}. Carry task ID ${candidate.task_id} through every refresh. Inspect the full source delta. For ready work, create and inspect an ordinary source commit, use the assigned-worktree no-commit transport, refresh explicit-task admission, and capture a fresh work-context snapshot with --task ${candidate.task_id}. Do not verify, complete, stage the destination, commit the destination, or spawn an agent. For an implementer blocker or derived proposal, preserve it and classify only whether it is task-local or requires a shared decision.`,
      { label: `zdev ${candidate.task_id}: serial transport`, schema: transportSchema },
    ), ['task_id', 'status', 'head', 'summary', 'findings', 'preserved'])
    const validTransport = transported
      && transported.task_id === candidate.task_id && ['integrated', 'blocked', 'shared-decision'].includes(transported.status)
      && typeof transported.summary === 'string' && transported.summary.trim() && Array.isArray(transported.findings)
      && transported.findings.every(item => typeof item === 'string' && item.trim()) && typeof transported.preserved === 'string'
      && (transported.status === 'integrated' ? /^[0-9a-f]{40}$/.test(transported.head) : transported.head === null)
    if (!validTransport) return preserved(candidate, 'transport returned a malformed result')
    if (transported.status !== 'integrated') return { task_id: candidate.task_id,
      status: transported.status, summary: transported.summary, commit: null,
      findings: transported.findings, preserved: transported.preserved }
    const admitted = strictObject(await agent(
      `Act only as fresh verification admission coordination for task ${candidate.task_id}. In authoritative checkout ${destinationRoot}, run zdev work-context ${area} --task ${candidate.task_id} --store --format json exactly once, then show that exact snapshot once. Validate the complete shown context and return only the requested projection. Keep files and Git state unchanged.`,
      { label: `zdev ${candidate.task_id}: fresh verifier admission`, schema: admissionSchema },
    ), admissionKeys)
    const validAdmission = admitted && admitted.schema_version === 1 && admitted.area === area
      && admitted.task_id === candidate.task_id && admitted.head === transported.head
      && /^W[0-9a-f]{16}$/.test(admitted.snapshot) && admitted.lifecycle === 'open' && admitted.queue === 'ready'
      && admitted.safe === true && typeof admitted.stale_advisory === 'boolean' && admitted.checkout === destinationRoot
    if (!validAdmission) return preserved(candidate, 'fresh verifier admission was malformed, mismatched, or unsafe', transported.findings)
    staleAdvisory ||= admitted.stale_advisory
    const snapshot = admitted.snapshot
    let verification = parseVerifier(await agent(
      `${workerContract}\n\nIndependently verify parallel task ${candidate.task_id} in authoritative checkout ${destinationRoot}. Load only fresh snapshot ${snapshot} with zdev work-context ${area} --show ${snapshot} --format json and require exact task ${candidate.task_id}. ${priorReview ? `Perform the focused recheck under the coding guidance Review reference: check accepted corrections, affected contracts, and repair regressions; retain conclusions for unchanged code. Prior review and accepted corrections: ${JSON.stringify(priorReview)}.` : "Check the whole task against requirements and coding guidance."} Run required validation. Put optional suggestions in summary, never findings. Keep verification read-only and return exactly the four-key semantic verifier object.`,
      profiled('verifier', { agentType: 'zdev:zdev-verifier', label: `zdev ${candidate.task_id}: verify destination` }),
    ))
    const compared = strictObject(await agent(
      `Act only as deterministic verification coordination. Run zdev work-context ${area} --compare ${snapshot} --format json exactly once in ${destinationRoot} and return its JSON stdout unchanged.`,
      { label: `zdev ${candidate.task_id}: compare verification snapshot`, schema: comparisonSchema },
    ), ['schema_version', 'area', 'snapshot', 'equal'])
    if (!verification || !compared
      || compared.schema_version !== 1 || compared.area !== area || compared.snapshot !== snapshot) {
      return preserved(candidate, 'invalid verifier or snapshot comparison result', verification?.findings ?? [])
    }
    const validationWrites = verification.findings.filter(item => item.startsWith('validation_write:'))
    const validWrites = verification.verdict === 'rework' && validationWrites.length > 0 && validationWrites.every(item => {
      if (!item.startsWith('validation_write: ')) return false
      const path = item.slice('validation_write: '.length)
      return !path.startsWith('/') && !path.includes('\\')
        && path.split('/').every(part => part && part !== '.' && part !== '..')
    })
    if (validationWrites.length > 0 && !validWrites) return preserved(candidate, 'malformed validation-write finding; verification remains incomplete')
    if (!compared.equal && !(verification.verdict === 'rework' && validWrites)) {
      return preserved(candidate, 'verifier changed the destination without attributable validation writes', verification.findings)
    }
    if (compared.equal && verification.verdict === 'rework' && validWrites) {
      verification.findings = verification.findings.filter(item => !item.startsWith('validation_write:'))
      if (verification.findings.length === 0) {
        verification.verdict = 'pass'
        verification.escalation = 'none'
      }
    }
    if (verification.verdict === 'rework' && verification.findings.some(item => !item.startsWith('validation_write:'))) {
      if (!compared.equal) return preserved(candidate, 'substantive findings require a fresh review of the changed destination')
      const vetted = parseVerifier(await agent(
        `${workerContract}\n\nCheck substantive findings for parallel task ${candidate.task_id} independently of their author. Work read-only in ${destinationRoot}; load context with zdev work-context ${area} --show ${snapshot} --format json. Follow Review in the supplied coding guidance. Inspect locations, callers, rules, consequences, severity, and counterevidence without repeating the whole review or rerunning validation. Preserve original validation conclusions. Return the reconciled four-field semantic verifier object: rework for supported corrections, pass only when all substantive findings are disproved and the original review otherwise established completion, or blocker for unresolved claims or missing evidence. Explain removed or corrected claims in summary. Do not add new findings, validation_write markers, or escalate beyond the candidate. Do not delegate or edit.\n\nCandidate review: ${JSON.stringify(verification)}`,
        profiled('verifier', { agentType: 'zdev:zdev-verifier', label: `zdev ${candidate.task_id}: check findings` }),
      ))
      const unchanged = strictObject(await agent(
        `Act only as deterministic findings coordination. Run zdev work-context ${area} --compare ${snapshot} --format json exactly once in ${destinationRoot} and return its JSON stdout unchanged.`,
        { label: `zdev ${candidate.task_id}: confirm findings snapshot unchanged`, schema: comparisonSchema },
      ), ['schema_version', 'area', 'snapshot', 'equal'])
      if (!vetted || !unchanged || unchanged.schema_version !== 1 || unchanged.area !== area
        || unchanged.snapshot !== snapshot || unchanged.equal !== true
        || vetted.findings.some(item => item.startsWith('validation_write:'))
        || (vetted.escalation !== 'none' && vetted.escalation !== verification.escalation)) {
        return preserved(candidate, 'finding verification was invalid or the reviewed destination changed; findings remain unchecked')
      }
      verification = vetted
    }
    if (verification.verdict === 'blocker') return { task_id: candidate.task_id, status: 'blocked',
      summary: verification.summary, commit: null, findings: verification.findings, preserved: transported.preserved }
    if (verification.verdict === 'rework') {
      if (priorReview) return preserved(candidate, 'required corrections remain after one correction pass; correction allowance exhausted, explicit user continuation needed', verification.findings)
      priorReview = { snapshot, ...verification }
      if (verification.escalation === 'advanced-implementer') {
        if (candidate.assignment.complexity !== 'standard' || escalated) return preserved(candidate, 'invalid verifier escalation', verification.findings)
        escalated = true
      }
      const profile = escalated ? 'zdev:zdev-advanced-implementer'
        : candidate.assignment.complexity === 'routine' ? 'zdev:zdev-routine-implementer'
        : candidate.assignment.complexity === 'advanced' ? 'zdev:zdev-advanced-implementer' : 'zdev:zdev-implementer'
      const corrected = parseImplementer(await agent(
        `${workerContract}\n\nCorrect every verifier finding for task ${candidate.task_id} in assigned source worktree ${candidate.assignment.worktree}. Use that absolute cwd for all source and validation commands. Reconcile against the currently integrated destination ${destinationRoot}; do not edit .zdev, integrate, complete, stage, or commit. Findings: ${JSON.stringify(verification)}`,
        profiled(escalated || candidate.assignment.complexity === 'advanced' ? 'advanced-implementer'
          : candidate.assignment.complexity === 'routine' ? 'routine-implementer' : 'implementer',
        { agentType: profile, label: `zdev ${candidate.task_id}: rework`, schema: implementerSchema }),
      ), candidate.task_id)
      if (!validImplementer(corrected, candidate.task_id)) return preserved(candidate, 'rework returned an invalid result', verification.findings)
      implementation = corrected
      continue
    }
    const completed = strictObject(await agent(
      `${repositoryGuidance}\n\nAct only as completion coordinator for verified parallel task ${candidate.task_id} in ${destinationRoot}. Before mutation compare snapshot ${snapshot} exactly once and require equal true. Then run zdev task done, stage only attributable task files and exact generated task records, inspect the staged diff, and run exactly one zdev commit. Do not spawn an agent. Return the requested committed result with full commit ID, or preserve the exact checkpoint on failure. Verification: ${JSON.stringify(verification)}`,
      { label: `zdev ${candidate.task_id}: complete and commit`, schema: gateSchema },
    ), ['task_id', 'status', 'summary', 'commit', 'findings', 'preserved'])
    const validCompletion = completed
      && completed.task_id === candidate.task_id && ['committed', 'blocked', 'shared-decision', 'preserved'].includes(completed.status)
      && typeof completed.summary === 'string' && completed.summary.trim() && Array.isArray(completed.findings)
      && typeof completed.preserved === 'string'
      && (completed.status === 'committed' ? /^[0-9a-f]{40}$/.test(completed.commit) : completed.commit === null)
    if (validCompletion && completed.status === 'committed') destinationPending = false
    return validCompletion ? completed : preserved(candidate, 'completion returned a malformed result', verification.findings)
  }
}
const outcomes = [...resume.filter(item => item.status === 'committed').map(item => ({ task_id: item.task_id,
  status: 'committed', summary: 'accepted committed recovery task', commit: item.commit, findings: [], preserved: '' })),
...recoveredCompletions]
let sharedDecision = false
const pending = [...assignments]
const active = new Set()
const arrivals = []
let wake = null
let stopDispatch = false
const signal = event => { arrivals.push(event); if (wake) { wake(); wake = null } }
const launch = assignment => {
  const promise = Promise.resolve().then(() => runLane(assignment))
    .then(value => ({ assignment, value }), error => ({ assignment, value: null, error: String(error) }))
    .then(event => { active.delete(promise); signal(event) })
  active.add(promise)
}
const fill = () => { while (!stopDispatch && pending.length > 0 && active.size + arrivals.length < limit) launch(pending.shift()) }
fill()
while (active.size > 0 || arrivals.length > 0) {
  if (arrivals.length === 0) await new Promise(resolve => { wake = resolve })
  const arrived = arrivals.shift()
  const candidate = arrived.value ?? { task_id: arrived.assignment.task_id, assignment: arrived.assignment,
    result: null, failure: arrived.error ?? 'cancelled worker' }
  if (candidate.failure) {
    stopDispatch = true
    outcomes.push(preserved(candidate, candidate.failure))
  } else if (sharedDecision || destinationPending) {
    outcomes.push(preserved(candidate, destinationPending ? 'paused by unresolved destination checkpoint' : 'paused by shared decision'))
  } else {
    const gated = await runDestinationGate(candidate)
    outcomes.push(gated)
    if (gated.status === 'shared-decision') { sharedDecision = true; stopDispatch = true }
    else if (gated.status === 'preserved' || destinationPending) stopDispatch = true
  }
  fill()
}
for (const assignment of pending) outcomes.push(preserved({ task_id: assignment.task_id, assignment }, 'not dispatched after cancellation or capacity loss'))
const committed = outcomes.filter(item => item.status === 'committed')
const unfinished = outcomes.filter(item => item.status !== 'committed')
return `${unfinished.length === 0 ? 'PASS' : 'BLOCKER'} zdev-parallel ${area}\n\nArea: ${area}\nDestination: ${destinationRoot} (${destinationBranch})\nCommitted tasks: ${committed.map(item => `${item.task_id} ${item.commit}`).join(', ') || 'none'}\nUnfinished tasks: ${unfinished.map(item => `${item.task_id} [${item.status}] ${item.summary}; findings=${item.findings.join('; ') || 'none'}; preserved=${item.preserved}`).join(' | ') || 'none'}\nBase advisory: ${staleAdvisory ? 'stale base; safe task work continued' : 'none'}\nCleanup: ${cleanup && unfinished.length === 0 ? 'authorized for stopped, integrated run-owned worktrees' : 'withheld; retain unfinished or ambiguous work'}\nStop reason: ${unfinished.length === 0 ? 'approved batch committed' : sharedDecision ? 'shared decision required' : 'one or more task results require recovery'}`

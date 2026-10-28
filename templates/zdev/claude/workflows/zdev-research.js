export const meta = {
  name: 'zdev-research',
  description: 'Authorize or resume serial or rolling research with counted workers and durable recovery',
}

const runOneTask = async (args, agent) => {
__ZDEV_ONE_TASK_BODY__
}

const repositoryGuidance = {{repository_guidance}}
const taskWorkflowContractPath = {{task_workflows_contract_path_json}}
const researchReferencePath = {{claude_research_reference_path_json}}
const codingReferencePath = {{claude_coding_reference_path_json}}
__ZDEV_ROLE_RESULT_HELPERS__

const roles = ['routine-implementer', 'implementer', 'advanced-implementer', 'planner', 'verifier']
const exactObject = (value, keys) => value && !Array.isArray(value) && typeof value === 'object'
  && JSON.stringify(Object.keys(value).sort()) === JSON.stringify([...keys].sort())
const validText = value => typeof value === 'string' && value.trim() === value && value.length > 0
const json = raw => {
  const decoded = decodeJsonObject(raw)
  return decoded && !hasDuplicateObjectKeys(decoded.raw) ? decoded.value : null
}
const validRoleRecord = (record, role) => exactObject(record,
  ['schema_version', 'profile', 'harness', 'role', 'value', 'origin', 'fallback'])
  && record.schema_version === 1 && record.harness === 'claude' && record.role === role
  && validText(record.profile) && ((exactObject(record.value, ['inherit']) && record.value.inherit === true)
    || (exactObject(record.value, ['model', 'effort']) && validText(record.value.model)
      && ['inherit', 'low', 'medium', 'high', 'xhigh', 'max'].includes(record.value.effort)))
const validProfiles = profiles => exactObject(profiles, roles)
  && roles.every(role => validRoleRecord(profiles[role], role))
const input = args && !Array.isArray(args) && typeof args === 'object' ? args : null
const area = input?.area
const invocation = input?.invocation_id
const mode = input?.authorization
const execution = input?.execution ?? 'serial'
const capacities = ['worker_limit', 'measurement_limit']
const limits = ['attempt_limit', 'command_limit', 'command_seconds', 'time_limit_seconds', 'target']
let nativeCalls = 0
let reporting = false
// Claude documents 1,000 agents per workflow. Keep room for draining and live-state reporting.
const callAgent = (prompt, options) => {
  if (nativeCalls >= (reporting ? 1_000 : 980)) throw new Error('Workflow call boundary reached; preserve effects and resume in a fresh invocation')
  nativeCalls++
  return agent(prompt, options)
}
let root = null
let runId = null
let startAttempted = false
let profiles = input?.frozen_profiles ?? null
const retainedPlans = input?.retained_plans ?? {}
const retainedCandidates = input?.retained_candidates ?? {}
let context = null
let requestNumber = 0
let pendingRecoveryVerification = null
const completed = []
const recovery = []
const nextRequest = kind => `${invocation}-${kind}-${++requestNumber}`
const report = (verdict, reason, crossAreaWaits = null) => JSON.stringify({
  schema_version: 1, kind: 'research', area: area ?? 'unknown', run_id: runId,
  scope: context?.run?.scope ?? input?.scope ?? null,
  completed: [...completed, ...(context?.checkpoints ?? [])
    .filter(checkpoint => checkpoint.index_updated && !completed.some(item => item.task_id === checkpoint.task))
    .map(checkpoint => ({ task_id: checkpoint.task, decision: checkpoint.final_record.decision, commit: checkpoint.commit }))],
  accepted_baseline: context?.accepted_baseline ?? null,
  remaining: context?.remaining ?? null, stop_reason: reason,
  execution, cleanup: input?.cleanup === true, native_calls: nativeCalls, recovery_actions: recovery, frozen_profiles: profiles, retained_plans: retainedPlans, retained_candidates: retainedCandidates, verdict,
  ...(crossAreaWaits ? { cross_area_waits: crossAreaWaits } : {}),
})
if (!input || !/^[a-z0-9][a-z0-9-]*$/.test(area ?? '')
  || !/^[A-Za-z0-9][A-Za-z0-9_-]*$/.test(invocation ?? '')
  || !['start', 'resume'].includes(mode)
  || !['serial', 'rolling'].includes(execution)
  || (Object.hasOwn(input, 'cleanup') && typeof input.cleanup !== 'boolean')
  || capacities.some(key => Object.hasOwn(input, key) && (!Number.isInteger(input[key]) || input[key] <= 0))
  || (mode === 'start' && !validText(input.scope))
  || (mode === 'resume' && !validText(input.run_id))
  || (Object.hasOwn(input, 'until_interrupted') && typeof input.until_interrupted !== 'boolean')
  || limits.some(key => Object.hasOwn(input, key)
    && (typeof input[key] !== 'number' || !Number.isFinite(input[key])
      || (key !== 'target' && input[key] <= 0)
      || (['attempt_limit', 'command_limit'].includes(key) && !Number.isInteger(input[key]))))
  || (mode === 'resume' && ['scope', 'until_interrupted', ...limits, ...capacities].some(key => Object.hasOwn(input, key)))
  || (input.run_profile && !/^[A-Za-z0-9][A-Za-z0-9_-]*$/.test(input.run_profile))
  || (input.role_profiles && (!exactObject(input.role_profiles, Object.keys(input.role_profiles))
    || Object.entries(input.role_profiles).some(([role, profile]) => !roles.includes(role)
      || !/^[A-Za-z0-9][A-Za-z0-9_-]*$/.test(profile))))
  || (profiles !== null && !validProfiles(profiles))
  || !retainedPlans || Array.isArray(retainedPlans) || typeof retainedPlans !== 'object'
  || !retainedCandidates || Array.isArray(retainedCandidates) || typeof retainedCandidates !== 'object'
  || Object.entries(retainedCandidates).some(([task, handoff]) => !task.startsWith(`${area}-`)
    || !exactObject(handoff, ['findings', 'escalated', 'needs_repair']) || !Array.isArray(handoff.findings)
    || !handoff.findings.every(validText) || typeof handoff.escalated !== 'boolean' || typeof handoff.needs_repair !== 'boolean')
  || Object.entries(retainedPlans).some(([task, plan]) => !task.startsWith(`${area}-`)
    || validateSemanticPlannerResult(plan)?.verdict !== 'plan')) {
  recovery.push('Supply structured area, authorization start/resume, fresh invocation_id, scope for start or run_id for resume, and valid limits/profiles.')
  return report('blocker', 'input')
}

const control = async (label, prompt, schema = null) => {
  const raw = await callAgent(`${repositoryGuidance}\n\n${root ? `Use absolute repository cwd ${JSON.stringify(root)} for every tool. ` : ''}Read installed research reference ${JSON.stringify(researchReferencePath)} and coding guidance ${JSON.stringify(codingReferencePath)}. Task contract: ${JSON.stringify(taskWorkflowContractPath)}. ${prompt}`,
    { label: `zdev research ${area}: ${label}`, model: 'claude-opus-5-5', effort: 'low', ...(schema ? { schema } : {}) })
  const value = json(raw)
  if (!value || value.error) throw new Error(value?.error ?? `${label} returned invalid JSON; inspect effects before retrying`)
  return value
}
const validRunContext = value => value?.schema_version === 1 && value.area === area
  && validText(value.run?.id) && validText(value.run.scope)
  && typeof value.run.comparison_valid === 'boolean' && typeof value.run.ended === 'boolean'
  && value.run.reservations && value.run.admissions
  && !Array.isArray(value.run.reservations) && !Array.isArray(value.run.admissions)
  && typeof value.run.reservations === 'object' && typeof value.run.admissions === 'object'
  && [null, 'target', 'budget', 'interruption', 'blocker', 'ended'].includes(value.stop_reason)
  && Array.isArray(value.checkpoints) && Array.isArray(value.tasks?.tasks)
  && value.accepted_baseline && value.consumed && value.remaining
const command = async (argv, label) => control(label,
  `Act only as command mediation. Execute the following argument array exactly once, preserving argument boundaries, then return its complete JSON stdout unchanged. On failure return only {"error":"concrete command failure and preserved effects"}. Do not launch workers, retry commands or invent successful state. Research command: ${JSON.stringify(['zdev', ...argv, '--format', 'json'])}`)
const runCommand = (action, flags = [], label = action) => command(
  ['research', 'run', area, action, '--run', runId, ...flags], label)
// With no ready task, open tasks that wait on other areas mean no work exists
// here until those blockers are done; generating more work would bypass them.
const waitsOnlyOnOtherAreas = value => Array.isArray(value.cross_area_waits)
  && value.cross_area_waits.length > 0 && !value.tasks.tasks.some(task => task.state === 'ready')
const stopForCrossAreaWaits = async () => {
  const waits = context.cross_area_waits
  if (!context.stop_reason) { await runCommand('stop'); await refresh() }
  return report('stopped', 'cross-area-wait', waits)
}
const refresh = async () => {
  const value = await command(['research', 'run', area, 'context'], 'fresh run context')
  if (!validRunContext(value) || (runId && value.run.id !== runId)) throw new Error('Fresh run context did not preserve the authorized identity')
  context = value
  return value
}
const reserve = async (task, kind) => {
  const request = nextRequest(kind)
  const value = await runCommand('reserve', ['--request', request, '--task', task, '--kind', kind, ...(execution === 'rolling' ? ['--worker-slot'] : [])], `reserve ${kind} ${task}`)
  const reservation = value.run?.reservations?.[request]
  // The last attempt authorizes this dispatch even though the returned context now says budget.
  if (!validRunContext(value) || value.run.id !== runId || !reservation
    || reservation.task !== task || reservation.kind !== kind || reservation.settled || reservation.bound
    || (execution === 'rolling' && (value.dispatch_authorized !== true || !reservation.worker || !reservation.requested_worker_slot))) {
    throw new Error('Reservation did not authorize the requested fresh dispatch')
  }
  context = value
  return request
}
const dispatch = async (task, kind, role, prompt, extra = {}) => {
  const request = await reserve(task, kind)
  const options = nativeWorkerOptions({ profiles }, role,
    { agentType: `zdev:zdev-${role}`, label: `zdev research ${task}: ${role}`, ...extra })
  const raw = await callAgent(prompt, options)
  if (role === 'planner') {
    const plan = parsePlannerResult(raw)
    // Preserve completed planning even when settlement's outcome needs recovery.
    if (plan?.verdict === 'plan' && reconstructPlannerResult(plan, area, task)) retainedPlans[task] = plan
  }
  if (execution === 'rolling') {
    const valid = role === 'planner' ? parsePlannerResult(raw)
      : role === 'verifier' ? parseVerifierResult(raw) : parseWorkerResult(raw, 'implementer', area, task)
    if (!valid) throw new Error(`${role} returned invalid or interrupted evidence; preserve its worker claim`)
    await checkedControl('confirm standalone role stopped', `Confirm actual role ${role} for ${task}, request ${request}, has stopped, including owned subprocesses. A returned Promise alone is insufficient. Preserve an ambiguous claim. Result: ${JSON.stringify(valid)}`)
  }
  await runCommand('settle', ['--request', request], `settle ${request}`)
  return raw
}
const workerPrompt = (selected, action) => `${repositoryGuidance}\n\nUse absolute repository cwd ${JSON.stringify(root)} for every file and command tool. Read task contract ${JSON.stringify(taskWorkflowContractPath)}, coding guidance ${JSON.stringify(codingReferencePath)}, and research reference ${JSON.stringify(researchReferencePath)}. Read .zdev/${area}/brief.md first, complete task ${selected.task_id} and applicable repository guidance. Load zdev work-context ${area} --show ${selected.context.snapshot} --format json. ${action} Never edit .zdev, stage, finalize, complete a task or commit. Return only your strict role result.`
const select = async (unfinishedTask = null, excluded = []) => {
  const selected = await control('select ready work',
    `Read fresh work-context and tasks list/show. Exclude these already assigned or unfinished tasks: ${JSON.stringify(excluded)}. ${unfinishedTask ? `Select only unfinished checkpoint task ${unfinishedTask} for recording/restoration, not new dispatch.` : 'Select from the entire ready frontier using ordinary eligibility and dependencies, preferring afk true. Under shared blocker choose only supporting work relevant to the recorded blocker; otherwise return task_id null when no eligible work remains.'} Read all prior rejected, inconclusive and failed-to-run lessons. For a selected task store explicit work-context --task <task> --store. Read full task/policy and declare exact clean candidate files before any hypothesis edit, including named new files; exclude managed records, directories, symlinks and unrelated changes. Return exactly {"task_id":string|null,"kind":"supporting"|"hypothesis"|null,"complexity":"routine"|"standard"|"advanced"|null,"context":compact-work-context|null,"paths":string[],"reason":string}. Keep files and Git unchanged. Fresh run data: ${JSON.stringify(context)}`)
  if (!exactObject(selected, ['task_id', 'kind', 'complexity', 'context', 'paths', 'reason'])
    || !validText(selected.reason) || !Array.isArray(selected.paths)) throw new Error('Invalid ready-work selection')
  if (selected.task_id === null) {
    if (selected.kind !== null || selected.complexity !== null || selected.context !== null || selected.paths.length) throw new Error('Ambiguous empty selection')
    return selected
  }
  const stored = parseStoredContext(selected.context, area)
  if (!stored || stored.taskId !== selected.task_id || stored.lifecycle !== 'open' || stored.queue !== 'ready'
    || selected.complexity !== stored.complexity || !['supporting', 'hypothesis'].includes(selected.kind)
    || stored.hypothesis !== (selected.kind === 'hypothesis')
    || (unfinishedTask && selected.task_id !== unfinishedTask)
    || excluded.includes(selected.task_id)
    || (!unfinishedTask && context.stop_reason === 'blocker' && selected.kind !== 'supporting')
    || (selected.kind === 'hypothesis' && (!selected.paths.length
      || new Set(selected.paths).size !== selected.paths.length
      || selected.paths.some(path => !normalizedRepositoryPath(path) || path.startsWith('/')
        || path === '.zdev' || path.startsWith('.zdev/'))))) throw new Error('Selection did not admit safe explicit task and declared files')
  return selected
}
const checkedControl = async (label, prompt) => {
  const result = await control(label, `${prompt} Return exactly {"safe":boolean,"reason":string}.`)
  if (!exactObject(result, ['safe', 'reason']) || result.safe !== true || !validText(result.reason)) {
    throw new Error(result.reason ?? `${label} could not establish safe continuation`)
  }
}
const recoverEffects = async (allowDispatch = true) => {
  const recovered = await control('recover pending effects',
    `Recover live durable effects before selection. For rolling execution confirm actual native workers and evaluator processes stopped before settling claims or Return --worker-stopped. A null, thrown, invalid or missing role response does not establish stop. Preserve unresolved capacity and sources; never redispatch a checkpoint through recovery. Inspect admissions, attributable uncommitted records, outstanding reservations, rebaseline requests and experiments. Retry only identical admission content under its original request; finish exact publication/staging/cached-diff/commit checkpoints through zdev commit. Inspect/recover experiments and interrupted rebaseline using the original request; never relaunch unknown measurements. Publish/commit an actual rebaseline's exact area metadata before new work. Reconcile reservations from actual worker effects without refunds or duplicate dispatch. Resume an already verified ordinary completion with its retained verification evidence and existing completion coordinator. When that evidence is unavailable but original baseline and ownership are established, safely reopen an uncommitted done ordinary task with zdev task reopen, store explicit ready work-context, and return reverify {"task_id":string,"context":compact-work-context,"baseline_snapshot":original-snapshot,"implementation":nonempty-summary}; otherwise preserve ambiguous state and report its concrete recovery action. Return exactly {"safe":boolean,"reason":string,"reverify":object|null}. Do not launch workers in this helper. Pending candidate code must be restored through authorized lifecycle before accepted supporting edits. For stopped isolated reconciliation conflicts, preserve the source and establish exact paths/base for a newly counted repair; do not launch or perform that repair in recovery. Preserve unresolved measurement/process ownership. Context: ${JSON.stringify(context)}`)
  if (!exactObject(recovered, ['safe', 'reason', 'reverify']) || recovered.safe !== true || !validText(recovered.reason)) throw new Error(recovered.reason ?? 'Unresolved recovery effects')
  pendingRecoveryVerification = recovered.reverify ?? pendingRecoveryVerification
  if (pendingRecoveryVerification !== null && allowDispatch) {
    const handoff = pendingRecoveryVerification
    const stored = parseStoredContext(handoff?.context, area)
    if (!exactObject(handoff, ['task_id', 'context', 'baseline_snapshot', 'implementation'])
      || !stored || stored.taskId !== handoff.task_id || stored.hypothesis !== false
      || !/^W[0-9a-f]{16}$/.test(handoff.baseline_snapshot ?? '') || !validText(handoff.implementation)) throw new Error('Invalid supporting completion recovery handoff')
    const selected = { task_id: handoff.task_id, context: handoff.context, complexity: stored.complexity }
    const raw = await dispatch(handoff.task_id, 'supporting', 'verifier', workerPrompt(selected,
      `Independently verify this interrupted ordinary completion against original baseline zdev work-context ${area} --show ${handoff.baseline_snapshot} --format json and supplied snapshot ${stored.baselineSnapshot}. Check the whole task and coding Review/Code standards. Implementation summary: ${handoff.implementation}. Return exactly verdict, summary, findings, escalation.`))
    const semantic = parseVerifierResult(raw)
    const comparison = parseComparison(await command(['work-context', area, '--compare', stored.baselineSnapshot], 'compare recovered completion'), area, stored.baselineSnapshot)
    if (!semantic || !comparison?.equal || semantic.verdict === 'blocker') throw new Error('Recovered completion lacks valid independent verification and stable snapshot')
    if (semantic.verdict === 'rework') {
      if (execution === 'rolling') await checkRollingFindings(selected, semantic, 'supporting', stored.baselineSnapshot)
      else await checkedControl('check recovered completion findings', `Independently check substantive task-owned findings against snapshot ${stored.baselineSnapshot}: ${JSON.stringify(semantic.findings)}`)
      if (stored.complexity === 'advanced' && !retainedPlans[handoff.task_id]) throw new Error('Advanced completion rework requires the retained original semantic plan')
      await support(selected)
    } else {
      const result = await callAgent(`${repositoryGuidance}\n\nUse absolute cwd ${JSON.stringify(root)}. Act as the existing ordinary completion coordinator for ${handoff.task_id}. Compare zdev work-context ${area} --compare ${stored.baselineSnapshot} --format json and require exact equal true before mutation. Read ${JSON.stringify(taskWorkflowContractPath)}. Run task done, stage exact attributed source paths, exact task/TASKS.md records and every dependent TASKS.md path that task done reports in dependent_indexes or as Updated dependent indexes, inspect full cached diff and zdev commit. Preserve unfinished checkpoint on failure. Return the standard PASS/BLOCKER zdev-implement ${area} ${handoff.task_id} result and required fields. Independent verification: ${JSON.stringify(semantic)}. Original baseline: ${handoff.baseline_snapshot}.`,
        { label: `zdev ${handoff.task_id}: complete and commit` })
      recordSupportingCompletion(result, handoff.task_id)
    }
    pendingRecoveryVerification = null
  }
  await refresh()
}
const publish = async admitted => checkedControl('publish admitted tasks',
  `Inspect allocated tasks in this admission and its exact delta. Stage only admitted task files and .zdev/${area}/TASKS.md, inspect full cached diff and Git status, then use zdev commit. Keep source, policy and unrelated records unchanged. Retain any interrupted publication checkpoint. Admission: ${JSON.stringify(admitted)}`)
const generate = async () => {
  const proposal = await control('propose scoped work',
    `Propose a small nonempty mixed TaskBundle as data. Read brief, accepted baseline, task evidence and prior rejected/inconclusive/failed-to-run lessons. Respect original scope, permitted changes, graph links and requires_code (only verified retained acceptance establishes code). Under a shared blocker propose only relevant ordinary repairs. Return exactly {"bundle":TaskBundle,"reason":string}. Do not admit, edit, stage or commit. Fresh authorized context: ${JSON.stringify(context)}`)
  if (!exactObject(proposal, ['bundle', 'reason']) || !validText(proposal.reason)
    || proposal.bundle?.schema_version !== 1 || proposal.bundle.area !== area
    || !Array.isArray(proposal.bundle.tasks) || !proposal.bundle.tasks.length) throw new Error('Proposal was empty, malformed or outside the selected area')
  const request = nextRequest('generation')
  const admitted = await control('admit scoped proposal',
    `Check the exact proposal against brief, scope, graph and requires_code. Write only a temporary bundle file outside tracked paths, then run zdev research run ${area} admit --run ${runId} --request ${request} --from <that-file> --format json once. Return complete JSON stdout or {"error":"concrete failure"}. No worker or code edits. Proposal: ${JSON.stringify(proposal.bundle)}`)
  if (!validRunContext(admitted) || admitted.run.id !== runId || !admitted.run.admissions?.[request]?.tasks?.length) throw new Error('Admission did not allocate tasks')
  await publish(admitted)
}
const recordSupportingCompletion = (result, task) => {
  const pass = typeof result === 'string' && fromExactLine(result, `PASS zdev-implement ${area} ${task}`)
  const commit = pass && commitIdFrom(field(pass, 'Commit ID'))
  if (!pass || field(pass, 'Area') !== area || field(pass, 'Task') !== task
    || !['Summary', 'Changed files', 'Validation', 'Verifier evidence'].every(name => field(pass, name))
    || !commit) throw new Error(typeof result === 'string' ? result : 'Supporting workflow returned no valid completion')
  completed.push({ task_id: task,
    decision: field(pass, 'Derived proposal') === 'implementation_split' ? 'implementation_split' : 'supporting', commit })
}
const support = async selected => {
  const retained = retainedPlans[selected.task_id]
  const result = await runOneTask({ area, task_id: selected.task_id, frozen_profiles: profiles,
    ...(retained ? { retained_plan: 'applicable', retained_plan_result: retained } : {}) }, async (prompt, options) => {
    if (options?.agentType) {
      const role = options.agentType.replace('zdev:zdev-', '')
      if (!roles.includes(role)) throw new Error('Unknown supporting role')
      const raw = await dispatch(selected.task_id, 'supporting', role,
        `${workerPrompt(selected, 'Follow the ordinary task contract and this exact dispatch.')}\n\n${prompt}`, options)
      if (role === 'planner') {
        const plan = parsePlannerResult(raw)
        if (plan?.verdict === 'plan') retainedPlans[selected.task_id] = plan
      }
      return raw
    }
    return callAgent(`${root ? `Use absolute repository cwd ${JSON.stringify(root)} for every tool.\n` : ''}${prompt}`, options)
  })
  recordSupportingCompletion(result, selected.task_id)
}
const experiment = (task, action, flags = []) => command(['research', 'experiment', area, task, action, ...flags], `${task}: ${action}`)
const inspectExperiment = (value, task) => {
  if (value?.schema_version !== 1 || value.area !== area || value.task !== task
    || !value.checkpoint || !Array.isArray(value.checkpoint.attempts)
    || typeof value.comparison_valid !== 'boolean' || typeof value.measurement_current !== 'boolean') throw new Error('Invalid experiment checkpoint')
  return value
}
const hypothesis = async selected => {
  const task = selected.task_id
  let measured = context.checkpoints.find(checkpoint => checkpoint.task === task && !checkpoint.index_updated)
  let plan = retainedPlans[task] ?? null
  let evaluationReservation = null
  if (measured) {
    measured = inspectExperiment(await experiment(task, 'inspect'), task)
    if (!measured.checkpoint.attempts.length) {
      if (selected.complexity === 'advanced' && !plan) throw new Error('Interrupted advanced preparation requires its retained semantic plan; preserve checkpoint without replanning')
      evaluationReservation = await reserve(task, 'repair')
    }
  } else {
    if (selected.complexity === 'advanced' && !plan) {
      const raw = await dispatch(task, 'preparation', 'planner', workerPrompt(selected,
        'Plan advanced hypothesis without edits. Return the strict four-field semantic planner object.'), { schema: plannerSchema })
      plan = parsePlannerResult(raw)
      if (!plan || !reconstructPlannerResult(plan, area, task) || plan.verdict !== 'plan') throw new Error('Advanced hypothesis planning did not return a valid plan')
    }
    const preparation = await reserve(task, 'preparation')
    measured = inspectExperiment(await experiment(task, 'begin', [
      ...selected.paths.flatMap(path => ['--path', path]), '--reservation', preparation,
    ]), task)
  }
  if (!measured.checkpoint.attempts.length) {
    const admitted = await command(['work-context', area, '--task', task, '--store'], `${task}: refresh hypothesis admission`)
    const current = parseStoredContext(admitted, area)
    if (!current || current.taskId !== task || current.hypothesis !== true
      || current.head !== measured.checkpoint.head || current.complexity !== selected.complexity
      || JSON.stringify(measured.checkpoint.paths) !== JSON.stringify(selected.paths)) throw new Error('Begin did not preserve explicit hypothesis admission and ownership')
    const role = selected.complexity === 'routine' ? 'routine-implementer'
      : selected.complexity === 'advanced' ? 'advanced-implementer' : 'implementer'
    // Begin has already charged this preparation; a second reserve would double-charge it.
    let raw = await callAgent(workerPrompt({ ...selected, context: admitted },
      `Implement only these declared files: ${JSON.stringify(selected.paths)}. Report another necessary file without editing it. ${plan ? `Follow this validated semantic plan unchanged: ${JSON.stringify(plan)}.` : ''}`),
      nativeWorkerOptions({ profiles }, role, { agentType: `zdev:zdev-${role}`, label: `zdev research ${task}: implement hypothesis` }))
    let implementation = parseWorkerResult(raw, 'implementer', area, task)
    let priorBlocker = null
    while (implementation?.verdict === 'blocker') {
      const disposition = parseBlockerDisposition(await control('classify hypothesis implementation blocker',
        `Inspect actual candidate delta against declared ownership. Return exactly {"action":"continue"|"stop","reason":string}. Continue only directly actionable work within declared files and original task/policy; another necessary file requires lifecycle resolution before a revised hypothesis. Stop for unsafe state, external/user-owned decisions or the same unresolved obstacle without new actionable evidence. No edits, workers or records. Prior blocker: ${JSON.stringify(priorBlocker)}. Current blocker: ${JSON.stringify(implementation)}. Checkpoint: ${JSON.stringify(measured)}`))
      if (!disposition || disposition.action !== 'continue') throw new Error(disposition?.reason ?? 'Invalid implementation blocker disposition')
      priorBlocker = implementation
      if (evaluationReservation) await runCommand('settle', ['--request', evaluationReservation])
      evaluationReservation = await reserve(task, 'repair')
      const refreshed = await command(['work-context', area, '--task', task, '--store'], `${task}: refresh replacement admission`)
      const replacement = parseStoredContext(refreshed, area)
      if (!replacement || replacement.taskId !== task || replacement.hypothesis !== true
        || replacement.head !== measured.checkpoint.head || replacement.complexity !== selected.complexity) throw new Error('Replacement lost explicit hypothesis admission')
      raw = await callAgent(workerPrompt({ ...selected, context: refreshed },
        `Finish directly actionable work only in declared files ${JSON.stringify(selected.paths)}. Prior blocker: ${JSON.stringify(implementation)}. ${plan ? `Follow the unchanged semantic plan: ${JSON.stringify(plan)}.` : ''}`),
        nativeWorkerOptions({ profiles }, role, { agentType: `zdev:zdev-${role}`, label: `zdev research ${task}: replacement` }))
      implementation = parseWorkerResult(raw, 'implementer', area, task)
    }
    if (!implementation || implementation.verdict !== 'ready') throw new Error('Hypothesis implementer did not return a valid ready result; preserve effects before replacement')
    await checkedControl('inspect candidate delta',
      `Inspect the whole staged/unstaged/untracked delta against checkpoint baseline. Require edits only to declared exact candidate files with unchanged records, HEAD, policy and unrelated work. Implementation: ${JSON.stringify(implementation)}. Checkpoint: ${JSON.stringify(measured)}`)
    measured = inspectExperiment(await experiment(task, 'evaluate', evaluationReservation ? ['--reservation', evaluationReservation] : []), task)
  }
  let escalated = false
  while (true) {
    const checkpoint = measured.checkpoint
    if (checkpoint.attempts.some(attempt => attempt.measuring || !attempt.published)) throw new Error('Unresolved measurement must recover without relaunch')
    if (checkpoint.final_record || checkpoint.index_updated) {
      const recovered = inspectExperiment(await experiment(task, 'recover'), task)
      if (!recovered.checkpoint.index_updated) throw new Error('Finalization still requires recovery')
      completed.push({ task_id: task, decision: recovered.checkpoint.final_record.decision, commit: recovered.checkpoint.commit })
      return
    }
    const decision = await control('assess measured hypothesis',
      `Check actual requirements, samples, checks, assessment, comparison validity and retained candidate references. Return exactly {"action":"keep"|"discard"|"failed-to-run"|"inconclusive"|"repair"|"rerun"|"block","lesson":string,"reason":string}. Keep requires measured improvement and all configured checks; discard needs usable evidence. Exhausted attempts without usable observations permit failed-to-run; with observations but no answer permit inconclusive. Never fabricate terminal evidence to clear interruption. Shared environment failure returns block. A changed comparison condition cannot keep; recover safe restoration before actual rebaseline. No worker/measurement or record edits. Evidence: ${JSON.stringify(measured)}`)
    if (!exactObject(decision, ['action', 'lesson', 'reason'])
      || !['keep', 'discard', 'failed-to-run', 'inconclusive', 'repair', 'rerun', 'block'].includes(decision.action)
      || !validText(decision.lesson) || !validText(decision.reason)) throw new Error('Invalid measured disposition')
    if (decision.action === 'block') {
      await runCommand('block', ['--reason', decision.reason])
      throw new Error(`Shared blocker: ${decision.reason}; recover candidate before supporting repairs`)
    }
    let verification = null
    let repairFindings = null
    if (decision.action === 'keep') {
      if (!measured.comparison_valid || measured.assessment?.improved !== true
        || !/^W[0-9a-f]{16}$/.test(checkpoint.verification_snapshot ?? '')) throw new Error('Keep lacks valid improving measurements and verification snapshot')
      const snapshot = checkpoint.verification_snapshot
      const raw = await dispatch(task, 'confirmation', 'verifier', workerPrompt(selected,
        `Independently review all acceptance requirements and declared code, read coding Review and Code standards. Show supplied zdev work-context ${area} --show ${snapshot} --format json. Starting baseline, owned paths, measured samples and full policy: ${JSON.stringify(measured)}. Return exactly verdict, summary, findings, escalation as the ordinary strict semantic verifier object.`))
      const semantic = parseVerifierResult(raw)
      const comparison = parseComparison(await command(['work-context', area, '--compare', snapshot], `${task}: compare acceptance snapshot`), area, snapshot)
      if (!semantic || !comparison?.equal) throw new Error('Independent acceptance result or stable snapshot comparison failed')
      if (semantic.verdict === 'blocker') throw new Error(semantic.summary)
      if (semantic.verdict === 'rework') {
        await checkedControl('check acceptance findings', `Independently check every substantive verifier finding against snapshot ${snapshot}; require concrete task-owned repairs. Findings: ${JSON.stringify(semantic.findings)}`)
        if (semantic.escalation === 'advanced-implementer') {
          if (selected.complexity !== 'standard' || escalated) throw new Error('Inapplicable or repeated acceptance escalation')
          escalated = true
        }
        repairFindings = semantic.findings
      } else verification = { schema_version: 1, kind: 'verifier', area, task_id: task,
        ...semantic, evidence: [`work_context_snapshot: ${snapshot}`] }
    }
    if (repairFindings || ['repair', 'rerun'].includes(decision.action)) {
      if (selected.complexity === 'advanced' && !plan && (repairFindings || decision.action === 'repair')) throw new Error('Advanced repair requires its retained semantic plan; preserve checkpoint without replanning')
      const kind = repairFindings || decision.action === 'repair' ? 'repair' : 'rerun'
      const request = await reserve(task, kind)
      if (kind === 'repair') {
        const refreshed = await command(['work-context', area, '--task', task, '--store'], `${task}: refresh repair admission`)
        const admission = parseStoredContext(refreshed, area)
        if (!admission || admission.taskId !== task || admission.hypothesis !== true
          || admission.head !== checkpoint.head || admission.complexity !== selected.complexity) throw new Error('Repair lost explicit hypothesis admission and baseline HEAD')
        const role = escalated || selected.complexity === 'advanced' ? 'advanced-implementer'
          : selected.complexity === 'routine' ? 'routine-implementer' : 'implementer'
        const raw = await callAgent(workerPrompt({ ...selected, context: refreshed },
          `Repair only declared files ${JSON.stringify(checkpoint.paths)}. Load original baseline zdev work-context ${area} --show ${checkpoint.starting_snapshot} --format json. Findings: ${JSON.stringify(repairFindings ?? [decision.reason])}. ${plan ? `Retain this unchanged semantic plan: ${JSON.stringify(plan)}.` : ''}`),
          nativeWorkerOptions({ profiles }, role, { agentType: `zdev:zdev-${role}`, label: `zdev research ${task}: repair` }))
        if (parseWorkerResult(raw, 'implementer', area, task)?.verdict !== 'ready') throw new Error('Repair did not return a valid ready result')
        await checkedControl('inspect repaired delta', `Require whole candidate delta within declared files and unchanged records/HEAD/policy. Checkpoint: ${JSON.stringify(checkpoint)}`)
      }
      measured = inspectExperiment(await experiment(task, 'evaluate', ['--reservation', request]), task)
      continue
    }
    const finalized = await control('finalize measured hypothesis',
      `Use managed experiment finalize for ${task} exactly once with --decision ${decision.action} --lesson ${JSON.stringify(decision.lesson)}${verification ? ` and --verification <temporary-envelope-file> containing ${JSON.stringify(verification)}` : ''}. Keep temporary files outside tracked paths. Return complete experiment JSON stdout or {"error":"concrete failure"}. Finalize alone owns candidate retention/restoration, records, completion and exact commit. Never repeat ordinary task done/staging/commit.`)
    inspectExperiment(finalized, task)
    if (!finalized.checkpoint.index_updated || !/^[0-9a-f]{40}$/.test(finalized.checkpoint.commit ?? '')) throw new Error('Managed finalization did not finish its exact commit')
    completed.push({ task_id: task, decision: finalized.checkpoint.final_record.decision, commit: finalized.checkpoint.commit })
    return
  }
}

const checkRollingFindings = async (selected, semantic, kind = 'confirmation', snapshot = null) => {
  const marked = semantic.findings.filter(finding => finding.startsWith(validationWriteMarker))
  if (marked.length && !reportsValidationWrite(semantic)) throw new Error('Malformed validation-write findings refuse rework')
  const substantive = semantic.findings.filter(finding => !finding.startsWith(validationWritePrefix))
  if (!substantive.length) return
  const request = await reserve(selected.task_id, kind)
  const raw = await callAgent(workerPrompt(selected,
    `Independently check only these substantive findings, their checked location, consequence and applicability: ${JSON.stringify(substantive)}. ${snapshot ? `Use exact snapshot ${snapshot}.` : 'Inspect the current supporting verification state.'} Keep files and records unchanged. Return exactly {"safe":boolean,"reason":string}; safe true means every finding supports the requested in-scope correction.`),
    nativeWorkerOptions({ profiles }, 'verifier', { label: `zdev research ${selected.task_id}: check findings` }))
  const checked = json(raw)
  if (!exactObject(checked, ['safe', 'reason']) || checked.safe !== true || !validText(checked.reason)) throw new Error('Finding check unavailable or unsupported; preserve claim and candidate')
  await checkedControl('confirm standalone role stopped', `Confirm actual finding checker ${request} and its subprocesses stopped. Result: ${JSON.stringify(checked)}`)
  await runCommand('settle', ['--request', request])
  if (snapshot) {
    const compared = parseComparison(await command(['work-context', area, '--compare', snapshot], 'compare finding check'), area, snapshot)
    if (!compared?.equal) throw new Error('Candidate changed during independent finding check')
  }
}

// Only promises and arrival identities are transient. Run claims/checkpoints own recovery.
const runRollingResearch = async () => {
  const candidates = new Map()
  const active = new Map()
  const measuring = new Map()
  const arrivals = []
  let wake = null
  let generatedWithoutWork = false
  let supportWaiting = null
  let stopping = false
  const signal = event => { arrivals.push(event); if (wake) { wake(); wake = null } }
  const launch = (kind, candidate, operation) => {
    const calls = kind === 'source' ? active : measuring
    const promise = Promise.resolve().then(operation)
      .then(value => ({ kind, candidate, value }), error => ({ kind, candidate, error: String(error?.message ?? error) }))
      .then(event => { signal(event) })
    calls.set(candidate.selected.task_id, promise)
  }
  const sourcePrompt = (candidate, action) => {
    const checkpoint = candidate.measured.checkpoint
    const isolated = checkpoint.isolated
    return `${repositoryGuidance}\n\nRead task contract ${JSON.stringify(taskWorkflowContractPath)}, coding Design/Completion/Code standards ${JSON.stringify(codingReferencePath)}, and research reference ${JSON.stringify(researchReferencePath)}. Complete task ${candidate.selected.task_id}. Read authoritative brief ${JSON.stringify(isolated.brief)} FIRST, complete task record ${JSON.stringify(isolated.task_record)} and repository instructions ${JSON.stringify(isolated.repository_instructions)}. Read immutable admitted snapshot ${isolated.admitted_snapshot} through zdev work-context ${area} --show in authoritative cwd ${JSON.stringify(root)}. Use absolute assigned source cwd ${JSON.stringify(isolated.source_root)} for EVERY source edit and validation command. Never read or edit source .zdev, edit authoritative records, integrate, stage, complete, finalize or commit. Edit only exact files ${JSON.stringify(checkpoint.paths)}; report any additional file before editing. ${action} ${candidate.plan ? `Follow this semantic plan unchanged: ${JSON.stringify(candidate.plan)}.` : ''} Return only the strict implementer result.`
  }
  const launchSource = candidate => {
    retainedCandidates[candidate.selected.task_id] = { findings: candidate.findings ?? [], escalated: candidate.escalated === true, needs_repair: true }
    const role = candidate.escalated || candidate.selected.complexity === 'advanced' ? 'advanced-implementer'
      : candidate.selected.complexity === 'routine' ? 'routine-implementer' : 'implementer'
    launch('source', candidate, () => callAgent(sourcePrompt(candidate,
      `Implement the complete hypothesis. Repair findings: ${JSON.stringify(candidate.findings ?? [])}. Never integrate, stage or commit.`),
      nativeWorkerOptions({ profiles }, role, { agentType: `zdev:zdev-${role}`, label: `zdev research ${candidate.selected.task_id}: source implementation` })))
  }
  const collect = async event => {
    const candidate = event.candidate
    const task = candidate.selected.task_id
    const pendingCalls = event.kind === 'source' ? active : measuring
    pendingCalls.delete(task)
    if (event.error) throw new Error(`${task} ${event.kind}: ${event.error}; confirm actual worker/process stop before recovery`)
    if (event.kind === 'source') {
      const result = parseWorkerResult(event.value, 'implementer', area, task)
      if (!result) throw new Error(`${task}: invalid or interrupted source result; preserve worker claim and source`)
      await checkedControl('confirm source stopped', `Inspect actual native source role and owned subprocesses for task ${task}, request ${candidate.request}, source ${candidate.measured.checkpoint.isolated.source_root}. Confirm completion/stop and exact owned delta before Return. For a source reconciliation whose conflicts are fully resolved in owned files, inspect and stage only those resolved source paths to clear its unmerged index; never stage destination records. Promise completion alone is insufficient. Worker result: ${JSON.stringify(result)}`)
      candidate.measured = inspectExperiment(await experiment(task, 'return', ['--worker-stopped']), task)
      candidate.needsMeasurement = true
      if (result.verdict === 'ready') retainedCandidates[task] = { findings: [], escalated: candidate.escalated === true, needs_repair: false }
      if (result.verdict === 'blocker') {
        const disposition = parseBlockerDisposition(await control('classify hypothesis implementation blocker',
          `Inspect stopped source effects. Continue only directly actionable work inside exact paths, original scope and policy. Return exactly {"action":"continue"|"stop","reason":string}. Result: ${JSON.stringify(result)}. Checkpoint: ${JSON.stringify(candidate.measured)}`))
        if (disposition?.action !== 'continue') throw new Error(disposition?.reason ?? 'Source has a concrete unresolved blocker')
        candidate.findings = [result.summary, ...result.findings]
        candidate.needsRepair = true
        retainedCandidates[task] = { findings: candidate.findings, escalated: candidate.escalated === true, needs_repair: true }
      }
    } else {
      candidate.measured = inspectExperiment(event.value, task)
      if (candidate.measured.checkpoint.attempts.some(attempt => attempt.measuring || !attempt.published)) throw new Error(`${task}: unresolved measurement; preserve process ownership without relaunch`)
      candidate.needsMeasurement = false
    }
  }
  const nextArrival = async () => {
    if (!arrivals.length && (active.size || measuring.size)) await new Promise(resolve => { wake = resolve })
    if (arrivals.length) await collect(arrivals.shift())
  }
  const roleCapacity = async () => {
    await refresh()
    while (context.remaining.workers === 0 && active.size) { await nextArrival(); await refresh() }
    if (context.remaining.workers === 0) throw new Error('Native role capacity remains occupied by unresolved durable claims')
  }
  const prepareRepair = async candidate => {
    if (candidate.selected.complexity === 'advanced' && !candidate.plan) throw new Error('Advanced repair requires its retained original semantic plan')
    await roleCapacity()
    candidate.request = await reserve(candidate.selected.task_id, 'repair')
    candidate.measured = inspectExperiment(await experiment(candidate.selected.task_id, 'prepare', ['--reservation', candidate.request]), candidate.selected.task_id)
    if (candidate.measured.dispatch_authorized !== true || candidate.measured.checkpoint.preparation?.request !== candidate.request) throw new Error('Prepare replay or changed request refused source dispatch')
    candidate.needsRepair = false
    launchSource(candidate)
  }
  const synchronize = async candidate => {
    const task = candidate.selected.task_id
    candidate.measured = inspectExperiment(await experiment(task, 'inspect'), task)
    if (candidate.measured.checkpoint.isolated.integrated) return
    const checked = await control('check candidate baseline',
      `Read fresh destination HEAD, accepted code and comparison policy, and this stopped returned source. Return exactly {"refresh":boolean,"remeasure":boolean,"reason":string}. Refresh whenever destination HEAD or comparison conditions differ. Remeasure when accepted CODE or conditions changed; record-only HEAD drift may refresh retained measurements. Do not mutate. Checkpoint: ${JSON.stringify(candidate.measured)}. Fresh run: ${JSON.stringify(context)}`)
    if (!exactObject(checked, ['refresh', 'remeasure', 'reason']) || typeof checked.refresh !== 'boolean' || typeof checked.remeasure !== 'boolean'
      || (checked.remeasure && !checked.refresh) || !validText(checked.reason)) throw new Error('Invalid current-baseline check')
    if (checked.refresh) {
      try {
        candidate.measured = inspectExperiment(await experiment(task, 'reconcile'), task)
      } catch (error) {
        const preserved = inspectExperiment(await experiment(task, 'inspect'), task)
        if (!preserved.checkpoint.isolated.refreshing || !preserved.checkpoint.isolated.worker_stopped
          || preserved.checkpoint.attempts.some(attempt => attempt.measuring)) throw error
        candidate.measured = preserved
        candidate.findings = [`Resolve the retained source reconciliation conflicts against current accepted code: ${String(error?.message ?? error)}`]
        candidate.needsRepair = true
        retainedCandidates[task] = { findings: candidate.findings, escalated: candidate.escalated === true, needs_repair: true }
      }
      candidate.needsMeasurement ||= checked.remeasure
    }
  }
  const finalizeCandidate = async (candidate, decision, verification = null) => {
    const task = candidate.selected.task_id
    const finalized = await control('finalize measured hypothesis',
      `Use managed experiment finalize for ${task} exactly once with --decision ${decision.action} --lesson ${JSON.stringify(decision.lesson)}${verification ? ` and --verification <temporary-envelope-file> containing ${JSON.stringify(verification)}` : ''}. Keep temporary files outside tracked paths. Return complete experiment JSON stdout or {"error":"concrete failure"}. Finalize alone owns candidate retention/restoration, records, completion and exact commit. Never repeat ordinary task done/staging/commit.`)
    inspectExperiment(finalized, task)
    if (!finalized.checkpoint.index_updated || !/^[0-9a-f]{40}$/.test(finalized.checkpoint.commit ?? '')) throw new Error('Managed finalization requires recovery of its exact commit')
    completed.push({ task_id: task, decision: finalized.checkpoint.final_record.decision, commit: finalized.checkpoint.commit })
    candidates.delete(task)
    delete retainedCandidates[task]
    generatedWithoutWork = false
    if (input.cleanup === true && finalized.cleanup_eligible === true) {
      await checkedControl('cleanup completed source', `Remove only this exclusively run-owned local linked worktree after actual worker/process stop, durable completion and retained history. Never remove pre-existing or ambiguous worktrees. Context: ${JSON.stringify(finalized)}`)
    }
  }
  const accept = async candidate => {
    const task = candidate.selected.task_id
    const decision = await control('assess measured hypothesis',
      `Check requirements, samples, checks, current assessment, comparison validity and retained history. Return exactly {"action":"keep"|"discard"|"failed-to-run"|"inconclusive"|"repair"|"rerun"|"block","lesson":string,"reason":string}. Only measured improvement with all checks permits keep. Exhausted unusable attempts permit failed-to-run; usable observations without an answer permit inconclusive. Interruption stays unfinished. Shared failure returns block. Evidence: ${JSON.stringify(candidate.measured)}`)
    if (!exactObject(decision, ['action', 'lesson', 'reason']) || !['keep', 'discard', 'failed-to-run', 'inconclusive', 'repair', 'rerun', 'block'].includes(decision.action)
      || !validText(decision.lesson) || !validText(decision.reason)) throw new Error('Invalid measured disposition')
    if (decision.action === 'block') {
      await runCommand('block', ['--reason', decision.reason])
      return
    }
    if (decision.action === 'repair') {
      candidate.findings = [decision.reason]
      retainedCandidates[task] = { findings: candidate.findings, escalated: candidate.escalated === true, needs_repair: true }
      await prepareRepair(candidate); return
    }
    if (decision.action === 'rerun') { candidate.measurementRequest = await reserveMeasurement(task, 'rerun'); candidate.needsMeasurement = true; return }
    if (decision.action !== 'keep') { await finalizeCandidate(candidate, decision); return }
    if (!candidate.measured.comparison_valid || candidate.measured.assessment?.improved !== true) throw new Error('Keep lacks current valid improving measurement')
    candidate.measured = inspectExperiment(await experiment(task, 'integrate'), task)
    const snapshot = candidate.measured.checkpoint.verification_snapshot
    if (!/^W[0-9a-f]{16}$/.test(snapshot ?? '')) throw new Error('Integration did not supply fresh destination verification')
    await roleCapacity() // This acceptance owner has priority over another source preparation.
    const raw = await dispatch(task, 'confirmation', 'verifier', workerPrompt(candidate.selected,
      `Independently verify the entire integrated task in authoritative destination ${root}. Read coding Review and Code standards. Show exact supplied snapshot ${snapshot}, original baseline ${candidate.measured.checkpoint.starting_snapshot}, declared paths, full samples and policy: ${JSON.stringify(candidate.measured)}. Return exactly verdict, summary, findings, escalation.`))
    let semantic = parseVerifierResult(raw)
    const compared = parseComparison(await command(['work-context', area, '--compare', snapshot], `${task}: compare acceptance snapshot`), area, snapshot)
    if (!semantic || !compared?.equal) throw new Error('Acceptance verification or immutable destination snapshot failed')
    if (onlyValidationWrites(semantic)) semantic = { ...semantic, verdict: 'pass', findings: [], escalation: 'none' }
    if (semantic.verdict === 'blocker') throw new Error(semantic.summary)
    if (semantic.verdict === 'rework') {
      await checkRollingFindings(candidate.selected, semantic, 'confirmation', snapshot)
      if (semantic.escalation === 'advanced-implementer') {
        if (candidate.selected.complexity !== 'standard' || candidate.escalated) throw new Error('Inapplicable or repeated acceptance escalation')
        candidate.escalated = true
      }
      candidate.findings = semantic.findings
      retainedCandidates[task] = { findings: semantic.findings, escalated: candidate.escalated === true, needs_repair: true }
      await prepareRepair(candidate) // Prepare withdraws integrated owned paths before source edits.
      return
    }
    await finalizeCandidate(candidate, decision, { schema_version: 1, kind: 'verifier', area, task_id: task,
      ...semantic, evidence: [`work_context_snapshot: ${snapshot}`] })
  }
  const reserveMeasurement = async (task, kind) => {
    const request = nextRequest(kind)
    const value = await runCommand('reserve', ['--request', request, '--task', task, '--kind', kind])
    const reservation = value.run?.reservations?.[request]
    if (!validRunContext(value) || value.run.id !== runId || !reservation || reservation.task !== task
      || reservation.kind !== kind || reservation.settled || reservation.bound || reservation.requested_worker_slot) throw new Error('Measurement reservation did not authorize a fresh attempt')
    context = value
    return request
  }
  try {
    for (const checkpoint of context.checkpoints.filter(item => !item.index_updated)) {
      if (!checkpoint.isolated) throw new Error('Rolling resume found local candidate; resume its serial route')
      const measured = inspectExperiment(await experiment(checkpoint.task, checkpoint.isolated.refreshing ? 'inspect' : 'recover'), checkpoint.task)
      if (measured.checkpoint.index_updated) continue
      if (!measured.checkpoint.isolated.worker_stopped || measured.checkpoint.attempts.some(attempt => attempt.measuring || !attempt.published)) throw new Error('Unresolved worker or measurement must remain owned before rolling resume')
      const selected = await select(checkpoint.task)
      if (selected.complexity === 'advanced' && !retainedPlans[checkpoint.task]) throw new Error('Advanced rolling resume requires the unchanged original semantic plan before dispatch')
      candidates.set(checkpoint.task, { selected, measured, plan: retainedPlans[checkpoint.task] ?? null,
        needsMeasurement: !measured.measurement_current,
        ...(retainedCandidates[checkpoint.task] ? { findings: retainedCandidates[checkpoint.task].findings,
          escalated: retainedCandidates[checkpoint.task].escalated, needsRepair: retainedCandidates[checkpoint.task].needs_repair || measured.checkpoint.isolated.refreshing }
          : !measured.checkpoint.attempts.length || measured.checkpoint.isolated.refreshing ? { needsRepair: true } : {}) })
    }
    while (true) {
      await refresh()
      if (nativeCalls >= 900) stopping = true
      while (arrivals.length) await collect(arrivals.shift())
      await refresh()
      if (context.stop_reason && context.stop_reason !== 'blocker') stopping = true
      if (stopping) {
        if (active.size || measuring.size) { await nextArrival(); continue }
        if (!context.stop_reason) { await runCommand('stop'); await refresh() }
        recovery.push(...[...candidates.keys()].map(task => `Resume unfinished ${task} from its retained source and checkpoint; do not replay dispatch.`))
        return report('stopped', nativeCalls >= 900 ? 'workflow-call-boundary' : context.stop_reason)
      }
      if (pendingRecoveryVerification) {
        if (active.size || measuring.size) { await nextArrival(); continue }
        if (context.run.acceptance) throw new Error('Recover candidate acceptance before ordinary completion verification')
        await recoverEffects()
        continue
      }
      // Shared support and rebaseline require a quiet destination and stopped measurements.
      if (context.stop_reason === 'blocker' || !context.run.comparison_valid || supportWaiting) {
        if (active.size || measuring.size) { await nextArrival(); continue }
        if (context.run.acceptance) throw new Error('Recover integrated candidate withdrawal before shared supporting work')
        const selected = supportWaiting ?? (context.stop_reason === 'blocker' ? await select(null, [...candidates.keys()]) : null)
        supportWaiting = null
        if (selected?.task_id === null) {
          if (generatedWithoutWork) throw new Error('No relevant supporting repair was admitted for shared blocker')
          await generate(); generatedWithoutWork = true; continue
        }
        if (selected) {
          await support(selected); generatedWithoutWork = false; await refresh()
          if (context.stop_reason === 'blocker') {
            await checkedControl('confirm shared repair', `Confirm committed supporting work resolved the recorded shared failure without changing frozen requirements. Context: ${JSON.stringify(context)}`)
            await runCommand('resolve')
          }
        }
        await checkedControl('prepare accepted rebaseline', `Require clean accepted HEAD, stopped measurements and unchanged metric/direction/correctness/acceptance requirements. Preserve stopped isolated sources. Run actual commands rather than supplied samples. Context: ${JSON.stringify(context)}`)
        await runCommand('rebaseline', ['--request', nextRequest('rebaseline')])
        await recoverEffects(false) // Publish the actual area baseline before comparison/refill.
        continue
      }
      const acceptance = [...candidates.values()].find(candidate => candidate.measured.checkpoint.isolated.integrated)
      const measured = acceptance ?? [...candidates.values()].find(candidate => !active.has(candidate.selected.task_id)
        && !measuring.has(candidate.selected.task_id) && !candidate.needsMeasurement && !candidate.needsRepair)
      if (measured && measuring.size === 0) {
        await synchronize(measured)
        if (!measured.needsMeasurement && !measured.needsRepair) { await accept(measured); continue }
      }
      // Refill independently of the evaluator. Never wait for every sibling first.
      if (context.remaining.workers > 0 && !context.run.acceptance) {
        if (!candidates.size && waitsOnlyOnOtherAreas(context)) return await stopForCrossAreaWaits()
        const repair = [...candidates.values()].find(candidate => candidate.needsRepair && !active.has(candidate.selected.task_id))
        if (repair) { await prepareRepair(repair); continue }
        const selected = await select(null, [...candidates.keys()])
        if (selected.task_id === null) {
          if (!generatedWithoutWork && measuring.size === 0 && context.remaining.measurements === context.run.measurement_limit) {
            await generate(); generatedWithoutWork = true; continue
          }
          if (!candidates.size) throw new Error('Proposal made no eligible progress')
        } else if (selected.kind === 'supporting') { supportWaiting = selected; continue }
        else {
          generatedWithoutWork = false
          let plan = retainedPlans[selected.task_id] ?? null
          if (selected.complexity === 'advanced' && !plan) {
            const raw = await dispatch(selected.task_id, 'preparation', 'planner', workerPrompt(selected,
              'Plan advanced hypothesis without edits. Return the strict four-field semantic planner object.'), { schema: plannerSchema })
            plan = parsePlannerResult(raw)
            if (!plan || plan.verdict !== 'plan' || !reconstructPlannerResult(plan, area, selected.task_id)) throw new Error('Advanced source requires a valid retained semantic plan')
          }
          const admitted = await command(['work-context', area, '--task', selected.task_id, '--store'], `${selected.task_id}: fresh source admission`)
          const stored = parseStoredContext(admitted, area)
          if (!stored || stored.taskId !== selected.task_id || !stored.hypothesis) throw new Error('Source dispatch lost its fresh explicit task')
          const source = await control('create current source',
            `Create an exclusively run-owned linked Git worktree outside the authoritative checkout and source branch from authoritative destination HEAD ${stored.head} current at THIS dispatch, verifying its code matches accepted baseline. Do not reuse a sibling or change destination. Keep records unchanged. Return exactly {"source_root":canonical-absolute-path,"head":full-commit}. Run ${runId}, task ${selected.task_id}, request namespace ${invocation}. Authoritative root: ${root}`)
          if (!exactObject(source, ['source_root', 'head']) || !validText(source.source_root) || !source.source_root.startsWith('/')
            || source.source_root === root || source.source_root.startsWith(`${root}/`) || source.head !== stored.head) throw new Error('Source was not created from current dispatch HEAD')
          const request = await reserve(selected.task_id, 'preparation')
          const value = inspectExperiment(await experiment(selected.task_id, 'admit', ['--source', source.source_root,
            ...selected.paths.flatMap(path => ['--path', path]), '--reservation', request]), selected.task_id)
          if (value.dispatch_authorized !== true || value.checkpoint.preparation?.request !== request
            || value.checkpoint.isolated?.source_root !== source.source_root || value.checkpoint.isolated.dispatch_head !== stored.head) throw new Error('Admit replay or changed source refused implementation dispatch')
          const candidate = { selected: { ...selected, context: admitted }, measured: value, plan, request, needsMeasurement: false }
          candidates.set(selected.task_id, candidate)
          launchSource(candidate)
          continue
        }
      }
      await refresh()
      if (!context.run.acceptance && context.remaining.measurements > 0 && measuring.size < context.run.measurement_limit) {
        const candidate = [...candidates.values()].find(item => item.needsMeasurement && !item.needsRepair
          && !active.has(item.selected.task_id) && !measuring.has(item.selected.task_id))
        if (candidate) {
          await synchronize(candidate)
          if (candidate.needsRepair) continue
          const measurementRequest = candidate.measurementRequest
          launch('measurement', candidate, () => experiment(candidate.selected.task_id, 'evaluate',
            measurementRequest ? ['--reservation', measurementRequest] : []))
          candidate.measurementRequest = null
          continue
        }
      }
      if (active.size || measuring.size || arrivals.length) { await nextArrival(); continue }
      throw new Error('No safe dispatch, measurement or acceptance progress; inspect retained capacity and candidates')
    }
  } catch (error) {
    // No cancellation API is assumed. Drain known calls, retaining ambiguous effects for fresh recovery.
    await Promise.all([...active.values(), ...measuring.values()])
    recovery.push(...arrivals.map(event => `${event.candidate.selected.task_id}: inspect stopped ${event.kind} effects and original request before recovery`))
    throw error
  }
}

try {
  const initial = await control('inspect authorization',
    `This is fresh invocation ${JSON.stringify(invocation)}, authorization ${mode}. Inspect live durable state; never use stopped-script replay as fresh dispatch authority. Read brief first, research settings, complete work-context and existing run context. Return exactly {"repository_root":absolute-path,"safe":boolean,"reason":string,"run_context":run-context|null}. Require open area, safe branch and attributable staged/unstaged/untracked work. Do not mutate. No existing run is expected for first authorization.`)
  if (!exactObject(initial, ['repository_root', 'safe', 'reason', 'run_context'])
    || !validText(initial.repository_root) || !initial.repository_root.startsWith('/') || initial.safe !== true
    || !validText(initial.reason)) throw new Error(initial.reason ?? 'Unsafe authorization context')
  root = initial.repository_root
  if (initial.run_context !== null && !validRunContext(initial.run_context)) throw new Error('Invalid original run context')
  context = initial.run_context
  if (context && [...Object.keys(context.run.reservations), ...Object.keys(context.run.admissions)]
    .some(request => request.startsWith(`${invocation}-`))) {
    throw new Error('Invocation identity was already used; resume with a fresh invocation_id and inspect live effects')
  }
  if (mode === 'start') {
    if (context && !context.run.ended) throw new Error('An unfinished run must resume its existing identity')
    const flags = ['--scope', input.scope]
    if (input.until_interrupted === true) flags.push('--until-interrupted')
    for (const key of [...limits, ...capacities]) if (Object.hasOwn(input, key)) flags.push(`--${key.replaceAll('_', '-')}`, String(input[key]))
    startAttempted = true
    context = await command(['research', 'run', area, 'start', ...flags], 'start authorized run')
    if (!validRunContext(context)) throw new Error('Start did not establish run context')
    runId = context.run.id
  } else {
    if (!context || context.run.id !== input.run_id || context.run.ended) throw new Error('Resume must name the same unfinished authorization')
    runId = input.run_id
  }
  if (!profiles) {
    if (mode === 'resume') recovery.push('Concrete worker handoff unavailable; resolve all five Claude roles afresh before the next dispatch.')
    const flags = [...(input.run_profile ? ['--run-profile', input.run_profile] : []),
      ...Object.entries(input.role_profiles ?? {}).flatMap(([role, profile]) => ['--role-profile', `${role}=${profile}`])]
    const resolved = {}
    for (const role of roles) resolved[role] = await command(['config', 'profile', 'resolve', 'claude', role, '--area', area, ...flags], `resolve ${role}${mode === 'resume' ? ' afresh; concrete handoff unavailable' : ''}`)
    if (!validProfiles(resolved)) throw new Error('Could not freeze all five concrete Claude roles')
    profiles = resolved
  }
  await recoverEffects(false)
  if (mode === 'resume' && context.stop_reason === 'interruption') await runCommand('resume')
  if (execution === 'rolling') return await runRollingResearch()
  let generatedWithoutWork = false
  while (true) {
    await refresh()
    await recoverEffects()
    const unfinished = context.checkpoints.find(checkpoint => !checkpoint.index_updated)
    if (unfinished) {
      const selected = await select(unfinished.task)
      await hypothesis(selected)
      continue
    }
    if (context.stop_reason && context.stop_reason !== 'blocker') return report('stopped', context.stop_reason)
    if (!context.run.comparison_valid && context.stop_reason !== 'blocker') {
      await checkedControl('prepare accepted rebaseline', `Require clean accepted HEAD and unchanged metric/direction/correctness policy; recover or safely restore unfinished candidates before rebaseline. Do not replace supplied samples to clear invalidation. Context: ${JSON.stringify(context)}`)
      await runCommand('rebaseline', ['--request', nextRequest('rebaseline')])
      continue
    }
    if (context.stop_reason !== 'blocker' && waitsOnlyOnOtherAreas(context)) return await stopForCrossAreaWaits()
    const selected = await select()
    if (selected.task_id === null) {
      if (generatedWithoutWork) throw new Error('Proposal made no eligible progress; no further generation authorized by this invocation')
      await generate()
      generatedWithoutWork = true
      continue
    }
    generatedWithoutWork = false
    if (selected.kind === 'supporting') {
      await support(selected)
      await refresh()
      if (context.stop_reason === 'blocker') {
        await checkedControl('confirm shared repair', `Check the recorded shared environment failure is actually resolved by committed supporting work. Preserve original policy. Context: ${JSON.stringify(context)}`)
        await runCommand('resolve')
      }
    } else await hypothesis(selected)
  }
} catch (error) {
  reporting = true
  recovery.push(String(error?.message ?? error))
  if (root && startAttempted && !runId) {
    try {
      const live = await command(['research', 'run', area, 'context'], 'inspect uncertain start effects')
      if (validRunContext(live) && !live.run.ended) { context = live; runId = live.run.id }
    } catch (inspectionError) { recovery.push(`Inspect uncertain start effects: ${String(inspectionError?.message ?? inspectionError)}`) }
  }
  if (runId) {
    try {
      await refresh()
      if (!context.stop_reason) await runCommand('stop')
      await refresh()
    } catch (inspectionError) { recovery.push(`Inspect live run and effects: ${String(inspectionError?.message ?? inspectionError)}`) }
  }
  recovery.push('After stop/failure use a new zdev-research resume invocation_id with the same run_id; recover live effects before a newly reserved dispatch.')
  return report('blocker', context?.stop_reason ?? 'unsafe state')
}

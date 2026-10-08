export const meta = {
  name: 'zdev-research',
  description: 'Authorize or resume serial research with counted workers and durable experiment recovery',
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
const limits = ['attempt_limit', 'command_limit', 'command_seconds', 'time_limit_seconds', 'target']
let root = null
let runId = null
let startAttempted = false
let profiles = input?.frozen_profiles ?? null
const retainedPlans = input?.retained_plans ?? {}
let context = null
let requestNumber = 0
let pendingRecoveryVerification = null
const completed = []
const recovery = []
const nextRequest = kind => `${invocation}-${kind}-${++requestNumber}`
const report = (verdict, reason) => JSON.stringify({
  schema_version: 1, kind: 'research', area: area ?? 'unknown', run_id: runId,
  scope: context?.run?.scope ?? input?.scope ?? null,
  completed: [...completed, ...(context?.checkpoints ?? [])
    .filter(checkpoint => checkpoint.index_updated && !completed.some(item => item.task_id === checkpoint.task))
    .map(checkpoint => ({ task_id: checkpoint.task, decision: checkpoint.final_record.decision, commit: checkpoint.commit }))],
  accepted_baseline: context?.accepted_baseline ?? null,
  remaining: context?.remaining ?? null, stop_reason: reason,
  recovery_actions: recovery, frozen_profiles: profiles, retained_plans: retainedPlans, verdict,
})
if (!input || !/^[a-z0-9][a-z0-9-]*$/.test(area ?? '')
  || !/^[A-Za-z0-9][A-Za-z0-9_-]*$/.test(invocation ?? '')
  || !['start', 'resume'].includes(mode)
  || (mode === 'start' && !validText(input.scope))
  || (mode === 'resume' && !validText(input.run_id))
  || (Object.hasOwn(input, 'until_interrupted') && typeof input.until_interrupted !== 'boolean')
  || limits.some(key => Object.hasOwn(input, key)
    && (typeof input[key] !== 'number' || !Number.isFinite(input[key])
      || (key !== 'target' && input[key] <= 0)
      || (['attempt_limit', 'command_limit'].includes(key) && !Number.isInteger(input[key]))))
  || (mode === 'resume' && ['scope', 'until_interrupted', ...limits].some(key => Object.hasOwn(input, key)))
  || (input.run_profile && !/^[A-Za-z0-9][A-Za-z0-9_-]*$/.test(input.run_profile))
  || (input.role_profiles && (!exactObject(input.role_profiles, Object.keys(input.role_profiles))
    || Object.entries(input.role_profiles).some(([role, profile]) => !roles.includes(role)
      || !/^[A-Za-z0-9][A-Za-z0-9_-]*$/.test(profile))))
  || (profiles !== null && !validProfiles(profiles))
  || !retainedPlans || Array.isArray(retainedPlans) || typeof retainedPlans !== 'object'
  || Object.entries(retainedPlans).some(([task, plan]) => !task.startsWith(`${area}-`)
    || validateSemanticPlannerResult(plan)?.verdict !== 'plan')) {
  recovery.push('Supply structured area, authorization start/resume, fresh invocation_id, scope for start or run_id for resume, and valid limits/profiles.')
  return report('blocker', 'input')
}

const control = async (label, prompt, schema = null) => {
  const raw = await agent(`${repositoryGuidance}\n\n${root ? `Use absolute repository cwd ${JSON.stringify(root)} for every tool. ` : ''}Read installed research reference ${JSON.stringify(researchReferencePath)} and coding guidance ${JSON.stringify(codingReferencePath)}. Task contract: ${JSON.stringify(taskWorkflowContractPath)}. ${prompt}`,
    { label: `zdev research ${area}: ${label}`, model: 'haiku', ...(schema ? { schema } : {}) })
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
const refresh = async () => {
  const value = await command(['research', 'run', area, 'context'], 'fresh run context')
  if (!validRunContext(value) || (runId && value.run.id !== runId)) throw new Error('Fresh run context did not preserve the authorized identity')
  context = value
  return value
}
const reserve = async (task, kind) => {
  const request = nextRequest(kind)
  const value = await runCommand('reserve', ['--request', request, '--task', task, '--kind', kind], `reserve ${kind} ${task}`)
  const reservation = value.run?.reservations?.[request]
  // The last attempt authorizes this dispatch even though the returned context now says budget.
  if (!validRunContext(value) || value.run.id !== runId || !reservation
    || reservation.task !== task || reservation.kind !== kind || reservation.settled || reservation.bound) {
    throw new Error('Reservation did not authorize the requested fresh dispatch')
  }
  context = value
  return request
}
const dispatch = async (task, kind, role, prompt, extra = {}) => {
  const request = await reserve(task, kind)
  const options = nativeWorkerOptions({ profiles }, role,
    { agentType: `zdev:zdev-${role}`, label: `zdev research ${task}: ${role}`, ...extra })
  const raw = await agent(prompt, options)
  if (role === 'planner') {
    const plan = parsePlannerResult(raw)
    // Preserve completed planning even when settlement's outcome needs recovery.
    if (plan?.verdict === 'plan' && reconstructPlannerResult(plan, area, task)) retainedPlans[task] = plan
  }
  await runCommand('settle', ['--request', request], `settle ${request}`)
  return raw
}
const workerPrompt = (selected, action) => `${repositoryGuidance}\n\nUse absolute repository cwd ${JSON.stringify(root)} for every file and command tool. Read task contract ${JSON.stringify(taskWorkflowContractPath)}, coding guidance ${JSON.stringify(codingReferencePath)}, and research reference ${JSON.stringify(researchReferencePath)}. Read .zdev/${area}/brief.md first, complete task ${selected.task_id} and applicable repository guidance. Load zdev work-context ${area} --show ${selected.context.snapshot} --format json. ${action} Never edit .zdev, stage, finalize, complete a task or commit. Return only your strict role result.`
const select = async (unfinishedTask = null) => {
  const selected = await control('select ready work',
    `Read fresh work-context and tasks list/show. ${unfinishedTask ? `Select only unfinished checkpoint task ${unfinishedTask} for recording/restoration, not new dispatch.` : 'Select from the entire ready frontier using ordinary eligibility and dependencies, preferring afk true. Under shared blocker choose only supporting work relevant to the recorded blocker; otherwise return task_id null when no eligible work remains.'} Read all prior rejected, inconclusive and failed-to-run lessons. For a selected task store explicit work-context --task <task> --store. Read full task/policy and declare exact clean candidate files before any hypothesis edit, including named new files; exclude managed records, directories, symlinks and unrelated changes. Return exactly {"task_id":string|null,"kind":"supporting"|"hypothesis"|null,"complexity":"routine"|"standard"|"advanced"|null,"context":compact-work-context|null,"paths":string[],"reason":string}. Keep files and Git unchanged. Fresh run data: ${JSON.stringify(context)}`)
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
    `Recover live durable effects before selection. Inspect admissions, attributable uncommitted records, outstanding reservations, rebaseline requests and experiments. Retry only identical admission content under its original request; finish exact publication/staging/cached-diff/commit checkpoints through zdev commit. Inspect/recover experiments and interrupted rebaseline using the original request; never relaunch unknown measurements. Publish/commit an actual rebaseline's exact area metadata before new work. Reconcile reservations from actual worker effects without refunds or duplicate dispatch. Resume an already verified ordinary completion with its retained verification evidence and existing completion coordinator. When that evidence is unavailable but original baseline and ownership are established, safely reopen an uncommitted done ordinary task with zdev task reopen, store explicit ready work-context, and return reverify {"task_id":string,"context":compact-work-context,"baseline_snapshot":original-snapshot,"implementation":nonempty-summary}; otherwise preserve ambiguous state and report its concrete recovery action. Return exactly {"safe":boolean,"reason":string,"reverify":object|null}. Do not launch workers in this helper. Pending candidate code must be restored through authorized lifecycle before accepted supporting edits. Preserve unresolved measurement/process ownership. Context: ${JSON.stringify(context)}`)
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
      await checkedControl('check recovered completion findings', `Independently check substantive task-owned findings against snapshot ${stored.baselineSnapshot}: ${JSON.stringify(semantic.findings)}`)
      if (stored.complexity === 'advanced' && !retainedPlans[handoff.task_id]) throw new Error('Advanced completion rework requires the retained original semantic plan')
      await support(selected)
    } else {
      const result = await agent(`${repositoryGuidance}\n\nUse absolute cwd ${JSON.stringify(root)}. Act as the existing ordinary completion coordinator for ${handoff.task_id}. Compare zdev work-context ${area} --compare ${stored.baselineSnapshot} --format json and require exact equal true before mutation. Read ${JSON.stringify(taskWorkflowContractPath)}. Run task done, stage exact attributed source paths and exact task/TASKS.md records, inspect full cached diff and zdev commit. Preserve unfinished checkpoint on failure. Return the standard PASS/BLOCKER zdev-implement ${area} ${handoff.task_id} result and required fields. Independent verification: ${JSON.stringify(semantic)}. Original baseline: ${handoff.baseline_snapshot}.`,
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
  const commit = pass && field(pass, 'Commit ID')?.replace(/^`|`$/g, '')
  if (!pass || field(pass, 'Area') !== area || field(pass, 'Task') !== task
    || !['Summary', 'Changed files', 'Validation', 'Verifier evidence'].every(name => field(pass, name))
    || !/^[0-9a-f]{40}$/.test(commit ?? '')) throw new Error(typeof result === 'string' ? result : 'Supporting workflow returned no valid completion')
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
      return dispatch(selected.task_id, 'supporting', role,
        `${workerPrompt(selected, 'Follow the ordinary task contract and this exact dispatch.')}\n\n${prompt}`, options)
    }
    return agent(`${root ? `Use absolute repository cwd ${JSON.stringify(root)} for every tool.\n` : ''}${prompt}`, options)
  })
  recordSupportingCompletion(result, selected.task_id)
}
const experiment = (task, action, flags = []) => command(['research', 'experiment', area, task, action, ...flags], `${task}: ${action}`)
const inspectExperiment = (value, task) => {
  if (value?.schema_version !== 1 || value.area !== area || value.task !== task
    || !value.checkpoint || !Array.isArray(value.checkpoint.attempts)
    || typeof value.comparison_valid !== 'boolean') throw new Error('Invalid experiment checkpoint')
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
    let raw = await agent(workerPrompt({ ...selected, context: admitted },
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
      raw = await agent(workerPrompt({ ...selected, context: refreshed },
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
        const raw = await agent(workerPrompt({ ...selected, context: refreshed },
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
    for (const key of limits) if (Object.hasOwn(input, key)) flags.push(`--${key.replaceAll('_', '-')}`, String(input[key]))
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

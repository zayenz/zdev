import fs from 'node:fs'
import { spawnSync } from 'node:child_process'

const root = process.env.ZDEV_PROBE_ROOT
const binary = process.env.ZDEV_PROBE_BINARY
const scenario = process.env.ZDEV_PROBE_SCENARIO
const planRecovery = ['hypothesis-plan-settlement', 'hypothesis-plan-begin', 'support-plan-settlement'].includes(scenario)
const source = fs.readFileSync(process.env.ZDEV_PROBE_SOURCE, 'utf8').replace('export const meta =', 'const meta =')
const run = new (Object.getPrototypeOf(async function () {}).constructor)('args', 'agent', source)
for (const invalid of [{area: 'work'}, {area: 'work', authorization: 'resume', invocation_id: 'invalid', run_id: 'run', attempt_limit: 1}]) {
  const result = JSON.parse(await run(invalid, async () => { throw new Error('Invalid input dispatched a helper') }))
  if (result.stop_reason !== 'input' || result.run_id !== null) throw new Error('Invalid structured authorization mutated run state')
}
const execute = (file, argv, input) => {
  const result = spawnSync(file, argv, { cwd: root, encoding: 'utf8', input })
  if (result.status !== 0) throw new Error(`${file} ${argv.join(' ')}: ${result.stderr}`)
  return result.stdout.trim()
}
const zdev = argv => JSON.parse(execute(binary, [...argv, '--format', 'json']))
const git = argv => execute('git', argv)
const temporary = (name, value) => {
  const path = `${root}/.git/${name}.json`
  fs.writeFileSync(path, JSON.stringify(value))
  return path
}
const state = () => zdev(['research', 'run', 'work', 'context'])
const compact = task => zdev(['work-context', 'work', '--task', task, '--store'])
const safe = { safe: true, reason: 'Checked actual fixture state.' }
const taskNumber = task => Number(task.split('-').at(-1))
const worker = (task, verdict = 'ready') => JSON.stringify({ schema_version: 1, kind: 'implementer', area: 'work', task_id: task,
  verdict, summary: 'Changed declared implementation and checked task requirements.', evidence: ['seed.txt'], findings: [], escalation: 'none' })
const planner = { verdict: 'plan', summary: 'Use the existing representation seam.', plan: {
  approach: 'Change seed.txt within the declared checkpoint and validate the fixed evaluator.', paths: ['seed.txt'], validation: ['Fixed evaluator and correctness check.'],
}, findings: [] }
const verifier = (verdict = 'pass') => JSON.stringify({ verdict, summary: 'Reviewed the complete task and measured policy against the supplied snapshot.',
  findings: verdict === 'rework' ? ['Fix the supporting representation.'] : [], escalation: 'none' })
const calls = []
const workers = []
const dispatched = new Set()
const proposals = []
let injected = false
let supportReviews = 0
let supportWork = 0
let generation = 0
let resumed = false
let runId = null
let activeTask = null
let supportBaseline = null
const pathsForAdmission = context => Object.values(context.run.admissions).flatMap(admission => admission.tasks)
  .map(task => zdev(['task', 'show', 'work', task]).path)
const publish = context => {
  const paths = pathsForAdmission(context).filter(path => git(['status', '--porcelain', '--', path]).startsWith('??'))
  if (paths.length) {
    git(['add', '--', ...paths, '.zdev/work/TASKS.md'])
    git(['diff', '--cached'])
    zdev(['commit', '-m', 'Publish authorized generated research work'])
  }
}
const hypothesisDraft = (key, complexity = 'standard') => ({ key, title: key, complexity, afk: true,
  outcome: 'Measure the candidate representation.', done_when: ['Retain measured evidence and lesson.'], validation: ['Fixed evaluator and correctness.'],
  hypothesis: { mechanism: 'The representation changes evaluator cost.', context_tasks: [] } })
const supportDraft = { key: 'support', title: 'Support the evaluator', complexity: 'advanced', afk: true,
  outcome: 'Repair accepted supporting code.', done_when: ['Commit independently verified supporting code.'], validation: ['Inspect supporting file.'] }
const optionsInput = { area: 'work', authorization: 'start', invocation_id: `probe-${scenario}-first`, scope: 'Implementation and supporting evaluation work in this area.' }
if (planRecovery || ['acceptance-rework', 'support-completion', 'conditions', 'admission'].includes(scenario)) optionsInput.target = 5
if (scenario === 'limits') Object.assign(optionsInput, { until_interrupted: true, target: 5, command_limit: 20, command_seconds: 100, time_limit_seconds: 3600 })
if (scenario === 'last-unit') optionsInput.attempt_limit = 1
if (scenario === 'mixed') Object.assign(optionsInput, { until_interrupted: true, target: 3 })
if (scenario === 'shared') Object.assign(optionsInput, { until_interrupted: true, target: 5 })
const candidate = task => {
  if (['support-completion', 'support-plan-settlement', 'conditions', 'admission'].includes(scenario) && taskNumber(task) > 1) return 'keep'
  if (scenario === 'mixed') return ['discard', '', 'keep', 'fail', 'close'][taskNumber(task) - 1]
  if (['limits', 'shared', 'stale', 'advanced-resume', 'acceptance-rework', 'hypothesis-plan-settlement', 'hypothesis-plan-begin'].includes(scenario)) return 'keep'
  return 'discard'
}
const agent = async (prompt, options) => {
  calls.push({ label: options.label, prompt, model: options.model, effort: options.effort })
  if (options.agentType) {
    const role = options.agentType.replace('zdev:zdev-', '')
    for (const name of ['research.md', 'coding.md']) {
      const reference = `${process.env.ZDEV_PROBE_SOURCE.replace(/workflows\/zdev-research.js$/, '')}skills/zdev/references/${name}`
      if (!prompt.includes(reference) || !fs.existsSync(reference)) throw new Error(`Worker omitted exact installed ${name} path`)
    }
    const task = prompt.match(/[Cc]omplete task (work-\d+)/)?.[1] ?? activeTask
    const current = state()
    const pending = Object.values(current.run.reservations).filter(reservation => reservation.task === task && !reservation.settled)
    const preparation = current.checkpoints.find(checkpoint => checkpoint.task === task)?.preparation?.request
    const reservation = pending.length ? Object.entries(current.run.reservations).find(([key, value]) => pending.includes(value))?.[0]
      : options.label.endsWith('implement hypothesis') ? preparation : null
    if (!reservation || dispatched.has(reservation)) throw new Error(`Unreserved/duplicate ${role} dispatch for ${task}`)
    dispatched.add(reservation)
    if (!options.model || (options.effort === undefined && current.run.policy)) throw new Error('Role lost concrete native model/effort')
    workers.push({ role, task, reserved: current.consumed.work_attempts })
    if (role === 'planner') return JSON.stringify(planner)
    if (planRecovery && resumed && role === 'advanced-implementer' && !prompt.includes(JSON.stringify(planner))) throw new Error('Resume changed or omitted the retained semantic plan')
    const supporting = zdev(['task', 'show', 'work', task]).hypothesis === undefined
    if (role === 'verifier') {
      if (supporting && scenario === 'mixed' && supportReviews++ === 0) return verifier('rework')
      if (!supporting && scenario === 'acceptance-rework' && !injected) { injected = true; return verifier('rework') }
      if (scenario === 'stale' && !injected) { injected = true; fs.writeFileSync(`${root}/seed.txt`, 'keep changed after snapshot\n') }
      return verifier()
    }
    if (supporting) { supportWork++; fs.writeFileSync(`${root}/support.txt`, `support repair ${supportWork}\n`); return worker(task) }
    fs.writeFileSync(`${root}/seed.txt`, `${candidate(task)}\n`)
    if (scenario === 'implementation-blocker' && !injected) { injected = true; return worker(task, 'blocker') }
    if (!injected && ['null', 'throw', 'multiple', 'duplicate', 'malformed', 'advanced-resume'].includes(scenario)) {
      injected = true
      if (scenario === 'throw' || scenario === 'advanced-resume') throw new Error('Worker interrupted after attributable edits')
      if (scenario === 'null') return null
      if (scenario === 'multiple') return `${worker(task)}\n${worker(task)}`
      if (scenario === 'duplicate') return worker(task).replace('"verdict":"ready"', '"verdict":"blocker","verdict":"ready"')
      return '{broken'
    }
    return worker(task)
  }
  const label = options.label.replace('zdev research work: ', '')
  if (label === 'inspect authorization') {
    let context = null
    try { context = state() } catch {}
    if (!prompt.includes(optionsInput.invocation_id) && !prompt.includes(`probe-${scenario}-resume`)) throw new Error('First coordination omitted fresh invocation')
    return JSON.stringify({ repository_root: root, ...safe, run_context: context })
  }
  const commandMatch = prompt.match(/Research command: (\[[^\n]+\])/)
  if (commandMatch) {
    const argv = JSON.parse(commandMatch[1]).slice(1)
    if (scenario === 'refused' && argv.includes('reserve')) return JSON.stringify({ error: 'Preparation reservation refused before launch.' })
    if (planRecovery && !injected && ((scenario.endsWith('settlement') && argv.includes('settle'))
      || (scenario === 'hypothesis-plan-begin' && argv.includes('begin')))) {
      injected = true
      throw new Error('Interrupted after valid planning, before settlement or begin effects')
    }
    if (scenario === 'conditions' && !injected && argv.includes('context')) {
      const prior = state()
      if (prior.checkpoints.some(checkpoint => checkpoint.index_updated)) {
        injected = true
        const settings = zdev(['area', 'research', 'work']).research
        settings.comparison.conditions = 'Changed fixed input; one measurement slot'
        zdev(['area', 'research', 'work', '--from', temporary('changed-settings', settings)])
      }
    }
    const value = JSON.parse(execute(binary, argv))
    if (argv.includes('start')) {
      runId = value.run.id
      if (!optionsInput.until_interrupted && !Object.hasOwn(optionsInput, 'attempt_limit') && value.run.attempt_limit !== 20) throw new Error('Binary default allowance was overridden')
      if (scenario === 'shared') return JSON.stringify(zdev(['research', 'run', 'work', 'block', '--run', runId, '--reason', 'Missing shared support file']))
    }
    return JSON.stringify(value)
  }
  if (label === 'recover pending effects') {
    const context = state()
    publish(context)
    for (const checkpoint of context.checkpoints.filter(checkpoint => !checkpoint.index_updated)) {
      if (checkpoint.final_record || checkpoint.attempts.some(attempt => attempt.measuring)) zdev(['research', 'experiment', 'work', checkpoint.task, 'recover'])
    }
    for (const [request, reservation] of Object.entries(state().run.reservations)) {
      if (!reservation.settled && !reservation.bound) zdev(['research', 'run', 'work', 'settle', '--run', runId ?? context.run.id, '--request', request])
    }
    if (git(['status', '--porcelain', '--', '.zdev/work/area.toml'])) {
      git(['add', '.zdev/work/area.toml']); git(['diff', '--cached']); zdev(['commit', '-m', 'Publish actual research baseline'])
    }
    if (scenario === 'support-completion' && resumed && zdev(['task', 'show', 'work', 'work-002']).status === 'done' && git(['status', '--porcelain', '--', zdev(['task', 'show', 'work', 'work-002']).path])) {
      zdev(['task', 'reopen', 'work', 'work-002'])
      activeTask = 'work-002'
      return JSON.stringify({ ...safe, reverify: { task_id: activeTask, context: compact(activeTask), baseline_snapshot: supportBaseline, implementation: 'Accepted supporting representation awaiting completion.' } })
    }
    return JSON.stringify({ ...safe, reverify: null })
  }
  if (label === 'select ready work') {
    const context = state()
    const unfinished = context.checkpoints.find(checkpoint => !checkpoint.index_updated)
    const tasks = context.tasks.tasks.filter(task => task.state === 'ready')
    const chosen = unfinished ? tasks.find(task => task.id === unfinished.task)
      : context.stop_reason === 'blocker' ? tasks.find(task => !task.hypothesis) : tasks[0]
    activeTask = chosen?.id ?? null
    if (chosen && !chosen.hypothesis && !supportBaseline) supportBaseline = compact(chosen.id).snapshot
    if (!chosen) return JSON.stringify({ task_id: null, kind: null, complexity: null, context: null, paths: [], reason: 'No eligible tasks.' })
    return JSON.stringify({ task_id: chosen.id, kind: chosen.hypothesis ? 'hypothesis' : 'supporting', complexity: chosen.complexity,
      context: compact(chosen.id), paths: chosen.hypothesis ? ['seed.txt'] : [], reason: 'Selected the eligible task from fresh graph.' })
  }
  if (label === 'propose scoped work') {
    const context = state()
    proposals.push(context.tasks.tasks.flatMap(task => (task.research_results ?? []).filter(record => record.lesson).map(record => record.lesson)))
    let tasks
    if (scenario === 'shared' || (['support-completion', 'support-plan-settlement'].includes(scenario) && generation++ === 0)) tasks = [supportDraft]
    else if (['support-completion', 'support-plan-settlement', 'conditions'].includes(scenario)) tasks = [hypothesisDraft('after-rebaseline')]
    else if (scenario === 'mixed' && generation++ === 0) tasks = [supportDraft, hypothesisDraft('keep', 'advanced'), hypothesisDraft('failed')]
    else if (scenario === 'mixed') tasks = [hypothesisDraft('close')]
    else if (scenario === 'admission') tasks = [hypothesisDraft('after-admission')]
    else return JSON.stringify({ bundle: { schema_version: 1, area: 'work', tasks: [] }, reason: 'No useful further proposal.' })
    return JSON.stringify({ bundle: { schema_version: 1, area: 'work', tasks }, reason: 'Use actual prior lessons in scoped work.' })
  }
  if (label === 'admit scoped proposal') {
    const bundle = JSON.parse(prompt.split('Proposal: ')[1])
    const request = prompt.match(/--request ([^ ]+)/)[1]
    const value = zdev(['research', 'run', 'work', 'admit', '--run', runId, '--request', request, '--from', temporary('proposal', bundle)])
    if (scenario === 'admission' && !injected) { injected = true; throw new Error('Admission interrupted before exact publication commit') }
    return JSON.stringify(value)
  }
  if (label === 'publish admitted tasks') { publish(state()); return JSON.stringify(safe) }
  if (['inspect candidate delta', 'inspect repaired delta', 'check acceptance findings', 'confirm shared repair', 'prepare accepted rebaseline'].includes(label)) return JSON.stringify(safe)
  if (label === 'classify hypothesis implementation blocker') return JSON.stringify({ action: 'continue', reason: 'A concrete declared-file correction remains.' })
  if (label === 'assess measured hypothesis') {
    const measured = JSON.parse(prompt.split('Evidence: ')[1])
    const attempts = measured.checkpoint.attempts.length
    let action = measured.assessment?.improved ? 'keep' : measured.assessment?.usable ? 'discard' : attempts < measured.checkpoint.settings.attempt_limit ? 'rerun' : 'failed-to-run'
    if (scenario === 'measurement') action = 'block'
    if (scenario === 'inconclusive') action = attempts < measured.checkpoint.settings.attempt_limit ? 'rerun' : 'inconclusive'
    if (scenario === 'stale' && resumed && !zdev(['work-context', 'work', '--compare', measured.checkpoint.verification_snapshot]).equal) action = 'rerun'
    return JSON.stringify({ action, lesson: `${measured.task}: ${action} measured representation.`, reason: 'Checked actual measured assessment and attempt allowance.' })
  }
  if (label === 'finalize measured hypothesis') {
    const task = prompt.match(/finalize for (work-\d+)/)[1]
    const decision = prompt.match(/--decision ([^ ]+)/)[1]
    const lesson = JSON.parse(prompt.match(/--lesson ("(?:\\.|[^"\\])*")/)[1])
    const argv = ['research', 'experiment', 'work', task, 'finalize', '--decision', decision, '--lesson', lesson]
    const envelope = prompt.match(/ containing (\{[^\n]+\})\. Keep temporary/)
    if (envelope) argv.push('--verification', temporary('acceptance', JSON.parse(envelope[1])))
    const value = zdev(argv)
    if (scenario === 'finalization' && !injected) { injected = true; throw new Error('Finalization response interrupted after actual commit') }
    return JSON.stringify(value)
  }
  // Calls from the composed ordinary body keep its command and completion contracts.
  if (/select ready task|capture verification snapshot|refresh before rework|refresh after implementation blocker/.test(options.label)) return JSON.stringify(compact(activeTask))
  if (options.label.includes('confirm verifier')) {
    const snapshot = prompt.match(/--compare (W[0-9a-f]{16})/)[1]
    return JSON.stringify(zdev(['work-context', 'work', '--compare', snapshot]))
  }
  if (options.label.includes('complete and commit')) {
    const snapshot = prompt.match(/--compare (W[0-9a-f]{16})/)[1]
    if (!zdev(['work-context', 'work', '--compare', snapshot]).equal) throw new Error('Ordinary completion snapshot changed')
    zdev(['task', 'done', 'work', activeTask, '--summary', 'Completed supporting repair.', '--validation', 'Independent verification passed.'])
    if (scenario === 'support-completion' && !injected) { injected = true; throw new Error('Ordinary completion interrupted after task done, verification handoff unavailable') }
    const path = zdev(['task', 'show', 'work', activeTask]).path
    git(['add', '--', 'support.txt', path, '.zdev/work/TASKS.md']); git(['diff', '--cached'])
    zdev(['commit', '-m', 'Complete supporting repair'])
    return `PASS zdev-implement work ${activeTask}\n\nArea: work\nTask: ${activeTask}\nSummary: complete\nChanged files: support.txt\nValidation: passed\nVerifier evidence: independent review\nCommit ID: ${git(['rev-parse', 'HEAD'])}`
  }
  throw new Error(`Unexpected coordination ${options.label}`)
}
if (scenario === 'rolling-cap') {
  // The cap simulator exercises nested agent accounting, not provider execution or measurement evidence.
  const original = zdev(['research', 'run', 'work', 'start', '--scope', 'Cap handoff simulation', '--until-interrupted', '--worker-limit', '1'])
  const admitted = zdev(['research', 'run', 'work', 'admit', '--run', original.run.id, '--request', 'cap-support', '--from',
    temporary('cap-support', { schema_version: 1, area: 'work', tasks: [supportDraft] })])
  publish(admitted)
  const task = admitted.run.admissions['cap-support'].tasks[0]
  const stored = compact(task)
  const frozen = Object.fromEntries(['routine-implementer', 'implementer', 'advanced-implementer', 'planner', 'verifier']
    .map(role => [role, zdev(['config', 'profile', 'resolve', 'claude', role, '--area', 'work'])]))
  const live = structuredClone(state())
  let count = 0
  let namedCalls = 0
  let second = false
  const capAgent = async (prompt, options) => {
    count++
    const label = options.label.replace('zdev research work: ', '')
    if (label === 'inspect authorization') return JSON.stringify({ repository_root: root, ...safe, run_context: live })
    const commandMatch = prompt.match(/Research command: (\[[^\n]+\])/)
    if (commandMatch) {
      const argv = JSON.parse(commandMatch[1]).slice(1)
      if (argv.includes('reserve')) {
        const request = argv[argv.indexOf('--request') + 1]
        live.run.reservations[request] = { task, kind: argv[argv.indexOf('--kind') + 1], worker: true,
          requested_worker_slot: true, bound: false, settled: false, commands: 0, elapsed_seconds: 0 }
        live.consumed.work_attempts++
        live.remaining.workers = 0
        return JSON.stringify({ ...live, dispatch_authorized: true })
      }
      if (argv.includes('settle')) { live.run.reservations[argv[argv.indexOf('--request') + 1]].settled = true; live.remaining.workers = 1 }
      if (argv.includes('stop')) live.stop_reason = 'interruption'
      if (argv.includes('resume')) { live.stop_reason = null; second = true }
      if (second && argv.includes('context')) live.stop_reason = 'budget'
      return JSON.stringify(live)
    }
    if (label === 'recover pending effects') return JSON.stringify({ ...safe, reverify: null })
    if (label === 'select ready work') return JSON.stringify({ task_id: task, kind: 'supporting', complexity: 'advanced',
      context: stored, paths: [], reason: 'Cap simulator repeats a complete composed ordinary iteration.' })
    if (options.agentType) {
      namedCalls++
      const role = options.agentType.replace('zdev:zdev-', '')
      return role === 'planner' ? JSON.stringify(planner) : role === 'verifier' ? verifier() : worker(task)
    }
    if (label === 'confirm standalone role stopped' || label === 'prepare accepted rebaseline') return JSON.stringify(safe)
    if (/select ready task|capture verification snapshot|refresh before rework/.test(options.label)) return JSON.stringify(stored)
    if (options.label.includes('confirm verifier')) return JSON.stringify({ schema_version: 1, area: 'work', snapshot: stored.snapshot, equal: true })
    if (options.label.includes('complete and commit')) return `PASS zdev-implement work ${task}\n\nArea: work\nTask: ${task}\nSummary: cap simulation\nChanged files: support.txt\nValidation: simulated\nVerifier evidence: simulated\nCommit ID: ${stored.head}`
    throw new Error(`Unexpected cap simulation helper ${options.label}`)
  }
  const first = JSON.parse(await run({ area: 'work', authorization: 'resume', execution: 'rolling', cleanup: false,
    invocation_id: 'cap-first', run_id: original.run.id, frozen_profiles: frozen }, capAgent))
  if (first.verdict !== 'stopped' || first.stop_reason !== 'workflow-call-boundary' || first.native_calls !== count
    || count < 900 || count >= 1_000 || namedCalls >= count / 2 || !first.retained_plans[task]) throw new Error(`Native cap omitted mediation/composed calls or lost handoff: ${JSON.stringify(first)}`)
  const before = live.consumed.work_attempts
  const priorNamed = namedCalls
  count = 0
  const resumed = JSON.parse(await run({ area: 'work', authorization: 'resume', execution: first.execution, cleanup: first.cleanup,
    invocation_id: 'cap-second', run_id: first.run_id, frozen_profiles: first.frozen_profiles,
    retained_plans: first.retained_plans, retained_candidates: first.retained_candidates }, capAgent))
  if (resumed.run_id !== first.run_id || resumed.stop_reason !== 'budget' || live.consumed.work_attempts !== before
    || namedCalls !== priorNamed || resumed.native_calls !== count) throw new Error('Fresh invocation cap recovery replayed roles or reset allowance')
  console.log(JSON.stringify({ report: first, resumed, consumed: live.consumed, named_calls: priorNamed }))
  process.exit(0)
}

if (scenario.startsWith('rolling')) {
  const { spawn } = await import('node:child_process')
  const runAsync = argv => new Promise((resolve, reject) => {
    const child = spawn(binary, argv, { cwd: root, env: process.env })
    let stdout = '', stderr = ''
    child.stdout.on('data', bytes => { stdout += bytes })
    child.stderr.on('data', bytes => { stderr += bytes })
    child.on('exit', code => code === 0 ? resolve(JSON.parse(stdout)) : reject(new Error(stderr)))
  })
  const capacityOne = scenario === 'rolling-one'
  const failures = ['rolling-null', 'rolling-throw', 'rolling-invalid', 'rolling-replay', 'rolling-prepare', 'rolling-admission', 'rolling-finalization', 'rolling-measurement', 'rolling-ambiguous', 'rolling-process', 'rolling-support-completion']
  const events = []
  const sources = new Map()
  let releaseSlow = null
  let releaseThird = null
  let overlapped = false
  let refill = false
  let checkedRework = false
  let reviews = 0
  let measurementActive = 0
  let changedConditions = false
  let slowCompleted = false
  let malformedClaim = null
  let safeResume = false
  let createdAfterKeep = false
  let generated = false
  let rollingGeneration = 0
  const initialHead = git(['rev-parse', 'HEAD'])
  const taskPaths = task => task === 'work-001' || taskNumber(task) > 3 || (scenario === 'rolling-conflict' && task === 'work-002') ? ['seed.txt']
    : task === 'work-002' ? ['delete.txt'] : ['mode.txt']
  const readySelection = prompt => {
    const excluded = JSON.parse(prompt.match(/Exclude these already assigned or unfinished tasks: (\[[^\]]*\])\./)[1])
    const explicit = prompt.match(/Select only unfinished checkpoint task (work-\d+)/)?.[1]
    const current = state()
    const eligible = current.tasks.tasks.filter(task => task.state === 'ready' && !excluded.includes(task.id))
    const chosen = explicit ? eligible.find(task => task.id === explicit) : current.stop_reason === 'blocker'
      ? eligible.find(task => !task.hypothesis) : eligible[0]
    activeTask = chosen?.id ?? null
    if (chosen && !chosen.hypothesis && !supportBaseline) supportBaseline = compact(chosen.id).snapshot
    if (!chosen) return { task_id: null, kind: null, complexity: null, context: null, paths: [], reason: 'No unassigned ready work.' }
    return { task_id: chosen.id, kind: chosen.hypothesis ? 'hypothesis' : 'supporting', complexity: chosen.complexity,
      context: compact(chosen.id), paths: chosen.hypothesis ? taskPaths(chosen.id) : [], reason: 'Fresh complete frontier selection.' }
  }
  const rollingAgent = async (prompt, options) => {
    const label = options.label.replace('zdev research work: ', '')
    const commandMatch = prompt.match(/Research command: (\[[^\n]+\])/)
    if (commandMatch) {
      calls.push({ label: options.label, prompt })
      const argv = JSON.parse(commandMatch[1]).slice(1)
      if (argv.includes('evaluate')) {
        const task = argv[3]
        if (measurementActive++) throw new Error('Second actual evaluator launched despite measurement limit one')
        events.push(`measure:${task}`)
        // Mediation can be pending before the public command owns its durable slot.
        await new Promise(resolve => setTimeout(resolve, 15))
        const pending = runAsync(argv)
        let finished = false
        pending.then(() => { finished = true }, () => { finished = true })
        // Wait until the child has acquired durable measurement ownership.
        while (!finished && !state().checkpoints.find(checkpoint => checkpoint.task === task)?.attempts.some(attempt => attempt.measuring)) {
          await new Promise(resolve => setTimeout(resolve, 2))
        }
        if (releaseThird) { overlapped = true; releaseThird(); releaseThird = null }
        if (releaseSlow) { releaseSlow(); releaseSlow = null }
        let value
        try { value = await pending } catch (error) {
          if (scenario === 'rolling-process') injected = true
          throw error
        } finally { measurementActive-- }
        if (scenario === 'rolling-record-only' && !injected) { injected = true; git(['commit', '--allow-empty', '-m', 'Record-only checkpoint revision']) }
        if (scenario === 'rolling-measurement' && !injected) { injected = true; throw new Error('Lost evaluator response after actual published measurement') }
        return JSON.stringify(value)
      }
      if (argv.includes('reconcile')) events.push(`reconcile:${argv[3]}`)
      const value = JSON.parse(execute(binary, argv))
      if (argv.includes('start')) runId = value.run.id
      if (argv.includes('reserve') && argv.includes('--worker-slot')) {
        if (scenario === 'rolling-replay' && !injected) { injected = true; value.dispatch_authorized = false; malformedClaim = argv[argv.indexOf('--request') + 1] }
        // Public same-request replay never grants a second launch, nor permits changed identity.
        const retry = JSON.parse(execute(binary, argv))
        if (retry.dispatch_authorized !== false) throw new Error('Role reservation replay authorized dispatch')
        const changed = spawnSync(binary, argv.map(item => item === 'preparation' ? 'confirmation' : item), { cwd: root, encoding: 'utf8' })
        if (argv.includes('preparation') && changed.status === 0) throw new Error('Changed role claim identity was accepted')
      }
      if (argv.includes('admit') && argv.includes('--source')) {
        const retry = JSON.parse(execute(binary, argv))
        if (retry.dispatch_authorized !== false) throw new Error('Admit replay authorized source dispatch')
        if (scenario === 'rolling-admission' && !injected) { injected = true; throw new Error('Admission response interrupted before source dispatch') }
      }
      if (argv.includes('prepare')) {
        const retry = JSON.parse(execute(binary, argv))
        if (retry.dispatch_authorized !== false) throw new Error('Prepare replay authorized repair dispatch')
        if (scenario === 'rolling-prepare' && !injected) { injected = true; throw new Error('Prepare response interrupted after owned withdrawal') }
      }
      if (argv.includes('integrate') && measurementActive) throw new Error('Acceptance integrated over another evaluator')
      return JSON.stringify(value)
    }
    if (options.agentType || options.label.endsWith('check findings')) {
      const role = options.agentType?.replace('zdev:zdev-', '') ?? 'verifier'
      const task = prompt.match(/[Cc]omplete task (work-\d+)/)?.[1] ?? activeTask
      const current = state()
      const checkpoint = current.checkpoints.find(checkpoint => checkpoint.task === task)
      const request = options.label.endsWith('source implementation') ? checkpoint?.preparation?.request
        : Object.entries(current.run.reservations).find(([, claim]) => claim.task === task && claim.worker && !claim.settled && !claim.bound)?.[0]
      const claim = current.run.reservations[request]
      if (!claim?.worker || !claim.requested_worker_slot || dispatched.has(request)) throw new Error(`Missing or replayed native ${role} worker claim ${request}`)
      dispatched.add(request)
      if (!options.model || options.effort === undefined) throw new Error('Native role lost frozen model/effort')
      workers.push({ role, task, request, model: options.model, effort: options.effort })
      if (options.label.endsWith('check findings')) { checkedRework = true; return JSON.stringify(safe) }
      if (role === 'planner') return JSON.stringify(planner)
      if (role === 'verifier') {
        events.push(`verify:${task}`)
        if (checkpoint) {
          if (!checkpoint.isolated.integrated || !prompt.includes(checkpoint.verification_snapshot)) throw new Error('Verifier did not inspect fresh integrated destination')
          if ((scenario === 'rolling-rework' || scenario === 'rolling-prepare') && reviews++ === 0) return verifier('rework')
          if (scenario === 'rolling-stale-verifier' && !injected) { injected = true; fs.writeFileSync(`${root}/unrelated-work.txt`, 'changed during verification\n') }
        }
        return verifier()
      }
      if (!checkpoint) { supportWork++; fs.writeFileSync(`${root}/support.txt`, 'supported\n'); return worker(task) }
      const assigned = checkpoint.isolated.source_root
      if (!prompt.includes(assigned) || !prompt.includes(checkpoint.isolated.brief) || !prompt.includes(checkpoint.isolated.task_record)) throw new Error('Source dispatch omitted absolute source/authoritative record paths')
      if (task === 'work-001' && safeResume && !prompt.includes(JSON.stringify(planner))) throw new Error('Advanced recovery lost retained semantic plan')
      events.push(`source:${task}`)
      if (scenario === 'rolling' && task === 'work-002') await new Promise(resolve => { releaseSlow = resolve })
      if (scenario === 'rolling' && task === 'work-003') {
        if (!slowCompleted) refill = true
        await new Promise(resolve => { releaseThird = resolve })
      }
      const path = taskPaths(task)[0]
      fs.writeFileSync(`${assigned}/${path}`, scenario === 'rolling-conflict' && task === 'work-002' && checkpoint.isolated.refreshing
        ? 'k\n' : taskNumber(task) > 3 ? '' : task === 'work-001' ? 'keep\n' : 'candidate\n')
      if (task === 'work-002') slowCompleted = true
      if (['rolling-null', 'rolling-throw', 'rolling-invalid', 'rolling-ambiguous'].includes(scenario) && !injected) {
        injected = true
        malformedClaim = request
        if (scenario === 'rolling-throw') throw new Error('Source transport failed after attributed edit')
        if (scenario === 'rolling-invalid') return '{invalid'
        return null
      }
      events.push(`returned:${task}`)
      return worker(task)
    }
    if (label === 'select ready work') return JSON.stringify(readySelection(prompt))
    if (label === 'create current source') {
      const task = prompt.match(/task (work-\d+)/)[1]
      const head = prompt.match(/HEAD ([0-9a-f]{40})/)[1]
      if (head !== git(['rev-parse', 'HEAD'])) throw new Error('Source creation used cached destination HEAD')
      const sourceRoot = `${process.env.ZDEV_PROBE_SOURCES}/source-${task}-${sources.size}`
      git(['worktree', 'add', '-b', `probe-${task}-${sources.size}`, sourceRoot, head])
      if (state().checkpoints.some(checkpoint => checkpoint.final_record?.decision === 'accepted') && head !== initialHead) createdAfterKeep = true
      sources.set(task, fs.realpathSync(sourceRoot))
      return JSON.stringify({ source_root: fs.realpathSync(sourceRoot), head })
    }
    if (label === 'check candidate baseline') {
      const measured = JSON.parse(prompt.split('Checkpoint: ')[1].split('. Fresh run: ')[0])
      const settings = zdev(['area', 'research', 'work']).research
      const head = git(['rev-parse', 'HEAD'])
      const code = git(['diff', '--name-only', measured.checkpoint.head, head, '--', '.', ':(exclude).zdev'])
      return JSON.stringify({ refresh: head !== measured.checkpoint.head || JSON.stringify(settings) !== JSON.stringify(measured.checkpoint.settings),
        remeasure: !!code || JSON.stringify(settings) !== JSON.stringify(measured.checkpoint.settings), reason: 'Checked current destination code and actual comparison policy.' })
    }
    if (label === 'confirm source stopped') {
      const task = prompt.match(/for task (work-\d+)/)[1]
      const checkpoint = state().checkpoints.find(item => item.task === task)
      if (checkpoint.isolated.refreshing && execute('git', ['-C', checkpoint.isolated.source_root, 'ls-files', '--unmerged'])) {
        execute('git', ['-C', checkpoint.isolated.source_root, 'add', '--', ...checkpoint.paths])
      }
      return JSON.stringify(safe)
    }
    if (label === 'confirm standalone role stopped') return JSON.stringify(safe)
    if (label === 'recover pending effects') {
      if (scenario === 'rolling-process' && injected) return JSON.stringify({ safe: false, reason: 'Evaluator process stop and final command costs remain unconfirmed.', reverify: null })
      if (scenario === 'rolling-ambiguous' && injected) return JSON.stringify({ safe: false, reason: 'Actual source subprocess stop remains unknown.', reverify: null })
      publish(state())
      for (const checkpoint of state().checkpoints.filter(checkpoint => !checkpoint.index_updated)) {
        let value = zdev(['research', 'experiment', 'work', checkpoint.task, 'recover'])
        if (!value.checkpoint.isolated.worker_stopped) zdev(['research', 'experiment', 'work', checkpoint.task, 'return', '--worker-stopped'])
      }
      for (const [request, claim] of Object.entries(state().run.reservations)) if (!claim.settled && !claim.bound) zdev(['research', 'run', 'work', 'settle', '--run', runId, '--request', request])
      const metadata = '.zdev/work/area.toml'
      if (git(['status', '--porcelain', '--', metadata])) { git(['add', metadata]); zdev(['commit', '-m', 'Publish actual rolling baseline']) }
      if (scenario === 'rolling-support-completion' && safeResume && zdev(['task', 'show', 'work', 'work-004']).status === 'done'
        && git(['status', '--porcelain', '--', zdev(['task', 'show', 'work', 'work-004']).path])) {
        zdev(['task', 'reopen', 'work', 'work-004'])
        activeTask = 'work-004'
        return JSON.stringify({ ...safe, reverify: { task_id: activeTask, context: compact(activeTask), baseline_snapshot: supportBaseline, implementation: 'Verified support waiting for exact completion.' } })
      }
      return JSON.stringify({ ...safe, reverify: null })
    }
    if (label === 'propose scoped work') {
      generated = true
      // Mix ordinary support with a hypothesis that starts after accepted support/rebaseline.
      const tasks = state().stop_reason === 'blocker'
        ? [{ ...supportDraft, key: `shared-repair-${++rollingGeneration}` }]
        : rollingGeneration++ === 0
          ? [{ ...supportDraft, blocked_by: ['work-001', 'work-002', 'work-003'] }, { ...hypothesisDraft('final-current'), blocked_by: ['support'] }]
          : [{ ...hypothesisDraft(`followup-${rollingGeneration}`), blocked_by: ['work-005'] }]
      return JSON.stringify({ bundle: { schema_version: 1, area: 'work', tasks }, reason: 'Use measured lessons in mixed scoped work.' })
    }
    if (label === 'cleanup completed source') {
      const finalized = JSON.parse(prompt.split('Context: ')[1].split(' Return exactly ')[0])
      if (!finalized.cleanup_eligible || !finalized.checkpoint.index_updated || !finalized.checkpoint.isolated.worker_stopped) throw new Error('Cleanup preceded durable completion/actual stop')
      git(['worktree', 'remove', '--force', finalized.checkpoint.isolated.source_root])
      return JSON.stringify(safe)
    }
    if (label === 'prepare accepted rebaseline' || label === 'confirm shared repair') return JSON.stringify(safe)
    if (label === 'finalize measured hypothesis') {
      const raw = await agent(prompt, options)
      if (scenario === 'rolling-finalization' && !injected) { injected = true; throw new Error('Finalization response lost after durable commit') }
      if (scenario === 'rolling-conditions' && !changedConditions) {
        changedConditions = true
        const settings = zdev(['area', 'research', 'work']).research
        settings.comparison.conditions = 'Changed held workload'
        zdev(['area', 'research', 'work', '--from', temporary('rolling-conditions', settings)])
      }
      if (scenario === 'rolling-shared' && !changedConditions) {
        changedConditions = true
        zdev(['research', 'run', 'work', 'block', '--run', runId, '--reason', 'Missing supporting evaluator input'])
      }
      return raw
    }
    if (scenario === 'rolling-support-completion' && !injected && options.label.includes('complete and commit')) {
      const snapshot = prompt.match(/--compare (W[0-9a-f]{16})/)[1]
      if (!zdev(['work-context', 'work', '--compare', snapshot]).equal) throw new Error('Supporting completion changed snapshot')
      zdev(['task', 'done', 'work', activeTask, '--summary', 'Verified supporting implementation.', '--validation', 'Independent verification passed.'])
      injected = true
      throw new Error('Ordinary completion interrupted after task done before its exact commit')
    }
    return agent(prompt, options)
  }
  const input = { ...optionsInput, execution: 'rolling', until_interrupted: true, target: 1, worker_limit: capacityOne ? 1 : 2, measurement_limit: 1,
    cleanup: scenario === 'rolling-cleanup', ...(scenario === 'rolling-budget' ? { attempt_limit: 2 } : {}) }
  let first = JSON.parse(await run(input, rollingAgent))
  if (scenario === 'rolling-budget') {
    const stopped = state()
    if (first.verdict !== 'stopped' || first.stop_reason !== 'budget' || workers.length !== 2 || stopped.consumed.commands
      || stopped.checkpoints[0].index_updated || !stopped.checkpoints[0].isolated.worker_stopped) throw new Error('Last counted unit lost its existing source dispatch or launched free work')
    console.log(JSON.stringify({ report: first, consumed: stopped.consumed, workers, events })); process.exit(0)
  }
  if (scenario === 'rolling-stale-verifier') {
    if (first.verdict !== 'blocker' || state().checkpoints.some(checkpoint => checkpoint.index_updated)
      || fs.readFileSync(`${root}/unrelated-work.txt`, 'utf8') !== 'changed during verification\n') throw new Error('Changed verification snapshot accepted or erased unrelated destination changes')
    console.log(JSON.stringify({ report: first, consumed: state().consumed, workers, events })); process.exit(0)
  }
  if (failures.includes(scenario)) {
    if (!injected && scenario !== 'rolling-process') throw new Error(`Scenario stopped before its intended interruption: ${JSON.stringify(first)}`)
    const before = state()
    if (first.verdict !== 'blocker') throw new Error(`Interrupted rolling workflow claimed success: ${JSON.stringify(first)}`)
    if (malformedClaim && before.run.reservations[malformedClaim].settled) throw new Error('Ambiguous source/role claim was released without confirmed stop')
    if (['rolling-null', 'rolling-throw', 'rolling-invalid'].includes(scenario) && !first.retained_candidates['work-001']?.needs_repair) throw new Error('Interrupted source lost its implementation requirement')
    safeResume = true
    first = JSON.parse(await run({ area: 'work', authorization: 'resume', execution: first.execution, cleanup: first.cleanup, invocation_id: `probe-${scenario}-resume`,
      run_id: before.run.id, frozen_profiles: first.frozen_profiles, retained_plans: first.retained_plans, retained_candidates: first.retained_candidates }, rollingAgent))
    if (state().run.id !== before.run.id || state().run.deadline !== before.run.deadline || state().consumed.work_attempts < before.consumed.work_attempts) throw new Error('Fresh rolling resume replaced identity/deadline/costs')
    if (scenario === 'rolling-process') {
      if (first.verdict !== 'blocker' || !before.consumed.commands || state().consumed.commands !== before.consumed.commands
        || state().checkpoints.some(checkpoint => checkpoint.index_updated)) throw new Error('Unknown evaluator recovery relaunched commands or fabricated completion')
      console.log(JSON.stringify({ report: first, consumed: state().consumed, workers, events })); process.exit(0)
    }
    if (scenario === 'rolling-ambiguous') {
      if (first.verdict !== 'blocker' || state().run.reservations[malformedClaim].settled || state().consumed.commands) throw new Error('Unknown worker stop released claim or ran evaluator')
      console.log(JSON.stringify({ report: first, consumed: state().consumed, workers, events })); process.exit(0)
    }
  }
  const final = state()
  if (first.stop_reason !== 'target') throw new Error(`Rolling run did not reach measured target: ${JSON.stringify({ first, events })}`)
  if (!generated || !createdAfterKeep || !final.run.rebaseline?.published || !workers.some(item => item.task === 'work-004' && item.role === 'planner')) throw new Error('Rolling mixed generation/current HEAD/support rebaseline was omitted')
  if (scenario === 'rolling' && (!refill || !overlapped || events.indexOf('returned:work-003') > events.indexOf('returned:work-002')
    || events.indexOf('measure:work-001') > events.indexOf('verify:work-001'))) throw new Error(`Missing real continuous refill/measurement overlap: ${JSON.stringify(events)}`)
  if ((scenario === 'rolling-rework' || scenario === 'rolling-prepare') && (!checkedRework || reviews < 2 || final.checkpoints[0].attempts.length < 2)) throw new Error('Integrated repair skipped counted checker/withdrawal/new measurement/fresh verification')
  if (scenario === 'rolling-support-completion' && workers.filter(item => item.task === 'work-004' && item.role === 'verifier').length !== 2) throw new Error('Recovered ordinary completion bypassed counted fresh verification')
  if (scenario === 'rolling-conflict' && (workers.filter(item => item.task === 'work-002' && item.role === 'implementer').length !== 2
    || events.indexOf('measure:work-002') < events.indexOf('reconcile:work-002') || !events.includes('reconcile:work-002'))) throw new Error(`Stale source conflict skipped counted repair or current measurement: ${JSON.stringify({ workers, events })}`)
  if (scenario === 'rolling-record-only' && (events.filter(event => event === 'measure:work-001').length !== 1 || !events.includes('reconcile:work-001'))) throw new Error('Record-only refresh relaunched a measurement or omitted public reconcile')
  if (scenario === 'rolling-conditions' && (!changedConditions || final.run.policy.comparison.conditions !== 'Changed held workload')) throw new Error('Condition change bypassed actual baseline gate')
  if (scenario === 'rolling-shared' && (!changedConditions || final.stop_reason === 'blocker')) throw new Error('Shared support failed to resolve/rebaseline')
  if (workers.some(item => item.model !== first.frozen_profiles[item.role].value.model || item.effort !== first.frozen_profiles[item.role].value.effort)) throw new Error('Rolling role changed frozen concrete model/effort')
  for (const checkpoint of final.checkpoints.filter(item => item.index_updated)) {
    for (const attempt of checkpoint.attempts) if (git(['rev-parse', attempt.retained_ref]) !== attempt.candidate) throw new Error('Rolling transport/history was lost')
  }
  console.log(JSON.stringify({ report: first, consumed: final.consumed, workers, events, refill, overlapped }))
  process.exit(0)
}

let first = JSON.parse(await run(optionsInput, agent))
if (scenario === 'last-unit' || scenario === 'refused') {
  if (workers.length !== (scenario === 'last-unit' ? 1 : 0)) throw new Error('Refused/last-unit reservation dispatch count wrong')
  if (first.verdict !== 'blocker' || state().consumed.commands !== 0) throw new Error('Refusal produced false evaluation/completion')
} else if (planRecovery || ['null', 'throw', 'multiple', 'duplicate', 'malformed', 'stale', 'advanced-resume', 'admission', 'finalization', 'support-completion', 'measurement'].includes(scenario)) {
  if (first.verdict !== 'blocker') throw new Error(`Failure reported success: ${JSON.stringify(first)}`)
  const before = state()
  if (['null', 'throw', 'multiple', 'duplicate', 'malformed', 'advanced-resume'].includes(scenario) && before.consumed.commands !== 0) throw new Error('Malformed/interrupted implementation ran evaluator')
  const retained = first.retained_plans
  const plannedTask = scenario === 'support-plan-settlement' ? 'work-002' : 'work-001'
  if (planRecovery) {
    if (JSON.stringify(retained[plannedTask]) !== JSON.stringify(planner)) throw new Error('Completed advanced plan was lost before settlement/begin recovery')
    const expectedBefore = scenario === 'hypothesis-plan-settlement' ? 1 : scenario === 'hypothesis-plan-begin' ? 2 : 3
    if (before.consumed.work_attempts !== expectedBefore || before.checkpoints.some(checkpoint => checkpoint.task === plannedTask)) throw new Error('Interrupted planning changed costs or created an experiment')
    if (!Object.values(before.run.reservations).some(reservation => !reservation.settled && !reservation.bound)) throw new Error('Interrupted reservation effects were not preserved')
  }
  resumed = true
  const original = before.run
  first = JSON.parse(await run({ area: 'work', authorization: 'resume', invocation_id: `probe-${scenario}-resume`, run_id: original.id,
    frozen_profiles: first.frozen_profiles, retained_plans: retained }, agent))
  const after = state()
  if (after.run.id !== original.id || after.run.deadline !== original.deadline || after.consumed.work_attempts < before.consumed.work_attempts) throw new Error('Resume replaced identity/deadline/costs')
  if (planRecovery) {
    const expectedAfter = scenario === 'support-plan-settlement' ? 9 : scenario === 'hypothesis-plan-begin' ? 5 : 4
    if (workers.filter(worker => worker.role === 'planner' && worker.task === plannedTask).length !== 1
      || JSON.stringify(first.retained_plans[plannedTask]) !== JSON.stringify(planner)
      || after.consumed.work_attempts !== expectedAfter || first.stop_reason !== 'target') throw new Error(`Fresh resume replanned, refunded costs or lost the original plan/target: ${JSON.stringify({ first, workers, consumed: after.consumed })}`)
  }
  if (scenario === 'measurement') {
    if (after.consumed.commands !== before.consumed.commands || after.checkpoints[0].attempts[0].measuring || after.checkpoints[0].index_updated || first.stop_reason !== 'blocker') throw new Error('Unknown measurement recovery relaunched commands or fabricated completion')
  } else if (!after.checkpoints.some(checkpoint => checkpoint.index_updated)) throw new Error(`Fresh resume did not finish: ${JSON.stringify(first)}`)
}
const final = state()
if (scenario === 'mixed') {
  const decisions = final.checkpoints.map(checkpoint => checkpoint.final_record?.decision)
  if (JSON.stringify(decisions) !== JSON.stringify(['rejected', 'accepted', 'failed_to_run', 'accepted'])) throw new Error(`Mixed decisions ${JSON.stringify(decisions)}: ${JSON.stringify(first)}`)
  if (first.stop_reason !== 'target' || fs.readFileSync(`${root}/seed.txt`, 'utf8') !== 'close\n') throw new Error('Mixed continuation did not retain target candidate')
  if (!proposals.at(-1).some(lesson => lesson.includes('failed-to-run')) || !proposals.at(-1).some(lesson => lesson.includes('discard'))) throw new Error('Next proposal lost durable failed/rejected lessons')
  if (workers.filter(worker => worker.task === 'work-002' && worker.role === 'planner').length !== 1 || supportWork !== 2 || supportReviews !== 2) throw new Error('Ordinary advanced plan/rework/verification changed')
  if (workers.some(worker => worker.task === 'work-001' && worker.role === 'verifier')) throw new Error('Discard launched an unnecessary verifier')
}
if (scenario === 'implementation-blocker' && workers.filter(worker => worker.role === 'implementer').length !== 2) throw new Error('Actionable hypothesis blocker did not use a counted replacement')
if (scenario === 'support-completion') {
  if (workers.filter(worker => worker.task === 'work-002' && worker.role === 'verifier').length !== 2 || workers.filter(worker => worker.task === 'work-002' && worker.role === 'planner').length !== 1 || first.stop_reason !== 'target') throw new Error(`Ordinary completion recovery skipped fresh counted verification or replanned: ${JSON.stringify({first, workers})}`)
}
if (scenario === 'conditions') {
  if (first.stop_reason !== 'target' || !final.run.rebaseline?.published || final.run.policy.comparison.conditions !== 'Changed fixed input; one measurement slot') throw new Error('Changed conditions did not run actual rebaseline before comparison')
}
if (scenario === 'acceptance-rework') {
  if (workers.filter(worker => worker.role === 'verifier').length !== 2 || !workers.some(worker => worker.role === 'implementer' && worker.reserved > 3) || final.checkpoints[0].attempts.length !== 2 || first.stop_reason !== 'target') throw new Error('Acceptance repair skipped counted work/evaluation/fresh verification')
}
if (scenario === 'limits') {
  if (first.stop_reason !== 'target' || final.run.attempt_limit !== null || final.run.target !== 5 || final.run.command_limit !== 20 || !final.run.deadline) throw new Error('Explicit limits/target were lost')
}
if (scenario === 'shared') {
  if (first.stop_reason !== 'target' || !final.run.comparison_valid || !final.run.rebaseline?.published) throw new Error(`Shared repair did not actually rebaseline: ${JSON.stringify(first)}`)
  if (!workers.some(worker => worker.task === 'work-002') || final.consumed.commands < 4) throw new Error('Shared repair bypassed counted supporting work/actual evaluator')
}
if (scenario === 'inconclusive') {
  if (final.checkpoints[0].final_record?.decision !== 'inconclusive' || final.checkpoints[0].attempts.length !== 2 || !proposals[0].some(lesson => lesson.includes('inconclusive')) || fs.readFileSync(`${root}/seed.txt`, 'utf8') !== 'accepted\n') throw new Error('Exhausted usable observations lost inconclusive lesson/restoration')
}
for (const checkpoint of final.checkpoints.filter(checkpoint => checkpoint.index_updated)) {
  if (!checkpoint.commit || git(['rev-parse', checkpoint.commit]) !== checkpoint.commit) throw new Error('Managed result omitted actual commit')
  for (const attempt of checkpoint.attempts) if (git(['rev-parse', attempt.retained_ref]) !== attempt.candidate) throw new Error('Candidate history was not retained')
}
console.log(JSON.stringify({ report: first, consumed: final.consumed, workers, proposals, correctness_runs: fs.existsSync(`${root}/.git/correctness-runs`) ? fs.readFileSync(`${root}/.git/correctness-runs`, 'utf8') : '', evaluator_runs: fs.existsSync(`${root}/.git/evaluator-runs`) ? fs.readFileSync(`${root}/.git/evaluator-runs`, 'utf8') : '' }))

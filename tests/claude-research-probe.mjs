import fs from 'node:fs'
import { spawnSync } from 'node:child_process'

// Workflows call the scripted workers below. No provider or harness is launched.

const root = process.env.ZDEV_PROBE_ROOT
const binary = process.env.ZDEV_PROBE_BINARY
const scenario = process.env.ZDEV_PROBE_SCENARIO
const source = fs.readFileSync(process.env.ZDEV_PROBE_SOURCE, 'utf8').replace('export const meta =', 'const meta =')
const run = new (Object.getPrototypeOf(async function () {}).constructor)('args', 'agent', source)
for (const invalid of [{area: 'work'}, {area: 'work', authorization: 'resume', invocation_id: 'invalid', run_id: 'run', attempt_limit: 1}]) {
  const result = JSON.parse(await run(invalid, async () => { throw new Error('Invalid input dispatched a helper') }))
  if (result.stop_reason !== 'input' || result.run_id !== null) throw new Error('Invalid structured authorization mutated run state')
}
const execute = (file, argv, input) => {
  if (file !== binary && file !== 'git') throw new Error(`Unexpected test subprocess: ${file}`)
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
let supportFindingChecks = 0
let supportWork = 0
let generation = 0
let runId = null
let activeTask = null
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
if (scenario === 'mixed') Object.assign(optionsInput, { until_interrupted: true, target: 3 })
const candidate = task => ['discard', '', 'keep', 'fail', 'close'][taskNumber(task) - 1]
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
    const supporting = zdev(['task', 'show', 'work', task]).hypothesis === undefined
    if (role === 'verifier') {
      if (supporting && options.label.endsWith('check findings')) { supportFindingChecks++; return verifier('rework') }
      if (supporting && scenario === 'mixed' && supportReviews++ === 0) return verifier('rework')
      return verifier()
    }
    if (supporting) { supportWork++; fs.writeFileSync(`${root}/support.txt`, `support repair ${supportWork}\n`); return worker(task) }
    fs.writeFileSync(`${root}/seed.txt`, `${candidate(task)}\n`)
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
    const value = JSON.parse(execute(binary, argv))
    if (argv.includes('start')) {
      runId = value.run.id
      if (!optionsInput.until_interrupted && !Object.hasOwn(optionsInput, 'attempt_limit') && value.run.attempt_limit !== 20) throw new Error('Binary default allowance was overridden')
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
    return JSON.stringify({ ...safe, reverify: null })
  }
  if (label === 'select ready work') {
    const context = state()
    const unfinished = context.checkpoints.find(checkpoint => !checkpoint.index_updated)
    const tasks = context.tasks.tasks.filter(task => task.state === 'ready')
    const chosen = unfinished ? tasks.find(task => task.id === unfinished.task)
      : context.stop_reason === 'blocker' ? tasks.find(task => !task.hypothesis) : tasks[0]
    activeTask = chosen?.id ?? null
    if (!chosen) return JSON.stringify({ task_id: null, kind: null, complexity: null, context: null, paths: [], reason: 'No eligible tasks.' })
    return JSON.stringify({ task_id: chosen.id, kind: chosen.hypothesis ? 'hypothesis' : 'supporting', complexity: chosen.complexity,
      context: compact(chosen.id), paths: chosen.hypothesis ? ['seed.txt'] : [], reason: 'Selected the eligible task from fresh graph.' })
  }
  if (label === 'propose scoped work') {
    const context = state()
    proposals.push(context.tasks.tasks.flatMap(task => (task.research_results ?? []).filter(record => record.lesson).map(record => record.lesson)))
    const tasks = generation++ === 0
      ? [supportDraft, hypothesisDraft('keep', 'advanced'), hypothesisDraft('failed')]
      : [hypothesisDraft('close')]
    return JSON.stringify({ bundle: { schema_version: 1, area: 'work', tasks }, reason: 'Use actual prior lessons in scoped work.' })
  }
  if (label === 'admit scoped proposal') {
    const bundle = JSON.parse(prompt.split('Proposal: ')[1])
    const request = prompt.match(/--request ([^ ]+)/)[1]
    const value = zdev(['research', 'run', 'work', 'admit', '--run', runId, '--request', request, '--from', temporary('proposal', bundle)])
    return JSON.stringify(value)
  }
  if (label === 'publish admitted tasks') { publish(state()); return JSON.stringify(safe) }
  if (['inspect candidate delta', 'inspect repaired delta', 'check acceptance findings', 'confirm shared repair', 'prepare accepted rebaseline'].includes(label)) return JSON.stringify(safe)
  if (label === 'classify hypothesis implementation blocker') return JSON.stringify({ action: 'continue', reason: 'A concrete declared-file correction remains.' })
  if (label === 'assess measured hypothesis') {
    const measured = JSON.parse(prompt.split('Evidence: ')[1])
    const attempts = measured.checkpoint.attempts.length
    let action = measured.assessment?.improved ? 'keep' : measured.assessment?.usable ? 'discard' : attempts < measured.checkpoint.settings.attempt_limit ? 'rerun' : 'failed-to-run'
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
    return JSON.stringify(value)
  }
  // Calls from the composed ordinary body keep its command and completion contracts.
  if (/select ready task|capture verification snapshot|refresh before rework|refresh after implementation blocker/.test(options.label)) return JSON.stringify(compact(activeTask))
  if (options.label.includes('confirm verifier') || options.label.includes('confirm findings snapshot')) {
    const snapshot = prompt.match(/--compare (W[0-9a-f]{16})/)[1]
    return JSON.stringify(zdev(['work-context', 'work', '--compare', snapshot]))
  }
  if (options.label.includes('complete and commit')) {
    const snapshot = prompt.match(/--compare (W[0-9a-f]{16})/)[1]
    if (!zdev(['work-context', 'work', '--compare', snapshot]).equal) throw new Error('Ordinary completion snapshot changed')
    zdev(['task', 'done', 'work', activeTask, '--summary', 'Completed supporting repair.', '--validation', 'Independent verification passed.'])
    const path = zdev(['task', 'show', 'work', activeTask]).path
    git(['add', '--', 'support.txt', path, '.zdev/work/TASKS.md']); git(['diff', '--cached'])
    zdev(['commit', '-m', 'Complete supporting repair'])
    return `PASS zdev-implement work ${activeTask}\n\nArea: work\nTask: ${activeTask}\nSummary: complete\nChanged files: support.txt\nValidation: passed\nVerifier evidence: independent review\nCommit ID: ${git(['rev-parse', 'HEAD'])}`
  }
  throw new Error(`Unexpected coordination ${options.label}`)
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
  const events = []
  const sources = new Map()
  let releaseSlow = null
  let releaseThird = null
  let overlapped = false
  let refill = false
  let measurementActive = 0
  let slowCompleted = false
  let createdAfterKeep = false
  let generated = false
  let rollingGeneration = 0
  const initialHead = git(['rev-parse', 'HEAD'])
  const taskPaths = task => task === 'work-001' || taskNumber(task) > 3 ? ['seed.txt']
    : task === 'work-002' ? ['delete.txt'] : ['mode.txt']
  const readySelection = prompt => {
    const excluded = JSON.parse(prompt.match(/Exclude these already assigned or unfinished tasks: (\[[^\]]*\])\./)[1])
    const explicit = prompt.match(/Select only unfinished checkpoint task (work-\d+)/)?.[1]
    const current = state()
    const eligible = current.tasks.tasks.filter(task => task.state === 'ready' && !excluded.includes(task.id))
    const chosen = explicit ? eligible.find(task => task.id === explicit) : current.stop_reason === 'blocker'
      ? eligible.find(task => !task.hypothesis) : eligible[0]
    activeTask = chosen?.id ?? null
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
        try { value = await pending } finally { measurementActive-- }
        if (scenario === 'rolling-record-only' && !injected) {
          injected = true
          fs.appendFileSync(`${root}/.zdev/work/brief.md`, '\nClarified the unchanged research objective.\n')
          git(['commit', '--only', '-m', 'Record-only checkpoint revision', '.zdev/work/brief.md'])
          throw new Error('Interrupted after a completed measurement')
        }
        return JSON.stringify(value)
      }
      if (argv.includes('reconcile')) events.push(`reconcile:${argv[3]}`)
      const value = JSON.parse(execute(binary, argv))
      if (argv.includes('start')) runId = value.run.id
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
      if (options.label.endsWith('check findings')) {
        if (!checkpoint) {
          if (options.label.startsWith('zdev research ')) throw new Error('Supporting findings were checked twice')
          supportFindingChecks++
          return verifier('rework')
        }
        return JSON.stringify(safe)
      }
      if (role === 'planner') return JSON.stringify(planner)
      if (role === 'verifier') {
        events.push(`verify:${task}`)
        if (checkpoint) {
          if (!checkpoint.isolated.integrated || !prompt.includes(checkpoint.verification_snapshot)) throw new Error('Verifier did not inspect fresh integrated destination')
        }
        if (!checkpoint && scenario === 'rolling' && supportReviews++ === 0) return verifier('rework')
        return verifier()
      }
      if (!checkpoint) { supportWork++; fs.writeFileSync(`${root}/support.txt`, 'supported\n'); return worker(task) }
      const assigned = checkpoint.isolated.source_root
      if (!prompt.includes(assigned) || !prompt.includes(checkpoint.isolated.brief) || !prompt.includes(checkpoint.isolated.task_record)) throw new Error('Source dispatch omitted absolute source/authoritative record paths')
      events.push(`source:${task}`)
      if (scenario === 'rolling' && task === 'work-002') await new Promise(resolve => { releaseSlow = resolve })
      if (scenario === 'rolling' && task === 'work-003') {
        if (!slowCompleted) refill = true
        await new Promise(resolve => { releaseThird = resolve })
      }
      const path = taskPaths(task)[0]
      fs.writeFileSync(`${assigned}/${path}`, taskNumber(task) > 3 ? '' : task === 'work-001' ? 'keep\n' : 'candidate\n')
      if (task === 'work-002') slowCompleted = true
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
      publish(state())
      for (const checkpoint of state().checkpoints.filter(checkpoint => !checkpoint.index_updated)) {
        let value = zdev(['research', 'experiment', 'work', checkpoint.task, 'recover'])
        if (!value.checkpoint.isolated.worker_stopped) zdev(['research', 'experiment', 'work', checkpoint.task, 'return', '--worker-stopped'])
      }
      for (const [request, claim] of Object.entries(state().run.reservations)) if (!claim.settled && !claim.bound) zdev(['research', 'run', 'work', 'settle', '--run', runId, '--request', request])
      const metadata = '.zdev/work/area.toml'
      if (git(['status', '--porcelain', '--', metadata])) { git(['add', metadata]); zdev(['commit', '-m', 'Publish actual rolling baseline']) }
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
    if (label === 'prepare accepted rebaseline' || label === 'confirm shared repair') return JSON.stringify(safe)
    return agent(prompt, options)
  }
  const input = { ...optionsInput, execution: 'rolling', until_interrupted: true, target: 1, worker_limit: 2, measurement_limit: 1, cleanup: false }
  let first = JSON.parse(await run(input, rollingAgent))
  if (scenario === 'rolling-record-only') {
    if (first.verdict !== 'blocker') throw new Error('Measurement interruption was not preserved')
    first = JSON.parse(await run({ area: 'work', execution: 'rolling', authorization: 'resume',
      invocation_id: `probe-${scenario}-resume`, run_id: first.run_id,
      frozen_profiles: first.frozen_profiles, retained_plans: first.retained_plans,
      retained_candidates: first.retained_candidates }, rollingAgent))
  }
  const final = state()
  if (first.stop_reason !== 'target') throw new Error(`Rolling run did not reach measured target: ${JSON.stringify({ first, events })}`)
  if (!generated || !createdAfterKeep || !final.run.rebaseline?.published || !workers.some(item => item.task === 'work-004' && item.role === 'planner')) throw new Error('Rolling mixed generation/current HEAD/support rebaseline was omitted')
  if (scenario === 'rolling' && (!refill || !overlapped || events.indexOf('returned:work-003') > events.indexOf('returned:work-002')
    || events.indexOf('measure:work-001') > events.indexOf('verify:work-001'))) throw new Error(`Missing real continuous refill/measurement overlap: ${JSON.stringify(events)}`)
  if (scenario === 'rolling' && (supportWork !== 2 || supportReviews !== 2 || supportFindingChecks !== 1)) throw new Error('Supporting correction did not use exactly one findings check')
  if (scenario === 'rolling-record-only' && (events.filter(event => event === 'measure:work-001').length !== 1 || !events.includes('reconcile:work-001'))) throw new Error('Record-only refresh relaunched a measurement or omitted public reconcile')
  if (workers.some(item => item.model !== first.frozen_profiles[item.role].value.model || item.effort !== first.frozen_profiles[item.role].value.effort)) throw new Error('Rolling role changed frozen concrete model/effort')
  for (const checkpoint of final.checkpoints.filter(item => item.index_updated)) {
    for (const attempt of checkpoint.attempts) if (git(['rev-parse', attempt.retained_ref]) !== attempt.candidate) throw new Error('Rolling transport/history was lost')
  }
  // Pipe writes are asynchronous on macOS; flush before exiting so the evidence is not truncated.
  const evidence = `${JSON.stringify({ report: first, consumed: final.consumed, workers, events, refill, overlapped })}\n`
  await new Promise(resolve => process.stdout.write(evidence, resolve))
  process.exit(0)
}

const first = JSON.parse(await run(optionsInput, agent))
const final = state()
if (scenario === 'mixed') {
  const decisions = final.checkpoints.map(checkpoint => checkpoint.final_record?.decision)
  if (JSON.stringify(decisions) !== JSON.stringify(['rejected', 'accepted', 'failed_to_run', 'accepted'])) throw new Error(`Mixed decisions ${JSON.stringify(decisions)}: ${JSON.stringify(first)}`)
  if (first.stop_reason !== 'target' || fs.readFileSync(`${root}/seed.txt`, 'utf8') !== 'close\n') throw new Error('Mixed continuation did not retain target candidate')
  if (!proposals.at(-1).some(lesson => lesson.includes('failed-to-run')) || !proposals.at(-1).some(lesson => lesson.includes('discard'))) throw new Error('Next proposal lost durable failed/rejected lessons')
  if (workers.filter(worker => worker.task === 'work-002' && worker.role === 'planner').length !== 1 || supportWork !== 2 || supportReviews !== 2 || supportFindingChecks !== 1) throw new Error('Ordinary advanced plan/rework/verification changed')
  if (workers.some(worker => worker.task === 'work-001' && worker.role === 'verifier')) throw new Error('Discard launched an unnecessary verifier')
}
for (const checkpoint of final.checkpoints.filter(checkpoint => checkpoint.index_updated)) {
  if (!checkpoint.commit || git(['rev-parse', checkpoint.commit]) !== checkpoint.commit) throw new Error('Managed result omitted actual commit')
  for (const attempt of checkpoint.attempts) if (git(['rev-parse', attempt.retained_ref]) !== attempt.candidate) throw new Error('Candidate history was not retained')
}
console.log(JSON.stringify({ report: first, consumed: final.consumed, workers, proposals, correctness_runs: fs.existsSync(`${root}/.git/correctness-runs`) ? fs.readFileSync(`${root}/.git/correctness-runs`, 'utf8') : '', evaluator_runs: fs.existsSync(`${root}/.git/evaluator-runs`) ? fs.readFileSync(`${root}/.git/evaluator-runs`, 'utf8') : '' }))

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
    const task = prompt.match(/complete task (work-\d+)/)?.[1] ?? activeTask
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

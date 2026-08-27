export const meta = {
  name: '__ZDEV_WORKFLOW_NAME__',
  description: 'Continue one zdev area through independently verified task commits',
}

const runOneTask = async (args, agent) => {
__ZDEV_ONE_TASK_BODY__
}

const normalizeLoopArgs = value => {
  if (Array.isArray(value)) return { area: value[0], focus: value.slice(1).join(' ') }
  if (typeof value === 'string') {
    const [area, ...rest] = value.trim().split(/\s+/)
    return { area, focus: rest.join(' ').replace(/^--focus(?:=|\s+)?/, '') }
  }
  if (value && typeof value === 'object') {
    return { area: value.area, focus: value.focus ?? value.intent ?? '' }
  }
  return { area: '', focus: '' }
}
const loopInput = normalizeLoopArgs(args)
const loopArea = String(loopInput.area ?? '').trim()
const loopFocus = String(loopInput.focus ?? '').trim()
const loopField = (text, name) => {
  const lines = text.split('\n')
  const matches = lines.flatMap((line, index) =>
    line.startsWith(`${name}: `) || line.trimEnd() === `${name}:` ? [index] : [])
  if (matches.length !== 1) return null
  const index = matches[0]
  const inline = lines[index].slice(name.length + 1).trim()
  if (inline) return inline
  const values = []
  for (const line of lines.slice(index + 1)) {
    if (!line.trim() || /^[A-Z][A-Za-z ]*:(?: |$)/.test(line)) break
    values.push(line.trim().replace(/^[-*]\s*/, ''))
  }
  return values.length > 0 ? values.join(', ') : null
}
const loopHasExactLine = (text, expected) => {
  const results = text.split('\n').filter(line =>
    /^(?:PASS|BLOCKER) zdev-implement /.test(line.trim()))
  return results.length === 1 && results[0].trim() === expected
}
const plainCommit = value => /^`[0-9a-f]{40}`$/.test(value ?? '') ? value.slice(1, -1) : value
const loopAdvisory = 'stale effective-base link; managed rebase remains optional.'
const completedTasks = []
const commits = []
let sawAdvisory = false
let latestCompletedTask = null
let latestCommit = null

const loopJson = raw => {
  if (raw && !Array.isArray(raw) && typeof raw === 'object') return raw
  if (typeof raw !== 'string') return null
  let start = -1
  let depth = 0
  let inString = false
  const values = []
  for (let index = 0; index < raw.length; index += 1) {
    const character = raw[index]
    if (inString) {
      if (character === '\\') index += 1
      else if (character === '"') inString = false
    } else if (character === '"' && depth > 0) {
      inString = true
    } else if (character === '{') {
      if (depth === 0) start = index
      depth += 1
    } else if (character === '}' && depth > 0) {
      depth -= 1
      if (depth === 0) {
        try { values.push(JSON.parse(raw.slice(start, index + 1))) } catch {}
      }
    }
  }
  return depth === 0 && values.length === 1 ? values[0] : null
}
const stateFrom = raw => {
  const context = loopJson(raw)
  return context?.area === loopArea
    && ['open', 'closed'].includes(context.lifecycle)
    && ['ready', 'empty', 'exhausted'].includes(context.queue)
    ? { lifecycle: context.lifecycle, queue: context.queue, taskId: context.task_id, head: context.head }
    : { lifecycle: 'unknown', queue: 'unknown', taskId: null, head: null }
}
const list = values => values.length === 0 ? 'none' : values.join(', ')
const advisoryLine = () => sawAdvisory ? `Advisory: ${loopAdvisory}\n` : ''
const pass = (state, reason) =>
  `PASS zdev-loop ${loopArea}\n\nArea: ${loopArea}\n${loopFocus ? `Focus: ${loopFocus}\n` : ''}Lifecycle: ${state.lifecycle}\nQueue: ${state.queue}\n${advisoryLine()}Tasks completed: ${list(completedTasks)}\nCommits: ${list(commits)}\nStop reason: ${reason}`
const block = (state, task, stage, reason, preserved) =>
  `BLOCKER zdev-loop ${loopArea || 'unknown'}\n\nArea: ${loopArea || 'unknown'}\n${loopFocus ? `Focus: ${loopFocus}\n` : ''}Lifecycle: ${state.lifecycle}\nQueue: ${state.queue}\n${advisoryLine()}Tasks completed: ${list(completedTasks)}\nCommits: ${list(commits)}\nStop reason: blocked.\nCurrent task: ${task}\nFailed stage: ${stage}\nReason: ${reason}\nPreserved state: ${preserved}`

if (!/^[a-z0-9][a-z0-9-]*$/.test(loopArea)) {
  return block({ lifecycle: 'unknown', queue: 'unknown' }, 'none', 'input', 'a lowercase area is required.', 'no preflight or worker was started.')
}

const selectorSchema = {
  type: 'object', additionalProperties: false,
  required: ['task_id', 'ready', 'reason'],
  properties: {
    task_id: { anyOf: [{ type: 'null' }, { type: 'string', minLength: 1 }] },
    ready: { type: 'array', items: { type: 'string', minLength: 1 } },
    reason: { type: 'string', minLength: 1 },
  },
}
const selectFocusedTask = async () => {
  if (!loopFocus) return { taskId: null, ready: null }
  const raw = await agent(
    `Select the next task in zdev area ${loopArea} for this fuzzy focus: ${JSON.stringify(loopFocus)}. Run zdev tasks list ${loopArea} --format json once, then run zdev task show ${loopArea} <task-id> --format json for every task whose state is "ready". Put the complete ready frontier in ready. Choose the task that best advances the focus using each full task; the focus is guidance, not an exact filter. Return task_id null only when ready is empty. Keep files unchanged.`,
    { label: `zdev ${loopArea}: choose from ready frontier`, schema: selectorSchema },
  )
  const selected = loopJson(raw)
  if (!selected || !Array.isArray(selected.ready)
    || !selected.ready.every(id => typeof id === 'string')
    || typeof selected.reason !== 'string') return null
  if (selected.ready.length === 0) return selected.task_id === null
    ? { taskId: null, ready: [] } : null
  return typeof selected.task_id === 'string' && selected.ready.includes(selected.task_id)
    ? { taskId: selected.task_id, ready: selected.ready } : null
}
const freshContext = async selected => agent(
  `Act only as the area-loop read-only preflight for area ${loopArea}. Run zdev work-context ${loopArea}${selected ? ` --task ${selected}` : ''} --store --format json exactly once and return its JSON stdout. Do not show the snapshot. Keep files and Git state unchanged.`,
  { label: `zdev ${loopArea}: ${selected ? `prepare ${selected}` : 'select next task'}`, model: 'haiku' },
)

while (true) {
  const selection = await selectFocusedTask()
  if (loopFocus && !selection) {
    return block({ lifecycle: 'unknown', queue: 'unknown' }, 'none', 'selection', 'the focus selector returned an invalid frontier selection.', 'no task worker was started.')
  }
  const chosenTask = selection?.taskId ?? null
  const contextRaw = (await freshContext(chosenTask))?.trim() ?? ''
  const state = stateFrom(contextRaw)
  if (loopFocus && selection.ready?.length === 0 && state.queue === 'ready') {
    return block(state, state.taskId ?? 'none', 'selection', 'the selector reported an empty frontier but zdev found ready work.', 'no task worker was started.')
  }
  if (state.lifecycle === 'unknown' || (chosenTask && state.taskId !== chosenTask)) {
    return block(state, chosenTask ?? 'none', 'preflight', 'work-context did not confirm the selected ready task.', 'no task worker was started.')
  }
  if (latestCompletedTask && state.lifecycle === 'open'
    && (state.head !== latestCommit || state.taskId === latestCompletedTask)) {
    return block(state, latestCompletedTask, 'continuation refresh', 'fresh work-context did not confirm the committed task advanced.', 'the committed pair remains recorded and no next worker was started.')
  }
  let supplied = false
  const result = (await runOneTask({ area: loopArea, task_id: chosenTask }, async (prompt, options) => {
    if (!supplied && options?.label === `zdev ${loopArea}: select ready task`) {
      supplied = true
      return contextRaw
    }
    return agent(prompt, options)
  }))?.trim() ?? ''
  const task = loopField(result, 'Task')
  if (loopField(result, 'Advisory') === loopAdvisory) sawAdvisory = true

  if (loopHasExactLine(result, `PASS zdev-implement ${loopArea} none`) && task === 'none') {
    return pass(state, `no ready work; ${state.lifecycle}/${state.queue}.`)
  }
  if (task && task !== 'none' && loopHasExactLine(result, `PASS zdev-implement ${loopArea} ${task}`)) {
    const commit = plainCommit(loopField(result, 'Commit ID'))
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
  if (task && loopHasExactLine(result, `BLOCKER zdev-implement ${loopArea} ${task}`)) {
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

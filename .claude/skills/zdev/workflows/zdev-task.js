export const meta = {
  name: 'zdev-task',
  description: 'Implement and independently verify one selected zdev task',
}

const input = args ?? {}
const target = input.area ? `area ${input.area}` : 'the selected area'
const preflight = await agent(
  `Read-only preflight for ${target}. Run zd status and zd next as JSON. Require the four area gates: recorded branch checked out, effective-base link fresh, anchor valid, and base finalization complete. Read the brief, selected task, and rendered repository guidance. Record the three-part Git baseline: status including untracked files, the staged diff, and the unstaged diff. Identify task-owned paths and return BLOCKER for ambiguous overlap. Begin with READY and the exact task context plus baseline, or BLOCKER and the concrete reason. Do not edit files or zdev state.`,
  { label: 'zdev preflight' },
)

const preflightResult = preflight?.trim()
if (!preflightResult || !/^READY\b/.test(preflightResult)) {
  return preflightResult && /^BLOCKER\b/.test(preflightResult)
    ? preflightResult
    : `BLOCKER: preflight did not return a valid READY context.\n\n${preflightResult ?? ''}`
}

let implementation = await agent(
  `Act as zdev-implementer for this context:\n${preflight}\nImplement in the current checkout. Respect the recorded baseline and task-owned paths; stop on ambiguous overlap. Follow the brief's testing level and established repository patterns. You may edit source and tests and run validation. Do not edit .zd, run zd task done, change task lifecycle state, commit, open a pull request, create run state, or delegate.`,
  { label: 'zdev implementation' },
)

const review = async implementationContext => {
  const result = await agent(
    `Act as a fresh read-only zdev-verifier. Treat the implementer summary as context only, never as evidence. Compare the recorded three-part baseline with the actual checkout: git status including untracked files, the staged diff, the unstaged diff, and relevant files. Ignore an intervening commit only if its complete diff adds new .zd/<area>/tasks/*.md files, regenerates .zd/<area>/TASKS.md, and changes no other path; otherwise return BLOCKER. Record the same complete state before and after validation; report writes without restoring or discarding them. Perform separate Spec and Standards passes and honor the brief's testing level.\n\nReturn a first line beginning with exactly PASS, REWORK, or BLOCKER. PASS requires both passes and all required validation. Use REWORK only for concrete task-owned defects or task-owned validation writes. Use BLOCKER for ambiguous ownership, unavailable required evidence or validation, or user-owned design, scope, or testing decisions. Only optional checks may be residual limitations. Include classified findings and locations. Do not edit files or zdev state, complete the task, commit, or open a pull request.\n\nTask context:\n${preflight}\n\nImplementer summary (context only):\n${implementationContext}`,
    { label: 'zdev evidence verification' },
  )
  const verdict = result?.trim()
  return verdict && /^(PASS|REWORK|BLOCKER)\b/.test(verdict)
    ? verdict
    : `BLOCKER: verification returned no valid PASS, REWORK, or BLOCKER verdict.\n\n${verdict ?? ''}`
}

let verdict = await review(implementation)
while (/^REWORK\b/.test(verdict)) {
  const rework = await agent(
    `Act as zdev-implementer and correct every concrete task-owned finding below. Inspect the current checkout, preserve the recorded baseline and task-owned path boundary, and keep the brief's testing level as a scope boundary. Stop for ambiguous ownership or a user-owned decision. You may edit source and tests and run validation. Never edit .zd, run zd task done, change task lifecycle state, commit, or open a pull request.\n\nTask, brief, and rendered-guidance context:\n${preflight}\n\nFindings:\n${verdict}`,
    { label: 'zdev rework' },
  )
  if (!rework) {
    return 'BLOCKER: implementation rework returned no result.'
  }
  implementation = `${implementation}\n\nRework:\n${rework}`
  verdict = await review(implementation)
}

return verdict

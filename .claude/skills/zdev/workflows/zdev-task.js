export const meta = {
  name: 'zdev-task',
  description: 'Implement and independently verify one selected zdev task',
}

const input = args ?? {}
const target = input.area ? `area ${input.area}` : 'the selected area'
const preflight = await agent(
  `Prepare ${target} for implementation without changing files. Run zdev status and zdev next as JSON. Confirm the recorded branch, effective base, anchor, and base finalization. Read the brief, selected task, and repository guidance. Record status with untracked files, the staged diff, and the unstaged diff. Identify task-owned paths. Return READY with the task context and baseline, or BLOCKER with the concrete reason.`,
  { label: 'zdev preflight' },
)

const preflightResult = preflight?.trim()
if (!preflightResult || !/^READY\b/.test(preflightResult)) {
  return preflightResult && /^BLOCKER\b/.test(preflightResult)
    ? preflightResult
    : `BLOCKER: preflight did not return a valid READY context.\n\n${preflightResult ?? ''}`
}

let implementation = await agent(
  `Implement the selected task in the current checkout. Use this context:\n${preflight}\nChange only task-owned source and test paths. Follow the brief's testing level, reuse repository patterns, and run the listed validation. Leave .zdev, task lifecycle, commits, pull requests, and delegation to the coordinating agent. Return changed files, validation results, and blockers.`,
  { label: 'zdev implementation' },
)

const review = async implementationContext => {
  const result = await agent(
    `Verify the selected task from the current checkout. Use the implementer summary only to locate evidence. Compare the recorded baseline with status, staged and unstaged diffs, untracked files, and relevant source. Attribute every change. Check every task requirement, inspect touched code for defects and regressions, run required validation, and compare Git state before and after validation. Report files written by checks.\n\nBegin with PASS, REWORK, or BLOCKER. Use PASS when all required checks succeed, REWORK for concrete task-owned defects, and BLOCKER when ownership, evidence, validation, or a user decision prevents a verdict. Return findings with locations. Make no intentional edits; leave zdev state, task completion, commits, and pull requests to the coordinating agent.\n\nTask context:\n${preflight}\n\nImplementer summary:\n${implementationContext}`,
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
    `Correct every task-owned finding below. Inspect the current checkout, change only task-owned source and test paths, follow the brief's testing level, and run the relevant validation. Leave .zdev, task lifecycle, commits, and pull requests to the coordinating agent.\n\nTask context:\n${preflight}\n\nFindings:\n${verdict}`,
    { label: 'zdev rework' },
  )
  if (!rework) {
    return 'BLOCKER: implementation rework returned no result.'
  }
  implementation = `${implementation}\n\nRework:\n${rework}`
  verdict = await review(implementation)
}

return verdict

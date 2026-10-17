# Rebaseline rejects unrelated untracked result captures

Reported 2026-10-17 from Gecode's `feature/portfolio` worktree during
`autoport-027`. Local zdev 2.0.0 rejects an actual managed rebaseline with:

> Actual rebaseline requires clean accepted code; retain or restore unfinished candidates first

The tracked source and index are clean, the candidate remains the accepted
Random profile, and there are no unfinished experiment checkpoints. About
52,000 existing files under `results/` are saved benchmark captures, not source
edits. The rejection happens before any evaluator launch or resource charge.

Reproduce in a research fixture with clean accepted code: create an untracked
`results/capture.json`, then run `research run AREA rebaseline --run RUN
--request REQUEST`. The blanket `git ls-files --others` guard in
`src/research/run.rs` rejects every non-`.zdev` untracked path.

Allow a caller to explicitly identify generated output with repeatable
`--untracked-output PATH`. Keep the default refusal, every tracked-change guard,
and refusal of untracked files outside those exact paths or their descendants.
Do not delete, move, stage or globally ignore captures. Do not infer that a
directory named `results` is always safe. A named output boundary is a caller
assertion, not proof that its contents cannot influence the evaluator.

Validation must exercise real managed rebaseline, capture preservation,
rejection without the option, dirty tracked code and other untracked code.
After installing the fix, resolve the existing Gecode run's blocker, preserve
its original deadline and costs, bind a separately admitted live evaluator and
measure its baseline before executing the source-declared-group hypothesis.

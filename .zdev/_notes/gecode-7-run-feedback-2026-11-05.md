# Feedback from the Gecode 7 run, 2026-11-05

- `zdev commit` updates dependent area indexes after staging. Completing
  scoped-enums-004 rewrote .zdev/source-cleanup/TASKS.md, which stayed modified
  after the commit and needed an amend. The dependent index should be staged
  with the commit, or `task done` should report the file to stage.
- `zdev cleanup squash` needs a clean worktree and commits to the checked-out
  branch. A user wanted the cleanup only on the remote PR branch, kept out of
  local history and rebuilt on each update. Worth a mode that builds the commit
  from a named ref without touching the worktree, for example `--from <ref>
  --print-sha`, so the push stays a separate step.
- `zdev task done <area> 003` fails with "Unknown task"; only the full id
  works. Accepting the numeric suffix within a named area would be friendlier.
- A T3 server restart killed a worker's background validation and left the
  worker's thread completed with no wake. Long workers should be told to detach
  long steps and record exit statuses; the coordinator had to notice and resume.

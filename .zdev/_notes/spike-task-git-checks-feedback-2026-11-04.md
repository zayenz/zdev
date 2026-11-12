# Spike task git checks (field feedback, 2026-11-04)

From a Gecode spike task (boost-007) run with T3 orchestration.

- The plan's final check said `git status --short` must be empty, while the
  task allowed one tracked edit (a brief index line). The implementer had to
  report a "deviation". Plans could derive the expected status from the
  allowed-changes list instead of asserting an empty tree.
- When `.zdev` is gitignored or in `.git/info/exclude`, a plain `git add` of
  already tracked `.zdev` files still fails if the same command also names an
  ignored new file. `zdev commit` guidance could say to use `git add -f` for
  every `.zdev` path, or `zdev task done` could print the exact staging
  command.
- Long-running implementers send several "completed" progress turns before
  the final JSON. T3 shows the task as completed after each one, which makes
  it look finished. A clearer "final result pending" state would avoid
  re-checks.

# Repository instructions

## Git history

Keep `main` history linear. Merge commits are not allowed on `main`.
Rebase feature branches onto the current `main`, then integrate them with
`git merge --ff-only`. If a fast-forward fails, rebase again; never create a
merge commit to resolve diverged history.

Before finishing an integration, verify that `git rev-list --min-parents=2 main`
produces no output.

# Recover interrupted work

Use recover when a session stopped during implementation, verification,
completion, or a managed rebase. It resumes from the actual files and Git state.

## Examples

Paste either request into your coding-agent chat:

```text
Use zdev to recover interrupted work in area greeting and resume the task.
Keep my existing changes and inspect the previous verification result before
deciding what remains.
```

```text
Use zdev to recover the interrupted managed rebase for area greeting.
Inspect the Git state and explain any conflict that needs a decision.
```

## What happens

The agent reads the task records, checkout, and available handoff. It works
out which changes belong to the task, restores the known review stage, and
resumes the unfinished step. If verification passed and only the final commit
failed, recovery can finish that checkpoint without starting another implementation.

An interruption does not reset the correction allowance. If the previous run
exhausted its correction pass, the agent reports the remaining findings and
the continuation decision needed. If ownership of a local change cannot be
established, it preserves the change and asks.

For an interrupted parallel batch, name it explicitly:

```text
Use zdev to recover and resume the interrupted parallel batch in greeting.
```

The result names completed commits, remaining work, preserved worktrees, and
any blocker. A managed rebase uses `zdev area rebase` and its continue or abort
operations; recovery inspects the state before choosing one. Explicit trunk
areas do not need a managed rebase.

See [workflow recovery](../workflow.md#recovery) and
[managed rebases](../workflow.md#managed-rebases). Research runs have their own
recorded measurements and limits; use the resume request in [research](research.md).

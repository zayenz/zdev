# Agent commands

Use these guides when you know what you want to do but are unsure what to ask
the coding agent. Each guide opens with example requests you can paste into chat and
explains where that interaction stops.

Zdev has two interfaces. The installed skill interprets chat requests such as
`Use zdev to implement the next task in greeting.` The `zdev` binary runs shell
commands such as `zdev status greeting`. You can run those commands yourself or
ask the agent to run them and explain the result. Agent workflows also coordinate
planning, implementation, and review. For example, running `zdev goal greeting`
prints work context; asking the agent to loop through greeting starts execution.

## Examples

Paste requests like these into your coding-agent chat:

```text
Use zdev to explore area greeting: add --name to hello.py so that
python3 hello.py --name Ada prints Hello, Ada! Keep the current default greeting.
```

```text
Use zdev to implement the next task in greeting.
```

For terminal invocations, use the [CLI reference](../cli/index.md).

## Start with a small objective

The examples use an area named `greeting` to add `--name` to `hello.py`. An area
keeps its objective, decisions, and tasks together under `.zdev/greeting/`.
Start with [setup](setup.md) if this is your first repository.

[Explore](explore.md) produces the brief. [Create tasks](tasks.md) turns the
agreed brief into work you can review. [Implement](implement.md) completes and
commits one task. You can stop after any of these steps and continue later.

## Pick the interaction

| What you need | Request guide | Result |
| --- | --- | --- |
| Save an idea for later | [Notes](notes.md) | A note or folder under `.zdev/_notes/` |
| Define the outcome and scope | [Explore](explore.md) | An area brief |
| Challenge an existing brief | [Discuss](discuss.md) | Clearer decisions in the brief |
| Find worthwhile changes | [Improve](improve.md) | Prioritized candidate work |
| Review a change or code boundary | [Review](review.md) | Checked feedback; availability depends on the installed integration |
| Inspect a boundary for defects | [Audit](audit.md) | Independently checked findings |
| Answer one uncertain question | [Investigate](investigate.md) | Evidence and a conclusion |
| Split agreed work | [Create tasks](tasks.md) | A reviewed task bundle and imported tasks |
| Finish one ready task | [Implement](implement.md) | A verified commit |
| Run compatible tasks together | [Parallel](parallel.md) | Separate verified task commits |
| Think through one task first | [Plan](plan.md) | A plan in the conversation |
| Check the current ready task | [Verify](verify.md) | A task verification result |
| Continue through approved work | [Loop or goal](loop.md) | One verified commit per completed task |
| Resume interrupted work | [Recover](recover.md) | Resumed work or a precise recovery decision |
| Change settings | [Configure](configure.md) | The requested setting |
| Run measured experiments | [Research](research.md) | Measurements, decisions, and retained lessons |
| Initialize or install | [Setup](setup.md) | A repository record and harness integration |

Name the area in a request when several areas exist. Zdev asks when that choice
is ambiguous. You can combine steps explicitly, for example:

```text
Use zdev to create and import tasks for greeting without asking me to approve
the bundle, then implement the first task.
```

That delegates bundle approval and authorizes one implementation. A request to
create tasks alone stops after import. Read the [getting started guide](../getting-started.md)
for a complete first run and the [workflow reference](../workflow.md) for Git
handling, verification, and completion rules.

[Review and verification](../review-and-verification.md) explains the two scopes.
Verification includes code review against the [coding guidelines](../coding-guidelines.md)
as well as the task's required validation.

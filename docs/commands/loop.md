# Continue through an area

Use loop when you want zdev to work through a named area's approved tasks,
with independent review and verification and a commit for each completed task. "Goal"
means the same thing in an active zdev chat.

## Examples

Paste either request into your coding-agent chat:

```text
Use zdev to loop through area greeting until its approved tasks are finished
or a blocker needs my input.
```

```text
Use zdev to set a goal for area greeting: finish its approved tasks,
prioritizing command-line usability.
```

## What happens

After each commit, the agent reads the current task queue again. Ready tasks
that can finish unattended come first, followed by priority and task number.
You can add a focus such as "prioritize command-line usability". The agent
uses that as guidance among eligible tasks, not as an exact text filter.

The loop reports each selected task and verified commit. It stops when the
queue is empty or exhausted, the area is already closed, or progress requires
a decision or resolution of a blocker. Finishing the queue leaves an open area
open. The ordinary loop does not invent more work when it runs out of tasks.

Continuation depends on the harness. When native continuation cannot be
started safely, the supported fallback completes at most one task and returns
control. A `CONTINUE zdev-loop greeting` result means more safe work remains;
it does not mean a background process is running. `PASS` reports the completed
stop condition, and `BLOCKER` explains what stopped progress.

The shell command `zdev goal greeting` only prints context. Use a chat request
to start the loop. For parallel continuation, explicitly ask to use
[parallel tasks](parallel.md) throughout the run. For hypothesis generation,
use [research](research.md). The [area-loop reference](../area-loop.md) details
harness support and stopping behavior.

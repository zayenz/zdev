---
description: Implement, independently verify, complete, and commit one ready zdev task
---

{{task_workflow_contract}}

Use `$ARGUMENTS` as the area. The current Oh My Pi session is the coordinator.
After preflight, invoke the blocking `zdev-implementer` task agent with the
unchanged work-context and baseline. Use a fresh blocking `zdev-verifier` agent for
every full verification. `hub` may return concrete rework to the implementer;
otherwise start a replacement with the complete context.

{{repository_guidance}}

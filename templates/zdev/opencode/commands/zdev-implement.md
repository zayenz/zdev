---
description: Implement, independently verify, complete, and commit one ready zdev task
---

{{task_workflow_contract}}

Use `$ARGUMENTS` as the area. The primary agent is the coordinator. After
preflight, invoke the `zdev-implementer` subagent with the unchanged goal JSON
and baseline. Use a new `zdev-verifier` subagent for every full verification.
Resume the implementer only for concrete rework when safe; otherwise start a
replacement implementer with the complete context.

{{repository_guidance}}

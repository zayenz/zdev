---
description: Implement, independently verify, complete, and commit one ready zdev task
---

{{task_workflow_contract}}

Use `$ARGUMENTS` as the area. The current Pi session is the coordinator. After
preflight, call `zdev_subagent` with role `implementer`, the unchanged work-context,
and baseline. Use a fresh call with role `verifier` for every full verification.
Pi children have no resumed session, so each rework uses a replacement
implementer with the complete current context and findings.

{{repository_guidance}}

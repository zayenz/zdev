---
description: Independently verify the explicit current ready zdev task
---

{{verify_workflow_contract}}

Parse `$ARGUMENTS` as `<area> <task-id>`. The current Pi session performs
preflight and exact ID matching before one fresh `zdev_subagent` call with role
`verifier`. Store and validate the snapshot immediately before dispatch,
require the four-field semantic response, compare the snapshot afterward, and
return the coordinator-constructed strict envelope without lifecycle or Git
mutation.

{{repository_guidance}}

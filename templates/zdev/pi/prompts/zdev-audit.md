---
description: Review a codebase and return checked findings
---

Review $ARGUMENTS without changing the repository. Use one `zdev_subagent`
verifier to open every cited location and remove weak or duplicate findings.
Use additional focused agents only for a large boundary or an explicit swarm
request. Rank the remaining findings by impact and return their locations,
confidence, and recommended next action. The coordinating agent decides whether
to record any work.

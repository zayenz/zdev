---
description: Audit a bounded codebase and return independently checked candidate work
---

Audit $ARGUMENTS without editing files. Perform one bounded primary review,
then use one fresh `zdev_subagent` verifier call to inspect the cited evidence
and reject weak or duplicate findings. Reserve additional focused lens calls
for a substantial boundary or an explicit swarm request; do not fan out by
default. Return ranked, evidence-backed candidates for human selection. Do not
change `.zd`, create or complete tasks, commit, or open a pull request. The
primary conversation keeps all authority to select and record work.

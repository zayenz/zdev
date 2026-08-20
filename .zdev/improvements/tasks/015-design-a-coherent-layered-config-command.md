+++
schema_version = 1
id = "improvements-015"
key = "config-command-research"
area = "improvements"
status = "open"
blocked_by = ["improvements-011"]
+++
# Design a coherent layered config command

## Outcome

A source-backed design defines a useful zdev config command that shows effective values and their origins, explains local overrides of global values, and supports setting and removing validated general and worker-profile configuration.

## Context

The current `zdev config` surface only sets the project trunk, while project state lives in `.zdev/config.toml` and the worker-profile research proposes editable harness and role overrides. Inspect the current configuration seams and compare them with Git's current config command, especially scoped reads, origin reporting, setting, and unsetting. Use Git as design evidence rather than a compatibility target. Write `docs/config-command.md`; this task settles the contract before runtime implementation.

## Boundaries

- Do not implement the runtime command, migrate files, or change existing configuration in this research task.
- Do not expose arbitrary TOML editing, store secrets, manage provider credentials, or copy Git's command grammar where a smaller zdev-specific surface is clearer.
- Keep durable project identity, record policy, trunk and area metadata distinct from user preferences unless the document gives a concrete reason to make a field globally overridable.
- Keep the configuration model small, hand-editable, deterministic, and strict; do not introduce a plugin-style schema or general configuration framework.

## Done when

- [ ] The document inventories every existing and proposed zdev setting, including worker profiles, and settles which values may be global, repository-local, or repository-only.
- [ ] It defines exact show, get, set, and unset behavior in stable human and JSON forms, including effective values, winning origins, shadowed global values, defaults, and sensitive-value handling.
- [ ] It settles global and local file locations, precedence, key names, atomic writes, validation, unknown or unsupported values, missing files, and fallback behavior.
- [ ] It records the useful Git config ideas adopted or rejected and explains the zdev-specific differences without claiming Git compatibility.
- [ ] It covers backward compatibility for existing `.zdev/config.toml` and the worker-profile contract, then ends with a narrow implementation seam and acceptance criteria requiring no further product decision.

## Validation

- Check current Git config behavior against official Git documentation and inspect every current zdev configuration read and write path.
- Run `git diff --check`.

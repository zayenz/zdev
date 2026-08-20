+++
schema_version = 1
id = "improvements-018"
key = "layered-config-inspection"
area = "improvements"
status = "open"
blocked_by = ["improvements-016"]
+++
# Expose layered configuration through show and get

## Outcome

Users can inspect effective or scoped zdev configuration with stable values, origins, and shadowed worker profiles.

## Context

Implement the read-only half of docs/config-command.md using the worker registry and resolver introduced by worker-profile-integrations. Extend ConfigCommand in src/lib.rs with show and get. Keep project record parsing in src/project.rs and place the fixed key registry, views, and global-path handling in the focused configuration module.

## Boundaries

- Implement show and get only; leave set and unset to the following task.
- Expose exactly the fifteen settled keys in fixed project, harness, and role order.
- Scoped reads return only stored values; effective reads validate and resolve every consulted layer.
- Do not expose area, slice, task, schema, invocation, credential, or arbitrary TOML keys.
- Use the existing JSON renderer and fixed value registry rather than a general configuration framework.

## Done when

- [ ] Effective show renders all fifteen keys with correct local, global, default, null, origin, and shadowed values in the documented human and JSON forms.
- [ ] Global and local show omit unstored keys and preserve fixed registry order.
- [ ] Get returns the same effective or scoped value contract for one key and fails cleanly for an absent scoped value.
- [ ] Global-path resolution ignores empty or relative candidates and reports the first absolute normalized origin.
- [ ] Malformed consulted files, unknown keys, unsupported scopes, and unavailable homes fail without output or mutation.
- [ ] Existing config trunk behavior and existing project records remain unchanged.

## Validation

- Run focused black-box tests for the complete effective fixture, one scoped view, one get, and one strict read failure.
- Run cargo test --locked --test lean.
- Run the repository's standard full validation from the area brief.

# Layered zdev configuration

> **Status: current behavior.** The `zdev config` command described here is
> implemented. Research and external behavior were checked on 2026-08-20;
> implementation-seam sections preserve the decision record that led to it.

## What is configuration

Zdev has three kinds of durable data. They should not be collapsed into one
generic settings store.

1. The project record in `.zdev/config.toml` identifies the zdev project and
   controls repository topology and record handling.
2. Area, slice, and task files describe the work. They are domain records, not
   preferences.
3. Worker profiles choose harness model controls. They are preferences: a user
   can have a useful default across repositories, and a repository can override
   it.

The distinction matters. A global trunk or area branch would let one user's
machine reinterpret shared task history. A global project name or record policy
could make cleanup act on the wrong storage contract. Worker selection has no
such effect on durable work identity, so layering it is useful.

Initialization creates the strict `.zdev/config.toml`. `zdev config trunk` and
project integration install read, modify, and atomically replace it. Root
discovery looks only for the file; initialization checks an existing record;
cleanup, status, checks, task selection, branch operations, and integration
guidance parse it. No command currently writes `default_area`. Area creation,
binding, parenting, and managed rebase separately write strict
`.zdev/<area>/area.toml` records. Worker preferences use the implemented strict
`.zdev/workers.toml` format described below.

## Supported keys and scopes

`local` means the initialized repository selected by discovery or `--root`.
`global` means the current user's zdev preferences. `repo-only` keys can be
read through the local or effective view, but `--global` cannot set them.
Repository-local does not mean clone-private: the chosen `personal`, `project`,
or `pull-request` record policy still determines whether `.zdev` is shared.

| CLI key | Value | Scope | Mutation |
| --- | --- | --- | --- |
| `project.name` | non-empty project name | repo-only | Set by `zdev init`; read-only afterward. |
| `project.record` | `personal`, `project`, or `pull-request` | repo-only | Set by `zdev init`; read-only afterward. |
| `project.trunk` | Git branch name, or absent | repo-only | Set or unset locally. |
| `project.default-area` | existing area tag, or absent | repo-only | Set or unset locally. |
| `project.guidance` | `auto`, `agents`, `zdev`, or a safe repository-relative Markdown path; absent means `auto` | repo-only | Set or unset locally. Project integration install may continue recording the source it used. |
| `worker.codex.routine-implementer` | worker profile | global and local | Set or unset. |
| `worker.codex.implementer` | worker profile | global and local | Set or unset. |
| `worker.codex.verifier` | worker profile | global and local | Set or unset. |
| `worker.codex.advanced-implementer` | worker profile | global and local | Set or unset. |
| `worker.claude.routine-implementer` | worker profile | global and local | Set or unset. |
| `worker.claude.implementer` | worker profile | global and local | Set or unset. |
| `worker.claude.verifier` | worker profile | global and local | Set or unset. |
| `worker.claude.advanced-implementer` | worker profile | global and local | Set or unset. |
| `worker.opencode.routine-implementer` | worker profile | global and local | Set or unset. |
| `worker.opencode.implementer` | worker profile | global and local | Set or unset. |
| `worker.opencode.verifier` | worker profile | global and local | Set or unset. |
| `worker.opencode.advanced-implementer` | worker profile | global and local | Set or unset. |
| `worker.pi.routine-implementer` | worker profile | global and local | Set or unset. |
| `worker.pi.implementer` | worker profile | global and local | Set or unset. |
| `worker.pi.verifier` | worker profile | global and local | Set or unset. |
| `worker.pi.advanced-implementer` | worker profile | global and local | Set or unset. |
| `worker.omp.routine-implementer` | worker profile | global and local | Set or unset. |
| `worker.omp.implementer` | worker profile | global and local | Set or unset. |
| `worker.omp.verifier` | worker profile | global and local | Set or unset. |
| `worker.omp.advanced-implementer` | worker profile | global and local | Set or unset. |

A worker profile is one atomic value. It is either `inherit`, or a non-empty
model plus one effort from `inherit`, `low`, `medium`, `high`, `xhigh`, or
`max`. Atomic profiles avoid an invalid intermediate file containing a model
without an effort. `effort = inherit` means set the model but omit the harness
effort control; a whole-profile `inherit` omits both controls.

The built-in worker defaults are the dated suggestions in
[Worker profiles](worker-profiles.md):

| Harness | Routine implementer | Implementer | Verifier | Advanced implementer |
| --- | --- | --- | --- | --- |
| Codex | `gpt-5.6-luna low` | `gpt-5.6-sol low` | `gpt-5.6-sol low` | `gpt-5.6-sol high` |
| Claude | `haiku low` | `claude-opus-5 low` | `claude-opus-5 low` | `claude-opus-5 high` |
| OpenCode | `openai/gpt-5.6-luna low` | `openai/gpt-5.6-sol low` | `anthropic/claude-opus-5 inherit` | `openai/gpt-5.6-sol high` |
| Pi | `openai/gpt-5.6-luna low` | `openai/gpt-5.6-sol low` | `anthropic/claude-opus-5 low` | `openai/gpt-5.6-sol high` |
| OMP | `openai/gpt-5.6-luna low` | `openai/gpt-5.6-sol low` | `anthropic/claude-opus-5 low` | `openai/gpt-5.6-sol high` |

The Oh My Pi adapter renders these two abstract fields into its native combined
representation. It must validate that the pair is expressible rather than
silently dropping either field.

Project identity and record policy have no default. An absent trunk has the
effective value `null` and means unbound. An absent default area has the
effective value `null` and leaves the existing unambiguous-area selection in
place. An absent guidance value has the effective value `auto`.

Area `tag`, `title`, `objective`, `branch`, `parent`, and `base_commit` remain
in `area.toml`. Slice and task identity, dependencies, status, outcomes, and
validation remain in their Markdown records. None is addressable by `zdev
config`; their existing domain commands own their validation and transitions.
`schema_version` is file-format metadata, not a configurable key. Command
arguments such as output format, repository root, integration scope,
destination, and force are per-invocation controls and are not persisted.

## Files and precedence

The repository files keep their current purposes:

- `.zdev/config.toml` is the strict project record.
- `.zdev/workers.toml` is the optional local worker-profile file.

The global worker path uses the first absolute base in this order:

1. An absolute, non-empty `XDG_CONFIG_HOME` gives
   `$XDG_CONFIG_HOME/zdev/workers.toml`.
2. Otherwise an absolute, non-empty `HOME` gives
   `$HOME/.config/zdev/workers.toml`.
3. Otherwise an absolute, non-empty `USERPROFILE` gives
   `$USERPROFILE/.config/zdev/workers.toml`.

An unset, empty, or relative value is unavailable and resolution continues to
the next variable. If none is absolute, the operation fails with `Cannot locate
global zdev configuration; set XDG_CONFIG_HOME, HOME, or USERPROFILE to an
absolute path` and changes nothing. Paths are lexically normalized, not
canonicalized, because the file may not exist yet. Reported global origins are
therefore always absolute. There is no zdev-specific path override.

Worker resolution is whole-profile, in this order:

1. a local `.zdev/workers.toml` profile;
2. the matching global profile;
3. the built-in profile.

An explicit `inherit` is a winning value. It does not fall through to the next
layer. Profiles are not merged field by field. This keeps an effort from one
scope from accidentally attaching to a model from another.

Project values come only from `.zdev/config.toml`, followed by the explicit
defaults above where applicable. Harness-native settings may later override or
reject rendered model controls. Zdev reports its effective input to integration
rendering, not the model that a provider ultimately runs.

Both worker files use this hand-editable TOML schema:

```toml
schema_version = 1

[codex.implementer]
model = "gpt-5.6-sol"
effort = "high"

[codex.verifier]
inherit = true
```

The global and local files have the same schema. A missing worker file is an
empty layer. Removing its final profile leaves a deterministic file containing
only `schema_version = 1`; the command does not delete the file. The local
project file is required for local and effective operations. Schema version 1
is unchanged: the two added roles are optional tables, so every valid legacy
implementer/verifier file retains the same meaning.

## Command grammar

The public surface is:

```text
zdev config show [--global | --local]
zdev config get [--global | --local] <key>
zdev config set [--global | --local] [--allow-divergent] <key> <value>...
zdev config unset [--global | --local] <key>
zdev config trunk [<branch>] [--allow-divergent]
```

`--global` and `--local` are mutually exclusive. The existing global
`--format text|json` and `--root` options keep their current placement and
meaning. `--root` does not change the global file location.

Reads without a scope show the effective value. Scoped reads show only values
stored in that scope; they do not inject defaults or values from another
scope. `show` succeeds with an empty `values` array when a selected worker
scope has no entries. `get` of an absent scoped key fails without output.

Writes default to local, matching the common repository use and the existing
trunk command. A global write works outside a repository. A local write and an
unscoped read require an initialized repository. Using a repo-only key with
`--global` is an unsupported-scope error.

Scalar settings take exactly one value:

```text
zdev config set project.trunk main
zdev config set project.default-area improvements
zdev config set project.guidance docs/build.md
```

A worker key takes either the single value `inherit`, or exactly two values,
model and effort:

```text
zdev config set --global worker.codex.implementer gpt-5.6-sol high
zdev config set worker.codex.verifier inherit
```

`set` replaces the one value in the selected scope. `unset` removes only the
selected scope's value, exposing the next layer when one exists. Unsetting a
missing value is an error. `project.name` and `project.record` are visible to
`show` and `get`, but `set` and `unset` reject them and point to `zdev init` and
the record-policy documentation.

A successful worker-profile `set` or `unset` reports that its harness
integration must be refreshed; it does not refresh it. A local mutation names
`zdev skill install <harness> --scope project --force`, and a global mutation
names `zdev skill install <harness> --scope user --force`. Project-key
mutations omit the refresh fields and hint because they do not change rendered
worker controls.

`zdev config trunk [<branch>]` remains supported. It is the existing
branch-aware convenience command: omission means the checked-out branch. With
an explicit branch it has the same stored result as `zdev config set
project.trunk <branch>`. The generic form never infers a value. When explicit
trunk areas exist, both forms use the locked ancestry and ownership checks in
[Trunk-based area work](trunk-area-mode.md). `--allow-divergent` is accepted
only for these local trunk writes and waives only a resolved false ancestry
result.

## Stable output

Key order is the order in the scope table above: project keys first, then
harnesses in `codex`, `claude`, `opencode`, `pi`, `omp` order, with
`routine-implementer`, `implementer`, `verifier`, and
`advanced-implementer` in that order. A scoped view omits keys not stored in that
scope. Every JSON object uses the lexical key order produced by the current
`serde_json::Value` map and existing `serde_json::to_string_pretty` renderer;
arrays retain the registry and precedence order defined here. No preserve-order
feature or config-specific serializer is needed. Every output ends in one
newline. Paths use `/` separators; local paths are repository-relative and
global paths are absolute.

Human `show` prints one effective or stored value per line. A lower-precedence
candidate follows its winner on an indented `shadows` line. Strings use TOML
quoting, profiles use TOML inline-table notation, and `null` is literal.

### Effective view shape

The current view contains all four roles for each harness in the registry order
above. The older standard-role fixture below is retained as a compact rendering
example; it omits routine and advanced rows. The defaults table above, rather
than values in this abbreviated fixture, is authoritative.

Selected human rows have this form:

```text
project.name = "checkout"  [local .zdev/config.toml]
project.record = "project"  [local .zdev/config.toml]
project.trunk = null  [default]
project.default-area = "payments"  [local .zdev/config.toml]
  shadows null  [default]
project.guidance = "auto"  [default]
worker.codex.implementer = { model = "gpt-5.6-sol", effort = "high" }  [local .zdev/workers.toml]
  shadows { model = "gpt-5.5", effort = "xhigh" }  [global /home/alice/.config/zdev/workers.toml]
  shadows { model = "gpt-5.6-sol", effort = "low" }  [default]
worker.codex.verifier = { model = "gpt-5.5", effort = "high" }  [global /home/alice/.config/zdev/workers.toml]
  shadows { model = "gpt-5.6-sol", effort = "low" }  [default]
worker.claude.implementer = { model = "claude-opus-5", effort = "low" }  [default]
worker.claude.verifier = { inherit = true }  [local .zdev/workers.toml]
  shadows { model = "claude-opus-5", effort = "medium" }  [global /home/alice/.config/zdev/workers.toml]
  shadows { model = "claude-opus-5", effort = "low" }  [default]
worker.opencode.implementer = { model = "openai/gpt-5.6-sol", effort = "low" }  [default]
worker.opencode.verifier = { model = "anthropic/claude-opus-5", effort = "inherit" }  [default]
worker.pi.implementer = { model = "openai/gpt-5.5", effort = "high" }  [global /home/alice/.config/zdev/workers.toml]
  shadows { model = "openai/gpt-5.6-sol", effort = "low" }  [default]
worker.pi.verifier = { model = "anthropic/claude-opus-5", effort = "low" }  [default]
worker.omp.implementer = { model = "openai/gpt-5.6-sol", effort = "low" }  [default]
worker.omp.verifier = { model = "anthropic/claude-opus-5", effort = "low" }  [default]
```

The same selected rows have this JSON shape:

```json
{
  "schema_version": 1,
  "scope": "effective",
  "values": [
    {
      "key": "project.name",
      "origin": {
        "path": ".zdev/config.toml",
        "scope": "local"
      },
      "shadowed": [],
      "value": "checkout"
    },
    {
      "key": "project.record",
      "origin": {
        "path": ".zdev/config.toml",
        "scope": "local"
      },
      "shadowed": [],
      "value": "project"
    },
    {
      "key": "project.trunk",
      "origin": {
        "path": null,
        "scope": "default"
      },
      "shadowed": [],
      "value": null
    },
    {
      "key": "project.default-area",
      "origin": {
        "path": ".zdev/config.toml",
        "scope": "local"
      },
      "shadowed": [
        {
          "origin": {
            "path": null,
            "scope": "default"
          },
          "value": null
        }
      ],
      "value": "payments"
    },
    {
      "key": "project.guidance",
      "origin": {
        "path": null,
        "scope": "default"
      },
      "shadowed": [],
      "value": "auto"
    },
    {
      "key": "worker.codex.implementer",
      "origin": {
        "path": ".zdev/workers.toml",
        "scope": "local"
      },
      "shadowed": [
        {
          "origin": {
            "path": "/home/alice/.config/zdev/workers.toml",
            "scope": "global"
          },
          "value": {
            "effort": "xhigh",
            "model": "gpt-5.5"
          }
        },
        {
          "origin": {
            "path": null,
            "scope": "default"
          },
          "value": {
            "effort": "low",
            "model": "gpt-5.6-sol"
          }
        }
      ],
      "value": {
        "effort": "high",
        "model": "gpt-5.6-sol"
      }
    },
    {
      "key": "worker.codex.verifier",
      "origin": {
        "path": "/home/alice/.config/zdev/workers.toml",
        "scope": "global"
      },
      "shadowed": [
        {
          "origin": {
            "path": null,
            "scope": "default"
          },
          "value": {
            "effort": "low",
            "model": "gpt-5.6-sol"
          }
        }
      ],
      "value": {
        "effort": "high",
        "model": "gpt-5.5"
      }
    },
    {
      "key": "worker.claude.implementer",
      "origin": {
        "path": null,
        "scope": "default"
      },
      "shadowed": [],
      "value": {
        "effort": "low",
        "model": "claude-opus-5"
      }
    },
    {
      "key": "worker.claude.verifier",
      "origin": {
        "path": ".zdev/workers.toml",
        "scope": "local"
      },
      "shadowed": [
        {
          "origin": {
            "path": "/home/alice/.config/zdev/workers.toml",
            "scope": "global"
          },
          "value": {
            "effort": "medium",
            "model": "claude-opus-5"
          }
        },
        {
          "origin": {
            "path": null,
            "scope": "default"
          },
          "value": {
            "effort": "low",
            "model": "claude-opus-5"
          }
        }
      ],
      "value": {
        "inherit": true
      }
    },
    {
      "key": "worker.opencode.implementer",
      "origin": {
        "path": null,
        "scope": "default"
      },
      "shadowed": [],
      "value": {
        "effort": "low",
        "model": "openai/gpt-5.6-sol"
      }
    },
    {
      "key": "worker.opencode.verifier",
      "origin": {
        "path": null,
        "scope": "default"
      },
      "shadowed": [],
      "value": {
        "effort": "inherit",
        "model": "anthropic/claude-opus-5"
      }
    },
    {
      "key": "worker.pi.implementer",
      "origin": {
        "path": "/home/alice/.config/zdev/workers.toml",
        "scope": "global"
      },
      "shadowed": [
        {
          "origin": {
            "path": null,
            "scope": "default"
          },
          "value": {
            "effort": "low",
            "model": "openai/gpt-5.6-sol"
          }
        }
      ],
      "value": {
        "effort": "high",
        "model": "openai/gpt-5.5"
      }
    },
    {
      "key": "worker.pi.verifier",
      "origin": {
        "path": null,
        "scope": "default"
      },
      "shadowed": [],
      "value": {
        "effort": "low",
        "model": "anthropic/claude-opus-5"
      }
    },
    {
      "key": "worker.omp.implementer",
      "origin": {
        "path": null,
        "scope": "default"
      },
      "shadowed": [],
      "value": {
        "effort": "low",
        "model": "openai/gpt-5.6-sol"
      }
    },
    {
      "key": "worker.omp.verifier",
      "origin": {
        "path": null,
        "scope": "default"
      },
      "shadowed": [],
      "value": {
        "effort": "low",
        "model": "anthropic/claude-opus-5"
      }
    }
  ]
}
```

`scope` is always present: `effective` for an unscoped read and `global` or
`local` for a scoped `show`. `shadowed` is always present, including when empty.
An origin always contains `path` then `scope`; default origins use a null path.
Scoped views omit unstored keys instead of emitting null placeholders.

### Scoped and single-key views

For a fixture with two global profiles, `zdev config show --global` is exactly:

```text
worker.codex.implementer = { model = "gpt-5.5", effort = "xhigh" }  [global /home/alice/.config/zdev/workers.toml]
worker.codex.verifier = { inherit = true }  [global /home/alice/.config/zdev/workers.toml]
```

Its JSON form is exactly:

```json
{
  "schema_version": 1,
  "scope": "global",
  "values": [
    {
      "key": "worker.codex.implementer",
      "origin": {
        "path": "/home/alice/.config/zdev/workers.toml",
        "scope": "global"
      },
      "shadowed": [],
      "value": {
        "effort": "xhigh",
        "model": "gpt-5.5"
      }
    },
    {
      "key": "worker.codex.verifier",
      "origin": {
        "path": "/home/alice/.config/zdev/workers.toml",
        "scope": "global"
      },
      "shadowed": [],
      "value": {
        "inherit": true
      }
    }
  ]
}
```

Assume the repository locally sets the implementer to `gpt-5.6-sol high` and
the global value above is present. `zdev config get worker.codex.implementer`
is exactly:

```text
worker.codex.implementer = { model = "gpt-5.6-sol", effort = "high" }  [local .zdev/workers.toml]
  shadows { model = "gpt-5.5", effort = "xhigh" }  [global /home/alice/.config/zdev/workers.toml]
  shadows { model = "gpt-5.6-sol", effort = "low" }  [default]
```

Its JSON form is exactly:

```json
{
  "key": "worker.codex.implementer",
  "origin": {
    "path": ".zdev/workers.toml",
    "scope": "local"
  },
  "schema_version": 1,
  "shadowed": [
    {
      "origin": {
        "path": "/home/alice/.config/zdev/workers.toml",
        "scope": "global"
      },
      "value": {
        "effort": "xhigh",
        "model": "gpt-5.5"
      }
    },
    {
      "origin": {
        "path": null,
        "scope": "default"
      },
      "value": {
        "effort": "low",
        "model": "gpt-5.6-sol"
      }
    }
  ],
  "value": {
    "effort": "high",
    "model": "gpt-5.6-sol"
  }
}
```

Successful mutation output is also stable. For example:

```text
Set worker.codex.implementer in global /home/alice/.config/zdev/workers.toml.
Refresh integration: zdev skill install codex --scope user --force
```

```json
{
  "integration_refresh_command": "zdev skill install codex --scope user --force",
  "integration_refresh_required": true,
  "key": "worker.codex.implementer",
  "origin": {
    "path": "/home/alice/.config/zdev/workers.toml",
    "scope": "global"
  },
  "schema_version": 1,
  "status": "set",
  "value": {
    "effort": "high",
    "model": "gpt-5.6-sol"
  }
}
```

After removing a local override whose global fallback is the fixture above,
the human result is:

```text
Unset worker.codex.implementer from local .zdev/workers.toml.
Effective value: { model = "gpt-5.5", effort = "xhigh" }  [global /home/alice/.config/zdev/workers.toml]
Refresh integration: zdev skill install codex --scope project --force
```

The JSON result is:

```json
{
  "effective": {
    "origin": {
      "path": "/home/alice/.config/zdev/workers.toml",
      "scope": "global"
    },
    "value": {
      "effort": "xhigh",
      "model": "gpt-5.5"
    }
  },
  "integration_refresh_command": "zdev skill install codex --scope project --force",
  "integration_refresh_required": true,
  "key": "worker.codex.implementer",
  "origin": {
    "path": ".zdev/workers.toml",
    "scope": "local"
  },
  "schema_version": 1,
  "status": "unset"
}
```

No supported key is sensitive. Model identifiers, effort names, area tags, and
repository-relative guidance paths are ordinary configuration. Zdev does not
read or display provider credentials, tokens, native harness secrets, or
environment secrets. Credential-like and unknown keys are rejected rather
than accepted and later redacted. Version 1 therefore has no redaction flag or
secret-shaped output. Adding a sensitive key would require a separate public
contract rather than a generic secret store.

## Validation and failure behavior

Both worker files and `.zdev/config.toml` remain strict, versioned TOML. Parsing
rejects an unsupported schema version, unknown table or key, duplicate key,
unknown harness or role, empty model, unknown effort, `inherit` combined with
another field, and an explicit model-effort pair the selected adapter cannot
express. Project values retain their current strict schema.

Additional key validation is narrow:

- `project.trunk` uses the existing Git branch-name canonicalization. Without
  explicit trunk areas it retains the existing ability to name a branch that
  does not exist. With trunk areas, old and candidate tips must exist and be
  inspectable, the complete area ownership graph must remain valid, and the old
  tip must be contained by the candidate unless `--allow-divergent` explicitly
  waives a resolved false result.
- `project.default-area` must be a valid segment naming an existing area.
- `project.guidance` accepts the three named modes or the integration code's
  existing safe repository-relative Markdown path rules. File presence and
  guidance markers are checked when an integration is installed or checked,
  where they matter.
- Worker validation is syntactic and adapter-specific. Zdev does not contact a
  provider or promise account access to a model.

An effective operation validates every present source it would consult before
producing output or changing a file. A scoped global operation consults only
the global worker file. A scoped local operation validates the project record
and the local worker file. Malformed or unreadable present files fail with the
path and offending field. The command does not ignore bad configuration and
does not silently fall back. The user repairs the named hand-editable file.

Unknown keys, unsupported scopes or values, a missing scoped `get` or `unset`,
an uninitialized local operation, and an unavailable configuration home all
fail through zdev's existing text or JSON error envelope. They print no success
payload and change no file. Missing worker files and missing profile tables are
normal fallbacks on effective reads.

For a mutation, zdev validates arguments and all required input. On the first
global mutation it creates the resolved parent `zdev` directory, then opens
`workers.lock` in that directory. Failure to create the directory or lock
changes no configuration file. Local project and worker writes use the existing
Git-path `zdev-state.lock`. With the appropriate lock held, zdev rereads the
target, computes and serializes the complete document, writes and syncs a
temporary file in the destination directory, and atomically replaces only that
destination. Publication failure leaves the previous file bytes in place. One
command changes one file, so there is no cross-file rollback protocol.

Serialization is deterministic and preserves values, not comments or authored
layout. This matches the current project-config writer. A hand edit remains a
supported input; the next CLI mutation normalizes that one file.

## Git evidence and deliberate differences

Current Git documents `get`, `set`, and `unset` subcommands; effective reads
across configuration files; scope-limited reads and writes; origin and scope
reporting; last-value precedence; repository-local writes by default; and a
non-zero result when a requested value is absent. It also says one invocation
changes only one file. These are the useful ideas adopted here. [Official
`git-config` command, options, files, and scopes](https://git-scm.com/docs/git-config)
(accessed 2026-08-20).

The installed Git 2.42.0 still exposes the older option grammar. A local probe
confirmed that `--show-scope --show-origin --get-all` reports both global and
local candidates in precedence order, plain `--get` returns the local winner,
and unsetting a missing local key exits 5. The current official documentation
uses the newer subcommands and describes the same underlying behavior. This is
versioned design evidence, not a compatibility promise.

Zdev rejects Git's system, worktree, command-line, arbitrary-file, and included
configuration scopes. It also rejects multivalued keys, regex matching,
open-ended type coercion, environment-injected values, and a config editor.
Git needs a broad configuration language for many commands and repository
layouts; zdev has twenty-five typed keys and two useful preference layers. In
particular, a zdev worktree scope would make worker choice depend on checkout
plumbing without improving task identity. Git's optional worktree file exists
for genuinely worktree-specific Git settings, a distinction zdev does not
need. [Official `git-worktree` configuration
documentation](https://git-scm.com/docs/git-worktree) (accessed 2026-08-20).

The command names and concepts are familiar, but the grammar, files, values,
and exit behavior are zdev's own. Zdev makes no Git-config compatibility claim.

## Backward compatibility

Existing `.zdev/config.toml` files remain valid without migration. Their field
names, schema version, strict parsing, and meaning do not change. The current
`zdev config trunk` command remains available. The new generic commands expose
and mutate the same typed project fields through the existing reader and
writer; they do not move project state into a preference file.

The `.zdev/workers.toml` shape already specified in
[Worker profiles](worker-profiles.md) becomes the canonical local layer without
renaming or migration. A repository-local profile still wins exactly as
documented earlier. The implemented fallback is a global file before the dated
built-in. User-scoped integration install and check use global then built-in;
project-scoped install and check use local, global, then built-in. Explicit
`inherit` remains authoritative at either scope.

Install and check must use the same resolver and report the zdev origin they
rendered. Existing repositories with neither worker file continue to render the
same built-in profiles byte for byte. No command scans or imports harness-native
configuration.

## Implemented seam and acceptance record

The implementation stays in three places:

1. One configuration module contains the fixed key registry, strict
   worker-file parser, global-path resolution, whole-profile resolver, stable
   views, and atomic scoped mutation. Reuse the existing project config types,
   state lock, validators, and atomic writer rather than creating a general
   settings framework.
2. `ConfigCommand` routing includes `show`, `get`, `set`, and
   `unset`; retain `trunk` as the convenience alias. The command shell renders
   the fixed human views above and passes ordinary `serde_json::Value` objects
   to the existing success renderer.
3. Integration install and check receive the same resolved worker-profile input.
   The existing renderer remains responsible for adapter capability and
   all-or-nothing artifact publication.

No migration command, arbitrary TOML API, secret manager, provider credential
store, environment override system, plugin schema, or harness configuration
scanner belongs in this work.

The current contract requires:

- the twenty-five keys, scope restrictions, fixed ordering, and exact value grammar
  above are enforced;
- effective worker reads resolve local, global, then built-in whole profiles,
  while scoped reads return only stored values;
- unscoped `show` returns the complete twenty-five-key effective view described above,
  including `scope: effective`, local, global, built-in, null/default values,
  shadowed candidates, fixed array order, and one final newline;
- scoped `show` and `get` reproduce their byte-level human and JSON contracts;
  ordinary `serde_json::Value` rendering supplies lexical object-key order;
- set and unset reproduce their byte-level result contracts, reject read-only
  and missing keys, and never leave an invalid intermediate profile;
- missing optional files fall back, while malformed, unknown, unsupported, or
  unreadable values fail with their source and no state mutation;
- project config remains schema-compatible, `config trunk` remains compatible,
  and existing repositories without worker files render the prior built-ins;
- concurrent writes serialize through the appropriate lock, mutate one file,
  and either publish the complete validated bytes atomically or preserve the
  previous file;
- global path resolution ignores relative or empty candidates, reports only an
  absolute normalized origin, and creates the parent directory before locking,
  staging, and publication on the first mutation;
- project and user integration install and check consume the identical resolved
  profile and fail before artifact publication on an unsupported explicit
  value; and
- focused black-box coverage proves one effective shadowed read, one default,
  one scoped mutation and fallback, one strict failure with preserved bytes,
  and the unchanged `config trunk` path. No generic configuration test matrix
  or provider probe is required.

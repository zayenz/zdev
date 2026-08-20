use std::collections::BTreeSet;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Component, Path, PathBuf};

use clap::{Subcommand, ValueEnum};
use minijinja::{AutoEscape, Environment, UndefinedBehavior, context};
use serde_json::{Value, json};

#[cfg(test)]
use super::config::built_in_worker_profiles;
use super::config::{ResolvedWorkers, WorkerHarness, resolve_worker_profiles};
use super::project::{read_config, write_config};
use super::{CommandOutput, SCHEMA_VERSION, ZdevError, relative, resolve_root, write_atomic};

const GUIDANCE_START_MARKER: &str = "<!-- zdev:guidance:start -->";
const GUIDANCE_END_MARKER: &str = "<!-- zdev:guidance:end -->";
const GUIDANCE_TEMPLATE: &str = "# Repository guidance for zdev\n\n## Understand and navigate\n\n## Build and compile\n\n## Run locally\n\n## Test and validate\n\n## Format and lint\n\n## Generated files and migrations\n\n## Safety, secrets, and unavailable services\n";
const SHARED_REFERENCE_FILES: &[(&str, &str)] = &[
    (
        "references/discuss.md",
        include_str!("../templates/zdev/references/discuss.md"),
    ),
    (
        "references/implement.md",
        include_str!("../templates/zdev/references/implement.md"),
    ),
    (
        "references/improve.md",
        include_str!("../templates/zdev/references/improve.md"),
    ),
    (
        "references/investigate.md",
        include_str!("../templates/zdev/references/investigate.md"),
    ),
    (
        "references/shape-work.md",
        include_str!("../templates/zdev/references/shape-work.md"),
    ),
    (
        "references/setup.md",
        include_str!("../templates/zdev/references/setup.md"),
    ),
    (
        "references/task-format.md",
        include_str!("../templates/zdev/references/task-format.md"),
    ),
    (
        "references/to-tasks.md",
        include_str!("../templates/zdev/references/to-tasks.md"),
    ),
    (
        "references/recovery.md",
        include_str!("../templates/zdev/references/recovery.md"),
    ),
    (
        "references/verify.md",
        include_str!("../templates/zdev/references/verify.md"),
    ),
];
const SHARED_CONTRACT_TEMPLATE: &str = include_str!("../templates/zdev/shared-contract.md");
const CODEX_SKILL_TEMPLATE: &str = include_str!("../templates/zdev/codex-skill.md");
const CODEX_OPENAI_YAML: &str = include_str!("../templates/zdev/codex/agents/openai.yaml");
const CLAUDE_SKILL_TEMPLATE: &str = include_str!("../templates/zdev/claude-skill.md");
const CLAUDE_PLUGIN_TEMPLATE: &str = include_str!("../templates/zdev/claude/plugin.json");
const CLAUDE_IMPLEMENTER: &str =
    include_str!("../templates/zdev/claude/agents/zdev-implementer.md");
const CLAUDE_VERIFIER: &str = include_str!("../templates/zdev/claude/agents/zdev-verifier.md");
const CLAUDE_TASK_WORKFLOW: &str = include_str!("../templates/zdev/claude/workflows/zdev-task.js");
const CLAUDE_AUDIT_WORKFLOW: &str =
    include_str!("../templates/zdev/claude/workflows/zdev-audit.js");
const OPENCODE_SKILL_TEMPLATE: &str = include_str!("../templates/zdev/opencode-skill.md");
const OPENCODE_IMPLEMENTER: &str =
    include_str!("../templates/zdev/opencode/agents/zdev-implementer.md");
const OPENCODE_VERIFIER: &str = include_str!("../templates/zdev/opencode/agents/zdev-verifier.md");
const OPENCODE_TASK_COMMAND: &str = include_str!("../templates/zdev/opencode/command/zdev-task.md");
const OPENCODE_AUDIT_COMMAND: &str =
    include_str!("../templates/zdev/opencode/command/zdev-audit.md");
const PI_SKILL_TEMPLATE: &str = include_str!("../templates/zdev/pi-skill.md");
const PI_TASK_PROMPT: &str = include_str!("../templates/zdev/pi/prompts/zdev-task.md");
const PI_AUDIT_PROMPT: &str = include_str!("../templates/zdev/pi/prompts/zdev-audit.md");
const PI_SUBAGENT_EXTENSION: &str =
    include_str!("../templates/zdev/pi/extensions/zdev-subagent.ts");
const OMP_SKILL_TEMPLATE: &str = include_str!("../templates/zdev/omp-skill.md");
const OMP_IMPLEMENTER: &str = include_str!("../templates/zdev/omp/agents/zdev-implementer.md");
const OMP_VERIFIER: &str = include_str!("../templates/zdev/omp/agents/zdev-verifier.md");
const OMP_RELOCATED_USER_WARNING: &str = "Oh My Pi 17.2.15 discovers the zdev skill at this PI_CODING_AGENT_DIR location but may not discover its user task agents. Unset PI_CODING_AGENT_DIR to use ~/.omp/agent, or install with --scope project under .omp, until upstream task-agent discovery is fixed.";

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub(super) enum Harness {
    Codex,
    Claude,
    Opencode,
    Pi,
    Omp,
}
impl Harness {
    fn worker_harness(self) -> WorkerHarness {
        match self {
            Self::Codex => WorkerHarness::Codex,
            Self::Claude => WorkerHarness::Claude,
            Self::Opencode => WorkerHarness::Opencode,
            Self::Pi => WorkerHarness::Pi,
            Self::Omp => WorkerHarness::Omp,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Codex => "codex",
            Self::Claude => "claude",
            Self::Opencode => "opencode",
            Self::Pi => "pi",
            Self::Omp => "omp",
        }
    }

    fn display_name(self) -> &'static str {
        match self {
            Self::Codex => "Codex",
            Self::Claude => "Claude Code",
            Self::Opencode => "OpenCode",
            Self::Pi => "Pi",
            Self::Omp => "Oh My Pi",
        }
    }

    fn question_tool_guidance(self) -> &'static str {
        match self {
            Self::Codex => {
                "Use Codex's `request_user_input` tool with two or three questions in one call when it is available. Put the recommended option first for each question and explain its impact. If the tool is unavailable, present the same round as a concise numbered list."
            }
            Self::Claude => {
                "Use Claude Code's `AskUserQuestion` tool with multiple questions in one call. Give each question concrete options, mark the recommended answer in its wording or description, and reserve plain text for free-form explanation."
            }
            Self::Opencode => {
                "Use OpenCode's `question` tool with multiple questions in one call. Give each question concrete options, put the recommended answer first, and reserve plain text for free-form explanation."
            }
            Self::Pi => {
                "Stock Pi has no structured question tool. If an installed tool such as `ask_user` or `ask_question` is available, use it and batch questions when its schema permits; otherwise present the round as a concise numbered list with the recommended answer under each question."
            }
            Self::Omp => {
                "Use Oh My Pi's `ask` tool with its `questions` array so one call presents the whole round. Give each question concrete options and descriptions, put the recommended answer first, and reserve plain text for free-form explanation."
            }
        }
    }

    fn integration(
        self,
        guidance: Option<(&str, &str)>,
        workers: ResolvedWorkers,
    ) -> Result<SkillIntegration, ZdevError> {
        let mut files = Vec::new();
        match self {
            Self::Codex => {
                files.push(IntegrationFile {
                    path: "SKILL.md".to_owned(),
                    content: CODEX_SKILL_TEMPLATE.to_owned(),
                });
                files.push(IntegrationFile {
                    path: "agents/openai.yaml".to_owned(),
                    content: CODEX_OPENAI_YAML.to_owned(),
                });
                files.extend(SHARED_REFERENCE_FILES.iter().map(|(path, content)| {
                    IntegrationFile {
                        path: (*path).to_owned(),
                        content: (*content).to_owned(),
                    }
                }));
            }
            Self::Claude => {
                files.push(IntegrationFile {
                    path: ".claude-plugin/plugin.json".to_owned(),
                    content: CLAUDE_PLUGIN_TEMPLATE.to_owned(),
                });
                files.push(IntegrationFile {
                    path: "skills/zdev/SKILL.md".to_owned(),
                    content: CLAUDE_SKILL_TEMPLATE.to_owned(),
                });
                files.extend(SHARED_REFERENCE_FILES.iter().map(|(path, content)| {
                    IntegrationFile {
                        path: format!("skills/zdev/{path}"),
                        content: (*content).to_owned(),
                    }
                }));
                files.extend([
                    IntegrationFile {
                        path: "agents/zdev-implementer.md".to_owned(),
                        content: CLAUDE_IMPLEMENTER.to_owned(),
                    },
                    IntegrationFile {
                        path: "agents/zdev-verifier.md".to_owned(),
                        content: CLAUDE_VERIFIER.to_owned(),
                    },
                    IntegrationFile {
                        path: "workflows/zdev-task.js".to_owned(),
                        content: CLAUDE_TASK_WORKFLOW.to_owned(),
                    },
                    IntegrationFile {
                        path: "workflows/zdev-audit.js".to_owned(),
                        content: CLAUDE_AUDIT_WORKFLOW.to_owned(),
                    },
                ]);
            }
            Self::Opencode => {
                files.push(IntegrationFile {
                    path: "skills/zdev-opencode/SKILL.md".to_owned(),
                    content: OPENCODE_SKILL_TEMPLATE.to_owned(),
                });
                files.extend(SHARED_REFERENCE_FILES.iter().map(|(path, content)| {
                    IntegrationFile {
                        path: format!("skills/zdev-opencode/{path}"),
                        content: (*content).to_owned(),
                    }
                }));
                files.extend([
                    IntegrationFile {
                        path: "agents/zdev-implementer.md".to_owned(),
                        content: OPENCODE_IMPLEMENTER.to_owned(),
                    },
                    IntegrationFile {
                        path: "agents/zdev-verifier.md".to_owned(),
                        content: OPENCODE_VERIFIER.to_owned(),
                    },
                    IntegrationFile {
                        path: "command/zdev-task.md".to_owned(),
                        content: OPENCODE_TASK_COMMAND.to_owned(),
                    },
                    IntegrationFile {
                        path: "command/zdev-audit.md".to_owned(),
                        content: OPENCODE_AUDIT_COMMAND.to_owned(),
                    },
                ]);
            }
            Self::Pi => {
                files.push(IntegrationFile {
                    path: "skills/zdev-pi/SKILL.md".to_owned(),
                    content: PI_SKILL_TEMPLATE.to_owned(),
                });
                files.extend(SHARED_REFERENCE_FILES.iter().map(|(path, content)| {
                    IntegrationFile {
                        path: format!("skills/zdev-pi/{path}"),
                        content: (*content).to_owned(),
                    }
                }));
                files.extend([
                    IntegrationFile {
                        path: "prompts/zdev-task.md".to_owned(),
                        content: PI_TASK_PROMPT.to_owned(),
                    },
                    IntegrationFile {
                        path: "prompts/zdev-audit.md".to_owned(),
                        content: PI_AUDIT_PROMPT.to_owned(),
                    },
                    IntegrationFile {
                        path: "extensions/zdev-subagent.ts".to_owned(),
                        content: PI_SUBAGENT_EXTENSION.to_owned(),
                    },
                ]);
            }
            Self::Omp => {
                files.push(IntegrationFile {
                    path: "skills/zdev/SKILL.md".to_owned(),
                    content: OMP_SKILL_TEMPLATE.to_owned(),
                });
                files.extend(SHARED_REFERENCE_FILES.iter().map(|(path, content)| {
                    IntegrationFile {
                        path: format!("skills/zdev/{path}"),
                        content: (*content).to_owned(),
                    }
                }));
                files.extend([
                    IntegrationFile {
                        path: "agents/zdev-implementer.md".to_owned(),
                        content: OMP_IMPLEMENTER.to_owned(),
                    },
                    IntegrationFile {
                        path: "agents/zdev-verifier.md".to_owned(),
                        content: OMP_VERIFIER.to_owned(),
                    },
                ]);
            }
        }
        realize_templates(self, guidance, &workers, &mut files)?;
        Ok(SkillIntegration {
            harness: self,
            version: env!("CARGO_PKG_VERSION"),
            layout: if matches!(self, Self::Opencode | Self::Pi | Self::Omp) {
                IntegrationLayout::SharedRoot
            } else {
                IntegrationLayout::ExactTree
            },
            files,
            workers,
        })
    }
}

fn repository_guidance(guidance: Option<(&str, &str)>) -> String {
    match guidance {
        Some((source, content)) => format!(
            "<!-- zdev:generated-repository-guidance:start -->\n## Rendered repository guidance\n\nSource: `{source}`. The source file remains authoritative.\n\n{}\n<!-- zdev:generated-repository-guidance:end -->",
            content.trim_end()
        ),
        None => "<!-- zdev:generated-repository-guidance:start -->\n## Repository guidance discovery\n\nBefore planning or changing code, read applicable repository and directory-specific `AGENTS.md` files, `.zdev/guidance.md` when present, and harness-native repository instructions. Pass relevant build, run, test, generated-file, and safety guidance to every delegated role.\n<!-- zdev:generated-repository-guidance:end -->".to_owned(),
    }
}

fn template_environment() -> Environment<'static> {
    let mut environment = Environment::empty();
    environment.set_undefined_behavior(UndefinedBehavior::Strict);
    environment.set_auto_escape_callback(|_| AutoEscape::None);
    environment
}

fn render_template(
    name: &str,
    source: &str,
    shared_contract: &str,
    repository_guidance: &str,
    question_tool_guidance: &str,
    version: &str,
    workers: &ResolvedWorkers,
) -> Result<String, ZdevError> {
    let environment = template_environment();
    let template = environment
        .template_from_named_str(name, source)
        .map_err(|error| {
            ZdevError::new(format!(
                "Cannot parse zdev integration template {name}: {error}"
            ))
        })?;
    let mut rendered = template
        .render(context! {
            shared_contract,
            repository_guidance,
            question_tool_guidance,
            version,
            implementer_has_model => workers.implementer.has_model(),
            implementer_has_effort => workers.implementer.has_effort(),
            implementer_model => workers.implementer.model_literal(),
            implementer_effort => workers.implementer.effort_literal(),
            verifier_has_model => workers.verifier.has_model(),
            verifier_has_effort => workers.verifier.has_effort(),
            verifier_model => workers.verifier.model_literal(),
            verifier_effort => workers.verifier.effort_literal(),
        })
        .map_err(|error| {
            ZdevError::new(format!(
                "Cannot render zdev integration template {name}: {error}"
            ))
        })?;
    if source.ends_with('\n') && !rendered.ends_with('\n') {
        rendered.push('\n');
    }
    Ok(rendered)
}

fn prepare_template_value(path: &str, value: &str) -> Result<String, ZdevError> {
    if path.ends_with(".json") {
        serde_json::to_string(value).map_err(|error| {
            ZdevError::new(format!(
                "Cannot prepare values for zdev integration template {path}: {error}"
            ))
        })
    } else {
        Ok(value.to_owned())
    }
}

fn realize_templates(
    harness: Harness,
    guidance: Option<(&str, &str)>,
    workers: &ResolvedWorkers,
    files: &mut [IntegrationFile],
) -> Result<(), ZdevError> {
    let repository_guidance = repository_guidance(guidance);
    let version = env!("CARGO_PKG_VERSION");
    let shared_contract = render_template(
        "shared-contract.md",
        SHARED_CONTRACT_TEMPLATE,
        "",
        &repository_guidance,
        harness.question_tool_guidance(),
        version,
        workers,
    )?;

    for file in files {
        let is_json = file.path.ends_with(".json");
        let shared_contract = prepare_template_value(&file.path, shared_contract.trim_end())?;
        let repository_guidance = prepare_template_value(&file.path, &repository_guidance)?;
        let question_tool_guidance =
            prepare_template_value(&file.path, harness.question_tool_guidance())?;
        let version = prepare_template_value(&file.path, version)?;
        file.content = render_template(
            &file.path,
            &file.content,
            &shared_contract,
            &repository_guidance,
            &question_tool_guidance,
            &version,
            workers,
        )?;
        if is_json {
            serde_json::from_str::<Value>(&file.content).map_err(|error| {
                ZdevError::new(format!(
                    "Rendered zdev integration template {} is invalid JSON: {error}",
                    file.path
                ))
            })?;
        }
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ValueEnum)]
pub(super) enum InstallationScope {
    #[default]
    User,
    Project,
}

impl InstallationScope {
    fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Project => "project",
        }
    }
}

#[derive(Debug, Subcommand)]
pub(super) enum SkillCommand {
    /// Install or update one coding-harness integration
    ///
    /// User scope is the default and works across repositories. Project scope
    /// writes harness-native files into this repository; use --guidance to
    /// choose the repository instructions embedded in those files.
    /// Harness values are codex (Codex), claude (Claude Code), opencode
    /// (OpenCode), pi (Pi), and omp (Oh My Pi).
    Install {
        /// Coding harness to integrate with zdev
        harness: Harness,
        /// Install for the current user or in this repository
        #[arg(long, value_enum, default_value_t = InstallationScope::User)]
        scope: InstallationScope,
        /// Install into this exact directory instead of the scope's default
        #[arg(long, value_name = "PATH")]
        to: Option<PathBuf>,
        /// Project guidance source: auto, agents, zdev, or a repository-relative Markdown file
        #[arg(long, value_name = "SOURCE")]
        guidance: Option<String>,
        /// Replace an installed integration whose files differ from this version
        #[arg(long)]
        force: bool,
    },
    /// Check whether one coding-harness integration is ready to use
    ///
    /// Exits successfully with status `ok`. Missing or differing files produce
    /// status `missing` or `conflict` and a nonzero exit status.
    Check {
        /// Coding harness whose zdev integration to check
        harness: Harness,
        /// Check the current user's installation or this repository's installation
        #[arg(long, value_enum, default_value_t = InstallationScope::User)]
        scope: InstallationScope,
        /// Check this exact directory instead of the scope's default
        #[arg(long, value_name = "PATH")]
        to: Option<PathBuf>,
        /// Project guidance source: auto, agents, zdev, or a repository-relative Markdown file
        #[arg(long, value_name = "SOURCE")]
        guidance: Option<String>,
    },
}

struct SkillIntegration {
    harness: Harness,
    version: &'static str,
    layout: IntegrationLayout,
    files: Vec<IntegrationFile>,
    workers: ResolvedWorkers,
}

#[derive(Clone, Copy)]
enum IntegrationLayout {
    ExactTree,
    SharedRoot,
}

struct IntegrationFile {
    path: String,
    content: String,
}

struct ResolvedIntegrationDestination {
    path: PathBuf,
    scope: &'static str,
}

struct InstallResult {
    status: &'static str,
    destination: PathBuf,
}

struct GuidanceStatus {
    mode: String,
    status: &'static str,
    source: String,
    present: bool,
    readable: bool,
    marked_block_current: Option<bool>,
    content: Option<String>,
}

impl GuidanceStatus {
    fn healthy(&self) -> bool {
        matches!(self.status, "ok" | "created")
    }

    fn value(&self) -> Value {
        json!({
            "mode": self.mode,
            "status": self.status,
            "source": self.source,
            "present": self.present,
            "readable": self.readable,
            "marked_block_current": self.marked_block_current,
        })
    }
}

pub(super) fn run_skill_command(
    requested_root: Option<&Path>,
    command: &SkillCommand,
) -> Result<CommandOutput, ZdevError> {
    let (harness, scope, requested, guidance) = match command {
        SkillCommand::Install {
            harness,
            scope,
            to,
            guidance,
            ..
        }
        | SkillCommand::Check {
            harness,
            scope,
            to,
            guidance,
        } => (*harness, *scope, to.as_deref(), guidance.as_deref()),
    };
    if scope != InstallationScope::Project && guidance.is_some() {
        return Err(ZdevError::new(
            "--guidance is available only with --scope project",
        ));
    }
    let project_root = if scope == InstallationScope::Project {
        Some(resolve_root(requested_root)?)
    } else {
        None
    };
    if let Some(root) = project_root.as_deref() {
        read_config(root).map_err(|error| {
            ZdevError::new(format!(
                "Initialize zdev with `zdev init --record <personal|project|pull-request>` before using a project integration: {error}"
            ))
        })?;
    }
    let workers = resolve_worker_profiles(project_root.as_deref(), harness.worker_harness())?;
    let destination =
        resolve_integration_destination(harness, scope, requested, project_root.as_deref())?;
    let warnings = integration_warnings(harness, scope, requested);
    let guidance_selection = if let Some(root) = project_root.as_deref() {
        guidance
            .map(str::to_owned)
            .or(read_recorded_guidance(root)?)
            .unwrap_or_else(|| "auto".to_owned())
    } else {
        "auto".to_owned()
    };
    match command {
        SkillCommand::Install { force, .. } => {
            let guidance = project_root
                .as_deref()
                .map(|root| inspect_guidance(root, &guidance_selection, true, true))
                .transpose()?;
            let guidance_view = guidance.as_ref().and_then(|guidance| {
                guidance
                    .content
                    .as_deref()
                    .map(|content| (guidance.source.as_str(), content))
            });
            let integration = harness.integration(guidance_view, workers)?;
            let result = publish_integration(&integration, &destination.path, *force)?;
            if let (Some(root), Some(guidance)) = (project_root.as_deref(), guidance.as_ref()) {
                let recorded = if guidance_selection == "agents" {
                    "agents"
                } else {
                    &guidance.source
                };
                record_guidance(root, recorded)?;
            }
            Ok(install_integration_output(
                integration,
                destination,
                result,
                guidance,
                &warnings,
            ))
        }
        SkillCommand::Check { .. } => {
            let guidance = project_root
                .as_deref()
                .map(|root| inspect_guidance(root, &guidance_selection, false, false))
                .transpose()?;
            let guidance_view = guidance.as_ref().and_then(|guidance| {
                guidance
                    .content
                    .as_deref()
                    .map(|content| (guidance.source.as_str(), content))
            });
            check_integration(
                harness.integration(guidance_view, workers)?,
                destination,
                guidance,
                project_root.as_ref().map(|_| guidance_selection.as_str()),
                &warnings,
            )
        }
    }
}

fn integration_warnings(
    harness: Harness,
    scope: InstallationScope,
    requested: Option<&Path>,
) -> Vec<&'static str> {
    if harness == Harness::Omp
        && scope == InstallationScope::User
        && requested.is_none()
        && env::var_os("PI_CODING_AGENT_DIR").is_some_and(|value| !value.is_empty())
    {
        vec![OMP_RELOCATED_USER_WARNING]
    } else {
        Vec::new()
    }
}

fn resolve_integration_destination(
    harness: Harness,
    scope: InstallationScope,
    requested: Option<&Path>,
    project_root: Option<&Path>,
) -> Result<ResolvedIntegrationDestination, ZdevError> {
    if let Some(path) = requested {
        let path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            env::current_dir()
                .map_err(|error| ZdevError::io("Cannot read current directory", error))?
                .join(path)
        };
        return Ok(ResolvedIntegrationDestination {
            path,
            scope: "explicit",
        });
    }
    let path = match scope {
        InstallationScope::Project => {
            let root = project_root.ok_or_else(|| {
                ZdevError::new("Cannot resolve the project integration destination")
            })?;
            match harness {
                Harness::Codex => root.join(".codex/skills/zdev"),
                Harness::Claude => root.join(".claude/skills/zdev"),
                Harness::Opencode => root.join(".opencode"),
                Harness::Pi => root.join(".pi"),
                Harness::Omp => root.join(".omp"),
            }
        }
        InstallationScope::User => {
            let configured = match harness {
                Harness::Codex => env::var_os("CODEX_HOME"),
                Harness::Claude => env::var_os("CLAUDE_CONFIG_DIR"),
                Harness::Opencode => env::var_os("XDG_CONFIG_HOME")
                    .filter(|value| !value.is_empty())
                    .map(|value| PathBuf::from(value).join("opencode").into_os_string()),
                Harness::Pi => env::var_os("PI_CODING_AGENT_DIR"),
                Harness::Omp => env::var_os("PI_CODING_AGENT_DIR"),
            }
            .filter(|value| !value.is_empty());
            let config_home = if let Some(configured) = configured {
                PathBuf::from(configured)
            } else {
                let home = env::var_os("HOME")
                    .or_else(|| env::var_os("USERPROFILE"))
                    .filter(|value| !value.is_empty())
                    .ok_or_else(|| {
                        let variable = match harness {
                            Harness::Codex => "CODEX_HOME",
                            Harness::Claude => "CLAUDE_CONFIG_DIR",
                            Harness::Opencode => "XDG_CONFIG_HOME",
                            Harness::Pi => "PI_CODING_AGENT_DIR",
                            Harness::Omp => "PI_CODING_AGENT_DIR",
                        };
                        ZdevError::new(format!(
                            "Cannot locate the {} home; set {variable} or pass --to",
                            harness.display_name()
                        ))
                    })?;
                PathBuf::from(home).join(match harness {
                    Harness::Codex => ".codex",
                    Harness::Claude => ".claude",
                    Harness::Opencode => ".config/opencode",
                    Harness::Pi => ".pi/agent",
                    Harness::Omp => ".omp/agent",
                })
            };
            match harness {
                Harness::Codex | Harness::Claude => config_home.join("skills/zdev"),
                Harness::Opencode | Harness::Pi | Harness::Omp => config_home,
            }
        }
    };
    Ok(ResolvedIntegrationDestination {
        path,
        scope: scope.as_str(),
    })
}

fn read_recorded_guidance(root: &Path) -> Result<Option<String>, ZdevError> {
    if !root.join(".zdev/config.toml").is_file() {
        return Ok(None);
    }
    Ok(read_config(root)?.project.guidance)
}

fn record_guidance(root: &Path, source: &str) -> Result<(), ZdevError> {
    let mut config = read_config(root)?;
    if config.project.guidance.as_deref() == Some(source) {
        return Ok(());
    }
    config.project.guidance = Some(source.to_owned());
    fs::create_dir_all(root.join(".zdev"))
        .map_err(|error| ZdevError::io("Cannot create .zdev", error))?;
    write_config(root, &config)
}

fn inspect_guidance(
    root: &Path,
    selection: &str,
    scaffold: bool,
    installing: bool,
) -> Result<GuidanceStatus, ZdevError> {
    let agents = root.join("AGENTS.md");
    let fallback = root.join(".zdev/guidance.md");
    let (mode, path, marked, may_scaffold) = match selection {
        "auto" => {
            if fs::symlink_metadata(&agents).is_ok() {
                ("auto", agents, false, false)
            } else {
                ("auto", fallback, false, true)
            }
        }
        "agents" => ("agents", agents, true, false),
        "zdev" => ("zdev", fallback, false, true),
        custom => (
            "custom",
            resolve_custom_guidance_path(root, custom)?,
            false,
            false,
        ),
    };
    let mut created = false;
    if !path.exists() && scaffold && may_scaffold {
        let parent = path.parent().ok_or_else(|| {
            ZdevError::new(format!("Guidance path has no parent: {}", path.display()))
        })?;
        fs::create_dir_all(parent)
            .map_err(|error| ZdevError::io(format!("Cannot create {}", parent.display()), error))?;
        write_atomic(&path, GUIDANCE_TEMPLATE.as_bytes())?;
        created = true;
    }
    if !path.exists() {
        if installing && !may_scaffold {
            return Err(ZdevError::new(format!(
                "Selected guidance file does not exist: {}",
                path.display()
            )));
        }
        return Ok(GuidanceStatus {
            mode: mode.to_owned(),
            status: "missing",
            source: relative(root, &path),
            present: false,
            readable: false,
            marked_block_current: marked.then_some(false),
            content: None,
        });
    }
    let metadata = fs::symlink_metadata(&path)
        .map_err(|error| ZdevError::io(format!("Cannot inspect {}", path.display()), error))?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(ZdevError::new(format!(
            "Guidance must be a regular file and not a symlink: {}",
            path.display()
        )));
    }
    let bytes = match fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) if installing => {
            return Err(ZdevError::io(
                format!("Cannot read selected guidance {}", path.display()),
                error,
            ));
        }
        Err(_) => {
            return Ok(GuidanceStatus {
                mode: mode.to_owned(),
                status: "unreadable",
                source: relative(root, &path),
                present: true,
                readable: false,
                marked_block_current: marked.then_some(false),
                content: None,
            });
        }
    };
    let Ok(content) = String::from_utf8(bytes) else {
        if installing {
            return Err(ZdevError::new(format!(
                "Selected guidance is not UTF-8: {}",
                path.display()
            )));
        }
        return Ok(GuidanceStatus {
            mode: mode.to_owned(),
            status: "unreadable",
            source: relative(root, &path),
            present: true,
            readable: false,
            marked_block_current: marked.then_some(false),
            content: None,
        });
    };
    let marked_block_current = marked.then(|| guidance_markers_are_current(&content));
    if installing && marked_block_current == Some(false) {
        return Err(ZdevError::new(format!(
            "AGENTS.md does not contain one current zdev guidance marker block ({GUIDANCE_START_MARKER} ... {GUIDANCE_END_MARKER})"
        )));
    }
    let status = if marked_block_current == Some(false) {
        "unmarked"
    } else if created {
        "created"
    } else {
        "ok"
    };
    let rendered_content = if marked {
        marked_guidance_content(&content).map(str::to_owned)
    } else {
        Some(content)
    };
    Ok(GuidanceStatus {
        mode: mode.to_owned(),
        status,
        source: relative(root, &path),
        present: true,
        readable: true,
        marked_block_current,
        content: rendered_content,
    })
}

fn resolve_custom_guidance_path(root: &Path, selection: &str) -> Result<PathBuf, ZdevError> {
    let requested = Path::new(selection);
    if requested.is_absolute()
        || requested.extension().and_then(OsStr::to_str) != Some("md")
        || requested
            .components()
            .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(ZdevError::new(
            "Custom guidance must be a repository-relative .md file",
        ));
    }
    let path = normalize_path(&root.join(requested));
    if !path.starts_with(root) || path == root {
        return Err(ZdevError::new(
            "Custom guidance must remain inside the repository",
        ));
    }
    if path.exists() {
        let canonical = fs::canonicalize(&path)
            .map_err(|error| ZdevError::io(format!("Cannot resolve {}", path.display()), error))?;
        if !canonical.starts_with(root) {
            return Err(ZdevError::new(
                "Custom guidance must remain inside the repository",
            ));
        }
    }
    Ok(path)
}

pub(super) fn validate_guidance_selection(root: &Path, selection: &str) -> Result<(), ZdevError> {
    match selection {
        "auto" | "agents" | "zdev" => Ok(()),
        custom => resolve_custom_guidance_path(root, custom).map(|_| ()),
    }
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            other => normalized.push(other.as_os_str()),
        }
    }
    normalized
}

fn guidance_markers_are_current(content: &str) -> bool {
    marked_guidance_content(content).is_some()
}

fn marked_guidance_content(content: &str) -> Option<&str> {
    let starts = content
        .match_indices(GUIDANCE_START_MARKER)
        .collect::<Vec<_>>();
    let ends = content
        .match_indices(GUIDANCE_END_MARKER)
        .collect::<Vec<_>>();
    match (starts.as_slice(), ends.as_slice()) {
        ([(start, marker)], [(end, _)]) if start < end => {
            Some(content[start + marker.len()..*end].trim())
        }
        _ => None,
    }
}

fn installed_integration_matches(
    integration: &SkillIntegration,
    destination: &Path,
) -> Result<bool, ZdevError> {
    if matches!(integration.layout, IntegrationLayout::SharedRoot) {
        for expected in &integration.files {
            let file = destination.join(&expected.path);
            if !file.is_file() {
                return Ok(false);
            }
            let bytes = fs::read(&file)
                .map_err(|error| ZdevError::io(format!("Cannot read {}", file.display()), error))?;
            if bytes != expected.content.as_bytes() {
                return Ok(false);
            }
        }
        return Ok(true);
    }
    let mut actual_files = Vec::new();
    let mut actual_directories = BTreeSet::new();
    let mut pending = vec![destination.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(&directory).map_err(|error| {
            ZdevError::io(format!("Cannot inspect {}", directory.display()), error)
        })? {
            let entry = entry.map_err(|error| ZdevError::io("Cannot inspect skill", error))?;
            let kind = entry
                .file_type()
                .map_err(|error| ZdevError::io("Cannot inspect skill entry", error))?;
            if kind.is_dir() {
                actual_directories.insert(
                    entry
                        .path()
                        .strip_prefix(destination)
                        .map_err(|_| {
                            ZdevError::new("Integration directory escaped its destination")
                        })?
                        .to_path_buf(),
                );
                pending.push(entry.path());
            } else if kind.is_file() {
                actual_files.push(
                    entry
                        .path()
                        .strip_prefix(destination)
                        .map_err(|_| ZdevError::new("Skill file escaped its destination"))?
                        .to_path_buf(),
                );
            } else {
                return Ok(false);
            }
        }
    }
    actual_files.sort();
    let mut expected_files = integration
        .files
        .iter()
        .map(|file| PathBuf::from(&file.path))
        .collect::<Vec<_>>();
    expected_files.sort();
    let expected_directories = expected_files
        .iter()
        .flat_map(|path| path.ancestors().skip(1))
        .filter(|path| !path.as_os_str().is_empty())
        .map(Path::to_path_buf)
        .collect::<BTreeSet<_>>();
    if actual_files != expected_files || actual_directories != expected_directories {
        return Ok(false);
    }
    for expected in &integration.files {
        let file = destination.join(&expected.path);
        let bytes = fs::read(&file)
            .map_err(|error| ZdevError::io(format!("Cannot read {}", file.display()), error))?;
        if bytes != expected.content.as_bytes() {
            return Ok(false);
        }
    }
    Ok(true)
}

fn install_integration_output(
    integration: SkillIntegration,
    resolved: ResolvedIntegrationDestination,
    result: InstallResult,
    guidance: Option<GuidanceStatus>,
    warnings: &[&str],
) -> CommandOutput {
    let mut text = if result.status == "unchanged" {
        format!(
            "{} zdev integration is already current at {}",
            integration.harness.display_name(),
            result.destination.display()
        )
    } else {
        format!(
            "Installed zdev integration for {} at {}",
            integration.harness.display_name(),
            result.destination.display()
        )
    };
    if let Some(guidance) = &guidance {
        text.push_str(&format!(
            "; guidance: {} ({})",
            guidance.source, guidance.status
        ));
    }
    for warning in warnings {
        text.push_str("\nWarning: ");
        text.push_str(warning);
    }
    CommandOutput::new(
        text,
        json!({
            "schema_version": SCHEMA_VERSION,
            "harness": integration.harness.as_str(),
            "bundle_version": integration.version,
            "scope": resolved.scope,
            "status": result.status,
            "path": result.destination,
            "files": integration.files.len(),
            "bundle": {
                "status": result.status,
                "path": result.destination,
                "files": integration.files.len(),
                "version": integration.version,
            },
            "guidance": guidance.as_ref().map(GuidanceStatus::value),
            "workers": integration.workers.value(),
            "warnings": warnings,
        }),
    )
}

fn check_integration(
    integration: SkillIntegration,
    resolved: ResolvedIntegrationDestination,
    guidance: Option<GuidanceStatus>,
    guidance_selection: Option<&str>,
    warnings: &[&str],
) -> Result<CommandOutput, ZdevError> {
    let integration_status = if !resolved.path.exists() {
        "missing"
    } else if resolved.path.is_dir() && installed_integration_matches(&integration, &resolved.path)?
    {
        "ok"
    } else {
        "conflict"
    };
    let status =
        if integration_status == "ok" && guidance.as_ref().is_none_or(GuidanceStatus::healthy) {
            "ok"
        } else if integration_status != "ok" {
            integration_status
        } else {
            "guidance-incomplete"
        };
    let mut install_command = match resolved.scope {
        "user" | "project" => format!(
            "zdev skill install {} --scope {}",
            integration.harness.as_str(),
            resolved.scope
        ),
        _ => format!(
            "zdev skill install {} --to {}",
            integration.harness.as_str(),
            resolved.path.display()
        ),
    };
    if let Some(selection) = guidance_selection {
        install_command.push_str(&format!(" --guidance {selection}"));
    }
    let mut text = match status {
        "guidance-incomplete" => format!(
            "{} zdev integration files are current at {}, but the selected guidance is not ready",
            integration.harness.display_name(),
            resolved.path.display()
        ),
        "ok" => format!(
            "{} zdev integration is ready at {}",
            integration.harness.display_name(),
            resolved.path.display()
        ),
        "missing" => format!(
            "{} zdev integration is missing at {}. Install it with `{install_command}`",
            integration.harness.display_name(),
            resolved.path.display()
        ),
        _ => format!(
            "{} zdev integration differs from this version at {}. Replace it with `{install_command} --force`",
            integration.harness.display_name(),
            resolved.path.display()
        ),
    };
    if let Some(guidance) = &guidance {
        text.push_str(&format!(
            "; guidance: {} ({})",
            guidance.source, guidance.status
        ));
        if status == "guidance-incomplete" {
            text.push_str(&format!(
                ". Fix the guidance source, then rerun `{install_command} --force`"
            ));
        }
    }
    for warning in warnings {
        text.push_str("\nWarning: ");
        text.push_str(warning);
    }
    let mut output = CommandOutput::new(
        text,
        json!({
            "schema_version": SCHEMA_VERSION,
            "harness": integration.harness.as_str(),
            "bundle_version": integration.version,
            "scope": resolved.scope,
            "status": status,
            "path": resolved.path,
            "files": integration.files.len(),
            "bundle": {
                "status": integration_status,
                "path": resolved.path,
                "files": integration.files.len(),
                "version": integration.version,
            },
            "guidance": guidance.as_ref().map(GuidanceStatus::value),
            "workers": integration.workers.value(),
            "warnings": warnings,
        }),
    );
    if status != "ok" {
        output.exit_code = 1;
    }
    Ok(output)
}

fn publish_integration(
    integration: &SkillIntegration,
    destination: &Path,
    force: bool,
) -> Result<InstallResult, ZdevError> {
    if matches!(integration.layout, IntegrationLayout::SharedRoot) {
        return publish_shared_root_integration(integration, destination, force);
    }
    if destination.is_dir() && installed_integration_matches(integration, destination)? {
        return Ok(InstallResult {
            status: "unchanged",
            destination: destination.to_path_buf(),
        });
    }
    if destination.exists() {
        if !destination.is_dir() {
            return Err(ZdevError::new(format!(
                "The zdev integration destination for {} is not a directory: {}",
                integration.harness.display_name(),
                destination.display()
            )));
        }
        if !force {
            return Err(ZdevError::new(format!(
                "A different zdev integration for {} exists at {}; inspect it or rerun with --force",
                integration.harness.display_name(),
                destination.display()
            )));
        }
    }
    let parent = destination.parent().ok_or_else(|| {
        ZdevError::new(format!(
            "Integration destination has no parent: {}",
            destination.display()
        ))
    })?;
    fs::create_dir_all(parent)
        .map_err(|error| ZdevError::io(format!("Cannot create {}", parent.display()), error))?;
    let stage = tempfile::Builder::new()
        .prefix(".zdev-integration-")
        .tempdir_in(parent)
        .map_err(|error| ZdevError::io("Cannot stage the zdev integration", error))?;
    for file in &integration.files {
        let output = stage.path().join(&file.path);
        if let Some(directory) = output.parent() {
            fs::create_dir_all(directory).map_err(|error| {
                ZdevError::io(format!("Cannot create {}", directory.display()), error)
            })?;
        }
        fs::write(&output, &file.content)
            .map_err(|error| ZdevError::io(format!("Cannot write {}", output.display()), error))?;
    }
    let replaced = destination.exists();
    if replaced {
        let backup = tempfile::Builder::new()
            .prefix(".zdev-integration-backup-")
            .tempdir_in(parent)
            .map_err(|error| ZdevError::io("Cannot stage the existing integration", error))?;
        let previous = backup.path().join("previous");
        fs::rename(destination, &previous).map_err(|error| {
            ZdevError::io(format!("Cannot preserve {}", destination.display()), error)
        })?;
        if let Err(error) = fs::rename(stage.keep(), destination) {
            let _ = fs::rename(&previous, destination);
            return Err(ZdevError::io(
                format!("Cannot publish {}", destination.display()),
                error,
            ));
        }
    } else {
        fs::rename(stage.keep(), destination).map_err(|error| {
            ZdevError::io(format!("Cannot publish {}", destination.display()), error)
        })?;
    }
    let status = if replaced { "replaced" } else { "created" };
    Ok(InstallResult {
        status,
        destination: destination.to_path_buf(),
    })
}

fn publish_shared_root_integration(
    integration: &SkillIntegration,
    destination: &Path,
    force: bool,
) -> Result<InstallResult, ZdevError> {
    if destination.exists() && !destination.is_dir() {
        return Err(ZdevError::new(format!(
            "The zdev integration destination for {} is not a directory: {}",
            integration.harness.display_name(),
            destination.display()
        )));
    }
    if destination.is_dir() && installed_integration_matches(integration, destination)? {
        return Ok(InstallResult {
            status: "unchanged",
            destination: destination.to_path_buf(),
        });
    }
    let mut replaced = false;
    for expected in &integration.files {
        let path = destination.join(&expected.path);
        if path.exists() {
            replaced = true;
            let matches = path.is_file()
                && fs::read(&path)
                    .map(|bytes| bytes == expected.content.as_bytes())
                    .unwrap_or(false);
            if !matches && !force {
                return Err(ZdevError::new(format!(
                    "A different zdev integration file for {} exists at {}; inspect it or rerun with --force",
                    integration.harness.display_name(),
                    path.display()
                )));
            }
        }
    }
    fs::create_dir_all(destination).map_err(|error| {
        ZdevError::io(format!("Cannot create {}", destination.display()), error)
    })?;
    for expected in &integration.files {
        let path = destination.join(&expected.path);
        let parent = path.parent().ok_or_else(|| {
            ZdevError::new(format!(
                "Integration file has no parent: {}",
                path.display()
            ))
        })?;
        fs::create_dir_all(parent)
            .map_err(|error| ZdevError::io(format!("Cannot create {}", parent.display()), error))?;
        write_atomic(&path, expected.content.as_bytes())?;
    }
    Ok(InstallResult {
        status: if replaced { "replaced" } else { "created" },
        destination: destination.to_path_buf(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_failures_name_the_source_and_fail_realization() {
        let workers = built_in_worker_profiles(WorkerHarness::Claude);
        for (name, source, expected) in [
            ("unknown.md", "{{unknown}}", "Cannot render"),
            ("invalid.md", "{{", "Cannot parse"),
        ] {
            let error = render_template(name, source, "", "", "", "", &workers)
                .expect_err("invalid template must fail");
            assert!(error.to_string().contains(expected));
            assert!(error.to_string().contains(name));
        }
    }

    #[test]
    fn json_destinations_prepare_trusted_values_before_rendering() {
        let mut files = vec![IntegrationFile {
            path: "manifest.json".to_owned(),
            content: "{\"guidance\": {{repository_guidance}}}\n".to_owned(),
        }];
        let guidance = "quoted \"text\" and {{trusted_fragment}}";
        let workers = built_in_worker_profiles(WorkerHarness::Claude);
        realize_templates(
            Harness::Claude,
            Some(("AGENTS.md", guidance)),
            &workers,
            &mut files,
        )
        .expect("render JSON artifact");

        let manifest: Value = serde_json::from_str(&files[0].content).expect("rendered JSON");
        let rendered = manifest["guidance"].as_str().expect("guidance string");
        assert!(rendered.contains(guidance));
        assert!(rendered.contains("{{trusted_fragment}}"));

        files[0].content = "{\"version\": {{version}}\n".to_owned();
        let error = realize_templates(Harness::Claude, None, &workers, &mut files)
            .expect_err("invalid destination syntax must fail");
        assert!(error.to_string().contains("invalid JSON"));
        assert!(error.to_string().contains("manifest.json"));
    }
}

#![forbid(unsafe_code)]

mod config;
mod goal;
mod integrations;
mod project;
mod tasks;

use std::collections::BTreeMap;
use std::env;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;

use clap::{Parser, Subcommand, ValueEnum};
use serde_json::{Value, json};

use integrations::{SkillCommand, run_skill_command};

const SCHEMA_VERSION: u64 = 1;
const CHANGE_ID_TRAILER: &str = "Zdev-Change-Id";
const AGENT_INSTRUCTIONS: &str = "Zdev stores durable plans and tasks in `.zdev`; the coding harness shapes, implements, and independently verifies the work. Use the harness's zdev integration for work tracked there: `zdev status` and `zdev next` orient and select work, `zdev check` validates state, and `zdev commit` records verified staged changes. Do not edit generated task indexes. If the integration is unavailable or conflicts with repository setup, ask the user how to configure it.";

#[derive(Debug)]
pub struct ZdevError {
    message: String,
    details: Option<Value>,
}

impl ZdevError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            details: None,
        }
    }

    fn with_details(message: impl Into<String>, details: Value) -> Self {
        Self {
            message: message.into(),
            details: Some(details),
        }
    }

    fn io(context: impl Display, error: io::Error) -> Self {
        Self::new(format!("{context}: {error}"))
    }
}

impl Display for ZdevError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ZdevError {}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
}

#[derive(Debug, Parser)]
#[command(
    name = "zdev",
    version,
    about = "Keep development plans and tasks in the repository",
    long_about = "Keep development plans and tasks in plain files under .zdev.\n\nZdev creates and checks those files, selects ready tasks, checks area branches, installs coding-harness integrations, and adds stable IDs to Git commits.",
    after_help = "Start a repository:\n  1. Check your integration:       zdev skill check <HARNESS> --scope user\n  2. Choose personal, project, or pull-request record storage:\n     zdev init --record <POLICY>\n  3. Create or switch branches, then:\n     zdev area create <TAG> --title <TITLE> --objective <OBJECTIVE>\n\nIf the integration check reports missing or conflict, run `zdev skill install <HARNESS>`. Run `zdev <COMMAND> --help` for command-specific details."
)]
pub struct Cli {
    /// Use PATH as the repository root instead of discovering it from the current directory
    #[arg(long, global = true, value_name = "PATH")]
    root: Option<PathBuf>,
    /// Choose human-readable text or machine-readable JSON output
    #[arg(long, global = true, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Print concise instructions for a coding harness
    Instructions,
    /// Initialize zdev in this repository
    ///
    /// Creates .zdev/config.toml and records the checked-out branch as trunk when
    /// HEAD names a branch. On detached HEAD, trunk remains unbound until you
    /// run `zdev config trunk <BRANCH>`. This command does not install a
    /// coding-harness integration or create an area.
    /// Use --record personal to keep .zdev clone-local, project to keep it as
    /// lasting shared state, or pull-request to track it for review and remove
    /// it with `zdev cleanup squash` before squash merge.
    Init {
        /// How the .zdev planning record is stored and shared
        #[arg(long, value_enum, value_name = "POLICY")]
        record: project::RecordPolicy,
    },
    /// Remove pull-request-only zdev development records
    Cleanup {
        #[command(subcommand)]
        command: CleanupCommand,
    },
    /// Inspect layered configuration or set the project trunk
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },
    /// Create areas and manage their branches
    ///
    /// An area groups one objective, its brief, its tasks, and the branch that
    /// owns the work.
    Area {
        #[command(subcommand)]
        command: AreaCommand,
    },
    /// Create and inspect lightweight briefs within an area
    Slice {
        #[command(subcommand)]
        command: SliceCommand,
    },
    /// Review, import, list, and reindex an area's task files
    Tasks {
        #[command(subcommand)]
        command: TasksCommand,
    },
    /// Inspect or change the state of one task
    Task {
        #[command(subcommand)]
        command: TaskCommand,
    },
    /// Show the next ready task in an area
    ///
    /// Omit AREA only when zdev can select one unambiguously: a configured
    /// default area or the sole area with open work.
    Next {
        /// Area tag; omit to let zdev select an unambiguous active area
        area: Option<String>,
        /// Select ready work across all structurally safe areas
        #[arg(long, conflicts_with = "area")]
        any: bool,
    },
    /// Show a stable read-only projection of the next ready task in an area
    Goal {
        /// Area tag whose next ready task to project
        area: String,
    },
    /// Capture the validated task-work context for one area
    ///
    /// Closed areas return branch-independent goal context. Open areas also
    /// include matching status and complete staged, unstaged, and untracked Git
    /// evidence. This command does not change repository state.
    WorkContext {
        /// Area tag whose task-work context to capture
        area: String,
    },
    /// Show task counts and branch health
    ///
    /// With AREA, shows that area's ready, blocked, and completed task counts.
    /// Without AREA, shows the configured default area or a project-wide summary.
    Status {
        /// Area tag; omit for the configured default or a project-wide summary
        area: Option<String>,
    },
    /// Validate briefs, task files, indexes, and area relationships
    ///
    /// Checks existing files without rewriting them. Omit AREA to check every area.
    Check {
        /// Area tag; omit to check every area
        area: Option<String>,
    },
    /// Install or check a coding-harness integration
    ///
    /// Integrations teach Codex, Claude Code, OpenCode, Pi, or Oh My Pi how to
    /// use zdev. User-scoped integrations are shared across repositories;
    /// project-scoped integrations live in the current repository.
    Skill {
        #[command(subcommand)]
        command: SkillCommand,
    },
    /// Commit the staged Git changes and add a stable change ID
    ///
    /// Runs `git commit` for the existing index and appends a
    /// Zdev-Change-Id trailer. This command does not stage files.
    Commit {
        /// Git commit subject
        #[arg(short = 'm', long)]
        message: String,
        /// Commit-body paragraph; repeat for multiple paragraphs
        #[arg(long = "body")]
        body: Vec<String>,
    },
    /// Generate a new Zdev-Change-Id value
    ///
    /// Prints an ID without changing Git state. `zdev commit` generates and adds
    /// an ID automatically.
    ChangeId,
    /// Inspect or find commits by stable change ID
    Change {
        #[command(subcommand)]
        command: ChangeCommand,
    },
}

#[derive(Debug, Subcommand)]
enum ConfigCommand {
    /// Show effective configuration or values stored in one scope
    Show {
        /// Show only values stored in the global worker-profile file
        #[arg(long, conflicts_with = "local")]
        global: bool,
        /// Show only values stored in this repository
        #[arg(long, conflicts_with = "global")]
        local: bool,
    },
    /// Show one effective or scoped configuration value
    Get {
        /// Configuration key from the fixed project and worker registry
        key: String,
        /// Read only the global worker-profile file
        #[arg(long, conflicts_with = "local")]
        global: bool,
        /// Read only values stored in this repository
        #[arg(long, conflicts_with = "global")]
        local: bool,
    },
    /// Set one typed project or worker configuration value
    Set {
        /// Configuration key from the fixed project and worker registry
        key: String,
        /// Value, or model and effort for a worker profile
        #[arg(value_name = "VALUE", num_args = 1..)]
        values: Vec<String>,
        /// Write the global worker-profile file
        #[arg(long, conflicts_with = "local")]
        global: bool,
        /// Write this repository's configuration
        #[arg(long, conflicts_with = "global")]
        local: bool,
    },
    /// Remove one value from its selected scope
    Unset {
        /// Configuration key from the fixed project and worker registry
        key: String,
        /// Write the global worker-profile file
        #[arg(long, conflicts_with = "local")]
        global: bool,
        /// Write this repository's configuration
        #[arg(long, conflicts_with = "global")]
        local: bool,
    },
    /// Set the branch that areas use as their default base
    Trunk {
        /// Trunk branch name; omit to use the checked-out branch
        branch: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
enum CleanupCommand {
    /// Delete tracked .zdev files in one plain commit before a squash merge
    Squash,
}

#[derive(Debug, Subcommand)]
enum AreaCommand {
    /// Create an area for one objective on a branch
    ///
    /// Writes area.toml, brief.md, TASKS.md, and an empty tasks directory under
    /// .zdev/<TAG>. When --branch is omitted, the checked-out branch owns the area.
    Create {
        /// Short identifier used in paths and task IDs (lowercase letters, digits, and hyphens)
        tag: String,
        /// Human-readable area name
        #[arg(long)]
        title: String,
        /// One-line description of the outcome this area should achieve
        #[arg(long)]
        objective: String,
        /// Area branch; omit to use the checked-out branch
        #[arg(long)]
        branch: Option<String>,
    },
    /// Close an area whose task queue is empty or exhausted
    Close {
        /// Area tag to close
        area: String,
    },
    /// Reopen a closed area without changing its tasks
    Reopen {
        /// Area tag to reopen
        area: String,
    },
    /// Set the branch for an existing area
    Bind {
        /// Area tag to update
        area: String,
        /// Area branch; omit to use the checked-out branch
        branch: Option<String>,
    },
    /// Set or remove the area whose branch provides this area's base
    Parent {
        /// Child area tag to update
        area: String,
        /// Parent area tag; required unless --remove is used
        parent: Option<String>,
        /// Clear the area's current parent and use project trunk as its base
        #[arg(long, conflicts_with = "parent")]
        remove: bool,
    },
    /// Rebase an area's checked-out branch onto its current base
    ///
    /// The base is the parent area's branch, or project trunk when the area has
    /// no parent. The worktree must be clean for a new rebase.
    Rebase {
        /// Area tag whose branch is checked out
        area: String,
        /// Continue this area's stopped rebase after conflicts have been resolved and staged
        #[arg(long, conflicts_with = "abort")]
        r#continue: bool,
        /// Abort this area's stopped rebase and restore its previous branch state
        #[arg(long, conflicts_with = "continue")]
        abort: bool,
    },
}

#[derive(Debug, Subcommand)]
enum SliceCommand {
    /// Create a lightweight brief within an area
    Create {
        /// Area tag that owns the slice
        area: String,
        /// Lowercase identifier used as the slice filename
        key: String,
        /// Human-readable slice name
        #[arg(long)]
        title: String,
        /// Concise description of the slice outcome
        #[arg(long)]
        objective: String,
        /// Scope boundary; repeat for each boundary
        #[arg(long, required = true)]
        boundary: Vec<String>,
    },
    /// List every slice brief in an area
    List {
        /// Area tag whose slices to list
        area: String,
    },
    /// Show one slice brief
    Show {
        /// Area tag that owns the slice
        area: String,
        /// Slice key
        key: String,
    },
}

#[derive(Debug, Subcommand)]
enum TasksCommand {
    /// Render a JSON task bundle for human review
    ///
    /// Validates the bundle's shape, renders its complete Markdown review
    /// document, and returns an opaque fingerprint for `tasks import --approval`.
    Review {
        /// Area tag that will own the reviewed tasks
        area: String,
        /// JSON bundle path, or - to read the bundle from standard input
        #[arg(long = "from", value_name = "PATH_OR_DASH")]
        source: PathBuf,
    },
    /// Import a reviewed JSON task bundle into an area
    ///
    /// Creates one Markdown file per task and regenerates TASKS.md. Existing
    /// task files are preserved; conflicting task keys fail the import.
    Import {
        /// Area tag that will own the imported tasks
        area: String,
        /// JSON bundle path, or - to read the bundle from standard input
        #[arg(long = "from", value_name = "PATH_OR_DASH")]
        source: PathBuf,
        /// Commit only the imported task files and regenerated summary
        #[arg(long)]
        commit: bool,
        /// Opaque review fingerprint carried from `zdev tasks review`
        #[arg(long, value_name = "FINGERPRINT")]
        approval: Option<String>,
    },
    /// List every task in an area with its current state
    List {
        /// Area tag whose tasks to list
        area: String,
    },
    /// Regenerate an area's TASKS.md from its individual task files
    Index {
        /// Area tag whose task index to regenerate
        area: String,
    },
}

#[derive(Debug, Subcommand)]
enum TaskCommand {
    /// Print one task's complete Markdown file
    Show {
        /// Area tag that owns the task
        area: String,
        /// Task ID, such as scheduling-001
        task: String,
    },
    /// Mark a verified task complete and regenerate TASKS.md
    Done {
        /// Area tag that owns the task
        area: String,
        /// Task ID to complete, such as scheduling-001
        task: String,
        /// Concise description of the completed outcome
        #[arg(long)]
        summary: String,
        /// Validation result that passed; repeat for each independent check
        #[arg(long, required = true)]
        validation: Vec<String>,
    },
    /// Mark a completed task open again and regenerate TASKS.md
    Reopen {
        /// Area tag that owns the task
        area: String,
        /// Task ID to reopen, such as scheduling-001
        task: String,
    },
}

#[derive(Debug, Subcommand)]
enum ChangeCommand {
    /// Show a commit's Zdev-Change-Id and subject
    Inspect {
        /// Git revision to inspect, such as HEAD or a commit hash
        revision: String,
    },
    /// Find all reachable commits with a Zdev-Change-Id
    Lookup {
        /// Stable ID beginning with Z, as printed by `zdev change inspect`
        change_id: String,
    },
}

impl Cli {
    pub fn format(&self) -> OutputFormat {
        self.format
    }

    pub fn command_name(&self) -> &'static str {
        match self.command {
            Command::Instructions => "instructions",
            Command::Init { .. } => "init",
            Command::Cleanup { .. } => "cleanup",
            Command::Config { .. } => "config",
            Command::Area { .. } => "area",
            Command::Slice { .. } => "slice",
            Command::Tasks { .. } => "tasks",
            Command::Task { .. } => "task",
            Command::Next { .. } => "next",
            Command::Goal { .. } => "goal",
            Command::WorkContext { .. } => "work-context",
            Command::Status { .. } => "status",
            Command::Check { .. } => "check",
            Command::Skill { .. } => "skill",
            Command::Commit { .. } => "commit",
            Command::ChangeId => "change-id",
            Command::Change { .. } => "change",
        }
    }
}

pub struct CommandOutput {
    pub exit_code: u8,
    text: String,
    value: Value,
    json: Option<String>,
}

impl CommandOutput {
    fn new(text: impl Into<String>, value: Value) -> Self {
        Self {
            exit_code: 0,
            text: text.into(),
            value,
            json: None,
        }
    }

    fn with_json(mut self, json: String) -> Self {
        self.json = Some(json);
        self
    }
}

pub fn render_success(format: OutputFormat, output: &CommandOutput) {
    match format {
        OutputFormat::Text => println!("{}", output.text),
        OutputFormat::Json => match &output.json {
            Some(json) => println!("{json}"),
            None => println!("{}", serde_json::to_string_pretty(&output.value).unwrap()),
        },
    }
}

pub fn render_error(format: OutputFormat, command: &str, error: &ZdevError) {
    match format {
        OutputFormat::Text => eprintln!("error: {error}"),
        OutputFormat::Json => {
            let mut value = json!({
                "schema_version": SCHEMA_VERSION,
                "command": command,
                "ok": false,
                "error": error.to_string(),
            });
            if let Some(details) = &error.details {
                value["details"] = details.clone();
            }
            eprintln!("{}", serde_json::to_string_pretty(&value).unwrap());
        }
    }
}

pub fn run(cli: &Cli) -> Result<CommandOutput, ZdevError> {
    if let Command::Instructions = cli.command {
        return Ok(CommandOutput::new(
            AGENT_INSTRUCTIONS,
            json!({
                "schema_version": SCHEMA_VERSION,
                "agent_instructions": AGENT_INSTRUCTIONS,
            }),
        ));
    }
    if let Command::Skill { command } = &cli.command {
        return run_skill_command(cli.root.as_deref(), command);
    }
    if let Command::Config { command } = &cli.command {
        match command {
            ConfigCommand::Show { global: true, .. } => {
                return config::show(None, config::ConfigReadScope::Global);
            }
            ConfigCommand::Get {
                key, global: true, ..
            } => return config::get(None, config::ConfigReadScope::Global, key),
            ConfigCommand::Set {
                key,
                values,
                global: true,
                ..
            } => {
                return config::set(None, config::ConfigWriteScope::Global, key, values);
            }
            ConfigCommand::Unset {
                key, global: true, ..
            } => return config::unset(None, config::ConfigWriteScope::Global, key),
            _ => {}
        }
    }
    let root = resolve_root(cli.root.as_deref())?;
    match &cli.command {
        Command::Instructions => unreachable!(),
        Command::Init { record } => project::initialize(&root, *record),
        Command::Cleanup { command } => match command {
            CleanupCommand::Squash => project::cleanup_squash(&root),
        },
        Command::Config { command } => match command {
            ConfigCommand::Show { local, .. } => config::show(
                Some(&root),
                if *local {
                    config::ConfigReadScope::Local
                } else {
                    config::ConfigReadScope::Effective
                },
            ),
            ConfigCommand::Get { key, local, .. } => config::get(
                Some(&root),
                if *local {
                    config::ConfigReadScope::Local
                } else {
                    config::ConfigReadScope::Effective
                },
                key,
            ),
            ConfigCommand::Set { key, values, .. } => {
                config::set(Some(&root), config::ConfigWriteScope::Local, key, values)
            }
            ConfigCommand::Unset { key, .. } => {
                config::unset(Some(&root), config::ConfigWriteScope::Local, key)
            }
            ConfigCommand::Trunk { branch } => project::configure_trunk(&root, branch.as_deref()),
        },
        Command::Area { command } => match command {
            AreaCommand::Create {
                tag,
                title,
                objective,
                branch,
            } => project::create_area(&root, tag, title, objective, branch.as_deref()),
            AreaCommand::Close { area } => project::close_area(&root, area),
            AreaCommand::Reopen { area } => project::reopen_area(&root, area),
            AreaCommand::Bind { area, branch } => {
                project::bind_area(&root, area, branch.as_deref())
            }
            AreaCommand::Parent {
                area,
                parent,
                remove,
            } => project::configure_parent(&root, area, parent.as_deref(), *remove),
            AreaCommand::Rebase {
                area,
                r#continue,
                abort,
            } => project::rebase_area(&root, area, *r#continue, *abort),
        },
        Command::Slice { command } => match command {
            SliceCommand::Create {
                area,
                key,
                title,
                objective,
                boundary,
            } => project::create_slice(&root, area, key, title, objective, boundary),
            SliceCommand::List { area } => project::list_slices(&root, area),
            SliceCommand::Show { area, key } => project::show_slice(&root, area, key),
        },
        Command::Tasks { command } => match command {
            TasksCommand::Review { area, source } => tasks::review(&root, area, source),
            TasksCommand::Import {
                area,
                source,
                commit,
                approval,
            } => tasks::import(&root, area, source, *commit, approval.as_deref()),
            TasksCommand::List { area } => tasks::list(&root, area),
            TasksCommand::Index { area } => tasks::index(&root, area),
        },
        Command::Task { command } => match command {
            TaskCommand::Show { area, task } => tasks::show(&root, area, task),
            TaskCommand::Done {
                area,
                task,
                summary,
                validation,
            } => tasks::complete(&root, area, task, summary, validation),
            TaskCommand::Reopen { area, task } => tasks::reopen(&root, area, task),
        },
        Command::Next { area, any } => {
            if *any {
                tasks::next_any(&root)
            } else {
                tasks::next(&root, area.as_deref())
            }
        }
        Command::Goal { area } => goal::show(&root, area),
        Command::WorkContext { area } => work_context_output(&root, area),
        Command::Status { area } => status_output(&root, area.as_deref()),
        Command::Check { area } => check_output(&root, area.as_deref()),
        Command::Skill { .. } => unreachable!(),
        Command::Commit { message, body } => commit(&root, message, body),
        Command::ChangeId => {
            let id = generate_change_id()?;
            Ok(CommandOutput::new(
                format!("{CHANGE_ID_TRAILER}: {id}"),
                json!({"schema_version": SCHEMA_VERSION, "change_id": id, "trailer": format!("{CHANGE_ID_TRAILER}: {id}")}),
            ))
        }
        Command::Change { command } => match command {
            ChangeCommand::Inspect { revision } => inspect_change(&root, revision),
            ChangeCommand::Lookup { change_id } => lookup_change(&root, change_id),
        },
    }
}

fn resolve_root(explicit: Option<&Path>) -> Result<PathBuf, ZdevError> {
    let current = env::current_dir()
        .map_err(|error| ZdevError::io("Cannot read current directory", error))?;
    if let Some(path) = explicit {
        let path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            current.join(path)
        };
        return fs::canonicalize(&path)
            .map_err(|error| ZdevError::io(format!("Cannot resolve {}", path.display()), error));
    }
    if let Some(root) = current
        .ancestors()
        .find(|path| path.join(".zdev/config.toml").is_file())
    {
        return Ok(root.to_path_buf());
    }
    let output = ProcessCommand::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(&current)
        .output();
    if let Ok(output) = output
        && output.status.success()
    {
        let root = String::from_utf8_lossy(&output.stdout);
        return fs::canonicalize(root.trim())
            .map_err(|error| ZdevError::io("Cannot resolve Git root", error));
    }
    Ok(current)
}

fn validate_segment(value: &str, what: &str) -> Result<(), ZdevError> {
    if value.is_empty()
        || value.len() > 80
        || value.starts_with('-')
        || value.ends_with('-')
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    {
        return Err(ZdevError::new(format!(
            "{what} must contain lowercase letters, digits, and single hyphens"
        )));
    }
    Ok(())
}

fn validate_nonempty_line(value: &str, what: &str) -> Result<(), ZdevError> {
    if value.trim().is_empty() || value.contains(['\n', '\r']) {
        return Err(ZdevError::new(format!("{what} must be one non-empty line")));
    }
    Ok(())
}

fn git_operation(root: &Path) -> Result<Option<&'static str>, ZdevError> {
    for (marker, operation) in [
        ("rebase-merge", "rebase"),
        ("rebase-apply", "rebase"),
        ("MERGE_HEAD", "merge"),
        ("CHERRY_PICK_HEAD", "cherry-pick"),
        ("REVERT_HEAD", "revert"),
        ("BISECT_LOG", "bisect"),
        ("sequencer", "sequenced Git operation"),
    ] {
        let path = git_output(root, &["rev-parse", "--git-path", marker])?;
        let path = PathBuf::from(path);
        let path = if path.is_absolute() {
            path
        } else {
            root.join(path)
        };
        if path.exists() {
            return Ok(Some(operation));
        }
    }
    Ok(None)
}

struct ZdevStateLock {
    _file: Option<File>,
}

impl ZdevStateLock {
    fn acquire(root: &Path) -> Result<Self, ZdevError> {
        let output = ProcessCommand::new("git")
            .args(["rev-parse", "--git-path", "zdev-state.lock"])
            .current_dir(root)
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .output()
            .map_err(|error| ZdevError::io("Cannot inspect the Git state directory", error))?;
        if !output.status.success() {
            return Ok(Self { _file: None });
        }
        let git_path = PathBuf::from(String::from_utf8_lossy(&output.stdout).trim());
        let path = if git_path.is_absolute() {
            git_path
        } else {
            root.join(git_path)
        };
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)
            .map_err(|error| {
                ZdevError::io(
                    format!("Cannot open zdev state lock {}", path.display()),
                    error,
                )
            })?;
        for _ in 0..100 {
            match file.try_lock() {
                Ok(()) => {
                    if let Err(error) = file
                        .set_len(0)
                        .and_then(|()| writeln!(file, "{}", std::process::id()))
                    {
                        return Err(ZdevError::io(
                            format!("Cannot write zdev state lock {}", path.display()),
                            error,
                        ));
                    }
                    return Ok(Self { _file: Some(file) });
                }
                Err(std::fs::TryLockError::WouldBlock) => {
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
                Err(std::fs::TryLockError::Error(error)) => {
                    return Err(ZdevError::io(
                        format!("Cannot acquire zdev state lock {}", path.display()),
                        error,
                    ));
                }
            }
        }
        Err(ZdevError::new(format!(
            "Another zdev update is running. Retry when it finishes. Lock: {}",
            path.display(),
        )))
    }
}

fn render_area_branch_line(area: &project::AreaMetadata, status: &Value) -> String {
    let branch = &area.branch;
    let base = status["effective_base"]["branch"]
        .as_str()
        .unwrap_or("unbound");
    let relationship = match area.parent.as_deref() {
        Some(parent) => format!("parent {parent} on {base}"),
        None => format!("trunk {base}"),
    };
    let diagnostics = status["diagnostics"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect::<Vec<_>>()
        .join(", ");
    format!("{}: {branch} -> {relationship} [{diagnostics}]", area.tag)
}

fn required_json_string<'a>(
    value: &'a Value,
    pointer: &str,
    context: &str,
) -> Result<&'a str, ZdevError> {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .ok_or_else(|| ZdevError::new(format!("{context} lacks string field {pointer}")))
}

fn optional_task_id(
    value: &Value,
    pointer: &str,
    context: &str,
) -> Result<Option<String>, ZdevError> {
    match value.pointer(pointer) {
        Some(Value::Null) => Ok(None),
        Some(Value::String(id)) => Ok(Some(id.clone())),
        Some(Value::Object(task)) => task
            .get("id")
            .and_then(Value::as_str)
            .map(|id| Some(id.to_owned()))
            .ok_or_else(|| ZdevError::new(format!("{context} lacks string task ID"))),
        Some(_) => Err(ZdevError::new(format!(
            "{context} has invalid task field {pointer}"
        ))),
        None => Err(ZdevError::new(format!(
            "{context} lacks task field {pointer}"
        ))),
    }
}

fn work_context_git_stdout(root: &Path, arguments: &[&str]) -> Result<String, ZdevError> {
    let command = format!("git {}", arguments.join(" "));
    let output = ProcessCommand::new("git")
        .args(arguments)
        .current_dir(root)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .map_err(|error| ZdevError::io(format!("Cannot run {command}"), error))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(ZdevError::new(format!(
            "{command} failed{}",
            if stderr.trim().is_empty() {
                String::new()
            } else {
                format!(": {}", stderr.trim())
            }
        )));
    }
    String::from_utf8(output.stdout)
        .map_err(|_| ZdevError::new(format!("{command} returned invalid UTF-8")))
}

fn work_context_output(root: &Path, area: &str) -> Result<CommandOutput, ZdevError> {
    let goal = goal::show(root, area)?.value;
    let goal_area = required_json_string(&goal, "/area/tag", "Goal projection")?;
    if goal_area != area {
        return Err(ZdevError::new(format!(
            "Goal projection identifies area {goal_area}, expected {area}"
        )));
    }
    let lifecycle = required_json_string(&goal, "/lifecycle", "Goal projection")?;
    let queue = required_json_string(&goal, "/queue", "Goal projection")?;
    let goal_task = optional_task_id(&goal, "/task", "Goal projection")?;

    if lifecycle == "closed" {
        if goal_task.is_some() {
            return Err(ZdevError::new(format!(
                "Closed area {area} unexpectedly projects a task"
            )));
        }
        let text = format!(
            "Area: {area}\nLifecycle: closed\nQueue: {queue}\nTask: none\nNo branch or Git context is required for closed work."
        );
        return Ok(CommandOutput::new(
            text,
            json!({
                "schema_version": SCHEMA_VERSION,
                "area": area,
                "lifecycle": lifecycle,
                "queue": queue,
                "task_id": Value::Null,
                "goal": goal,
            }),
        ));
    }
    if lifecycle != "open" {
        return Err(ZdevError::new(format!(
            "Goal projection has unsupported lifecycle {lifecycle}"
        )));
    }

    let status = status_output(root, Some(area))?.value;
    let status_area = required_json_string(&status, "/area/tag", "Status projection")?;
    let status_lifecycle = required_json_string(&status, "/lifecycle", "Status projection")?;
    let status_queue = required_json_string(&status, "/queue", "Status projection")?;
    let status_task = optional_task_id(&status, "/next", "Status projection")?;
    if status_area != area
        || status_lifecycle != lifecycle
        || status_queue != queue
        || status_task != goal_task
    {
        return Err(ZdevError::with_details(
            format!("Status and goal disagree for area {area}; retry from fresh repository state"),
            json!({"area": area, "status": status, "goal": goal}),
        ));
    }
    let safe = status
        .pointer("/branch_status/task_work/safe")
        .and_then(Value::as_bool)
        .ok_or_else(|| ZdevError::new("Status projection lacks boolean task_work.safe"))?;
    let stale_advisory = status
        .pointer("/branch_status/task_work/stale_advisory")
        .and_then(Value::as_bool)
        .ok_or_else(|| {
            ZdevError::new("Status projection lacks boolean task_work.stale_advisory")
        })?;
    if !safe {
        return Err(ZdevError::with_details(
            format!(
                "Cannot capture task-work context for area {area}: task work is unsafe. Resolve the reported branch state and retry"
            ),
            json!({"area": area, "status": status, "goal": goal}),
        ));
    }

    let head = work_context_git_stdout(root, &["rev-parse", "HEAD"])?;
    let head = head.trim();
    if head.len() != 40
        || !head
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(ZdevError::new(
            "git rev-parse HEAD returned an invalid commit ID",
        ));
    }
    let git_status =
        work_context_git_stdout(root, &["status", "--short", "--untracked-files=all"])?;
    let git_diff_cached = work_context_git_stdout(root, &["diff", "--cached"])?;
    let git_diff = work_context_git_stdout(root, &["diff"])?;
    let task_id = goal_task.as_deref().map(Value::from).unwrap_or(Value::Null);
    let task_text = goal_task.as_deref().unwrap_or("none");
    let text = format!(
        "Area: {area}\nLifecycle: open\nQueue: {queue}\nTask: {task_text}\nStale advisory: {}\nCaptured status, goal, and complete Git evidence.",
        if stale_advisory { "yes" } else { "no" }
    );
    Ok(CommandOutput::new(
        text,
        json!({
            "schema_version": SCHEMA_VERSION,
            "area": area,
            "lifecycle": lifecycle,
            "queue": queue,
            "task_id": task_id,
            "stale_advisory": stale_advisory,
            "status": status,
            "goal": goal,
            "head": head,
            "git_status": git_status,
            "git_diff_cached": git_diff_cached,
            "git_diff": git_diff,
        }),
    ))
}

fn status_output(root: &Path, requested: Option<&str>) -> Result<CommandOutput, ZdevError> {
    let config = project::read_config(root)?;
    let all_areas = project::list_areas(root)?;
    let by_tag = all_areas
        .iter()
        .map(|area| (area.tag.as_str(), area))
        .collect::<BTreeMap<_, _>>();
    let checked_out = project::current_branch(root)?;
    let selected = match requested {
        Some(area) => Some(area.to_owned()),
        None => config.project.default_area.clone(),
    };
    if let Some(area) = selected {
        let metadata = project::load_area(root, &area)?.0;
        let tasks = tasks::summary(root, &area)?;
        tasks::validate_area_lifecycle(&metadata, &tasks)?;
        let lifecycle = metadata.lifecycle.as_str();
        let queue = tasks.queue().as_str();
        let branch_status =
            project::area_branch_status(root, &config, &metadata, &by_tag, checked_out.as_deref());
        let branch_line = render_area_branch_line(&metadata, &branch_status);
        let advisory = branch_status["task_work"]["stale_advisory"]
            .as_bool()
            .unwrap_or(false)
            .then(|| project::rebase_advisory(&area));
        let mut text = format!(
            "{}\nLifecycle: {lifecycle}\nQueue: {queue}\nCounts: {} total; {} ready; {} blocked; {} done",
            metadata.title, tasks.total, tasks.ready, tasks.blocked, tasks.done,
        );
        for slice in &tasks.slices {
            text.push_str(&format!(
                "\nSlice {}: {} ready, {} blocked, {} done",
                slice.key, slice.ready, slice.blocked, slice.done
            ));
        }
        text.push('\n');
        text.push_str(&branch_line);
        if let Some(advisory) = &advisory {
            text.push('\n');
            text.push_str(advisory);
        }
        return Ok(CommandOutput::new(
            text,
            json!({"schema_version": SCHEMA_VERSION, "project": config.project.name, "trunk": config.project.trunk, "area": metadata, "lifecycle": lifecycle, "queue": queue, "branch_status": branch_status, "counts": {"total": tasks.total, "ready": tasks.ready, "blocked": tasks.blocked, "done": tasks.done}, "slices": tasks.slices, "next": tasks.next, "advisory": advisory}),
        ));
    }
    let mut summaries = Vec::new();
    let mut branch_lines = Vec::new();
    for area in &all_areas {
        let tasks = tasks::summary(root, &area.tag)?;
        tasks::validate_area_lifecycle(area, &tasks)?;
        let lifecycle = area.lifecycle.as_str();
        let queue = tasks.queue().as_str();
        let branch_status =
            project::area_branch_status(root, &config, area, &by_tag, checked_out.as_deref());
        let branch_line = render_area_branch_line(area, &branch_status);
        let branch_detail = branch_line
            .strip_prefix(&format!("{}: ", area.tag))
            .unwrap_or(&branch_line);
        branch_lines.push(format!(
            "{}: {lifecycle}, {queue}; {branch_detail}",
            area.tag
        ));
        summaries.push(json!({
            "tag": area.tag,
            "title": area.title,
            "lifecycle": lifecycle,
            "queue": queue,
            "branch_status": branch_status,
            "total": tasks.total,
            "ready": tasks.ready,
            "blocked": tasks.blocked,
            "done": tasks.done,
        }));
    }
    let trunk = config.project.trunk.as_deref().unwrap_or("unbound");
    let mut text = format!(
        "{}: {} areas (trunk: {trunk})",
        config.project.name,
        summaries.len()
    );
    if !branch_lines.is_empty() {
        text.push('\n');
        text.push_str(&branch_lines.join("\n"));
    }
    Ok(CommandOutput::new(
        text,
        json!({"schema_version": SCHEMA_VERSION, "project": config.project.name, "trunk": config.project.trunk, "checked_out_branch": checked_out, "areas": summaries}),
    ))
}

fn check_output(root: &Path, requested: Option<&str>) -> Result<CommandOutput, ZdevError> {
    project::read_config(root)?;
    project::validate_area_relationships(&project::list_areas(root)?)?;
    let areas = if let Some(area) = requested {
        vec![project::load_area(root, area)?.0]
    } else {
        project::list_areas(root)?
    };
    let mut checked = Vec::new();
    for area in areas {
        validate_brief(&root.join(".zdev").join(&area.tag).join("brief.md"))?;
        project::validate_slices(root, &area.tag)?;
        tasks::validate_index(root, &area.tag)?;
        let summary = tasks::summary(root, &area.tag)?;
        tasks::validate_area_lifecycle(&area, &summary)?;
        checked.push(area.tag);
    }
    Ok(CommandOutput::new(
        format!("Checked {} areas", checked.len()),
        json!({"schema_version": SCHEMA_VERSION, "status": "ok", "areas": checked}),
    ))
}

pub(crate) fn validate_brief(path: &Path) -> Result<(), ZdevError> {
    let content = fs::read_to_string(path)
        .map_err(|error| ZdevError::io(format!("Cannot read {}", path.display()), error))?;
    required_brief_section(&content, "Objective", path)?;
    let testing = required_brief_section(&content, "Testing", path)?.to_ascii_lowercase();
    let levels = [
        "no new tests",
        "existing checks only",
        "focused coverage",
        "broader regression coverage",
    ];
    if !levels.iter().any(|level| testing.contains(level)) {
        return Err(ZdevError::new(format!(
            "Brief ## Testing must state one of: no new tests, existing checks only, focused coverage, or broader regression coverage: {}",
            path.display()
        )));
    }
    Ok(())
}

fn required_brief_section<'a>(
    content: &'a str,
    heading: &str,
    path: &Path,
) -> Result<&'a str, ZdevError> {
    let marker = format!("## {heading}");
    let mut found = false;
    let mut start = 0;
    let mut end = content.len();
    let mut offset = 0;

    for line in content.split_inclusive('\n') {
        let text = line.trim_end_matches(['\r', '\n']);
        if text.starts_with("## ") {
            if found && end == content.len() {
                end = offset;
            }
            if text == marker {
                if found {
                    return Err(ZdevError::new(format!(
                        "Brief repeats ## {heading}: {}",
                        path.display()
                    )));
                }
                found = true;
                start = offset + line.len();
                end = content.len();
            }
        }
        offset += line.len();
    }

    if !found {
        return Err(ZdevError::new(format!(
            "Brief lacks ## {heading}: {}",
            path.display()
        )));
    }
    let section = content[start..end].trim();
    if section.is_empty() {
        return Err(ZdevError::new(format!(
            "Brief has empty ## {heading}: {}",
            path.display()
        )));
    }
    Ok(section)
}

fn commit(root: &Path, message: &str, body: &[String]) -> Result<CommandOutput, ZdevError> {
    validate_nonempty_line(message, "Commit message")?;
    if body.iter().any(|paragraph| {
        paragraph.lines().any(|line| {
            line.split_once(':')
                .is_some_and(|(key, _)| key.trim().eq_ignore_ascii_case(CHANGE_ID_TRAILER))
        })
    }) {
        return Err(ZdevError::new(format!(
            "Commit body must not contain a {CHANGE_ID_TRAILER} trailer; zdev adds one automatically"
        )));
    }
    let _lock = ZdevStateLock::acquire(root)?;
    let change_id = generate_change_id()?;
    let mut command = ProcessCommand::new("git");
    command.arg("commit").arg("-m").arg(message);
    for paragraph in body {
        command.arg("-m").arg(paragraph);
    }
    command
        .arg("-m")
        .arg(format!("{CHANGE_ID_TRAILER}: {change_id}"));
    let output = command
        .current_dir(root)
        .output()
        .map_err(|error| ZdevError::io("Cannot run git commit", error))?;
    if !output.status.success() {
        return Err(ZdevError::new(format!(
            "git commit failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    let revision = git_output(root, &["rev-parse", "HEAD"])?;
    Ok(CommandOutput::new(
        format!("Committed {revision} ({change_id}): {message}"),
        json!({"schema_version": SCHEMA_VERSION, "status": "committed", "commit": revision, "change_id": change_id}),
    ))
}

fn generate_change_id() -> Result<String, ZdevError> {
    let mut random = [0_u8; 32];
    getrandom::fill(&mut random)
        .map_err(|error| ZdevError::new(format!("Cannot generate change ID: {error}")))?;
    let mut id = String::from("Z");
    for byte in random {
        use std::fmt::Write as _;
        write!(&mut id, "{byte:02x}").unwrap();
    }
    Ok(id)
}

fn valid_change_id(id: &str) -> bool {
    id.len() == 65
        && id.starts_with('Z')
        && id[1..]
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn inspect_change(root: &Path, revision: &str) -> Result<CommandOutput, ZdevError> {
    let commit = git_output(
        root,
        &["rev-parse", "--verify", &format!("{revision}^{{commit}}")],
    )?;
    let subject = git_output(root, &["show", "-s", "--format=%s", &commit])?;
    let trailers = git_output(
        root,
        &[
            "show",
            "-s",
            &format!("--format=%(trailers:key={CHANGE_ID_TRAILER},valueonly)"),
            &commit,
        ],
    )?;
    let ids = trailers
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>();
    let change_id = match ids.as_slice() {
        [] => None,
        [id] if valid_change_id(id) => Some((*id).to_owned()),
        [id] => return Err(ZdevError::new(format!("Invalid stable change ID: {id}"))),
        _ => return Err(ZdevError::new("Commit has more than one stable change ID")),
    };
    Ok(CommandOutput::new(
        match &change_id {
            Some(id) => format!("{commit}  {id}  {subject}"),
            None => format!("{commit} has no stable change ID"),
        },
        json!({"schema_version": SCHEMA_VERSION, "commit": commit, "subject": subject, "change_id": change_id}),
    ))
}

fn lookup_change(root: &Path, change_id: &str) -> Result<CommandOutput, ZdevError> {
    if !valid_change_id(change_id) {
        return Err(ZdevError::new(
            "Stable change ID must be Z followed by 64 lowercase hexadecimal characters",
        ));
    }
    let format = format!("--format=%H%x00%s%x00%(trailers:key={CHANGE_ID_TRAILER},valueonly)");
    let log = git_output_bytes(root, &["log", "--all", "-z", &format])?;
    let mut matches = Vec::new();
    let mut fields = log.split(|byte| *byte == 0);
    while let Some(commit) = fields.next() {
        if commit.is_empty() {
            break;
        }
        let subject = fields
            .next()
            .ok_or_else(|| ZdevError::new("Git log omitted a commit subject"))?;
        let trailers = fields
            .next()
            .ok_or_else(|| ZdevError::new("Git log omitted commit trailers"))?;
        let commit = String::from_utf8_lossy(commit).into_owned();
        let subject = String::from_utf8_lossy(subject).into_owned();
        if String::from_utf8_lossy(trailers)
            .lines()
            .any(|line| line == change_id)
        {
            matches.push(json!({
                "commit": commit,
                "subject": subject,
            }));
        }
    }
    let text = if matches.is_empty() {
        format!("No reachable commit has {change_id}. Check the ID or fetch the missing history")
    } else {
        matches
            .iter()
            .map(|value| {
                format!(
                    "{}  {}",
                    value["commit"].as_str().unwrap(),
                    value["subject"].as_str().unwrap()
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    Ok(CommandOutput::new(
        text,
        json!({"schema_version": SCHEMA_VERSION, "change_id": change_id, "commits": matches}),
    ))
}

fn git_output(root: &Path, arguments: &[&str]) -> Result<String, ZdevError> {
    let bytes = git_output_bytes(root, arguments)?;
    Ok(String::from_utf8_lossy(&bytes).trim().to_owned())
}

fn git_output_bytes(root: &Path, arguments: &[&str]) -> Result<Vec<u8>, ZdevError> {
    let output = ProcessCommand::new("git")
        .args(arguments)
        .current_dir(root)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .map_err(|error| ZdevError::io("Cannot run git", error))?;
    if !output.status.success() {
        return Err(ZdevError::new(format!(
            "git {} failed: {}",
            arguments.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    Ok(output.stdout)
}

fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), ZdevError> {
    let parent = path
        .parent()
        .ok_or_else(|| ZdevError::new(format!("Path has no parent: {}", path.display())))?;
    let mut stage = tempfile::NamedTempFile::new_in(parent)
        .map_err(|error| ZdevError::io(format!("Cannot stage {}", path.display()), error))?;
    stage
        .write_all(bytes)
        .and_then(|()| stage.as_file().sync_all())
        .map_err(|error| ZdevError::io(format!("Cannot write {}", path.display()), error))?;
    stage.persist(path).map_err(|error| {
        ZdevError::io(format!("Cannot publish {}", path.display()), error.error)
    })?;
    Ok(())
}

fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

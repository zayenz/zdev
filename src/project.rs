use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::{
    CommandOutput, SCHEMA_VERSION, ZdevError, ZdevStateLock, git_operation, git_output,
    git_output_bytes, relative, tasks, validate_nonempty_line, validate_segment, write_atomic,
};

const CLEANUP_SQUASH_COMMIT_MESSAGE: &str = "chore: remove zdev development record";

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(super) enum RecordPolicy {
    Personal,
    Project,
    PullRequest,
}

impl RecordPolicy {
    fn as_str(self) -> &'static str {
        match self {
            Self::Personal => "personal",
            Self::Project => "project",
            Self::PullRequest => "pull-request",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Config {
    pub(super) schema_version: u64,
    pub(super) project: ProjectConfig,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ProjectConfig {
    pub(super) name: String,
    pub(super) record: RecordPolicy,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) default_area: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) trunk: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) guidance: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct AreaMetadata {
    pub(super) schema_version: u64,
    pub(super) tag: String,
    pub(super) title: String,
    pub(super) objective: String,
    pub(super) branch: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) parent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) base_commit: Option<String>,
}

pub(super) fn initialize(root: &Path, record: RecordPolicy) -> Result<CommandOutput, ZdevError> {
    let state_dir = root.join(".zdev");
    if state_dir.exists() {
        let config = read_config(root)?;
        return Ok(CommandOutput::new(
            format!("zdev is already initialized for {}", config.project.name),
            json!({"schema_version": SCHEMA_VERSION, "status": "unchanged", "root": root}),
        ));
    }
    fs::create_dir(&state_dir)
        .map_err(|error| ZdevError::io(format!("Cannot create {}", state_dir.display()), error))?;
    let name = root
        .file_name()
        .and_then(OsStr::to_str)
        .filter(|name| !name.trim().is_empty())
        .ok_or_else(|| ZdevError::new("Repository has no usable directory name"))?;
    let config = Config {
        schema_version: SCHEMA_VERSION,
        project: ProjectConfig {
            name: name.to_owned(),
            record,
            default_area: None,
            trunk: current_branch(root)?,
            guidance: None,
        },
    };
    write_atomic(
        &state_dir.join("config.toml"),
        toml::to_string_pretty(&config).unwrap().as_bytes(),
    )?;
    let trunk = match config.project.trunk.as_deref() {
        Some(branch) => format!("Recorded {branch} as the project trunk."),
        None => "HEAD is detached, so no project trunk was recorded. Set one with:\n  zdev config trunk <branch>".to_owned(),
    };
    let record_text = match record {
        RecordPolicy::PullRequest => {
            "\nRecord policy: pull-request. .zdev is tracked for pull-request review and must be cleaned with `zdev cleanup squash` before squash merge.\n"
        }
        RecordPolicy::Personal => "\nRecord policy: personal. Keep .zdev local to this clone.\n",
        RecordPolicy::Project => "\nRecord policy: project. Track .zdev as shared project state.\n",
    };
    let record_json = if record == RecordPolicy::PullRequest {
        json!({
            "policy": record.as_str(),
            "tracked_for": "pull-request-review",
            "cleanup_required_before": "squash-merge",
            "cleanup_command": ["cleanup", "squash"],
            "notice": ".zdev is tracked for pull-request review and must be cleaned before squash merge"
        })
    } else {
        json!({"policy": record.as_str()})
    };
    Ok(CommandOutput::new(
        format!(
            "Initialized zdev for {name}.\nCreated .zdev/config.toml. {trunk}{record_text}\nCheck the integration you use:\n  zdev skill check <codex|claude|opencode|pi|omp> --scope user\n\nThen create or switch to a feature branch and run:\n  zdev area create <tag> --title <title> --objective <objective>"
        ),
        json!({
            "schema_version": SCHEMA_VERSION,
            "status": "created",
            "root": root,
            "record": record_json,
            "setup": {
                "status": "check-existing-user-integrations",
                "check_for": "each-requested-harness",
                "check_command": [
                    "skill", "check", "<harness>", "--scope", "user"
                ],
                "reuse_when": {
                    "status": "ok",
                    "action": "reuse-user-integration"
                },
                "install_when": [
                    "missing", "conflict", "project-scope-requested"
                ],
                "project_guidance_options": ["auto", "agents", "zdev", "PATH"],
                "install_command": [
                    "skill", "install", "<harness>", "--scope", "<scope>"
                ]
            }
        }),
    ))
}

pub(super) fn cleanup_squash(root: &Path) -> Result<CommandOutput, ZdevError> {
    let _lock = ZdevStateLock::acquire(root)?;
    let config = read_config(root)?;
    if config.project.record != RecordPolicy::PullRequest {
        return Err(ZdevError::new(format!(
            "Cannot clean the zdev record for squash: config record policy is {}, not pull-request",
            config.project.record.as_str()
        )));
    }
    if let Some(operation) = git_operation(root)? {
        return Err(ZdevError::new(format!(
            "Cannot clean the zdev record while a {operation} is in progress. Finish or abort it, then retry"
        )));
    }
    let branch = current_branch(root)?.ok_or_else(|| {
        ZdevError::new(
            "Cannot clean the zdev record without a checked-out branch; attach HEAD to the pull-request branch, then retry",
        )
    })?;
    if config.project.trunk.as_deref() == Some(branch.as_str()) {
        return Err(ZdevError::new(format!(
            "Cannot clean the zdev record on configured trunk {branch}; switch to the pull-request branch, then retry"
        )));
    }
    git_output(root, &["rev-parse", "--verify", "HEAD^{commit}"]).map_err(|_| {
        ZdevError::new(
            "Cannot clean the zdev record before the repository has a commit; commit the pull-request branch, then retry",
        )
    })?;
    if !git_output(root, &["status", "--porcelain=v1", "--untracked-files=all"])?.is_empty() {
        return Err(ZdevError::new(
            "Cannot clean the zdev record with local changes. Commit or resolve the index and worktree, then retry",
        ));
    }

    let tracked = git_output_bytes(root, &["ls-files", "-z", "--", ".zdev"])?
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .map(|path| {
            String::from_utf8(path.to_vec()).map_err(|_| {
                ZdevError::new("Cannot clean a tracked .zdev path whose name is not valid UTF-8")
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if tracked.is_empty() {
        return Err(ZdevError::new(
            "Cannot clean the zdev record because no tracked .zdev files exist on this branch",
        ));
    }

    let mut remove = ProcessCommand::new("git");
    remove.arg("rm").arg("--");
    remove.args(&tracked);
    let removed = remove
        .current_dir(root)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .map_err(|error| ZdevError::io("Cannot run git rm for tracked .zdev files", error))?;
    if !removed.status.success() {
        return Err(ZdevError::new(format!(
            "git rm for tracked .zdev files failed before the cleanup commit: {}",
            String::from_utf8_lossy(&removed.stderr).trim()
        )));
    }

    let committed = ProcessCommand::new("git")
        .args(["commit", "-m", CLEANUP_SQUASH_COMMIT_MESSAGE])
        .current_dir(root)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .map_err(|error| {
            ZdevError::io(
                "Cannot run the cleanup commit; the tracked .zdev deletion remains staged for inspection and recovery",
                error,
            )
        })?;
    if !committed.status.success() {
        return Err(ZdevError::new(format!(
            "Could not commit the cleanup. The tracked .zdev deletion remains staged for inspection and recovery: {}",
            String::from_utf8_lossy(&committed.stderr).trim()
        )));
    }
    let revision = git_output(root, &["rev-parse", "HEAD"])?;
    Ok(CommandOutput::new(
        format!(
            "Removed {} tracked .zdev files and committed {revision} on {branch}: {CLEANUP_SQUASH_COMMIT_MESSAGE}",
            tracked.len()
        ),
        json!({
            "schema_version": SCHEMA_VERSION,
            "status": "committed",
            "record": "pull-request",
            "cleanup": "squash",
            "branch": branch,
            "commit": revision,
            "message": CLEANUP_SQUASH_COMMIT_MESSAGE,
            "removed": tracked,
        }),
    ))
}

pub(super) fn read_config(root: &Path) -> Result<Config, ZdevError> {
    let path = root.join(".zdev/config.toml");
    let text = fs::read_to_string(&path)
        .map_err(|error| ZdevError::io(format!("Cannot read {}", path.display()), error))?;
    let config: Config = toml::from_str(&text)
        .map_err(|error| ZdevError::new(format!("Invalid {}: {error}", path.display())))?;
    if config.schema_version != SCHEMA_VERSION {
        return Err(ZdevError::new(format!(
            "Unsupported zdev schema version {}",
            config.schema_version
        )));
    }
    Ok(config)
}

pub(super) fn write_config(root: &Path, config: &Config) -> Result<(), ZdevError> {
    write_atomic(
        &root.join(".zdev/config.toml"),
        toml::to_string_pretty(config).unwrap().as_bytes(),
    )
}

pub(super) fn current_branch(root: &Path) -> Result<Option<String>, ZdevError> {
    let output = ProcessCommand::new("git")
        .args(["symbolic-ref", "--quiet", "--short", "HEAD"])
        .current_dir(root)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .map_err(|error| ZdevError::io("Cannot inspect the checked-out branch", error))?;
    if output.status.success() {
        let branch = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        return Ok((!branch.is_empty()).then_some(branch));
    }
    Ok(None)
}

fn canonical_branch(root: &Path, branch: &str) -> Result<String, ZdevError> {
    validate_nonempty_line(branch, "Branch")?;
    let output = ProcessCommand::new("git")
        .args(["check-ref-format", "--branch", branch])
        .current_dir(root)
        .output()
        .map_err(|error| ZdevError::io("Cannot validate branch name", error))?;
    if !output.status.success() {
        return Err(ZdevError::new(format!("Invalid branch name: {branch}")));
    }
    let canonical = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if canonical.is_empty() {
        return Err(ZdevError::new(format!("Invalid branch name: {branch}")));
    }
    Ok(canonical)
}

fn resolve_requested_branch(
    root: &Path,
    requested: Option<&str>,
) -> Result<Option<String>, ZdevError> {
    match requested {
        Some(branch) => canonical_branch(root, branch).map(Some),
        None => current_branch(root),
    }
}

fn required_requested_branch(root: &Path, requested: Option<&str>) -> Result<String, ZdevError> {
    resolve_requested_branch(root, requested)?
        .ok_or_else(|| ZdevError::new("No branch is checked out; pass an explicit branch name"))
}

pub(super) fn configure_trunk(
    root: &Path,
    requested: Option<&str>,
) -> Result<CommandOutput, ZdevError> {
    let mut config = read_config(root)?;
    let branch = required_requested_branch(root, requested)?;
    config.project.trunk = Some(branch.clone());
    write_config(root, &config)?;
    Ok(CommandOutput::new(
        format!("Configured project trunk {branch}"),
        json!({"schema_version": SCHEMA_VERSION, "status": "updated", "trunk": branch}),
    ))
}

fn area_path(root: &Path, tag: &str) -> Result<PathBuf, ZdevError> {
    validate_segment(tag, "Area tag")?;
    Ok(root.join(".zdev").join(tag))
}

pub(super) fn create_area(
    root: &Path,
    tag: &str,
    title: &str,
    objective: &str,
    requested_branch: Option<&str>,
) -> Result<CommandOutput, ZdevError> {
    let config = read_config(root)?;
    validate_nonempty_line(title, "Area title")?;
    validate_nonempty_line(objective, "Area objective")?;
    let path = area_path(root, tag)?;
    if path.exists() {
        return Err(ZdevError::new(format!("Area already exists: {tag}")));
    }
    let stage = tempfile::Builder::new()
        .prefix(".area-")
        .tempdir_in(root.join(".zdev"))
        .map_err(|error| ZdevError::io("Cannot stage area", error))?;
    fs::create_dir(stage.path().join("tasks"))
        .map_err(|error| ZdevError::io("Cannot create task directory", error))?;
    let branch = required_requested_branch(root, requested_branch)?;
    ensure_branch_available(root, &branch, None)?;
    let base_commit = config
        .project
        .trunk
        .as_deref()
        .map(|base| compute_base_anchor(root, &branch, base))
        .transpose()?
        .flatten();
    let metadata = AreaMetadata {
        schema_version: SCHEMA_VERSION,
        tag: tag.to_owned(),
        title: title.to_owned(),
        objective: objective.to_owned(),
        branch,
        parent: None,
        base_commit,
    };
    fs::write(
        stage.path().join("area.toml"),
        toml::to_string_pretty(&metadata).unwrap(),
    )
    .map_err(|error| ZdevError::io("Cannot write area metadata", error))?;
    fs::write(
        stage.path().join("brief.md"),
        format!(
            "# {title}\n\n## Objective\n\n{objective}\n\n## Testing\n\nExisting checks only.\n"
        ),
    )
    .map_err(|error| ZdevError::io("Cannot write area brief", error))?;
    let empty = tasks::empty_index(tag)?;
    fs::write(stage.path().join("TASKS.md"), empty)
        .map_err(|error| ZdevError::io("Cannot write task summary", error))?;
    fs::rename(stage.keep(), &path)
        .map_err(|error| ZdevError::io(format!("Cannot publish area {tag}"), error))?;
    Ok(CommandOutput::new(
        format!("Created area {tag}"),
        json!({"schema_version": SCHEMA_VERSION, "status": "created", "area": tag, "branch": metadata.branch, "path": relative(root, &path)}),
    ))
}

pub(super) fn load_area(root: &Path, tag: &str) -> Result<(AreaMetadata, PathBuf), ZdevError> {
    let path = area_path(root, tag)?;
    let metadata_path = path.join("area.toml");
    let text = fs::read_to_string(&metadata_path).map_err(|error| {
        ZdevError::io(format!("Cannot read {}", metadata_path.display()), error)
    })?;
    let metadata: AreaMetadata = toml::from_str(&text)
        .map_err(|error| ZdevError::new(format!("Invalid {}: {error}", metadata_path.display())))?;
    if metadata.schema_version != SCHEMA_VERSION || metadata.tag != tag {
        return Err(ZdevError::new(format!("Invalid area identity for {tag}")));
    }
    Ok((metadata, path))
}

pub(super) fn list_areas(root: &Path) -> Result<Vec<AreaMetadata>, ZdevError> {
    read_config(root)?;
    let mut areas = Vec::new();
    for entry in fs::read_dir(root.join(".zdev"))
        .map_err(|error| ZdevError::io("Cannot list zdev areas", error))?
    {
        let entry = entry.map_err(|error| ZdevError::io("Cannot inspect zdev area", error))?;
        let name = entry.file_name();
        let Some(tag) = name.to_str() else { continue };
        if tag.starts_with('_') || !entry.path().join("area.toml").is_file() {
            continue;
        }
        areas.push(load_area(root, tag)?.0);
    }
    areas.sort_by(|left, right| left.tag.cmp(&right.tag));
    Ok(areas)
}

fn write_area_metadata(root: &Path, area: &AreaMetadata) -> Result<(), ZdevError> {
    let path = area_path(root, &area.tag)?.join("area.toml");
    write_atomic(&path, toml::to_string_pretty(area).unwrap().as_bytes())
}

fn ensure_branch_available(
    root: &Path,
    branch: &str,
    owner: Option<&str>,
) -> Result<(), ZdevError> {
    if let Some(area) = list_areas(root)?
        .into_iter()
        .find(|area| area.branch == branch && owner != Some(area.tag.as_str()))
    {
        return Err(ZdevError::new(format!(
            "Branch {branch} is already owned by area {}",
            area.tag
        )));
    }
    Ok(())
}

pub(super) fn bind_area(
    root: &Path,
    tag: &str,
    requested: Option<&str>,
) -> Result<CommandOutput, ZdevError> {
    let config = read_config(root)?;
    let (mut area, _) = load_area(root, tag)?;
    let branch = required_requested_branch(root, requested)?;
    ensure_branch_available(root, &branch, Some(tag))?;
    area.branch = branch.clone();
    if let Some(anchor) = &area.base_commit {
        if local_branch_exists(root, &branch)
            && commit_is_ancestor(root, anchor, &format!("refs/heads/{branch}")) != Some(true)
        {
            return Err(ZdevError::new(format!(
                "Branch {branch} does not contain area {tag}'s recorded base commit {anchor}"
            )));
        }
    } else {
        area.base_commit = configured_effective_base(root, &config, &area)?
            .map(|base| compute_base_anchor(root, &branch, &base))
            .transpose()?
            .flatten();
    }
    write_area_metadata(root, &area)?;
    Ok(CommandOutput::new(
        format!("Bound area {tag} to branch {branch}"),
        json!({"schema_version": SCHEMA_VERSION, "status": "updated", "area": tag, "branch": branch}),
    ))
}

pub(super) fn validate_area_relationships(areas: &[AreaMetadata]) -> Result<(), ZdevError> {
    let by_tag = areas
        .iter()
        .map(|area| (area.tag.as_str(), area))
        .collect::<BTreeMap<_, _>>();
    let mut branches = BTreeMap::new();
    for area in areas {
        if let Some(other) = branches.insert(area.branch.as_str(), area.tag.as_str()) {
            return Err(ZdevError::new(format!(
                "Branch {} is owned by both {other} and {}",
                area.branch, area.tag
            )));
        }
        if area.parent.as_deref() == Some(area.tag.as_str()) {
            return Err(ZdevError::new(format!(
                "Area {} cannot be its own parent",
                area.tag
            )));
        }
        if let Some(parent) = &area.parent
            && !by_tag.contains_key(parent.as_str())
        {
            return Err(ZdevError::new(format!(
                "Area {} has unknown parent {parent}",
                area.tag
            )));
        }
    }
    for area in areas {
        let mut seen = BTreeSet::new();
        let mut current = area;
        while let Some(parent) = &current.parent {
            if !seen.insert(current.tag.as_str()) {
                return Err(ZdevError::new(format!(
                    "Area dependency cycle includes {}",
                    current.tag
                )));
            }
            current = by_tag[parent.as_str()];
        }
    }
    Ok(())
}

pub(super) fn configure_parent(
    root: &Path,
    tag: &str,
    requested_parent: Option<&str>,
    remove: bool,
) -> Result<CommandOutput, ZdevError> {
    if !remove && requested_parent.is_none() {
        return Err(ZdevError::new(
            "Pass a parent area or use --remove to clear it",
        ));
    }
    let (mut area, _) = load_area(root, tag)?;
    let previous_parent = area.parent.clone();
    let (parent, parent_area) = if remove {
        (None, None)
    } else {
        let parent = requested_parent.unwrap();
        let parent_area = load_area(root, parent)?.0;
        (Some(parent.to_owned()), Some(parent_area))
    };
    area.parent = parent.clone();
    if let Some(parent_area) = &parent_area {
        let child_branch = area.branch.clone();
        let parent_branch = parent_area.branch.clone();
        require_existing_branch(root, &child_branch, "area")?;
        require_existing_branch(root, &parent_branch, "parent-area")?;
        if previous_parent.is_none()
            && base_is_ancestor(root, &parent_branch, &child_branch) == Some(true)
        {
            area.base_commit = branch_tip(root, &parent_branch)?;
        } else {
            let anchor = require_base_anchor(&area)?;
            if !commit_exists(root, anchor)
                || commit_is_ancestor(root, anchor, &format!("refs/heads/{child_branch}"))
                    != Some(true)
            {
                return Err(ZdevError::new(format!(
                    "Cannot set parent {parent_branch}: area {tag} has no trustworthy base boundary contained in {child_branch}"
                )));
            }
        }
    }
    let mut areas = list_areas(root)?;
    let candidate = areas
        .iter_mut()
        .find(|candidate| candidate.tag == tag)
        .ok_or_else(|| ZdevError::new(format!("Unknown area: {tag}")))?;
    *candidate = area.clone();
    validate_area_relationships(&areas)?;
    write_area_metadata(root, &area)?;
    let text = match &parent {
        Some(parent) => format!("Set parent of {tag} to {parent}"),
        None => format!("Removed parent of {tag}"),
    };
    Ok(CommandOutput::new(
        text,
        json!({"schema_version": SCHEMA_VERSION, "status": "updated", "area": tag, "parent": parent}),
    ))
}

fn configured_effective_base(
    root: &Path,
    config: &Config,
    area: &AreaMetadata,
) -> Result<Option<String>, ZdevError> {
    if let Some(parent) = &area.parent {
        let parent_area = load_area(root, parent)?.0;
        return Ok(Some(parent_area.branch));
    }
    Ok(config.project.trunk.clone())
}

fn effective_base(root: &Path, config: &Config, area: &AreaMetadata) -> Result<String, ZdevError> {
    configured_effective_base(root, config, area)?.ok_or_else(|| {
        ZdevError::new("Project trunk is not configured; set it with `zdev config trunk <branch>`")
    })
}

fn branch_tip(root: &Path, branch: &str) -> Result<Option<String>, ZdevError> {
    if !local_branch_exists(root, branch) {
        return Ok(None);
    }
    git_output(
        root,
        &[
            "rev-parse",
            "--verify",
            &format!("refs/heads/{branch}^{{commit}}"),
        ],
    )
    .map(Some)
}

fn compute_base_anchor(root: &Path, branch: &str, base: &str) -> Result<Option<String>, ZdevError> {
    let Some(base_tip) = branch_tip(root, base)? else {
        return Ok(None);
    };
    let Some(branch_tip) = branch_tip(root, branch)? else {
        return Ok(Some(base_tip));
    };
    if base_is_ancestor(root, base, branch) == Some(true) {
        return Ok(Some(base_tip));
    }
    let output = ProcessCommand::new("git")
        .args(["merge-base", &branch_tip, &base_tip])
        .current_dir(root)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .map_err(|error| ZdevError::io("Cannot determine the incorporated base commit", error))?;
    if !output.status.success() {
        return Err(ZdevError::new(format!(
            "Branch {branch} and effective base {base} have no common commit"
        )));
    }
    let anchor = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    Ok((!anchor.is_empty()).then_some(anchor))
}

fn require_base_anchor(area: &AreaMetadata) -> Result<&str, ZdevError> {
    area.base_commit.as_deref().ok_or_else(|| {
        ZdevError::new(format!(
            "Area {} has no recorded base commit; rebind its branch with `zdev area bind {} <branch>`",
            area.tag, area.tag
        ))
    })
}

fn commit_exists(root: &Path, commit: &str) -> bool {
    ProcessCommand::new("git")
        .args(["cat-file", "-e", &format!("{commit}^{{commit}}")])
        .current_dir(root)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .status()
        .is_ok_and(|status| status.success())
}

pub(super) fn require_checked_out_area_branch(
    root: &Path,
    area: &AreaMetadata,
) -> Result<String, ZdevError> {
    let branch = &area.branch;
    let checked_out = current_branch(root)?.ok_or_else(|| {
        ZdevError::new(format!(
            "Cannot use area {} while HEAD is detached. Check out branch {branch} and retry",
            area.tag
        ))
    })?;
    if checked_out != *branch {
        return Err(ZdevError::new(format!(
            "Cannot use area {} on branch {checked_out}. Switch to {branch} and retry",
            area.tag
        )));
    }
    Ok(branch.to_owned())
}

fn require_existing_branch(root: &Path, branch: &str, role: &str) -> Result<(), ZdevError> {
    if !local_branch_exists(root, branch) {
        return Err(ZdevError::new(format!(
            "Cannot continue because the recorded {role} branch {branch} is missing locally. Restore or create it, then retry"
        )));
    }
    Ok(())
}

fn child_side_has_merges(root: &Path, anchor: &str, branch: &str) -> Result<bool, ZdevError> {
    let range = format!("{anchor}..refs/heads/{branch}");
    let count = git_output(root, &["rev-list", "--count", "--merges", &range])?;
    Ok(count != "0")
}

fn require_linear_child_history(root: &Path, anchor: &str, branch: &str) -> Result<(), ZdevError> {
    if child_side_has_merges(root, anchor, branch)? {
        return Err(ZdevError::new(format!(
            "Cannot manage area branch {branch}: it contains merge commits after base {anchor}. Use rebase-only history for this branch"
        )));
    }
    Ok(())
}

pub(super) struct TaskWorkBranchState {
    pub branch_status: Value,
    pub advisory: Option<String>,
}

pub(super) fn rebase_advisory(tag: &str) -> String {
    format!("Advisory: run `zdev area rebase {tag}` when you need current base changes")
}

pub(super) fn require_task_work_area_link(
    root: &Path,
    tag: &str,
) -> Result<TaskWorkBranchState, ZdevError> {
    let config = read_config(root)?;
    let area = load_area(root, tag)?.0;
    let areas = list_areas(root)?;
    let by_tag = areas
        .iter()
        .map(|area| (area.tag.as_str(), area))
        .collect::<BTreeMap<_, _>>();
    let checked_out = current_branch(root)?;
    let branch_status = area_branch_status(root, &config, &area, &by_tag, checked_out.as_deref());
    let blocked = |message: String| {
        ZdevError::with_details(
            message,
            json!({"area": tag, "branch_status": branch_status.clone()}),
        )
    };

    if let Some(operation) = git_operation(root).map_err(|error| blocked(error.to_string()))? {
        return Err(blocked(format!(
            "Cannot use area {tag} while a {operation} is in progress. Finish or abort it, then retry"
        )));
    }
    let branch =
        require_checked_out_area_branch(root, &area).map_err(|error| blocked(error.to_string()))?;
    let base = effective_base(root, &config, &area).map_err(|error| blocked(error.to_string()))?;
    if branch == base {
        return Ok(TaskWorkBranchState {
            branch_status,
            advisory: None,
        });
    }
    require_existing_branch(root, &branch, "area").map_err(|error| blocked(error.to_string()))?;
    require_existing_branch(root, &base, "effective-base")
        .map_err(|error| blocked(error.to_string()))?;
    let anchor = require_base_anchor(&area).map_err(|error| blocked(error.to_string()))?;
    if !commit_exists(root, anchor)
        || commit_is_ancestor(root, anchor, &format!("refs/heads/{branch}")) != Some(true)
    {
        return Err(blocked(format!(
            "Cannot verify area {tag}'s base on branch {branch}. Rebind the area with `zdev area bind {tag} <branch>`"
        )));
    }
    require_linear_child_history(root, anchor, &branch)
        .map_err(|error| blocked(error.to_string()))?;
    match base_is_ancestor(root, &base, &branch) {
        Some(true) => {
            let base_tip = branch_tip(root, &base)
            .map_err(|error| blocked(error.to_string()))?
            .ok_or_else(|| {
                blocked(format!(
                    "Cannot inspect the effective-base tip for {base}. Restore the local branch, then retry"
                ))
            })?;
            if anchor != base_tip {
                return Err(blocked(format!(
                    "Area {tag} needs its current base recorded. Run `zdev area rebase {tag}`"
                )));
            }
            Ok(TaskWorkBranchState {
                branch_status,
                advisory: None,
            })
        }
        Some(false) => Ok(TaskWorkBranchState {
            branch_status,
            advisory: Some(rebase_advisory(tag)),
        }),
        None => Err(blocked(format!(
            "Cannot inspect ancestry between area branch {branch} and effective base {base}. Restore inspectable local branches, then retry"
        ))),
    }
}

fn require_clean_worktree(root: &Path) -> Result<(), ZdevError> {
    if !git_output(root, &["status", "--porcelain=v1", "--untracked-files=all"])?.is_empty() {
        return Err(ZdevError::new(
            "Cannot rebase with local changes. Commit or stash them, then retry",
        ));
    }
    Ok(())
}

struct GitRebaseState {
    head_name: String,
    onto: String,
    orig_head: String,
}

fn git_path(root: &Path, name: &str) -> Result<PathBuf, ZdevError> {
    let path = PathBuf::from(git_output(root, &["rev-parse", "--git-path", name])?);
    Ok(if path.is_absolute() {
        path
    } else {
        root.join(path)
    })
}

fn read_git_rebase_state(root: &Path) -> Result<Option<GitRebaseState>, ZdevError> {
    let directory = ["rebase-merge", "rebase-apply"]
        .into_iter()
        .map(|name| git_path(root, name))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .find(|path| path.is_dir());
    let Some(directory) = directory else {
        return Ok(None);
    };
    let read_field = |name: &str| -> Result<String, ZdevError> {
        let path = directory.join(name);
        fs::read_to_string(&path)
            .map(|value| value.trim().to_owned())
            .map_err(|error| ZdevError::io(format!("Cannot read {}", path.display()), error))
    };
    Ok(Some(GitRebaseState {
        head_name: read_field("head-name")?,
        onto: read_field("onto")?,
        orig_head: read_field("orig-head")?,
    }))
}

fn managed_rebase_guidance(tag: &str) -> String {
    format!(
        "Resolve and stage the conflicts, then run `zdev area rebase {tag} --continue`. To cancel, run `zdev area rebase {tag} --abort`. The equivalent Git commands are `git rebase --continue` and `git rebase --abort`"
    )
}

fn finalize_managed_rebase(
    root: &Path,
    tag: &str,
    effective_base: &str,
    new_base_commit: &str,
) -> Result<CommandOutput, ZdevError> {
    let (mut area, _) = load_area(root, tag)?;
    let branch = require_checked_out_area_branch(root, &area)?;
    if commit_is_ancestor(root, new_base_commit, &format!("refs/heads/{branch}")) != Some(true) {
        return Err(ZdevError::new(format!(
            "Cannot finish the rebase: branch {branch} does not contain target {}. Inspect the Git history before retrying",
            new_base_commit
        )));
    }
    require_linear_child_history(root, new_base_commit, &branch)?;
    area.base_commit = Some(new_base_commit.to_owned());
    write_area_metadata(root, &area)?;
    Ok(CommandOutput::new(
        format!("Rebased area {tag} onto {effective_base}"),
        json!({"schema_version": SCHEMA_VERSION, "status": "rebased", "area": tag, "branch": branch, "effective_base": effective_base, "base_commit": new_base_commit, "fresh": true}),
    ))
}

fn continue_managed_rebase(root: &Path, tag: &str) -> Result<CommandOutput, ZdevError> {
    let state = read_git_rebase_state(root)?.ok_or_else(|| {
        ZdevError::new(format!(
            "Cannot continue: no rebase is in progress for area {tag}"
        ))
    })?;
    let config = read_config(root)?;
    let area = load_area(root, tag)?.0;
    let branch = &area.branch;
    if state.head_name != format!("refs/heads/{branch}") {
        return Err(ZdevError::new(format!(
            "Cannot continue area {tag}: Git is rebasing {}, not branch {branch}. Finish or abort that rebase first",
            state.head_name
        )));
    }
    let anchor = require_base_anchor(&area)?;
    if commit_is_ancestor(root, anchor, &state.orig_head) != Some(true)
        || !commit_exists(root, &state.onto)
    {
        return Err(ZdevError::new(
            "Cannot continue: Git's rebase boundary does not match the area's recorded base. Abort and restart the area rebase",
        ));
    }
    let effective_base = effective_base(root, &config, &area)?;
    let expected_onto = branch_tip(root, &effective_base)?.ok_or_else(|| {
        ZdevError::new(format!(
            "Cannot continue: base branch {effective_base} is missing locally. Restore it, abort this rebase, and retry"
        ))
    })?;
    let original_branch_tip = branch_tip(root, branch)?;
    if expected_onto != state.onto
        || original_branch_tip.as_deref() != Some(state.orig_head.as_str())
    {
        return Err(ZdevError::new(format!(
            "Cannot continue: Git's rebase state no longer matches area {tag}. Run `zdev area rebase {tag} --abort`, then start the rebase again"
        )));
    }
    let output = ProcessCommand::new("git")
        .args(["-c", "core.editor=true", "rebase", "--continue"])
        .current_dir(root)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .map_err(|error| ZdevError::io("Cannot continue git rebase", error))?;
    if !output.status.success() {
        let details = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        if git_operation(root)? == Some("rebase") {
            return Err(ZdevError::new(format!(
                "Area {tag} still has rebase conflicts. {}. {details}",
                managed_rebase_guidance(tag)
            )));
        }
        return Err(ZdevError::new(format!(
            "Could not continue area {tag}'s rebase, and Git left no rebase to resume. Inspect the branch before retrying: {details}"
        )));
    }
    finalize_managed_rebase(root, tag, &effective_base, &state.onto)
}

fn abort_managed_rebase(root: &Path, tag: &str) -> Result<CommandOutput, ZdevError> {
    let state = read_git_rebase_state(root)?.ok_or_else(|| {
        ZdevError::new(format!(
            "Cannot abort: no rebase is in progress for area {tag}"
        ))
    })?;
    let area = load_area(root, tag)?.0;
    let branch = &area.branch;
    if state.head_name != format!("refs/heads/{branch}") {
        return Err(ZdevError::new(format!(
            "Cannot abort area {tag}: Git is rebasing {}, not branch {branch}. Finish or abort that rebase directly",
            state.head_name
        )));
    }
    let output = ProcessCommand::new("git")
        .args(["rebase", "--abort"])
        .current_dir(root)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .map_err(|error| ZdevError::io("Cannot abort git rebase", error))?;
    if !output.status.success() {
        return Err(ZdevError::new(format!(
            "Could not abort area {tag}'s rebase. Run `git rebase --abort` after resolving this error: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    Ok(CommandOutput::new(
        format!("Aborted rebase of area {tag}"),
        json!({"schema_version": SCHEMA_VERSION, "status": "aborted", "area": tag, "base_commit": area.base_commit}),
    ))
}

pub(super) fn rebase_area(
    root: &Path,
    tag: &str,
    continue_rebase: bool,
    abort: bool,
) -> Result<CommandOutput, ZdevError> {
    if continue_rebase {
        return continue_managed_rebase(root, tag);
    }
    if abort {
        return abort_managed_rebase(root, tag);
    }
    let config = read_config(root)?;
    let area = load_area(root, tag)?.0;
    if let Some(operation) = git_operation(root)? {
        return Err(ZdevError::new(format!(
            "Cannot rebase area {tag} while a {operation} is in progress. Finish or abort it, then retry"
        )));
    }
    let branch = require_checked_out_area_branch(root, &area)?;
    let base = effective_base(root, &config, &area)?;
    let anchor = require_base_anchor(&area)?.to_owned();
    require_clean_worktree(root)?;
    require_existing_branch(root, &branch, "area")?;
    require_existing_branch(root, &base, "effective-base")?;
    let new_base_commit = branch_tip(root, &base)?.unwrap();
    if base_is_ancestor(root, &base, &branch) == Some(true) {
        require_linear_child_history(root, &new_base_commit, &branch)?;
        if area.base_commit.as_deref() == Some(new_base_commit.as_str()) {
            return Ok(CommandOutput::new(
                format!("Area {tag} is already fresh on {base}"),
                json!({"schema_version": SCHEMA_VERSION, "status": "unchanged", "area": tag, "branch": branch, "effective_base": base, "base_commit": new_base_commit, "fresh": true}),
            ));
        }
        let mut area = area;
        area.base_commit = Some(new_base_commit.clone());
        write_area_metadata(root, &area)?;
        return Ok(CommandOutput::new(
            format!("Finalized area {tag}'s incorporated base at {base}"),
            json!({"schema_version": SCHEMA_VERSION, "status": "updated", "area": tag, "branch": branch, "effective_base": base, "base_commit": new_base_commit, "fresh": true}),
        ));
    }
    if !commit_exists(root, &anchor)
        || commit_is_ancestor(root, &anchor, &format!("refs/heads/{branch}")) != Some(true)
    {
        return Err(ZdevError::new(format!(
            "Cannot rebase area {tag}: its recorded base is unavailable on {branch}. Rebind it with `zdev area bind {tag} <branch>`"
        )));
    }
    require_linear_child_history(root, &anchor, &branch)?;
    let output = ProcessCommand::new("git")
        .args(["rebase", "--onto", &new_base_commit, &anchor])
        .current_dir(root)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .map_err(|error| ZdevError::io("Cannot run git rebase", error))?;
    if !output.status.success() {
        let details = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        if git_operation(root)? == Some("rebase") {
            return Err(ZdevError::new(format!(
                "Area {tag} has rebase conflicts. {}. {details}",
                managed_rebase_guidance(tag)
            )));
        }
        return Err(ZdevError::new(format!(
            "Could not rebase area {tag}, and Git left no rebase to resume. Inspect the branch before retrying: {details}"
        )));
    }
    finalize_managed_rebase(root, tag, &base, &new_base_commit)
}

fn local_branch_exists(root: &Path, branch: &str) -> bool {
    ProcessCommand::new("git")
        .args([
            "show-ref",
            "--verify",
            "--quiet",
            &format!("refs/heads/{branch}"),
        ])
        .current_dir(root)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .status()
        .is_ok_and(|status| status.success())
}

fn base_is_ancestor(root: &Path, base: &str, branch: &str) -> Option<bool> {
    commit_is_ancestor(
        root,
        &format!("refs/heads/{base}"),
        &format!("refs/heads/{branch}"),
    )
}

fn commit_is_ancestor(root: &Path, ancestor: &str, descendant: &str) -> Option<bool> {
    let status = ProcessCommand::new("git")
        .args(["merge-base", "--is-ancestor", ancestor, descendant])
        .current_dir(root)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .status()
        .ok()?;
    match status.code() {
        Some(0) => Some(true),
        Some(1) => Some(false),
        _ => None,
    }
}

pub(super) fn area_branch_status(
    root: &Path,
    config: &Config,
    area: &AreaMetadata,
    areas: &BTreeMap<&str, &AreaMetadata>,
    checked_out: Option<&str>,
) -> Value {
    let mut diagnostics = Vec::new();
    let (operation, git_state_inspectable) = match git_operation(root) {
        Ok(operation) => (operation, true),
        Err(_) => {
            diagnostics.push("git-state-unavailable");
            (None, false)
        }
    };
    if operation.is_some() {
        diagnostics.push("git-operation-in-progress");
    }
    let branch_matches = checked_out.map(|checked_out| area.branch == checked_out);
    if branch_matches == Some(false) {
        diagnostics.push("wrong-branch");
    } else if checked_out.is_none() {
        diagnostics.push("detached-head");
    }

    let (base_kind, base_area, effective_base) = if let Some(parent) = &area.parent {
        match areas.get(parent.as_str()) {
            Some(parent_area) => (
                "area",
                Some(parent.as_str()),
                Some(parent_area.branch.as_str()),
            ),
            None => {
                diagnostics.push("parent-area-missing");
                ("area", Some(parent.as_str()), None)
            }
        }
    } else {
        ("trunk", None, config.project.trunk.as_deref())
    };
    if effective_base.is_none() {
        diagnostics.push(if base_kind == "trunk" {
            "project-trunk-unbound"
        } else {
            "effective-base-unbound"
        });
    }
    let branch_exists = local_branch_exists(root, &area.branch);
    if area.base_commit.is_none() {
        diagnostics.push("base-anchor-unbound");
    } else if area
        .base_commit
        .as_deref()
        .is_some_and(|anchor| !commit_exists(root, anchor))
    {
        diagnostics.push("base-anchor-missing");
    }

    let anchor_valid = match area.base_commit.as_deref() {
        Some(anchor) if branch_exists && commit_exists(root, anchor) => {
            commit_is_ancestor(root, anchor, &format!("refs/heads/{}", area.branch))
        }
        _ => None,
    };
    if anchor_valid == Some(false) {
        diagnostics.push("base-anchor-not-contained");
    }
    let linear_history = match area.base_commit.as_deref() {
        Some(anchor) if branch_exists && commit_exists(root, anchor) => {
            match child_side_has_merges(root, anchor, &area.branch) {
                Ok(has_merges) => {
                    if has_merges {
                        diagnostics.push("merge-history");
                    }
                    Some(!has_merges)
                }
                Err(_) => {
                    diagnostics.push("history-unavailable");
                    None
                }
            }
        }
        _ => None,
    };
    let mut finalized = None;
    let fresh = match effective_base {
        Some(base) => {
            let branch = area.branch.as_str();
            let base_exists = local_branch_exists(root, base);
            if !base_exists {
                diagnostics.push("effective-base-missing");
            }
            if !branch_exists {
                diagnostics.push("area-branch-missing");
            }
            if base_exists && branch_exists {
                let fresh = base_is_ancestor(root, base, branch);
                finalized = branch_tip(root, base).ok().flatten().map(|base_tip| {
                    fresh == Some(true) && area.base_commit.as_deref() == Some(base_tip.as_str())
                });
                match fresh {
                    Some(true) => {
                        diagnostics.push("fresh");
                        if finalized == Some(false) {
                            diagnostics.push("base-anchor-needs-finalization");
                        }
                    }
                    Some(false) => diagnostics.push("stale"),
                    None => diagnostics.push("ancestry-unavailable"),
                }
                fresh
            } else {
                None
            }
        }
        None => None,
    };
    let same_branch = effective_base == Some(area.branch.as_str());
    let stale_advisory = branch_matches == Some(true)
        && git_state_inspectable
        && operation.is_none()
        && anchor_valid == Some(true)
        && linear_history == Some(true)
        && fresh == Some(false);
    let task_work_safe = (same_branch
        && branch_matches == Some(true)
        && git_state_inspectable
        && operation.is_none())
        || stale_advisory
        || (branch_matches == Some(true)
            && git_state_inspectable
            && operation.is_none()
            && anchor_valid == Some(true)
            && linear_history == Some(true)
            && fresh == Some(true)
            && finalized == Some(true));

    json!({
        "branch": area.branch,
        "checked_out_branch": checked_out,
        "branch_matches": branch_matches,
        "parent_area": area.parent,
        "base_commit": area.base_commit,
        "effective_base": {
            "kind": base_kind,
            "area": base_area,
            "branch": effective_base,
        },
        "fresh": fresh,
        "anchor_valid": anchor_valid,
        "finalized": finalized,
        "linear_history": linear_history,
        "task_work": {
            "safe": task_work_safe,
            "stale_advisory": stale_advisory,
            "git_operation": operation,
        },
        "diagnostics": diagnostics,
    })
}

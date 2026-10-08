use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use clap::{Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::{
    Check, Decision, Observation, ObservationOutcome, ResultRecord, Settings, Verification,
};
use crate::{
    CommandOutput, ZdevError, ZdevStateLock, git_output, git_output_bytes, tasks, write_atomic,
};

#[derive(Debug, Subcommand)]
pub(crate) enum Action {
    /// Record the accepted baseline and exact clean code paths before editing
    Begin {
        #[arg(long, required = true)]
        path: Vec<String>,
        /// Existing run reservation for this preparation dispatch
        #[arg(long)]
        reservation: Option<String>,
    },
    /// Admit an existing linked source worktree under the active run
    Admit {
        #[arg(long)]
        source: PathBuf,
        #[arg(long, required = true)]
        path: Vec<String>,
        #[arg(long)]
        reservation: Option<String>,
    },
    /// Admit another counted repair worker in the stopped source
    Prepare {
        #[arg(long)]
        reservation: String,
    },
    /// Confirm the source worker stopped and retain its complete attributed delta
    Return {
        #[arg(long, required = true)]
        worker_stopped: bool,
    },
    /// Refresh a stopped source against the current accepted baseline
    Reconcile {
        #[arg(long)]
        reservation: Option<String>,
    },
    /// Integrate current measured source code and capture destination verification
    Integrate,
    /// Reserve one attempt, run correctness commands and evaluator, and record evidence
    Evaluate {
        /// Existing run reservation for repair, rerun, confirmation or reconciliation
        #[arg(long)]
        reservation: Option<String>,
    },
    /// Retain candidate code, keep or restore it, complete the task, and commit exact paths
    Finalize {
        #[arg(long, value_enum)]
        decision: Finish,
        #[arg(long)]
        lesson: String,
        /// Fresh independent verifier envelope containing the supplied candidate snapshot
        #[arg(long)]
        verification: Option<PathBuf>,
    },
    /// Read the checkpoint without changing it
    Inspect,
    /// Recover recorded measurement or resume an already authorized finalization
    Recover,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Finish {
    Keep,
    Discard,
    FailedToRun,
    Inconclusive,
}

// This task-owned checkpoint records unfinished effects; task frontmatter owns published evidence.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Checkpoint {
    schema_version: u64,
    area: String,
    task: String,
    branch: String,
    head: String,
    settings: Settings,
    paths: Vec<String>,
    records: Vec<String>,
    record_hashes: BTreeMap<String, String>,
    pending_records: BTreeMap<String, Vec<u8>>,
    starting_snapshot: String,
    attempts: Vec<Attempt>,
    verification_snapshot: Option<String>,
    final_record: Option<ResultRecord>,
    restored: bool,
    completed: bool,
    commit: Option<String>,
    index_updated: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    preparation: Option<super::run::Binding>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    area_metadata: Option<crate::project::AreaMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    isolated: Option<Candidate>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Attempt {
    candidate: String,
    retained_ref: String,
    observation: Observation,
    owner_pid: u32,
    running_pid: Option<u32>,
    invocation_started: Option<f64>,
    measuring: bool,
    published: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reservation: Option<super::run::Binding>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    settings: Option<Settings>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Candidate {
    authoritative_root: PathBuf,
    brief: PathBuf,
    task_record: PathBuf,
    repository_instructions: Vec<PathBuf>,
    admitted_snapshot: String,
    dispatch_head: String,
    dispatch_settings: Settings,
    source_root: PathBuf,
    source_branch: String,
    source_base: String,
    worker_stopped: bool,
    returned_commit: Option<String>,
    retained_ref: String,
    reconciliation: Option<super::run::Binding>,
    refreshing: bool,
    refresh_from: Option<String>,
    integrating: bool,
    integrated: bool,
}

pub(crate) fn experiment(
    root: &Path,
    area: &str,
    task: &str,
    action: &Action,
) -> Result<CommandOutput, ZdevError> {
    let mut dispatch_authorized = false;
    if let Action::Evaluate { reservation } = action {
        evaluate(root, area, task, reservation.as_deref())?;
    } else {
        let _lock = ZdevStateLock::acquire(root)?;
        match action {
            Action::Begin { path, reservation } => {
                begin(root, area, task, path, reservation.as_deref())?
            }
            Action::Admit {
                source,
                path,
                reservation,
            } => {
                dispatch_authorized = !exists(root, area, task)?;
                admit_candidate(root, area, task, source, path, reservation.as_deref())?;
            }
            Action::Prepare { reservation } => {
                let mut checkpoint = load(root, area, task)?;
                dispatch_authorized = prepare_source(root, &mut checkpoint, reservation)?;
            }
            Action::Return { worker_stopped } => {
                let mut checkpoint = load(root, area, task)?;
                return_candidate(root, &mut checkpoint, *worker_stopped)?;
            }
            Action::Reconcile { reservation } => {
                let mut checkpoint = load(root, area, task)?;
                reconcile(root, &mut checkpoint, reservation.as_deref())?;
            }
            Action::Integrate => {
                let mut checkpoint = load(root, area, task)?;
                integrate(root, &mut checkpoint)?;
            }
            Action::Finalize {
                decision,
                lesson,
                verification,
            } => {
                let mut checkpoint = load(root, area, task)?;
                authorize_finalization(
                    root,
                    &mut checkpoint,
                    *decision,
                    lesson,
                    verification.as_deref(),
                )?;
                settle_checkpoint(root, &checkpoint)?;
                finish(root, &mut checkpoint)?;
            }
            Action::Recover => {
                let mut checkpoint = load(root, area, task)?;
                if checkpoint.final_record.is_some() {
                    if !checkpoint.index_updated {
                        settle_checkpoint(root, &checkpoint)?;
                    }
                    finish(root, &mut checkpoint)?;
                } else if checkpoint
                    .isolated
                    .as_ref()
                    .is_some_and(|candidate| candidate.refreshing)
                {
                    refresh_source(root, &mut checkpoint)?;
                } else if checkpoint
                    .isolated
                    .as_ref()
                    .is_some_and(|candidate| candidate.integrating)
                {
                    integrate(root, &mut checkpoint)?;
                } else {
                    recover_measurement(root, &mut checkpoint)?;
                    settle_checkpoint(root, &checkpoint)?;
                }
            }
            Action::Inspect => {}
            Action::Evaluate { .. } => unreachable!(),
        }
    }
    let checkpoint = load(root, area, task)?;
    let next_action = if checkpoint.index_updated {
        "complete"
    } else if checkpoint.final_record.is_some() {
        "recover"
    } else if checkpoint
        .attempts
        .last()
        .is_some_and(|attempt| attempt.measuring)
    {
        "recover measurement; do not relaunch"
    } else if checkpoint.attempts.is_empty() {
        "edit owned code, then evaluate"
    } else {
        "finalize, or edit and evaluate another allotted attempt"
    };
    let comparison_valid = comparison_current(root, &checkpoint)?;
    let candidate_state = candidate_state(root, &checkpoint)?;
    let cleanup_eligible = checkpoint.index_updated
        && checkpoint
            .isolated
            .as_ref()
            .is_some_and(|candidate| candidate.worker_stopped)
        && checkpoint.attempts.iter().all(|attempt| !attempt.measuring);
    let assessment = checkpoint
        .attempts
        .last()
        .filter(|attempt| !attempt.measuring && comparison_valid)
        .map(|attempt| super::compare(&checkpoint.settings.comparison, &attempt.observation))
        .transpose()?;
    let value = json!({"schema_version": 1, "area": area, "task": task, "dispatch_authorized": dispatch_authorized, "candidate_state": candidate_state, "cleanup_eligible": cleanup_eligible, "checkpoint": checkpoint, "assessment": assessment, "comparison_valid": comparison_valid, "next_action": next_action});
    Ok(CommandOutput::new(
        serde_json::to_string_pretty(&value).unwrap(),
        value,
    ))
}

fn checkpoint_path(root: &Path, area: &str, task: &str) -> Result<PathBuf, ZdevError> {
    crate::validate_segment(area, "Area tag")?;
    crate::validate_segment(task, "Task ID")?;
    let path = PathBuf::from(git_output(
        root,
        &[
            "rev-parse",
            "--git-path",
            &format!("zdev/experiments/{area}/{task}.json"),
        ],
    )?);
    Ok(if path.is_absolute() {
        path
    } else {
        root.join(path)
    })
}

pub(crate) fn exists(root: &Path, area: &str, task: &str) -> Result<bool, ZdevError> {
    match fs::symlink_metadata(checkpoint_path(root, area, task)?) {
        Ok(metadata) if metadata.is_file() => Ok(true),
        Ok(_) => Err(ZdevError::new(
            "Experiment checkpoint must be a regular file",
        )),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(ZdevError::io("Cannot inspect experiment checkpoint", error)),
    }
}

fn load(root: &Path, area: &str, task: &str) -> Result<Checkpoint, ZdevError> {
    let checkpoint: Checkpoint = super::read_json(&checkpoint_path(root, area, task)?)?;
    if checkpoint.schema_version != 1 || checkpoint.area != area || checkpoint.task != task {
        return Err(ZdevError::new(
            "Experiment checkpoint does not match the selected task",
        ));
    }
    checkpoint.settings.validate()?;
    if let Some(candidate) = &checkpoint.isolated {
        candidate.dispatch_settings.validate()?;
        if candidate.authoritative_root
            != fs::canonicalize(root)
                .map_err(|error| ZdevError::io("Cannot resolve authoritative root", error))?
            || !candidate.source_root.is_absolute()
            || candidate.source_root == candidate.authoritative_root
            || candidate.source_branch == checkpoint.branch
        {
            return Err(ZdevError::new(
                "Candidate context does not identify a distinct authoritative/source pair",
            ));
        }
    }
    for attempt in &checkpoint.attempts {
        if let Some(settings) = &attempt.settings {
            settings.validate()?;
        }
    }
    if checkpoint.paths.is_empty()
        || checkpoint.attempts.len() > checkpoint.settings.attempt_limit
        || (checkpoint.final_record.is_some() && checkpoint.attempts.is_empty())
    {
        return Err(ZdevError::new(
            "Experiment checkpoint has invalid owned paths or attempts",
        ));
    }
    for path in &checkpoint.paths {
        validate_path(root, path)?;
    }
    let expected_records = [
        format!(".zdev/{area}/TASKS.md"),
        format!(".zdev/{area}/area.toml"),
    ];
    if checkpoint.records.len() != 3
        || !expected_records
            .iter()
            .all(|path| checkpoint.records.contains(path))
        || !checkpoint.records.iter().any(|path| {
            path.starts_with(&format!(".zdev/{area}/tasks/"))
                && Path::new(path)
                    .components()
                    .all(|component| matches!(component, Component::Normal(_)))
        })
        || checkpoint.record_hashes.len() != checkpoint.records.len()
        || checkpoint
            .records
            .iter()
            .any(|path| !checkpoint.record_hashes.contains_key(path))
        || checkpoint
            .pending_records
            .keys()
            .any(|path| !checkpoint.records.contains(path))
    {
        return Err(ZdevError::new(
            "Experiment checkpoint has invalid record ownership",
        ));
    }
    Ok(checkpoint)
}

fn save(root: &Path, checkpoint: &Checkpoint) -> Result<(), ZdevError> {
    let path = checkpoint_path(root, &checkpoint.area, &checkpoint.task)?;
    fs::create_dir_all(path.parent().unwrap())
        .map_err(|error| ZdevError::io("Cannot create experiment checkpoint directory", error))?;
    write_atomic(&path, &serde_json::to_vec_pretty(checkpoint).unwrap())
}

fn snapshot(root: &Path, area: &str, task: &str) -> Result<String, ZdevError> {
    let output = crate::work_context_output(root, area, Some(task))?;
    let bytes = crate::work_context_json_bytes(&output)?;
    let id = crate::work_context_snapshot_id(&bytes);
    let store = crate::work_context_snapshot_store(root, area)?;
    fs::create_dir_all(&store)
        .map_err(|error| ZdevError::io("Cannot create snapshot directory", error))?;
    let path = store.join(format!("{id}.json"));
    if path.exists() {
        crate::read_work_context_snapshot(root, area, &id)?;
    } else {
        write_atomic(&path, &bytes)?;
    }
    Ok(id)
}

fn begin(
    root: &Path,
    area: &str,
    task: &str,
    paths: &[String],
    reservation: Option<&str>,
) -> Result<(), ZdevError> {
    begin_at(root, area, task, paths, reservation, None)
}

fn begin_at(
    root: &Path,
    area: &str,
    task: &str,
    paths: &[String],
    reservation: Option<&str>,
    source: Option<&Path>,
) -> Result<(), ZdevError> {
    if exists(root, area, task)? {
        return Err(ZdevError::new(
            "Experiment already began; inspect or recover it",
        ));
    }
    crate::project::require_task_work_area_link(root, area)?;
    let view = tasks::show(root, area, task)?.value;
    if view["state"] != "ready"
        || view["hypothesis"].is_null()
        || view["status"] != "open"
        || view["research_results"]
            .as_array()
            .is_none_or(|records| !records.is_empty())
    {
        return Err(ZdevError::new(
            "Begin requires an already recorded ready hypothesis with no observations",
        ));
    }
    let settings = super::settings(root, area)?;
    for check in &settings.comparison.required_checks {
        if !settings.correctness_commands.contains_key(check) {
            return Err(ZdevError::new(format!(
                "Missing executable correctness command: {check}"
            )));
        }
    }
    let head = git_output(root, &["rev-parse", "HEAD"])?;
    let baseline = &settings.comparison.baseline_revision;
    git_output(
        root,
        &["rev-parse", "--verify", &format!("{baseline}^{{commit}}")],
    )?;
    if !git_output_bytes(
        root,
        &[
            "diff",
            "--name-only",
            "-z",
            baseline,
            &head,
            "--",
            ".",
            ":(exclude).zdev",
        ],
    )?
    .is_empty()
    {
        return Err(ZdevError::new(
            "Accepted baseline code differs from HEAD; record-only commits are permitted",
        ));
    }
    let mut seen = BTreeSet::new();
    for path in paths {
        validate_path(root, path)?;
        if seen
            .iter()
            .any(|other| Path::new(path).starts_with(other) || Path::new(other).starts_with(path))
        {
            return Err(ZdevError::new("Owned paths overlap"));
        }
        seen.insert(path);
    }
    let records = vec![
        view["path"]
            .as_str()
            .ok_or_else(|| ZdevError::new("Missing task path"))?
            .to_owned(),
        format!(".zdev/{area}/TASKS.md"),
        format!(".zdev/{area}/area.toml"),
    ];
    for path in paths.iter().chain(&records) {
        require_clean_path(root, path)?;
    }
    let starting_snapshot = snapshot(root, area, task)?;
    let isolated = source
        .map(|source| {
            validate_source(
                root,
                source,
                &head,
                area,
                task,
                &starting_snapshot,
                &settings,
                &records,
            )
        })
        .transpose()?;
    let preparation = super::run::bind_work_with_capacity(
        root,
        area,
        task,
        &format!("experiment:{task}:preparation"),
        super::run::WorkKind::Preparation,
        reservation,
        isolated.is_some(),
    )?;
    let mut checkpoint = Checkpoint {
        schema_version: 1,
        area: area.to_owned(),
        task: task.to_owned(),
        branch: git_output(root, &["symbolic-ref", "HEAD"])?,
        head,
        settings,
        paths: paths.to_vec(),
        records,
        record_hashes: BTreeMap::new(),
        pending_records: BTreeMap::new(),
        starting_snapshot,
        attempts: Vec::new(),
        verification_snapshot: None,
        final_record: None,
        restored: false,
        completed: false,
        commit: None,
        index_updated: false,
        preparation,
        area_metadata: Some(area_metadata_without_research(root, area)?),
        isolated,
    };
    update_record_hashes(root, &mut checkpoint)?;
    save(root, &checkpoint)?;
    settle_checkpoint(root, &checkpoint)
}

fn validate_path(root: &Path, path: &str) -> Result<(), ZdevError> {
    let relative = Path::new(path);
    if relative
        .components()
        .any(|component| !matches!(component, Component::Normal(_)))
        || path.is_empty()
        || path.contains(['\\', '*', '?', '[', '\n', '\r'])
        || path.starts_with(':')
        || relative.starts_with(".git")
        || relative.starts_with(".zdev")
    {
        return Err(ZdevError::new(
            "Owned paths must be exact repository-relative code files",
        ));
    }
    let mut parent = relative.parent();
    while let Some(directory) = parent {
        if !directory.as_os_str().is_empty()
            && fs::symlink_metadata(root.join(directory)).is_ok_and(|metadata| !metadata.is_dir())
        {
            return Err(ZdevError::new(
                "Owned path traverses a symlink or non-directory",
            ));
        }
        parent = directory.parent();
    }
    if fs::symlink_metadata(root.join(path)).is_ok_and(|metadata| metadata.is_dir()) {
        return Err(ZdevError::new("Owned paths cannot be directories"));
    }
    if git_output(root, &["ls-files", "--stage", "--", path])?.starts_with("160000 ") {
        return Err(ZdevError::new("Owned paths cannot be submodules"));
    }
    Ok(())
}

fn require_clean_path(root: &Path, path: &str) -> Result<(), ZdevError> {
    if !git_output(
        root,
        &["status", "--porcelain", "--untracked-files=all", "--", path],
    )?
    .is_empty()
        || !git_output(
            root,
            &[
                "ls-files",
                "--others",
                "--ignored",
                "--exclude-standard",
                "--",
                path,
            ],
        )?
        .is_empty()
    {
        return Err(ZdevError::new(format!(
            "Owned path overlaps existing local work: {path}"
        )));
    }
    Ok(())
}

fn require_environment(root: &Path, checkpoint: &Checkpoint) -> Result<(), ZdevError> {
    crate::project::require_task_work_area_link(root, &checkpoint.area)?;
    if git_output(root, &["symbolic-ref", "HEAD"])? != checkpoint.branch
        || (checkpoint.isolated.is_none()
            && git_output(root, &["rev-parse", "HEAD"])? != checkpoint.head)
    {
        return Err(ZdevError::new(
            "Experiment branch or HEAD changed; inspect before recovery",
        ));
    }
    if super::settings(root, &checkpoint.area)? != checkpoint.settings
        && !super::run::record_policy_compatible(root, &checkpoint.area, &checkpoint.settings)?
    {
        return Err(ZdevError::new(
            "Research settings changed during the experiment",
        ));
    }
    if checkpoint.isolated.is_some() {
        require_source(checkpoint)?;
        require_record_ownership(root, checkpoint)?;
        return Ok(());
    }
    for path in &checkpoint.paths {
        validate_path(root, path)?;
        if !git_output_bytes(root, &["diff", "--cached", "--name-only", "-z", "--", path])?
            .is_empty()
        {
            return Err(ZdevError::new(format!(
                "Owned path has staged work: {path}; leave candidate changes unstaged"
            )));
        }
    }
    require_record_ownership(root, checkpoint)?;
    Ok(())
}

fn area_metadata_without_research(
    root: &Path,
    area: &str,
) -> Result<crate::project::AreaMetadata, ZdevError> {
    let mut metadata = crate::project::load_area(root, area)?.0;
    metadata.research = None;
    Ok(metadata)
}

fn area_metadata_matches(root: &Path, checkpoint: &Checkpoint) -> Result<bool, ZdevError> {
    let Some(expected) = &checkpoint.area_metadata else {
        return Ok(false);
    };
    Ok(serde_json::to_value(expected).map_err(json_error)?
        == serde_json::to_value(area_metadata_without_research(root, &checkpoint.area)?)
            .map_err(json_error)?)
}

fn require_record_ownership(root: &Path, checkpoint: &Checkpoint) -> Result<(), ZdevError> {
    for (path, expected) in &checkpoint.record_hashes {
        if checkpoint.isolated.is_some() && path == &format!(".zdev/{}/TASKS.md", checkpoint.area) {
            tasks::validate_index(root, &checkpoint.area)?;
            require_clean_index(root, path)?;
            continue;
        }
        if git_output(root, &["hash-object", "--", path])? != *expected
            && !(path == &format!(".zdev/{}/area.toml", checkpoint.area)
                && area_metadata_matches(root, checkpoint)?
                && (checkpoint.isolated.is_some()
                    || super::settings(root, &checkpoint.area)? != checkpoint.settings)
                && super::run::record_policy_compatible(
                    root,
                    &checkpoint.area,
                    &checkpoint.settings,
                )?)
        {
            return Err(ZdevError::new(format!(
                "Experiment record changed outside the lifecycle: {path}"
            )));
        }
        if !git_output_bytes(root, &["diff", "--cached", "--name-only", "-z", "--", path])?
            .is_empty()
        {
            return Err(ZdevError::new(format!(
                "Experiment record has staged work: {path}"
            )));
        }
    }
    Ok(())
}

fn prepare_publication(
    root: &Path,
    checkpoint: &mut Checkpoint,
    files: &[(PathBuf, Vec<u8>)],
) -> Result<(), ZdevError> {
    require_record_ownership(root, checkpoint)?;
    checkpoint.pending_records = files
        .iter()
        .map(|(path, bytes)| (crate::relative(root, path), bytes.clone()))
        .collect();
    if checkpoint
        .pending_records
        .keys()
        .any(|path| !checkpoint.records.contains(path))
    {
        return Err(ZdevError::new(
            "Publication includes an unowned experiment record",
        ));
    }
    save(root, checkpoint)
}

fn recover_publication(root: &Path, checkpoint: &mut Checkpoint) -> Result<(), ZdevError> {
    if checkpoint.pending_records.is_empty() {
        return Ok(());
    }
    for (path, expected) in &checkpoint.pending_records {
        let bytes = fs::read(root.join(path))
            .map_err(|error| ZdevError::io("Cannot inspect pending experiment record", error))?;
        if bytes != *expected
            && git_output(root, &["hash-object", "--", path])? != checkpoint.record_hashes[path]
        {
            if checkpoint.isolated.is_some()
                && path == &format!(".zdev/{}/TASKS.md", checkpoint.area)
            {
                tasks::validate_index(root, &checkpoint.area)?;
                require_clean_index(root, path)?;
                continue;
            }
            return Err(ZdevError::new(format!(
                "Pending experiment record has unrelated changes: {path}"
            )));
        }
        if !git_output_bytes(root, &["diff", "--cached", "--name-only", "-z", "--", path])?
            .is_empty()
        {
            return Err(ZdevError::new(format!(
                "Pending experiment record has staged work: {path}"
            )));
        }
    }
    for (path, bytes) in &checkpoint.pending_records {
        if checkpoint.isolated.is_none() || path != &format!(".zdev/{}/TASKS.md", checkpoint.area) {
            write_atomic(&root.join(path), bytes)?;
        }
    }
    if checkpoint.isolated.is_some()
        && checkpoint
            .pending_records
            .contains_key(&format!(".zdev/{}/TASKS.md", checkpoint.area))
    {
        tasks::write_index(root, &checkpoint.area)?;
    }
    update_record_hashes(root, checkpoint)?;
    checkpoint.pending_records.clear();
    save(root, checkpoint)
}

fn update_record_hashes(root: &Path, checkpoint: &mut Checkpoint) -> Result<(), ZdevError> {
    checkpoint.record_hashes = checkpoint
        .records
        .iter()
        .map(|path| {
            Ok((
                path.clone(),
                git_output(root, &["hash-object", "--", path])?,
            ))
        })
        .collect::<Result<_, ZdevError>>()?;
    Ok(())
}

// A private index captures only attributed files, including deletions, modes and binary blobs.
fn code_tree(root: &Path, base: &str, paths: &[String]) -> Result<String, ZdevError> {
    let directory = tempfile::tempdir()
        .map_err(|error| ZdevError::io("Cannot create isolated Git index", error))?;
    let index = directory.path().join("index");
    index_git(root, &index, &["read-tree", base])?;
    for path in paths {
        if fs::symlink_metadata(root.join(path)).is_ok()
            || !index_git(root, &index, &["ls-files", "--", path])?.is_empty()
        {
            index_git(
                root,
                &index,
                &["--literal-pathspecs", "add", "-f", "-A", "--", path],
            )?;
        }
    }
    index_git(root, &index, &["write-tree"])
}

fn index_git(root: &Path, index: &Path, arguments: &[&str]) -> Result<String, ZdevError> {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(root)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env("GIT_INDEX_FILE", index)
        .output()
        .map_err(|error| ZdevError::io("Cannot run isolated-index Git operation", error))?;
    if !output.status.success() {
        return Err(ZdevError::new(format!(
            "Isolated-index Git operation failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn candidate_matches(
    root: &Path,
    checkpoint: &Checkpoint,
    candidate: &str,
) -> Result<bool, ZdevError> {
    let code_root = if checkpoint
        .isolated
        .as_ref()
        .is_some_and(|candidate| !candidate.integrated)
    {
        source_root(checkpoint)
    } else {
        root
    };
    let directory = tempfile::tempdir()
        .map_err(|error| ZdevError::io("Cannot create candidate comparison index", error))?;
    let index = directory.path().join("index");
    index_git(root, &index, &["read-tree", &checkpoint.head])?;
    for path in &checkpoint.paths {
        index_git(
            root,
            &index,
            &["--literal-pathspecs", "reset", "-q", candidate, "--", path],
        )?;
    }
    Ok(code_tree(code_root, &checkpoint.head, &checkpoint.paths)?
        == index_git(root, &index, &["write-tree"])?)
}

fn evaluate(
    root: &Path,
    area: &str,
    task: &str,
    reservation: Option<&str>,
) -> Result<(), ZdevError> {
    let settings = {
        let _lock = ZdevStateLock::acquire(root)?;
        let mut checkpoint = load(root, area, task)?;
        require_environment(root, &checkpoint)?;
        if !comparison_current(root, &checkpoint)? {
            return Err(ZdevError::new(
                "Comparison conditions changed; retain evidence and discard this candidate before rebaselining",
            ));
        }
        if checkpoint.final_record.is_some()
            || checkpoint
                .attempts
                .last()
                .is_some_and(|attempt| attempt.measuring || !attempt.published)
        {
            return Err(ZdevError::new(
                "Recover the unfinished experiment before evaluating",
            ));
        }
        if checkpoint.attempts.len() >= checkpoint.settings.attempt_limit {
            return Err(ZdevError::new("Allotted experiment attempts are exhausted"));
        }
        if let Some(candidate) = &checkpoint.isolated {
            if !candidate.worker_stopped
                || candidate.returned_commit.is_none()
                || candidate.integrating
                || candidate.integrated
            {
                return Err(ZdevError::new(
                    "Return a stopped isolated candidate before measurement",
                ));
            }
            require_source_delta(&checkpoint)?;
            if !candidate_matches(
                root,
                &checkpoint,
                candidate.returned_commit.as_ref().unwrap(),
            )? {
                return Err(ZdevError::new(
                    "Source changed after return; return its stopped work again",
                ));
            }
        }
        let reconciliation = checkpoint
            .isolated
            .as_mut()
            .and_then(|candidate| candidate.reconciliation.take());
        let reservation = if let Some(binding) = reconciliation {
            super::run::claim_measurement(root, area, &binding)?;
            Some(binding)
        } else {
            super::run::bind_work(
                root,
                area,
                task,
                &format!(
                    "experiment:{task}:attempt:{}",
                    checkpoint.attempts.len() + 1
                ),
                super::run::WorkKind::Rerun,
                reservation,
            )?
        };
        let tree = code_tree(
            source_root_or(root, &checkpoint),
            &checkpoint.head,
            &checkpoint.paths,
        )?;
        let candidate = git_output(
            root,
            &[
                "commit-tree",
                &tree,
                "-p",
                &checkpoint.head,
                "-m",
                &format!(
                    "experiment {} attempt {}",
                    task,
                    checkpoint.attempts.len() + 1
                ),
            ],
        )?;
        let retained_ref = format!(
            "refs/zdev/experiments/{area}/{task}/{}",
            checkpoint.attempts.len() + 1
        );
        checkpoint.attempts.push(Attempt {
            candidate: candidate.clone(),
            retained_ref,
            observation: Observation {
                starting_revision: checkpoint.settings.comparison.baseline_revision.clone(),
                candidate_revision: candidate,
                outcome: ObservationOutcome::Measured,
                samples: Vec::new(),
                checks: Vec::new(),
                elapsed_seconds: 0.0,
                evaluation_runs: Some(0),
                correctness_runs: Some(0),
                failure: None,
            },
            owner_pid: std::process::id(),
            running_pid: None,
            invocation_started: None,
            measuring: true,
            published: false,
            reservation,
            settings: Some(checkpoint.settings.clone()),
        });
        checkpoint.verification_snapshot = None;
        save(root, &checkpoint)?;
        retain(root, checkpoint.attempts.last().unwrap())?;
        checkpoint.settings
    };
    for name in &settings.comparison.required_checks {
        let command = format!(
            "( {} ) >/dev/null && printf 1",
            settings.correctness_commands[name]
        );
        let (result, elapsed) =
            run_measured_command(root, area, task, &command, true, settings.timeout_seconds)?;
        let _lock = ZdevStateLock::acquire(root)?;
        let mut checkpoint = load(root, area, task)?;
        let attempt = checkpoint.attempts.last_mut().unwrap();
        attempt.running_pid = None;
        attempt.invocation_started = None;
        attempt.observation.elapsed_seconds += elapsed;
        attempt.observation.checks.push(Check {
            name: name.clone(),
            passed: result.is_ok(),
        });
        if let Err((outcome, failure)) = result {
            attempt.observation.outcome = outcome;
            attempt.observation.failure = Some(format!("Correctness command {name}: {failure}"));
        }
        save(root, &checkpoint)?;
        settle_checkpoint(root, &checkpoint)?;
    }
    for _ in 0..settings.comparison.repetitions {
        let (result, elapsed) = run_measured_command(
            root,
            area,
            task,
            &settings.evaluator_command,
            false,
            settings.timeout_seconds,
        )?;
        let _lock = ZdevStateLock::acquire(root)?;
        let mut checkpoint = load(root, area, task)?;
        let attempt = checkpoint.attempts.last_mut().unwrap();
        attempt.running_pid = None;
        attempt.invocation_started = None;
        attempt.observation.elapsed_seconds += elapsed;
        let failed = result.is_err();
        match result {
            Ok(sample) => attempt.observation.samples.push(sample),
            Err((outcome, failure)) => {
                attempt.observation.outcome = outcome;
                attempt.observation.failure = Some(failure);
            }
        }
        save(root, &checkpoint)?;
        settle_checkpoint(root, &checkpoint)?;
        if failed {
            break;
        }
    }
    let _lock = ZdevStateLock::acquire(root)?;
    let mut checkpoint = load(root, area, task)?;
    let candidate = checkpoint.attempts.last().unwrap().candidate.clone();
    let unchanged = candidate_matches(root, &checkpoint, &candidate)?;
    let attempt = checkpoint.attempts.last_mut().unwrap();
    attempt.measuring = false;
    if !unchanged
        || (attempt.observation.outcome == ObservationOutcome::Measured
            && super::summarize(&attempt.observation.samples).is_err())
    {
        attempt.observation.outcome = ObservationOutcome::Invalid;
        attempt.observation.failure = Some(
            "Candidate changed during measurement or samples have an invalid aggregate".to_owned(),
        );
    }
    save(root, &checkpoint)?;
    publish_observations(root, &mut checkpoint)?;
    if checkpoint.isolated.is_none() {
        checkpoint.verification_snapshot = Some(snapshot(root, area, task)?);
    }
    save(root, &checkpoint)?;
    settle_checkpoint(root, &checkpoint)
}

fn settle_checkpoint(root: &Path, checkpoint: &Checkpoint) -> Result<(), ZdevError> {
    if checkpoint.final_record.is_some()
        && let Some(binding) = checkpoint
            .isolated
            .as_ref()
            .and_then(|candidate| candidate.reconciliation.as_ref())
    {
        super::run::settle(root, &checkpoint.area, binding, 0.0, true, 0, false)?;
    }
    if let Some(binding) = &checkpoint.preparation {
        super::run::settle(
            root,
            &checkpoint.area,
            binding,
            0.0,
            checkpoint
                .isolated
                .as_ref()
                .is_none_or(|candidate| candidate.worker_stopped),
            0,
            false,
        )?;
    }
    for attempt in &checkpoint.attempts {
        if let Some(binding) = &attempt.reservation {
            super::run::settle(
                root,
                &checkpoint.area,
                binding,
                attempt.observation.elapsed_seconds,
                !attempt.measuring,
                attempt.observation.correctness_runs.unwrap_or(0)
                    + attempt.observation.evaluation_runs.unwrap_or(0),
                attempt.invocation_started.is_some(),
            )?;
        }
    }
    Ok(())
}

type CommandResult = Result<f64, (ObservationOutcome, String)>;

fn run_measured_command(
    root: &Path,
    area: &str,
    task: &str,
    command: &str,
    correctness: bool,
    timeout: f64,
) -> Result<(CommandResult, f64), ZdevError> {
    let (repetition, timeout) = {
        let _lock = ZdevStateLock::acquire(root)?;
        let mut checkpoint = load(root, area, task)?;
        require_environment(root, &checkpoint)?;
        if super::settings(root, area)? != checkpoint.settings {
            return Err(ZdevError::new(
                "Research settings changed during measurement; recover without relaunching",
            ));
        }
        let attempt = checkpoint.attempts.last_mut().unwrap();
        if attempt.owner_pid != std::process::id() || !attempt.measuring {
            return Err(ZdevError::new("Measurement ownership changed"));
        }
        let known_commands = attempt.observation.correctness_runs.unwrap_or(0)
            + attempt.observation.evaluation_runs.unwrap_or(0);
        let timeout = if let Some(binding) = &attempt.reservation {
            super::run::reserve_command(root, area, binding, known_commands, timeout, true)?
        } else if super::run::active(root, area)? {
            return Err(ZdevError::new(
                "Experiment has no active run reservation; recover existing work before dispatch",
            ));
        } else {
            timeout
        };
        let runs = if correctness {
            &mut attempt.observation.correctness_runs
        } else {
            &mut attempt.observation.evaluation_runs
        };
        *runs = Some(runs.unwrap_or(0) + 1);
        let repetition = runs.unwrap();
        attempt.invocation_started = Some(unix_seconds());
        save(root, &checkpoint)?;
        (repetition, timeout)
    };
    let checkpoint = load(root, area, task)?;
    let code_root = source_root_or(root, &checkpoint);
    let started = std::time::Instant::now();
    let result = super::evaluate_with_started(code_root, command, timeout, repetition, |pid| {
        let _lock = ZdevStateLock::acquire(root)?;
        let mut checkpoint = load(root, area, task)?;
        checkpoint.attempts.last_mut().unwrap().running_pid = Some(pid);
        save(root, &checkpoint)
    });
    Ok((result, started.elapsed().as_secs_f64()))
}

fn retain(root: &Path, attempt: &Attempt) -> Result<(), ZdevError> {
    let existing = Command::new("git")
        .args(["rev-parse", "--verify", &attempt.retained_ref])
        .current_dir(root)
        .output()
        .map_err(|error| ZdevError::io("Cannot inspect retained candidate", error))?;
    if existing.status.success() {
        if String::from_utf8_lossy(&existing.stdout).trim() != attempt.candidate {
            return Err(ZdevError::new(
                "Retained candidate reference points to different code",
            ));
        }
    } else {
        git_output(
            root,
            &[
                "update-ref",
                &attempt.retained_ref,
                &attempt.candidate,
                &"0".repeat(attempt.candidate.len()),
            ],
        )?;
    }
    git_output(
        root,
        &[
            "cat-file",
            "-e",
            &format!("{}^{{commit}}", attempt.candidate),
        ],
    )?;
    Ok(())
}

fn observed_record(checkpoint: &Checkpoint, attempt: &Attempt) -> ResultRecord {
    ResultRecord {
        comparison: attempt
            .settings
            .as_ref()
            .unwrap_or(&checkpoint.settings)
            .comparison
            .clone(),
        attempt_limit: attempt
            .settings
            .as_ref()
            .unwrap_or(&checkpoint.settings)
            .attempt_limit,
        observations: vec![attempt.observation.clone()],
        decision: Decision::Observed,
        lesson: "Recorded correctness commands and evaluator output; final lesson pending."
            .to_owned(),
        verification: None,
        retained_candidate: Some(attempt.retained_ref.clone()),
        restored_revision: None,
        accepted_revision: None,
    }
}

fn publish_observations(root: &Path, checkpoint: &mut Checkpoint) -> Result<(), ZdevError> {
    recover_publication(root, checkpoint)?;
    require_record_ownership(root, checkpoint)?;
    let existing =
        tasks::show(root, &checkpoint.area, &checkpoint.task)?.value["research_results"].clone();
    let mut expected = checkpoint
        .attempts
        .iter()
        .filter(|attempt| !attempt.measuring)
        .map(|attempt| observed_record(checkpoint, attempt))
        .collect::<Vec<_>>();
    if let Some(record) = &checkpoint.final_record {
        expected.push(record.clone());
    }
    let actual: Vec<ResultRecord> =
        serde_json::from_value(existing).map_err(|error| ZdevError::new(error.to_string()))?;
    if actual.len() > expected.len()
        || serde_json::to_value(&actual).map_err(json_error)?
            != serde_json::to_value(&expected[..actual.len()]).map_err(json_error)?
    {
        return Err(ZdevError::new(
            "Task evidence differs from this experiment; inspect before recovery",
        ));
    }
    for record in &expected[actual.len()..] {
        let area = checkpoint.area.clone();
        let task = checkpoint.task.clone();
        tasks::append_research_record_with_checkpoint(root, &area, &task, record, |files| {
            prepare_publication(root, checkpoint, files)
        })?;
        recover_publication(root, checkpoint)?;
    }
    for attempt in &mut checkpoint.attempts {
        if !attempt.measuring {
            attempt.published = true;
        }
    }
    update_record_hashes(root, checkpoint)?;
    save(root, checkpoint)
}

fn unix_seconds() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

pub(super) fn process_running(pid: u32) -> bool {
    let alive = Command::new("kill")
        .args(["-0", &pid.to_string()])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok_and(|status| status.success());
    if !alive {
        return false;
    }
    // An interrupted parent can leave a zombie child until the host reaps it.
    let zombie = Command::new("ps")
        .args(["-o", "stat=", "-p", &pid.to_string()])
        .output()
        .is_ok_and(|output| {
            output.status.success()
                && String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .starts_with('Z')
        });
    !zombie
}

pub(super) fn process_group_running(id: u32) -> bool {
    #[cfg(unix)]
    {
        // The shell leader can exit while evaluator descendants still own the group.
        if let Ok(output) = Command::new("ps")
            .args(["-ax", "-o", "pgid=,stat="])
            .output()
            && output.status.success()
        {
            return String::from_utf8_lossy(&output.stdout).lines().any(|line| {
                let mut fields = line.split_whitespace();
                fields.next().and_then(|field| field.parse::<u32>().ok()) == Some(id)
                    && fields.next().is_some_and(|status| !status.starts_with('Z'))
            });
        }
        // If process inspection fails, preserve uncertainty whenever the group is reachable.
        Command::new("sh")
            .args(["-c", &format!("kill -0 -{id}")])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .is_ok_and(|status| status.success())
    }
    #[cfg(not(unix))]
    process_running(id)
}

fn recover_measurement(root: &Path, checkpoint: &mut Checkpoint) -> Result<(), ZdevError> {
    if let Some(candidate) = &checkpoint.isolated
        && let Some(commit) = &candidate.returned_commit
    {
        git_output(root, &["cat-file", "-e", &format!("{commit}^{{commit}}")])?;
        git_output(root, &["update-ref", &candidate.retained_ref, commit])?;
    }
    recover_publication(root, checkpoint)?;
    if let Some(attempt) = checkpoint.attempts.last_mut() {
        if attempt.measuring {
            if process_running(attempt.owner_pid)
                || attempt.running_pid.is_some_and(process_group_running)
            {
                return Err(ZdevError::new(
                    "Measurement process is unresolved or still running; no command was relaunched",
                ));
            }
            attempt.measuring = false;
            attempt.running_pid = None;
            if let Some(started) = attempt.invocation_started.take() {
                attempt.observation.elapsed_seconds += (unix_seconds() - started).max(0.0);
            }
            attempt.observation.outcome = ObservationOutcome::Crashed;
            attempt.observation.failure = Some("Measurement interrupted; reserved invocation counts and completed samples were retained. Task remains open.".to_owned());
            save(root, checkpoint)?;
        }
        let attempt = checkpoint.attempts.last_mut().unwrap();
        // Earlier checkpoints omitted the outcome when a correctness command failed.
        // Repair only unpublished evidence; its samples, failure and costs remain intact.
        if !attempt.published
            && attempt.observation.outcome == ObservationOutcome::Measured
            && attempt.observation.checks.iter().any(|check| !check.passed)
            && attempt
                .observation
                .failure
                .as_deref()
                .is_some_and(|failure| failure.starts_with("Correctness command "))
        {
            attempt.observation.outcome = ObservationOutcome::Invalid;
            save(root, checkpoint)?;
        }
        retain(root, checkpoint.attempts.last().unwrap())?;
        publish_observations(root, checkpoint)?;
        require_environment(root, checkpoint)?;
        if checkpoint.isolated.is_none() && checkpoint.verification_snapshot.is_none() {
            checkpoint.verification_snapshot =
                Some(snapshot(root, &checkpoint.area, &checkpoint.task)?);
        }
        save(root, checkpoint)?;
    }
    Ok(())
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct VerifierEnvelope {
    schema_version: u64,
    kind: String,
    area: String,
    task_id: String,
    verdict: String,
    summary: String,
    evidence: Vec<String>,
    findings: Vec<String>,
    escalation: String,
}

fn authorize_finalization(
    root: &Path,
    checkpoint: &mut Checkpoint,
    decision: Finish,
    lesson: &str,
    verification: Option<&Path>,
) -> Result<(), ZdevError> {
    if checkpoint.final_record.is_some() {
        return Err(ZdevError::new(
            "Finalization already authorized; use recover",
        ));
    }
    require_environment(root, checkpoint)?;
    if matches!(decision, Finish::Keep) && !comparison_current(root, checkpoint)? {
        return Err(ZdevError::new(
            "Changed comparison conditions cannot accept this candidate; discard and rebaseline",
        ));
    }
    if let Some(candidate) = &checkpoint.isolated {
        if !candidate.worker_stopped
            || candidate.refreshing
            || candidate.integrating
            || (matches!(decision, Finish::Keep) && !candidate.integrated)
        {
            return Err(ZdevError::new(
                "Keep requires clean destination integration; unfinished source or integration is preserved",
            ));
        }
        if !matches!(decision, Finish::Keep) && candidate.integrated {
            return Err(ZdevError::new(
                "Integrated acceptance is unfinished; recover and verify it before keep",
            ));
        }
    }
    crate::validate_nonempty_line(lesson, "Research lesson")?;
    if checkpoint.attempts.is_empty()
        || checkpoint
            .attempts
            .iter()
            .any(|attempt| attempt.measuring || !attempt.published)
    {
        return Err(ZdevError::new(
            "Finalize requires recorded completed attempts; recover measurement first",
        ));
    }
    let last = checkpoint.attempts.last().unwrap();
    if !candidate_matches(root, checkpoint, &last.candidate)? {
        return Err(ZdevError::new("Candidate code changed after evaluation"));
    }
    for attempt in &checkpoint.attempts {
        retain(root, attempt)?;
    }
    let verification = if matches!(decision, Finish::Keep) {
        let envelope: VerifierEnvelope = super::read_json(
            verification
                .ok_or_else(|| ZdevError::new("Keep requires fresh independent verification"))?,
        )?;
        let snapshot = checkpoint
            .verification_snapshot
            .as_deref()
            .ok_or_else(|| ZdevError::new("Missing candidate verification snapshot"))?;
        if envelope.schema_version != 1
            || envelope.kind != "verifier"
            || envelope.area != checkpoint.area
            || envelope.task_id != checkpoint.task
            || envelope.verdict != "pass"
            || envelope.escalation != "none"
            || !envelope.findings.is_empty()
            || envelope.summary.trim().is_empty()
            || !envelope
                .evidence
                .contains(&format!("work_context_snapshot: {snapshot}"))
            || envelope.evidence.iter().any(|item| item.trim().is_empty())
        {
            return Err(ZdevError::new(
                "Keep requires a passing independent verifier envelope bound to the candidate snapshot",
            ));
        }
        let (_, stored, _) = crate::read_work_context_snapshot(root, &checkpoint.area, snapshot)?;
        let current = crate::work_context_json_bytes(&crate::work_context_output(
            root,
            &checkpoint.area,
            Some(&checkpoint.task),
        )?)?;
        if stored != current {
            return Err(ZdevError::new("Candidate verification snapshot is stale"));
        }
        Some(Verification {
            reviewer: "fresh independent verifier".to_owned(),
            passed: true,
            summary: envelope.summary,
        })
    } else {
        None
    };
    let record = ResultRecord {
        comparison: last
            .settings
            .as_ref()
            .unwrap_or(&checkpoint.settings)
            .comparison
            .clone(),
        attempt_limit: checkpoint.settings.attempt_limit,
        observations: Vec::new(),
        decision: match decision {
            Finish::Keep => Decision::Accepted,
            Finish::Discard => Decision::Rejected,
            Finish::FailedToRun => Decision::FailedToRun,
            Finish::Inconclusive => Decision::Inconclusive,
        },
        lesson: lesson.to_owned(),
        verification,
        retained_candidate: Some(last.retained_ref.clone()),
        accepted_revision: if checkpoint.isolated.is_some() && !matches!(decision, Finish::Keep) {
            Some(
                super::settings(root, &checkpoint.area)?
                    .comparison
                    .baseline_revision,
            )
        } else {
            None
        },
        restored_revision: if matches!(decision, Finish::Keep) || checkpoint.isolated.is_some() {
            None
        } else {
            Some(if checkpoint.isolated.is_some() {
                super::settings(root, &checkpoint.area)?
                    .comparison
                    .baseline_revision
            } else {
                checkpoint.settings.comparison.baseline_revision.clone()
            })
        },
    };
    let mut records = checkpoint
        .attempts
        .iter()
        .map(|attempt| observed_record(checkpoint, attempt))
        .collect::<Vec<_>>();
    records.push(record.clone());
    super::validate_evidence(
        &records,
        true,
        if matches!(decision, Finish::Keep) {
            Some(&checkpoint.settings)
        } else {
            None
        },
    )?;
    if checkpoint.isolated.is_some() {
        checkpoint.head = git_output(root, &["rev-parse", "HEAD"])?;
    }
    checkpoint.final_record = Some(record);
    save(root, checkpoint)
}

fn finish(root: &Path, checkpoint: &mut Checkpoint) -> Result<(), ZdevError> {
    if checkpoint.index_updated {
        if checkpoint.isolated.is_some() {
            super::run::acceptance(root, &checkpoint.area, &checkpoint.task, false)?;
        }
        return Ok(());
    }
    crate::project::require_task_work_area_link(root, &checkpoint.area)?;
    let record = checkpoint
        .final_record
        .as_ref()
        .ok_or_else(|| ZdevError::new("No authorized finalization to recover"))?
        .clone();
    let last = checkpoint.attempts.last().unwrap();
    let candidate = last.candidate.clone();
    if let Some(commit) = &checkpoint.commit {
        let mut committed_paths = checkpoint.records.clone();
        committed_paths.extend(checkpoint.paths.clone());
        if code_tree(root, &checkpoint.head, &committed_paths)?
            != git_output(root, &["rev-parse", &format!("{commit}^{{tree}}")])?
        {
            return Err(ZdevError::new("Owned files changed during commit recovery"));
        }
        for path in &committed_paths {
            if !git_output_bytes(
                root,
                &[
                    "--literal-pathspecs",
                    "diff",
                    "--cached",
                    "--name-only",
                    "-z",
                    &checkpoint.head,
                    "--",
                    path,
                ],
            )?
            .is_empty()
                && !git_output_bytes(
                    root,
                    &[
                        "--literal-pathspecs",
                        "diff",
                        "--cached",
                        "--name-only",
                        "-z",
                        commit,
                        "--",
                        path,
                    ],
                )?
                .is_empty()
            {
                return Err(ZdevError::new(format!(
                    "Owned index path changed during commit recovery: {path}"
                )));
            }
        }
        let head = git_output(root, &["rev-parse", "HEAD"])?;
        if head == checkpoint.head {
            git_output(
                root,
                &["update-ref", &checkpoint.branch, commit, &checkpoint.head],
            )?;
        } else if head != *commit {
            return Err(ZdevError::new("HEAD changed during final commit recovery"));
        }
        if !checkpoint.index_updated {
            // Update only committed paths in the real index; all other staged entries remain intact.
            let mut paths = checkpoint.records.clone();
            if record.decision == Decision::Accepted {
                paths.extend(checkpoint.paths.clone());
            }
            for path in &paths {
                git_output(
                    root,
                    &["--literal-pathspecs", "reset", "-q", commit, "--", path],
                )?;
            }
            checkpoint.index_updated = true;
            save(root, checkpoint)?;
            if checkpoint.isolated.is_some() {
                super::run::acceptance(root, &checkpoint.area, &checkpoint.task, false)?;
            }
        }
        return Ok(());
    }
    if git_output(root, &["symbolic-ref", "HEAD"])? != checkpoint.branch
        || git_output(root, &["rev-parse", "HEAD"])? != checkpoint.head
    {
        return Err(ZdevError::new(
            "Experiment branch or HEAD changed during finalization",
        ));
    }
    recover_publication(root, checkpoint)?;
    require_record_ownership(root, checkpoint)?;
    for path in &checkpoint.paths {
        validate_path(root, path)?;
        require_candidate_index(root, checkpoint, path)?;
    }
    for attempt in &checkpoint.attempts {
        retain(root, attempt)?;
    }
    // Publishing compares complete expected records, so a crash after publication cannot append twice.
    publish_observations(root, checkpoint)?;
    if record.decision != Decision::Accepted && !checkpoint.restored {
        if checkpoint.isolated.is_some() {
            require_accepted_destination(root, checkpoint)?;
        } else {
            restore_owned_paths(root, checkpoint, &candidate)?;
        }
        checkpoint.restored = true;
        save(root, checkpoint)?;
    }
    if record.decision == Decision::Accepted {
        if !candidate_matches(root, checkpoint, &candidate)? {
            return Err(ZdevError::new(
                "Verified candidate code changed before acceptance",
            ));
        }
        let (mut metadata, _) = crate::project::load_area(root, &checkpoint.area)?;
        let mut accepted_settings = checkpoint.settings.clone();
        accepted_settings.comparison.baseline_revision = candidate.clone();
        accepted_settings.comparison.baseline_samples = checkpoint
            .attempts
            .last()
            .unwrap()
            .observation
            .samples
            .clone();
        if metadata.research.as_ref() != Some(&checkpoint.settings)
            && metadata.research.as_ref() != Some(&accepted_settings)
        {
            return Err(ZdevError::new("Research policy changed during acceptance"));
        }
        super::run::accepted(root, &checkpoint.area, &accepted_settings)?;
        metadata.research = Some(accepted_settings);
        let area_path = root.join(format!(".zdev/{}/area.toml", checkpoint.area));
        prepare_publication(
            root,
            checkpoint,
            &[(
                area_path,
                toml::to_string_pretty(&metadata).unwrap().into_bytes(),
            )],
        )?;
        recover_publication(root, checkpoint)?;
    } else if super::settings(root, &checkpoint.area)? != checkpoint.settings
        && !super::run::record_policy_compatible(root, &checkpoint.area, &checkpoint.settings)?
    {
        return Err(ZdevError::new("Research policy changed during discard"));
    }
    if !checkpoint.completed {
        let area = checkpoint.area.clone();
        let task = checkpoint.task.clone();
        tasks::complete_with_checkpoint(root, &area, &task, &record.lesson,
            &["Correctness commands, evaluator observations, retained candidate and owned code state checked.".to_owned()],
            |files| prepare_publication(root, checkpoint, files))?;
        recover_publication(root, checkpoint)?;
        checkpoint.completed = true;
        update_record_hashes(root, checkpoint)?;
        save(root, checkpoint)?;
    }
    let records: Vec<ResultRecord> = serde_json::from_value(
        tasks::show(root, &checkpoint.area, &checkpoint.task)?.value["research_results"].clone(),
    )
    .map_err(json_error)?;
    require_completion(root, &checkpoint.area, &checkpoint.task, &records)?;
    let mut paths = checkpoint.records.clone();
    if record.decision == Decision::Accepted {
        paths.extend(checkpoint.paths.clone());
    }
    let tree = code_tree(root, &checkpoint.head, &paths)?;
    let parent = if record.decision == Decision::Accepted && checkpoint.isolated.is_none() {
        &candidate
    } else {
        &checkpoint.head
    };
    let commit = git_output(
        root,
        &[
            "commit-tree",
            &tree,
            "-p",
            parent,
            "-m",
            &format!(
                "research: {}: {}\n\nChange-Id: {}",
                checkpoint.task,
                record.lesson,
                crate::generate_change_id()?
            ),
        ],
    )?;
    checkpoint.commit = Some(commit);
    save(root, checkpoint)?;
    finish(root, checkpoint)
}

fn restore_owned_paths(
    root: &Path,
    checkpoint: &Checkpoint,
    candidate: &str,
) -> Result<(), ZdevError> {
    let baseline = &checkpoint.settings.comparison.baseline_revision;
    // Each path may already be restored after interruption. Refuse anything else before changing it.
    for path in &checkpoint.paths {
        validate_path(root, path)?;
        if !git_output_bytes(root, &["diff", "--cached", "--name-only", "-z", "--", path])?
            .is_empty()
        {
            return Err(ZdevError::new(
                "Owned path has staged work during restoration",
            ));
        }
        let tree = code_tree(root, &checkpoint.head, std::slice::from_ref(path))?;
        let candidate_tree = path_tree(root, &checkpoint.head, candidate, path)?;
        let baseline_tree = path_tree(root, &checkpoint.head, baseline, path)?;
        if tree != candidate_tree && tree != baseline_tree {
            return Err(ZdevError::new(format!(
                "Owned path changed after measurement: {path}"
            )));
        }
    }
    for path in &checkpoint.paths {
        if git_output(
            root,
            &["--literal-pathspecs", "ls-tree", baseline, "--", path],
        )?
        .is_empty()
        {
            if fs::symlink_metadata(root.join(path)).is_ok() {
                fs::remove_file(root.join(path)).map_err(|error| {
                    ZdevError::io(format!("Cannot restore absent baseline path {path}"), error)
                })?;
            }
        } else {
            git_output(
                root,
                &[
                    "--literal-pathspecs",
                    "restore",
                    "--source",
                    baseline,
                    "--worktree",
                    "--",
                    path,
                ],
            )?;
        }
    }
    if code_tree(root, &checkpoint.head, &checkpoint.paths)?
        != git_output(
            root,
            &["rev-parse", &format!("{}^{{tree}}", checkpoint.head)],
        )?
    {
        return Err(ZdevError::new(
            "Owned paths did not restore to the accepted implementation",
        ));
    }
    Ok(())
}

fn path_tree(root: &Path, base: &str, source: &str, path: &str) -> Result<String, ZdevError> {
    let directory = tempfile::tempdir()
        .map_err(|error| ZdevError::io("Cannot create isolated Git index", error))?;
    let index = directory.path().join("index");
    index_git(root, &index, &["read-tree", base])?;
    index_git(
        root,
        &index,
        &["--literal-pathspecs", "reset", "-q", source, "--", path],
    )?;
    index_git(root, &index, &["write-tree"])
}

pub(crate) fn require_completion(
    root: &Path,
    area: &str,
    task: &str,
    records: &[ResultRecord],
) -> Result<(), ZdevError> {
    let checkpoint = load(root, area, task)?;
    let record = checkpoint
        .final_record
        .as_ref()
        .ok_or_else(|| ZdevError::new("Managed experiment has no authorized terminal decision"))?;
    let mut expected = checkpoint
        .attempts
        .iter()
        .map(|attempt| observed_record(&checkpoint, attempt))
        .collect::<Vec<_>>();
    expected.push(record.clone());
    if serde_json::to_value(records).map_err(json_error)?
        != serde_json::to_value(expected).map_err(json_error)?
    {
        return Err(ZdevError::new(
            "Managed terminal result differs from its checkpoint",
        ));
    }
    super::validate_evidence(records, true, None)?;
    for attempt in &checkpoint.attempts {
        if !attempt.published
            || attempt.measuring
            || git_output(root, &["rev-parse", "--verify", &attempt.retained_ref])?
                != attempt.candidate
        {
            return Err(ZdevError::new(
                "Managed experiment candidate retention or measurement is incomplete",
            ));
        }
    }
    let mut completed_settings = checkpoint.settings.clone();
    if record.decision == Decision::Accepted {
        let last = checkpoint.attempts.last().unwrap();
        completed_settings.comparison.baseline_revision = last.candidate.clone();
        completed_settings.comparison.baseline_samples = last.observation.samples.clone();
    }
    if super::settings(root, area)? != completed_settings
        && (record.decision == Decision::Accepted
            || !super::run::record_policy_compatible(root, area, &checkpoint.settings)?)
    {
        return Err(ZdevError::new(
            "Managed completion differs from the experiment's accepted policy",
        ));
    }
    if record.decision == Decision::Accepted {
        if checkpoint.verification_snapshot.is_none()
            || !record
                .verification
                .as_ref()
                .is_some_and(|verification| verification.passed)
            || !candidate_matches(
                root,
                &checkpoint,
                &checkpoint.attempts.last().unwrap().candidate,
            )?
        {
            return Err(ZdevError::new(
                "Managed acceptance requires verified retained candidate code",
            ));
        }
    } else if checkpoint.isolated.is_some() {
        if !checkpoint.restored {
            return Err(ZdevError::new("Isolated rejection is not complete"));
        }
        require_accepted_destination(root, &checkpoint)?;
    } else if !checkpoint.restored
        || code_tree(root, &checkpoint.head, &checkpoint.paths)?
            != git_output(
                root,
                &["rev-parse", &format!("{}^{{tree}}", checkpoint.head)],
            )?
    {
        return Err(ZdevError::new(
            "Managed discard requires restored accepted code",
        ));
    }
    Ok(())
}

fn source_root(checkpoint: &Checkpoint) -> &Path {
    &checkpoint.isolated.as_ref().unwrap().source_root
}

fn source_root_or<'a>(root: &'a Path, checkpoint: &'a Checkpoint) -> &'a Path {
    checkpoint
        .isolated
        .as_ref()
        .map_or(root, |candidate| candidate.source_root.as_path())
}

fn require_clean_index(root: &Path, path: &str) -> Result<(), ZdevError> {
    if !git_output_bytes(root, &["diff", "--cached", "--name-only", "-z", "--", path])?.is_empty() {
        return Err(ZdevError::new(format!(
            "Experiment record has staged work: {path}"
        )));
    }
    Ok(())
}

fn comparison_current(root: &Path, checkpoint: &Checkpoint) -> Result<bool, ZdevError> {
    Ok(super::run::comparison_valid(root, &checkpoint.area)?
        && super::settings(root, &checkpoint.area)? == checkpoint.settings)
}

fn candidate_state(
    root: &Path,
    checkpoint: &Checkpoint,
) -> Result<Option<&'static str>, ZdevError> {
    let Some(candidate) = &checkpoint.isolated else {
        return Ok(None);
    };
    Ok(Some(if checkpoint.index_updated {
        if checkpoint
            .final_record
            .as_ref()
            .is_some_and(|record| record.decision == Decision::Accepted)
        {
            "accepted"
        } else {
            "rejected"
        }
    } else if candidate.refreshing
        || candidate.integrating
        || checkpoint
            .attempts
            .last()
            .is_some_and(|attempt| attempt.measuring && !process_running(attempt.owner_pid))
    {
        "ambiguous"
    } else if !comparison_current(root, checkpoint)? {
        "paused"
    } else if !candidate.worker_stopped {
        "active"
    } else if candidate.returned_commit.is_some() {
        "returned"
    } else {
        "ambiguous"
    }))
}

#[allow(clippy::too_many_arguments)]
fn validate_source(
    root: &Path,
    source: &Path,
    head: &str,
    area: &str,
    task: &str,
    admitted_snapshot: &str,
    settings: &Settings,
    records: &[String],
) -> Result<Candidate, ZdevError> {
    let root = fs::canonicalize(root)
        .map_err(|error| ZdevError::io("Cannot resolve authoritative checkout", error))?;
    let source = fs::canonicalize(source)
        .map_err(|error| ZdevError::io("Cannot resolve source checkout", error))?;
    if root == source || root.starts_with(&source) || source.starts_with(&root) {
        return Err(ZdevError::new(
            "Source must be a distinct linked worktree outside the authoritative checkout",
        ));
    }
    if common_git_directory(&root)? != common_git_directory(&source)?
        || !source.join(".git").is_file()
        || git_output(&source, &["rev-parse", "HEAD"])? != head
    {
        return Err(ZdevError::new(
            "Source must share the repository and admitted HEAD as a linked worktree",
        ));
    }
    let source_branch = git_output(&source, &["symbolic-ref", "HEAD"])?;
    if source_branch == git_output(&root, &["symbolic-ref", "HEAD"])? {
        return Err(ZdevError::new("Source must have its own branch"));
    }
    require_available_source(&root, &source, &source_branch)?;
    if !git_output(&source, &["status", "--porcelain", "--untracked-files=all"])?.is_empty()
        || !git_output_bytes(
            &source,
            &[
                "ls-files",
                "--others",
                "--ignored",
                "--exclude-standard",
                "-z",
                "--",
                ".zdev",
            ],
        )?
        .is_empty()
    {
        return Err(ZdevError::new(
            "Source admission requires a clean worktree and index",
        ));
    }
    Ok(Candidate {
        authoritative_root: root.clone(),
        brief: root.join(format!(".zdev/{area}/brief.md")),
        task_record: root.join(&records[0]),
        repository_instructions: [
            "AGENTS.md",
            "CLAUDE.md",
            "CONTRIBUTING.md",
            ".zdev/guidance.md",
        ]
        .into_iter()
        .map(|path| root.join(path))
        .filter(|path| path.is_file())
        .collect(),
        admitted_snapshot: admitted_snapshot.to_owned(),
        dispatch_head: head.to_owned(),
        dispatch_settings: settings.clone(),
        source_root: source,
        source_branch,
        source_base: head.to_owned(),
        worker_stopped: false,
        returned_commit: None,
        retained_ref: format!("refs/zdev/candidates/{area}/{task}/returned"),
        reconciliation: None,
        refreshing: false,
        refresh_from: None,
        integrating: false,
        integrated: false,
    })
}

fn require_available_source(root: &Path, source: &Path, branch: &str) -> Result<(), ZdevError> {
    let directory = PathBuf::from(git_output(
        root,
        &["rev-parse", "--git-path", "zdev/experiments"],
    )?);
    let directory = if directory.is_absolute() {
        directory
    } else {
        root.join(directory)
    };
    let areas = match fs::read_dir(directory) {
        Ok(areas) => areas,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(ZdevError::io("Cannot inspect experiments", error)),
    };
    for area in areas {
        let area =
            area.map_err(|error| ZdevError::io("Cannot read experiment directory", error))?;
        for entry in fs::read_dir(area.path())
            .map_err(|error| ZdevError::io("Cannot inspect experiment area", error))?
        {
            let entry =
                entry.map_err(|error| ZdevError::io("Cannot read experiment checkpoint", error))?;
            if entry
                .path()
                .extension()
                .is_none_or(|extension| extension != "json")
            {
                continue;
            }
            let checkpoint: Checkpoint = super::read_json(&entry.path())?;
            let Some(candidate) = &checkpoint.isolated else {
                continue;
            };
            if (candidate.source_root == source || candidate.source_branch == branch)
                && (!checkpoint.index_updated
                    || !candidate.worker_stopped
                    || candidate.refreshing
                    || candidate.integrating
                    || checkpoint.attempts.iter().any(|attempt| attempt.measuring))
            {
                return Err(ZdevError::new(format!(
                    "Source is owned by unfinished candidate {}/{}; preserve it until completion and confirmed stop",
                    checkpoint.area, checkpoint.task
                )));
            }
        }
    }
    Ok(())
}

fn admit_candidate(
    root: &Path,
    area: &str,
    task: &str,
    source: &Path,
    paths: &[String],
    reservation: Option<&str>,
) -> Result<(), ZdevError> {
    if exists(root, area, task)? {
        let checkpoint = load(root, area, task)?;
        let candidate = checkpoint
            .isolated
            .as_ref()
            .ok_or_else(|| ZdevError::new("Existing experiment is local"))?;
        let source = fs::canonicalize(source)
            .map_err(|error| ZdevError::io("Cannot resolve source", error))?;
        if candidate.source_root != source || checkpoint.paths != paths {
            return Err(ZdevError::new(
                "Changed source or owned paths under existing admission",
            ));
        }
        return Ok(()); // The durable admission never authorizes a second worker dispatch.
    }
    begin_at(root, area, task, paths, reservation, Some(source))
}

fn prepare_source(
    root: &Path,
    checkpoint: &mut Checkpoint,
    reservation: &str,
) -> Result<bool, ZdevError> {
    let candidate = checkpoint
        .isolated
        .as_ref()
        .ok_or_else(|| ZdevError::new("Prepare requires an isolated source"))?;
    if !candidate.worker_stopped {
        if checkpoint
            .preparation
            .as_ref()
            .is_some_and(|binding| binding.request == reservation)
        {
            return Ok(false);
        }
        return Err(ZdevError::new(
            "Source worker is already active or unresolved; no worker was admitted",
        ));
    }
    if checkpoint.final_record.is_some()
        || candidate.integrating
        || candidate.integrated
        || checkpoint
            .attempts
            .last()
            .is_some_and(|attempt| attempt.measuring)
    {
        return Err(ZdevError::new(
            "Recover unfinished measurement or acceptance before source repair",
        ));
    }
    require_environment(root, checkpoint)?;
    let preparation = super::run::bind_work_with_capacity(
        root,
        &checkpoint.area,
        &checkpoint.task,
        reservation,
        super::run::WorkKind::Repair,
        Some(reservation),
        true,
    )?;
    checkpoint.preparation = preparation;
    checkpoint.isolated.as_mut().unwrap().worker_stopped = false;
    checkpoint.verification_snapshot = None;
    save(root, checkpoint)?;
    Ok(true)
}

fn common_git_directory(root: &Path) -> Result<PathBuf, ZdevError> {
    let path = git_output(
        root,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?;
    fs::canonicalize(path)
        .map_err(|error| ZdevError::io("Cannot resolve shared Git repository", error))
}

fn require_source(checkpoint: &Checkpoint) -> Result<(), ZdevError> {
    let candidate = checkpoint.isolated.as_ref().unwrap();
    let source = &candidate.source_root;
    if common_git_directory(source)? != common_git_directory(&candidate.authoritative_root)? {
        return Err(ZdevError::new(
            "Source no longer shares the authoritative Git repository",
        ));
    }
    if git_output(source, &["symbolic-ref", "HEAD"])? != candidate.source_branch
        || git_output(source, &["rev-parse", "HEAD"])? != candidate.source_base
    {
        return Err(ZdevError::new(
            "Source branch or HEAD changed outside the candidate lifecycle",
        ));
    }
    for path in &checkpoint.paths {
        validate_path(source, path)?;
    }
    Ok(())
}

fn require_source_delta(checkpoint: &Checkpoint) -> Result<(), ZdevError> {
    require_source(checkpoint)?;
    let candidate = checkpoint.isolated.as_ref().unwrap();
    let source = &candidate.source_root;
    let changed = git_output_bytes(
        source,
        &["diff", "--name-only", "-z", &candidate.source_base],
    )?;
    let staged = git_output_bytes(
        source,
        &[
            "diff",
            "--cached",
            "--name-only",
            "-z",
            &candidate.source_base,
        ],
    )?;
    let untracked = git_output_bytes(
        source,
        &["ls-files", "--others", "--exclude-standard", "-z"],
    )?;
    let ignored = git_output_bytes(
        source,
        &[
            "ls-files",
            "--others",
            "--ignored",
            "--exclude-standard",
            "-z",
            "--",
            ".zdev",
        ],
    )?;
    for bytes in [&changed, &staged, &untracked, &ignored] {
        for path in bytes
            .split(|byte| *byte == 0)
            .filter(|path| !path.is_empty())
        {
            let path = std::str::from_utf8(path)
                .map_err(|_| ZdevError::new("Source contains a non-UTF-8 changed path"))?;
            if !checkpoint.paths.iter().any(|owned| owned == path) {
                return Err(ZdevError::new(format!(
                    "Source has unowned or research record changes: {path}; changes were preserved"
                )));
            }
        }
    }
    if !git_output(source, &["ls-files", "--unmerged"])?.is_empty() {
        return Err(ZdevError::new(
            "Source integration conflicts remain; repair and stage their resolution before return",
        ));
    }
    Ok(())
}

fn return_candidate(
    root: &Path,
    checkpoint: &mut Checkpoint,
    worker_stopped: bool,
) -> Result<(), ZdevError> {
    let candidate = checkpoint
        .isolated
        .as_ref()
        .ok_or_else(|| ZdevError::new("Return requires an isolated candidate"))?;
    if !worker_stopped
        || candidate.integrating
        || candidate.integrated
        || checkpoint.final_record.is_some()
        || checkpoint
            .attempts
            .last()
            .is_some_and(|attempt| attempt.measuring)
    {
        return Err(ZdevError::new(
            "Confirm stopped source work and settle measurements before return",
        ));
    }
    require_source_delta(checkpoint)?;
    if let Some(commit) = &candidate.returned_commit
        && candidate.worker_stopped
        && !candidate.refreshing
        && candidate_matches(root, checkpoint, commit)?
    {
        return settle_checkpoint(root, checkpoint);
    }
    let tree = code_tree(
        source_root(checkpoint),
        &candidate.source_base,
        &checkpoint.paths,
    )?;
    let commit = git_output(
        root,
        &[
            "commit-tree",
            &tree,
            "-p",
            &candidate.source_base,
            "-m",
            &format!("isolated candidate {}", checkpoint.task),
        ],
    )?;
    // A commit-addressed reference keeps every returned version accessible after reconciliation.
    git_output(
        root,
        &[
            "update-ref",
            &format!(
                "refs/zdev/candidates/{}/{}/{}",
                checkpoint.area, checkpoint.task, commit
            ),
            &commit,
        ],
    )?;
    let candidate = checkpoint.isolated.as_mut().unwrap();
    candidate.returned_commit = Some(commit.clone());
    candidate.worker_stopped = true;
    candidate.refreshing = false;
    candidate.refresh_from = None;
    checkpoint.verification_snapshot = None;
    save(root, checkpoint)?;
    git_output(
        root,
        &[
            "update-ref",
            &checkpoint.isolated.as_ref().unwrap().retained_ref,
            &commit,
        ],
    )?;
    settle_checkpoint(root, checkpoint)
}

fn reconcile(
    root: &Path,
    checkpoint: &mut Checkpoint,
    reservation: Option<&str>,
) -> Result<(), ZdevError> {
    require_environment(root, checkpoint)?;
    if checkpoint.final_record.is_some()
        || checkpoint
            .attempts
            .last()
            .is_some_and(|attempt| attempt.measuring)
    {
        return Err(ZdevError::new(
            "Recover unfinished measurement or finalization before reconciliation",
        ));
    }
    let candidate = checkpoint
        .isolated
        .as_ref()
        .ok_or_else(|| ZdevError::new("Reconcile requires an isolated candidate"))?;
    if !candidate.worker_stopped
        || candidate.returned_commit.is_none()
        || candidate.integrated
        || candidate.integrating
    {
        return Err(ZdevError::new(
            "Reconcile requires exclusively owned stopped returned source work",
        ));
    }
    if candidate.refreshing {
        return Err(ZdevError::new(
            "Source refresh is unfinished; preserve conflicts, repair and return stopped work",
        ));
    }
    if !super::run::comparison_valid(root, &checkpoint.area)? {
        return Err(ZdevError::new(
            "Evaluation conditions require an actual counted rebaseline before reconciliation",
        ));
    }
    require_source_delta(checkpoint)?;
    if !candidate_matches(
        root,
        checkpoint,
        candidate.returned_commit.as_ref().unwrap(),
    )? {
        return Err(ZdevError::new("Source changed after return"));
    }
    let head = git_output(root, &["rev-parse", "HEAD"])?;
    let settings = super::settings(root, &checkpoint.area)?;
    if checkpoint.head == head && checkpoint.settings == settings {
        return Ok(());
    }
    let code_changed = !git_output_bytes(
        root,
        &[
            "diff",
            "--name-only",
            "-z",
            &checkpoint.head,
            &head,
            "--",
            ".",
            ":(exclude).zdev",
        ],
    )?
    .is_empty();
    let measured = checkpoint
        .attempts
        .last()
        .filter(|attempt| !attempt.measuring)
        .map(|attempt| attempt.candidate.clone());
    if code_changed || checkpoint.settings != settings {
        let binding = super::run::bind_work(
            root,
            &checkpoint.area,
            &checkpoint.task,
            &format!(
                "experiment:{}:reconcile:{}",
                checkpoint.task,
                checkpoint.attempts.len() + 1
            ),
            super::run::WorkKind::Reconciliation,
            reservation,
        )?;
        checkpoint.isolated.as_mut().unwrap().reconciliation = binding;
    }
    let from = checkpoint.isolated.as_ref().unwrap().source_base.clone();
    let candidate = checkpoint.isolated.as_mut().unwrap();
    candidate.refreshing = true;
    candidate.refresh_from = Some(from);
    candidate.source_base = head.clone();
    checkpoint.head = head;
    checkpoint.settings = settings;
    checkpoint.verification_snapshot = None;
    save(root, checkpoint)?;
    refresh_source(root, checkpoint)?;
    if !code_changed
        && checkpoint
            .isolated
            .as_ref()
            .unwrap()
            .reconciliation
            .is_none()
    {
        // Record-only commits leave the measured code unchanged; the returned transport has new records.
        if let Some(measured) = measured
            && !git_output_bytes(
                root,
                &[
                    "diff",
                    "--name-only",
                    "-z",
                    &measured,
                    checkpoint
                        .isolated
                        .as_ref()
                        .unwrap()
                        .returned_commit
                        .as_ref()
                        .unwrap(),
                    "--",
                    ".",
                    ":(exclude).zdev",
                ],
            )?
            .is_empty()
        {
            return Err(ZdevError::new("Record-only refresh changed measured code"));
        }
    }
    Ok(())
}

fn refresh_source(root: &Path, checkpoint: &mut Checkpoint) -> Result<(), ZdevError> {
    let candidate = checkpoint.isolated.as_ref().unwrap();
    if !candidate.worker_stopped || !candidate.refreshing {
        return Err(ZdevError::new("Source refresh is not authorized"));
    }
    let source = candidate.source_root.clone();
    if common_git_directory(&source)? != common_git_directory(root)? {
        return Err(ZdevError::new("Source repository changed during refresh"));
    }
    let from = candidate
        .refresh_from
        .clone()
        .ok_or_else(|| ZdevError::new("Missing source refresh base"))?;
    let target = candidate.source_base.clone();
    let transport = candidate.returned_commit.clone().unwrap();
    let actual = git_output(&source, &["rev-parse", "HEAD"])?;
    if git_output(&source, &["symbolic-ref", "HEAD"])? != candidate.source_branch
        || (actual != from && actual != target)
    {
        return Err(ZdevError::new(
            "Source identity changed during refresh; work was preserved",
        ));
    }
    if actual != target {
        // Validate the complete old source before resetting exclusively owned stopped work.
        checkpoint.isolated.as_mut().unwrap().source_base = from;
        let valid = require_source_delta(checkpoint).and_then(|_| {
            if candidate_matches(root, checkpoint, &transport)? {
                Ok(())
            } else {
                Err(ZdevError::new("Source changed during refresh"))
            }
        });
        checkpoint.isolated.as_mut().unwrap().source_base = target.clone();
        valid?;
        git_output(&source, &["reset", "--hard", &target])?;
    }
    if !git_output(&source, &["ls-files", "--unmerged"])?.is_empty() {
        return Err(ZdevError::new(
            "Source refresh conflicts remain; repair and return stopped work",
        ));
    }
    let clean =
        git_output(&source, &["status", "--porcelain", "--untracked-files=all"])?.is_empty();
    if clean {
        git_output(&source, &["cherry-pick", "--no-commit", &transport])?;
    }
    // A completed application or an explicitly resolved source is retained; measurement is still required.
    return_candidate(root, checkpoint, true)
}

fn require_accepted_destination(root: &Path, checkpoint: &Checkpoint) -> Result<(), ZdevError> {
    let accepted = super::settings(root, &checkpoint.area)?
        .comparison
        .baseline_revision;
    if !git_output_bytes(
        root,
        &[
            "diff",
            "--name-only",
            "-z",
            &accepted,
            "HEAD",
            "--",
            ".",
            ":(exclude).zdev",
        ],
    )?
    .is_empty()
    {
        return Err(ZdevError::new(
            "Destination HEAD does not contain the current accepted implementation",
        ));
    }
    for path in &checkpoint.paths {
        require_clean_path(root, path)?;
    }
    Ok(())
}

fn require_candidate_index(
    root: &Path,
    checkpoint: &Checkpoint,
    path: &str,
) -> Result<(), ZdevError> {
    if checkpoint
        .isolated
        .as_ref()
        .is_some_and(|candidate| candidate.integrated)
    {
        let candidate = checkpoint.attempts.last().unwrap();
        if !git_output_bytes(
            root,
            &[
                "diff",
                "--cached",
                "--name-only",
                "-z",
                &candidate.candidate,
                "--",
                path,
            ],
        )?
        .is_empty()
        {
            return Err(ZdevError::new(format!(
                "Expected candidate staging changed: {path}"
            )));
        }
        Ok(())
    } else {
        require_clean_index(root, path)
    }
}

fn integrate(root: &Path, checkpoint: &mut Checkpoint) -> Result<(), ZdevError> {
    require_environment(root, checkpoint)?;
    if checkpoint.final_record.is_some() {
        return Err(ZdevError::new("Recover authorized finalization"));
    }
    let candidate = checkpoint
        .isolated
        .as_ref()
        .ok_or_else(|| ZdevError::new("Integrate requires an isolated candidate"))?;
    if !candidate.worker_stopped || candidate.refreshing || candidate.returned_commit.is_none() {
        return Err(ZdevError::new(
            "Integration requires a stopped returned candidate",
        ));
    }
    let head = git_output(root, &["rev-parse", "HEAD"])?;
    if candidate.integrated
        && checkpoint.head != head
        && comparison_current(root, checkpoint)?
        && git_output_bytes(
            root,
            &[
                "diff",
                "--name-only",
                "-z",
                &checkpoint.head,
                &head,
                "--",
                ".",
                ":(exclude).zdev",
            ],
        )?
        .is_empty()
    {
        for path in &checkpoint.paths {
            require_candidate_index(root, checkpoint, path)?;
        }
        if !candidate_matches(
            root,
            checkpoint,
            &checkpoint.attempts.last().unwrap().candidate,
        )? {
            return Err(ZdevError::new(
                "Integrated candidate changed during record-only refresh",
            ));
        }
        checkpoint.head = head;
        checkpoint.verification_snapshot =
            Some(snapshot(root, &checkpoint.area, &checkpoint.task)?);
        return save(root, checkpoint);
    }
    if !comparison_current(root, checkpoint)? || checkpoint.head != head {
        return Err(ZdevError::new(
            "Baseline or conditions changed; reconcile and measure against the current baseline before integration",
        ));
    }
    if !tasks::task_is_ready(root, &checkpoint.area, &checkpoint.task)? {
        return Err(ZdevError::new(
            "Integration requires the same ready open task",
        ));
    }
    let last = checkpoint
        .attempts
        .last()
        .ok_or_else(|| ZdevError::new("Integration requires fresh measurements"))?;
    if last.measuring
        || !last.published
        || last.settings.as_ref().unwrap_or(&checkpoint.settings) != &checkpoint.settings
        || !super::compare(&checkpoint.settings.comparison, &last.observation)?.improved
    {
        return Err(ZdevError::new(
            "Integration requires a current passing improving measurement",
        ));
    }
    require_source_delta(checkpoint)?;
    if !candidate_matches(&candidate.source_root, checkpoint, &last.candidate)? {
        return Err(ZdevError::new("Returned source changed after measurement"));
    }
    retain(root, last)?;
    let transport = last.candidate.clone();
    if candidate.integrated {
        return Ok(());
    }
    let was_integrating = candidate.integrating;
    let unchanged_destination = checkpoint
        .paths
        .iter()
        .all(|path| require_clean_path(root, path).is_ok());
    if !was_integrating || unchanged_destination {
        for path in &checkpoint.paths {
            validate_path(root, path)?;
            require_clean_path(root, path)?;
        }
        super::run::acceptance(root, &checkpoint.area, &checkpoint.task, true)?;
        checkpoint.isolated.as_mut().unwrap().integrating = true;
        save(root, checkpoint)?;
        git_output(root, &["cherry-pick", "--no-commit", &transport])?;
    }
    if !git_output(root, &["ls-files", "--unmerged"])?.is_empty() {
        return Err(ZdevError::new(
            "Destination integration conflicts are preserved; repair before recovery",
        ));
    }
    if code_tree(root, &checkpoint.head, &checkpoint.paths)?
        != git_output(root, &["rev-parse", &format!("{transport}^{{tree}}")])?
    {
        if was_integrating {
            return Err(ZdevError::new(
                "Unfinished integration differs from the measured candidate; preserve and inspect destination state",
            ));
        }
        return Err(ZdevError::new(
            "Integration differs from the measured candidate",
        ));
    }
    let candidate = checkpoint.isolated.as_mut().unwrap();
    candidate.integrating = false;
    candidate.integrated = true;
    for path in &checkpoint.paths {
        require_candidate_index(root, checkpoint, path)?;
    }
    checkpoint.verification_snapshot = Some(snapshot(root, &checkpoint.area, &checkpoint.task)?);
    save(root, checkpoint)
}

fn json_error(error: serde_json::Error) -> ZdevError {
    ZdevError::new(format!("Cannot compare experiment evidence: {error}"))
}

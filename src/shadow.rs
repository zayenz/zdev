//! Shadow records: `.zdev` versioned in its own nested Git repository.
//!
//! Toward the main repository a shadow record behaves like a personal record:
//! `.zdev` is untracked and excluded. Inside `.zdev`, a separate repository
//! keeps the record's history. Its checked-out branch mirrors the main
//! repository's branch. zdev manages the shadow branches whenever it drives
//! Git itself, and follows other branch changes on a best-effort basis.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command as ProcessCommand, Output};

use serde_json::{Value, json};

use super::config::{self, ShadowPush};
use super::project::{
    self, Config, RecordPolicy, branch_tip, commit_is_ancestor, current_branch, local_branch_exists,
};
use super::{
    CHANGE_ID_TRAILER, CommandOutput, SCHEMA_VERSION, ZdevError, ZdevStateLock, generate_change_id,
    git_operation, git_output, validate_nonempty_line, write_atomic,
};

pub(super) const SOURCE_COMMIT_TRAILER: &str = "Zdev-Source-Commit";
pub(super) const SOURCE_BRANCH_TRAILER: &str = "Zdev-Source-Branch";
const EXCLUDE_RULE: &str = "/.zdev/";
const INITIAL_SUBJECT: &str = "Initialize the zdev shadow record";
const ADOPT_SUBJECT: &str = "Adopt the zdev record as a shadow record";

/// The directory that holds both the record and the shadow repository.
pub(super) fn dir(root: &Path) -> PathBuf {
    root.join(".zdev")
}

/// Whether `.zdev` has its own Git repository.
pub(super) fn has_repository(root: &Path) -> bool {
    dir(root).join(".git").exists()
}

/// Whether the project's record policy is `shadow`. An unreadable
/// configuration counts as not shadow; the command itself reports it.
pub(super) fn is_shadow(root: &Path) -> bool {
    project::read_config(root).is_ok_and(|config| config.project.record == RecordPolicy::Shadow)
}

/// The shadow repository directory, or an error when it is missing.
pub(super) fn repository(root: &Path) -> Result<PathBuf, ZdevError> {
    if has_repository(root) {
        Ok(dir(root))
    } else {
        Err(ZdevError::new(
            "The shadow record has no Git repository at .zdev/.git. Clone the shadow remote into .zdev or restore the repository, then retry",
        ))
    }
}

/// The repository a Git command runs in. The two differ only in
/// `GIT_INDEX_FILE`: a Git hook in the main repository may set it to an
/// alternate main index, which main-repository commands keep, as zdev's other
/// main-repository commands do, and shadow commands must never read.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Repo {
    Main,
    Shadow,
}

impl Repo {
    fn git(self, directory: &Path, arguments: &[&str]) -> Result<Output, ZdevError> {
        let mut command = ProcessCommand::new("git");
        command
            .args(arguments)
            .current_dir(directory)
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE");
        if self == Self::Shadow {
            command.env_remove("GIT_INDEX_FILE");
        }
        command
            .output()
            .map_err(|error| ZdevError::io("Cannot run git", error))
    }

    /// Runs Git and returns its trimmed standard output, or an error naming
    /// the command.
    fn output(self, directory: &Path, arguments: &[&str]) -> Result<String, ZdevError> {
        let output = self.git(directory, arguments)?;
        if !output.status.success() {
            return Err(ZdevError::new(format!(
                "git {} failed: {}",
                arguments.join(" "),
                stderr(&output)
            )));
        }
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    }
}

/// Runs Git in the shadow repository.
fn git(shadow: &Path, arguments: &[&str]) -> Result<Output, ZdevError> {
    Repo::Shadow.git(shadow, arguments)
}

/// Runs Git in the main repository.
fn main_git(root: &Path, arguments: &[&str]) -> Result<Output, ZdevError> {
    Repo::Main.git(root, arguments)
}

/// Runs Git in the shadow repository and returns its trimmed output.
fn shadow_output(shadow: &Path, arguments: &[&str]) -> Result<String, ZdevError> {
    Repo::Shadow.output(shadow, arguments)
}

fn stderr(output: &Output) -> String {
    let error = String::from_utf8_lossy(&output.stderr).trim().to_owned();
    if error.is_empty() {
        String::from_utf8_lossy(&output.stdout).trim().to_owned()
    } else {
        error
    }
}

/// Runs Git in the shadow repository and turns a failure into `what: <error>`.
fn git_checked(shadow: &Path, arguments: &[&str], what: &str) -> Result<(), ZdevError> {
    let output = git(shadow, arguments)?;
    if output.status.success() {
        Ok(())
    } else {
        Err(ZdevError::new(format!("{what}: {}", stderr(&output))))
    }
}

fn has_changes(shadow: &Path) -> Result<bool, ZdevError> {
    Ok(!shadow_output(
        shadow,
        &["status", "--porcelain=v1", "--untracked-files=all"],
    )?
    .is_empty())
}

fn change_count(shadow: &Path) -> Result<usize, ZdevError> {
    Ok(shadow_output(
        shadow,
        &["status", "--porcelain=v1", "--untracked-files=all"],
    )?
    .lines()
    .count())
}

/// Whether the shadow record has uncommitted changes.
pub(super) fn has_pending_changes(root: &Path) -> Result<bool, ZdevError> {
    has_changes(&repository(root)?)
}

fn head(shadow: &Path) -> Result<String, ZdevError> {
    shadow_output(shadow, &["rev-parse", "HEAD"])
}

/// Copy the main repository's own commit identity into the shadow
/// repository, so shadow commits carry the same author as main commits.
/// Global identity needs no copy; Git applies it to both repositories.
fn inherit_identity(root: &Path, shadow: &Path) -> Result<(), ZdevError> {
    for key in ["user.name", "user.email"] {
        let output = main_git(root, &["config", "--local", "--get", key])?;
        let value = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        if output.status.success() && !value.is_empty() {
            git_checked(
                shadow,
                &["config", "--local", key, &value],
                "Cannot configure the shadow repository",
            )?;
        }
    }
    Ok(())
}

fn require_untracked_in_main(root: &Path) -> Result<(), ZdevError> {
    if !git_output(root, &["ls-files", "--", ".zdev"])?.is_empty() {
        return Err(ZdevError::new(
            "Cannot use a shadow record while the repository tracks .zdev files. Remove them from the main index with `git rm -r --cached .zdev`, commit, then retry",
        ));
    }
    Ok(())
}

fn tracked_in_main(root: &Path) -> bool {
    git_output(root, &["ls-files", "--", ".zdev"]).is_ok_and(|tracked| !tracked.is_empty())
}

fn excluded(root: &Path) -> Result<bool, ZdevError> {
    let output = main_git(root, &["check-ignore", "-q", ".zdev/"])?;
    match output.status.code() {
        Some(0) => Ok(true),
        Some(1) => Ok(false),
        _ => Err(ZdevError::new(format!(
            "Cannot check whether .zdev is ignored: {}",
            stderr(&output)
        ))),
    }
}

/// Adds `/.zdev/` to the main repository's `info/exclude` unless `.zdev` is
/// already ignored. Returns whether the rule was added.
fn ensure_excluded(root: &Path) -> Result<bool, ZdevError> {
    if excluded(root)? {
        return Ok(false);
    }
    let path = PathBuf::from(git_output(
        root,
        &["rev-parse", "--git-path", "info/exclude"],
    )?);
    let path = if path.is_absolute() {
        path
    } else {
        root.join(path)
    };
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| ZdevError::io(format!("Cannot create {}", parent.display()), error))?;
    }
    let existing = match fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
        Err(error) => {
            return Err(ZdevError::io(
                format!("Cannot read {}", path.display()),
                error,
            ));
        }
    };
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|error| ZdevError::io(format!("Cannot open {}", path.display()), error))?;
    let separator = if existing.is_empty() || existing.ends_with(b"\n") {
        ""
    } else {
        "\n"
    };
    writeln!(file, "{separator}{EXCLUDE_RULE}")
        .map_err(|error| ZdevError::io(format!("Cannot write {}", path.display()), error))?;
    Ok(true)
}

fn remote_has_history(root: &Path, url: &str) -> Result<bool, ZdevError> {
    let output = main_git(root, &["ls-remote", "--heads", url])?;
    if !output.status.success() {
        return Err(ZdevError::new(format!(
            "Cannot read shadow remote {url}: {}",
            stderr(&output)
        )));
    }
    Ok(!output.stdout.is_empty())
}

/// Makes a remote given as a relative local path absolute, so it still
/// resolves from inside `.zdev`. URLs and absolute paths stay unchanged.
fn resolve_remote(url: &str) -> String {
    let path = Path::new(url);
    if url.contains("://") || path.is_absolute() {
        return url.to_owned();
    }
    match std::env::current_dir().map(|current| current.join(path)) {
        Ok(candidate) if candidate.exists() => fs::canonicalize(&candidate)
            .unwrap_or(candidate)
            .to_string_lossy()
            .into_owned(),
        _ => url.to_owned(),
    }
}

pub(super) struct Setup {
    pub text: String,
    pub value: Value,
    pub cloned: bool,
}

/// Creates the shadow record for `zdev init --record shadow`. With a remote
/// that already holds a shadow record, clones it; otherwise writes the initial
/// configuration from `config`, creates the repository, and commits it.
pub(super) fn initialize(
    root: &Path,
    remote: Option<&str>,
    config: impl FnOnce() -> Result<Config, ZdevError>,
) -> Result<Setup, ZdevError> {
    require_untracked_in_main(root)?;
    let remote = remote.map(resolve_remote);
    let remote = remote.as_deref();
    if let Some(url) = remote
        && remote_has_history(root, url)?
    {
        return clone(root, url);
    }
    let config = config()?;
    let shadow = dir(root);
    fs::create_dir(&shadow)
        .map_err(|error| ZdevError::io(format!("Cannot create {}", shadow.display()), error))?;
    let created = write_atomic(
        &shadow.join("config.toml"),
        toml::to_string_pretty(&config).unwrap().as_bytes(),
    )
    .and_then(|()| create_repository(root, remote, INITIAL_SUBJECT))
    .map_err(|error| discard_new_record(root, error))?;
    Ok(Setup {
        text: created.text(),
        value: created.value(),
        cloned: false,
    })
}

/// Removes the `.zdev` that a failed `init --record shadow` created, so the
/// command can simply be retried, and returns the error to report.
fn discard_new_record(root: &Path, error: ZdevError) -> ZdevError {
    match fs::remove_dir_all(dir(root)) {
        Ok(()) => ZdevError::new(format!(
            "Could not create the shadow record, so zdev removed .zdev again. {error}"
        )),
        Err(removal) => ZdevError::new(format!(
            "Could not create the shadow record: {error}. Removing the partial .zdev also failed ({removal}); remove it, then retry"
        )),
    }
}

/// Converts a personal record in place into a shadow record.
pub(super) fn adopt(root: &Path, remote: Option<&str>) -> Result<CommandOutput, ZdevError> {
    let _lock = ZdevStateLock::acquire(root)?;
    let mut config = project::read_config(root)?;
    if config.project.record != RecordPolicy::Personal {
        return Err(ZdevError::new(format!(
            "Cannot adopt a {} record; only a personal record can become a shadow record",
            config.project.record.as_str()
        )));
    }
    if has_repository(root) {
        return Err(ZdevError::new(
            "Cannot adopt: .zdev/.git already exists. Inspect or remove that repository, then retry",
        ));
    }
    require_untracked_in_main(root)?;
    let remote = remote.map(resolve_remote);
    let remote = remote.as_deref();
    if let Some(url) = remote
        && remote_has_history(root, url)?
    {
        return Err(ZdevError::new(format!(
            "Cannot adopt into {url}: the remote already has history. Adopt into an empty remote, or move this record aside and clone the remote with `zdev init --record shadow --shadow-remote {url}`"
        )));
    }
    config.project.record = RecordPolicy::Shadow;
    project::write_config(root, &config)?;
    let created = match create_repository(root, remote, ADOPT_SUBJECT) {
        Ok(created) => created,
        Err(error) => {
            config.project.record = RecordPolicy::Personal;
            let restore = project::write_config(root, &config);
            let removed = fs::remove_dir_all(dir(root).join(".git"));
            return Err(match (restore, removed) {
                (Ok(()), Ok(())) => ZdevError::new(format!(
                    "Could not adopt the record; it remains personal. {error}"
                )),
                _ => ZdevError::new(format!(
                    "Could not adopt the record, and restoring the personal record was incomplete. Check .zdev/config.toml and remove .zdev/.git. {error}"
                )),
            });
        }
    };
    let mut value = created.value();
    value["schema_version"] = json!(SCHEMA_VERSION);
    value["status"] = json!("adopted");
    value["record"] = json!("shadow");
    Ok(CommandOutput::new(
        format!(
            "Converted the personal record into a shadow record. {}",
            created.text()
        ),
        value,
    ))
}

struct Created {
    branch: Option<String>,
    commit: String,
    change_id: String,
    remote: Option<String>,
    excluded: bool,
    push: PushOutcome,
}

impl Created {
    fn text(&self) -> String {
        let mut text = match &self.branch {
            Some(branch) => format!(
                "Committed the record in the shadow repository at .zdev on branch {branch} ({}).",
                short(&self.commit)
            ),
            None => format!(
                "Committed the record in the shadow repository at .zdev ({}).",
                short(&self.commit)
            ),
        };
        if self.excluded {
            text.push_str(" Added /.zdev/ to the repository's info/exclude.");
        }
        if let Some(remote) = &self.remote {
            text.push_str(&format!(" Shadow remote: {remote}."));
        }
        if let Some(push) = self.push.text() {
            text.push(' ');
            text.push_str(&push);
        }
        text
    }

    fn value(&self) -> Value {
        json!({
            "repository": ".zdev",
            "branch": self.branch,
            "commit": self.commit,
            "change_id": self.change_id,
            "remote": self.remote,
            "exclude_added": self.excluded,
            "push": self.push.value(),
        })
    }
}

fn create_repository(
    root: &Path,
    remote: Option<&str>,
    subject: &str,
) -> Result<Created, ZdevError> {
    let shadow = dir(root);
    let branch = current_branch(root)?;
    let initial_branch = branch
        .as_ref()
        .map(|branch| format!("--initial-branch={branch}"));
    let mut init = vec!["init", "-q"];
    if let Some(argument) = &initial_branch {
        init.push(argument);
    }
    git_checked(&shadow, &init, "Cannot create the shadow repository")?;
    inherit_identity(root, &shadow)?;
    let excluded = ensure_excluded(root)?;
    if let Some(url) = remote {
        git_checked(
            &shadow,
            &["remote", "add", "origin", url],
            "Cannot add the shadow remote",
        )?;
    }
    let change_id = generate_change_id()?;
    git_checked(&shadow, &["add", "-A"], "Cannot stage the shadow record")?;
    git_checked(
        &shadow,
        &[
            "commit",
            "-q",
            "-m",
            subject,
            "-m",
            &format!("{CHANGE_ID_TRAILER}: {change_id}"),
        ],
        "Cannot make the first shadow commit",
    )?;
    let commit = head(&shadow)?;
    let branch = current_branch(&shadow)?;
    let push = match &branch {
        Some(branch) => push_after_commit(root, &shadow, branch),
        None => PushOutcome::Disabled,
    };
    Ok(Created {
        branch,
        commit,
        change_id,
        remote: remote.map(str::to_owned),
        excluded,
        push,
    })
}

/// The remote branch a fresh shadow clone starts on: the main repository's
/// branch when the remote has it, else the remote's default branch, else its
/// only branch.
fn clone_branch(root: &Path, clone: &Path) -> Result<String, ZdevError> {
    let refs = shadow_output(
        clone,
        &[
            "for-each-ref",
            "--format=%(refname:short)",
            "refs/remotes/origin",
        ],
    )?;
    let branches = refs
        .lines()
        .filter_map(|name| name.strip_prefix("origin/"))
        .filter(|name| *name != "HEAD")
        .collect::<Vec<_>>();
    if let Some(branch) = current_branch(root)?
        && branches.contains(&branch.as_str())
    {
        return Ok(branch);
    }
    if let Ok(default) = shadow_output(
        clone,
        &[
            "symbolic-ref",
            "--quiet",
            "--short",
            "refs/remotes/origin/HEAD",
        ],
    ) && let Some(default) = default.strip_prefix("origin/")
        && branches.contains(&default)
    {
        return Ok(default.to_owned());
    }
    match branches.as_slice() {
        [single] => Ok((*single).to_owned()),
        _ => Err(ZdevError::new(format!(
            "Cannot choose a shadow branch to start from: the remote has no default branch and several branches ({}). Check out this repository's matching branch, then retry",
            branches.join(", ")
        ))),
    }
}

fn clone(root: &Path, url: &str) -> Result<Setup, ZdevError> {
    let stage = tempfile::Builder::new()
        .prefix(".zdev-shadow-clone-")
        .tempdir_in(root)
        .map_err(|error| ZdevError::io("Cannot stage the shadow clone", error))?;
    let target = stage.path().join("record");
    let target_text = target.to_string_lossy().into_owned();
    git_checked(
        root,
        &["clone", "-q", "--no-checkout", url, &target_text],
        &format!("Cannot clone shadow remote {url}"),
    )?;
    let branch = clone_branch(root, &target)?;
    git_checked(
        &target,
        &["checkout", "-q", "-B", &branch, &format!("origin/{branch}")],
        &format!("Cannot check out shadow branch {branch}"),
    )?;
    git_checked(
        &target,
        &[
            "branch",
            "-q",
            &format!("--set-upstream-to=origin/{branch}"),
        ],
        &format!("Cannot track origin/{branch}"),
    )?;
    inherit_identity(root, &target)?;
    let config_path = target.join("config.toml");
    let text = fs::read_to_string(&config_path).map_err(|error| {
        ZdevError::io(
            format!("Shadow remote {url} has no readable config.toml on its default branch"),
            error,
        )
    })?;
    let config: Config = toml::from_str(&text).map_err(|error| {
        ZdevError::new(format!(
            "Shadow remote {url} has an invalid config.toml: {error}"
        ))
    })?;
    if config.project.record != RecordPolicy::Shadow {
        return Err(ZdevError::new(format!(
            "Shadow remote {url} holds a {} record, not a shadow record",
            config.project.record.as_str()
        )));
    }
    fs::rename(&target, dir(root))
        .map_err(|error| ZdevError::io("Cannot move the shadow clone into .zdev", error))?;
    let (excluded, notices) = ensure_excluded(root)
        .and_then(|excluded| Ok((excluded, align_locked(root)?)))
        .map_err(|error| discard_new_record(root, error))?;
    let branch = current_branch(&dir(root))?;
    let mut text = format!("Cloned the shadow record from {url}.");
    if let Some(branch) = &branch {
        text.push_str(&format!(" Shadow branch: {branch}."));
    }
    if excluded {
        text.push_str(" Added /.zdev/ to the repository's info/exclude.");
    }
    Ok(Setup {
        text,
        value: json!({
            "repository": ".zdev",
            "cloned_from": url,
            "branch": branch,
            "exclude_added": excluded,
            "notices": notices,
        }),
        cloned: true,
    })
}

/// Makes the shadow repository follow the main repository's branch, then
/// integrates merged shadow branches that fast-forward cleanly. Does nothing
/// for other record policies. Returns notices describing what changed.
pub(super) fn align(root: &Path) -> Result<Vec<String>, ZdevError> {
    if !is_shadow(root) || !has_repository(root) {
        return Ok(Vec::new());
    }
    let _lock = ZdevStateLock::acquire(root)?;
    align_locked(root)
}

/// [`align`] for callers that already hold the state lock.
pub(super) fn align_locked(root: &Path) -> Result<Vec<String>, ZdevError> {
    let shadow = repository(root)?;
    if let Some(operation) = git_operation(&shadow)? {
        return Ok(vec![format!(
            "The shadow record has a {operation} in progress; finish it in .zdev so zdev can follow branch changes again"
        )]);
    }
    let Some(branch) = current_branch(root)? else {
        restore_task_directories(&shadow)?;
        return Ok(Vec::new());
    };
    let mut notices = Vec::new();
    let shadow_branch = current_branch(&shadow)?;
    if shadow_branch.as_deref() != Some(branch.as_str()) {
        let main_commit = git_output(root, &["rev-parse", "HEAD"])?;
        notices.push(switch_to(
            &shadow,
            &branch,
            shadow_branch.as_deref(),
            &main_commit,
        )?);
    }
    notices.extend(auto_integrate(root, &shadow, &branch)?);
    restore_task_directories(&shadow)?;
    Ok(notices)
}

/// Git does not version empty directories, so an area without tasks loses its
/// `tasks` directory in a clone or checkout. Recreates it for every area.
fn restore_task_directories(shadow: &Path) -> Result<(), ZdevError> {
    let entries =
        fs::read_dir(shadow).map_err(|error| ZdevError::io("Cannot list zdev areas", error))?;
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        if name.to_string_lossy().starts_with(['.', '_'])
            || !path.join("area.toml").is_file()
            || fs::symlink_metadata(path.join("tasks")).is_ok()
        {
            continue;
        }
        fs::create_dir(path.join("tasks")).map_err(|error| {
            ZdevError::io(
                format!("Cannot restore {}", path.join("tasks").display()),
                error,
            )
        })?;
    }
    Ok(())
}

fn remote_branch(shadow: &Path, branch: &str) -> Option<String> {
    let refs = shadow_output(
        shadow,
        &["for-each-ref", "--format=%(refname:short)", "refs/remotes"],
    )
    .ok()?;
    let matches = refs
        .lines()
        .filter(|name| {
            name.split_once('/')
                .is_some_and(|(_, remote_branch)| remote_branch == branch)
        })
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [single] => Some((*single).to_owned()),
        _ => None,
    }
}

/// Shadow repository configuration key, per branch, holding the main commit
/// the main branch was at when zdev created the shadow branch.
fn fork_point_key(branch: &str) -> String {
    format!("branch.{branch}.zdevForkPoint")
}

/// The main commit recorded when zdev created shadow branch `branch`.
fn fork_point(shadow: &Path, branch: &str) -> Option<String> {
    shadow_output(shadow, &["config", "--get", &fork_point_key(branch)]).ok()
}

/// Switches the shadow repository to `branch`, creating it when needed. A
/// branch zdev creates records `main_commit` as its fork point, so later
/// merge detection can tell new main commits from the commit it started at.
fn switch_to(
    shadow: &Path,
    branch: &str,
    from: Option<&str>,
    main_commit: &str,
) -> Result<String, ZdevError> {
    let mut created = false;
    let (output, notice) = if local_branch_exists(shadow, branch) {
        (
            git(shadow, &["switch", "-q", branch])?,
            format!("Shadow record switched to branch {branch}"),
        )
    } else if let Some(tracking) = remote_branch(shadow, branch) {
        (
            git(
                shadow,
                &["switch", "-q", "-c", branch, "--track", &tracking],
            )?,
            format!("Shadow record switched to branch {branch} from {tracking}"),
        )
    } else {
        created = true;
        (
            git(shadow, &["switch", "-q", "-c", branch])?,
            match from {
                Some(from) => format!("Shadow record created branch {branch} from {from}"),
                None => format!("Shadow record created branch {branch}"),
            },
        )
    };
    if !output.status.success() {
        return Err(ZdevError::new(format!(
            "The shadow record is on {} but the repository is on {branch}, and the shadow record cannot follow: {}. Commit the record changes to the shadow record's current branch with `zdev shadow commit -m <subject>`, or resolve .zdev, then retry",
            from.unwrap_or("a detached HEAD"),
            stderr(&output)
        )));
    }
    if created {
        git_checked(
            shadow,
            &["config", &fork_point_key(branch), main_commit],
            "Cannot record the shadow branch fork point",
        )?;
    }
    Ok(notice)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Pending {
    /// Its main branch is contained in the current branch.
    Merged,
    /// Its main branch is gone and its records are not merged.
    Orphaned,
}

impl Pending {
    fn as_str(self) -> &'static str {
        match self {
            Self::Merged => "awaiting-integration",
            Self::Orphaned => "orphaned",
        }
    }
}

/// Shadow branches with records missing from `current` that need attention.
///
/// A branch is merged when its main branch is contained in the main HEAD and
/// has moved on since the shadow branch began: either a shadow commit names a
/// main commit, or the main branch has left the fork point zdev recorded. A
/// main branch that is still at its starting commit is contained in trunk
/// without being merged, so its records stay put. For a deleted main branch,
/// the newest named main commit decides.
fn pending_branches(
    root: &Path,
    shadow: &Path,
    current: &str,
) -> Result<Vec<(String, Pending)>, ZdevError> {
    if !local_branch_exists(shadow, current) {
        return Ok(Vec::new());
    }
    let branches = shadow_output(
        shadow,
        &["for-each-ref", "--format=%(refname:short)", "refs/heads"],
    )?;
    let mut pending = Vec::new();
    for branch in branches.lines().filter(|branch| *branch != current) {
        let range = format!("refs/heads/{current}..refs/heads/{branch}");
        let ahead = shadow_output(shadow, &["rev-list", "--count", &range])
            .ok()
            .and_then(|count| count.parse::<u64>().ok())
            .unwrap_or(0);
        if ahead == 0 {
            continue;
        }
        let sources = shadow_output(
            shadow,
            &[
                "log",
                &format!("--format=%(trailers:key={SOURCE_COMMIT_TRAILER},valueonly)"),
                &range,
            ],
        )?;
        let latest = sources.lines().map(str::trim).find(|line| !line.is_empty());
        let main_tip = branch_tip(root, branch)?;
        let merged = match &main_tip {
            Some(tip) => {
                let moved_on =
                    latest.is_some() || fork_point(shadow, branch).is_some_and(|fork| fork != *tip);
                moved_on && commit_is_ancestor(root, tip, "HEAD") == Some(true)
            }
            None => {
                latest.is_some_and(|source| commit_is_ancestor(root, source, "HEAD") == Some(true))
            }
        };
        if merged {
            pending.push((branch.to_owned(), Pending::Merged));
        } else if main_tip.is_none() {
            pending.push((branch.to_owned(), Pending::Orphaned));
        }
    }
    Ok(pending)
}

fn auto_integrate(root: &Path, shadow: &Path, current: &str) -> Result<Vec<String>, ZdevError> {
    let mut notices = Vec::new();
    for (branch, state) in pending_branches(root, shadow, current)? {
        if state != Pending::Merged {
            continue;
        }
        let output = git(
            shadow,
            &["merge", "--ff-only", "-q", &format!("refs/heads/{branch}")],
        )?;
        if output.status.success() {
            notices.push(format!(
                "Integrated shadow branch {branch} into {current} because {branch} is merged"
            ));
            if let Some(push) = push_after_commit(root, shadow, current).text() {
                notices.push(push);
            }
        }
    }
    Ok(notices)
}

#[derive(Clone, Debug)]
pub(super) enum PushOutcome {
    Disabled,
    Pushed(String),
    NoRemote,
    Failed(String),
}

impl PushOutcome {
    fn value(&self) -> Value {
        match self {
            Self::Disabled => json!({"status": "disabled"}),
            Self::Pushed(target) => json!({"status": "pushed", "target": target}),
            Self::NoRemote => json!({"status": "no-remote"}),
            Self::Failed(error) => json!({"status": "failed", "error": error}),
        }
    }

    fn text(&self) -> Option<String> {
        match self {
            Self::Disabled => None,
            Self::Pushed(target) => Some(format!("Pushed the shadow record to {target}.")),
            Self::NoRemote => Some(
                "shadow.push is after-commit, but the shadow record has no remote to push to."
                    .to_owned(),
            ),
            Self::Failed(error) => Some(format!(
                "Warning: pushing the shadow record failed; the commit stands. Retry with `zdev shadow push`. {error}"
            )),
        }
    }
}

fn push_after_commit(root: &Path, shadow: &Path, branch: &str) -> PushOutcome {
    match config::shadow_push(root) {
        Ok(ShadowPush::Never) => PushOutcome::Disabled,
        Ok(ShadowPush::AfterCommit) => push_branch(shadow, branch),
        Err(error) => PushOutcome::Failed(error.to_string()),
    }
}

fn push_branch(shadow: &Path, branch: &str) -> PushOutcome {
    let upstream = shadow_output(
        shadow,
        &[
            "rev-parse",
            "--abbrev-ref",
            "--symbolic-full-name",
            &format!("{branch}@{{upstream}}"),
        ],
    )
    .ok();
    let output = if let Some(upstream) = &upstream {
        let Some((remote, remote_branch)) = upstream.split_once('/') else {
            return PushOutcome::Failed(format!("Cannot parse upstream {upstream}"));
        };
        git(
            shadow,
            &[
                "push",
                "-q",
                remote,
                &format!("refs/heads/{branch}:refs/heads/{remote_branch}"),
            ],
        )
        .map(|output| (output, upstream.clone()))
    } else {
        let remotes = shadow_output(shadow, &["remote"]).unwrap_or_default();
        let remote = if remotes.lines().any(|remote| remote == "origin") {
            "origin".to_owned()
        } else {
            match remotes.lines().collect::<Vec<_>>().as_slice() {
                [single] => (*single).to_owned(),
                [] => return PushOutcome::NoRemote,
                _ => {
                    return PushOutcome::Failed(
                        "The shadow repository has several remotes and no origin; set an upstream for the branch"
                            .to_owned(),
                    );
                }
            }
        };
        git(shadow, &["push", "-q", "-u", &remote, branch])
            .map(|output| (output, format!("{remote}/{branch}")))
    };
    match output {
        Ok((output, target)) if output.status.success() => PushOutcome::Pushed(target),
        Ok((output, _)) => PushOutcome::Failed(stderr(&output)),
        Err(error) => PushOutcome::Failed(error.to_string()),
    }
}

pub(super) struct ShadowCommit {
    pub commit: String,
    pub branch: String,
    pub push: PushOutcome,
    /// A problem that left the commit incomplete, such as missing source
    /// trailers. The command reports it and exits with status 1.
    pub warning: Option<String>,
}

impl ShadowCommit {
    pub(super) fn value(&self) -> Value {
        let mut value = json!({
            "status": "committed",
            "commit": self.commit,
            "branch": self.branch,
            "push": self.push.value(),
        });
        if let Some(warning) = &self.warning {
            value["warning"] = json!(warning);
        }
        value
    }

    /// The exit status for a command whose result includes this commit.
    pub(super) fn exit_code(&self) -> u8 {
        u8::from(self.warning.is_some())
    }

    pub(super) fn text(&self) -> String {
        let mut text = format!(
            "Committed the shadow record {} on {}",
            short(&self.commit),
            self.branch
        );
        if let Some(push) = self.push.text() {
            text.push_str(". ");
            text.push_str(&push);
        }
        if let Some(warning) = &self.warning {
            text.push_str(&format!("\nWarning: {warning}"));
        }
        text
    }
}

fn short(commit: &str) -> &str {
    commit.get(..12).unwrap_or(commit)
}

fn commit_message_arguments<'a>(
    subject: &'a str,
    body: &'a [String],
    trailers: &'a str,
) -> Vec<&'a str> {
    let mut arguments = vec!["commit", "-q", "-m", subject];
    for paragraph in body {
        arguments.push("-m");
        arguments.push(paragraph);
    }
    arguments.push("-m");
    arguments.push(trailers);
    arguments
}

fn trailers(change_id: &str, source: Option<(&str, &str)>) -> String {
    let mut trailers = format!("{CHANGE_ID_TRAILER}: {change_id}");
    if let Some((commit, branch)) = source {
        trailers.push_str(&format!(
            "\n{SOURCE_COMMIT_TRAILER}: {commit}\n{SOURCE_BRANCH_TRAILER}: {branch}"
        ));
    }
    trailers
}

fn index_has_staged_changes(shadow: &Path) -> Result<bool, ZdevError> {
    let output = git(shadow, &["diff", "--cached", "--quiet"])?;
    match output.status.code() {
        Some(0) => Ok(false),
        Some(1) => Ok(true),
        _ => Err(ZdevError::new(format!(
            "Cannot inspect the shadow index: {}",
            stderr(&output)
        ))),
    }
}

/// Commits the shadow record: exactly the staged shadow changes when the
/// shadow index has any, otherwise every record change. `source` names the
/// main commit and branch the record change belongs to. Returns `None` when
/// the shadow record has no changes.
pub(super) fn commit_all(
    root: &Path,
    subject: &str,
    body: &[String],
    change_id: &str,
    source: Option<(&str, &str)>,
) -> Result<Option<ShadowCommit>, ZdevError> {
    let shadow = repository(root)?;
    if let Some(operation) = git_operation(&shadow)? {
        return Err(ZdevError::new(format!(
            "Cannot commit the shadow record while a {operation} is in progress in .zdev. Finish or abort it, then retry"
        )));
    }
    if !has_changes(&shadow)? {
        return Ok(None);
    }
    let branch = current_branch(&shadow)?.ok_or_else(|| {
        ZdevError::new(
            "Cannot commit the shadow record on a detached HEAD in .zdev. Check out a branch there, then retry",
        )
    })?;
    let staged = index_has_staged_changes(&shadow)?;
    if !staged {
        git_checked(&shadow, &["add", "-A"], "Cannot stage the shadow record")?;
    }
    let trailers = trailers(change_id, source);
    let output = git(&shadow, &commit_message_arguments(subject, body, &trailers))?;
    if !output.status.success() {
        if !staged {
            let _ = git(&shadow, &["reset", "-q"]);
        }
        return Err(ZdevError::new(format!(
            "Cannot commit the shadow record: {}",
            stderr(&output)
        )));
    }
    let commit = head(&shadow)?;
    let push = push_after_commit(root, &shadow, &branch);
    Ok(Some(ShadowCommit {
        commit,
        branch,
        push,
        warning: None,
    }))
}

/// The outcome of committing an explicit set of record paths.
pub(super) struct RecordCommit {
    /// The main commit when one was made, otherwise the shadow commit.
    pub revision: String,
    pub change_id: String,
    pub shadow: Option<ShadowCommit>,
}

/// Where the record lives for path-level Git operations: the main repository
/// with `.zdev/`-prefixed paths, or the shadow repository with paths relative
/// to `.zdev`.
pub(super) struct RecordRepository {
    pub directory: PathBuf,
    pub prefix: &'static str,
    pub shadow: bool,
}

impl RecordRepository {
    /// Runs Git in this repository and returns its trimmed output.
    pub(super) fn output(&self, arguments: &[&str]) -> Result<String, ZdevError> {
        let repo = if self.shadow {
            Repo::Shadow
        } else {
            Repo::Main
        };
        repo.output(&self.directory, arguments)
    }
}

/// The repository that versions the record, or `None` for a personal record.
pub(super) fn record_repository(
    root: &Path,
    record: RecordPolicy,
) -> Result<Option<RecordRepository>, ZdevError> {
    Ok(match record {
        RecordPolicy::Personal => None,
        RecordPolicy::Project | RecordPolicy::PullRequest => Some(RecordRepository {
            directory: root.to_path_buf(),
            prefix: ".zdev/",
            shadow: false,
        }),
        RecordPolicy::Shadow => Some(RecordRepository {
            directory: repository(root)?,
            prefix: "",
            shadow: true,
        }),
    })
}

fn relative_to(directory: &Path, paths: &[&PathBuf]) -> Result<Vec<String>, ZdevError> {
    paths
        .iter()
        .map(|path| {
            path.strip_prefix(directory)
                .map(|path| path.to_string_lossy().replace('\\', "/"))
                .map_err(|_| {
                    ZdevError::new(format!("Path is outside repository: {}", path.display()))
                })
        })
        .collect()
}

fn commit_only(
    repo: Repo,
    directory: &Path,
    subject: &str,
    trailers: &str,
    relative: &[String],
    what: &str,
) -> Result<(), ZdevError> {
    let mut add = vec!["add", "-f", "--"];
    add.extend(relative.iter().map(String::as_str));
    let output = repo.git(directory, &add)?;
    if !output.status.success() {
        return Err(ZdevError::new(format!(
            "Could not stage the new {what}. Resolve the Git error, then retry: {}",
            stderr(&output)
        )));
    }
    let mut commit = vec![
        "commit", "-q", "--only", "-m", subject, "-m", trailers, "--",
    ];
    commit.extend(relative.iter().map(String::as_str));
    let output = repo.git(directory, &commit)?;
    if !output.status.success() {
        return Err(ZdevError::new(format!(
            "Could not commit the new {what}. Resolve the Git error, then retry: {}",
            stderr(&output)
        )));
    }
    Ok(())
}

/// Unstages `relative` in the shadow repository, back to its HEAD.
fn unstage(shadow: &Path, relative: &[String]) -> Result<(), String> {
    if relative.is_empty() {
        return Ok(());
    }
    let mut reset = vec!["reset", "-q", "HEAD", "--"];
    reset.extend(relative.iter().map(String::as_str));
    match git(shadow, &reset) {
        Ok(output) if output.status.success() => Ok(()),
        Ok(output) => Err(stderr(&output)),
        Err(error) => Err(error.to_string()),
    }
}

/// Removes the shadow commit just made for `relative` and unstages those
/// paths, leaving every other staged shadow change in place.
fn undo_record_commit(shadow: &Path, relative: &[String]) -> Result<(), String> {
    match git(shadow, &["reset", "-q", "--soft", "HEAD~1"]) {
        Ok(output) if output.status.success() => unstage(shadow, relative),
        Ok(output) => Err(stderr(&output)),
        Err(error) => Err(error.to_string()),
    }
}

/// Error detail key that marks a record commit whose shadow commit could not
/// be undone after the main commit failed.
const SHADOW_ROLLBACK_DETAIL: &str = "shadow_rollback_failure";

/// The rollback failure recorded by [`commit_record_paths`], if any, so a
/// caller that restores files does not claim a complete rollback.
pub(super) fn shadow_rollback_failure(error: &ZdevError) -> Option<&str> {
    error
        .details
        .as_ref()?
        .get(SHADOW_ROLLBACK_DETAIL)?
        .as_str()
}

/// Commits exactly `paths`, which are files under `root`. For a shadow record,
/// paths inside `.zdev` go to the shadow repository and any other paths to the
/// main repository, under one change ID; the shadow commit then names the main
/// commit. A failed shadow-side commit leaves both indexes as they were.
pub(super) fn commit_record_paths(
    root: &Path,
    subject: &str,
    paths: &[PathBuf],
) -> Result<RecordCommit, ZdevError> {
    let config = project::read_config(root)?;
    let change_id = generate_change_id()?;
    let plain = trailers(&change_id, None);
    if config.project.record != RecordPolicy::Shadow {
        let all = paths.iter().collect::<Vec<_>>();
        let relative = relative_to(root, &all)?;
        commit_only(Repo::Main, root, subject, &plain, &relative, "task files")?;
        return Ok(RecordCommit {
            revision: git_output(root, &["rev-parse", "HEAD"])?,
            change_id,
            shadow: None,
        });
    }
    let shadow = repository(root)?;
    if let Some(operation) = git_operation(&shadow)? {
        return Err(ZdevError::new(format!(
            "Cannot commit the shadow record while a {operation} is in progress in .zdev. Finish or abort it, then retry"
        )));
    }
    let branch = current_branch(&shadow)?.ok_or_else(|| {
        ZdevError::new(
            "Cannot commit the shadow record on a detached HEAD in .zdev. Check out a branch there, then retry",
        )
    })?;
    let (records, main): (Vec<&PathBuf>, Vec<&PathBuf>) =
        paths.iter().partition(|path| path.starts_with(&shadow));
    let record_relative = relative_to(&shadow, &records)?;
    let main_relative = relative_to(root, &main)?;
    if !record_relative.is_empty()
        && let Err(error) = commit_only(
            Repo::Shadow,
            &shadow,
            subject,
            &plain,
            &record_relative,
            "task files",
        )
    {
        return Err(match unstage(&shadow, &record_relative) {
            Ok(()) => error,
            Err(rollback) => ZdevError::new(format!(
                "{error}; cannot unstage the task files in .zdev during rollback: {rollback}"
            )),
        });
    }
    let shadow_commit = (!record_relative.is_empty())
        .then(|| head(&shadow))
        .transpose()?;
    let mut revision = shadow_commit.clone();
    let mut warning = None;
    if !main_relative.is_empty() {
        if let Err(error) = commit_only(Repo::Main, root, subject, &plain, &main_relative, "files")
        {
            return Err(match &shadow_commit {
                None => error,
                Some(commit) => match undo_record_commit(&shadow, &record_relative) {
                    Ok(()) => error,
                    Err(rollback) => {
                        let failure = format!(
                            "the shadow commit {} remains on {branch} in .zdev ({rollback}); remove it with `git -C .zdev reset --soft HEAD~1`",
                            short(commit)
                        );
                        ZdevError::with_details(
                            format!("{error}; {failure}"),
                            json!({ SHADOW_ROLLBACK_DETAIL: failure }),
                        )
                    }
                },
            });
        }
        let main_commit = git_output(root, &["rev-parse", "HEAD"])?;
        if shadow_commit.is_some() {
            warning = add_source_trailers(&shadow, root, subject, &change_id, &main_commit)?;
        }
        revision = Some(main_commit);
    }
    let shadow = match shadow_commit {
        Some(_) => Some(ShadowCommit {
            commit: head(&shadow)?,
            branch: branch.clone(),
            push: push_after_commit(root, &shadow, &branch),
            warning,
        }),
        None => None,
    };
    Ok(RecordCommit {
        revision: revision.ok_or_else(|| ZdevError::new("No paths to commit"))?,
        change_id,
        shadow,
    })
}

/// Amends the shadow commit just made so it names `main_commit`. Both commits
/// stand either way; a failed amendment is returned as a warning that tells
/// the user how to add the trailers by hand.
fn add_source_trailers(
    shadow: &Path,
    root: &Path,
    subject: &str,
    change_id: &str,
    main_commit: &str,
) -> Result<Option<String>, ZdevError> {
    let main_branch = current_branch(root)?.unwrap_or_else(|| "HEAD".to_owned());
    let sourced = trailers(change_id, Some((main_commit, &main_branch)));
    let output = git(
        shadow,
        &[
            "commit", "-q", "--amend", "--only", "-m", subject, "-m", &sourced,
        ],
    )?;
    Ok((!output.status.success()).then(|| {
        format!(
            "the shadow commit does not name main commit {} because amending it failed: {}. Add the trailers with `git -C .zdev commit --amend --trailer \"{SOURCE_COMMIT_TRAILER}: {main_commit}\" --trailer \"{SOURCE_BRANCH_TRAILER}: {main_branch}\"`",
            short(main_commit),
            stderr(&output)
        )
    }))
}

/// Unstages `paths` after a failed record commit. Shadow-side staging is
/// already undone by [`commit_record_paths`].
pub(super) fn rollback_record_paths(root: &Path, paths: &[PathBuf], error: ZdevError) -> ZdevError {
    let shadow = dir(root);
    let main = if is_shadow(root) {
        paths
            .iter()
            .filter(|path| !path.starts_with(&shadow))
            .collect::<Vec<_>>()
    } else {
        paths.iter().collect::<Vec<_>>()
    };
    if main.is_empty() {
        return error;
    }
    let relative = match relative_to(root, &main) {
        Ok(relative) => relative,
        Err(rollback) => {
            return ZdevError::new(format!(
                "{error}; cannot identify staged task paths during rollback: {rollback}"
            ));
        }
    };
    let mut reset = vec!["reset", "-q", "HEAD", "--"];
    reset.extend(relative.iter().map(String::as_str));
    match main_git(root, &reset) {
        Ok(output) if output.status.success() => error,
        Ok(output) => ZdevError::new(format!(
            "{error}; cannot unstage imported task paths during rollback: {}",
            stderr(&output)
        )),
        Err(rollback) => ZdevError::new(format!(
            "{error}; cannot run Git to unstage imported task paths during rollback: {rollback}"
        )),
    }
}

/// `zdev shadow commit`: commit all record changes without a main commit.
pub(super) fn commit_command(
    root: &Path,
    message: &str,
    body: &[String],
) -> Result<CommandOutput, ZdevError> {
    validate_nonempty_line(message, "Commit message")?;
    require_shadow(root)?;
    let _lock = ZdevStateLock::acquire(root)?;
    let change_id = generate_change_id()?;
    let committed = commit_all(root, message, body, &change_id, None)?
        .ok_or_else(|| ZdevError::new("The shadow record has no changes to commit"))?;
    Ok(CommandOutput::new(
        format!("{} ({change_id}): {message}", committed.text()),
        json!({
            "schema_version": SCHEMA_VERSION,
            "status": "committed",
            "repository": "shadow",
            "commit": committed.commit,
            "change_id": change_id,
            "shadow": committed.value(),
        }),
    ))
}

/// `zdev shadow push`: push the current shadow branch regardless of `shadow.push`.
pub(super) fn push_command(root: &Path) -> Result<CommandOutput, ZdevError> {
    require_shadow(root)?;
    let _lock = ZdevStateLock::acquire(root)?;
    let shadow = repository(root)?;
    let branch = current_branch(&shadow)?.ok_or_else(|| {
        ZdevError::new("Cannot push the shadow record from a detached HEAD in .zdev")
    })?;
    match push_branch(&shadow, &branch) {
        PushOutcome::Pushed(target) => Ok(CommandOutput::new(
            format!("Pushed shadow branch {branch} to {target}"),
            json!({"schema_version": SCHEMA_VERSION, "status": "pushed", "branch": branch, "target": target}),
        )),
        PushOutcome::NoRemote => Err(ZdevError::new(
            "The shadow repository has no remote. Add one with `git -C .zdev remote add origin <url>`, then retry",
        )),
        PushOutcome::Failed(error) => Err(ZdevError::new(format!(
            "Cannot push shadow branch {branch}: {error}"
        ))),
        PushOutcome::Disabled => unreachable!("explicit pushes ignore shadow.push"),
    }
}

/// `zdev shadow integrate <branch>`: bring another shadow branch's records
/// into the current one. Fast-forwards when possible. Otherwise the configured
/// trunk is never rewritten: on trunk the other branch is rebased onto it, and
/// elsewhere the current branch is rebased onto the other branch.
pub(super) fn integrate(root: &Path, branch: &str) -> Result<CommandOutput, ZdevError> {
    require_shadow(root)?;
    let _lock = ZdevStateLock::acquire(root)?;
    let config = project::read_config(root)?;
    align_locked(root)?;
    let shadow = repository(root)?;
    if let Some(operation) = git_operation(&shadow)? {
        return Err(ZdevError::new(format!(
            "Cannot integrate while a {operation} is in progress in .zdev. Finish or abort it, then retry"
        )));
    }
    let current = current_branch(&shadow)?.ok_or_else(|| {
        ZdevError::new("Cannot integrate into a detached HEAD in .zdev; check out a branch first")
    })?;
    if current == branch {
        return Err(ZdevError::new(format!(
            "Shadow branch {branch} is already checked out; name another branch to integrate"
        )));
    }
    if !local_branch_exists(&shadow, branch) {
        return Err(ZdevError::new(format!(
            "The shadow repository has no branch {branch}"
        )));
    }
    let other = format!("refs/heads/{branch}");
    if commit_is_ancestor(&shadow, &other, "HEAD") == Some(true) {
        return Ok(CommandOutput::new(
            format!("Shadow branch {current} already contains {branch}"),
            json!({"schema_version": SCHEMA_VERSION, "status": "unchanged", "branch": current, "integrated": branch}),
        ));
    }
    if has_changes(&shadow)? {
        return Err(ZdevError::new(
            "Cannot integrate with uncommitted shadow record changes. Commit them with `zdev shadow commit -m <subject>`, then retry",
        ));
    }
    let method = if commit_is_ancestor(&shadow, "HEAD", &other) == Some(true) {
        git_checked(
            &shadow,
            &["merge", "--ff-only", "-q", &other],
            "Cannot fast-forward the shadow branch",
        )?;
        "fast-forward"
    } else if config.project.trunk.as_deref() == Some(current.as_str()) {
        let output = git(&shadow, &["rebase", "-q", &current, branch])?;
        if !output.status.success() {
            return Err(rebase_conflict(&shadow, branch, &current, &output, true));
        }
        git_checked(
            &shadow,
            &["switch", "-q", &current],
            "Cannot return to the shadow branch",
        )?;
        git_checked(
            &shadow,
            &["merge", "--ff-only", "-q", &other],
            "Cannot fast-forward the shadow branch",
        )?;
        "rebase-other"
    } else {
        let output = git(&shadow, &["rebase", "-q", branch])?;
        if !output.status.success() {
            return Err(rebase_conflict(&shadow, &current, branch, &output, false));
        }
        "rebase-current"
    };
    let push = push_after_commit(root, &shadow, &current);
    let mut text = format!("Integrated shadow branch {branch} into {current} ({method})");
    if let Some(push) = push.text() {
        text.push_str(". ");
        text.push_str(&push);
    }
    Ok(CommandOutput::new(
        text,
        json!({
            "schema_version": SCHEMA_VERSION,
            "status": "integrated",
            "branch": current,
            "integrated": branch,
            "method": method,
            "commit": head(&shadow)?,
            "push": push.value(),
        }),
    ))
}

fn rebase_conflict(
    shadow: &Path,
    rebased: &str,
    onto: &str,
    output: &Output,
    rerun: bool,
) -> ZdevError {
    let details = stderr(output);
    if git_operation(shadow).ok().flatten() == Some("rebase") {
        let next = if rerun {
            format!(", then run `zdev shadow integrate {rebased}` again")
        } else {
            String::new()
        };
        ZdevError::new(format!(
            "Rebasing shadow branch {rebased} onto {onto} has conflicts. Resolve them in .zdev and run `git -C .zdev rebase --continue`{next}, or cancel with `git -C .zdev rebase --abort`. {details}"
        ))
    } else {
        ZdevError::new(format!(
            "Could not rebase shadow branch {rebased} onto {onto}: {details}"
        ))
    }
}

/// After `zdev area rebase` moved an area branch onto its base, moves the
/// area's shadow branch onto the base's shadow branch. Best effort: a dirty or
/// conflicting shadow record is left unchanged and the returned notice says
/// how to finish by hand.
pub(super) fn rebase_area_branch(root: &Path, branch: &str, base: &str) -> Option<String> {
    if !is_shadow(root) || !has_repository(root) {
        return None;
    }
    let shadow = dir(root);
    if !local_branch_exists(&shadow, base) {
        return None;
    }
    let manual = format!("`git -C .zdev rebase {base}` on shadow branch {branch}");
    if current_branch(&shadow).ok().flatten().as_deref() != Some(branch) {
        return Some(format!(
            "Warning: the shadow record is not on {branch}, so its branch was not rebased. Run {manual}"
        ));
    }
    if commit_is_ancestor(&shadow, &format!("refs/heads/{base}"), "HEAD") == Some(true) {
        return None;
    }
    if git_operation(&shadow).ok().flatten().is_some() || has_changes(&shadow).unwrap_or(true) {
        return Some(format!(
            "Warning: the shadow record has uncommitted changes or a Git operation in progress, so its branch was not rebased. Commit or finish them, then run {manual}"
        ));
    }
    match git(&shadow, &["rebase", "-q", base]) {
        Ok(output) if output.status.success() => {
            Some(format!("Rebased shadow branch {branch} onto {base}"))
        }
        Ok(output) => {
            let details = stderr(&output);
            let _ = git(&shadow, &["rebase", "--abort"]);
            Some(format!(
                "Warning: rebasing shadow branch {branch} onto {base} conflicted and was cancelled. Run {manual} and resolve the conflicts. {details}"
            ))
        }
        Err(error) => Some(format!(
            "Warning: cannot rebase shadow branch {branch}: {error}. Run {manual}"
        )),
    }
}

fn require_shadow(root: &Path) -> Result<Config, ZdevError> {
    let config = project::read_config(root)?;
    if config.project.record != RecordPolicy::Shadow {
        return Err(ZdevError::new(format!(
            "This command needs a shadow record; the record policy is {}",
            config.project.record.as_str()
        )));
    }
    Ok(config)
}

/// The shadow record's state for status and check output. `None` for other
/// record policies.
pub(super) struct Report {
    pub value: Value,
    pub lines: Vec<String>,
    pub problems: Vec<String>,
}

pub(super) fn report(root: &Path) -> Option<Report> {
    if !is_shadow(root) {
        return None;
    }
    let mut problems = Vec::new();
    let mut warnings = Vec::new();
    if tracked_in_main(root) {
        problems.push(
            "The main repository tracks .zdev files; remove them with `git rm -r --cached .zdev`"
                .to_owned(),
        );
    }
    let is_excluded = excluded(root).unwrap_or(false);
    if !is_excluded {
        warnings.push(
            "The main repository does not ignore .zdev; add /.zdev/ to .git/info/exclude"
                .to_owned(),
        );
    }
    let push = config::shadow_push(root)
        .map(|push| push.as_str())
        .unwrap_or("never");
    if !has_repository(root) {
        problems.push("The shadow record has no Git repository at .zdev/.git".to_owned());
        let lines = vec![format!(
            "Shadow record: missing repository at .zdev/.git{}",
            render_notes(&problems, &warnings)
        )];
        return Some(Report {
            value: json!({"repository": ".zdev", "present": false, "excluded": is_excluded, "push": push, "problems": problems, "warnings": warnings}),
            lines,
            problems,
        });
    }
    let shadow = dir(root);
    let main_branch = current_branch(root).ok().flatten();
    let branch = current_branch(&shadow).ok().flatten();
    let operation = git_operation(&shadow).ok().flatten();
    let changes = change_count(&shadow).unwrap_or(0);
    let aligned = main_branch.is_none() || main_branch == branch;
    if !aligned {
        warnings.push(format!(
            "The shadow record is on {} but the repository is on {}",
            branch.as_deref().unwrap_or("a detached HEAD"),
            main_branch.as_deref().unwrap_or("a detached HEAD")
        ));
    }
    if let Some(operation) = operation {
        warnings.push(format!(
            "The shadow record has a {operation} in progress in .zdev"
        ));
    }
    let upstream = branch.as_ref().and_then(|branch| {
        let upstream = shadow_output(
            &shadow,
            &[
                "rev-parse",
                "--abbrev-ref",
                "--symbolic-full-name",
                &format!("{branch}@{{upstream}}"),
            ],
        )
        .ok()?;
        let counts = shadow_output(
            &shadow,
            &[
                "rev-list",
                "--left-right",
                "--count",
                &format!("{branch}...{branch}@{{upstream}}"),
            ],
        )
        .ok()?;
        let (ahead, behind) = counts.split_once(char::is_whitespace)?;
        Some((
            upstream,
            ahead.trim().parse::<u64>().ok()?,
            behind.trim().parse::<u64>().ok()?,
        ))
    });
    let pending = branch
        .as_deref()
        .and_then(|branch| pending_branches(root, &shadow, branch).ok())
        .unwrap_or_default();
    let awaiting = pending
        .iter()
        .filter(|(_, state)| *state == Pending::Merged)
        .map(|(branch, _)| branch.clone())
        .collect::<Vec<_>>();
    let orphaned = pending
        .iter()
        .filter(|(_, state)| *state == Pending::Orphaned)
        .map(|(branch, _)| branch.clone())
        .collect::<Vec<_>>();
    if !awaiting.is_empty() {
        warnings.push(format!(
            "Merged shadow branches await integration: {}; run `zdev shadow integrate <branch>`",
            awaiting.join(", ")
        ));
    }
    let mut line = format!(
        "Shadow record: branch {}",
        branch.as_deref().unwrap_or("detached")
    );
    line.push_str(if aligned {
        " (aligned)"
    } else {
        " (not aligned)"
    });
    line.push_str(&format!("; {changes} uncommitted changes"));
    match &upstream {
        Some((upstream, ahead, behind)) => {
            line.push_str(&format!("; {ahead} ahead, {behind} behind {upstream}"))
        }
        None => line.push_str("; no upstream"),
    }
    line.push_str(&format!("; push: {push}"));
    if !orphaned.is_empty() {
        line.push_str(&format!("; orphaned branches: {}", orphaned.join(", ")));
    }
    line.push_str(&render_notes(&problems, &warnings));
    let value = json!({
        "repository": ".zdev",
        "present": true,
        "branch": branch,
        "main_branch": main_branch,
        "aligned": aligned,
        "operation": operation,
        "uncommitted_changes": changes,
        "upstream": upstream.as_ref().map(|(upstream, ahead, behind)| json!({"name": upstream, "ahead": ahead, "behind": behind})),
        "push": push,
        "excluded": is_excluded,
        "pending": pending.iter().map(|(branch, state)| json!({"branch": branch, "state": state.as_str()})).collect::<Vec<_>>(),
        "problems": problems,
        "warnings": warnings,
    });
    Some(Report {
        value,
        lines: vec![line],
        problems,
    })
}

fn render_notes(problems: &[String], warnings: &[String]) -> String {
    problems
        .iter()
        .map(|problem| format!("\nProblem: {problem}"))
        .chain(
            warnings
                .iter()
                .map(|warning| format!("\nWarning: {warning}")),
        )
        .collect()
}

/// `zdev shadow status`.
pub(super) fn status_command(root: &Path) -> Result<CommandOutput, ZdevError> {
    require_shadow(root)?;
    let report = report(root).expect("shadow policy was checked");
    let mut value = report.value;
    value["schema_version"] = json!(SCHEMA_VERSION);
    Ok(CommandOutput::new(report.lines.join("\n"), value))
}

/// Shadow commits carrying `change_id`, for `zdev change lookup`.
pub(super) fn lookup_change(root: &Path, change_id: &str) -> Vec<Value> {
    if !is_shadow(root) || !has_repository(root) {
        return Vec::new();
    }
    let format = format!("--format=%H%x00%s%x00%(trailers:key={CHANGE_ID_TRAILER},valueonly)");
    let Ok(log) = shadow_output(&dir(root), &["log", "--all", "-z", &format]) else {
        return Vec::new();
    };
    let mut matches = Vec::new();
    let mut fields = log.split('\0').map(|field| field.trim_matches('\n'));
    while let (Some(commit), Some(subject), Some(trailers)) =
        (fields.next(), fields.next(), fields.next())
    {
        if commit.is_empty() {
            break;
        }
        if trailers.lines().any(|line| line.trim() == change_id) {
            matches.push(json!({"commit": commit, "subject": subject, "repository": "shadow"}));
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;

    use super::{commit_record_paths, initialize};
    use crate::project::{RecordPolicy, initial_config};

    fn run(directory: &Path, arguments: &[&str]) -> String {
        let output = std::process::Command::new("git")
            .args(arguments)
            .current_dir(directory)
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .env_remove("GIT_INDEX_FILE")
            .output()
            .expect("run git");
        assert!(
            output.status.success(),
            "git {arguments:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8_lossy(&output.stdout).trim().to_owned()
    }

    fn hook(path: &Path, script: &str) {
        fs::create_dir_all(path.parent().expect("hooks directory")).expect("hooks directory");
        fs::write(path, format!("#!/bin/sh\n{script}\n")).expect("hook");
        fs::set_permissions(path, fs::Permissions::from_mode(0o755)).expect("hook mode");
    }

    /// A main repository with a shadow record and one record file and one
    /// main file ready to commit together, as derived task application does.
    fn shadow_record() -> tempfile::TempDir {
        let directory = tempfile::tempdir().expect("temporary repository");
        let root = directory.path();
        run(root, &["init", "-q", "--initial-branch", "main"]);
        run(root, &["config", "user.name", "Zdev Test"]);
        run(root, &["config", "user.email", "zdev@example.invalid"]);
        fs::write(root.join("seed.txt"), "seed\n").expect("seed");
        run(root, &["add", "seed.txt"]);
        run(root, &["commit", "-q", "-m", "seed"]);
        initialize(root, None, || initial_config(root, RecordPolicy::Shadow))
            .expect("initialize shadow record");
        fs::write(root.join(".zdev/work.md"), "record\n").expect("record file");
        fs::write(root.join("main.txt"), "main\n").expect("main file");
        directory
    }

    #[test]
    fn failed_main_commit_keeps_unrelated_shadow_staging() {
        let directory = shadow_record();
        let root = directory.path();
        let shadow = root.join(".zdev");
        fs::write(shadow.join("_staged.md"), "staged\n").expect("staged record");
        run(&shadow, &["add", "_staged.md"]);
        let shadow_head = run(&shadow, &["rev-parse", "HEAD"]);
        hook(&root.join(".git/hooks/pre-commit"), "exit 1");

        let error = commit_record_paths(
            root,
            "Add tasks",
            &[shadow.join("work.md"), root.join("main.txt")],
        )
        .err()
        .expect("the main commit is rejected");

        assert!(super::shadow_rollback_failure(&error).is_none());
        assert_eq!(run(&shadow, &["rev-parse", "HEAD"]), shadow_head);
        assert_eq!(
            run(&shadow, &["diff", "--cached", "--name-only"]),
            "_staged.md"
        );
        assert_eq!(
            run(&shadow, &["status", "--porcelain", "--", "work.md"]),
            "?? work.md"
        );
    }

    #[test]
    fn rejected_source_trailers_are_reported_as_a_partial_commit() {
        let directory = shadow_record();
        let root = directory.path();
        let shadow = root.join(".zdev");
        hook(
            &shadow.join(".git/hooks/commit-msg"),
            "if grep -q Zdev-Source-Commit \"$1\"; then exit 1; fi",
        );

        let committed = commit_record_paths(
            root,
            "Add tasks",
            &[shadow.join("work.md"), root.join("main.txt")],
        )
        .expect("both commits are made");

        let record = committed.shadow.expect("shadow commit");
        assert_eq!(record.exit_code(), 1);
        assert!(
            record
                .warning
                .as_deref()
                .is_some_and(|warning| warning.contains("--trailer"))
        );
        assert_eq!(record.value()["status"], "committed");
        assert_eq!(run(root, &["rev-parse", "HEAD"]), committed.revision);
        let message = run(&shadow, &["log", "-1", "--format=%B"]);
        assert!(message.contains(&committed.change_id));
        assert!(!message.contains("Zdev-Source-Commit"));
    }
}

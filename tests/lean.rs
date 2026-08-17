use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Output, Stdio};

use serde_json::{Value, json};
use tempfile::TempDir;

fn run_zdev(root: &Path, arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_zdev"))
        .arg("--root")
        .arg(root)
        .args(arguments)
        .output()
        .expect("run zdev")
}

fn json_output(root: &Path, arguments: &[&str]) -> Value {
    let mut arguments = arguments.to_vec();
    arguments.extend(["--format", "json"]);
    let output = run_zdev(root, &arguments);
    assert!(
        output.status.success(),
        "zdev failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("JSON output")
}

fn json_output_with_stdin(root: &Path, arguments: &[&str], input: &[u8]) -> Value {
    let output = json_output_with_stdin_status(root, arguments, input);
    assert!(
        output.status.success(),
        "zdev failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("JSON output")
}

fn json_output_with_stdin_status(root: &Path, arguments: &[&str], input: &[u8]) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_zdev"))
        .arg("--root")
        .arg(root)
        .args(arguments)
        .args(["--format", "json"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("run zdev");
    child
        .stdin
        .take()
        .expect("zdev stdin")
        .write_all(input)
        .expect("write zdev stdin");
    child.wait_with_output().expect("wait for zdev")
}

fn json_output_with_env(root: &Path, arguments: &[&str], environment: &[(&str, &Path)]) -> Value {
    let mut arguments = arguments.to_vec();
    arguments.extend(["--format", "json"]);
    let output = run_zdev_with_env(root, &arguments, environment);
    assert!(
        output.status.success(),
        "zdev failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("JSON output")
}

fn run_zdev_with_env(root: &Path, arguments: &[&str], environment: &[(&str, &Path)]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_zdev"));
    command.arg("--root").arg(root).args(arguments);
    for (name, value) in environment {
        command.env(name, value);
    }
    command.output().expect("run zdev")
}

fn json_output_with_exit_code(root: &Path, arguments: &[&str], expected: i32) -> Value {
    let mut arguments = arguments.to_vec();
    arguments.extend(["--format", "json"]);
    let output = run_zdev(root, &arguments);
    assert_eq!(
        output.status.code(),
        Some(expected),
        "unexpected exit: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("JSON output")
}

fn git(root: &Path, arguments: &[&str]) -> String {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(root)
        .output()
        .expect("run git");
    assert!(
        output.status.success(),
        "git failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().to_owned()
}

fn repository() -> TempDir {
    let directory = tempfile::tempdir().expect("temporary repository");
    git(directory.path(), &["init", "-q"]);
    git(directory.path(), &["config", "user.name", "Zdev Test"]);
    git(
        directory.path(),
        &["config", "user.email", "zdev@example.invalid"],
    );
    directory
}

fn commit_file(root: &Path, name: &str, contents: &str, message: &str) {
    fs::write(root.join(name), contents).expect("write committed file");
    git(root, &["add", name]);
    git(root, &["commit", "-q", "-m", message]);
}

fn commit_all(root: &Path, message: &str) {
    git(root, &["add", "-A"]);
    git(root, &["commit", "-q", "-m", message]);
}

fn file_inventory(root: &Path) -> Vec<String> {
    let mut files = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(&directory).expect("bundle directory") {
            let entry = entry.expect("bundle entry");
            if entry.file_type().expect("bundle entry type").is_dir() {
                pending.push(entry.path());
            } else {
                files.push(
                    entry
                        .path()
                        .strip_prefix(root)
                        .expect("bundle-relative path")
                        .to_string_lossy()
                        .replace('\\', "/"),
                );
            }
        }
    }
    files.sort();
    files
}

fn create_area(root: &Path, area: &str, branch: &str) {
    json_output(
        root,
        &[
            "area",
            "create",
            area,
            "--title",
            area,
            "--objective",
            "Exercise managed area branches.",
            "--branch",
            branch,
        ],
    );
}

fn import_one_task(root: &Path, area: &str) {
    let bundle = root.join(format!("{area}-task.json"));
    fs::write(
        &bundle,
        serde_json::to_vec(&json!({
            "schema_version": 1,
            "area": area,
            "tasks": [{
                "key": "one",
                "title": "Complete one task",
                "outcome": "The guarded task completes.",
                "done_when": ["The task is complete."],
                "validation": ["Exercise the CLI."],
                "blocked_by": []
            }]
        }))
        .expect("task JSON"),
    )
    .expect("write task bundle");
    json_output(
        root,
        &[
            "tasks",
            "import",
            area,
            "--from",
            bundle.to_str().expect("bundle path"),
        ],
    );
    fs::remove_file(bundle).expect("remove imported bundle");
}

#[test]
fn managed_rebase_updates_an_independent_area_and_unlocks_task_work() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "shared.txt", "base\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "feature", "feature");
    import_one_task(root, "feature");
    commit_all(root, "configure feature area");
    git(root, &["switch", "-q", "-c", "feature"]);
    commit_file(root, "feature.txt", "feature\n", "feature work");
    git(root, &["switch", "-q", "main"]);
    commit_file(root, "trunk.txt", "trunk\n", "advance trunk");

    let wrong_branch = run_zdev(root, &["next", "feature"]);
    assert!(!wrong_branch.status.success());
    assert!(String::from_utf8_lossy(&wrong_branch.stderr).contains("Switch to feature and retry"));
    git(root, &["switch", "-q", "feature"]);
    let stale_next = run_zdev(root, &["next", "feature"]);
    assert!(!stale_next.status.success());
    assert!(String::from_utf8_lossy(&stale_next.stderr).contains("zdev area rebase feature"));
    let stale_done = run_zdev(
        root,
        &[
            "task",
            "done",
            "feature",
            "feature-001",
            "--summary",
            "Must remain open.",
            "--validation",
            "Guarded.",
        ],
    );
    assert!(!stale_done.status.success());
    assert!(String::from_utf8_lossy(&stale_done.stderr).contains("stale"));

    let rebased = json_output(root, &["area", "rebase", "feature"]);
    assert_eq!(rebased["status"], "rebased");
    assert_eq!(rebased["effective_base"], "main");
    assert_eq!(
        json_output(root, &["status", "feature"])["branch_status"]["fresh"],
        true
    );
    assert_eq!(
        json_output(root, &["next", "feature"])["task"]["id"],
        "feature-001"
    );
    commit_all(root, "record rebased base anchor");
    let no_op = json_output(root, &["area", "rebase", "feature"]);
    assert_eq!(no_op["status"], "unchanged");
}

#[test]
fn managed_rebase_uses_a_parent_area_as_the_effective_base() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "root-area", "root-area");
    create_area(root, "child-area", "child-area");
    commit_all(root, "configure dependent areas");
    git(root, &["switch", "-q", "-c", "root-area"]);
    commit_file(root, "root.txt", "one\n", "root work");
    git(root, &["switch", "-q", "-c", "child-area"]);
    json_output(root, &["area", "parent", "child-area", "root-area"]);
    fs::write(root.join("child.txt"), "child\n").expect("child work");
    commit_all(root, "configure child and add its work");
    git(root, &["switch", "-q", "root-area"]);
    commit_file(root, "root-two.txt", "two\n", "advance root");
    git(root, &["switch", "-q", "child-area"]);

    let before = json_output(root, &["status", "child-area"]);
    assert_eq!(before["branch_status"]["fresh"], false);
    assert_eq!(
        before["branch_status"]["effective_base"]["branch"],
        "root-area"
    );
    let rebased = json_output(root, &["area", "rebase", "child-area"]);
    assert_eq!(rebased["status"], "rebased");
    assert_eq!(rebased["effective_base"], "root-area");
    assert_eq!(
        json_output(root, &["status", "child-area"])["branch_status"]["fresh"],
        true
    );
}

#[test]
fn child_rebase_excludes_parent_commits_rewritten_during_parent_rebase() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "shared.txt", "base\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "parent-area", "parent-area");
    create_area(root, "child-area", "child-area");
    commit_all(root, "configure stacked areas");

    git(root, &["switch", "-q", "-c", "parent-area"]);
    commit_file(root, "shared.txt", "parent\n", "parent logical work");
    let old_parent_tip = git(root, &["rev-parse", "HEAD"]);
    git(root, &["switch", "-q", "-c", "child-area"]);
    json_output(root, &["area", "parent", "child-area", "parent-area"]);
    fs::write(root.join("child.txt"), "child only\n").expect("child work");
    commit_all(root, "child logical work");
    let child_anchor = json_output(root, &["status", "child-area"])["area"]["base_commit"]
        .as_str()
        .expect("child anchor")
        .to_owned();
    assert_eq!(child_anchor, old_parent_tip);

    git(root, &["switch", "-q", "main"]);
    commit_file(root, "shared.txt", "main\n", "advance trunk incompatibly");
    git(root, &["switch", "-q", "parent-area"]);
    let stopped = run_zdev(root, &["area", "rebase", "parent-area"]);
    assert!(!stopped.status.success());
    fs::write(root.join("shared.txt"), "resolved parent\n").expect("resolve parent");
    git(root, &["add", "shared.txt"]);
    assert_eq!(
        json_output(root, &["area", "rebase", "parent-area", "--continue"])["status"],
        "rebased"
    );
    commit_all(root, "record rewritten parent anchor");
    let rewritten_parent_tip = git(root, &["rev-parse", "HEAD"]);
    assert_ne!(rewritten_parent_tip, old_parent_tip);

    git(root, &["switch", "-q", "child-area"]);
    let stale = json_output(root, &["status", "child-area"]);
    assert_eq!(stale["branch_status"]["fresh"], false);
    assert_eq!(stale["branch_status"]["anchor_valid"], true);
    assert_eq!(stale["branch_status"]["finalized"], false);
    assert!(
        stale["branch_status"]["diagnostics"]
            .as_array()
            .expect("diagnostics")
            .contains(&json!("stale"))
    );
    assert!(
        !stale["branch_status"]["diagnostics"]
            .as_array()
            .expect("diagnostics")
            .contains(&json!("fresh"))
    );
    let child_rebased = json_output(root, &["area", "rebase", "child-area"]);
    assert_eq!(child_rebased["status"], "rebased");
    assert_eq!(child_rebased["effective_base"], "parent-area");
    assert_eq!(
        git(root, &["rev-list", "--count", "parent-area..child-area"]),
        "1"
    );
    assert_eq!(
        git(root, &["log", "-1", "--format=%s", "child-area"]),
        "child logical work"
    );
    assert_eq!(
        fs::read_to_string(root.join("child.txt")).expect("child result"),
        "child only\n"
    );
    assert_eq!(
        fs::read_to_string(root.join("shared.txt")).expect("parent result"),
        "resolved parent\n"
    );
}

#[test]
fn conflicting_managed_rebase_preserves_git_recovery_state_and_guidance() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "conflict.txt", "base\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "feature", "feature");
    commit_all(root, "configure feature area");
    git(root, &["switch", "-q", "-c", "feature"]);
    commit_file(root, "conflict.txt", "feature\n", "feature change");
    git(root, &["switch", "-q", "main"]);
    commit_file(root, "conflict.txt", "main\n", "trunk change");
    git(root, &["switch", "-q", "feature"]);
    let metadata_path = root.join(".zdev/feature/area.toml");
    let old_metadata = fs::read_to_string(&metadata_path).expect("old metadata");

    let conflicted = run_zdev(root, &["area", "rebase", "feature"]);
    assert!(!conflicted.status.success());
    let guidance = String::from_utf8_lossy(&conflicted.stderr);
    assert!(guidance.contains("git rebase --continue"));
    assert!(guidance.contains("git rebase --abort"));
    let rebase_path = git(root, &["rev-parse", "--git-path", "rebase-merge"]);
    assert!(root.join(rebase_path).exists());
    assert_eq!(
        fs::read_to_string(&metadata_path).expect("stopped metadata"),
        old_metadata
    );

    let existing = run_zdev(root, &["area", "rebase", "feature"]);
    assert!(!existing.status.success());
    assert!(String::from_utf8_lossy(&existing.stderr).contains("Finish or abort it, then retry"));
    assert!(!root.join(".git/zdev-rebase.toml").exists());
    fs::write(root.join("conflict.txt"), "resolved\n").expect("resolve conflict");
    git(root, &["add", "conflict.txt"]);
    git(root, &["-c", "core.editor=true", "rebase", "--continue"]);
    assert_eq!(
        fs::read_to_string(&metadata_path).expect("manually completed metadata"),
        old_metadata
    );
    let finalized = json_output(root, &["area", "rebase", "feature"]);
    assert_eq!(finalized["status"], "updated");
    assert_ne!(
        fs::read_to_string(&metadata_path).expect("final metadata"),
        old_metadata
    );
    assert_eq!(
        json_output(root, &["status", "feature"])["branch_status"]["fresh"],
        true
    );
}

#[test]
fn managed_rebase_rejects_dirty_missing_and_merge_based_histories() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "feature", "feature");
    commit_all(root, "configure feature area");
    git(root, &["switch", "-q", "-c", "feature"]);

    fs::write(root.join("dirty.txt"), "dirty\n").expect("dirty file");
    let dirty = run_zdev(root, &["area", "rebase", "feature"]);
    assert!(!dirty.status.success());
    assert!(String::from_utf8_lossy(&dirty.stderr).contains("Commit or stash them, then retry"));
    fs::remove_file(root.join("dirty.txt")).expect("clean worktree");

    json_output(root, &["config", "trunk", "missing-base"]);
    commit_all(root, "record missing base");
    let missing = run_zdev(root, &["area", "rebase", "feature"]);
    assert!(!missing.status.success());
    assert!(
        String::from_utf8_lossy(&missing.stderr)
            .contains("missing locally. Restore or create it, then retry")
    );
    json_output(root, &["config", "trunk", "main"]);
    commit_all(root, "restore trunk");

    git(root, &["switch", "-q", "-c", "side"]);
    commit_file(root, "side.txt", "side\n", "side work");
    git(root, &["switch", "-q", "feature"]);
    git(
        root,
        &["merge", "-q", "--no-ff", "side", "-m", "merge side"],
    );
    let merged = run_zdev(root, &["area", "rebase", "feature"]);
    assert!(!merged.status.success());
    assert!(String::from_utf8_lossy(&merged.stderr).contains("rebase-only history"));
    let status = json_output(root, &["status", "feature"]);
    assert!(
        status["branch_status"]["diagnostics"]
            .as_array()
            .expect("diagnostics")
            .contains(&json!("merge-history"))
    );
}

#[test]
fn initialization_and_area_creation_record_the_checked_out_branches() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");

    let initialized = json_output(root, &["init", "--record", "project"]);
    assert_eq!(initialized["status"], "created");
    assert_eq!(
        initialized["setup"]["status"],
        "check-existing-user-integrations"
    );
    assert_eq!(initialized["setup"]["check_for"], "each-requested-harness");
    assert_eq!(
        initialized["setup"]["check_command"],
        json!(["skill", "check", "<harness>", "--scope", "user"])
    );
    assert_eq!(
        initialized["setup"]["reuse_when"],
        json!({"status": "ok", "action": "reuse-user-integration"})
    );
    assert_eq!(
        initialized["setup"]["install_when"],
        json!(["missing", "conflict", "project-scope-requested"])
    );
    assert_eq!(
        initialized["setup"]["project_guidance_options"],
        json!(["auto", "agents", "zdev", "PATH"])
    );
    let config = fs::read_to_string(root.join(".zdev/config.toml")).expect("config");
    assert!(config.contains("trunk = \"main\""));

    git(root, &["switch", "-q", "-c", "feature"]);
    let created = json_output(
        root,
        &[
            "area",
            "create",
            "feature",
            "--title",
            "Feature",
            "--objective",
            "Build the feature.",
        ],
    );
    assert_eq!(created["branch"], "feature");
    let status = json_output(root, &["status", "feature"]);
    assert_eq!(status["trunk"], "main");
    assert_eq!(status["area"]["branch"], "feature");
    assert_eq!(status["branch_status"]["branch_matches"], true);
    assert_eq!(
        status["branch_status"]["effective_base"],
        json!({"kind": "trunk", "area": null, "branch": "main"})
    );
    assert_eq!(status["branch_status"]["fresh"], true);
}

#[test]
fn initialization_text_explains_what_changed_and_what_to_do_next() {
    let repository = repository();
    let output = run_zdev(repository.path(), &["init", "--record", "project"]);
    assert!(output.status.success());
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(text.contains("Created .zdev/config.toml. Recorded"));
    assert!(text.contains("as the project trunk"));
    assert!(text.contains("zdev skill check <codex|claude|opencode|pi|omp> --scope user"));
    assert!(text.contains("zdev area create <tag> --title <title> --objective <objective>"));
    assert!(!text.contains("zdev skill install <codex|claude|opencode|pi|omp>"));
    assert!(!text.contains("Harness setup:"));
}

#[test]
fn initialization_records_each_record_policy_and_explains_pull_request_cleanup() {
    for policy in ["personal", "project", "pull-request"] {
        let repo = repository();
        let root = repo.path();
        let initialized = json_output(root, &["init", "--record", policy]);
        assert_eq!(initialized["record"]["policy"], policy);
        let config = fs::read_to_string(root.join(".zdev/config.toml")).expect("config");
        assert!(config.contains(&format!("record = \"{policy}\"")));
        if policy == "pull-request" {
            assert_eq!(
                initialized["record"]["cleanup_required_before"],
                "squash-merge"
            );
            assert_eq!(
                initialized["record"]["cleanup_command"],
                json!(["cleanup", "squash"])
            );
            assert!(
                initialized["record"]["notice"]
                    .as_str()
                    .expect("notice")
                    .contains("tracked for pull-request review")
            );

            let text_repository = repository();
            let text = run_zdev(
                text_repository.path(),
                &["init", "--record", "pull-request"],
            );
            assert!(text.status.success());
            let text = String::from_utf8_lossy(&text.stdout);
            assert!(text.contains(".zdev is tracked for pull-request review"));
            assert!(
                text.contains("must be cleaned with `zdev cleanup squash` before squash merge")
            );
        }
    }
}

#[test]
fn initialization_requires_a_record_policy() {
    let repository = repository();
    let root = repository.path();
    let output = run_zdev(root, &["init"]);
    assert!(!output.status.success());
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("--record <POLICY>"));
    assert!(error.contains("required"));
    assert!(!root.join(".zdev").exists());
}

#[test]
fn cleanup_squash_deletes_only_tracked_zdev_files_in_one_plain_commit() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "pull-request"]);
    git(root, &["switch", "-q", "-c", "feature"]);
    fs::create_dir_all(root.join(".codex/skills/example")).expect("create harness directory");
    fs::write(
        root.join(".codex/skills/example/SKILL.md"),
        "project harness integration\n",
    )
    .expect("write harness integration");
    commit_all(root, "add pull-request development record");
    let before = git(root, &["rev-list", "--count", "HEAD"])
        .parse::<u64>()
        .expect("commit count");

    let cleaned = json_output(root, &["cleanup", "squash"]);
    assert_eq!(cleaned["status"], "committed");
    assert_eq!(cleaned["record"], "pull-request");
    assert_eq!(cleaned["cleanup"], "squash");
    assert_eq!(cleaned["branch"], "feature");
    assert_eq!(cleaned["message"], "chore: remove zdev development record");
    assert!(!root.join(".zdev").exists());
    assert_eq!(git(root, &["ls-tree", "-r", "HEAD", "--", ".zdev"]), "");
    assert!(root.join(".codex/skills/example/SKILL.md").is_file());
    assert_eq!(git(root, &["status", "--porcelain=v1"]), "");
    assert_eq!(
        git(root, &["rev-list", "--count", "HEAD"])
            .parse::<u64>()
            .expect("commit count"),
        before + 1
    );
    assert_eq!(
        git(root, &["show", "-s", "--format=%B", "HEAD"]),
        "chore: remove zdev development record"
    );
    assert!(!git(root, &["show", "-s", "--format=%B", "HEAD"]).contains("Zdev-Change-Id"));
    let changed = git(
        root,
        &["diff-tree", "--no-commit-id", "--name-only", "-r", "HEAD"],
    );
    assert!(!changed.is_empty());
    assert!(changed.lines().all(|path| path.starts_with(".zdev/")));
}

#[test]
fn cleanup_squash_requires_pull_request_policy_clean_branch_and_tracked_files() {
    let wrong_policy = repository();
    let root = wrong_policy.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    git(root, &["switch", "-q", "-c", "feature"]);
    commit_all(root, "add zdev record");
    let output = run_zdev(root, &["cleanup", "squash"]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("not pull-request"));
    assert!(root.join(".zdev/config.toml").is_file());

    let dirty = repository();
    let root = dirty.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "pull-request"]);
    git(root, &["switch", "-q", "-c", "feature"]);
    commit_all(root, "add zdev record");
    fs::write(root.join("seed.txt"), "dirty\n").expect("dirty worktree");
    let output = run_zdev(root, &["cleanup", "squash"]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("with local changes"));
    assert!(root.join(".zdev/config.toml").is_file());

    let untracked = repository();
    let root = untracked.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    fs::write(root.join(".git/info/exclude"), "/.zdev/\n").expect("exclude zdev");
    json_output(root, &["init", "--record", "pull-request"]);
    git(root, &["switch", "-q", "-c", "feature"]);
    let output = run_zdev(root, &["cleanup", "squash"]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("no tracked .zdev files"));
    assert!(root.join(".zdev/config.toml").is_file());
}

#[test]
fn cleanup_squash_refuses_configured_trunk() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "pull-request"]);
    commit_all(root, "add zdev record");

    let output = run_zdev(root, &["cleanup", "squash"]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("configured trunk main"));
    assert!(root.join(".zdev/config.toml").is_file());
}

#[test]
fn initialization_on_detached_head_explains_how_to_bind_trunk() {
    let repository = repository();
    let root = repository.path();
    commit_file(root, "seed.txt", "seed\n", "seed");
    git(root, &["checkout", "--detach", "-q", "HEAD"]);

    let output = run_zdev(root, &["init", "--record", "project"]);
    assert!(output.status.success());
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(text.contains("HEAD is detached, so no project trunk was recorded"));
    assert!(text.contains("zdev config trunk <branch>"));

    let initialized = json_output(root, &["status"]);
    assert_eq!(initialized["trunk"], Value::Null);
}

#[test]
fn explicit_branch_shorthand_is_canonicalized_before_storage_and_ownership_checks() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    git(root, &["switch", "-q", "-c", "feature"]);
    git(root, &["switch", "-q", "main"]);

    let created = json_output(
        root,
        &[
            "area",
            "create",
            "feature",
            "--title",
            "Feature",
            "--objective",
            "Own the canonical feature branch.",
            "--branch",
            "@{-1}",
        ],
    );
    assert_eq!(created["branch"], "feature");
    assert!(
        fs::read_to_string(root.join(".zdev/feature/area.toml"))
            .expect("area metadata")
            .contains("branch = \"feature\"")
    );

    let duplicate = run_zdev(
        root,
        &[
            "area",
            "create",
            "duplicate",
            "--title",
            "Duplicate",
            "--objective",
            "Must not claim the same branch.",
            "--branch",
            "feature",
        ],
    );
    assert!(!duplicate.status.success());
    assert!(String::from_utf8_lossy(&duplicate.stderr).contains("already owned"));
    assert!(!root.join(".zdev/duplicate").exists());
}

#[test]
fn status_reports_wrong_branch_and_a_stale_effective_base_without_failing() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    git(root, &["switch", "-q", "-c", "feature"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "feature",
            "--title",
            "Feature",
            "--objective",
            "Build the feature.",
        ],
    );
    git(root, &["switch", "-q", "main"]);
    commit_file(root, "trunk.txt", "advance\n", "advance trunk");

    let status = json_output(root, &["status", "feature"]);
    assert_eq!(status["branch_status"]["branch_matches"], false);
    assert_eq!(status["branch_status"]["fresh"], false);
    let diagnostics = status["branch_status"]["diagnostics"]
        .as_array()
        .expect("diagnostics");
    assert!(diagnostics.contains(&json!("wrong-branch")));
    assert!(diagnostics.contains(&json!("stale")));
}

#[test]
fn area_binding_and_parent_configuration_validate_the_area_graph() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);

    git(root, &["switch", "-q", "-c", "root-area"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "root-area",
            "--title",
            "Root area",
            "--objective",
            "Build the root.",
        ],
    );
    git(root, &["switch", "-q", "-c", "child-area"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "child-area",
            "--title",
            "Child area",
            "--objective",
            "Build on the root.",
        ],
    );
    json_output(
        root,
        &[
            "area",
            "create",
            "alternate",
            "--title",
            "Alternate",
            "--objective",
            "Provide another base.",
            "--branch",
            "alternate-area",
        ],
    );
    git(root, &["branch", "alternate-area"]);
    json_output(root, &["area", "parent", "child-area", "alternate"]);
    assert_eq!(
        json_output(root, &["status", "child-area"])["area"]["parent"],
        "alternate"
    );
    let established_anchor = json_output(root, &["status", "child-area"])["area"]["base_commit"]
        .as_str()
        .expect("established anchor")
        .to_owned();
    json_output(root, &["area", "parent", "child-area", "root-area"]);
    let status = json_output(root, &["status", "child-area"]);
    assert_eq!(status["area"]["parent"], "root-area");
    assert_eq!(
        status["branch_status"]["effective_base"],
        json!({"kind": "area", "area": "root-area", "branch": "root-area"})
    );
    assert_eq!(status["branch_status"]["fresh"], true);
    assert_eq!(status["area"]["base_commit"], established_anchor);

    let duplicate = run_zdev(root, &["area", "bind", "child-area", "root-area"]);
    assert!(!duplicate.status.success());
    assert!(String::from_utf8_lossy(&duplicate.stderr).contains("already owned"));
    let self_parent = run_zdev(root, &["area", "parent", "root-area", "root-area"]);
    assert!(!self_parent.status.success());
    assert!(String::from_utf8_lossy(&self_parent.stderr).contains("own parent"));
    let cycle = run_zdev(root, &["area", "parent", "root-area", "child-area"]);
    assert!(!cycle.status.success());
    assert!(String::from_utf8_lossy(&cycle.stderr).contains("cycle"));

    json_output(root, &["area", "parent", "child-area", "--remove"]);
    let removed = json_output(root, &["status", "child-area"]);
    assert_eq!(removed["area"]["parent"], Value::Null);
    assert_eq!(removed["area"]["base_commit"], established_anchor);
    assert_eq!(removed["branch_status"]["effective_base"]["kind"], "trunk");
}

#[test]
fn parent_link_rejects_predeclared_branches_that_do_not_exist_yet() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "parent-area", "parent-area");
    create_area(root, "child-area", "child-area");

    let rejected = run_zdev(root, &["area", "parent", "child-area", "parent-area"]);
    assert!(!rejected.status.success());
    assert!(
        String::from_utf8_lossy(&rejected.stderr)
            .contains("missing locally. Restore or create it, then retry")
    );
    let metadata_path = root.join(".zdev/child-area/area.toml");
    let metadata = fs::read_to_string(&metadata_path).expect("child metadata");
    assert!(!metadata.contains("parent ="));

    fs::write(
        &metadata_path,
        format!("{metadata}parent = \"parent-area\"\n"),
    )
    .expect("write unverified parent metadata");
    let diagnosed = json_output(root, &["status", "child-area"]);
    assert_eq!(diagnosed["branch_status"]["fresh"], Value::Null);
    assert_eq!(diagnosed["branch_status"]["finalized"], Value::Null);
    let diagnostics = diagnosed["branch_status"]["diagnostics"]
        .as_array()
        .expect("diagnostics");
    assert!(diagnostics.contains(&json!("effective-base-missing")));
    assert!(!diagnostics.contains(&json!("fresh")));
}

#[test]
fn task_files_drive_selection_completion_and_generated_summary() {
    let repository = repository();
    let root = repository.path();

    assert_eq!(
        json_output(root, &["init", "--record", "project"])["status"],
        "created"
    );
    assert!(!root.join(".zdev/.gitignore").exists());
    assert_eq!(
        json_output(
            root,
            &[
                "area",
                "create",
                "lean-core",
                "--title",
                "Lean core",
                "--objective",
                "Run the smallest useful task loop.",
            ],
        )["status"],
        "created"
    );

    let bundle = root.join("tasks.json");
    fs::write(
        &bundle,
        serde_json::to_vec_pretty(&json!({
            "schema_version": 1,
            "area": "lean-core",
            "tasks": [
                {
                    "key": "first",
                    "title": "Build the first slice",
                    "outcome": "The first slice works.",
                    "context": "The current command stops at the storage boundary. Extend `src/lib.rs` at the task renderer and preserve the existing import behavior covered by this test.",
                    "boundaries": ["Do not change the dependent slice."],
                    "done_when": ["Focused behavior is covered."],
                    "validation": ["Run the focused test."],
                    "blocked_by": []
                },
                {
                    "key": "second",
                    "title": "Build the dependent slice",
                    "outcome": "The dependent slice works.",
                    "done_when": ["It uses the first slice."],
                    "validation": [],
                    "blocked_by": ["first"]
                }
            ]
        }))
        .expect("bundle JSON"),
    )
    .expect("write bundle");

    let imported = json_output(
        root,
        &[
            "tasks",
            "import",
            "lean-core",
            "--from",
            bundle.to_str().expect("bundle path"),
        ],
    );
    assert_eq!(imported["tasks"].as_array().expect("tasks").len(), 2);
    assert!(bundle.exists(), "path-based imports preserve their source");
    let first_task =
        fs::read_to_string(root.join(".zdev/lean-core/tasks/001-build-the-first-slice.md"))
            .expect("first task");
    assert!(first_task.contains(
        "## Context\n\nThe current command stops at the storage boundary. Extend `src/lib.rs` at \
         the task renderer and preserve the existing import behavior covered by this test."
    ));
    assert!(first_task.contains("## Boundaries\n\n- Do not change the dependent slice."));
    let second_task =
        fs::read_to_string(root.join(".zdev/lean-core/tasks/002-build-the-dependent-slice.md"))
            .expect("second task");
    assert!(!second_task.contains("## Context"));

    let next = json_output(root, &["next", "lean-core"]);
    assert_eq!(next["task"]["id"], "lean-core-001");

    let list = json_output(root, &["tasks", "list", "lean-core"]);
    assert_eq!(list["tasks"][0]["state"], "ready");
    assert_eq!(list["tasks"][1]["state"], "blocked");

    assert_eq!(
        json_output(
            root,
            &[
                "task",
                "done",
                "lean-core",
                "lean-core-001",
                "--summary",
                "Implemented and independently verified.",
                "--validation",
                "Focused test passed.",
            ],
        )["status"],
        "done"
    );

    let next = json_output(root, &["next", "lean-core"]);
    assert_eq!(next["task"]["id"], "lean-core-002");
    let summary = fs::read_to_string(root.join(".zdev/lean-core/TASKS.md")).expect("summary");
    assert!(summary.contains("- Done: 1"));
    assert!(summary.contains("lean-core-002"));
    assert_eq!(json_output(root, &["check", "lean-core"])["status"], "ok");
}

#[test]
fn task_order_uses_numeric_suffix_with_full_id_as_tie_breaker() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "numeric-order",
            "--title",
            "Numeric order",
            "--objective",
            "Order tasks by their numeric ID suffix.",
        ],
    );

    let tasks = root.join(".zdev/numeric-order/tasks");
    for (filename, id, key, title) in [
        ("010-ten.md", "numeric-order-10", "ten", "Ten"),
        ("002-two.md", "numeric-order-2", "two", "Two"),
        (
            "003-padded-two.md",
            "numeric-order-002",
            "padded-two",
            "Padded two",
        ),
    ] {
        fs::write(
            tasks.join(filename),
            format!(
                "+++\nschema_version = 1\nid = \"{id}\"\nkey = \"{key}\"\narea = \"numeric-order\"\nstatus = \"open\"\nblocked_by = []\n+++\n# {title}\n\n## Outcome\n\nExercise numeric task ordering.\n\n## Done when\n\n- [ ] The task is selected deterministically.\n\n## Validation\n\n- Exercise the CLI.\n"
            ),
        )
        .expect("numeric-order task");
    }

    let list = json_output(root, &["tasks", "list", "numeric-order"]);
    assert_eq!(
        list["tasks"]
            .as_array()
            .expect("tasks")
            .iter()
            .map(|task| task["id"].as_str().expect("task ID"))
            .collect::<Vec<_>>(),
        ["numeric-order-002", "numeric-order-2", "numeric-order-10"]
    );
    assert_eq!(
        json_output(root, &["next", "numeric-order"])["task"]["id"],
        "numeric-order-002"
    );

    json_output(root, &["tasks", "index", "numeric-order"]);
    let summary = fs::read_to_string(root.join(".zdev/numeric-order/TASKS.md")).expect("summary");
    let padded_two = summary.find("numeric-order-002").expect("padded two");
    let two = summary.find("numeric-order-2").expect("two");
    let ten = summary.find("numeric-order-10").expect("ten");
    assert!(padded_two < two && two < ten);
}

#[test]
fn task_import_reads_a_complete_bundle_from_standard_input() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "stdin",
            "--title",
            "Standard input",
            "--objective",
            "Import tasks without a transport file.",
        ],
    );
    let bundle = serde_json::to_vec(&json!({
        "schema_version": 1,
        "area": "stdin",
        "tasks": [{
            "key": "one",
            "title": "Import one task",
            "outcome": "The task is imported from standard input.",
            "done_when": ["The task file exists."],
            "validation": ["Exercise the CLI."],
            "blocked_by": []
        }]
    }))
    .expect("task bundle");

    let imported =
        json_output_with_stdin(root, &["tasks", "import", "stdin", "--from", "-"], &bundle);

    assert_eq!(imported["tasks"][0], "stdin-001");
    assert!(
        root.join(".zdev/stdin/tasks/001-import-one-task.md")
            .exists()
    );
}

#[test]
fn reviewed_task_bundle_fingerprint_guards_the_import() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "approval",
            "--title",
            "Approval",
            "--objective",
            "Import exactly the reviewed task bundle.",
        ],
    );
    let bundle = json!({
        "schema_version": 1,
        "area": "approval",
        "tasks": [{
            "key": "one",
            "title": "Import one reviewed task",
            "outcome": "The reviewed task is imported unchanged.",
            "context": "Use the existing task import path.",
            "boundaries": ["Keep approval stateless."],
            "done_when": ["The reviewed bundle imports."],
            "validation": ["Exercise review and import."],
            "blocked_by": []
        }]
    });
    let bytes = serde_json::to_vec(&bundle).expect("task bundle");

    let reviewed = json_output_with_stdin(
        root,
        &["tasks", "review", "approval", "--from", "-"],
        &bytes,
    );
    assert_eq!(reviewed["status"], "reviewed");
    assert_eq!(reviewed["area"], "approval");
    let approval = reviewed["approval"].as_str().expect("approval ID");
    assert_eq!(approval.len(), 17);
    let markdown = reviewed["markdown"].as_str().expect("approval Markdown");
    for value in [
        "approval",
        "one",
        "Import one reviewed task",
        "The reviewed task is imported unchanged.",
        "Use the existing task import path.",
        "Keep approval stateless.",
        "The reviewed bundle imports.",
        "Exercise review and import.",
    ] {
        assert!(markdown.contains(value));
    }
    let pretty = serde_json::to_vec_pretty(&bundle).expect("pretty task bundle");
    let reviewed_pretty = json_output_with_stdin(
        root,
        &["tasks", "review", "approval", "--from", "-"],
        &pretty,
    );
    assert_eq!(reviewed_pretty["approval"], reviewed["approval"]);

    let mut changed = bundle.clone();
    changed["tasks"][0]["title"] = json!("Changed after review");
    let changed = serde_json::to_vec(&changed).expect("changed task bundle");
    let rejected = json_output_with_stdin_status(
        root,
        &[
            "tasks",
            "import",
            "approval",
            "--from",
            "-",
            "--approval",
            approval,
        ],
        &changed,
    );
    assert!(!rejected.status.success());
    assert_eq!(
        fs::read_dir(root.join(".zdev/approval/tasks"))
            .expect("tasks directory")
            .count(),
        0
    );

    let imported = json_output_with_stdin(
        root,
        &[
            "tasks",
            "import",
            "approval",
            "--from",
            "-",
            "--approval",
            approval,
        ],
        &bytes,
    );
    assert_eq!(imported["tasks"], json!(["approval-001"]));
}

#[test]
fn state_lock_recovers_a_stale_file_without_stealing_a_live_lock() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "locking",
            "--title",
            "Locking",
            "--objective",
            "Serialize task state changes.",
        ],
    );
    import_one_task(root, "locking");
    let lock_path = root.join(".git/zdev-state.lock");
    fs::write(&lock_path, "owner process exited\n").expect("stale lock file");
    let bundle = |key: &str, title: &str| {
        serde_json::to_vec(&json!({
            "schema_version": 1,
            "area": "locking",
            "tasks": [{
                "key": key,
                "title": title,
                "outcome": "The state update is serialized.",
                "done_when": ["The task is imported."],
                "blocked_by": []
            }]
        }))
        .expect("bundle")
    };

    json_output_with_stdin(
        root,
        &["tasks", "import", "locking", "--from", "-"],
        &bundle("stale", "Recover stale lock"),
    );
    assert!(
        root.join(".zdev/locking/tasks/002-recover-stale-lock.md")
            .exists()
    );

    let live_lock = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&lock_path)
        .expect("live lock file");
    live_lock.lock().expect("hold live state lock");
    let mut child = Command::new(env!("CARGO_BIN_EXE_zdev"))
        .arg("--root")
        .arg(root)
        .args(["tasks", "import", "locking", "--from", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("start waiting import");
    child
        .stdin
        .take()
        .expect("child stdin")
        .write_all(&bundle("live", "Wait for live lock"))
        .expect("write bundle");

    std::thread::sleep(std::time::Duration::from_millis(200));
    assert!(child.try_wait().expect("inspect waiting import").is_none());
    assert!(
        !root
            .join(".zdev/locking/tasks/003-wait-for-live-lock.md")
            .exists()
    );
    drop(live_lock);
    let output = child.wait_with_output().expect("finish waiting import");
    assert!(
        output.status.success(),
        "zdev failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn non_git_task_import_remains_available_without_locking() {
    let directory = tempfile::tempdir().expect("non-Git directory");
    let root = directory.path();
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "standalone",
            "--title",
            "Standalone",
            "--objective",
            "Keep ordinary task state usable outside Git.",
            "--branch",
            "standalone",
        ],
    );
    let bundle = serde_json::to_vec(&json!({
        "schema_version": 1,
        "area": "standalone",
        "tasks": [{
            "key": "one",
            "title": "Use standalone task state",
            "outcome": "Ordinary task operations work without Git.",
            "done_when": ["The task can be imported."],
            "validation": ["Exercise the CLI."],
            "blocked_by": []
        }]
    }))
    .expect("bundle");

    json_output_with_stdin(
        root,
        &["tasks", "import", "standalone", "--from", "-"],
        &bundle,
    );
    json_output(root, &["tasks", "index", "standalone"]);
    let refused = json_output_with_stdin_status(
        root,
        &["tasks", "import", "standalone", "--from", "-", "--commit"],
        &bundle,
    );
    assert!(!refused.status.success());
    assert!(!root.join(".git").exists());
}

#[test]
fn committed_task_import_preserves_unrelated_index_and_worktree_changes() {
    let repository = repository();
    let root = repository.path();
    commit_file(root, "staged.txt", "before staged\n", "seed staged file");
    commit_file(
        root,
        "unstaged.txt",
        "before unstaged\n",
        "seed unstaged file",
    );
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "concurrent",
            "--title",
            "Concurrent",
            "--objective",
            "Accept additive tasks during implementation.",
        ],
    );
    import_one_task(root, "concurrent");
    commit_all(root, "configure concurrent area");
    fs::write(root.join(".git/info/exclude"), "/.zdev/concurrent/tasks/\n")
        .expect("ignore new task files");
    fs::write(root.join("staged.txt"), "staged implementation\n").expect("staged change");
    git(root, &["add", "staged.txt"]);
    fs::write(root.join("unstaged.txt"), "unstaged implementation\n").expect("unstaged change");
    let bundle = serde_json::to_vec(&json!({
        "schema_version": 1,
        "area": "concurrent",
        "tasks": [{
            "key": "added",
            "title": "Add concurrent task",
            "outcome": "The task joins the next selection boundary.",
            "done_when": ["The task is available."],
            "validation": ["Exercise the CLI."],
            "blocked_by": []
        }]
    }))
    .expect("bundle");

    let imported = json_output_with_stdin(
        root,
        &["tasks", "import", "concurrent", "--from", "-", "--commit"],
        &bundle,
    );

    assert_eq!(imported["status"], "committed");
    assert_eq!(imported["tasks"][0], "concurrent-002");
    assert_eq!(imported["paths"].as_array().expect("paths").len(), 2);
    assert!(
        imported["commit"]
            .as_str()
            .is_some_and(|value| value.len() == 40)
    );
    assert!(
        imported["change_id"]
            .as_str()
            .is_some_and(|value| value.starts_with('Z') && value.len() == 65)
    );
    assert_eq!(
        git(root, &["show", "--pretty=format:", "--name-only", "HEAD"])
            .lines()
            .collect::<Vec<_>>(),
        [
            ".zdev/concurrent/TASKS.md",
            ".zdev/concurrent/tasks/002-add-concurrent-task.md",
        ]
    );
    assert_eq!(
        git(root, &["diff", "--cached", "--name-only"]),
        "staged.txt"
    );
    assert_eq!(git(root, &["diff", "--name-only"]), "unstaged.txt");
    assert_eq!(
        json_output(root, &["next", "concurrent"])["task"]["id"],
        "concurrent-001"
    );
}

#[test]
fn committed_task_import_recovery_text_names_the_problem_and_action() {
    let repository = repository();
    let root = repository.path();
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "intake-action",
            "--title",
            "Intake action",
            "--objective",
            "Explain how to recover a refused task import.",
        ],
    );
    commit_all(root, "configure intake area");
    fs::write(
        root.join(".zdev/intake-action/brief.md"),
        "# Locally changed\n",
    )
    .expect("change area brief");
    let bundle = serde_json::to_vec(&json!({
        "schema_version": 1,
        "area": "intake-action",
        "tasks": [{
            "key": "added",
            "title": "Add one task",
            "outcome": "The task is added after recovery.",
            "done_when": ["The task exists."],
            "blocked_by": []
        }]
    }))
    .expect("bundle");

    let output = json_output_with_stdin_status(
        root,
        &[
            "tasks",
            "import",
            "intake-action",
            "--from",
            "-",
            "--commit",
        ],
        &bundle,
    );

    assert!(!output.status.success());
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("Cannot add and commit tasks"));
    assert!(error.contains("already has local changes"));
    assert!(error.contains("Commit or resolve them first"));
}

#[cfg(unix)]
#[test]
fn failed_committed_task_import_rolls_back_planning_changes_and_preserves_index() {
    use std::os::unix::fs::PermissionsExt;

    let repository = repository();
    let root = repository.path();
    commit_file(
        root,
        "implementation.txt",
        "before\n",
        "seed implementation",
    );
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "commit-failure",
            "--title",
            "Commit failure",
            "--objective",
            "Recover a failed additive import commit.",
        ],
    );
    import_one_task(root, "commit-failure");
    commit_all(root, "configure failure area");
    let summary_before =
        fs::read_to_string(root.join(".zdev/commit-failure/TASKS.md")).expect("summary");
    fs::write(root.join("implementation.txt"), "staged implementation\n")
        .expect("implementation change");
    git(root, &["add", "implementation.txt"]);
    let hook = root.join(".git/hooks/pre-commit");
    fs::write(&hook, "#!/bin/sh\nexit 1\n").expect("rejecting hook");
    let mut permissions = fs::metadata(&hook).expect("hook metadata").permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&hook, permissions).expect("executable hook");
    let bundle = serde_json::to_vec(&json!({
        "schema_version": 1,
        "area": "commit-failure",
        "tasks": [{
            "key": "added",
            "title": "Must roll back",
            "outcome": "The failed commit leaves no partial import.",
            "done_when": ["No partial task remains."],
            "blocked_by": []
        }]
    }))
    .expect("bundle");

    let output = json_output_with_stdin_status(
        root,
        &[
            "tasks",
            "import",
            "commit-failure",
            "--from",
            "-",
            "--commit",
        ],
        &bundle,
    );

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("rolled back"));
    assert!(
        !root
            .join(".zdev/commit-failure/tasks/002-must-roll-back.md")
            .exists()
    );
    assert_eq!(
        fs::read_to_string(root.join(".zdev/commit-failure/TASKS.md")).expect("summary"),
        summary_before
    );
    assert_eq!(
        git(root, &["diff", "--cached", "--name-only"]),
        "implementation.txt"
    );
    assert_eq!(
        git(root, &["log", "-1", "--format=%s"]),
        "configure failure area"
    );
}

#[test]
fn check_rejects_a_task_without_the_authored_contract() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "contract",
            "--title",
            "Contract",
            "--objective",
            "Reject malformed authored tasks.",
        ],
    );
    fs::write(
        root.join(".zdev/contract/tasks/001-malformed.md"),
        "+++\nschema_version = 1\nid = \"contract-001\"\nkey = \"malformed\"\narea = \"contract\"\nstatus = \"open\"\nblocked_by = []\n+++\n# Malformed\n\n## Validation\n\n- Nothing.\n",
    )
    .expect("malformed task");
    let output = run_zdev(root, &["check", "contract"]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("## Outcome"));
}

#[test]
fn check_validates_the_authored_brief_contract() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "brief-contract",
            "--title",
            "Brief contract",
            "--objective",
            "Reject malformed authored briefs.",
        ],
    );
    let brief = root.join(".zdev/brief-contract/brief.md");

    assert_eq!(
        json_output(root, &["check", "brief-contract"])["status"],
        "ok"
    );
    assert!(
        fs::read_to_string(&brief)
            .expect("created brief")
            .contains("## Testing\n\nExisting checks only.")
    );

    for (content, error) in [
        (
            "# Brief\n\n## Testing\n\nFocused coverage.\n",
            "Brief lacks ## Objective",
        ),
        (
            "# Brief\n\n## Objective\n\nOne.\n\n## Objective\n\nTwo.\n\n## Testing\n\nFocused coverage.\n",
            "Brief repeats ## Objective",
        ),
        (
            "# Brief\n\n## Objective\n\n## Testing\n\nFocused coverage.\n",
            "Brief has empty ## Objective",
        ),
        (
            "# Brief\n\n## Objective\n\nOne.\n",
            "Brief lacks ## Testing",
        ),
        (
            "# Brief\n\n## Objective\n\nOne.\n\n## Testing\n\nFocused coverage.\n\n## Testing\n\nNo new tests.\n",
            "Brief repeats ## Testing",
        ),
        (
            "# Brief\n\n## Objective\n\nOne.\n\n## Testing\n",
            "Brief has empty ## Testing",
        ),
        (
            "# Brief\n\n## Objective\n\nOne.\n\n## Testing\n\nChoose tests later.\n",
            "Brief ## Testing must state one of",
        ),
    ] {
        fs::write(&brief, content).expect("malformed brief");
        let output = run_zdev(root, &["check", "brief-contract"]);
        assert!(
            !output.status.success(),
            "brief unexpectedly passed: {content}"
        );
        assert!(
            String::from_utf8_lossy(&output.stderr).contains(error),
            "expected {error}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    fs::write(
        &brief,
        "# Brief\n\n## Objective\n\nOne.\n\n## Testing\n\nUse broader regression coverage.\n",
    )
    .expect("valid brief");
    assert_eq!(
        json_output(root, &["check", "brief-contract"])["status"],
        "ok"
    );
}

#[test]
fn failed_summary_preflight_does_not_complete_the_task() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "atomic",
            "--title",
            "Atomic",
            "--objective",
            "Publish task state and summary together.",
        ],
    );
    let bundle = root.join("atomic.json");
    fs::write(
        &bundle,
        serde_json::to_vec(&json!({
            "schema_version": 1,
            "area": "atomic",
            "tasks": [{
                "key": "one",
                "title": "Complete atomically",
                "outcome": "Task and summary agree.",
                "done_when": ["Both files change together."],
                "validation": ["Inspect both files."],
                "blocked_by": []
            }]
        }))
        .expect("JSON"),
    )
    .expect("bundle");
    json_output(
        root,
        &[
            "tasks",
            "import",
            "atomic",
            "--from",
            bundle.to_str().expect("bundle path"),
        ],
    );
    let summary = root.join(".zdev/atomic/TASKS.md");
    fs::remove_file(&summary).expect("remove summary");
    fs::create_dir(&summary).expect("replace summary with directory");

    let output = run_zdev(
        root,
        &[
            "task",
            "done",
            "atomic",
            "atomic-001",
            "--summary",
            "Should not be retained.",
            "--validation",
            "Forced failure.",
        ],
    );
    assert!(!output.status.success());
    let task = fs::read_to_string(root.join(".zdev/atomic/tasks/001-complete-atomically.md"))
        .expect("task");
    assert!(task.contains("status = \"open\""));
    assert!(task.contains("- [ ] Both files change together."));
}

#[test]
fn commit_adds_a_stable_change_id_that_lookup_can_find() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    fs::write(root.join("file.txt"), "one\n").expect("source file");
    git(root, &["add", "file.txt", ".zdev"]);

    let committed = json_output(root, &["commit", "-m", "feat: add one"]);
    let change_id = committed["change_id"].as_str().expect("change ID");
    assert!(change_id.starts_with('Z'));
    assert_eq!(change_id.len(), 65);

    let inspected = json_output(root, &["change", "inspect", "HEAD"]);
    assert_eq!(inspected["change_id"], change_id);
    fs::write(root.join("file.txt"), "two\n").expect("source file");
    git(root, &["add", "file.txt"]);
    git(
        root,
        &[
            "commit",
            "-m",
            "feat: add the same logical change again",
            "-m",
            &format!("Zdev-Change-Id: {change_id}"),
        ],
    );
    let found = json_output(root, &["change", "lookup", change_id]);
    assert_eq!(found["commits"].as_array().expect("commits").len(), 2);
}

#[test]
fn commit_human_output_reports_the_commit_change_id_and_subject() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    fs::write(root.join("file.txt"), "one\n").expect("source file");
    git(root, &["add", "file.txt", ".zdev"]);

    let output = run_zdev(root, &["commit", "-m", "feat: report the commit"]);

    assert!(output.status.success());
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(text.starts_with("Committed "));
    assert!(text.contains(" (Z"));
    assert!(text.ends_with(": feat: report the commit\n"));
}

#[test]
fn commit_rejects_a_user_supplied_stable_change_id_before_committing() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    commit_all(root, "initialize zdev");
    fs::write(root.join("file.txt"), "staged\n").expect("source file");
    git(root, &["add", "file.txt"]);
    let head = git(root, &["rev-parse", "HEAD"]);

    let output = run_zdev(
        root,
        &[
            "commit",
            "-m",
            "feat: rejected commit",
            "--body",
            "Context for the change.",
            "--body",
            "Zdev-Change-Id: Z0000000000000000000000000000000000000000000000000000000000000000",
        ],
    );

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("Commit body must not contain a Zdev-Change-Id trailer")
    );
    assert_eq!(git(root, &["rev-parse", "HEAD"]), head);
    assert!(
        git(root, &["diff", "--cached", "--name-only"])
            .lines()
            .any(|path| path == "file.txt")
    );
}

#[test]
fn malformed_dependencies_fail_before_any_task_file_is_created() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "invalid",
            "--title",
            "Invalid",
            "--objective",
            "Reject invalid task graphs.",
        ],
    );
    let bundle = root.join("invalid.json");
    fs::write(
        &bundle,
        serde_json::to_vec(&json!({
            "schema_version": 1,
            "area": "invalid",
            "tasks": [{
                "key": "one",
                "title": "One",
                "outcome": "One works.",
                "done_when": ["One is done."],
                "blocked_by": ["missing"]
            }]
        }))
        .expect("JSON"),
    )
    .expect("bundle");
    let output = run_zdev(
        root,
        &[
            "tasks",
            "import",
            "invalid",
            "--from",
            bundle.to_str().expect("path"),
        ],
    );
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("unknown blocker"));
    assert_eq!(
        fs::read_dir(root.join(".zdev/invalid/tasks"))
            .expect("tasks")
            .count(),
        0
    );
}

#[cfg(unix)]
#[test]
fn failed_task_write_removes_earlier_and_current_partial_files() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "rollback",
            "--title",
            "Rollback",
            "--objective",
            "Remove partial task imports.",
        ],
    );
    let bundle = root.join("rollback.json");
    fs::write(
        &bundle,
        serde_json::to_vec(&json!({
            "schema_version": 1,
            "area": "rollback",
            "tasks": [
                {
                    "key": "one",
                    "title": "One",
                    "outcome": "The first task is written.",
                    "done_when": ["The first task exists."],
                    "blocked_by": []
                },
                {
                    "key": "two",
                    "title": "Two",
                    "outcome": "A".repeat(2048),
                    "done_when": ["The second task exists."],
                    "blocked_by": []
                }
            ]
        }))
        .expect("JSON"),
    )
    .expect("bundle");

    let output = Command::new("sh")
        .arg("-c")
        .arg("trap '' XFSZ 2>/dev/null || true; ulimit -f 1; exec \"$@\"")
        .arg("sh")
        .arg(env!("CARGO_BIN_EXE_zdev"))
        .arg("--root")
        .arg(root)
        .args([
            "tasks",
            "import",
            "rollback",
            "--from",
            bundle.to_str().expect("bundle path"),
        ])
        .output()
        .expect("run size-limited zdev");

    assert!(!output.status.success());
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("Cannot write"));
    assert!(error.contains("002-two.md"));
    assert_eq!(
        fs::read_dir(root.join(".zdev/rollback/tasks"))
            .expect("tasks")
            .count(),
        0
    );
}

#[test]
fn skill_install_materializes_the_complete_embedded_skill_safely() {
    let repository = repository();
    let root = repository.path();
    let destination = root.join("installed/zdev");
    let destination_text = destination.to_str().expect("destination path");
    let packaged_skill = include_str!("../skills/zdev/SKILL.md");

    let installed = json_output(
        root,
        &["skill", "install", "codex", "--to", destination_text],
    );
    assert_eq!(installed["status"], "created");
    assert_eq!(installed["files"], 12);
    let rendered = fs::read_to_string(destination.join("SKILL.md")).expect("installed skill");
    assert_eq!(rendered, packaged_skill);
    assert_eq!(
        fs::read_to_string(destination.join("references/verify.md")).expect("verify reference"),
        include_str!("../skills/zdev/references/verify.md")
    );
    assert_eq!(
        fs::read_to_string(destination.join("references/discuss.md")).expect("discuss reference"),
        include_str!("../skills/zdev/references/discuss.md").replace(
            "{{question_tool_guidance}}",
            "Use Codex's `request_user_input` tool with two or three questions in one call when it is available. Put the recommended option first for each question and explain its impact. If the tool is unavailable, present the same round as a concise numbered list."
        )
    );
    assert_eq!(
        fs::read_to_string(destination.join("references/task-format.md"))
            .expect("task format reference"),
        include_str!("../skills/zdev/references/task-format.md")
    );

    let unchanged = json_output(
        root,
        &["skill", "install", "codex", "--to", destination_text],
    );
    assert_eq!(unchanged["status"], "unchanged");

    fs::write(destination.join("SKILL.md"), "locally changed\n").expect("change skill");
    let refused = run_zdev(
        root,
        &["skill", "install", "codex", "--to", destination_text],
    );
    assert!(!refused.status.success());
    assert!(String::from_utf8_lossy(&refused.stderr).contains("--force"));

    let replaced = json_output(
        root,
        &[
            "skill",
            "install",
            "codex",
            "--to",
            destination_text,
            "--force",
        ],
    );
    assert_eq!(replaced["status"], "replaced");
    assert_eq!(
        fs::read_to_string(destination.join("SKILL.md")).expect("replaced skill"),
        rendered
    );
}

#[test]
fn skill_human_output_names_zdev_integrations_and_their_harnesses() {
    let repository = repository();
    let root = repository.path();

    for (harness, display_name, skill_path) in [
        ("codex", "Codex", "SKILL.md"),
        ("claude", "Claude Code", "skills/zdev/SKILL.md"),
        ("opencode", "OpenCode", "skills/zdev-opencode/SKILL.md"),
        ("pi", "Pi", "skills/zdev-pi/SKILL.md"),
        ("omp", "Oh My Pi", "skills/zdev/SKILL.md"),
    ] {
        let destination = root.join(format!("human-output/{harness}"));
        let destination_text = destination.to_str().expect("destination path");

        let installed = run_zdev(
            root,
            &["skill", "install", harness, "--to", destination_text],
        );
        assert!(installed.status.success());
        assert_eq!(
            String::from_utf8_lossy(&installed.stdout).trim(),
            format!(
                "Installed zdev integration for {display_name} at {}",
                destination.display()
            )
        );

        let unchanged = run_zdev(
            root,
            &["skill", "install", harness, "--to", destination_text],
        );
        assert!(unchanged.status.success());
        assert_eq!(
            String::from_utf8_lossy(&unchanged.stdout).trim(),
            format!(
                "{display_name} zdev integration is already current at {}",
                destination.display()
            )
        );

        let checked = run_zdev(root, &["skill", "check", harness, "--to", destination_text]);
        assert!(checked.status.success());
        assert_eq!(
            String::from_utf8_lossy(&checked.stdout).trim(),
            format!(
                "{display_name} zdev integration is ready at {}",
                destination.display()
            )
        );

        fs::write(destination.join(skill_path), "locally changed\n")
            .expect("change installed skill");
        let conflict = run_zdev(root, &["skill", "check", harness, "--to", destination_text]);
        assert_eq!(conflict.status.code(), Some(1));
        assert_eq!(
            String::from_utf8_lossy(&conflict.stdout).trim(),
            format!(
                "{display_name} zdev integration differs from this version at {}. Replace it with `zdev skill install {harness} --to {} --force`",
                destination.display(),
                destination.display(),
            )
        );

        let refused = run_zdev(
            root,
            &["skill", "install", harness, "--to", destination_text],
        );
        assert!(!refused.status.success());
        let error = String::from_utf8_lossy(&refused.stderr);
        assert!(error.contains("zdev integration"), "{error}");
        assert!(error.contains(display_name), "{error}");
        assert!(!error.contains("harness installation"), "{error}");
    }
}

#[test]
fn skill_help_describes_integrations_for_harnesses() {
    let repository = repository();
    let root = repository.path();

    let skill_help = run_zdev(root, &["skill", "--help"]);
    assert!(skill_help.status.success());
    let skill_help = String::from_utf8_lossy(&skill_help.stdout);
    assert!(skill_help.contains("coding-harness integration"));
    assert!(!skill_help.contains("harness bundle"));

    let install_help = run_zdev(root, &["skill", "install", "--help"]);
    assert!(install_help.status.success());
    let install_help = String::from_utf8_lossy(&install_help.stdout);
    assert!(install_help.contains("coding-harness integration"));
    assert!(install_help.contains("Install into this exact directory"));
    assert!(install_help.contains("omp"));
    assert!(!install_help.contains("harness bundle"));
}

#[test]
fn every_help_page_explains_its_command_and_inputs() {
    let repository = repository();
    let root = repository.path();
    let cases: &[(&[&str], &[&str])] = &[
        (
            &["--help"],
            &[
                "development plans and tasks in plain files under .zdev",
                "Use PATH as the repository root",
                "instructions",
                "Print concise instructions for a coding harness",
                "Start a repository:",
                "Choose personal, project, or pull-request record storage",
                "zdev init --record <POLICY>",
            ],
        ),
        (
            &["instructions", "--help"],
            &["Print concise instructions for a coding harness"],
        ),
        (
            &["init", "--help"],
            &[
                "Initialize zdev in this repository",
                "records the checked-out branch as trunk when",
                "On detached HEAD, trunk remains unbound",
                "does not install a coding-harness integration",
                "Use --record personal to keep .zdev clone-local",
                "pull-request to track it for review",
                "`zdev cleanup squash` before squash merge",
                "How the .zdev planning record is stored and shared",
            ],
        ),
        (
            &["cleanup", "--help"],
            &["Remove pull-request-only zdev development records"],
        ),
        (
            &["cleanup", "squash", "--help"],
            &["Delete tracked .zdev files in one plain commit before a squash merge"],
        ),
        (
            &["config", "--help"],
            &["Configure repository-wide zdev settings"],
        ),
        (
            &["config", "trunk", "--help"],
            &[
                "Set the branch that areas use as their default base",
                "omit to use the checked-out branch",
            ],
        ),
        (
            &["area", "--help"],
            &["groups one objective, its brief, its tasks, and the branch"],
        ),
        (
            &["area", "create", "--help"],
            &[
                "Create an area for one objective on a branch",
                "Short identifier used in paths and task IDs",
                "Human-readable area name",
                "One-line description of the outcome",
            ],
        ),
        (
            &["area", "bind", "--help"],
            &["Area tag to update", "Area branch"],
        ),
        (
            &["area", "parent", "--help"],
            &[
                "Child area tag to update",
                "Parent area tag",
                "use project trunk as its base",
            ],
        ),
        (
            &["area", "rebase", "--help"],
            &[
                "parent area's branch, or project trunk",
                "after conflicts have been resolved and staged",
                "restore its previous branch state",
            ],
        ),
        (&["tasks", "--help"], &["review", "import", "list", "index"]),
        (&["tasks", "review", "--help"], &["--from", "PATH_OR_DASH"]),
        (
            &["tasks", "import", "--help"],
            &[
                "reviewed JSON task bundle",
                "Area tag that will own the imported tasks",
                "or - to read the bundle from standard input",
            ],
        ),
        (
            &["tasks", "list", "--help"],
            &["every task in an area with its current state", "Area tag"],
        ),
        (&["tasks", "index", "--help"], &["TASKS.md", "Area tag"]),
        (
            &["task", "--help"],
            &["Inspect or change the state of one task"],
        ),
        (
            &["task", "show", "--help"],
            &["complete Markdown file", "Task ID, such as scheduling-001"],
        ),
        (
            &["task", "done", "--help"],
            &[
                "Mark a verified task complete",
                "Concise description of the completed outcome",
                "repeat for each independent check",
            ],
        ),
        (
            &["task", "reopen", "--help"],
            &["Mark a completed task open again", "Task ID to reopen"],
        ),
        (
            &["next", "--help"],
            &[
                "Show the next ready task",
                "let zdev select an unambiguous active area",
            ],
        ),
        (
            &["status", "--help"],
            &[
                "ready, blocked, and completed task counts",
                "project-wide summary",
            ],
        ),
        (
            &["check", "--help"],
            &[
                "Checks existing files without rewriting them",
                "omit to check every area",
            ],
        ),
        (
            &["skill", "--help"],
            &["User-scoped integrations are shared across repositories"],
        ),
        (
            &["skill", "install", "--help"],
            &[
                "Harness values are codex (Codex)",
                "Install for the current user or in this repository",
                "Project guidance source",
                "Replace an installed integration whose files differ",
            ],
        ),
        (
            &["skill", "check", "--help"],
            &[
                "ready to use",
                "status `missing` or `conflict`",
                "Coding harness whose zdev integration to check",
                "Check the current user's installation",
            ],
        ),
        (
            &["commit", "--help"],
            &[
                "This command does not stage files",
                "Git commit subject",
                "repeat for multiple paragraphs",
            ],
        ),
        (
            &["change-id", "--help"],
            &["Prints an ID without changing Git state"],
        ),
        (
            &["change", "--help"],
            &["Inspect or find commits by stable change ID"],
        ),
        (
            &["change", "inspect", "--help"],
            &["Git revision to inspect, such as HEAD"],
        ),
        (
            &["change", "lookup", "--help"],
            &["all reachable commits", "Stable ID beginning with Z"],
        ),
    ];

    for (arguments, expected) in cases {
        let output = run_zdev(root, arguments);
        assert!(
            output.status.success(),
            "help command failed: zdev {}",
            arguments.join(" ")
        );
        let text = String::from_utf8_lossy(&output.stdout);
        assert!(
            text.contains("Usage:"),
            "missing usage: zdev {}",
            arguments.join(" ")
        );
        assert!(
            text.contains("Choose human-readable text or machine-readable JSON output"),
            "missing format explanation: zdev {}",
            arguments.join(" ")
        );
        for phrase in *expected {
            assert!(
                text.contains(phrase),
                "help for `zdev {}` lacks {phrase:?}:\n{text}",
                arguments.join(" ")
            );
        }
    }
}

#[test]
fn instructions_work_without_a_zdev_or_git_repository() {
    let directory = tempfile::tempdir().expect("temporary directory");
    let run = |arguments: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_zdev"))
            .args(arguments)
            .current_dir(directory.path())
            .output()
            .expect("run zdev")
    };

    let text = run(&["instructions"]);
    assert!(text.status.success());
    assert!(text.stderr.is_empty());
    let paragraph = String::from_utf8(text.stdout).expect("UTF-8 text output");
    assert_eq!(paragraph.lines().count(), 1);
    for required in [
        "durable plans and tasks in `.zdev`",
        "shapes, implements, and independently verifies the work",
        "integration for work tracked there",
        "`zdev status` and `zdev next` orient and select work",
        "`zdev check` validates state",
        "`zdev commit` records verified staged changes",
        "Do not edit generated task indexes",
        "ask the user how to configure it",
    ] {
        assert!(
            paragraph.contains(required),
            "missing {required:?}: {paragraph}"
        );
    }

    let json = run(&["instructions", "--format", "json"]);
    assert!(json.status.success());
    assert!(json.stderr.is_empty());
    let value: Value = serde_json::from_slice(&json.stdout).expect("JSON output");
    assert_eq!(value["schema_version"], 1);
    assert_eq!(value["agent_instructions"], paragraph.trim_end());

    let combined = run(&["instructions", "status"]);
    assert_eq!(combined.status.code(), Some(2));
    assert!(combined.stdout.is_empty());
    assert!(String::from_utf8_lossy(&combined.stderr).contains("unexpected argument 'status'"));
}

#[test]
fn global_options_work_after_commands_and_nested_commands() {
    let repository = repository();
    let root = repository.path().to_str().expect("UTF-8 path");

    let initialized = Command::new(env!("CARGO_BIN_EXE_zdev"))
        .args([
            "init", "--record", "project", "--root", root, "--format", "json",
        ])
        .output()
        .expect("initialize with trailing global options");
    assert!(initialized.status.success());
    let initialized: Value = serde_json::from_slice(&initialized.stdout).expect("JSON output");
    assert_eq!(initialized["status"], "created");

    let branch = git(repository.path(), &["branch", "--show-current"]);

    let configured = Command::new(env!("CARGO_BIN_EXE_zdev"))
        .args([
            "config",
            "trunk",
            branch.as_str(),
            "--root",
            root,
            "--format",
            "json",
        ])
        .output()
        .expect("configure with trailing global options");
    assert!(configured.status.success());
    let configured: Value = serde_json::from_slice(&configured.stdout).expect("JSON output");
    assert_eq!(configured["trunk"], branch);
}

#[test]
fn current_schema_rejects_unknown_and_missing_fields() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);

    let config_path = root.join(".zdev/config.toml");
    let config = fs::read_to_string(&config_path).expect("config");
    fs::write(&config_path, format!("{config}unknown_field = true\n"))
        .expect("write unsupported config field");
    let rejected_config = run_zdev(root, &["status"]);
    assert!(!rejected_config.status.success());
    assert!(String::from_utf8_lossy(&rejected_config.stderr).contains("unknown field"));

    let missing_record = config
        .lines()
        .filter(|line| !line.starts_with("record = "))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&config_path, format!("{missing_record}\n")).expect("remove required record policy");
    let rejected_missing_record = run_zdev(root, &["status"]);
    assert!(!rejected_missing_record.status.success());
    assert!(String::from_utf8_lossy(&rejected_missing_record.stderr).contains("missing field"));

    fs::write(&config_path, config).expect("restore current config");
    json_output(
        root,
        &[
            "area",
            "create",
            "strict-schema",
            "--title",
            "Strict schema",
            "--objective",
            "Reject fields outside the current format.",
        ],
    );
    let area_path = root.join(".zdev/strict-schema/area.toml");
    let area = fs::read_to_string(&area_path).expect("area metadata");
    fs::write(&area_path, format!("{area}unknown_field = true\n"))
        .expect("write unsupported area field");
    let rejected_area = run_zdev(root, &["status", "strict-schema"]);
    assert!(!rejected_area.status.success());
    assert!(String::from_utf8_lossy(&rejected_area.stderr).contains("unknown field"));

    let missing_branch = area
        .lines()
        .filter(|line| !line.starts_with("branch = "))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&area_path, format!("{missing_branch}\n")).expect("remove required branch");
    let rejected_missing_branch = run_zdev(root, &["status", "strict-schema"]);
    assert!(!rejected_missing_branch.status.success());
    assert!(String::from_utf8_lossy(&rejected_missing_branch.stderr).contains("missing field"));
}

#[test]
fn skill_install_and_check_support_explicit_destinations_and_replacement() {
    let repository = repository();
    let root = repository.path();

    for harness in ["codex", "claude"] {
        let destination = root.join(format!("installed/{harness}/zdev"));
        let destination_text = destination.to_str().expect("destination path");

        let installed = json_output(
            root,
            &["skill", "install", harness, "--to", destination_text],
        );
        assert_eq!(installed["harness"], harness);
        assert_eq!(installed["scope"], "explicit");
        assert_eq!(installed["status"], "created");
        assert_eq!(installed["files"], if harness == "codex" { 12 } else { 16 });

        let checked = json_output(root, &["skill", "check", harness, "--to", destination_text]);
        assert_eq!(checked["status"], "ok");
        assert_eq!(checked["harness"], harness);

        let unchanged = json_output(
            root,
            &["skill", "install", harness, "--to", destination_text],
        );
        assert_eq!(unchanged["status"], "unchanged");

        fs::create_dir(destination.join("unexpected-empty-directory"))
            .expect("create extra empty directory");
        let extra_directory = json_output_with_exit_code(
            root,
            &["skill", "check", harness, "--to", destination_text],
            1,
        );
        assert_eq!(extra_directory["status"], "conflict");
        let refused = run_zdev(
            root,
            &["skill", "install", harness, "--to", destination_text],
        );
        assert!(!refused.status.success());
        assert!(String::from_utf8_lossy(&refused.stderr).contains("--force"));
        let replaced = json_output(
            root,
            &[
                "skill",
                "install",
                harness,
                "--to",
                destination_text,
                "--force",
            ],
        );
        assert_eq!(replaced["status"], "replaced");
        assert!(!destination.join("unexpected-empty-directory").exists());

        let skill = if harness == "codex" {
            destination.join("SKILL.md")
        } else {
            destination.join("skills/zdev/SKILL.md")
        };
        fs::write(skill, "locally changed\n").expect("change integration");
        let conflict = json_output_with_exit_code(
            root,
            &["skill", "check", harness, "--to", destination_text],
            1,
        );
        assert_eq!(conflict["status"], "conflict");

        let refused = run_zdev(
            root,
            &["skill", "install", harness, "--to", destination_text],
        );
        assert!(!refused.status.success());
        assert!(String::from_utf8_lossy(&refused.stderr).contains("--force"));

        let replaced = json_output(
            root,
            &[
                "skill",
                "install",
                harness,
                "--to",
                destination_text,
                "--force",
            ],
        );
        assert_eq!(replaced["status"], "replaced");
    }
}

#[test]
fn harnesses_have_distinct_native_zdev_integration_inventories() {
    let repository = repository();
    let root = repository.path();
    let codex = root.join("codex-bundle");
    let claude = root.join("claude-bundle");
    json_output(
        root,
        &[
            "skill",
            "install",
            "codex",
            "--to",
            codex.to_str().expect("codex path"),
        ],
    );
    json_output(
        root,
        &[
            "skill",
            "install",
            "claude",
            "--to",
            claude.to_str().expect("claude path"),
        ],
    );

    assert_eq!(
        file_inventory(&codex),
        [
            "SKILL.md",
            "agents/openai.yaml",
            "references/discuss.md",
            "references/implement.md",
            "references/improve.md",
            "references/investigate.md",
            "references/recovery.md",
            "references/setup.md",
            "references/shape-work.md",
            "references/task-format.md",
            "references/to-tasks.md",
            "references/verify.md",
        ]
    );

    assert_eq!(
        fs::read(codex.join("agents/openai.yaml")).expect("Codex UI metadata"),
        include_bytes!("../skills/zdev/agents/openai.yaml")
    );
    assert_eq!(
        file_inventory(&claude),
        [
            ".claude-plugin/plugin.json",
            "agents/zdev-implementer.md",
            "agents/zdev-verifier.md",
            "skills/zdev/SKILL.md",
            "skills/zdev/references/discuss.md",
            "skills/zdev/references/implement.md",
            "skills/zdev/references/improve.md",
            "skills/zdev/references/investigate.md",
            "skills/zdev/references/recovery.md",
            "skills/zdev/references/setup.md",
            "skills/zdev/references/shape-work.md",
            "skills/zdev/references/task-format.md",
            "skills/zdev/references/to-tasks.md",
            "skills/zdev/references/verify.md",
            "workflows/zdev-audit.js",
            "workflows/zdev-task.js",
        ]
    );

    let manifest: Value = serde_json::from_slice(
        &fs::read(claude.join(".claude-plugin/plugin.json")).expect("Claude manifest"),
    )
    .expect("manifest JSON");
    assert_eq!(manifest["name"], "zdev");
    assert_eq!(manifest["version"], env!("CARGO_PKG_VERSION"));

    let verifier = fs::read_to_string(claude.join("agents/zdev-verifier.md")).expect("verifier");
    assert!(verifier.contains("tools: Read, Bash, Grep, Glob"));
    assert!(!verifier.contains("tools: Read, Write"));

    let task_workflow =
        fs::read_to_string(claude.join("workflows/zdev-task.js")).expect("task workflow");
    assert!(task_workflow.contains("while (/^REWORK\\b/.test(verdict))"));
    assert!(task_workflow.contains("/^(PASS|REWORK|BLOCKER)\\b/"));
    assert_eq!(task_workflow.matches("label: 'zdev rework'").count(), 1);
    assert_eq!(
        task_workflow
            .matches("verdict = await review(implementation)")
            .count(),
        2,
        "Claude workflow must freshly verify the initial implementation and every rework"
    );

    let audit_workflow =
        fs::read_to_string(claude.join("workflows/zdev-audit.js")).expect("audit workflow");
    assert!(audit_workflow.contains("Array.isArray(input.lenses)"));
    assert_eq!(audit_workflow.matches("audit evidence vetter").count(), 1);
}
#[test]
fn checked_in_harness_skills_match_current_templates() {
    let repository = repository();
    let root = repository.path();
    let source = Path::new(env!("CARGO_MANIFEST_DIR"));

    for (harness, rendered_skill, checked_in_skill) in [
        ("codex", "SKILL.md", "skills/zdev/SKILL.md"),
        (
            "claude",
            "skills/zdev/SKILL.md",
            ".claude/skills/zdev/skills/zdev/SKILL.md",
        ),
        (
            "opencode",
            "skills/zdev-opencode/SKILL.md",
            ".opencode/skills/zdev-opencode/SKILL.md",
        ),
        (
            "pi",
            "skills/zdev-pi/SKILL.md",
            ".pi/skills/zdev-pi/SKILL.md",
        ),
    ] {
        let destination = root.join(format!("checked-in-{harness}"));
        json_output(
            root,
            &[
                "skill",
                "install",
                harness,
                "--to",
                destination.to_str().expect("integration destination"),
            ],
        );
        assert_eq!(
            fs::read(destination.join(rendered_skill)).expect("rendered harness skill"),
            fs::read(source.join(checked_in_skill)).expect("checked-in harness skill"),
            "checked-in {harness} skill drifted from its template"
        );
    }
}

#[test]
fn codex_skill_check_and_force_install_manage_ui_metadata() {
    let repository = repository();
    let root = repository.path();
    let destination = root.join("codex-bundle");
    let destination_text = destination.to_str().expect("Codex destination");

    let installed = json_output(
        root,
        &["skill", "install", "codex", "--to", destination_text],
    );
    assert_eq!(installed["files"], 12);

    let metadata = destination.join("agents/openai.yaml");
    fs::remove_file(&metadata).expect("remove Codex UI metadata");
    let missing = json_output_with_exit_code(
        root,
        &["skill", "check", "codex", "--to", destination_text],
        1,
    );
    assert_eq!(missing["status"], "conflict");

    let restored = json_output(
        root,
        &[
            "skill",
            "install",
            "codex",
            "--to",
            destination_text,
            "--force",
        ],
    );
    assert_eq!(restored["status"], "replaced");
    assert_eq!(
        fs::read(&metadata).expect("restored Codex UI metadata"),
        include_bytes!("../skills/zdev/agents/openai.yaml")
    );

    fs::write(&metadata, "locally changed\n").expect("change Codex UI metadata");
    let changed = json_output_with_exit_code(
        root,
        &["skill", "check", "codex", "--to", destination_text],
        1,
    );
    assert_eq!(changed["status"], "conflict");

    let restored = json_output(
        root,
        &[
            "skill",
            "install",
            "codex",
            "--to",
            destination_text,
            "--force",
        ],
    );
    assert_eq!(restored["status"], "replaced");
    assert_eq!(
        fs::read(metadata).expect("restored Codex UI metadata"),
        include_bytes!("../skills/zdev/agents/openai.yaml")
    );
}

#[test]
fn harness_skill_templates_are_thin_wrappers_around_the_shared_contract() {
    for (harness, template) in [
        ("codex", include_str!("../templates/zdev/codex-skill.md")),
        ("claude", include_str!("../templates/zdev/claude-skill.md")),
        (
            "opencode",
            include_str!("../templates/zdev/opencode-skill.md"),
        ),
        ("pi", include_str!("../templates/zdev/pi-skill.md")),
        ("omp", include_str!("../templates/zdev/omp-skill.md")),
    ] {
        assert_eq!(
            template.matches("{{shared_contract}}").count(),
            1,
            "{harness} must render the shared contract exactly once"
        );
    }
}

#[test]
fn opencode_skill_uses_native_shared_root_assets_without_replacing_user_config() {
    let repository = repository();
    let root = repository.path();
    let destination = root.join("opencode-config");
    fs::create_dir_all(&destination).expect("OpenCode config");
    fs::write(
        destination.join("opencode.json"),
        "{\"theme\":\"system\"}\n",
    )
    .expect("unrelated OpenCode config");

    let installed = json_output(
        root,
        &[
            "skill",
            "install",
            "opencode",
            "--to",
            destination.to_str().expect("destination"),
        ],
    );
    assert_eq!(installed["status"], "created");
    assert_eq!(installed["files"], 15);
    assert_eq!(
        fs::read_to_string(destination.join("opencode.json")).expect("preserved config"),
        "{\"theme\":\"system\"}\n"
    );
    for path in [
        "skills/zdev-opencode/SKILL.md",
        "agents/zdev-implementer.md",
        "agents/zdev-verifier.md",
        "command/zdev-task.md",
        "command/zdev-audit.md",
    ] {
        assert!(destination.join(path).is_file(), "missing {path}");
    }
    let unchanged = json_output(
        root,
        &[
            "skill",
            "install",
            "opencode",
            "--to",
            destination.to_str().expect("destination"),
        ],
    );
    assert_eq!(unchanged["status"], "unchanged");
    fs::write(
        destination.join("agents/zdev-verifier.md"),
        "locally changed\n",
    )
    .expect("changed managed file");
    let refused = run_zdev(
        root,
        &[
            "skill",
            "install",
            "opencode",
            "--to",
            destination.to_str().expect("destination"),
        ],
    );
    assert!(!refused.status.success());
    json_output(
        root,
        &[
            "skill",
            "install",
            "opencode",
            "--to",
            destination.to_str().expect("destination"),
            "--force",
        ],
    );
    assert_eq!(
        fs::read_to_string(destination.join("opencode.json")).expect("preserved config"),
        "{\"theme\":\"system\"}\n"
    );
    assert_eq!(
        json_output(
            root,
            &[
                "skill",
                "check",
                "opencode",
                "--to",
                destination.to_str().expect("destination"),
            ],
        )["status"],
        "ok"
    );
}

#[test]
fn opencode_project_install_inlines_guidance_at_native_destination() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    fs::write(root.join("AGENTS.md"), "Run `just opencode-ci`.\n").expect("guidance");

    let installed = json_output(
        root,
        &["skill", "install", "opencode", "--scope", "project"],
    );
    assert_eq!(
        installed["path"],
        json!(
            fs::canonicalize(root)
                .expect("canonical root")
                .join(".opencode")
        )
    );
    assert_eq!(installed["guidance"]["source"], "AGENTS.md");
    let skill = fs::read_to_string(root.join(".opencode/skills/zdev-opencode/SKILL.md"))
        .expect("project OpenCode skill");
    assert!(skill.contains("Run `just opencode-ci`."));
    assert_eq!(
        json_output(root, &["skill", "check", "opencode", "--scope", "project"])["status"],
        "ok"
    );
}

#[test]
fn pi_skill_uses_native_shared_root_assets_without_replacing_user_config() {
    let repository = repository();
    let root = repository.path();
    let destination = root.join("pi-agent");
    fs::create_dir_all(&destination).expect("Pi agent directory");
    fs::write(destination.join("settings.json"), "{\"theme\":\"dark\"}\n")
        .expect("unrelated Pi config");

    let installed = json_output(
        root,
        &[
            "skill",
            "install",
            "pi",
            "--to",
            destination.to_str().expect("destination"),
        ],
    );
    assert_eq!(installed["status"], "created");
    assert_eq!(installed["files"], 14);
    assert_eq!(
        file_inventory(&destination),
        [
            "extensions/zdev-subagent.ts",
            "prompts/zdev-audit.md",
            "prompts/zdev-task.md",
            "settings.json",
            "skills/zdev-pi/SKILL.md",
            "skills/zdev-pi/references/discuss.md",
            "skills/zdev-pi/references/implement.md",
            "skills/zdev-pi/references/improve.md",
            "skills/zdev-pi/references/investigate.md",
            "skills/zdev-pi/references/recovery.md",
            "skills/zdev-pi/references/setup.md",
            "skills/zdev-pi/references/shape-work.md",
            "skills/zdev-pi/references/task-format.md",
            "skills/zdev-pi/references/to-tasks.md",
            "skills/zdev-pi/references/verify.md",
        ]
    );
    assert_eq!(
        fs::read_to_string(destination.join("settings.json")).expect("preserved config"),
        "{\"theme\":\"dark\"}\n"
    );
    for path in [
        "skills/zdev-pi/SKILL.md",
        "prompts/zdev-task.md",
        "prompts/zdev-audit.md",
        "extensions/zdev-subagent.ts",
    ] {
        assert!(destination.join(path).is_file(), "missing {path}");
    }

    let extension =
        fs::read_to_string(destination.join("extensions/zdev-subagent.ts")).expect("Pi extension");
    for expected in [
        "Type.Literal(\"implementer\")",
        "Type.Literal(\"verifier\")",
        "read,bash,edit,write,grep,find,ls",
        "read,bash,grep,find,ls",
        "--no-session",
        "--no-extensions",
        "--append-system-prompt",
        "ctx.model.provider",
        "pi.exec(\"pi\"",
    ] {
        assert!(extension.contains(expected), "missing {expected}");
    }
    fs::write(
        destination.join("extensions/zdev-subagent.ts"),
        "locally changed\n",
    )
    .expect("changed managed file");
    let refused = run_zdev(
        root,
        &[
            "skill",
            "install",
            "pi",
            "--to",
            destination.to_str().expect("destination"),
        ],
    );
    assert!(!refused.status.success());
    json_output(
        root,
        &[
            "skill",
            "install",
            "pi",
            "--to",
            destination.to_str().expect("destination"),
            "--force",
        ],
    );
    assert_eq!(
        fs::read_to_string(destination.join("settings.json")).expect("preserved config"),
        "{\"theme\":\"dark\"}\n"
    );
    assert_eq!(
        json_output(
            root,
            &[
                "skill",
                "check",
                "pi",
                "--to",
                destination.to_str().expect("destination"),
            ],
        )["status"],
        "ok"
    );
}

#[test]
fn pi_project_install_inlines_guidance_at_native_destination() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    fs::write(root.join("AGENTS.md"), "Run `just pi-ci`.\n").expect("guidance");

    let installed = json_output(root, &["skill", "install", "pi", "--scope", "project"]);
    assert_eq!(
        installed["path"],
        json!(fs::canonicalize(root).expect("canonical root").join(".pi"))
    );
    assert_eq!(installed["guidance"]["source"], "AGENTS.md");
    let skill =
        fs::read_to_string(root.join(".pi/skills/zdev-pi/SKILL.md")).expect("project Pi skill");
    assert!(skill.contains("Run `just pi-ci`."));
    assert_eq!(
        json_output(root, &["skill", "check", "pi", "--scope", "project"])["status"],
        "ok"
    );
}

#[test]
fn omp_skill_uses_native_shared_root_assets_without_replacing_user_config() {
    let repository = repository();
    let root = repository.path();
    let destination = root.join("omp-agent");
    fs::create_dir_all(&destination).expect("Oh My Pi agent directory");
    fs::write(destination.join("settings.json"), "{\"theme\":\"dark\"}\n")
        .expect("unrelated Oh My Pi config");

    let installed = json_output(
        root,
        &[
            "skill",
            "install",
            "omp",
            "--to",
            destination.to_str().expect("destination"),
        ],
    );
    assert_eq!(installed["harness"], "omp");
    assert_eq!(installed["status"], "created");
    assert_eq!(installed["files"], 13);
    assert_eq!(
        file_inventory(&destination),
        [
            "agents/zdev-implementer.md",
            "agents/zdev-verifier.md",
            "settings.json",
            "skills/zdev/SKILL.md",
            "skills/zdev/references/discuss.md",
            "skills/zdev/references/implement.md",
            "skills/zdev/references/improve.md",
            "skills/zdev/references/investigate.md",
            "skills/zdev/references/recovery.md",
            "skills/zdev/references/setup.md",
            "skills/zdev/references/shape-work.md",
            "skills/zdev/references/task-format.md",
            "skills/zdev/references/to-tasks.md",
            "skills/zdev/references/verify.md",
        ]
    );
    assert_eq!(
        fs::read_to_string(destination.join("settings.json")).expect("preserved config"),
        "{\"theme\":\"dark\"}\n"
    );

    for pi_only_asset in [
        "extensions/zdev-subagent.ts",
        "prompts/zdev-task.md",
        "prompts/zdev-audit.md",
    ] {
        assert!(!destination.join(pi_only_asset).exists());
    }

    let implementer = fs::read_to_string(destination.join("agents/zdev-implementer.md"))
        .expect("Oh My Pi implementer");
    for expected in [
        "name: zdev-implementer",
        "tools: read, grep, bash, edit, write",
        "blocking: true",
    ] {
        assert!(implementer.contains(expected), "missing {expected}");
    }
    assert!(!implementer.contains("tools: task"));

    let verifier =
        fs::read_to_string(destination.join("agents/zdev-verifier.md")).expect("Oh My Pi verifier");
    for expected in [
        "name: zdev-verifier",
        "tools: read, grep, bash",
        "blocking: true",
    ] {
        assert!(verifier.contains(expected), "missing {expected}");
    }
    assert!(!verifier.contains("edit, write"));
    assert!(!verifier.contains("tools: task"));

    assert_eq!(
        json_output(
            root,
            &[
                "skill",
                "check",
                "omp",
                "--to",
                destination.to_str().expect("destination"),
            ],
        )["status"],
        "ok"
    );
    fs::write(
        destination.join("agents/zdev-verifier.md"),
        "locally changed\n",
    )
    .expect("changed managed file");
    let conflict = json_output_with_exit_code(
        root,
        &[
            "skill",
            "check",
            "omp",
            "--to",
            destination.to_str().expect("destination"),
        ],
        1,
    );
    assert_eq!(conflict["status"], "conflict");
    let refused = run_zdev(
        root,
        &[
            "skill",
            "install",
            "omp",
            "--to",
            destination.to_str().expect("destination"),
        ],
    );
    assert!(!refused.status.success());
    assert!(String::from_utf8_lossy(&refused.stderr).contains("--force"));
    let replaced = json_output(
        root,
        &[
            "skill",
            "install",
            "omp",
            "--to",
            destination.to_str().expect("destination"),
            "--force",
        ],
    );
    assert_eq!(replaced["status"], "replaced");
    assert_eq!(
        fs::read_to_string(destination.join("settings.json")).expect("preserved config"),
        "{\"theme\":\"dark\"}\n"
    );
    assert_eq!(
        json_output(
            root,
            &[
                "skill",
                "check",
                "omp",
                "--to",
                destination.to_str().expect("destination"),
            ],
        )["status"],
        "ok"
    );
}

#[test]
fn omp_project_install_inlines_guidance_at_native_destination() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    fs::write(root.join("AGENTS.md"), "Run `just omp-ci`.\n").expect("guidance");

    let installed = json_output(root, &["skill", "install", "omp", "--scope", "project"]);
    assert_eq!(
        installed["path"],
        json!(fs::canonicalize(root).expect("canonical root").join(".omp"))
    );
    assert_eq!(installed["guidance"]["source"], "AGENTS.md");
    let skill =
        fs::read_to_string(root.join(".omp/skills/zdev/SKILL.md")).expect("project OMP skill");
    assert!(skill.contains("Run `just omp-ci`."));
    assert_eq!(
        json_output(root, &["skill", "check", "omp", "--scope", "project"])["status"],
        "ok"
    );
}

#[test]
fn omp_relocated_user_install_and_check_warn_without_failing() {
    let repository = repository();
    let root = repository.path();
    let relocated = root.join("relocated-omp-agent");
    let environment = [("PI_CODING_AGENT_DIR", relocated.as_path())];

    let installed = json_output_with_env(root, &["skill", "install", "omp"], &environment);
    assert_eq!(installed["status"], "created");
    assert_eq!(installed["warnings"].as_array().map(Vec::len), Some(1));
    let warning = installed["warnings"][0].as_str().expect("warning text");
    assert!(warning.contains("Oh My Pi 17.2.15"));
    assert!(warning.contains("may not discover its user task agents"));
    assert!(warning.contains("Unset PI_CODING_AGENT_DIR"));
    assert!(warning.contains("--scope project"));

    let checked = json_output_with_env(root, &["skill", "check", "omp"], &environment);
    assert_eq!(checked["status"], "ok");
    assert_eq!(checked["warnings"], installed["warnings"]);

    for arguments in [["skill", "install", "omp"], ["skill", "check", "omp"]] {
        let output = run_zdev_with_env(root, &arguments, &environment);
        assert!(output.status.success());
        let text = String::from_utf8_lossy(&output.stdout);
        assert!(text.contains("Warning: Oh My Pi 17.2.15"));
        assert!(text.contains("Unset PI_CODING_AGENT_DIR"));
        assert!(text.contains("--scope project"));
    }
}

#[test]
fn omp_destination_warning_exempts_default_project_and_explicit_destinations() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    let fallback_home = root.join("fallback-home");
    let relocated = root.join("relocated-omp-agent");
    let explicit = root.join("explicit-omp-agent");
    let default_environment = [
        ("PI_CODING_AGENT_DIR", Path::new("")),
        ("HOME", fallback_home.as_path()),
    ];
    let relocated_environment = [("PI_CODING_AGENT_DIR", relocated.as_path())];

    for command in ["install", "check"] {
        let pi = json_output_with_env(root, &["skill", command, "pi"], &relocated_environment);
        assert_eq!(pi["warnings"], json!([]));

        let default = json_output_with_env(root, &["skill", command, "omp"], &default_environment);
        assert_eq!(default["warnings"], json!([]));

        let project = json_output_with_env(
            root,
            &["skill", command, "omp", "--scope", "project"],
            &relocated_environment,
        );
        assert_eq!(project["warnings"], json!([]));

        let explicit = json_output_with_env(
            root,
            &[
                "skill",
                command,
                "omp",
                "--to",
                explicit.to_str().expect("explicit destination"),
            ],
            &relocated_environment,
        );
        assert_eq!(explicit["warnings"], json!([]));
    }

    for (arguments, environment) in [
        (
            vec!["skill", "install", "pi"],
            relocated_environment.as_slice(),
        ),
        (
            vec!["skill", "check", "pi"],
            relocated_environment.as_slice(),
        ),
        (
            vec!["skill", "install", "omp"],
            default_environment.as_slice(),
        ),
        (
            vec!["skill", "check", "omp"],
            default_environment.as_slice(),
        ),
        (
            vec!["skill", "install", "omp", "--scope", "project"],
            relocated_environment.as_slice(),
        ),
        (
            vec!["skill", "check", "omp", "--scope", "project"],
            relocated_environment.as_slice(),
        ),
        (
            vec![
                "skill",
                "install",
                "omp",
                "--to",
                explicit.to_str().expect("explicit destination"),
            ],
            relocated_environment.as_slice(),
        ),
        (
            vec![
                "skill",
                "check",
                "omp",
                "--to",
                explicit.to_str().expect("explicit destination"),
            ],
            relocated_environment.as_slice(),
        ),
    ] {
        let output = run_zdev_with_env(root, &arguments, environment);
        assert!(output.status.success());
        assert!(!String::from_utf8_lossy(&output.stdout).contains("Warning:"));
    }
}

#[test]
fn harness_destinations_respect_scope_and_config_home_variables() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    let codex_home = root.join("codex-home");
    let claude_home = root.join("claude-home");
    let xdg_home = root.join("xdg-home");
    let pi_home = root.join("pi-home");
    let omp_home = root.join("omp-home");
    let fallback_home = root.join("fallback-home");

    let codex = json_output_with_env(
        root,
        &["skill", "install", "codex"],
        &[("CODEX_HOME", codex_home.as_path())],
    );
    assert_eq!(codex["path"], json!(codex_home.join("skills/zdev")));
    assert_eq!(codex["scope"], "user");

    let claude = json_output_with_env(
        root,
        &["skill", "install", "claude"],
        &[("CLAUDE_CONFIG_DIR", claude_home.as_path())],
    );
    assert_eq!(claude["path"], json!(claude_home.join("skills/zdev")));
    assert_eq!(claude["scope"], "user");

    let opencode = json_output_with_env(
        root,
        &["skill", "install", "opencode"],
        &[("XDG_CONFIG_HOME", xdg_home.as_path())],
    );
    assert_eq!(opencode["path"], json!(xdg_home.join("opencode")));
    assert_eq!(opencode["scope"], "user");

    let pi = json_output_with_env(
        root,
        &["skill", "install", "pi"],
        &[("PI_CODING_AGENT_DIR", pi_home.as_path())],
    );
    assert_eq!(pi["path"], json!(pi_home));
    assert_eq!(pi["scope"], "user");

    let omp = json_output_with_env(
        root,
        &["skill", "install", "omp"],
        &[("PI_CODING_AGENT_DIR", omp_home.as_path())],
    );
    assert_eq!(omp["path"], json!(omp_home));
    assert_eq!(omp["scope"], "user");

    let omp_fallback = json_output_with_env(
        root,
        &["skill", "install", "omp"],
        &[
            ("PI_CODING_AGENT_DIR", Path::new("")),
            ("HOME", fallback_home.as_path()),
        ],
    );
    assert_eq!(
        omp_fallback["path"],
        json!(fallback_home.join(".omp/agent"))
    );
    assert_eq!(omp_fallback["scope"], "user");

    let codex_project = json_output(root, &["skill", "install", "codex", "--scope", "project"]);
    let canonical_root = fs::canonicalize(root).expect("canonical repository root");
    assert_eq!(
        codex_project["path"],
        json!(canonical_root.join(".codex/skills/zdev"))
    );
    assert_eq!(codex_project["scope"], "project");

    let claude_project = json_output(root, &["skill", "install", "claude", "--scope", "project"]);
    assert_eq!(
        claude_project["path"],
        json!(canonical_root.join(".claude/skills/zdev"))
    );
    assert_eq!(claude_project["scope"], "project");

    let opencode_project = json_output(
        root,
        &["skill", "install", "opencode", "--scope", "project"],
    );
    assert_eq!(
        opencode_project["path"],
        json!(canonical_root.join(".opencode"))
    );
    assert_eq!(opencode_project["scope"], "project");

    let pi_project = json_output(root, &["skill", "install", "pi", "--scope", "project"]);
    assert_eq!(pi_project["path"], json!(canonical_root.join(".pi")));
    assert_eq!(pi_project["scope"], "project");

    let omp_project = json_output(root, &["skill", "install", "omp", "--scope", "project"]);
    assert_eq!(omp_project["path"], json!(canonical_root.join(".omp")));
    assert_eq!(omp_project["scope"], "project");
}

#[test]
fn project_integrations_share_and_preserve_existing_agents_guidance() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    let guidance = b"# Repository instructions\n\nRun `cargo test`.\n";
    fs::write(root.join("AGENTS.md"), guidance).expect("repository guidance");

    for harness in ["codex", "claude"] {
        let installed = json_output(root, &["skill", "install", harness, "--scope", "project"]);
        assert_eq!(installed["guidance"]["status"], "ok");
        assert_eq!(installed["guidance"]["source"], "AGENTS.md");
        assert_eq!(installed["bundle"]["status"], "created");

        let destination = root.join(format!(".{harness}/skills/zdev/SKILL.md"));
        fs::write(&destination, "force replacement\n").expect("change installed integration");
        let replaced = json_output(
            root,
            &["skill", "install", harness, "--scope", "project", "--force"],
        );
        assert_eq!(replaced["bundle"]["status"], "replaced");
        assert_eq!(
            fs::read(root.join("AGENTS.md")).expect("guidance"),
            guidance
        );

        let checked = json_output(root, &["skill", "check", harness, "--scope", "project"]);
        assert_eq!(checked["bundle"]["status"], "ok");
        assert_eq!(checked["guidance"]["status"], "ok");
    }
}

#[test]
fn project_integration_install_requires_an_initialized_record() {
    let repository = repository();
    let root = repository.path();

    let output = run_zdev(root, &["skill", "install", "codex", "--scope", "project"]);

    assert!(!output.status.success());
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("zdev init --record <personal|project|pull-request>"));
    assert!(!root.join(".codex/skills/zdev").exists());
    assert!(!root.join(".zdev").exists());
}

#[test]
fn project_skill_install_always_inlines_guidance_while_user_install_does_not() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    let guidance = "# Repository instructions\n\nRun `just ci-project-only`.\n";
    fs::write(root.join("AGENTS.md"), guidance).expect("repository guidance");

    for (harness, skill_path) in [
        ("codex", ".codex/skills/zdev/SKILL.md"),
        ("claude", ".claude/skills/zdev/skills/zdev/SKILL.md"),
        ("opencode", ".opencode/skills/zdev-opencode/SKILL.md"),
        ("pi", ".pi/skills/zdev-pi/SKILL.md"),
        ("omp", ".omp/skills/zdev/SKILL.md"),
    ] {
        json_output(root, &["skill", "install", harness, "--scope", "project"]);
        let rendered = fs::read_to_string(root.join(skill_path)).expect("project skill");
        assert!(rendered.contains("## Rendered repository guidance"));
        assert!(rendered.contains("Source: `AGENTS.md`"));
        assert!(rendered.contains(guidance.trim_end()));
        assert!(!rendered.contains("## Repository guidance discovery"));

        let user_destination = root.join(format!("user-{harness}"));
        json_output(
            root,
            &[
                "skill",
                "install",
                harness,
                "--to",
                user_destination.to_str().expect("user destination"),
            ],
        );
        let user_skill = match harness {
            "codex" => user_destination.join("SKILL.md"),
            "claude" => user_destination.join("skills/zdev/SKILL.md"),
            "opencode" => user_destination.join("skills/zdev-opencode/SKILL.md"),
            "pi" => user_destination.join("skills/zdev-pi/SKILL.md"),
            "omp" => user_destination.join("skills/zdev/SKILL.md"),
            _ => unreachable!(),
        };
        let rendered = fs::read_to_string(user_skill).expect("user skill");
        assert!(rendered.contains("## Repository guidance discovery"));
        assert!(!rendered.contains("just ci-project-only"));
    }
}

#[test]
fn auto_guidance_scaffolds_one_shared_fallback_and_preserves_authored_bytes() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    let expected = "# Repository guidance for zdev\n\n## Understand and navigate\n\n## Build and compile\n\n## Run locally\n\n## Test and validate\n\n## Format and lint\n\n## Generated files and migrations\n\n## Safety, secrets, and unavailable services\n";

    let codex = json_output(root, &["skill", "install", "codex", "--scope", "project"]);
    assert_eq!(codex["guidance"]["status"], "created");
    assert_eq!(codex["guidance"]["source"], ".zdev/guidance.md");
    assert_eq!(
        fs::read_to_string(root.join(".zdev/guidance.md")).expect("fallback guidance"),
        expected
    );

    let authored = format!("{expected}\nUse `cargo test --locked`.\n");
    fs::write(root.join(".zdev/guidance.md"), &authored).expect("author guidance");
    let stale =
        json_output_with_exit_code(root, &["skill", "check", "codex", "--scope", "project"], 1);
    assert_eq!(stale["bundle"]["status"], "conflict");
    assert_eq!(stale["guidance"]["status"], "ok");
    json_output(
        root,
        &["skill", "install", "codex", "--scope", "project", "--force"],
    );
    let rendered =
        fs::read_to_string(root.join(".codex/skills/zdev/SKILL.md")).expect("rendered skill");
    assert!(rendered.contains("Source: `.zdev/guidance.md`"));
    assert!(rendered.contains("Use `cargo test --locked`."));
    let claude = json_output(root, &["skill", "install", "claude", "--scope", "project"]);
    assert_eq!(claude["guidance"]["status"], "ok");
    assert_eq!(
        fs::read_to_string(root.join(".zdev/guidance.md")).expect("preserved guidance"),
        authored
    );
    let claude_rendered = fs::read_to_string(root.join(".claude/skills/zdev/skills/zdev/SKILL.md"))
        .expect("rendered Claude skill");
    assert!(claude_rendered.contains("Source: `.zdev/guidance.md`"));
    assert!(claude_rendered.contains("Use `cargo test --locked`."));
}

#[test]
fn explicit_guidance_modes_report_markers_missing_files_and_safe_custom_paths() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    fs::write(
        root.join("AGENTS.md"),
        "# Agents\n\n<!-- zdev:guidance:start -->\nRun cargo test.\n<!-- zdev:guidance:end -->\n",
    )
    .expect("marked agents guidance");
    let marked = json_output(
        root,
        &[
            "skill",
            "install",
            "codex",
            "--scope",
            "project",
            "--guidance",
            "agents",
        ],
    );
    assert_eq!(marked["guidance"]["marked_block_current"], true);

    let zdev = json_output(
        root,
        &[
            "skill",
            "install",
            "claude",
            "--scope",
            "project",
            "--guidance",
            "zdev",
        ],
    );
    assert_eq!(zdev["guidance"]["source"], ".zdev/guidance.md");
    assert_eq!(zdev["guidance"]["status"], "created");

    fs::create_dir(root.join("docs")).expect("docs directory");
    let custom_path = root.join("docs/build.md");
    let custom_bytes = b"# Build\n\nRun cargo build.\n";
    fs::write(&custom_path, custom_bytes).expect("custom guidance");
    let custom_bundle = root.join("custom-bundle");
    let custom = json_output(
        root,
        &[
            "skill",
            "install",
            "codex",
            "--scope",
            "project",
            "--to",
            custom_bundle.to_str().expect("bundle path"),
            "--guidance",
            "docs/build.md",
        ],
    );
    assert_eq!(custom["guidance"]["mode"], "custom");
    assert_eq!(custom["guidance"]["source"], "docs/build.md");
    fs::write(custom_bundle.join("SKILL.md"), "replace\n").expect("change bundle");
    json_output(
        root,
        &[
            "skill",
            "install",
            "codex",
            "--scope",
            "project",
            "--to",
            custom_bundle.to_str().expect("bundle path"),
            "--guidance",
            "docs/build.md",
            "--force",
        ],
    );
    assert_eq!(
        fs::read(&custom_path).expect("custom guidance"),
        custom_bytes
    );
    for harness in ["codex", "claude"] {
        let stale =
            json_output_with_exit_code(root, &["skill", "check", harness, "--scope", "project"], 1);
        assert_eq!(stale["bundle"]["status"], "conflict");
        assert_eq!(stale["guidance"]["source"], "docs/build.md");
        json_output(
            root,
            &["skill", "install", harness, "--scope", "project", "--force"],
        );
        let checked = json_output(root, &["skill", "check", harness, "--scope", "project"]);
        assert_eq!(checked["guidance"]["source"], "docs/build.md");
        assert_eq!(checked["guidance"]["status"], "ok");
    }
    let config = fs::read_to_string(root.join(".zdev/config.toml")).expect("project config");
    assert!(config.contains("guidance = \"docs/build.md\""));

    for unsafe_path in ["../outside.md", "/tmp/outside.md", "docs/build.txt"] {
        let output = run_zdev(
            root,
            &[
                "skill",
                "check",
                "codex",
                "--scope",
                "project",
                "--guidance",
                unsafe_path,
            ],
        );
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("repository-relative .md"));
    }
}

#[test]
fn agents_guidance_without_markers_fails_before_bundle_publication() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    fs::write(root.join("AGENTS.md"), "# Unmarked instructions\n").expect("agents guidance");

    let output = run_zdev(
        root,
        &[
            "skill",
            "install",
            "claude",
            "--scope",
            "project",
            "--guidance",
            "agents",
        ],
    );
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("marker block"));
    assert!(!root.join(".claude/skills/zdev").exists());
    assert!(root.join(".zdev/config.toml").is_file());
}

#[test]
fn recorded_agents_guidance_rechecks_markers_on_default_check() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    fs::write(
        root.join("AGENTS.md"),
        "<!-- zdev:guidance:start -->\nRun cargo test.\n<!-- zdev:guidance:end -->\n",
    )
    .expect("marked agents guidance");
    json_output(
        root,
        &[
            "skill",
            "install",
            "codex",
            "--scope",
            "project",
            "--guidance",
            "agents",
        ],
    );
    let config = fs::read_to_string(root.join(".zdev/config.toml")).expect("project config");
    assert!(config.contains("guidance = \"agents\""));

    fs::write(root.join("AGENTS.md"), "# Markers removed\n").expect("change agents guidance");
    let checked =
        json_output_with_exit_code(root, &["skill", "check", "codex", "--scope", "project"], 1);
    assert_eq!(checked["status"], "conflict");
    assert_eq!(checked["bundle"]["status"], "conflict");
    assert_eq!(checked["guidance"]["mode"], "agents");
    assert_eq!(checked["guidance"]["status"], "unmarked");
    assert_eq!(checked["guidance"]["marked_block_current"], false);
}

#[test]
fn project_check_reports_missing_or_unmarked_guidance_separately() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    json_output(root, &["skill", "install", "codex", "--scope", "project"]);

    let missing = json_output_with_exit_code(
        root,
        &[
            "skill",
            "check",
            "codex",
            "--scope",
            "project",
            "--guidance",
            "agents",
        ],
        1,
    );
    assert_eq!(missing["bundle"]["status"], "conflict");
    assert_eq!(missing["guidance"]["status"], "missing");
    assert_eq!(missing["status"], "conflict");

    fs::write(root.join("AGENTS.md"), "# Unmarked instructions\n").expect("agents guidance");
    let unmarked = json_output_with_exit_code(
        root,
        &[
            "skill",
            "check",
            "codex",
            "--scope",
            "project",
            "--guidance",
            "agents",
        ],
        1,
    );
    assert_eq!(unmarked["guidance"]["status"], "unmarked");
    assert_eq!(unmarked["guidance"]["marked_block_current"], false);
}

#[test]
fn project_check_says_current_bundle_is_not_ready_when_guidance_is_missing() {
    let repository = repository();
    let root = repository.path();
    let destination = root.join(".codex/skills/zdev");
    json_output(
        root,
        &[
            "skill",
            "install",
            "codex",
            "--to",
            destination.to_str().expect("destination"),
        ],
    );

    let checked = run_zdev(
        root,
        &[
            "skill",
            "check",
            "codex",
            "--scope",
            "project",
            "--guidance",
            "agents",
        ],
    );

    assert_eq!(checked.status.code(), Some(1));
    let text = String::from_utf8_lossy(&checked.stdout);
    assert!(text.contains("integration files are current"));
    assert!(text.contains("selected guidance is not ready"));
    assert!(!text.contains("integration is ready"));
    assert!(text.contains("guidance: AGENTS.md (missing)"));
    assert!(text.contains("zdev skill install codex --scope project --guidance agents --force"));
}

#[cfg(unix)]
#[test]
fn custom_guidance_rejects_symlinks() {
    use std::os::unix::fs::symlink;

    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    fs::write(root.join("real.md"), "# Real\n").expect("real guidance");
    symlink(root.join("real.md"), root.join("linked.md")).expect("guidance symlink");
    let output = run_zdev(
        root,
        &[
            "skill",
            "install",
            "codex",
            "--scope",
            "project",
            "--guidance",
            "linked.md",
        ],
    );
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("not a symlink"));
}

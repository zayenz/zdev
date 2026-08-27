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

fn assert_pretty_json(output: &Output, expected: Value) {
    assert!(
        output.status.success(),
        "zdev failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut bytes = serde_json::to_vec_pretty(&expected).expect("expected JSON");
    bytes.push(b'\n');
    assert_eq!(output.stdout, bytes);
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

fn git_stdout(root: &Path, arguments: &[&str]) -> String {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(root)
        .output()
        .expect("run git");
    assert!(output.status.success());
    String::from_utf8(output.stdout).expect("Git UTF-8")
}

fn git_path(root: &Path, name: &str) -> std::path::PathBuf {
    let path = std::path::PathBuf::from(git(root, &["rev-parse", "--git-path", name]));
    if path.is_absolute() {
        path
    } else {
        root.join(path)
    }
}

fn reported_path(root: &Path, value: &Value) -> std::path::PathBuf {
    let path = std::path::PathBuf::from(value.as_str().expect("reported path"));
    if path.is_absolute() {
        path
    } else {
        root.join(path)
    }
}

fn executable_on_path(name: &str) -> std::path::PathBuf {
    std::env::split_paths(&std::env::var_os("PATH").expect("PATH"))
        .map(|directory| directory.join(name))
        .find(|path| path.is_file())
        .unwrap_or_else(|| panic!("{name} on PATH"))
}

fn repository() -> TempDir {
    let directory = tempfile::tempdir().expect("temporary repository");
    git(
        directory.path(),
        &["init", "-q", "--initial-branch", "main"],
    );
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

fn assert_task_workflows_locator(destination: &Path, skill_root: &str, coordinator_paths: &[&str]) {
    let contract = destination
        .join(skill_root)
        .join("references/task-workflows.md");
    assert!(contract.is_file(), "missing {}", contract.display());
    let contract = fs::canonicalize(contract).expect("canonical task-workflows contract path");
    let encoded = serde_json::to_string(contract.to_str().expect("UTF-8 contract path"))
        .expect("contract path JSON");
    for coordinator in coordinator_paths {
        let rendered = fs::read_to_string(destination.join(coordinator))
            .unwrap_or_else(|_| panic!("read coordinator {coordinator}"));
        assert!(
            rendered.contains(&encoded),
            "{coordinator} does not contain exact contract locator {encoded}"
        );
    }
}

fn normalize_task_workflows_locator(content: Vec<u8>) -> Vec<u8> {
    let content = String::from_utf8(content).expect("UTF-8 integration artifact");
    let mut normalized = String::with_capacity(content.len());
    let mut locator_value_follows = false;
    for line in content.split_inclusive('\n') {
        if line.starts_with("const taskWorkflowContractPath = ") {
            normalized.push_str("const taskWorkflowContractPath = \"<task-workflows-contract>\"\n");
        } else if locator_value_follows {
            let (_, tail) = line
                .split_once(". Decode")
                .expect("rendered contract locator value");
            normalized.push_str("\"<task-workflows-contract>\". Decode");
            normalized.push_str(tail);
            locator_value_follows = false;
        } else {
            normalized.push_str(line);
            locator_value_follows = line.contains("exact installed task-workflows contract path");
        }
    }
    normalized.into_bytes()
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

fn create_slice(root: &Path, area: &str, key: &str, title: &str) {
    json_output(
        root,
        &[
            "slice",
            "create",
            area,
            key,
            "--title",
            title,
            "--objective",
            "Exercise task slice membership.",
            "--boundary",
            "Keep the slice focused.",
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
fn goal_projects_the_sliced_ready_task_exactly_and_deterministically() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "checkout",
            "--title",
            "Checkout reliability",
            "--objective",
            "Make checkout failures safe and understandable.",
        ],
    );
    json_output(
        root,
        &[
            "slice",
            "create",
            "checkout",
            "payments",
            "--title",
            "Payment submission",
            "--objective",
            "Make payment submission safe to retry.",
            "--boundary",
            "Do not change provider selection.",
        ],
    );
    commit_all(root, "record checkout area");
    git(root, &["switch", "--detach", "-q", "HEAD"]);
    let tasks = root.join(".zdev/checkout/tasks");
    fs::write(
        tasks.join("001-shipped-foundation.md"),
        "+++\nschema_version = 1\nid = \"checkout-001\"\nkey = \"shipped-foundation\"\narea = \"checkout\"\nstatus = \"done\"\nblocked_by = []\n+++\n# Ship the foundation\n\n## Outcome\n\nThe foundation is shipped.\n\n## Done when\n\n- [x] The foundation is present.\n\n## Validation\n\n- Run the foundation check.\n\n## Result\n\nShipped and checked.\n",
    )
    .expect("done goal task");
    fs::write(
        tasks.join("002-reject-duplicate-payment.md"),
        "+++\nschema_version = 1\nid = \"checkout-002\"\nkey = \"reject-duplicate-payment\"\narea = \"checkout\"\nstatus = \"open\"\nslice = \"payments\"\nblocked_by = []\n+++\n# Reject duplicate payment submission\n\n## Outcome\n\nA repeated submission returns the original payment result without charging again.\n\n## Context\n\nThe provider can retry after losing our first response.\n\n## Boundaries\n\n- Keep the public response schema unchanged.\n\n## Done when\n\n- [ ] Duplicate provider calls are prevented.\n- [ ] The original result is returned.\n\n## Validation\n\n- Run the focused payment integration test.\n",
    )
    .expect("ready goal task");
    fs::write(
        tasks.join("003-follow-up.md"),
        "+++\nschema_version = 1\nid = \"checkout-003\"\nkey = \"follow-up\"\narea = \"checkout\"\nstatus = \"open\"\nblocked_by = [\"checkout-002\"]\n+++\n# Follow up\n\n## Outcome\n\nThe follow-up is complete.\n\n## Done when\n\n- [ ] The follow-up is complete.\n\n## Validation\n\n- Run the follow-up check.\n",
    )
    .expect("blocked goal task");

    let first = run_zdev(root, &["goal", "checkout"]);
    let second = run_zdev(root, &["goal", "checkout"]);
    assert!(first.status.success());
    assert_eq!(first.stdout, second.stdout);
    let native_goal = "Complete zdev task checkout-002 in area checkout. Treat .zdev/checkout/area.toml, .zdev/checkout/slices/payments.md, and .zdev/checkout/tasks/002-reject-duplicate-payment.md as authoritative. Meet the recorded outcome, boundaries, done-when conditions, and validation; preserve zdev approval, branch-safety, independent-verification, task-completion, and commit rules. Stop and report if the task is no longer ready or needs a product decision.";
    let expected_text = format!(
        "Area: checkout — Checkout reliability\nLifecycle: open\nQueue: ready\nObjective:\nMake checkout failures safe and understandable.\nCounts: 3 total; 2 open; 1 ready; 1 blocked; 1 done\n\nTask: checkout-002 — Reject duplicate payment submission\nComplexity: standard\nTask source: .zdev/checkout/tasks/002-reject-duplicate-payment.md\nOutcome:\nA repeated submission returns the original payment result without charging again.\n\nContext:\nThe provider can retry after losing our first response.\n\nSlice: payments — Payment submission\nSlice source: .zdev/checkout/slices/payments.md\nSlice objective:\nMake payment submission safe to retry.\nSlice boundaries:\n- Do not change provider selection.\n\nBoundaries:\n- Keep the public response schema unchanged.\n\nDone when:\n- [ ] Duplicate provider calls are prevented.\n- [ ] The original result is returned.\n\nValidation:\n- Run the focused payment integration test.\n\nNative goal:\n{native_goal}\n"
    );
    assert_eq!(first.stdout, expected_text.as_bytes());

    let json_first = run_zdev(root, &["goal", "checkout", "--format", "json"]);
    let json_second = run_zdev(root, &["goal", "checkout", "--format", "json"]);
    assert_eq!(json_first.stdout, json_second.stdout);
    let expected_json = format!(
        "{{\n  \"schema_version\": 1,\n  \"area\": {{\n    \"tag\": \"checkout\",\n    \"title\": \"Checkout reliability\",\n    \"objective\": \"Make checkout failures safe and understandable.\",\n    \"path\": \".zdev/checkout\"\n  }},\n  \"lifecycle\": \"open\",\n  \"queue\": \"ready\",\n  \"counts\": {{\n    \"total\": 3,\n    \"open\": 2,\n    \"ready\": 1,\n    \"blocked\": 1,\n    \"done\": 1\n  }},\n  \"task\": {{\n    \"id\": \"checkout-002\",\n    \"key\": \"reject-duplicate-payment\",\n    \"title\": \"Reject duplicate payment submission\",\n    \"complexity\": \"standard\",\n    \"path\": \".zdev/checkout/tasks/002-reject-duplicate-payment.md\",\n    \"outcome\": \"A repeated submission returns the original payment result without charging again.\",\n    \"context\": \"The provider can retry after losing our first response.\",\n    \"boundaries\": \"- Keep the public response schema unchanged.\",\n    \"done_when\": \"- [ ] Duplicate provider calls are prevented.\\n- [ ] The original result is returned.\",\n    \"validation\": \"- Run the focused payment integration test.\",\n    \"blocked_by\": [],\n    \"slice\": {{\n      \"key\": \"payments\",\n      \"title\": \"Payment submission\",\n      \"path\": \".zdev/checkout/slices/payments.md\",\n      \"objective\": \"Make payment submission safe to retry.\",\n      \"boundaries\": \"- Do not change provider selection.\"\n    }}\n  }},\n  \"native_goal\": \"{native_goal}\"\n}}\n"
    );
    assert_eq!(json_first.stdout, expected_json.as_bytes());
}

#[test]
fn work_context_returns_nested_ready_context_and_untrimmed_git_stdout() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "general", "main");
    commit_all(root, "record area");
    import_one_task(root, "general");
    fs::write(root.join("notes.txt"), "untracked\n").expect("untracked file");

    let output = run_zdev(root, &["work-context", "general", "--format", "json"]);
    let context: Value = serde_json::from_slice(&output.stdout).expect("work context JSON");
    let expected_git_status = git_stdout(root, &["status", "--short", "--untracked-files=all"]);
    let expected_git_diff_cached = git_stdout(root, &["diff", "--cached"]);
    let expected_git_diff = git_stdout(root, &["diff"]);
    let expected_head = git(root, &["rev-parse", "HEAD"]);
    assert_pretty_json(
        &output,
        json!({
            "schema_version": 1,
            "area": "general",
            "lifecycle": "open",
            "queue": "ready",
            "task_id": "general-001",
            "stale_advisory": false,
            "status": context["status"].clone(),
            "goal": context["goal"].clone(),
            "head": expected_head,
            "git_status": expected_git_status,
            "git_diff_cached": expected_git_diff_cached,
            "git_diff": expected_git_diff,
        }),
    );
    assert_eq!(context["area"], "general");
    assert_eq!(context["lifecycle"], "open");
    assert_eq!(context["queue"], "ready");
    assert_eq!(context["task_id"], "general-001");
    assert_eq!(context["stale_advisory"], false);
    assert_eq!(context["head"], expected_head);
    assert_eq!(context["goal"]["task"]["id"], "general-001");
    assert_eq!(context["status"]["next"], "general-001");
    assert!(context["status"].is_object());
    assert!(context["goal"].is_object());
    assert_eq!(context["git_status"], expected_git_status);
    assert_eq!(context["git_diff_cached"], expected_git_diff_cached);
    assert_eq!(context["git_diff"], expected_git_diff);
    assert!(
        context["git_status"]
            .as_str()
            .expect("status string")
            .ends_with('\n'),
        "Git stdout must retain its trailing newline"
    );
}

#[test]
fn work_context_snapshots_round_trip_exact_json_and_compare_fresh_state() {
    let checkout = repository();
    let root = checkout.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "general", "main");
    commit_all(root, "record area");
    import_one_task(root, "general");
    fs::write(root.join("notes.txt"), "untracked\n").expect("untracked file");

    let inline = run_zdev(root, &["work-context", "general", "--format", "json"]);
    assert!(inline.status.success());
    let stored = json_output(root, &["work-context", "general", "--store"]);
    let keys = stored
        .as_object()
        .expect("compact stored context")
        .keys()
        .map(String::as_str)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        keys,
        std::collections::BTreeSet::from([
            "area",
            "complexity",
            "head",
            "lifecycle",
            "path",
            "queue",
            "schema_version",
            "snapshot",
            "stale_advisory",
            "task_id",
        ])
    );
    let snapshot = stored["snapshot"].as_str().expect("snapshot ID");
    let path = reported_path(root, &stored["path"]);
    assert_eq!(fs::read(&path).expect("snapshot bytes"), inline.stdout);
    assert!(!path.starts_with(root.join(".zdev")));
    let published = fs::metadata(&path)
        .expect("snapshot metadata")
        .modified()
        .expect("snapshot publication time");
    std::thread::sleep(std::time::Duration::from_millis(2));
    let stored_again = json_output(root, &["work-context", "general", "--store"]);
    assert_eq!(stored_again["snapshot"], stored["snapshot"]);
    assert_eq!(stored_again["path"], stored["path"]);
    assert_eq!(
        fs::metadata(&path)
            .expect("unchanged snapshot metadata")
            .modified()
            .expect("unchanged publication time"),
        published,
        "an identical snapshot must not be rewritten"
    );

    for arguments in [
        vec!["work-context", "general", "--show", snapshot],
        vec![
            "work-context",
            "general",
            "--show",
            snapshot,
            "--format",
            "json",
        ],
    ] {
        let shown = run_zdev(root, &arguments);
        assert!(shown.status.success());
        assert_eq!(shown.stdout, inline.stdout);
    }
    assert_eq!(
        json_output(root, &["work-context", "general", "--compare", snapshot]),
        json!({
            "schema_version": 1,
            "area": "general",
            "snapshot": snapshot,
            "equal": true,
        })
    );
    let files_before = fs::read_dir(path.parent().expect("snapshot store"))
        .expect("snapshot store")
        .count();
    fs::write(root.join("seed.txt"), "changed\n").expect("change tracked state");
    let compared = json_output(root, &["work-context", "general", "--compare", snapshot]);
    assert_eq!(compared["equal"], false);
    assert_eq!(
        fs::read_dir(path.parent().expect("snapshot store"))
            .expect("snapshot store")
            .count(),
        files_before,
        "comparison must not create another artifact"
    );

    let closed_repository = repository();
    let closed_root = closed_repository.path();
    git(closed_root, &["branch", "-m", "main"]);
    commit_file(closed_root, "seed.txt", "seed\n", "seed");
    json_output(closed_root, &["init", "--record", "project"]);
    json_output(
        closed_root,
        &[
            "area",
            "create",
            "closed",
            "--title",
            "Closed",
            "--objective",
            "Remain closed.",
            "--trunk",
        ],
    );
    json_output(closed_root, &["area", "close", "closed"]);
    let closed = json_output(closed_root, &["work-context", "closed", "--store"]);
    assert_eq!(closed["lifecycle"], "closed");
    assert_eq!(closed["task_id"], Value::Null);
    assert!(closed.get("head").is_none());
}

#[test]
fn work_context_snapshots_fail_closed_and_remain_available() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    for area in ["source", "other"] {
        json_output(
            root,
            &[
                "area",
                "create",
                area,
                "--title",
                area,
                "--objective",
                "Exercise stored work-context failures.",
                "--trunk",
            ],
        );
    }
    commit_all(root, "record areas");

    let first = json_output(root, &["work-context", "source", "--store"]);
    let first_id = first["snapshot"]
        .as_str()
        .expect("first snapshot")
        .to_owned();
    let first_path = reported_path(root, &first["path"]);
    let other_store = git_path(root, "zdev/work-context/other");
    fs::create_dir_all(&other_store).expect("other snapshot store");
    fs::copy(&first_path, other_store.join(format!("{first_id}.json")))
        .expect("copy cross-area snapshot");
    let cross_area = run_zdev(root, &["work-context", "other", "--show", &first_id]);
    assert!(!cross_area.status.success());
    assert!(String::from_utf8_lossy(&cross_area.stderr).contains("does not match selected area"));

    fs::write(&first_path, b"{\n").expect("corrupt snapshot");
    let corrupt = run_zdev(root, &["work-context", "source", "--show", &first_id]);
    assert!(!corrupt.status.success());
    assert!(String::from_utf8_lossy(&corrupt.stderr).contains("is corrupt"));
    fs::remove_file(&first_path).expect("remove corrupt snapshot");

    let mut snapshots = Vec::new();
    for number in 0..9 {
        fs::write(root.join("seed.txt"), format!("state {number}\n")).expect("distinct state");
        let stored = json_output(root, &["work-context", "source", "--store"]);
        snapshots.push(stored["snapshot"].as_str().expect("snapshot ID").to_owned());
    }
    fs::write(root.join("seed.txt"), "state 0\n").expect("restore old baseline state");
    let restored = json_output(root, &["work-context", "source", "--store"]);
    assert_eq!(restored["snapshot"], snapshots[0]);
    fs::write(root.join("seed.txt"), "state 9\n").expect("publish later state");
    json_output(root, &["work-context", "source", "--store"]);
    let store = git_path(root, "zdev/work-context/source");
    let retained = fs::read_dir(&store)
        .expect("source snapshot store")
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().and_then(|value| value.to_str()) == Some("json"))
        .count();
    assert_eq!(retained, 10);
    let old_baseline = run_zdev(root, &["work-context", "source", "--show", &snapshots[0]]);
    assert!(old_baseline.status.success());
    fs::remove_file(store.join(format!("{}.json", snapshots[0]))).expect("remove snapshot");
    let unavailable = run_zdev(root, &["work-context", "source", "--show", &snapshots[0]]);
    assert!(!unavailable.status.success());
    assert!(String::from_utf8_lossy(&unavailable.stderr).contains("is unavailable"));
}

#[test]
fn work_context_snapshots_use_the_linked_worktree_git_path() {
    let repository = repository();
    let root = repository.path();
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "linked",
            "--title",
            "Linked",
            "--objective",
            "Store context in linked-worktree Git state.",
            "--trunk",
        ],
    );
    import_one_task(root, "linked");
    commit_all(root, "record area");
    git(root, &["switch", "-q", "-c", "parking"]);
    let linked_parent = tempfile::tempdir().expect("linked parent");
    let linked = linked_parent.path().join("checkout");
    git(
        root,
        &[
            "worktree",
            "add",
            "-q",
            linked.to_str().expect("linked path"),
            "main",
        ],
    );

    let stored = json_output(&linked, &["work-context", "linked", "--store"]);
    let expected = git_path(&linked, "zdev/work-context/linked").join(format!(
        "{}.json",
        stored["snapshot"].as_str().expect("snapshot ID")
    ));
    assert_eq!(reported_path(&linked, &stored["path"]), expected);
    assert!(expected.is_file());
    assert!(git(&linked, &["status", "--short", "--untracked-files=all"]).is_empty());
}

#[cfg(unix)]
#[test]
fn work_context_snapshot_publication_failure_preserves_existing_store() {
    use std::os::unix::fs::PermissionsExt;

    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "general",
            "--title",
            "General",
            "--objective",
            "Exercise publication failure.",
            "--trunk",
        ],
    );
    commit_all(root, "record area");
    let stored = json_output(root, &["work-context", "general", "--store"]);
    let store = reported_path(root, &stored["path"])
        .parent()
        .expect("snapshot store")
        .to_path_buf();
    let before = fs::read_dir(&store)
        .expect("snapshot store")
        .map(|entry| entry.expect("snapshot entry").file_name())
        .collect::<std::collections::BTreeSet<_>>();
    fs::write(root.join("seed.txt"), "changed\n").expect("new context");
    let original_permissions = fs::metadata(&store).expect("store metadata").permissions();
    let mut read_only = original_permissions.clone();
    read_only.set_mode(0o555);
    fs::set_permissions(&store, read_only).expect("read-only store");
    let failed = run_zdev(root, &["work-context", "general", "--store"]);
    fs::set_permissions(&store, original_permissions).expect("restore store permissions");
    assert!(!failed.status.success());
    assert!(String::from_utf8_lossy(&failed.stderr).contains("Cannot stage"));
    let after = fs::read_dir(&store)
        .expect("snapshot store")
        .map(|entry| entry.expect("snapshot entry").file_name())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(after, before);
    assert_eq!(
        git(root, &["status", "--short", "--untracked-files=all"]),
        "M seed.txt"
    );
}

#[test]
fn work_context_returns_open_empty_and_exhausted_contexts() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "general", "main");

    let empty = json_output(root, &["work-context", "general"]);
    assert_eq!(empty["queue"], "empty");
    assert_eq!(empty["task_id"], Value::Null);
    assert_eq!(empty["status"]["next"], Value::Null);
    assert_eq!(empty["goal"]["task"], Value::Null);

    import_one_task(root, "general");
    json_output(
        root,
        &[
            "task",
            "done",
            "general",
            "general-001",
            "--summary",
            "Completed the task.",
            "--validation",
            "Focused check passed.",
        ],
    );
    let exhausted = json_output(root, &["work-context", "general"]);
    assert_eq!(exhausted["queue"], "exhausted");
    assert_eq!(exhausted["task_id"], Value::Null);
    assert_eq!(exhausted["status"]["next"], Value::Null);
    assert_eq!(exhausted["goal"]["task"], Value::Null);
}

#[cfg(unix)]
#[test]
fn closed_work_context_is_branch_independent_and_never_invokes_git() {
    use std::os::unix::fs::PermissionsExt;

    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "general", "main");
    json_output(root, &["area", "close", "general"]);
    git(root, &["switch", "--detach", "-q", "HEAD"]);

    let fake_path = tempfile::tempdir().expect("fake PATH");
    let fake_git = fake_path.path().join("git");
    fs::write(&fake_git, "#!/bin/sh\nexit 97\n").expect("fake git");
    let mut permissions = fs::metadata(&fake_git)
        .expect("fake git metadata")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&fake_git, permissions).expect("executable fake git");

    let output = run_zdev_with_env(
        root,
        &["work-context", "general", "--format", "json"],
        &[("PATH", fake_path.path())],
    );
    let context: Value = serde_json::from_slice(&output.stdout).expect("closed context JSON");
    assert_pretty_json(
        &output,
        json!({
            "schema_version": 1,
            "area": "general",
            "lifecycle": "closed",
            "queue": "empty",
            "task_id": Value::Null,
            "goal": context["goal"].clone(),
        }),
    );
    assert_eq!(context["lifecycle"], "closed");
    assert_eq!(context["queue"], "empty");
    assert_eq!(context["task_id"], Value::Null);
    assert!(context["goal"].is_object());
    for omitted in [
        "status",
        "stale_advisory",
        "git_status",
        "git_diff_cached",
        "git_diff",
    ] {
        assert!(context.get(omitted).is_none(), "unexpected {omitted}");
    }
}

#[test]
fn work_context_rejects_unsafe_and_blocked_open_work_with_structured_errors() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "general", "main");
    import_one_task(root, "general");
    git(root, &["switch", "--detach", "-q", "HEAD"]);

    let unsafe_output = run_zdev(root, &["work-context", "general", "--format", "json"]);
    assert!(!unsafe_output.status.success());
    let unsafe_error: Value =
        serde_json::from_slice(&unsafe_output.stderr).expect("unsafe JSON error");
    assert_eq!(unsafe_error["command"], "work-context");
    assert_eq!(
        unsafe_error["details"]["status"]["branch_status"]["task_work"]["safe"],
        false
    );
    assert_eq!(unsafe_error["details"]["goal"]["task"]["id"], "general-001");

    git(root, &["switch", "-q", "main"]);
    let task_path = root.join(".zdev/general/tasks/001-complete-one-task.md");
    let task = fs::read_to_string(&task_path)
        .expect("task")
        .replace("blocked_by = []", "blocked_by = [\"general-999\"]");
    fs::write(task_path, task).expect("blocked task");
    let blocked = run_zdev(root, &["work-context", "general", "--format", "json"]);
    assert!(!blocked.status.success());
    let blocked_error: Value = serde_json::from_slice(&blocked.stderr).expect("blocked JSON error");
    assert_eq!(blocked_error["command"], "work-context");
    assert!(
        blocked_error["error"]
            .as_str()
            .expect("blocked error")
            .contains("missing blocker general-999")
    );
}

#[cfg(unix)]
#[test]
fn explicit_work_context_revalidates_selection_after_goal_read() {
    use std::os::unix::fs::PermissionsExt;

    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "general", "main");
    import_one_task(root, "general");
    let second = serde_json::to_vec(&json!({
        "schema_version": 1,
        "area": "general",
        "tasks": [{
            "key": "two",
            "title": "Complete another task",
            "outcome": "The second task completes.",
            "done_when": ["The second task is complete."],
            "validation": ["Exercise the CLI."],
            "blocked_by": []
        }]
    }))
    .expect("second task bundle");
    json_output_with_stdin(
        root,
        &["tasks", "import", "general", "--from", "-"],
        &second,
    );

    let task_path = root.join(".zdev/general/tasks/001-complete-one-task.md");
    let done_path = root.join("done-task.md");
    let done_task = fs::read_to_string(&task_path)
        .expect("task")
        .replace("status = \"open\"", "status = \"done\"")
        .replace("- [ ] ", "- [x] ")
        + "\n## Result\n\nCompleted during the test.\n\nValidation:\n\n- Focused check passed.\n";
    fs::write(&done_path, done_task).expect("done task source");

    let fake_path = tempfile::tempdir().expect("fake PATH");
    let fake_git = fake_path.path().join("git");
    let marker = root.join("mutation-ran");
    fs::write(
        &fake_git,
        "#!/bin/sh\nif [ \"$1\" = symbolic-ref ] && [ ! -e \"$MUTATED\" ]; then\n  while IFS= read -r line || [ -n \"$line\" ]; do printf '%s\\n' \"$line\"; done < \"$DONE_TASK\" > \"$TASK_PATH\"\n  : > \"$MUTATED\"\nfi\nexec \"$REAL_GIT\" \"$@\"\n",
    )
    .expect("mutating git");
    let mut permissions = fs::metadata(&fake_git)
        .expect("fake git metadata")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&fake_git, permissions).expect("executable fake git");
    let real_git = executable_on_path("git");

    let output = run_zdev_with_env(
        root,
        &[
            "work-context",
            "general",
            "--task",
            "general-001",
            "--format",
            "json",
        ],
        &[
            ("PATH", fake_path.path()),
            ("REAL_GIT", &real_git),
            ("DONE_TASK", &done_path),
            ("TASK_PATH", &task_path),
            ("MUTATED", &marker),
        ],
    );
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    let error: Value = serde_json::from_slice(&output.stderr).expect("disagreement JSON");
    assert!(
        error["error"]
            .as_str()
            .expect("disagreement error")
            .contains("not in the ready frontier")
    );
}

#[cfg(unix)]
#[test]
fn work_context_reports_git_collection_failure_without_partial_output() {
    use std::os::unix::fs::PermissionsExt;

    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "general", "main");

    let fake_path = tempfile::tempdir().expect("fake PATH");
    let fake_git = fake_path.path().join("git");
    fs::write(
        &fake_git,
        "#!/bin/sh\nif [ \"$1\" = diff ] && [ \"$2\" = --cached ]; then\n  echo deliberate failure >&2\n  exit 19\nfi\nexec \"$REAL_GIT\" \"$@\"\n",
    )
    .expect("fake git");
    let mut permissions = fs::metadata(&fake_git)
        .expect("fake git metadata")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&fake_git, permissions).expect("executable fake git");
    let real_git = executable_on_path("git");

    let output = run_zdev_with_env(
        root,
        &["work-context", "general", "--format", "json"],
        &[("PATH", fake_path.path()), ("REAL_GIT", &real_git)],
    );
    assert!(!output.status.success());
    assert!(
        output.stdout.is_empty(),
        "failure must not emit partial context"
    );
    let error: Value = serde_json::from_slice(&output.stderr).expect("failure JSON");
    assert_eq!(error["command"], "work-context");
    assert!(
        error["error"]
            .as_str()
            .expect("error")
            .contains("git diff --cached failed: deliberate failure")
    );

    fs::write(
        &fake_git,
        "#!/bin/sh\nif [ \"$1\" = diff ] && [ \"$#\" = 1 ]; then\n  printf '\\377'\n  exit 0\nfi\nexec \"$REAL_GIT\" \"$@\"\n",
    )
    .expect("invalid UTF-8 git");
    let invalid = run_zdev_with_env(
        root,
        &["work-context", "general", "--format", "json"],
        &[("PATH", fake_path.path()), ("REAL_GIT", &real_git)],
    );
    assert!(!invalid.status.success());
    assert!(invalid.stdout.is_empty());
    let invalid_error: Value =
        serde_json::from_slice(&invalid.stderr).expect("invalid UTF-8 JSON error");
    assert!(
        invalid_error["error"]
            .as_str()
            .expect("invalid UTF-8 error")
            .contains("git diff returned invalid UTF-8")
    );
}

#[test]
fn goal_handles_empty_unsliced_and_exhausted_states() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "general",
            "--title",
            "General improvements",
            "--objective",
            "Capture small approved improvements without inventing a product roadmap.",
        ],
    );
    let empty = run_zdev(root, &["goal", "general"]);
    assert_eq!(
        String::from_utf8(empty.stdout).expect("empty goal"),
        "Area: general — General improvements\nLifecycle: open\nQueue: empty\nObjective:\nCapture small approved improvements without inventing a product roadmap.\nCounts: 0 total; 0 open; 0 ready; 0 blocked; 0 done\n\nThe open area has no tasks. Create and approve a task, or close the area.\n"
    );
    let empty_output = run_zdev(root, &["goal", "general", "--format", "json"]);
    assert_eq!(
        empty_output.stdout,
        b"{\n  \"schema_version\": 1,\n  \"area\": {\n    \"tag\": \"general\",\n    \"title\": \"General improvements\",\n    \"objective\": \"Capture small approved improvements without inventing a product roadmap.\",\n    \"path\": \".zdev/general\"\n  },\n  \"lifecycle\": \"open\",\n  \"queue\": \"empty\",\n  \"counts\": {\n    \"total\": 0,\n    \"open\": 0,\n    \"ready\": 0,\n    \"blocked\": 0,\n    \"done\": 0\n  },\n  \"task\": null\n}\n"
    );
    let empty_json: Value = serde_json::from_slice(&empty_output.stdout).expect("empty JSON");
    assert_eq!(empty_json["lifecycle"], "open");
    assert_eq!(empty_json["queue"], "empty");
    assert_eq!(empty_json["task"], Value::Null);
    assert!(empty_json.get("native_goal").is_none());

    let task_path = root.join(".zdev/general/tasks/001-one-off.md");
    fs::write(
        &task_path,
        "+++\nschema_version = 1\nid = \"general-001\"\nkey = \"one-off\"\narea = \"general\"\nstatus = \"open\"\nblocked_by = []\n+++\n# Improve one thing\n\n## Outcome\n\nOne useful thing improves.\n\n## Done when\n\n- [ ] The improvement works.\n\n## Validation\n\n- Run the focused check.\n",
    )
    .expect("unsliced goal task");
    let unsliced = json_output(root, &["goal", "general"]);
    let task = &unsliced["task"];
    assert_eq!(task["id"], "general-001");
    for omitted in ["context", "boundaries", "slice"] {
        assert!(task.get(omitted).is_none(), "unexpected {omitted}");
    }
    assert!(
        !unsliced["native_goal"]
            .as_str()
            .expect("native goal")
            .contains("slices/")
    );

    fs::write(
        &task_path,
        "+++\nschema_version = 1\nid = \"general-001\"\nkey = \"one-off\"\narea = \"general\"\nstatus = \"done\"\nblocked_by = []\n+++\n# Improve one thing\n\n## Outcome\n\nOne useful thing improves.\n\n## Done when\n\n- [x] The improvement works.\n\n## Validation\n\n- Run the focused check.\n\n## Result\n\nThe improvement works.\n",
    )
    .expect("complete goal task");
    let complete = run_zdev(root, &["goal", "general"]);
    assert_eq!(
        String::from_utf8(complete.stdout).expect("complete goal"),
        "Area: general — General improvements\nLifecycle: open\nQueue: exhausted\nObjective:\nCapture small approved improvements without inventing a product roadmap.\nCounts: 1 total; 0 open; 0 ready; 0 blocked; 1 done\n\nThe open area's task queue is exhausted. Add approved work, reopen a task, or close the area.\n"
    );
    let complete_output = run_zdev(root, &["goal", "general", "--format", "json"]);
    assert_eq!(
        complete_output.stdout,
        b"{\n  \"schema_version\": 1,\n  \"area\": {\n    \"tag\": \"general\",\n    \"title\": \"General improvements\",\n    \"objective\": \"Capture small approved improvements without inventing a product roadmap.\",\n    \"path\": \".zdev/general\"\n  },\n  \"lifecycle\": \"open\",\n  \"queue\": \"exhausted\",\n  \"counts\": {\n    \"total\": 1,\n    \"open\": 0,\n    \"ready\": 0,\n    \"blocked\": 0,\n    \"done\": 1\n  },\n  \"task\": null\n}\n"
    );
    let complete_json: Value =
        serde_json::from_slice(&complete_output.stdout).expect("complete JSON");
    assert_eq!(complete_json["lifecycle"], "open");
    assert_eq!(complete_json["queue"], "exhausted");
    assert_eq!(complete_json["task"], Value::Null);
    assert!(complete_json.get("native_goal").is_none());
}

#[test]
fn malformed_goal_graph_fails_without_mutation() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "broken",
            "--title",
            "Broken graph",
            "--objective",
            "Reject malformed task dependencies.",
        ],
    );
    let task_path = root.join(".zdev/broken/tasks/001-broken.md");
    fs::write(
        &task_path,
        "+++\nschema_version = 1\nid = \"broken-001\"\nkey = \"broken\"\narea = \"broken\"\nstatus = \"open\"\nblocked_by = [\"broken-999\"]\n+++\n# Broken dependency\n\n## Outcome\n\nThe malformed graph is rejected.\n\n## Done when\n\n- [ ] The command fails.\n\n## Validation\n\n- Run the focused check.\n",
    )
    .expect("malformed goal task");
    let before = fs::read(&task_path).expect("task before goal");
    let inventory = file_inventory(&root.join(".zdev"));
    let git_before = git(root, &["status", "--porcelain=v1", "--untracked-files=all"]);

    let rejected = run_zdev(root, &["goal", "broken", "--format", "json"]);
    assert!(!rejected.status.success());
    assert!(rejected.stdout.is_empty());
    let error: Value = serde_json::from_slice(&rejected.stderr).expect("goal error JSON");
    assert_eq!(error["command"], "goal");
    assert!(
        error["error"]
            .as_str()
            .expect("goal error")
            .contains("missing blocker broken-999")
    );
    assert_eq!(fs::read(&task_path).expect("task after goal"), before);
    assert_eq!(file_inventory(&root.join(".zdev")), inventory);
    assert_eq!(
        git(root, &["status", "--porcelain=v1", "--untracked-files=all"]),
        git_before
    );
}

#[test]
fn next_any_selects_ready_work_across_areas_without_changing_git_state() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "alpha", "alpha");
    create_area(root, "zeta", "zeta");
    create_area(root, "unsafe", "missing-branch");
    import_one_task(root, "alpha");
    import_one_task(root, "zeta");
    import_one_task(root, "unsafe");
    commit_all(root, "configure areas");
    git(root, &["branch", "alpha"]);
    git(root, &["branch", "zeta"]);
    commit_file(root, "main.txt", "advance\n", "advance main");

    git(root, &["switch", "-q", "zeta"]);
    let on_area = json_output(root, &["next", "--any"]);
    assert_eq!(on_area["area"], "zeta");
    assert_eq!(on_area["branch"], "zeta");
    assert_eq!(on_area["branch_matches"], true);
    assert_eq!(on_area["task"]["id"], "zeta-001");
    assert_eq!(
        on_area["task"]["path"],
        ".zdev/zeta/tasks/001-complete-one-task.md"
    );
    assert_eq!(on_area["skipped"][0]["area"], "unsafe");
    assert!(
        on_area["skipped"][0]["diagnostics"]
            .as_array()
            .expect("unsafe diagnostics")
            .contains(&json!("area-branch-missing"))
    );

    git(root, &["switch", "-q", "main"]);
    let config_path = root.join(".zdev/config.toml");
    let config = fs::read_to_string(&config_path).expect("project config");
    fs::write(&config_path, format!("{config}default_area = \"zeta\"\n"))
        .expect("configure default area");
    let bare = run_zdev(root, &["next", "--format", "json"]);
    assert!(!bare.status.success());
    let bare: Value = serde_json::from_slice(&bare.stderr).expect("bare-next error");
    assert_eq!(bare["details"]["branch_status"]["branch"], "zeta");
    let head_before = git(root, &["rev-parse", "HEAD"]);
    let status_before = git(root, &["status", "--porcelain"]);
    let off_branch = json_output(root, &["next", "--any"]);
    assert_eq!(off_branch["area"], "alpha");
    assert_eq!(off_branch["branch"], "alpha");
    assert_eq!(off_branch["branch_matches"], false);
    let text = run_zdev(root, &["next", "--any"]);
    assert!(
        String::from_utf8_lossy(&text.stdout)
            .contains("Required branch: alpha (not checked out; current branch: main)")
    );
    assert_eq!(git(root, &["rev-parse", "HEAD"]), head_before);
    assert_eq!(git(root, &["status", "--porcelain"]), status_before);

    let conflict = run_zdev(root, &["next", "alpha", "--any"]);
    assert_eq!(conflict.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&conflict.stderr).contains("cannot be used with"));
}

#[test]
fn next_any_distinguishes_exhausted_work_from_unsafe_open_work() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "complete", "main");

    let complete = json_output(root, &["next", "--any"]);
    assert_eq!(complete["selection"], "none");
    assert_eq!(complete["reason"], "no-ready-open-area");
    assert_eq!(complete["task"], Value::Null);

    create_area(root, "unsafe", "missing-branch");
    import_one_task(root, "unsafe");
    let unsafe_work = json_output(root, &["next", "--any"]);
    assert_eq!(unsafe_work["selection"], "unsafe");
    assert_eq!(unsafe_work["skipped"][0]["area"], "unsafe");
    assert!(
        unsafe_work["skipped"][0]["diagnostics"]
            .as_array()
            .expect("unsafe diagnostics")
            .contains(&json!("area-branch-missing"))
    );
}

#[test]
fn area_lifecycle_distinguishes_queue_exhaustion_from_explicit_closure() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "general", "main");

    let metadata_path = root.join(".zdev/general/area.toml");
    let metadata = fs::read_to_string(&metadata_path).expect("area metadata");
    assert!(metadata.contains("lifecycle = \"open\""));
    let legacy = metadata
        .lines()
        .filter(|line| !line.starts_with("lifecycle = "))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&metadata_path, format!("{legacy}\n")).expect("legacy area metadata");
    assert_eq!(
        json_output(root, &["status", "general"])["lifecycle"],
        "open"
    );

    assert_eq!(
        String::from_utf8(run_zdev(root, &["area", "close", "general"]).stdout)
            .expect("close text"),
        "Closed area general\n"
    );
    assert_eq!(
        String::from_utf8(run_zdev(root, &["area", "reopen", "general"]).stdout)
            .expect("reopen text"),
        "Reopened area general\n"
    );
    let close_output = run_zdev(root, &["area", "close", "general", "--format", "json"]);
    let closed: Value = serde_json::from_slice(&close_output.stdout).expect("close JSON");
    assert_pretty_json(
        &close_output,
        json!({"advisory":Value::Null,"area":"general","branch_status":closed["branch_status"].clone(),"lifecycle":"closed","schema_version":1,"status":"closed"}),
    );
    assert_eq!(closed["status"], "closed");
    assert_eq!(closed["lifecycle"], "closed");
    let selected_status = run_zdev(root, &["status", "general", "--format", "json"]);
    let selected: Value =
        serde_json::from_slice(&selected_status.stdout).expect("selected status JSON");
    assert_pretty_json(
        &selected_status,
        json!({
            "advisory": Value::Null,
            "area": {"base_commit":selected["area"]["base_commit"].clone(),"branch":"main","lifecycle":"closed","mode":"isolated","objective":"Exercise managed area branches.","schema_version":1,"tag":"general","title":"general"},
            "branch_status": selected["branch_status"].clone(),
            "counts": {"blocked":0,"done":0,"ready":0,"total":0},
            "lifecycle":"closed","next":Value::Null,"next_complexity":Value::Null,"project":selected["project"].clone(),"queue":"empty","schema_version":1,"slices":[],"trunk":"main"
        }),
    );
    let status_text = String::from_utf8(run_zdev(root, &["status", "general"]).stdout)
        .expect("selected status text");
    assert_eq!(
        status_text,
        "general\nLifecycle: closed\nQueue: empty\nCounts: 0 total; 0 ready; 0 blocked; 0 done\ngeneral: main -> trunk main [fresh]\n"
    );
    let project_status = run_zdev(root, &["status", "--format", "json"]);
    let project: Value =
        serde_json::from_slice(&project_status.stdout).expect("project status JSON");
    assert_pretty_json(
        &project_status,
        json!({
            "areas":[{"blocked":0,"branch_status":project["areas"][0]["branch_status"].clone(),"done":0,"lifecycle":"closed","mode":"isolated","next":Value::Null,"next_complexity":Value::Null,"queue":"empty","ready":0,"tag":"general","title":"general","total":0}],
            "checked_out_branch":"main","project":project["project"].clone(),"schema_version":1,"trunk":"main"
        }),
    );
    assert_eq!(
        String::from_utf8(run_zdev(root, &["status"]).stdout).expect("project status text"),
        format!(
            "{}: 1 areas (trunk: main)\ngeneral: closed, empty; main -> trunk main [fresh]\n",
            project["project"].as_str().expect("project name")
        )
    );
    assert_eq!(
        String::from_utf8(run_zdev(root, &["next", "general"]).stdout).expect("closed next"),
        "Area general is closed. Run `zdev area reopen general` before adding or selecting work\n"
    );
    let closed_next = json_output(root, &["next", "general"]);
    assert_eq!(
        closed_next,
        json!({"area":"general","lifecycle":"closed","queue":"empty","schema_version":1,"task":Value::Null})
    );
    let closed_goal = json_output(root, &["goal", "general"]);
    assert_eq!(closed_goal["lifecycle"], "closed");
    assert_eq!(closed_goal["queue"], "empty");
    assert_eq!(closed_goal["task"], Value::Null);
    assert_eq!(
        String::from_utf8(run_zdev(root, &["area", "close", "general"]).stdout)
            .expect("unchanged close text"),
        "Area general is already closed\n"
    );
    assert_eq!(
        String::from_utf8(run_zdev(root, &["next", "--any"]).stdout).expect("all-closed next-any"),
        "No area is open. Run `zdev area reopen <area>` before selecting work\nExcluded closed areas: general\n"
    );
    assert_eq!(
        json_output(root, &["next", "--any"]),
        json!({"closed_areas":["general"],"mode":"any","reason":"no-open-area","schema_version":1,"selection":"none","skipped":[],"task":Value::Null})
    );

    git(root, &["switch", "-q", "-c", "other"]);
    assert_eq!(
        json_output(root, &["next", "general"])["lifecycle"],
        "closed"
    );
    let wrong_branch = run_zdev(root, &["area", "reopen", "general"]);
    assert!(!wrong_branch.status.success());
    git(root, &["switch", "-q", "main"]);
    let reopen_output = run_zdev(root, &["area", "reopen", "general", "--format", "json"]);
    let reopened: Value = serde_json::from_slice(&reopen_output.stdout).expect("reopen JSON");
    assert_pretty_json(
        &reopen_output,
        json!({"advisory":Value::Null,"area":"general","branch_status":reopened["branch_status"].clone(),"lifecycle":"open","schema_version":1,"status":"open"}),
    );
    assert_eq!(
        String::from_utf8(run_zdev(root, &["area", "reopen", "general"]).stdout)
            .expect("unchanged reopen text"),
        "Area general is already open\n"
    );
    let unchanged_reopen = run_zdev(root, &["area", "reopen", "general", "--format", "json"]);
    let unchanged: Value =
        serde_json::from_slice(&unchanged_reopen.stdout).expect("unchanged reopen JSON");
    assert_pretty_json(
        &unchanged_reopen,
        json!({"advisory":Value::Null,"area":"general","branch_status":unchanged["branch_status"].clone(),"lifecycle":"open","schema_version":1,"status":"unchanged"}),
    );
    assert_eq!(json_output(root, &["goal", "general"])["queue"], "empty");

    import_one_task(root, "general");
    assert_eq!(
        String::from_utf8(run_zdev(root, &["next", "general"]).stdout).expect("ready next"),
        format!(
            "Area: general\nLifecycle: open\nQueue: ready\n\ngeneral-001  Complete one task\nComplexity: standard\n{}\n",
            fs::canonicalize(root)
                .expect("canonical repository")
                .join(".zdev/general/tasks/001-complete-one-task.md")
                .display()
        )
    );
    let ready_next = run_zdev(root, &["next", "general", "--format", "json"]);
    let ready: Value = serde_json::from_slice(&ready_next.stdout).expect("ready next JSON");
    assert_pretty_json(
        &ready_next,
        json!({"advisory":Value::Null,"area":"general","branch":"main","branch_matches":true,"branch_status":ready["branch_status"].clone(),"lifecycle":"open","mode":"isolated","queue":"ready","schema_version":1,"task":{"afk":false,"blocked_by":[],"complexity":"standard","id":"general-001","path":".zdev/general/tasks/001-complete-one-task.md","priority":"normal","slice":Value::Null,"slice_brief":Value::Null,"state":"ready","status":"open","title":"Complete one task"}}),
    );
    let rejected_close = run_zdev(root, &["area", "close", "general"]);
    assert!(!rejected_close.status.success());
    assert!(String::from_utf8_lossy(&rejected_close.stderr).contains("1 tasks are open"));
    json_output(
        root,
        &[
            "task",
            "done",
            "general",
            "general-001",
            "--summary",
            "Done.",
            "--validation",
            "Checked.",
        ],
    );
    assert_eq!(
        json_output(root, &["goal", "general"])["queue"],
        "exhausted"
    );
    assert_eq!(
        String::from_utf8(run_zdev(root, &["next", "general"]).stdout).expect("exhausted next"),
        "The task queue is exhausted in open area general. Add approved tasks, reopen a task, or run `zdev area close general`\n"
    );
    let exhausted_next = run_zdev(root, &["next", "general", "--format", "json"]);
    let exhausted: Value =
        serde_json::from_slice(&exhausted_next.stdout).expect("exhausted next JSON");
    assert_pretty_json(
        &exhausted_next,
        json!({"advisory":Value::Null,"area":"general","branch":"main","branch_matches":true,"branch_status":exhausted["branch_status"].clone(),"lifecycle":"open","mode":"isolated","queue":"exhausted","schema_version":1,"task":Value::Null}),
    );
    json_output(root, &["area", "close", "general"]);
    let reopen_task = run_zdev(root, &["task", "reopen", "general", "general-001"]);
    assert!(!reopen_task.status.success());
    assert!(String::from_utf8_lossy(&reopen_task.stderr).contains("closed area general"));
    let bundle = root.join("closed-task.json");
    fs::write(
        &bundle,
        serde_json::to_vec(&json!({
            "schema_version": 1,
            "area": "general",
            "tasks": [{
                "key": "two",
                "title": "Another task",
                "outcome": "Another outcome.",
                "done_when": ["It is done."],
                "validation": ["Check it."],
                "blocked_by": []
            }]
        }))
        .expect("closed bundle"),
    )
    .expect("write closed bundle");
    let import = run_zdev(
        root,
        &[
            "tasks",
            "import",
            "general",
            "--from",
            bundle.to_str().expect("bundle path"),
        ],
    );
    assert!(!import.status.success());
    assert!(String::from_utf8_lossy(&import.stderr).contains("closed area general"));

    let task_path = root.join(".zdev/general/tasks/001-complete-one-task.md");
    let done_task = fs::read_to_string(&task_path).expect("done task");
    fs::write(
        &task_path,
        done_task.replace("status = \"done\"", "status = \"open\""),
    )
    .expect("malformed closed area");
    json_output(root, &["tasks", "index", "general"]);
    let malformed = run_zdev(root, &["check"]);
    assert!(!malformed.status.success());
    assert_eq!(
        String::from_utf8_lossy(&malformed.stderr).trim(),
        "error: Closed area general has 1 open tasks"
    );
    fs::write(&task_path, done_task).expect("restore done task");
    json_output(root, &["tasks", "index", "general"]);

    let invalid = fs::read_to_string(&metadata_path)
        .expect("closed metadata")
        .replace("lifecycle = \"closed\"", "lifecycle = \"retired\"");
    fs::write(&metadata_path, invalid).expect("invalid lifecycle");
    let rejected = run_zdev(root, &["status", "general"]);
    assert!(!rejected.status.success());
    assert!(String::from_utf8_lossy(&rejected.stderr).contains("unknown variant"));
}

#[test]
fn trunk_area_create_projects_mode_and_allows_explicit_trunk_sharing() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    let isolated = json_output(
        root,
        &[
            "area",
            "create",
            "legacy",
            "--title",
            "Legacy",
            "--objective",
            "Keep legacy bytes.",
            "--branch",
            "feature",
        ],
    );
    assert_eq!(isolated["mode"], "isolated");
    let legacy_bytes = fs::read_to_string(root.join(".zdev/legacy/area.toml")).unwrap();
    assert!(legacy_bytes.contains("branch = \"feature\""));
    assert!(!legacy_bytes.contains("mode ="));

    for area in ["docs", "quality"] {
        let created = json_output(
            root,
            &[
                "area",
                "create",
                area,
                "--title",
                area,
                "--objective",
                "Work directly on trunk.",
                "--trunk",
            ],
        );
        assert_eq!(created["mode"], "trunk");
        assert_eq!(created["branch"], "main");
        assert_eq!(created["base_commit"], Value::Null);
        let metadata = fs::read_to_string(root.join(format!(".zdev/{area}/area.toml"))).unwrap();
        assert!(metadata.contains("mode = \"trunk\""));
        assert!(!metadata.contains("branch ="));
        assert!(!metadata.contains("parent ="));
        assert!(!metadata.contains("base_commit ="));
    }

    let status = json_output(root, &["status", "docs"]);
    assert_eq!(status["area"]["mode"], "trunk");
    assert_eq!(status["area"]["branch"], "main");
    assert_eq!(status["area"]["base_commit"], Value::Null);
    assert_eq!(status["branch_status"]["mode"], "trunk");
    assert_eq!(status["branch_status"]["branch"], "main");
    assert_eq!(status["branch_status"]["fresh"], true);
    assert_eq!(status["branch_status"]["anchor_valid"], Value::Null);
    assert_eq!(status["branch_status"]["task_work"]["safe"], true);
    assert!(
        String::from_utf8(run_zdev(root, &["status", "docs"]).stdout)
            .unwrap()
            .contains("docs: trunk mode on main [fresh]")
    );
    let project = json_output(root, &["status"]);
    assert_eq!(project["areas"][0]["tag"], "docs");
    assert_eq!(project["areas"][0]["mode"], "trunk");
    assert_eq!(project["areas"][1]["tag"], "legacy");
    assert_eq!(project["areas"][2]["tag"], "quality");
    assert_eq!(json_output(root, &["check"])["status"], "ok");

    let collision = run_zdev(
        root,
        &[
            "area",
            "create",
            "isolated-main",
            "--title",
            "Collision",
            "--objective",
            "Must remain exclusive.",
            "--branch",
            "main",
        ],
    );
    assert!(!collision.status.success());
    let error = String::from_utf8_lossy(&collision.stderr);
    assert!(
        error.contains("already owned by trunk area docs"),
        "{error}"
    );

    git(root, &["branch", "stable"]);
    let config_path = root.join(".zdev/config.toml");
    let config = fs::read_to_string(&config_path).unwrap();
    fs::write(
        &config_path,
        config.replace("trunk = \"main\"", "trunk = \"stable\""),
    )
    .unwrap();
    let moved = json_output(root, &["status", "docs"]);
    assert_eq!(moved["area"]["branch"], "stable");
    assert_eq!(moved["branch_status"]["branch"], "stable");
    assert!(
        !fs::read_to_string(root.join(".zdev/docs/area.toml"))
            .unwrap()
            .contains("stable")
    );
}

#[test]
fn trunk_task_workflows_resolve_config_and_isolate_shared_area_lifecycle() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    git(root, &["branch", "stable"]);

    for area in ["quality", "docs"] {
        json_output(
            root,
            &[
                "area",
                "create",
                area,
                "--title",
                area,
                "--objective",
                "Work directly on trunk.",
                "--trunk",
            ],
        );
    }
    json_output(root, &["config", "trunk", "stable"]);

    // Ordinary import remains branch-independent even after trunk moves.
    import_one_task(root, "quality");
    import_one_task(root, "docs");
    let off_branch = json_output(root, &["next", "--any"]);
    assert_eq!(off_branch["area"], "docs");
    assert_eq!(off_branch["area_mode"], "trunk");
    assert_eq!(off_branch["branch"], "stable");
    assert_eq!(off_branch["branch_matches"], false);
    assert_eq!(off_branch["task"]["id"], "docs-001");

    let blocked = run_zdev(root, &["next", "docs", "--format", "json"]);
    assert!(!blocked.status.success());
    let blocked: Value = serde_json::from_slice(&blocked.stderr).expect("next error JSON");
    assert_eq!(blocked["details"]["branch_status"]["branch"], "stable");
    git(root, &["switch", "-q", "stable"]);

    let selected = json_output(root, &["next", "docs"]);
    assert_eq!(selected["mode"], "trunk");
    assert_eq!(selected["branch"], "stable");
    assert_eq!(selected["branch_matches"], true);
    let context = json_output(root, &["work-context", "docs"]);
    assert_eq!(context["status"]["area"]["mode"], "trunk");
    assert_eq!(context["status"]["area"]["branch"], "stable");
    assert_eq!(context["goal"]["task"]["id"], "docs-001");
    assert_eq!(context["goal"].get("mode"), None);
    assert_eq!(context["goal"].get("branch"), None);

    commit_all(root, "configure shared trunk work");
    let before_cleanup = git(root, &["status", "--short", "--untracked-files=all"]);
    let cleanup = run_zdev(root, &["cleanup", "squash"]);
    assert!(!cleanup.status.success());
    assert!(String::from_utf8_lossy(&cleanup.stderr).contains("not pull-request"));
    assert_eq!(
        git(root, &["status", "--short", "--untracked-files=all"]),
        before_cleanup
    );

    json_output(
        root,
        &[
            "task",
            "done",
            "docs",
            "docs-001",
            "--summary",
            "Done.",
            "--validation",
            "Checked.",
        ],
    );
    let changed = git_stdout(root, &["status", "--short", "--untracked-files=all"]);
    assert_eq!(
        changed.lines().collect::<Vec<_>>(),
        vec![
            " M .zdev/docs/TASKS.md",
            " M .zdev/docs/tasks/001-complete-one-task.md"
        ]
    );
    json_output(root, &["area", "close", "docs"]);
    let quality = json_output(root, &["next", "--any"]);
    assert_eq!(quality["area"], "quality");
    assert_eq!(quality["task"]["id"], "quality-001");
    assert_eq!(quality["closed_areas"], json!(["docs"]));

    json_output(root, &["area", "reopen", "docs"]);
    json_output(root, &["task", "reopen", "docs", "docs-001"]);
    let docs_again = json_output(root, &["next", "--any"]);
    assert_eq!(docs_again["area"], "docs");
    assert_eq!(docs_again["task"]["id"], "docs-001");
}

#[test]
fn area_mode_parser_rejects_contradictions_and_legacy_owner_blocks_trunk() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "legacy-main", "main");
    let metadata_path = root.join(".zdev/legacy-main/area.toml");
    let metadata = fs::read_to_string(&metadata_path).unwrap();
    fs::write(
        &metadata_path,
        metadata.replace(
            "branch = \"main\"",
            "mode = \"isolated\"\nbranch = \"main\"",
        ),
    )
    .unwrap();
    assert_eq!(
        json_output(root, &["status", "legacy-main"])["area"]["mode"],
        "isolated"
    );
    fs::write(&metadata_path, &metadata).unwrap();
    let collision = run_zdev(
        root,
        &[
            "area",
            "create",
            "trunk",
            "--title",
            "Trunk",
            "--objective",
            "Use explicit trunk.",
            "--trunk",
        ],
    );
    assert!(!collision.status.success());
    assert!(
        String::from_utf8_lossy(&collision.stderr)
            .contains("already owned by isolated area legacy-main")
    );

    fs::write(
        &metadata_path,
        metadata.replace("branch = \"main\"", "mode = \"trunk\"\nbranch = \"main\""),
    )
    .unwrap();
    let invalid = run_zdev(root, &["check"]);
    assert!(!invalid.status.success());
    assert!(
        String::from_utf8_lossy(&invalid.stderr)
            .contains("trunk area forbids branch, parent, and base_commit")
    );
}

#[test]
fn area_bind_transitions_preserve_reachable_history_and_prior_bytes() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);

    git(root, &["branch", "feature"]);
    create_area(root, "docs", "feature");
    let to_trunk = json_output(root, &["area", "bind", "docs", "--trunk"]);
    assert_eq!(to_trunk["status"], "updated");
    assert_eq!(to_trunk["mode"], "trunk");
    assert_eq!(to_trunk["branch"], "main");
    assert_eq!(to_trunk["base_commit"], Value::Null);
    let trunk_bytes = fs::read_to_string(root.join(".zdev/docs/area.toml")).unwrap();
    assert!(trunk_bytes.contains("mode = \"trunk\""));
    assert!(!trunk_bytes.contains("branch ="));
    assert_eq!(
        json_output(root, &["area", "bind", "docs", "--trunk"])["status"],
        "unchanged"
    );
    let no_rebase = json_output(root, &["area", "rebase", "docs"]);
    assert_eq!(no_rebase["status"], "unchanged");
    assert_eq!(no_rebase["mode"], "trunk");
    for arguments in [
        vec!["area", "parent", "docs", "missing"],
        vec!["area", "rebase", "docs", "--continue"],
        vec!["area", "rebase", "docs", "--abort"],
    ] {
        let rejected = run_zdev(root, &arguments);
        assert!(!rejected.status.success());
    }

    git(root, &["branch", "docs-work"]);
    let to_isolated = json_output(root, &["area", "bind", "docs", "docs-work"]);
    assert_eq!(to_isolated["status"], "updated");
    assert_eq!(to_isolated["mode"], "isolated");
    assert_eq!(to_isolated["branch"], "docs-work");
    assert_eq!(
        to_isolated["base_commit"],
        git(root, &["rev-parse", "main"])
    );
    let isolated_bytes = fs::read_to_string(root.join(".zdev/docs/area.toml")).unwrap();
    assert!(!isolated_bytes.contains("mode ="));
    assert!(isolated_bytes.contains("branch = \"docs-work\""));
    assert_eq!(
        json_output(root, &["area", "bind", "docs", "docs-work"])["status"],
        "unchanged"
    );

    git(root, &["branch", "unmerged"]);
    create_area(root, "unmerged", "unmerged");
    git(root, &["switch", "-q", "unmerged"]);
    commit_file(root, "unmerged.txt", "work\n", "unmerged work");
    git(root, &["switch", "-q", "main"]);
    let unmerged_path = root.join(".zdev/unmerged/area.toml");
    let unmerged_before = fs::read(&unmerged_path).unwrap();
    let rejected = run_zdev(root, &["area", "bind", "unmerged", "--trunk"]);
    assert!(!rejected.status.success());
    assert!(String::from_utf8_lossy(&rejected.stderr).contains("has commits not contained"));
    assert_eq!(fs::read(&unmerged_path).unwrap(), unmerged_before);

    json_output(
        root,
        &[
            "area",
            "create",
            "quality",
            "--title",
            "Quality",
            "--objective",
            "Work on trunk.",
            "--trunk",
        ],
    );
    git(root, &["branch", "behind"]);
    commit_file(root, "advance.txt", "advance\n", "advance trunk");
    let quality_path = root.join(".zdev/quality/area.toml");
    let quality_before = fs::read(&quality_path).unwrap();
    let rejected = run_zdev(root, &["area", "bind", "quality", "behind"]);
    assert!(!rejected.status.success());
    assert!(
        String::from_utf8_lossy(&rejected.stderr).contains("does not contain configured trunk")
    );
    assert_eq!(fs::read(&quality_path).unwrap(), quality_before);

    let conflict = run_zdev(root, &["area", "bind", "quality", "main", "--trunk"]);
    assert!(!conflict.status.success());
}

#[test]
fn trunk_reconfiguration_requires_resolved_containment_or_explicit_divergence() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    for area in ["quality", "docs"] {
        json_output(
            root,
            &[
                "area",
                "create",
                area,
                "--title",
                area,
                "--objective",
                "Work on trunk.",
                "--trunk",
            ],
        );
    }
    let area_bytes = ["docs", "quality"]
        .map(|area| fs::read(root.join(format!(".zdev/{area}/area.toml"))).expect("area metadata"));
    let old_tip = git(root, &["rev-parse", "main"]);
    git(root, &["switch", "-q", "-c", "stable"]);
    commit_file(root, "stable.txt", "stable\n", "advance stable");
    let stable_tip = git(root, &["rev-parse", "stable"]);
    git(root, &["switch", "-q", "main"]);

    let moved = json_output(root, &["config", "trunk", "stable"]);
    assert_eq!(moved["status"], "updated");
    assert_eq!(moved["previous_trunk"], "main");
    assert_eq!(moved["trunk"], "stable");
    assert_eq!(moved["affected_areas"], json!(["docs", "quality"]));
    assert_eq!(moved["ancestry"]["old_tip"], old_tip);
    assert_eq!(moved["ancestry"]["new_tip"], stable_tip);
    assert_eq!(moved["ancestry"]["old_is_ancestor"], true);
    assert_eq!(moved["ancestry"]["override"], false);
    assert_eq!(
        json_output(root, &["config", "trunk", "stable"])["status"],
        "unchanged"
    );
    for (area, before) in ["docs", "quality"].into_iter().zip(&area_bytes) {
        assert_eq!(
            fs::read(root.join(format!(".zdev/{area}/area.toml"))).unwrap(),
            *before
        );
    }

    let config_path = root.join(".zdev/config.toml");
    let stable_config = fs::read(&config_path).unwrap();
    let behind = run_zdev(root, &["config", "trunk", "main"]);
    assert!(!behind.status.success());
    assert!(String::from_utf8_lossy(&behind.stderr).contains("is not an ancestor"));
    assert_eq!(fs::read(&config_path).unwrap(), stable_config);

    git(root, &["switch", "-q", "-c", "divergent", "main"]);
    commit_file(root, "divergent.txt", "divergent\n", "divergent trunk");
    let divergent_tip = git(root, &["rev-parse", "divergent"]);
    git(root, &["switch", "-q", "main"]);
    let rejected = run_zdev(root, &["config", "set", "project.trunk", "divergent"]);
    assert!(!rejected.status.success());
    assert_eq!(fs::read(&config_path).unwrap(), stable_config);
    let stable_ref = git(root, &["rev-parse", "stable"]);
    let overridden = json_output(
        root,
        &[
            "config",
            "set",
            "--allow-divergent",
            "project.trunk",
            "divergent",
        ],
    );
    assert_eq!(overridden["ancestry"]["old_is_ancestor"], false);
    assert_eq!(overridden["ancestry"]["override"], true);
    assert_eq!(overridden["ancestry"]["new_tip"], divergent_tip);
    assert_eq!(git(root, &["rev-parse", "stable"]), stable_ref);

    let divergent_config = fs::read(&config_path).unwrap();
    git(root, &["branch", "collision", "main"]);
    create_area(root, "isolated-owner", "collision");
    let collision = run_zdev(root, &["config", "trunk", "collision", "--allow-divergent"]);
    assert!(!collision.status.success());
    assert!(String::from_utf8_lossy(&collision.stderr).contains("cannot be shared"));
    assert_eq!(fs::read(&config_path).unwrap(), divergent_config);

    git(root, &["branch", "-D", "divergent"]);
    let missing_old = run_zdev(root, &["config", "trunk", "stable", "--allow-divergent"]);
    assert!(!missing_old.status.success());
    assert!(
        String::from_utf8_lossy(&missing_old.stderr).contains("missing or cannot be inspected")
    );
    assert_eq!(fs::read(&config_path).unwrap(), divergent_config);

    let missing_candidate = run_zdev(root, &["config", "trunk", "missing"]);
    assert!(!missing_candidate.status.success());
    assert_eq!(fs::read(&config_path).unwrap(), divergent_config);
    let unset = run_zdev(root, &["config", "unset", "project.trunk"]);
    assert!(!unset.status.success());
    assert!(
        String::from_utf8_lossy(&unset.stderr)
            .contains("Cannot unset project.trunk while trunk areas exist: docs, quality")
    );
    assert_eq!(fs::read(&config_path).unwrap(), divergent_config);
}

#[cfg(unix)]
#[test]
fn trunk_reconfiguration_does_not_override_uninspectable_ancestry() {
    use std::os::unix::fs::PermissionsExt;

    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "docs",
            "--title",
            "Docs",
            "--objective",
            "Work on trunk.",
            "--trunk",
        ],
    );
    git(root, &["branch", "stable"]);
    let config_path = root.join(".zdev/config.toml");
    let before = fs::read(&config_path).unwrap();

    let fake_path = tempfile::tempdir().expect("fake PATH");
    let fake_git = fake_path.path().join("git");
    fs::write(
        &fake_git,
        "#!/bin/sh\nif [ \"$1\" = merge-base ] && [ \"$2\" = --is-ancestor ]; then\n  exit 2\nfi\nexec \"$REAL_GIT\" \"$@\"\n",
    )
    .expect("fake git");
    let mut permissions = fs::metadata(&fake_git).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&fake_git, permissions).unwrap();
    let real_git = executable_on_path("git");
    let failed = run_zdev_with_env(
        root,
        &["config", "trunk", "stable", "--allow-divergent"],
        &[("PATH", fake_path.path()), ("REAL_GIT", real_git.as_path())],
    );
    assert!(!failed.status.success());
    let error = String::from_utf8_lossy(&failed.stderr);
    assert!(error.contains("ancestry from previous trunk main to stable could not be inspected"));
    assert!(!error.contains("Re-run with --allow-divergent"));
    assert_eq!(fs::read(&config_path).unwrap(), before);
}

#[cfg(unix)]
#[test]
fn trunk_reconfiguration_publication_failure_preserves_config_bytes() {
    use std::os::unix::fs::PermissionsExt;

    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "docs",
            "--title",
            "Docs",
            "--objective",
            "Work on trunk.",
            "--trunk",
        ],
    );
    git(root, &["branch", "stable"]);
    let config_path = root.join(".zdev/config.toml");
    let before = fs::read(&config_path).unwrap();
    let state_dir = root.join(".zdev");
    let original_permissions = fs::metadata(&state_dir).unwrap().permissions();
    let mut read_only = original_permissions.clone();
    read_only.set_mode(0o555);
    fs::set_permissions(&state_dir, read_only).unwrap();
    let failed = run_zdev(root, &["config", "trunk", "stable"]);
    fs::set_permissions(&state_dir, original_permissions).unwrap();
    assert!(!failed.status.success());
    assert_eq!(fs::read(&config_path).unwrap(), before);
}

#[test]
fn stale_independent_base_is_advisory_for_task_work() {
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

    let wrong_branch = run_zdev(root, &["next", "feature", "--format", "json"]);
    assert!(!wrong_branch.status.success());
    let wrong_branch: Value = serde_json::from_slice(&wrong_branch.stderr).expect("JSON error");
    assert!(
        wrong_branch["error"]
            .as_str()
            .expect("error")
            .contains("Switch to feature and retry")
    );
    assert_eq!(
        wrong_branch["details"]["branch_status"]["task_work"]["safe"],
        false
    );
    assert!(
        wrong_branch["details"]["branch_status"]["diagnostics"]
            .as_array()
            .expect("diagnostics")
            .contains(&json!("wrong-branch"))
    );
    git(root, &["switch", "-q", "feature"]);
    let stale_status = json_output(root, &["status", "feature"]);
    assert_eq!(stale_status["branch_status"]["fresh"], false);
    assert_eq!(stale_status["branch_status"]["task_work"]["safe"], true);
    assert_eq!(
        stale_status["branch_status"]["task_work"]["stale_advisory"],
        true
    );
    let status_text = run_zdev(root, &["status", "feature"]);
    assert!(status_text.status.success());
    assert_eq!(
        String::from_utf8_lossy(&status_text.stdout)
            .matches("zdev area rebase feature")
            .count(),
        1
    );
    assert_eq!(
        String::from_utf8(run_zdev(root, &["area", "reopen", "feature"]).stdout)
            .expect("stale reopen text"),
        "Area feature is already open\nAdvisory: run `zdev area rebase feature` when you need current base changes\n"
    );
    let stale_next = json_output(root, &["next", "feature"]);
    assert_eq!(stale_next["task"]["id"], "feature-001");
    assert_eq!(stale_next["branch_status"]["task_work"]["safe"], true);
    assert!(
        stale_next["advisory"]
            .as_str()
            .expect("advisory")
            .contains("zdev area rebase feature")
    );
    let stale_done = json_output(
        root,
        &[
            "task",
            "done",
            "feature",
            "feature-001",
            "--summary",
            "Completed safely on the stale base.",
            "--validation",
            "Focused stale-base coverage passed.",
        ],
    );
    assert_eq!(stale_done["status"], "done");
    assert_eq!(stale_done["branch_status"]["task_work"]["safe"], true);
    assert!(
        stale_done["advisory"]
            .as_str()
            .expect("advisory")
            .contains("zdev area rebase feature")
    );

    commit_all(root, "complete task on stale base");
    let rebased = json_output(root, &["area", "rebase", "feature"]);
    assert_eq!(rebased["status"], "rebased");
    assert_eq!(rebased["effective_base"], "main");
    assert_eq!(
        json_output(root, &["status", "feature"])["branch_status"]["fresh"],
        true
    );
    assert_eq!(
        json_output(root, &["next", "feature"])["queue"],
        "exhausted"
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
    import_one_task(root, "child-area");
    commit_all(root, "configure dependent areas");
    git(root, &["switch", "-q", "-c", "root-area"]);
    commit_file(root, "root.txt", "one\n", "root work");
    json_output(root, &["area", "rebase", "root-area"]);
    assert_eq!(
        json_output(root, &["area", "close", "root-area"])["lifecycle"],
        "closed"
    );
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
    assert_eq!(before["branch_status"]["task_work"]["safe"], true);
    let next = json_output(root, &["next", "child-area"]);
    assert_eq!(next["task"]["id"], "child-area-001");
    assert!(
        next["advisory"]
            .as_str()
            .expect("advisory")
            .contains("zdev area rebase child-area")
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
    assert!(first_task.starts_with(
        "+++\nschema_version = 1\nid = \"lean-core-001\"\nkey = \"first\"\narea = \"lean-core\"\nstatus = \"open\"\nblocked_by = []\n+++\n"
    ));
    assert!(!first_task.contains("complexity ="));
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
    assert!(
        !fs::read_to_string(root.join(".zdev/lean-core/tasks/001-build-the-first-slice.md"))
            .expect("completed legacy task")
            .contains("complexity =")
    );

    assert_eq!(
        json_output(root, &["task", "reopen", "lean-core", "lean-core-001"])["status"],
        "open"
    );
    let reopened =
        fs::read_to_string(root.join(".zdev/lean-core/tasks/001-build-the-first-slice.md"))
            .expect("reopened task");
    assert!(reopened.contains("## History\n\n### Previous completion"));
    assert!(reopened.contains("Implemented and independently verified."));
    assert!(!reopened.contains("\n## Result\n"));
    json_output(
        root,
        &[
            "task",
            "done",
            "lean-core",
            "lean-core-001",
            "--summary",
            "Verified again after reopening.",
            "--validation",
            "Focused test passed again.",
        ],
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
fn default_ranking_and_explicit_frontier_selection_are_distinct() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "ranking", "main");
    commit_all(root, "record ranking area");

    let task = |key: &str, title: &str, afk: bool, priority: &str, blocked_by: Value| {
        json!({
            "key": key,
            "title": title,
            "afk": afk,
            "priority": priority,
            "outcome": format!("{title} is complete."),
            "done_when": [format!("{title} is done.")],
            "validation": ["Inspect the result."],
            "blocked_by": blocked_by,
        })
    };
    let bundle = serde_json::to_vec(&json!({
        "schema_version": 1,
        "area": "ranking",
        "tasks": [
            task("foreground-high", "Foreground high", false, "high", json!([])),
            task("afk-low", "AFK low", true, "low", json!([])),
            task("afk-high", "AFK high", true, "high", json!([])),
            task("blocked", "Blocked", true, "high", json!(["afk-high"])),
        ],
    }))
    .expect("ranking task bundle");
    json_output_with_stdin(
        root,
        &["tasks", "import", "ranking", "--from", "-"],
        &bundle,
    );

    assert_eq!(
        json_output(root, &["next", "ranking"])["task"]["id"],
        "ranking-003"
    );

    let selected = json_output(root, &["work-context", "ranking", "--task", "ranking-002"]);
    assert_eq!(selected["task_id"], "ranking-002");
    assert_eq!(selected["status"]["next"], "ranking-002");
    assert_eq!(selected["goal"]["task"]["id"], "ranking-002");

    let stored = json_output(
        root,
        &[
            "work-context",
            "ranking",
            "--task",
            "ranking-002",
            "--store",
        ],
    );
    assert_eq!(stored["task_id"], "ranking-002");
    assert_eq!(stored["complexity"], "standard");
    assert_eq!(stored["stale_advisory"], false);
    assert_eq!(
        json_output(
            root,
            &[
                "work-context",
                "ranking",
                "--compare",
                stored["snapshot"].as_str().expect("snapshot"),
            ],
        )["equal"],
        true
    );

    json_output(
        root,
        &[
            "task",
            "done",
            "ranking",
            "ranking-002",
            "--summary",
            "Completed the focused task.",
            "--validation",
            "Focused check passed.",
        ],
    );
    assert_eq!(
        json_output(
            root,
            &[
                "work-context",
                "ranking",
                "--compare",
                stored["snapshot"].as_str().expect("snapshot"),
            ],
        )["equal"],
        false
    );

    let blocked = run_zdev(
        root,
        &[
            "work-context",
            "ranking",
            "--task",
            "ranking-004",
            "--format",
            "json",
        ],
    );
    assert!(!blocked.status.success());
    assert!(String::from_utf8_lossy(&blocked.stderr).contains("not in the ready frontier"));
}

#[test]
fn derived_review_accepts_a_focused_non_default_ready_task() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "focused", "main");
    commit_all(root, "record focused area");

    let bundle = serde_json::to_vec(&json!({
        "schema_version": 1,
        "area": "focused",
        "tasks": [{
            "key": "default", "title": "Default task", "afk": true, "priority": "high",
            "outcome": "The default task is complete.", "done_when": ["Default is done."],
            "validation": ["Check default."], "blocked_by": []
        }, {
            "key": "chosen", "title": "Chosen task", "afk": false, "priority": "normal",
            "outcome": "The chosen task is complete.", "done_when": ["Chosen is done."],
            "validation": ["Check chosen."], "blocked_by": []
        }]
    }))
    .expect("task bundle");
    json_output_with_stdin(
        root,
        &["tasks", "import", "focused", "--from", "-"],
        &bundle,
    );
    assert_eq!(
        json_output(root, &["next", "focused"])["task"]["id"],
        "focused-001"
    );

    let proposal = json!({
        "schema_version": 1,
        "proposal": "investigation_follow_up",
        "area": "focused",
        "source_task": "focused-002",
        "source_result": {
            "status": "complete",
            "summary": "Settled the focused question.",
            "validation": ["Checked the focused result."]
        },
        "tasks": [{
            "key": "follow-up", "title": "Implement the focused result", "blocked_by": [],
            "outcome": "The focused result is implemented.", "done_when": ["Implementation is done."],
            "validation": ["Check the implementation."]
        }]
    });
    let reviewed = json_output_with_stdin(
        root,
        &["tasks", "derive", "review", "focused", "--from", "-"],
        &derived_proposal_bytes("focused", "focused-002", proposal),
    );
    assert_eq!(reviewed["mechanically_eligible"], true);
}

#[test]
fn task_import_returns_complete_ready_frontier_from_standard_input() {
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
    assert_eq!(imported["ready"], json!(["stdin-001"]));
    assert!(
        root.join(".zdev/stdin/tasks/001-import-one-task.md")
            .exists()
    );

    let blocked = serde_json::to_vec(&json!({
        "schema_version": 1,
        "area": "stdin",
        "tasks": [{
            "key": "two",
            "title": "Wait for the existing task",
            "outcome": "The new task remains blocked while existing work is ready.",
            "done_when": ["The existing task remains the ready frontier."],
            "validation": ["Inspect the import result."],
            "blocked_by": ["stdin-001"]
        }]
    }))
    .expect("blocked task bundle");
    let imported =
        json_output_with_stdin(root, &["tasks", "import", "stdin", "--from", "-"], &blocked);

    assert_eq!(imported["tasks"], json!(["stdin-002"]));
    assert_eq!(imported["ready"], json!(["stdin-001"]));
}

#[test]
fn stored_task_review_can_be_shown_replaced_and_imported() {
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
    let status_before = git(root, &["status", "--short", "--untracked-files=all"]);

    let reviewed = json_output_with_stdin(
        root,
        &["tasks", "review", "approval", "--from", "-"],
        &bytes,
    );
    assert_eq!(reviewed["status"], "reviewed");
    assert_eq!(reviewed["area"], "approval");
    assert_eq!(reviewed.as_object().expect("review metadata").len(), 5);
    assert!(reviewed.get("approval").is_none());
    assert!(reviewed.get("markdown").is_none());
    let review = reviewed["review"].as_str().expect("review ID");
    assert_eq!(review, "Ra8f4067d67f0ef2b");
    let markdown_path = reported_path(root, &reviewed["path"]);
    assert_eq!(
        markdown_path.extension().and_then(|value| value.to_str()),
        Some("md")
    );
    assert!(!markdown_path.starts_with(root.join(".zdev")));
    let markdown = fs::read_to_string(&markdown_path).expect("stored review Markdown");
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
    let shown = run_zdev(root, &["tasks", "review", "approval", "--show"]);
    assert!(shown.status.success());
    assert_eq!(
        String::from_utf8(shown.stdout).expect("shown review"),
        format!("{markdown}\n")
    );
    let shown_json = json_output(root, &["tasks", "review", "approval", "--show"]);
    assert_eq!(shown_json["review"], review);
    assert_eq!(shown_json["markdown"], markdown);
    assert_eq!(
        serde_json::from_slice::<Value>(
            &fs::read(markdown_path.with_file_name("bundle.json")).expect("stored bundle")
        )
        .expect("canonical bundle JSON"),
        bundle
    );
    let metadata: Value = serde_json::from_slice(
        &fs::read(markdown_path.with_file_name("metadata.json")).expect("stored metadata"),
    )
    .expect("review metadata");
    assert_eq!(metadata["fingerprint"], "Ta8f4067d67f0ef2b");
    assert_eq!(
        git(root, &["status", "--short", "--untracked-files=all"]),
        status_before
    );

    let pretty = serde_json::to_vec_pretty(&bundle).expect("pretty task bundle");
    let reviewed_pretty = json_output_with_stdin(
        root,
        &["tasks", "review", "approval", "--from", "-"],
        &pretty,
    );
    assert_eq!(reviewed_pretty["review"], reviewed["review"]);
    assert_eq!(reviewed_pretty["path"], reviewed["path"]);

    let mut explicit_standard = bundle.clone();
    explicit_standard["tasks"][0]["complexity"] = json!("standard");
    let explicit_standard = serde_json::to_vec(&explicit_standard).expect("explicit standard");
    let reviewed_standard = json_output_with_stdin(
        root,
        &["tasks", "review", "approval", "--from", "-"],
        &explicit_standard,
    );
    let replacement = reviewed_standard["review"]
        .as_str()
        .expect("replacement review");
    assert_ne!(replacement, review);
    assert!(
        !markdown_path.exists(),
        "replaced review directory is pruned"
    );
    let replacement_markdown = fs::read_to_string(reported_path(root, &reviewed_standard["path"]))
        .expect("replacement Markdown");
    assert!(replacement_markdown.contains("### Complexity\nstandard"));

    let rejected = run_zdev(root, &["tasks", "import", "approval", "--reviewed", review]);
    assert!(!rejected.status.success());
    assert!(String::from_utf8_lossy(&rejected.stderr).contains("was replaced"));
    assert_eq!(
        fs::read_dir(root.join(".zdev/approval/tasks"))
            .expect("tasks directory")
            .count(),
        0
    );

    let imported = json_output(
        root,
        &["tasks", "import", "approval", "--reviewed", replacement],
    );
    assert_eq!(imported["tasks"], json!(["approval-001"]));
}

#[test]
fn stored_task_review_accepts_pre_selection_field_markdown() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "legacy-review",
            "--title",
            "Legacy review",
            "--objective",
            "Import a review created before selection fields existed.",
        ],
    );
    let bundle = serde_json::to_vec(&json!({
        "schema_version": 1,
        "area": "legacy-review",
        "tasks": [{
            "key": "one", "title": "Import the reviewed task", "blocked_by": [],
            "outcome": "The reviewed task is imported.", "done_when": ["The task exists."],
            "validation": ["Inspect the imported task."]
        }]
    }))
    .expect("task bundle");
    let reviewed = json_output_with_stdin(
        root,
        &["tasks", "review", "legacy-review", "--from", "-"],
        &bundle,
    );
    let markdown_path = reported_path(root, &reviewed["path"]);
    let current = fs::read_to_string(&markdown_path).expect("current review");
    let legacy = current.replace("\n\n### AFK\nfalse\n\n### Priority\nnormal", "");
    assert_ne!(legacy, current);
    fs::write(&markdown_path, &legacy).expect("legacy review Markdown");

    let shown = json_output(root, &["tasks", "review", "legacy-review", "--show"]);
    assert_eq!(shown["markdown"], legacy);
    let imported = json_output(
        root,
        &[
            "tasks",
            "import",
            "legacy-review",
            "--reviewed",
            reviewed["review"].as_str().expect("review ID"),
        ],
    );
    assert_eq!(imported["tasks"], json!(["legacy-review-001"]));
}

#[test]
fn stored_task_review_failures_publish_no_tasks() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    for area in ["missing", "source", "other"] {
        json_output(
            root,
            &[
                "area",
                "create",
                area,
                "--title",
                area,
                "--objective",
                "Exercise stored review failures.",
                "--trunk",
            ],
        );
    }
    let bundle = serde_json::to_vec(&json!({
        "schema_version": 1,
        "area": "source",
        "tasks": [{
            "key": "one", "title": "Keep review exact", "blocked_by": [],
            "outcome": "Invalid review state publishes nothing.",
            "done_when": ["No task is created on failure."],
            "validation": ["Exercise stored review errors."]
        }]
    }))
    .expect("task bundle");

    let missing = run_zdev(
        root,
        &[
            "tasks",
            "import",
            "missing",
            "--reviewed",
            "R0000000000000000",
        ],
    );
    assert!(!missing.status.success());
    assert!(String::from_utf8_lossy(&missing.stderr).contains("No stored task review"));

    let reviewed =
        json_output_with_stdin(root, &["tasks", "review", "source", "--from", "-"], &bundle);
    let markdown_path = reported_path(root, &reviewed["path"]);
    fs::write(&markdown_path, "changed\n").expect("corrupt review Markdown");
    let mismatch = run_zdev(root, &["tasks", "review", "source", "--show"]);
    assert!(!mismatch.status.success());
    assert!(String::from_utf8_lossy(&mismatch.stderr).contains("does not match its bundle"));

    let reviewed =
        json_output_with_stdin(root, &["tasks", "review", "source", "--from", "-"], &bundle);
    let source_path = reported_path(root, &reviewed["path"]);
    let review = reviewed["review"].as_str().expect("review ID");
    let other_store = git_path(root, "zdev/reviews/other");
    let other_directory = other_store.join(review);
    fs::create_dir_all(&other_directory).expect("other review directory");
    for name in ["bundle.json", "metadata.json", "review.md"] {
        fs::copy(source_path.with_file_name(name), other_directory.join(name))
            .expect("copy cross-area review");
    }
    fs::write(other_store.join("current"), format!("{review}\n")).expect("other current pointer");
    let cross_area = run_zdev(root, &["tasks", "review", "other", "--show"]);
    assert!(!cross_area.status.success());
    assert!(String::from_utf8_lossy(&cross_area.stderr).contains("does not match selected area"));

    fs::write(source_path.with_file_name("metadata.json"), b"{").expect("corrupt metadata");
    let corrupt = run_zdev(root, &["tasks", "import", "source", "--reviewed", review]);
    assert!(!corrupt.status.success());
    assert!(String::from_utf8_lossy(&corrupt.stderr).contains("is corrupt"));
    for area in ["missing", "source", "other"] {
        assert_eq!(
            fs::read_dir(root.join(format!(".zdev/{area}/tasks")))
                .expect("tasks directory")
                .count(),
            0
        );
    }
}

#[test]
fn stored_task_review_uses_the_linked_worktree_git_path() {
    let repository = repository();
    let root = repository.path();
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "linked",
            "--title",
            "Linked",
            "--objective",
            "Store reviews in linked-worktree Git state.",
        ],
    );
    commit_all(root, "record zdev area");
    let linked_parent = tempfile::tempdir().expect("linked parent");
    let linked = linked_parent.path().join("checkout");
    git(
        root,
        &[
            "worktree",
            "add",
            "-q",
            "-b",
            "linked-review",
            linked.to_str().expect("linked path"),
        ],
    );
    let bundle = serde_json::to_vec(&json!({
        "schema_version": 1,
        "area": "linked",
        "tasks": [{
            "key": "one", "title": "Review from linked worktree", "blocked_by": [],
            "outcome": "The review uses this worktree's Git administrative path.",
            "done_when": ["The Markdown exists outside tracked state."],
            "validation": ["Inspect git rev-parse --git-path."]
        }]
    }))
    .expect("linked bundle");
    let reviewed = json_output_with_stdin(
        &linked,
        &["tasks", "review", "linked", "--from", "-"],
        &bundle,
    );
    let expected = git_path(&linked, "zdev/reviews/linked")
        .join(reviewed["review"].as_str().expect("review ID"))
        .join("review.md");
    assert_eq!(reported_path(&linked, &reviewed["path"]), expected);
    assert!(expected.is_file());
    assert!(git(&linked, &["status", "--short", "--untracked-files=all"]).is_empty());
}

fn derived_proposal_bytes(area: &str, source_task: &str, proposal: Value) -> Vec<u8> {
    let mut bytes = format!("PROPOSE zdev-derived {area} {source_task}\n").into_bytes();
    bytes.extend(serde_json::to_vec(&proposal).expect("derived proposal"));
    bytes
}

fn investigation_follow_up(area: &str, source_task: &str, key: &str, title: &str) -> Value {
    json!({
        "schema_version": 1,
        "proposal": "investigation_follow_up",
        "area": area,
        "source_task": source_task,
        "source_result": {
            "status": "complete",
            "summary": "Settled the investigation.",
            "validation": ["Checked the result."]
        },
        "tasks": [{
            "key": key,
            "title": title,
            "blocked_by": [],
            "outcome": "The follow-up is complete.",
            "done_when": ["The follow-up is implemented."],
            "validation": ["Check the follow-up."]
        }]
    })
}

#[test]
fn stored_derived_review_replaces_accepts_legacy_markdown_and_applies() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "stored-derived", "main");
    import_one_task(root, "stored-derived");
    commit_all(root, "record source");

    let first_input = derived_proposal_bytes(
        "stored-derived",
        "stored-derived-001",
        investigation_follow_up(
            "stored-derived",
            "stored-derived-001",
            "first-child",
            "Implement first child",
        ),
    );
    let first = json_output_with_stdin(
        root,
        &["tasks", "derive", "review", "stored-derived", "--from", "-"],
        &first_input,
    );
    assert_eq!(first.as_object().unwrap().len(), 9);
    assert!(first.get("approval").is_none());
    assert!(first.get("envelope").is_none());
    assert!(first.get("markdown").is_none());
    let first_review = first["review"].as_str().unwrap();

    let second_input = derived_proposal_bytes(
        "stored-derived",
        "stored-derived-001",
        investigation_follow_up(
            "stored-derived",
            "stored-derived-001",
            "second-child",
            "Implement second child",
        ),
    );
    let second = json_output_with_stdin(
        root,
        &["tasks", "derive", "review", "stored-derived", "--from", "-"],
        &second_input,
    );
    let second_review = second["review"].as_str().unwrap();
    assert_ne!(first_review, second_review);
    let rejected = run_zdev(
        root,
        &[
            "tasks",
            "derive",
            "apply",
            "stored-derived",
            "--reviewed",
            first_review,
        ],
    );
    assert!(!rejected.status.success());
    assert!(String::from_utf8_lossy(&rejected.stderr).contains("was replaced"));
    assert_eq!(
        fs::read_dir(root.join(".zdev/stored-derived/tasks"))
            .unwrap()
            .count(),
        1
    );

    let markdown_path = reported_path(root, &second["path"]);
    let current = fs::read_to_string(&markdown_path).expect("current derived review");
    let legacy = current.replace("\n\n### AFK\nfalse\n\n### Priority\nnormal", "");
    assert_ne!(legacy, current);
    fs::write(&markdown_path, &legacy).expect("legacy derived review Markdown");
    let shown = run_zdev(
        root,
        &["tasks", "derive", "review", "stored-derived", "--show"],
    );
    assert!(shown.status.success());
    assert_eq!(
        String::from_utf8(shown.stdout).unwrap(),
        format!("{legacy}\n")
    );

    let applied = json_output(
        root,
        &[
            "tasks",
            "derive",
            "apply",
            "stored-derived",
            "--reviewed",
            second_review,
        ],
    );
    assert_eq!(applied["source_task"], "stored-derived-001");
    assert_eq!(applied["tasks"], json!(["stored-derived-002"]));
    assert!(
        root.join(".zdev/stored-derived/tasks/002-implement-second-child.md")
            .is_file()
    );
}

#[test]
fn stored_derived_review_rejects_missing_cross_area_corrupt_and_wrong_source_state() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "source-derived", "main");
    create_area(root, "other-derived", "other-derived-work");
    import_one_task(root, "source-derived");
    import_one_task(root, "other-derived");
    commit_all(root, "record sources");
    let input = derived_proposal_bytes(
        "source-derived",
        "source-derived-001",
        investigation_follow_up(
            "source-derived",
            "source-derived-001",
            "child",
            "Implement child",
        ),
    );
    let reviewed = json_output_with_stdin(
        root,
        &["tasks", "derive", "review", "source-derived", "--from", "-"],
        &input,
    );
    let review = reviewed["review"].as_str().unwrap();
    let cross_area = run_zdev(
        root,
        &[
            "tasks",
            "derive",
            "apply",
            "other-derived",
            "--reviewed",
            review,
        ],
    );
    assert!(!cross_area.status.success());
    assert!(String::from_utf8_lossy(&cross_area.stderr).contains("No stored derived review"));

    let directory = reported_path(root, &reviewed["path"])
        .parent()
        .unwrap()
        .to_path_buf();
    let metadata_path = directory.join("metadata.json");
    let metadata_before = fs::read(&metadata_path).unwrap();
    let mut metadata: Value = serde_json::from_slice(&metadata_before).unwrap();
    metadata["source_task"] = json!("source-derived-999");
    fs::write(&metadata_path, serde_json::to_vec(&metadata).unwrap()).unwrap();
    let wrong_source = run_zdev(
        root,
        &[
            "tasks",
            "derive",
            "apply",
            "source-derived",
            "--reviewed",
            review,
        ],
    );
    assert!(!wrong_source.status.success());
    assert!(String::from_utf8_lossy(&wrong_source.stderr).contains("does not match its proposal"));
    fs::write(&metadata_path, metadata_before).unwrap();
    fs::write(directory.join("review.md"), "corrupt\n").unwrap();
    let corrupt = run_zdev(
        root,
        &[
            "tasks",
            "derive",
            "apply",
            "source-derived",
            "--reviewed",
            review,
        ],
    );
    assert!(!corrupt.status.success());
    assert!(String::from_utf8_lossy(&corrupt.stderr).contains("does not match its proposal"));
    assert_eq!(
        fs::read_dir(root.join(".zdev/source-derived/tasks"))
            .unwrap()
            .count(),
        1
    );

    let refreshed = json_output_with_stdin(
        root,
        &["tasks", "derive", "review", "source-derived", "--from", "-"],
        &input,
    );
    let refreshed_directory = reported_path(root, &refreshed["path"])
        .parent()
        .unwrap()
        .to_path_buf();
    let refreshed_metadata: Value =
        serde_json::from_slice(&fs::read(refreshed_directory.join("metadata.json")).unwrap())
            .unwrap();
    let compatible = json_output_with_stdin(
        root,
        &[
            "tasks",
            "derive",
            "apply",
            "source-derived",
            "--from",
            "-",
            "--approval",
            refreshed_metadata["fingerprint"].as_str().unwrap(),
        ],
        &input,
    );
    assert_eq!(compatible["tasks"], json!(["source-derived-002"]));
}

#[test]
fn stored_derived_review_uses_the_linked_worktree_git_path() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "linked-derived", "main");
    import_one_task(root, "linked-derived");
    commit_all(root, "record source");
    let linked_parent = tempfile::tempdir().expect("linked parent");
    let linked = linked_parent.path().join("checkout");
    git(
        root,
        &[
            "worktree",
            "add",
            "-q",
            "-b",
            "linked-derived-review",
            linked.to_str().unwrap(),
        ],
    );
    let input = derived_proposal_bytes(
        "linked-derived",
        "linked-derived-001",
        investigation_follow_up(
            "linked-derived",
            "linked-derived-001",
            "child",
            "Implement child",
        ),
    );
    let reviewed = json_output_with_stdin(
        &linked,
        &["tasks", "derive", "review", "linked-derived", "--from", "-"],
        &input,
    );
    let expected = git_path(&linked, "zdev/derived-reviews/linked-derived")
        .join(reviewed["review"].as_str().unwrap())
        .join("review.md");
    assert_eq!(reported_path(&linked, &reviewed["path"]), expected);
    assert!(expected.is_file());
    assert!(git(&linked, &["status", "--short", "--untracked-files=all"]).is_empty());
}

#[test]
fn derived_review_accepts_follow_up_and_exact_post_edit_split_ownership() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "derived",
            "--title",
            "Derived",
            "--objective",
            "Review direct derived work.",
        ],
    );
    create_slice(root, "derived", "focused", "Focused derived work");
    let source = json!({
        "schema_version": 1,
        "area": "derived",
        "tasks": [{
            "key": "investigate",
            "title": "Investigate the boundary",
            "outcome": "The implementation boundary is settled.",
            "done_when": ["The boundary is recorded."],
            "validation": ["Compare it with the implementation."],
            "blocked_by": []
        }]
    });
    json_output_with_stdin(
        root,
        &["tasks", "import", "derived", "--from", "-"],
        &serde_json::to_vec(&source).expect("source bundle"),
    );

    let follow_up = json!({
        "schema_version": 1,
        "proposal": "investigation_follow_up",
        "area": "derived",
        "source_task": "derived-001",
        "source_result": {
            "status": "complete",
            "summary": "Settled the implementation boundary.",
            "validation": ["Compared it with the implementation."]
        },
        "tasks": [{
            "key": "implement",
            "title": "Implement the settled boundary",
            "complexity": "advanced",
            "slice": "focused",
            "blocked_by": [],
            "outcome": "The implementation uses the settled boundary.",
            "context": "The investigation removed the design uncertainty.",
            "boundaries": ["Keep the public behavior unchanged."],
            "done_when": ["The settled boundary is implemented."],
            "validation": ["Run the focused behavior checks."]
        }]
    });
    let follow_up_input = derived_proposal_bytes("derived", "derived-001", follow_up);
    let reviewed = json_output_with_stdin(
        root,
        &["tasks", "derive", "review", "derived", "--from", "-"],
        &follow_up_input,
    );
    assert_eq!(reviewed["mechanically_eligible"], true);
    assert_eq!(reviewed["coordinator_authority_required"], true);
    assert!(reviewed["review"].as_str().is_some());
    assert!(reviewed.get("approval").is_none());
    assert!(reviewed.get("envelope").is_none());
    assert!(reviewed.get("markdown").is_none());
    let markdown_path = reported_path(root, &reviewed["path"]);
    assert!(
        fs::read_to_string(&markdown_path)
            .unwrap()
            .contains("### Complexity\nadvanced")
    );
    assert_eq!(
        fs::read(markdown_path.parent().unwrap().join("proposal.json")).unwrap(),
        follow_up_input
            .splitn(2, |byte| *byte == b'\n')
            .nth(1)
            .unwrap()
    );
    let shown = run_zdev(root, &["tasks", "derive", "review", "derived", "--show"]);
    assert!(shown.status.success());
    assert_eq!(
        String::from_utf8(shown.stdout).unwrap(),
        format!("{}\n", fs::read_to_string(markdown_path).unwrap())
    );

    commit_all(root, "record source task");
    fs::write(root.join("parent.rs"), "retained parent work\n").expect("edit parent path");
    git(root, &["add", "parent.rs"]);
    git(root, &["commit", "-q", "-m", "add parent path"]);
    fs::write(root.join("parent.rs"), "retained edited parent work\n").expect("retain edit");
    let split = json!({
        "schema_version": 1,
        "proposal": "implementation_split",
        "area": "derived",
        "source_task": "derived-001",
        "source_result": {
            "status": "split",
            "summary": "Separated disjoint implementation work.",
            "validation": []
        },
        "tasks": [{
            "key": "child-a",
            "title": "Implement child A",
            "blocked_by": [],
            "outcome": "Child A owns its exact future file.",
            "done_when": ["Child A is complete."],
            "validation": ["Check child A."]
        }, {
            "key": "child-b",
            "title": "Implement child B",
            "blocked_by": ["child-a"],
            "outcome": "Child B owns its exact future file.",
            "done_when": ["Child B is complete."],
            "validation": ["Check child B."]
        }],
        "split_ownership": {
            "retained_parent_paths": ["parent.rs"],
            "child_future_paths": [
                {"key": "child-a", "paths": ["child_a.rs"]},
                {"key": "child-b", "paths": ["child_b.rs"]}
            ]
        }
    });
    let before = git_stdout(root, &["status", "--porcelain=v1", "--untracked-files=all"]);
    let reviewed = json_output_with_stdin(
        root,
        &["tasks", "derive", "review", "derived", "--from", "-"],
        &derived_proposal_bytes("derived", "derived-001", split),
    );
    assert_eq!(reviewed["mechanically_eligible"], true);
    assert_eq!(
        git_stdout(root, &["status", "--porcelain=v1", "--untracked-files=all"]),
        before
    );
}

#[test]
fn derived_review_rejects_malformed_nested_duplicate_and_mismatched_proposals() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "strict",
            "--title",
            "Strict",
            "--objective",
            "Reject malformed derived proposals.",
        ],
    );
    import_one_task(root, "strict");
    let valid = json!({
        "schema_version": 1,
        "proposal": "investigation_follow_up",
        "area": "strict",
        "source_task": "strict-001",
        "source_result": {"status": "complete", "summary": "Done.", "validation": ["Checked."]},
        "tasks": [{
            "key": "next", "title": "Do the next task", "blocked_by": [],
            "outcome": "The next task is complete.", "done_when": ["It is complete."],
            "validation": ["Check it."]
        }]
    });
    let mut cases = Vec::new();
    let mut unknown = valid.clone();
    unknown["tasks"][0]["proposal"] = json!({"nested": true});
    cases.push(derived_proposal_bytes("strict", "strict-001", unknown));
    let mut oversized = valid.clone();
    oversized["tasks"] = Value::Array(vec![valid["tasks"][0].clone(); 6]);
    cases.push(derived_proposal_bytes("strict", "strict-001", oversized));
    cases.push(derived_proposal_bytes(
        "strict",
        "strict-999",
        valid.clone(),
    ));
    cases.push(
        "PROPOSE zdev-derived strict strict-001\n{\"schema_version\":1,\"schema_version\":1,\"proposal\":\"investigation_follow_up\",\"area\":\"strict\",\"source_task\":\"strict-001\",\"source_result\":{\"status\":\"complete\",\"summary\":\"Done.\",\"validation\":[\"Checked.\"]},\"tasks\":[]}"
            .as_bytes()
            .to_vec(),
    );
    for input in cases {
        let rejected = json_output_with_stdin_status(
            root,
            &["tasks", "derive", "review", "strict", "--from", "-"],
            &input,
        );
        assert!(!rejected.status.success());
    }
}

#[test]
fn derived_review_reports_unsafe_or_ambiguous_split_as_mechanically_ineligible() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "parent.rs", "parent\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "manual", "work");
    import_one_task(root, "manual");
    commit_all(root, "record task");
    let split = json!({
        "schema_version": 1,
        "proposal": "implementation_split",
        "area": "manual",
        "source_task": "manual-001",
        "source_result": {"status": "split", "summary": "Split direct work.", "validation": []},
        "tasks": [{
            "key": "child", "title": "Implement the child", "blocked_by": [],
            "outcome": "The child owns future work.", "done_when": ["The child is complete."],
            "validation": ["Check the child."]
        }],
        "split_ownership": {
            "retained_parent_paths": ["parent.rs"],
            "child_future_paths": [{"key": "child", "paths": ["child.rs"]}]
        }
    });
    let input = derived_proposal_bytes("manual", "manual-001", split);

    let wrong_branch = json_output_with_stdin(
        root,
        &["tasks", "derive", "review", "manual", "--from", "-"],
        &input,
    );
    assert_eq!(wrong_branch["mechanically_eligible"], false);
    assert!(wrong_branch["review"].as_str().is_some());
    assert!(wrong_branch.get("approval").is_none());
    assert!(wrong_branch.get("markdown").is_none());
    let shown = run_zdev(root, &["tasks", "derive", "review", "manual", "--show"]);
    assert!(String::from_utf8_lossy(&shown.stdout).contains("# Task Bundle"));

    git(root, &["switch", "-q", "-c", "work"]);
    json_output(root, &["area", "rebase", "manual"]);
    fs::write(root.join("parent.rs"), "edited parent\n").expect("edit parent");
    git(root, &["add", "parent.rs"]);
    let staged = json_output_with_stdin(
        root,
        &["tasks", "derive", "review", "manual", "--from", "-"],
        &input,
    );
    assert_eq!(staged["mechanically_eligible"], false);
    assert!(
        staged["reason"]
            .as_str()
            .expect("reason")
            .contains("index is not empty"),
        "{staged}"
    );
    git(root, &["reset", "-q", "HEAD", "--", "parent.rs"]);
    fs::write(root.join("unrelated.txt"), "unrelated\n").expect("unrelated path");
    let incomplete = json_output_with_stdin(
        root,
        &["tasks", "derive", "review", "manual", "--from", "-"],
        &input,
    );
    assert_eq!(incomplete["mechanically_eligible"], false);
    assert!(
        incomplete["reason"]
            .as_str()
            .expect("reason")
            .contains("complete unstaged path set")
    );
    assert_ne!(
        git_stdout(root, &["status", "--porcelain=v1", "--untracked-files=all"]),
        ""
    );
    assert_eq!(
        fs::read_dir(root.join(".zdev/manual/tasks"))
            .expect("tasks")
            .count(),
        1
    );
}

#[test]
fn derived_apply_commits_investigation_result_and_children_together() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "finding.md", "draft\n", "seed finding");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "follow", "main");
    create_slice(root, "follow", "focus", "Focused work");
    create_slice(root, "follow", "delivery", "Delivery work");
    let source = json!({
        "schema_version": 1, "area": "follow", "tasks": [{
            "key": "investigate", "title": "Investigate the boundary", "slice": "focus",
            "outcome": "The boundary is known.", "done_when": ["The boundary is recorded."],
            "validation": ["Check the finding."], "blocked_by": []
        }]
    });
    json_output_with_stdin(
        root,
        &["tasks", "import", "follow", "--from", "-"],
        &serde_json::to_vec(&source).unwrap(),
    );
    commit_all(root, "record investigation");
    fs::write(root.join("finding.md"), "settled\n").expect("settled finding");
    git(root, &["add", "finding.md"]);
    let proposal = json!({
        "schema_version": 1, "proposal": "investigation_follow_up", "area": "follow",
        "source_task": "follow-001",
        "source_result": {"status": "complete", "summary": "Settled the boundary.", "validation": ["Checked the finding."]},
        "tasks": [{
            "key": "implement", "title": "Implement the boundary", "slice": "delivery", "blocked_by": [],
            "outcome": "The boundary is implemented.", "done_when": ["The implementation is complete."],
            "validation": ["Run the focused check."]
        }]
    });
    let applied = json_output_with_stdin(
        root,
        &["tasks", "derive", "apply", "follow", "--from", "-"],
        &derived_proposal_bytes("follow", "follow-001", proposal),
    );
    assert_eq!(applied["source_task"], "follow-001");
    assert_eq!(applied["tasks"], json!(["follow-002"]));
    assert_eq!(applied["ready"], json!(["follow-002"]));
    assert_eq!(git(root, &["status", "--porcelain"]), "");
    assert_eq!(git(root, &["rev-parse", "HEAD"]), applied["commit"]);
    let source =
        fs::read_to_string(root.join(".zdev/follow/tasks/001-investigate-the-boundary.md"))
            .unwrap();
    assert!(source.contains("status = \"done\""));
    assert!(source.contains("## Result\n\nSettled the boundary."));
    let child =
        fs::read_to_string(root.join(".zdev/follow/tasks/002-implement-the-boundary.md")).unwrap();
    assert!(child.contains("slice = \"delivery\""));
}

#[test]
fn derived_apply_rejects_control_ownership_and_staged_managed_state() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "guarded", "main");
    create_area(root, "other", "other");
    import_one_task(root, "guarded");
    commit_all(root, "record guarded source");

    let split = |retained: Vec<&str>, future: Vec<&str>| {
        json!({
            "schema_version": 1, "proposal": "implementation_split", "area": "guarded",
            "source_task": "guarded-001",
            "source_result": {"status": "split", "summary": "Split guarded work.", "validation": []},
            "tasks": [{"key": "child", "title": "Implement child", "blocked_by": [], "outcome": "Child is complete.", "done_when": ["Done."], "validation": ["Check."]}],
            "split_ownership": {"retained_parent_paths": retained, "child_future_paths": [{"key": "child", "paths": future}]}
        })
    };
    for proposal in [
        split(vec![], vec![".git/derived-child.rs"]),
        split(vec![".zdev/other/brief.md"], vec!["child.rs"]),
    ] {
        let rejected = json_output_with_stdin_status(
            root,
            &["tasks", "derive", "apply", "guarded", "--from", "-"],
            &derived_proposal_bytes("guarded", "guarded-001", proposal),
        );
        assert!(!rejected.status.success());
        assert!(
            String::from_utf8_lossy(&rejected.stderr)
                .contains("repository control or zdev managed state")
        );
    }

    let other_brief = root.join(".zdev/other/brief.md");
    let mut contents = fs::read_to_string(&other_brief).unwrap();
    contents.push_str("\nStaged managed state.\n");
    fs::write(&other_brief, contents).unwrap();
    git(root, &["add", ".zdev/other/brief.md"]);
    let status_before = git_stdout(root, &["status", "--porcelain=v1", "--untracked-files=all"]);
    let follow_up = json!({
        "schema_version": 1, "proposal": "investigation_follow_up", "area": "guarded",
        "source_task": "guarded-001",
        "source_result": {"status": "complete", "summary": "Settled guarded work.", "validation": ["Checked it."]},
        "tasks": [{"key": "next", "title": "Implement next", "blocked_by": [], "outcome": "Next is complete.", "done_when": ["Done."], "validation": ["Check."]}]
    });
    let rejected = json_output_with_stdin_status(
        root,
        &["tasks", "derive", "apply", "guarded", "--from", "-"],
        &derived_proposal_bytes("guarded", "guarded-001", follow_up),
    );
    assert!(!rejected.status.success());
    assert!(String::from_utf8_lossy(&rejected.stderr).contains("zdev managed state"));
    assert_eq!(
        git_stdout(root, &["status", "--porcelain=v1", "--untracked-files=all"]),
        status_before
    );
    assert_eq!(
        fs::read_dir(root.join(".zdev/guarded/tasks"))
            .unwrap()
            .count(),
        1
    );
}

#[test]
fn derived_apply_split_preserves_retained_bytes_and_uses_ordinary_ready_order() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "parent.rs", "before\n", "seed parent");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "split", "main");
    import_one_task(root, "split");
    commit_all(root, "record split source");
    fs::create_dir(root.join("src")).expect("src directory");
    fs::create_dir(root.join("tests")).expect("tests directory");
    fs::write(root.join("parent.rs"), "retained parent edit\n").expect("parent edit");
    let proposal = json!({
        "schema_version": 1, "proposal": "implementation_split", "area": "split",
        "source_task": "split-001",
        "source_result": {"status": "split", "summary": "Separated the work.", "validation": []},
        "tasks": [{
            "key": "first", "title": "Implement first child", "blocked_by": [],
            "outcome": "The first child is complete.", "done_when": ["First is complete."], "validation": ["Check first."]
        }, {
            "key": "second", "title": "Implement second child", "blocked_by": ["first"],
            "outcome": "The second child is complete.", "done_when": ["Second is complete."], "validation": ["Check second."]
        }],
        "split_ownership": {
            "retained_parent_paths": ["parent.rs"],
            "child_future_paths": [
                {"key": "first", "paths": ["src/first.rs"]},
                {"key": "second", "paths": ["src/second.rs", "tests/second.rs"]}
            ]
        }
    });
    let applied = json_output_with_stdin(
        root,
        &["tasks", "derive", "apply", "split", "--from", "-"],
        &derived_proposal_bytes("split", "split-001", proposal),
    );
    assert_eq!(applied["tasks"], json!(["split-002", "split-003"]));
    assert_eq!(applied["ready"], json!(["split-002"]));
    assert_eq!(
        fs::read(root.join("parent.rs")).unwrap(),
        b"retained parent edit\n"
    );
    assert_eq!(git(root, &["diff", "--name-only"]), "parent.rs");
    assert_eq!(git(root, &["diff", "--cached", "--name-only"]), "");
    let source =
        fs::read_to_string(root.join(".zdev/split/tasks/001-complete-one-task.md")).unwrap();
    assert!(source.contains("status = \"open\""));
    assert!(source.contains("blocked_by = [\"split-002\", \"split-003\"]"));
    let child =
        fs::read_to_string(root.join(".zdev/split/tasks/003-implement-second-child.md")).unwrap();
    assert!(child.contains("Task-owned paths (exact): [\"src/second.rs\",\"tests/second.rs\"]"));
}

#[test]
fn derived_apply_accepts_a_pre_edit_split_with_no_retained_delta() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "pre-edit", "main");
    import_one_task(root, "pre-edit");
    commit_all(root, "record source");
    let proposal = json!({
        "schema_version": 1, "proposal": "implementation_split", "area": "pre-edit",
        "source_task": "pre-edit-001",
        "source_result": {"status": "split", "summary": "Split before edits.", "validation": []},
        "tasks": [{"key": "child", "title": "Implement child", "blocked_by": [], "outcome": "Child is complete.", "done_when": ["Done."], "validation": ["Check."]}],
        "split_ownership": {"retained_parent_paths": [], "child_future_paths": [{"key": "child", "paths": ["child.rs"]}]}
    });
    let applied = json_output_with_stdin(
        root,
        &["tasks", "derive", "apply", "pre-edit", "--from", "-"],
        &derived_proposal_bytes("pre-edit", "pre-edit-001", proposal),
    );
    assert_eq!(applied["tasks"], json!(["pre-edit-002"]));
    assert_eq!(applied["ready"], json!(["pre-edit-002"]));
    assert_eq!(git(root, &["status", "--porcelain"]), "");
}

#[cfg(unix)]
#[test]
fn failed_derived_apply_restores_tasks_index_and_retained_bytes() {
    use std::os::unix::fs::PermissionsExt;

    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "parent.rs", "before\n", "seed parent");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "rollback-derived", "main");
    import_one_task(root, "rollback-derived");
    commit_all(root, "record source");
    fs::write(root.join("parent.rs"), "retained\n").expect("parent edit");
    let source_path = root.join(".zdev/rollback-derived/tasks/001-complete-one-task.md");
    let index_path = root.join(".zdev/rollback-derived/TASKS.md");
    let source_before = fs::read(&source_path).unwrap();
    let index_before = fs::read(&index_path).unwrap();
    let status_before = git_stdout(root, &["status", "--porcelain=v1", "--untracked-files=all"]);
    let hook = root.join(".git/hooks/pre-commit");
    fs::write(&hook, "#!/bin/sh\nexit 1\n").expect("hook");
    let mut permissions = fs::metadata(&hook).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&hook, permissions).unwrap();
    let proposal = json!({
        "schema_version": 1, "proposal": "implementation_split", "area": "rollback-derived",
        "source_task": "rollback-derived-001",
        "source_result": {"status": "split", "summary": "Split it.", "validation": []},
        "tasks": [{"key": "child", "title": "Implement child", "blocked_by": [], "outcome": "Child is complete.", "done_when": ["Done."], "validation": ["Check."]}],
        "split_ownership": {"retained_parent_paths": ["parent.rs"], "child_future_paths": [{"key": "child", "paths": ["child.rs"]}]}
    });
    let proposal_input =
        derived_proposal_bytes("rollback-derived", "rollback-derived-001", proposal);
    let reviewed = json_output_with_stdin(
        root,
        &[
            "tasks",
            "derive",
            "review",
            "rollback-derived",
            "--from",
            "-",
        ],
        &proposal_input,
    );
    let rejected = run_zdev(
        root,
        &[
            "tasks",
            "derive",
            "apply",
            "rollback-derived",
            "--reviewed",
            reviewed["review"].as_str().unwrap(),
        ],
    );
    assert!(!rejected.status.success());
    assert_eq!(fs::read(&source_path).unwrap(), source_before);
    assert_eq!(fs::read(&index_path).unwrap(), index_before);
    assert!(
        !root
            .join(".zdev/rollback-derived/tasks/002-implement-child.md")
            .exists()
    );
    assert_eq!(fs::read(root.join("parent.rs")).unwrap(), b"retained\n");
    assert_eq!(
        git_stdout(root, &["status", "--porcelain=v1", "--untracked-files=all"]),
        status_before
    );
}

#[test]
fn explicit_task_complexity_round_trips_and_invalid_values_fail_closed() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "complexity", "main");
    commit_all(root, "record area");

    let bundle = json!({
        "schema_version": 1,
        "area": "complexity",
        "tasks": [{
            "key": "advanced",
            "title": "Implement advanced work",
            "complexity": "advanced",
            "blocked_by": [],
            "outcome": "Advanced work retains its authored routing level.",
            "done_when": ["Every task projection reports advanced."],
            "validation": ["Exercise task projections."]
        }]
    });
    let bytes = serde_json::to_vec(&bundle).expect("task bundle");
    let reviewed = json_output_with_stdin(
        root,
        &["tasks", "review", "complexity", "--from", "-"],
        &bytes,
    );
    let shown = json_output(root, &["tasks", "review", "complexity", "--show"]);
    assert!(
        shown["markdown"]
            .as_str()
            .expect("review markdown")
            .contains("### Complexity\nadvanced")
    );
    let review = reviewed["review"].as_str().expect("review identity");
    json_output(
        root,
        &["tasks", "import", "complexity", "--reviewed", review],
    );

    let task_path = root.join(".zdev/complexity/tasks/001-implement-advanced-work.md");
    let authored = fs::read_to_string(&task_path).expect("advanced task");
    assert!(authored.contains("complexity = \"advanced\""));
    assert_eq!(
        json_output(root, &["tasks", "list", "complexity"])["tasks"][0]["complexity"],
        "advanced"
    );
    assert_eq!(
        json_output(root, &["task", "show", "complexity", "complexity-001"])["complexity"],
        "advanced"
    );
    assert_eq!(
        json_output(root, &["next", "complexity"])["task"]["complexity"],
        "advanced"
    );
    assert_eq!(
        json_output(root, &["status", "complexity"])["next_complexity"],
        "advanced"
    );
    let project_status = run_zdev(root, &["status"]);
    let project_status = String::from_utf8_lossy(&project_status.stdout);
    assert!(
        project_status.contains(
            "complexity: open, ready; next complexity-001 (complexity: advanced); main -> trunk main ["
        ),
        "{project_status}"
    );
    assert_eq!(
        json_output(root, &["goal", "complexity"])["task"]["complexity"],
        "advanced"
    );
    for (arguments, expected) in [
        (vec!["tasks", "list", "complexity"], "complexity:advanced"),
        (
            vec!["task", "show", "complexity", "complexity-001"],
            "Complexity: advanced",
        ),
        (vec!["next", "complexity"], "Complexity: advanced"),
        (vec!["status", "complexity"], "complexity: advanced"),
        (vec!["goal", "complexity"], "Complexity: advanced"),
    ] {
        let output = run_zdev(root, &arguments);
        assert!(
            String::from_utf8_lossy(&output.stdout).contains(expected),
            "missing {expected} from {arguments:?}"
        );
    }
    assert_eq!(json_output(root, &["check", "complexity"])["status"], "ok");

    let mut invalid = bundle.clone();
    invalid["tasks"][0]["complexity"] = json!("complex");
    let invalid = serde_json::to_vec(&invalid).expect("invalid bundle");
    let rejected = json_output_with_stdin_status(
        root,
        &["tasks", "review", "complexity", "--from", "-"],
        &invalid,
    );
    assert!(!rejected.status.success());
    let error = String::from_utf8_lossy(&rejected.stderr);
    assert!(error.contains("unknown variant `complex`"));
    assert!(error.contains("`routine`, `standard`, `advanced`"));

    let malformed = authored.replace("complexity = \"advanced\"", "complexity = \"complex\"");
    fs::write(&task_path, malformed).expect("invalid task complexity");
    let rejected = run_zdev(root, &["check", "complexity"]);
    assert!(!rejected.status.success());
    assert!(String::from_utf8_lossy(&rejected.stderr).contains("unknown variant `complex`"));

    fs::write(
        &task_path,
        authored.replace("status = \"open\"", "status = \"open\"\npoints = 3"),
    )
    .expect("unknown task field");
    let rejected = run_zdev(root, &["check", "complexity"]);
    assert!(!rejected.status.success());
    assert!(String::from_utf8_lossy(&rejected.stderr).contains("unknown field `points`"));
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
fn first_committed_task_import_publishes_complete_project_record() {
    let repository = repository();
    let root = repository.path();
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "initial",
            "--title",
            "Initial",
            "--objective",
            "Publish the first task bundle.",
        ],
    );
    create_slice(root, "initial", "focused", "Focused");
    let bundle = serde_json::to_vec(&json!({
        "schema_version": 1,
        "area": "initial",
        "tasks": [{
            "key": "one",
            "title": "Commit initial task",
            "slice": "focused",
            "outcome": "Planning state is durable.",
            "done_when": ["The task is committed."],
            "blocked_by": []
        }]
    }))
    .expect("bundle");

    let imported = json_output_with_stdin(
        root,
        &["tasks", "import", "initial", "--from", "-", "--commit"],
        &bundle,
    );

    assert_eq!(imported["status"], "committed");
    assert_eq!(
        imported["paths"],
        json!([
            ".zdev/config.toml",
            ".zdev/initial/area.toml",
            ".zdev/initial/brief.md",
            ".zdev/initial/slices/focused.md",
            ".zdev/initial/tasks/001-commit-initial-task.md",
            ".zdev/initial/TASKS.md"
        ])
    );
    assert_eq!(
        git(root, &["show", "--pretty=format:", "--name-only", "HEAD"])
            .lines()
            .collect::<Vec<_>>(),
        [
            ".zdev/config.toml",
            ".zdev/initial/TASKS.md",
            ".zdev/initial/area.toml",
            ".zdev/initial/brief.md",
            ".zdev/initial/slices/focused.md",
            ".zdev/initial/tasks/001-commit-initial-task.md",
        ]
    );
    for path in [
        ".zdev/config.toml",
        ".zdev/initial/area.toml",
        ".zdev/initial/brief.md",
        ".zdev/initial/slices/focused.md",
        ".zdev/initial/tasks/001-commit-initial-task.md",
        ".zdev/initial/TASKS.md",
    ] {
        assert!(!git(root, &["show", &format!("HEAD:{path}")]).is_empty());
    }
    assert!(git(root, &["status", "--short", "--", ".zdev"]).is_empty());
}

#[test]
fn first_committed_task_import_accepts_tracked_config_and_pull_request_records() {
    for (record, track_config) in [
        ("project", true),
        ("pull-request", true),
        ("pull-request", false),
    ] {
        let repository = repository();
        let root = repository.path();
        commit_file(root, "seed.txt", "seed\n", "seed");
        json_output(root, &["init", "--record", record]);
        if track_config {
            git(root, &["add", ".zdev/config.toml"]);
            git(root, &["commit", "-m", "track zdev config"]);
        }
        json_output(
            root,
            &[
                "area",
                "create",
                "fresh",
                "--title",
                "Fresh",
                "--objective",
                "Add a new area under tracked configuration.",
            ],
        );
        let bundle = serde_json::to_vec(&json!({
            "schema_version": 1,
            "area": "fresh",
            "tasks": [{
                "key": "one", "title": "First task", "outcome": "The task is durable.",
                "done_when": ["The task is committed."], "blocked_by": []
            }]
        }))
        .expect("bundle");

        let imported = json_output_with_stdin(
            root,
            &["tasks", "import", "fresh", "--from", "-", "--commit"],
            &bundle,
        );

        assert_eq!(imported["status"], "committed");
        let committed_config = imported["paths"]
            .as_array()
            .expect("paths")
            .contains(&json!(".zdev/config.toml"));
        assert_eq!(committed_config, !track_config);
        if track_config {
            assert_eq!(
                git(root, &["show", "HEAD^:.zdev/config.toml"]),
                git(root, &["show", "HEAD:.zdev/config.toml"])
            );
        }
    }
}

#[test]
fn personal_first_import_stays_local_and_explicit_commit_is_rejected() {
    let repository = repository();
    let root = repository.path();
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "personal"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "local",
            "--title",
            "Local",
            "--objective",
            "Keep planning local.",
        ],
    );
    let bundle = serde_json::to_vec(&json!({
        "schema_version": 1, "area": "local", "tasks": [{
            "key": "one", "title": "Local task", "outcome": "The task remains local.",
            "done_when": ["The task exists."], "blocked_by": []
        }]
    }))
    .expect("bundle");

    let refused = json_output_with_stdin_status(
        root,
        &["tasks", "import", "local", "--from", "-", "--commit"],
        &bundle,
    );
    assert!(!refused.status.success());
    assert!(String::from_utf8_lossy(&refused.stderr).contains("personal record"));
    assert_eq!(
        fs::read_dir(root.join(".zdev/local/tasks"))
            .expect("tasks")
            .count(),
        0
    );

    let imported =
        json_output_with_stdin(root, &["tasks", "import", "local", "--from", "-"], &bundle);
    assert_eq!(imported["status"], "created");
}

#[cfg(unix)]
#[test]
fn first_committed_task_import_rejects_extra_files_and_rolls_back_commit_failure() {
    use std::os::unix::fs::PermissionsExt;

    let repository = repository();
    let root = repository.path();
    commit_file(root, "unrelated.txt", "before\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "safe",
            "--title",
            "Safe",
            "--objective",
            "Publish safely.",
        ],
    );
    let bundle = serde_json::to_vec(&json!({
        "schema_version": 1, "area": "safe", "tasks": [{
            "key": "one", "title": "Safe task", "outcome": "No partial publication remains.",
            "done_when": ["The import is atomic."], "blocked_by": []
        }]
    }))
    .expect("bundle");
    let index_path = root.join(".zdev/safe/TASKS.md");
    let index_before = fs::read(&index_path).expect("index");

    fs::write(root.join(".zdev/safe/unexpected.md"), "unexpected\n").expect("extra file");
    let unsafe_import = json_output_with_stdin_status(
        root,
        &["tasks", "import", "safe", "--from", "-", "--commit"],
        &bundle,
    );
    assert!(!unsafe_import.status.success());
    assert!(String::from_utf8_lossy(&unsafe_import.stderr).contains("unexpected owning-area path"));
    fs::remove_file(root.join(".zdev/safe/unexpected.md")).expect("remove extra file");

    fs::write(root.join("unrelated.txt"), "staged unrelated\n").expect("unrelated change");
    git(root, &["add", "unrelated.txt"]);
    let cached_before = git(root, &["diff", "--cached", "--", "unrelated.txt"]);
    let hook = root.join(".git/hooks/pre-commit");
    fs::write(&hook, "#!/bin/sh\nexit 1\n").expect("rejecting hook");
    let mut permissions = fs::metadata(&hook).expect("hook metadata").permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&hook, permissions).expect("executable hook");

    let failed = json_output_with_stdin_status(
        root,
        &["tasks", "import", "safe", "--from", "-", "--commit"],
        &bundle,
    );
    assert!(!failed.status.success());
    assert!(String::from_utf8_lossy(&failed.stderr).contains("rolled back"));
    assert_eq!(fs::read(&index_path).expect("restored index"), index_before);
    assert!(!root.join(".zdev/safe/tasks/001-safe-task.md").exists());
    assert_eq!(
        git(root, &["diff", "--cached", "--", "unrelated.txt"]),
        cached_before
    );
    assert!(git(root, &["diff", "--cached", "--name-only", "--", ".zdev"]).is_empty());
    assert_eq!(git(root, &["log", "-1", "--format=%s"]), "seed");
}

#[test]
fn first_committed_task_import_rejects_a_partially_tracked_area() {
    let repository = repository();
    let root = repository.path();
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "partial",
            "--title",
            "Partial",
            "--objective",
            "Reject incomplete planning records.",
        ],
    );
    git(root, &["add", ".zdev/partial/area.toml"]);
    git(root, &["commit", "-m", "partially track area"]);
    let bundle = serde_json::to_vec(&json!({
        "schema_version": 1, "area": "partial", "tasks": [{
            "key": "one", "title": "Unsafe task", "outcome": "Incomplete records are rejected.",
            "done_when": ["No task is published."], "blocked_by": []
        }]
    }))
    .expect("bundle");

    let refused = json_output_with_stdin_status(
        root,
        &["tasks", "import", "partial", "--from", "-", "--commit"],
        &bundle,
    );

    assert!(!refused.status.success());
    assert!(String::from_utf8_lossy(&refused.stderr).contains("ambiguously partially tracked"));
    assert_eq!(
        fs::read_dir(root.join(".zdev/partial/tasks"))
            .expect("tasks")
            .count(),
        0
    );
    assert!(
        git(
            root,
            &["ls-tree", "--name-only", "HEAD", ".zdev/partial/brief.md"]
        )
        .is_empty()
    );
    assert!(git(root, &["diff", "--cached", "--name-only"]).is_empty());
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
    let brief_path = root.join(".zdev/concurrent/brief.md");
    let brief = "# Concurrent\n\n## Objective\n\nAccept approved task additions.\n\n## Testing\n\nFocused coverage.\n";
    fs::write(&brief_path, brief).expect("updated brief");
    let bundle = serde_json::to_vec(&json!({
        "schema_version": 1,
        "area": "concurrent",
        "tasks": [{
            "key": "added",
            "title": "Add concurrent task",
            "outcome": "The task joins the next selection boundary.",
            "done_when": ["The task is available."],
            "validation": ["Exercise the CLI."],
            "blocked_by": ["concurrent-001"]
        }]
    }))
    .expect("bundle");

    let reviewed = json_output_with_stdin(
        root,
        &["tasks", "review", "concurrent", "--from", "-"],
        &bundle,
    );
    let imported = json_output(
        root,
        &[
            "tasks",
            "import",
            "concurrent",
            "--reviewed",
            reviewed["review"].as_str().expect("review ID"),
            "--commit",
        ],
    );

    assert_eq!(imported["status"], "committed");
    assert_eq!(imported["tasks"][0], "concurrent-002");
    assert_eq!(imported["ready"], json!(["concurrent-001"]));
    assert_eq!(
        imported["paths"],
        json!([
            ".zdev/concurrent/brief.md",
            ".zdev/concurrent/tasks/002-add-concurrent-task.md",
            ".zdev/concurrent/TASKS.md"
        ])
    );
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
            ".zdev/concurrent/brief.md",
            ".zdev/concurrent/tasks/002-add-concurrent-task.md",
        ]
    );
    assert_eq!(fs::read_to_string(&brief_path).expect("brief"), brief);
    assert_eq!(
        git(root, &["diff", "--cached", "--name-only"]),
        "staged.txt"
    );
    assert_eq!(git(root, &["diff", "--name-only"]), "unstaged.txt");
    assert!(git(root, &["status", "--short", "--", ".zdev/concurrent"]).is_empty());
    assert_eq!(
        json_output(root, &["next", "concurrent"])["task"]["id"],
        "concurrent-001"
    );

    let task_only = serde_json::to_vec(&json!({
        "schema_version": 1,
        "area": "concurrent",
        "tasks": [{
            "key": "task-only",
            "title": "Preserve task-only import",
            "outcome": "An unchanged brief keeps the existing path contract.",
            "done_when": ["Only the task and index are committed."],
            "blocked_by": []
        }]
    }))
    .expect("task-only bundle");
    let task_only = json_output_with_stdin(
        root,
        &["tasks", "import", "concurrent", "--from", "-", "--commit"],
        &task_only,
    );
    assert_eq!(
        task_only["paths"],
        json!([
            ".zdev/concurrent/tasks/003-preserve-task-only-import.md",
            ".zdev/concurrent/TASKS.md"
        ])
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
        root.join(".zdev/intake-action/TASKS.md"),
        "locally changed\n",
    )
    .expect("change generated index");
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
    let brief_path = root.join(".zdev/commit-failure/brief.md");
    let brief = "# Commit failure\n\n## Objective\n\nRecover a failed import.\n\n## Testing\n\nFocused coverage.\n";
    fs::write(&brief_path, brief).expect("updated brief");
    let brief_diff_before = git(root, &["diff", "--", ".zdev/commit-failure/brief.md"]);
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
    assert_eq!(fs::read_to_string(&brief_path).expect("brief"), brief);
    assert_eq!(
        git(root, &["diff", "--", ".zdev/commit-failure/brief.md"]),
        brief_diff_before
    );
    assert!(
        git(
            root,
            &["diff", "--cached", "--", ".zdev/commit-failure/brief.md"]
        )
        .is_empty()
    );
    assert_eq!(
        git(root, &["diff", "--cached", "--name-only"]),
        "implementation.txt"
    );
    assert_eq!(
        git(root, &["log", "-1", "--format=%s"]),
        "configure failure area"
    );
    assert_eq!(
        json_output(root, &["next", "commit-failure"])["task"]["id"],
        "commit-failure-001"
    );
}

#[test]
fn committed_task_import_rejects_invalid_or_partially_staged_briefs_before_publication() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "brief-state",
            "--title",
            "Brief state",
            "--objective",
            "Reject unsafe brief states.",
        ],
    );
    commit_all(root, "configure brief state area");
    let bundle = serde_json::to_vec(&json!({
        "schema_version": 1,
        "area": "brief-state",
        "tasks": [{
            "key": "added",
            "title": "Must not publish",
            "outcome": "Unsafe brief state prevents publication.",
            "done_when": ["No task is added."],
            "blocked_by": []
        }]
    }))
    .expect("bundle");
    let brief = root.join(".zdev/brief-state/brief.md");
    let index_before = fs::read_to_string(root.join(".zdev/brief-state/TASKS.md")).expect("index");

    fs::write(&brief, "# Invalid brief\n").expect("malformed brief");
    let malformed = json_output_with_stdin_status(
        root,
        &["tasks", "import", "brief-state", "--from", "-", "--commit"],
        &bundle,
    );
    assert!(!malformed.status.success());
    assert!(String::from_utf8_lossy(&malformed.stderr).contains("Brief lacks ## Objective"));

    fs::write(
        &brief,
        "# Brief state\n\n## Objective\n\nStaged version.\n\n## Testing\n\nFocused coverage.\n",
    )
    .expect("staged brief");
    git(root, &["add", ".zdev/brief-state/brief.md"]);
    fs::write(
        &brief,
        "# Brief state\n\n## Objective\n\nWorktree version.\n\n## Testing\n\nFocused coverage.\n",
    )
    .expect("worktree brief");
    let cached_before = git(
        root,
        &["diff", "--cached", "--", ".zdev/brief-state/brief.md"],
    );
    let worktree_before = git(root, &["diff", "--", ".zdev/brief-state/brief.md"]);
    let ambiguous = json_output_with_stdin_status(
        root,
        &["tasks", "import", "brief-state", "--from", "-", "--commit"],
        &bundle,
    );
    assert!(!ambiguous.status.success());
    assert_eq!(
        git(
            root,
            &["diff", "--cached", "--", ".zdev/brief-state/brief.md"]
        ),
        cached_before
    );
    assert_eq!(
        git(root, &["diff", "--", ".zdev/brief-state/brief.md"]),
        worktree_before
    );
    assert_eq!(
        fs::read_to_string(root.join(".zdev/brief-state/TASKS.md")).expect("index"),
        index_before
    );
    assert_eq!(
        fs::read_dir(root.join(".zdev/brief-state/tasks"))
            .expect("tasks")
            .count(),
        0
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
    let rendered = fs::read_to_string(destination.join("zdev/SKILL.md")).expect("installed skill");
    assert_eq!(
        normalize_task_workflows_locator(rendered.as_bytes().to_vec()),
        normalize_task_workflows_locator(packaged_skill.as_bytes().to_vec())
    );
    assert_eq!(
        fs::read_to_string(destination.join("zdev/references/verify.md"))
            .expect("verify reference"),
        include_str!("../templates/zdev/references/verify.md")
    );
    assert_eq!(
        fs::read_to_string(destination.join("zdev/references/discuss.md"))
            .expect("discuss reference"),
        include_str!("../skills/zdev/references/discuss.md")
    );
    assert_eq!(
        fs::read_to_string(destination.join("zdev/references/task-format.md"))
            .expect("task format reference"),
        include_str!("../templates/zdev/references/task-format.md")
    );

    let unchanged = json_output(
        root,
        &["skill", "install", "codex", "--to", destination_text],
    );
    assert_eq!(unchanged["status"], "unchanged");

    fs::write(destination.join("zdev/SKILL.md"), "locally changed\n").expect("change skill");
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
        fs::read_to_string(destination.join("zdev/SKILL.md")).expect("replaced skill"),
        rendered
    );
}

#[test]
fn skill_human_output_names_zdev_integrations_and_their_harnesses() {
    let repository = repository();
    let root = repository.path();

    for (harness, display_name, skill_path) in [
        ("codex", "Codex", "zdev/SKILL.md"),
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
            &["Inspect or change layered project and worker configuration"],
        ),
        (
            &["config", "show", "--help"],
            &[
                "Show effective configuration or values stored in one scope",
                "global worker-profile file",
                "only values stored in this repository",
            ],
        ),
        (
            &["config", "get", "--help"],
            &[
                "Show one effective or scoped configuration value",
                "fixed project and worker registry",
                "Read only the global worker-profile file",
            ],
        ),
        (
            &["config", "set", "--help"],
            &[
                "Set one typed project or worker configuration value",
                "Value, or model and effort for a worker profile",
                "Write the global worker-profile file",
            ],
        ),
        (
            &["config", "unset", "--help"],
            &[
                "Remove one value from its selected scope",
                "Write the global worker-profile file",
            ],
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
            &["either an isolated owning branch or the dynamically resolved project trunk"],
        ),
        (
            &["area", "create", "--help"],
            &[
                "Create an area for one objective on an isolated branch or project trunk",
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
        (
            &["tasks", "review", "--help"],
            &[
                "Store or show a JSON task bundle for human review",
                "--from",
                "PATH_OR_DASH",
                "--show",
            ],
        ),
        (
            &["tasks", "import", "--help"],
            &[
                "stored review or direct JSON task bundle",
                "Area tag that will own the imported tasks",
                "or - to read the bundle from standard input",
                "--reviewed",
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
    assert!(
        String::from_utf8_lossy(&rejected_missing_branch.stderr)
            .contains("isolated area requires branch")
    );
}

#[test]
fn skill_install_and_check_support_explicit_destinations_and_replacement() {
    let repository = repository();
    let root = repository.path();

    let harness = "claude";
    let destination = root.join(format!("installed/{harness}/zdev"));
    let destination_text = destination.to_str().expect("destination path");

    let installed = json_output(
        root,
        &["skill", "install", harness, "--to", destination_text],
    );
    assert_eq!(installed["harness"], harness);
    assert_eq!(installed["scope"], "explicit");
    assert_eq!(installed["status"], "created");

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

    let skill = destination.join("skills/zdev/SKILL.md");
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
            "zdev/SKILL.md",
            "zdev/agents/openai.yaml",
            "zdev/references/area-loop.md",
            "zdev/references/audit.md",
            "zdev/references/discuss.md",
            "zdev/references/implement.md",
            "zdev/references/improve.md",
            "zdev/references/investigate.md",
            "zdev/references/recovery.md",
            "zdev/references/setup.md",
            "zdev/references/shape-work.md",
            "zdev/references/task-format.md",
            "zdev/references/task-workflows.md",
            "zdev/references/to-tasks.md",
            "zdev/references/verify-workflow.md",
            "zdev/references/verify.md",
        ]
    );

    assert_eq!(
        file_inventory(&claude),
        [
            ".claude-plugin/plugin.json",
            "agents/zdev-advanced-implementer.md",
            "agents/zdev-implementer.md",
            "agents/zdev-planner.md",
            "agents/zdev-routine-implementer.md",
            "agents/zdev-verifier.md",
            "contracts/task-workflows.md",
            "skills/zdev/SKILL.md",
            "skills/zdev/references/area-loop.md",
            "skills/zdev/references/audit.md",
            "skills/zdev/references/discuss.md",
            "skills/zdev/references/implement.md",
            "skills/zdev/references/improve.md",
            "skills/zdev/references/investigate.md",
            "skills/zdev/references/recovery.md",
            "skills/zdev/references/setup.md",
            "skills/zdev/references/shape-work.md",
            "skills/zdev/references/task-format.md",
            "skills/zdev/references/task-workflows.md",
            "skills/zdev/references/to-tasks.md",
            "skills/zdev/references/verify-workflow.md",
            "skills/zdev/references/verify.md",
            "workflows/zdev-audit.js",
            "workflows/zdev-goal.js",
            "workflows/zdev-implement.js",
            "workflows/zdev-loop.js",
            "workflows/zdev-verify.js",
        ]
    );

    let manifest: Value = serde_json::from_slice(
        &fs::read(claude.join(".claude-plugin/plugin.json")).expect("Claude manifest"),
    )
    .expect("manifest JSON");
    assert_eq!(manifest["name"], "zdev");
    assert_eq!(manifest["version"], env!("CARGO_PKG_VERSION"));
}

#[test]
fn claude_task_workflows_extract_one_valid_structured_envelope() {
    let implement = include_str!("../templates/zdev/claude/workflows/zdev-implement.js");
    let verify = include_str!("../templates/zdev/claude/workflows/zdev-verify.js");

    let parser_start = implement
        .find("const decodeJsonObject")
        .expect("parser start");
    let parser_end = implement
        .find("const workerResultKeys")
        .expect("parser end");
    let probe = format!(
        r#"const area = 'general'
{}
const head = '0123456789abcdef0123456789abcdef01234567'
const stored = JSON.stringify({{ schema_version: 1, area, lifecycle: 'open', queue: 'ready', task_id: 'general-001', complexity: 'standard', stale_advisory: false, head, snapshot: 'W0123456789abcdef', path: '.git/zdev/work-context/general/W0123456789abcdef.json' }})
if (parseStoredContext(stored, area)?.baselineSnapshot !== 'W0123456789abcdef') throw new Error('stored baseline rejected')
if (!parseStoredContext('Stored context:\n```json\n' + stored + '\n```', area)) throw new Error('wrapped stored context rejected')
if (parseStoredContext(JSON.stringify({{ ...JSON.parse(stored), snapshot: '/tmp/context.json' }}), area)) throw new Error('baseline path accepted')
if (parseStoredContext(JSON.stringify({{ ...JSON.parse(stored), head: 'bad' }}), area)) throw new Error('malformed HEAD accepted')
if (parseStoredContext(stored, area, {{ taskId: 'general-002', head, complexity: 'standard' }})) throw new Error('changed task accepted')
const closed = JSON.stringify({{ schema_version: 1, area, lifecycle: 'closed', queue: 'empty', task_id: null, snapshot: 'W0123456789abcdef', path: '.git/zdev/work-context/general/W0123456789abcdef.json' }})
if (parseStoredContext(closed, area)?.taskId !== null) throw new Error('closed no-work rejected')
if (parseStoredContext(closed, area, {{ taskId: 'general-001', head, complexity: 'standard' }})) throw new Error('closed context accepted for a selected task')
"#,
        &implement[parser_start..parser_end]
    );
    let parser_probe = Command::new("node")
        .args(["--input-type=commonjs", "--eval", &probe])
        .output()
        .expect("run Claude no-work parser probe");
    assert!(
        parser_probe.status.success(),
        "Claude no-work parser probe failed: {}",
        String::from_utf8_lossy(&parser_probe.stderr)
    );

    for workflow in [implement, verify] {
        let worker_start = workflow
            .find("const decodeJsonObject")
            .expect("worker parser start");
        let worker_end = ["\nconst derivedSplitFrom", "\n\nif (!/^[a-z0-9]"]
            .into_iter()
            .filter_map(|marker| workflow[worker_start..].find(marker))
            .min()
            .map(|offset| worker_start + offset)
            .expect("worker parser end");
        let worker_probe = format!(
            r#"const area = 'general'
const taskId = 'general-001'
{}
const base = {{ verdict: 'pass', summary: 'Verified.', findings: [], escalation: 'none' }}
if (!parseVerifierResult(JSON.stringify(base))) throw new Error('semantic PASS rejected')
const rework = {{ ...base, verdict: 'rework', findings: ['src/lib.rs:1 is wrong'], escalation: 'advanced-implementer' }}
if (!parseVerifierResult(JSON.stringify(rework))) throw new Error('semantic REWORK escalation rejected')
if (!reportsValidationWrite({{ ...rework, findings: ['validation_write: src/generated.rs'] }})) throw new Error('validation write marker rejected')
for (const finding of ['src/generated.rs changed', 'validation_write: /tmp/file', 'validation_write: src/../file', 'validation_write: src\\file']) {{
  if (reportsValidationWrite({{ ...rework, findings: [finding] }})) throw new Error('invalid validation write marker accepted: ' + finding)
}}
if (reportsValidationWrite({{ ...rework, findings: ['validation_write: src/generated.rs', 'validation_write: ../outside'] }})) throw new Error('mixed valid and malformed validation markers accepted')
if (parseVerifierResult(JSON.stringify({{ ...base, findings: ['contradictory finding'] }}))) throw new Error('PASS with findings accepted')
if (parseVerifierResult(JSON.stringify({{ ...rework, findings: [] }}))) throw new Error('REWORK without findings accepted')
if (parseVerifierResult(JSON.stringify({{ ...base, extra: true }}))) throw new Error('unknown semantic key accepted')
if (parseVerifierResult(JSON.stringify({{ ...base, escalation: 'advanced-implementer' }}))) throw new Error('contradictory escalation accepted')
if (!parseVerifierResult(`Result:\n\`\`\`json\n${{JSON.stringify(base)}}\n\`\`\``)) throw new Error('wrapped JSON rejected')
if (parseVerifierResult(`${{JSON.stringify(base)}}\n${{JSON.stringify(base)}}`)) throw new Error('multiple JSON objects accepted')
const duplicate = '{{"verdict":"pass","summary":"Verified.","findings":[],"findings":[],"escalation":"none"}}'
if (parseVerifierResult(duplicate)) throw new Error('duplicate semantic key accepted')
const legacy = {{ schema_version: 1, kind: 'verifier', area, task_id: taskId, ...base, evidence: [] }}
if (parseVerifierResult(JSON.stringify(legacy))) throw new Error('legacy verifier envelope accepted')
const snapshot = 'W0123456789abcdef'
const compared = JSON.stringify({{ schema_version: 1, area, snapshot, equal: true }})
if (!parseComparison(compared, ...(parseComparison.length === 3 ? [area, snapshot] : [snapshot]))) throw new Error('valid comparison rejected')
if (parseComparison(JSON.stringify({{ schema_version: 1, area, snapshot, equal: true, extra: true }}), ...(parseComparison.length === 3 ? [area, snapshot] : [snapshot]))) throw new Error('extra comparison key accepted')
const publicResult = publicVerifier(base, snapshot, 'stale effective-base link; managed rebase remains optional.')
if (!publicResult || publicResult.schema_version !== 1 || publicResult.kind !== 'verifier' || publicResult.area !== area || publicResult.task_id !== taskId) throw new Error('public identity not generated')
if (JSON.stringify(publicResult.evidence) !== JSON.stringify(['work_context_snapshot: ' + snapshot, 'stale effective-base link; managed rebase remains optional.'])) throw new Error('public evidence not generated')
if (typeof parseWorkerResult === 'function') {{
  const implementer = {{ schema_version: 1, kind: 'implementer', area, task_id: taskId, verdict: 'blocker', summary: 'Cannot edit safely.', evidence: [], findings: ['overlap'], escalation: 'none' }}
  if (!parseWorkerResult(JSON.stringify(implementer), 'implementer', area, taskId)) throw new Error('implementer envelope rejected')
  if (parseWorkerResult(JSON.stringify({{ ...implementer, extra: true }}), 'implementer', area, taskId)) throw new Error('unknown implementer key accepted')
  const plan = {{ ...implementer, kind: 'planner', verdict: 'plan', evidence: ['Approach: inspect', 'Paths: src/lib.rs', 'Validation: cargo test'], findings: [] }}
  if (!parseWorkerResult(JSON.stringify(plan), 'planner', area, taskId)) throw new Error('planner envelope rejected')
  if (parseWorkerResult(JSON.stringify(legacy), 'verifier', area, taskId)) throw new Error('legacy verifier accepted by nine-key parser')
}}
"#,
            &workflow[worker_start..worker_end]
        );
        let output = Command::new("node")
            .args(["--input-type=commonjs", "--eval", &worker_probe])
            .output()
            .expect("run Claude worker-result parser probe");
        assert!(
            output.status.success(),
            "Claude worker-result parser probe failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn claude_standalone_verify_returns_only_valid_snapshot_locators() {
    let source = include_str!("../templates/zdev/claude/workflows/zdev-verify.js")
        .replacen("export const meta =", "const meta =", 1)
        .replace(
            "{{task_workflow_contract}}",
            &serde_json::to_string("workflow contract").expect("workflow contract JSON"),
        )
        .replace(
            "{{repository_guidance}}",
            &serde_json::to_string("repository guidance").expect("repository guidance JSON"),
        );
    let probe = format!(
        r#"
async function run(args, agent) {{
{source}
}}
const area = 'work'
const task = 'work-001'
const head = '0123456789abcdef0123456789abcdef01234567'
const snapshot = 'W0123456789abcdef'
const stored = JSON.stringify({{ schema_version: 1, area, lifecycle: 'open', queue: 'ready', task_id: task,
  complexity: 'standard', stale_advisory: false, head, snapshot, path: '.git/zdev/work-context/work/' + snapshot + '.json' }})
const comparison = equal => JSON.stringify({{ schema_version: 1, area, snapshot, equal }})
const worker = (verdict, findings = [], escalation = 'none') => JSON.stringify({{
  verdict,
  summary: verdict === 'pass' ? 'Checked task and validation.' : 'Correction required.',
  findings, escalation,
}})
const exercise = async (response, equal = true, invocation = {{ area, task_id: task }}, storedResponse = stored, comparedResponse = comparison(equal)) => {{
  const prompts = []
  const result = await run(invocation, async (prompt, options) => {{
    prompts.push({{ prompt, options }})
    if (options.agentType) return response
    if (options.label === 'zdev ' + task + ': capture verification snapshot') return storedResponse
    if (options.label === 'zdev ' + task + ': confirm verifier left snapshot unchanged') return comparedResponse
    throw new Error('unexpected coordination call: ' + options.label)
  }})
  return {{ result, prompts }}
}}
const valid = await exercise(worker('pass'))
if (valid.prompts.length !== 3) throw new Error('explicit verify did not use three Claude calls')
for (const call of valid.prompts.filter(call => !call.options.agentType)) {{
  if (call.options.model !== 'haiku') throw new Error('deterministic verification coordination did not use Haiku')
}}
const publicPass = JSON.stringify({{ schema_version: 1, kind: 'verifier', area, task_id: task, verdict: 'pass', summary: 'Checked task and validation.', evidence: ['work_context_snapshot: ' + snapshot], findings: [], escalation: 'none' }})
if (valid.result !== publicPass) throw new Error(valid.result)
for (const invocation of [['work', 'work-001'], 'work work-001']) {{
  const direct = await exercise(worker('pass'), true, invocation)
  if (!direct.result.startsWith('{{"schema_version":1,"kind":"verifier"')) throw new Error('direct args rejected: ' + direct.result)
}}
const verifierPrompt = valid.prompts.find(call => call.options.agentType)?.prompt ?? ''
if (verifierPrompt.includes('workflow contract')) throw new Error('full workflow contract was injected')
if (!verifierPrompt.includes('exactly these four keys') || !verifierPrompt.includes('validation_writes')) throw new Error('verifier dispatch omitted its exact semantic contract')
if (!valid.prompts.find(call => call.options.label.includes('capture verification snapshot'))?.prompt.includes('work-context work --task work-001 --store --format json')) throw new Error('coordinator store missing')
if (valid.prompts.some(call => call.options.label === 'zdev verify preflight')) throw new Error('redundant full-context preflight retained')
if (!verifierPrompt.includes('work-context work --show ' + snapshot + ' --format json')) throw new Error('supplied show command incomplete')
if (verifierPrompt.includes('--store') || verifierPrompt.includes('--compare')) throw new Error('verifier retained bookkeeping')
if (!valid.prompts.find(call => call.options.label.includes('confirm verifier'))?.prompt.includes('work-context work --compare ' + snapshot + ' --format json')) throw new Error('coordinator compare missing')
if (verifierPrompt.includes('large diff') || verifierPrompt.includes(' M src/lib.rs')) throw new Error('raw coordinator Git evidence transported')
for (const rejectedWorker of [
  '{{}}',
  JSON.stringify({{ schema_version: 1, kind: 'verifier', area, task_id: task, verdict: 'pass', summary: 'legacy', evidence: [], findings: [], escalation: 'none' }}),
]) {{
  const rejected = await exercise(rejectedWorker)
  if (!rejected.result.startsWith('BLOCKER zdev-verify work work-001')) throw new Error(rejected.result)
  if (rejected.prompts.some(call => call.options.label === 'zdev completion and commit')) throw new Error('mutation attempted')
}}
const rework = await exercise(worker('rework', ['validation_write: src/generated.rs']), false)
if (!rework.result.includes('"verdict":"rework"') || !rework.result.includes('work_context_snapshot: ' + snapshot)) throw new Error(rework.result)
const ambiguousRework = await exercise(worker('rework', ['src/lib.rs: ordinary implementation defect']), false)
if (!ambiguousRework.result.startsWith('BLOCKER zdev-verify work work-001')) throw new Error(ambiguousRework.result)
const mixedMarkers = await exercise(worker('rework', ['validation_write: src/generated.rs', 'validation_write: ../outside']), false)
if (!mixedMarkers.result.startsWith('BLOCKER zdev-verify work work-001')) throw new Error(mixedMarkers.result)
const escalated = await exercise(worker('rework', ['advanced correction required'], 'advanced-implementer'))
if (!escalated.result.includes('"escalation":"advanced-implementer"')) throw new Error(escalated.result)
const semanticBlocker = await exercise(worker('blocker', ['ownership is ambiguous']))
if (!semanticBlocker.result.includes('"verdict":"blocker"')) throw new Error(semanticBlocker.result)
const emptyRework = await exercise(worker('rework'))
if (!emptyRework.result.startsWith('BLOCKER zdev-verify work work-001')) throw new Error(emptyRework.result)
const contradictoryPass = await exercise(worker('pass', ['still broken']))
if (!contradictoryPass.result.startsWith('BLOCKER zdev-verify work work-001')) throw new Error(contradictoryPass.result)
const changedPass = await exercise(worker('pass'), false)
if (!changedPass.result.startsWith('BLOCKER zdev-verify work work-001')) throw new Error(changedPass.result)
const malformedCompare = await exercise(worker('pass'), true, {{ area, task_id: task }}, stored, '{{}}')
if (!malformedCompare.result.startsWith('BLOCKER zdev-verify work work-001')) throw new Error(malformedCompare.result)
const staleStored = JSON.stringify({{ ...JSON.parse(stored), stale_advisory: true }})
const stale = await exercise(worker('pass'), true, {{ area, task_id: task }}, staleStored, comparison(true))
const stalePublic = JSON.parse(stale.result)
if (JSON.stringify(stalePublic.evidence) !== JSON.stringify(['work_context_snapshot: ' + snapshot, 'stale effective-base link; managed rebase remains optional.'])) throw new Error(stale.result)
"#
    );
    let output = Command::new("node")
        .args(["--input-type=module", "--eval", &probe])
        .output()
        .expect("run Claude standalone verify snapshot probe");
    assert!(
        output.status.success(),
        "Claude standalone verify snapshot probe failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn claude_implementation_routes_complexity_planning_rework_and_escalation() {
    let source = include_str!("../templates/zdev/claude/workflows/zdev-implement.js")
        .replacen("export const meta =", "const meta =", 1)
        .replace(
            "{{task_workflow_contract}}",
            &serde_json::to_string("workflow contract").expect("workflow contract JSON"),
        )
        .replace(
            "{{task_workflows_contract_path_json}}",
            &serde_json::to_string("/installed/zdev/contracts/task-workflows.md")
                .expect("workflow contract path JSON"),
        )
        .replace(
            "{{repository_guidance}}",
            &serde_json::to_string("repository guidance").expect("repository guidance JSON"),
        );
    let probe = format!(
        r#"
async function run(args, agent) {{
{source}
}}
const area = 'work'
const taskId = 'work-001'
const head = '0123456789abcdef0123456789abcdef01234567'
const worker = (kind, verdict, escalation = 'none', evidence = [], findings = [], summary = verdict + ' result') => JSON.stringify(
  kind === 'verifier'
    ? {{ verdict, summary, findings, escalation }}
    : {{ schema_version: 1, kind, area, task_id: taskId, verdict, summary, evidence, findings, escalation }}
)
const structuredPlanner = (verdict, plan = null, findings = [], summary = verdict + ' result') => ({{
  verdict, summary, plan, findings,
}})
const semanticPlan = () => ({{ approach: 'inspect then edit', paths: ['src/lib.rs', 'tests/lean.rs'], validation: ['cargo test', 'git diff --check'] }})
const baselineSnapshot = 'Wfedcba9876543210'
const verificationSnapshot = 'W0123456789abcdef'
const compactContext = (complexity, snapshot, overrides = {{}}) => JSON.stringify({{
  schema_version: 1, area, lifecycle: 'open', queue: 'ready',
  task_id: overrides.taskId ?? taskId,
  complexity: overrides.complexity ?? complexity,
  stale_advisory: false,
  head: overrides.head ?? head,
  snapshot,
  path: '.git/zdev/work-context/work/' + snapshot + '.json',
}})
const storedContext = complexity => compactContext(complexity, baselineSnapshot)
const verificationContext = (complexity, overrides = {{}}) => {{
  return compactContext(complexity, verificationSnapshot, overrides)
}}
const comparison = equal => JSON.stringify({{ schema_version: 1, area, snapshot: verificationSnapshot, equal }})
const passEvidence = [
  'work_context_snapshot: W0123456789abcdef',
]
const completion = 'Completion finished.\nPASS zdev-implement work work-001\n\nArea: work\nTask: work-001\nSummary: complete\nChanged files:\n- src/lib.rs\nValidation: passed\nVerifier evidence: checked\nCommit ID: `abc123`'
const exercise = async (name, complexity, responses, expectedTypes, expectedPrefix = 'PASS', derived = null, invocation = {{ area }}, compareEquals = [], verificationOverrides = {{}}) => {{
  const types = []
  const calls = []
  const result = await run(invocation, async (prompt, options) => {{
    calls.push({{ prompt, options }})
    if (options.agentType) {{
      types.push(options.agentType)
      if (responses.length === 0) throw new Error(name + ': unexpected worker')
      return responses.shift()
    }}
    if (options.label.includes('coordinate derived split')) {{
      if (derived === null) throw new Error(name + ': unexpected derived coordination')
      const result = derived
      derived = null
      return result
    }}
    if (options.label.includes('complete and commit')) return completion
    if (options.label === 'zdev work: select ready task') return storedContext(complexity)
    if (options.label.includes('capture verification snapshot')) return verificationContext(complexity, verificationOverrides)
    if (options.label.includes('confirm verifier')) return comparison(compareEquals.length ? compareEquals.shift() : true)
    if (options.label.includes('refresh before rework')) return compactContext(complexity, verificationSnapshot)
    throw new Error(name + ': unexpected coordination call ' + options.label)
  }})
  if (!result.startsWith(expectedPrefix + ' zdev-implement')) throw new Error(name + ': ' + result)
  if (responses.length !== 0) throw new Error(name + ': unused responses')
  if (compareEquals.length !== 0) throw new Error(name + ': unused comparison responses')
  if (derived !== null) throw new Error(name + ': unused derived response')
  if (JSON.stringify(types) !== JSON.stringify(expectedTypes)) {{
    throw new Error(name + ': ' + JSON.stringify(types))
  }}
  const verifierPrompts = calls.filter(call => call.options.agentType === 'zdev:zdev-verifier').map(call => call.prompt)
  for (const call of calls.filter(call => call.options.agentType)) {{
    if (call.prompt.includes('workflow contract')) throw new Error(name + ': full workflow contract was injected')
  }}
  const plannerCalls = calls.filter(call => call.options.agentType === 'zdev:zdev-planner')
  if (plannerCalls.length > 1) throw new Error(name + ': planner dispatched more than once')
  for (const call of plannerCalls) {{
    const schema = call.options.schema
    if (!schema || schema.type !== 'object' || schema.additionalProperties !== false) throw new Error(name + ': planner schema missing exact object boundary')
    if (JSON.stringify([...schema.required].sort()) !== JSON.stringify(['findings', 'plan', 'summary', 'verdict'])) throw new Error(name + ': planner schema keys differ')
    if (call.options.outputFormat || call.options.jsonSchema) throw new Error(name + ': planner used a non-workflow schema option')
  }}
  for (const prompt of verifierPrompts) {{
    if (!prompt.includes('Implementer summary:')) throw new Error(name + ': verifier lost compact implementation summary')
    if (!prompt.includes('work-context work --show ' + baselineSnapshot + ' --format json')) throw new Error(name + ': verifier lost original baseline command')
    if (!prompt.includes('work-context work --show ' + verificationSnapshot + ' --format json')) throw new Error(name + ': verifier lost supplied snapshot command')
    if (prompt.includes('--store') || prompt.includes('--compare')) throw new Error(name + ': verifier retained coordinator bookkeeping')
    if (prompt.includes('implementer history')) throw new Error(name + ': verifier received history')
    if (prompt.includes('"git_status":') || prompt.includes('"git_diff":')) throw new Error(name + ': verifier received raw coordinator context')
    if (!prompt.includes('exactly these four keys') || !prompt.includes('validation_writes')) throw new Error(name + ': verifier dispatch omitted its exact semantic contract')
  }}
  for (const call of calls.filter(call => !call.options.agentType
    && !call.options.label.includes('coordinate derived split')
    && !call.options.label.includes('complete and commit'))) {{
    if (call.options.model !== 'haiku') throw new Error(name + ': deterministic coordination did not use Haiku: ' + call.options.label)
  }}
  const completionPrompt = calls.find(call => call.options.label.includes('complete and commit'))?.prompt
  if (completionPrompt) {{
    if (completionPrompt.includes('implementer envelope') || completionPrompt.includes('implementer history')) throw new Error(name + ': completion received implementation payload')
    if (completionPrompt.includes('Verifier pass:') || completionPrompt.includes('"kind":"verifier"')) throw new Error(name + ': completion received duplicate verifier payload')
    if (!completionPrompt.includes('"snapshot":"W0123456789abcdef"')) throw new Error(name + ': completion lost snapshot locator')
    if (!completionPrompt.includes('"implementation":"ready result"')) throw new Error(name + ': completion lost implementation summary')
    if (!completionPrompt.includes('"verification":"pass result"')) throw new Error(name + ': completion lost verification summary')
    if (completionPrompt.includes('"git_status":') || completionPrompt.includes('"git_diff":')) throw new Error(name + ': completion received raw Git evidence')
    if (!completionPrompt.includes('zdev work-context work --compare W0123456789abcdef --format json')) throw new Error(name + ': completion lost compact comparison')
  }}
  return {{ verifierPrompts, completionPrompt, calls }}
}}
const routinePass = await exercise(
  'routine pass',
  'routine',
  [worker('implementer', 'ready', 'none', ['routine locator']), worker('verifier', 'pass', 'none', passEvidence)],
  ['zdev:zdev-routine-implementer', 'zdev:zdev-verifier'],
)
if (routinePass.calls.length !== 6) throw new Error('ordinary PASS did not use six Claude calls')
if (routinePass.calls.some(call => call.options.label === 'zdev pre-verification refresh')) throw new Error('redundant pre-verification refresh retained')
if (!routinePass.verifierPrompts[0].includes('routine locator')) throw new Error('initial verifier lost implementation locator')
await exercise(
  'standard pass',
  'standard',
  [worker('implementer', 'ready'), worker('verifier', 'pass', 'none', passEvidence)],
  ['zdev:zdev-implementer', 'zdev:zdev-verifier'],
)
for (const [name, invocation] of [['array args', ['work']], ['string args', 'work']]) {{
  await exercise(
    name,
    'standard',
    [worker('implementer', 'ready'), worker('verifier', 'pass', 'none', passEvidence)],
    ['zdev:zdev-implementer', 'zdev:zdev-verifier'],
    'PASS',
    null,
    invocation,
  )
}}
const advancedPlanPass = await exercise(
  'advanced plan pass',
  'advanced',
  [
    structuredPlanner('plan', semanticPlan()),
    worker('implementer', 'ready'),
    worker('verifier', 'pass', 'none', passEvidence),
  ],
  ['zdev:zdev-planner', 'zdev:zdev-advanced-implementer', 'zdev:zdev-verifier'],
)
const advancedPrompt = advancedPlanPass.calls.find(call => call.options.agentType === 'zdev:zdev-advanced-implementer')?.prompt ?? ''
if (!advancedPrompt.includes('"approach":"inspect then edit"')
  || !advancedPrompt.includes('"paths":["src/lib.rs","tests/lean.rs"]')
  || !advancedPrompt.includes('"validation":["cargo test","git diff --check"]')) throw new Error('advanced implementer did not receive unchanged semantic plan')
if (advancedPrompt.includes('Approach: inspect then edit') || advancedPrompt.includes('"kind":"planner"')) throw new Error('advanced implementer received reconstructed planner envelope')
const flexiblePlan = {{
  ...semanticPlan(),
  paths: ['/opt/software/project/src/lib.rs', '/opt/software/project/tests/lean.rs'],
}}
const advancedFlexiblePlan = await exercise(
  'advanced plan with supporting findings and absolute paths',
  'advanced',
  [
    structuredPlanner('plan', flexiblePlan, ['Located the existing call sites.']),
    worker('implementer', 'ready'),
    worker('verifier', 'pass', 'none', passEvidence),
  ],
  ['zdev:zdev-planner', 'zdev:zdev-advanced-implementer', 'zdev:zdev-verifier'],
)
const flexiblePrompt = advancedFlexiblePlan.calls.find(call => call.options.agentType === 'zdev:zdev-advanced-implementer')?.prompt ?? ''
if (!flexiblePrompt.includes('"paths":["/opt/software/project/src/lib.rs","/opt/software/project/tests/lean.rs"]')
  || !flexiblePrompt.includes('"findings":["Located the existing call sites."]')) throw new Error('advanced implementer lost tolerated planner evidence')
await exercise(
  'advanced retained plan rework',
  'advanced',
  [
    JSON.stringify(structuredPlanner('plan', semanticPlan())),
    worker('implementer', 'ready'),
    worker('verifier', 'rework', 'none', [], ['fix the task-owned defect']),
    worker('implementer', 'ready'),
    worker('verifier', 'pass', 'none', passEvidence),
  ],
  ['zdev:zdev-planner', 'zdev:zdev-advanced-implementer', 'zdev:zdev-verifier', 'zdev:zdev-advanced-implementer', 'zdev:zdev-verifier'],
)
const ordinaryRework = await exercise(
  'ordinary rework',
  'standard',
  [
    worker('implementer', 'ready', 'none', ['initial locator']),
    worker('verifier', 'rework', 'none', [], ['fix the task-owned defect']),
    worker('implementer', 'ready', 'none', ['rework locator']),
    worker('verifier', 'pass', 'none', passEvidence),
  ],
  ['zdev:zdev-implementer', 'zdev:zdev-verifier', 'zdev:zdev-implementer', 'zdev:zdev-verifier'],
)
if (ordinaryRework.calls.length !== 11) throw new Error('one REWORK did not use eleven Claude calls')
if (ordinaryRework.calls.some(call => call.options.label === 'zdev post-rework verification refresh')) throw new Error('redundant post-rework verification refresh retained')
if (!ordinaryRework.verifierPrompts[0].includes('initial locator')) throw new Error('first verifier lost initial locator')
if (!ordinaryRework.verifierPrompts[1].includes('rework locator') || ordinaryRework.verifierPrompts[1].includes('initial locator')) throw new Error('second verifier did not receive only latest locator')
const reworkPrompt = ordinaryRework.calls.find(call => call.options.label.endsWith(': rework'))?.prompt ?? ''
if (!reworkPrompt.includes('work-context work --show ' + baselineSnapshot + ' --format json')) throw new Error('rework lost original baseline command')
if (reworkPrompt.includes('"git_status":') || reworkPrompt.includes('"git_diff":')) throw new Error('rework received raw coordinator context')
const advancedEscalation = await exercise(
  'advanced escalation',
  'standard',
  [
    worker('implementer', 'ready', 'none', ['standard locator']),
    worker('verifier', 'rework', 'advanced-implementer', [], ['broader reasoning is required']),
    worker('implementer', 'ready', 'none', ['advanced locator']),
    worker('verifier', 'pass', 'none', passEvidence),
  ],
  ['zdev:zdev-implementer', 'zdev:zdev-verifier', 'zdev:zdev-advanced-implementer', 'zdev:zdev-verifier'],
)
if (!advancedEscalation.verifierPrompts[1].includes('advanced locator') || advancedEscalation.verifierPrompts[1].includes('standard locator')) throw new Error('escalated verifier did not receive only latest locator')
await exercise(
  'invalid implementer envelope',
  'standard',
  ['{{}}'],
  ['zdev:zdev-implementer'],
  'BLOCKER',
)
await exercise(
  'invalid verifier envelope',
  'standard',
  [worker('implementer', 'ready'), '{{}}'],
  ['zdev:zdev-implementer', 'zdev:zdev-verifier'],
  'BLOCKER',
)
await exercise(
  'intervening commit before verifier',
  'standard',
  [worker('implementer', 'ready')],
  ['zdev:zdev-implementer'],
  'BLOCKER',
  null,
  {{ area }},
  [],
  {{ head: '1111111111111111111111111111111111111111' }},
)
await exercise(
  'changed task before verifier',
  'standard',
  [worker('implementer', 'ready')],
  ['zdev:zdev-implementer'],
  'BLOCKER',
  null,
  {{ area }},
  [],
  {{ taskId: 'work-002' }},
)
await exercise(
  'changed complexity before verifier',
  'standard',
  [worker('implementer', 'ready')],
  ['zdev:zdev-implementer'],
  'BLOCKER',
  null,
  {{ area }},
  [],
  {{ complexity: 'advanced' }},
)
await exercise(
  'legacy verifier envelope',
  'standard',
  [worker('implementer', 'ready'), JSON.stringify({{ schema_version: 1, kind: 'verifier', area, task_id: taskId, verdict: 'pass', summary: 'legacy', evidence: [], findings: [], escalation: 'none' }})],
  ['zdev:zdev-implementer', 'zdev:zdev-verifier'],
  'BLOCKER',
)
await exercise(
  'validation write cannot pass',
  'standard',
  [worker('implementer', 'ready'), worker('verifier', 'rework', 'none', [], ['validation_write: src/generated.rs']), worker('implementer', 'ready'), worker('verifier', 'pass', 'none', passEvidence)],
  ['zdev:zdev-implementer', 'zdev:zdev-verifier', 'zdev:zdev-implementer', 'zdev:zdev-verifier'],
  'PASS',
  null,
  {{ area }},
  [false, true],
)
await exercise(
  'ordinary defect plus ambiguous mismatch blocks',
  'standard',
  [worker('implementer', 'ready'), worker('verifier', 'rework', 'none', [], ['src/lib.rs: ordinary implementation defect'])],
  ['zdev:zdev-implementer', 'zdev:zdev-verifier'],
  'BLOCKER',
  null,
  {{ area }},
  [false],
)
await exercise(
  'mixed validation markers plus mismatch block',
  'standard',
  [worker('implementer', 'ready'), worker('verifier', 'rework', 'none', [], ['validation_write: src/generated.rs', 'validation_write: ../outside'])],
  ['zdev:zdev-implementer', 'zdev:zdev-verifier'],
  'BLOCKER',
  null,
  {{ area }},
  [false],
)
await exercise(
  'snapshot mismatch blocks pass',
  'standard',
  [worker('implementer', 'ready'), worker('verifier', 'pass', 'none', passEvidence)],
  ['zdev:zdev-implementer', 'zdev:zdev-verifier'],
  'BLOCKER',
  null,
  {{ area }},
  [false],
)
await exercise(
  'product decision blocker',
  'advanced',
  [structuredPlanner('blocker', null, ['public API choice belongs to the user'])],
  ['zdev:zdev-planner'],
  'BLOCKER',
)
await exercise(
  'unusable structured planner result',
  'advanced',
  [{{ schema_version: 1, kind: 'planner', area, task_id: taskId, verdict: 'plan', summary: 'legacy', evidence: [], findings: [], escalation: 'none' }}],
  ['zdev:zdev-planner'],
  'BLOCKER',
)
for (const [name, rejectedPlanner] of [
  ['non-normalized planner path', structuredPlanner('plan', {{ ...semanticPlan(), paths: ['src/../outside.rs'] }})],
  ['non-normalized absolute planner path', structuredPlanner('plan', {{ ...semanticPlan(), paths: ['/opt/software/../outside.rs'] }})],
  ['extra semantic planner key', {{ ...structuredPlanner('plan', semanticPlan()), extra: true }}],
  ['extra nested semantic plan key', structuredPlanner('plan', {{ ...semanticPlan(), extra: true }})],
  ['duplicate semantic planner key', '{{"verdict":"plan","summary":"first","summary":"second","plan":{{"approach":"inspect","paths":["src/lib.rs"],"validation":["cargo test"]}},"findings":[]}}'],
  ['duplicate nested semantic plan key', '{{"verdict":"plan","summary":"plan result","plan":{{"approach":"inspect","approach":"edit","paths":["src/lib.rs"],"validation":["cargo test"]}},"findings":[]}}'],
  ['blocker with plan', structuredPlanner('blocker', semanticPlan(), ['blocked'])],
  ['blocker without findings', structuredPlanner('blocker', null, [])],
  ['malformed semantic planner JSON', '{{"verdict":"plan"'],
]) {{
  const rejected = await exercise(
    name,
    'advanced',
    [rejectedPlanner],
    ['zdev:zdev-planner'],
    'BLOCKER',
  )
  if (rejected.calls.filter(call => call.options.agentType === 'zdev:zdev-planner').length !== 1) throw new Error(name + ': planner retry occurred')
  if (rejected.calls.some(call => call.options.agentType === 'zdev:zdev-advanced-implementer')) throw new Error(name + ': implementation started')
}}
const splitProposal = 'PROPOSE zdev-derived work work-001\n' + JSON.stringify({{
  schema_version: 1,
  proposal: 'implementation_split',
  area,
  source_task: taskId,
  source_result: {{ status: 'split', summary: 'split direct work', validation: [] }},
  tasks: [{{
    key: 'child', title: 'Implement child', blocked_by: [],
    outcome: 'Child completes direct work.', done_when: ['Child is complete.'],
    validation: ['Check the child.'],
  }}],
  split_ownership: {{
    retained_parent_paths: [],
    child_future_paths: [{{ key: 'child', paths: ['src/child.rs'] }}],
  }},
}})
const splitResult = worker('implementer', 'blocker', 'none', [splitProposal])
const automaticSplit = await exercise(
  'automatic derived split',
  'standard',
  [splitResult],
  ['zdev:zdev-implementer'],
  'PASS',
  'PASS zdev-implement work work-001\n\nArea: work\nTask: work-001\nDerived proposal: implementation_split\nSummary: split applied\nChanged files: task records\nValidation: derive apply\nVerifier evidence: source remains open\nCommit ID: ' + '3'.repeat(40),
)
const splitPrompt = automaticSplit.calls.find(call => call.options.label.includes('coordinate derived split'))?.prompt ?? ''
if (!splitPrompt.includes('work-context work --show ' + baselineSnapshot + ' --format json')) throw new Error('derived split lost original baseline command')
if (!splitPrompt.includes('/installed/zdev/contracts/task-workflows.md')) throw new Error('derived split lost installed contract path')
const splitWorkerPrompt = automaticSplit.calls.find(call => call.options.agentType === 'zdev:zdev-implementer')?.prompt ?? ''
if (!splitWorkerPrompt.includes('/installed/zdev/contracts/task-workflows.md')) throw new Error('implementer lost installed split contract path')
await exercise(
  'manual derived split',
  'standard',
  [splitResult],
  ['zdev:zdev-implementer'],
  'BLOCKER',
  'BLOCKER zdev-implement work work-001\n\nArea: work\nTask: work-001\nFailed stage: derived review\nReason: approval required\nPreserved state: no apply; Approve this derived task bundle for apply?',
)
"#
    );
    let output = Command::new("node")
        .args(["--input-type=module", "--eval", &probe])
        .output()
        .expect("run Claude complexity routing probe");
    assert!(
        output.status.success(),
        "Claude complexity routing probe failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn claude_area_loop_executes_continuation_stop_rework_resume_and_failure() {
    let repository = repository();
    let root = repository.path();
    let destination = root.join("claude-area-loop");
    let config_home = root.join("claude-area-loop-config");
    let environment = [("XDG_CONFIG_HOME", config_home.as_path())];
    json_output_with_env(
        root,
        &[
            "skill",
            "install",
            "claude",
            "--to",
            destination.to_str().expect("Claude loop destination"),
        ],
        &environment,
    );
    assert_eq!(
        json_output_with_env(
            root,
            &[
                "skill",
                "check",
                "claude",
                "--to",
                destination.to_str().expect("Claude loop destination"),
            ],
            &environment,
        )["status"],
        "ok"
    );

    let loop_source =
        fs::read_to_string(destination.join("workflows/zdev-loop.js")).expect("Claude loop");
    let goal_source =
        fs::read_to_string(destination.join("workflows/zdev-goal.js")).expect("Claude goal");
    assert_eq!(
        goal_source.replacen("name: 'zdev-goal'", "name: 'zdev-loop'", 1),
        loop_source,
        "Claude aliases may differ only in meta.name"
    );
    let source = loop_source.replacen("export const meta =", "const meta =", 1);
    let probe = format!(
        r#"
async function run(args, agent) {{
{source}
}}
const area = 'work'
const head = '0123456789abcdef0123456789abcdef01234567'
const commit1 = '1'.repeat(40)
const commit2 = '2'.repeat(40)
const worker = (task, kind, verdict, escalation = 'none', evidence = [], findings = []) => JSON.stringify(
  kind === 'verifier'
    ? {{ verdict, summary: verdict + ' result', findings, escalation }}
    : {{ schema_version: 1, kind, area, task_id: task, verdict, summary: verdict + ' result', evidence, findings, escalation }}
)
const baselineSnapshot = 'Wfedcba9876543210'
const verificationSnapshot = 'W0123456789abcdef'
const ready = (task, complexity = 'standard', contextHead = head) => JSON.stringify({{
  schema_version: 1, area, lifecycle: 'open', queue: 'ready', task_id: task,
  complexity, stale_advisory: false, head: contextHead, snapshot: baselineSnapshot,
  path: '.git/zdev/work-context/work/' + baselineSnapshot + '.json',
}})
const closed = JSON.stringify({{
  schema_version: 1, area, lifecycle: 'closed', queue: 'empty', task_id: null,
  snapshot: baselineSnapshot, path: '.git/zdev/work-context/work/' + baselineSnapshot + '.json',
}})
const empty = contextHead => JSON.stringify({{
  schema_version: 1, area, lifecycle: 'open', queue: 'empty', task_id: null,
  stale_advisory: false, head: contextHead, snapshot: baselineSnapshot,
  path: '.git/zdev/work-context/work/' + baselineSnapshot + '.json',
}})
const passEvidence = _contextHead => ['work_context_snapshot: W0123456789abcdef']
const completionPass = task =>
  'Completion finished.\nPASS zdev-implement ' + area + ' ' + task
  + '\n\nArea: ' + area + '\nTask: ' + task
  + '\nSummary: complete\nChanged files:\n- src/lib.rs\nValidation: passed'
  + '\nVerifier evidence: checked\nCommit ID: `' + (task.endsWith('1') ? commit1 : commit2) + '`'
const exercise = async (name, contexts, workers, completions, expectedPrefix, derived = [], invocation = {{ area }}, selections = []) => {{
  const calls = []
  let lastContext = null
  const result = await run(invocation, async (prompt, options) => {{
    calls.push({{ label: options.label, type: options.agentType ?? null, model: options.model ?? null, prompt }})
    if (options.label === 'zdev work: choose from ready frontier') {{
      if (selections.length === 0) throw new Error(name + ': unexpected focus selection')
      return selections.shift()
    }}
    if (options.label === 'zdev work: select next task' || options.label.startsWith('zdev work: prepare ')) {{
      if (contexts.length === 0) throw new Error(name + ': unexpected context request')
      lastContext = contexts.shift()
      return lastContext
    }}
    if (options.label.includes('refresh before rework')) {{
      if (contexts.length === 0) throw new Error(name + ': unexpected context request')
      lastContext = contexts.shift()
      return lastContext
    }}
    if (options.label.includes('capture verification snapshot')) {{
      if (contexts.length === 0) throw new Error(name + ': unexpected snapshot context request')
      lastContext = contexts.shift()
      const value = JSON.parse(lastContext)
      value.snapshot = verificationSnapshot
      value.path = '.git/zdev/work-context/work/' + verificationSnapshot + '.json'
      return JSON.stringify(value)
    }}
    if (options.label.includes('confirm verifier')) {{
      return JSON.stringify({{ schema_version: 1, area, snapshot: verificationSnapshot, equal: true }})
    }}
    if (options.agentType) {{
      if (workers.length === 0) throw new Error(name + ': unexpected worker')
      return workers.shift()
    }}
    if (options.label.includes('coordinate derived split')) {{
      if (derived.length === 0) throw new Error(name + ': unexpected derived coordination')
      return derived.shift()
    }}
    if (options.label.includes('complete and commit')) {{
      if (completions.length === 0) throw new Error(name + ': unexpected completion')
      return completions.shift()
    }}
    throw new Error(name + ': unknown call ' + JSON.stringify(options))
  }})
  if (!result.startsWith(expectedPrefix + ' zdev-loop ' + area)) throw new Error(name + ': ' + result)
  if (contexts.length || workers.length || completions.length || derived.length || selections.length) throw new Error(name + ': unused fixture values')
  return {{ result, calls }}
}}

const twoTask = await exercise(
  'two-task continuation',
  [ready('work-001'), ready('work-001'), ready('work-002', 'standard', commit1), ready('work-002', 'standard', commit1), closed],
  [
    worker('work-001', 'implementer', 'ready'),
    worker('work-001', 'verifier', 'pass', 'none', passEvidence(head)),
    worker('work-002', 'implementer', 'ready'),
    worker('work-002', 'verifier', 'pass', 'none', passEvidence(commit1)),
  ],
  [completionPass('work-001'), completionPass('work-002')],
  'PASS',
)
if (!twoTask.result.includes('Tasks completed: work-001, work-002')) throw new Error(twoTask.result)
if (!twoTask.result.includes('Lifecycle: closed\nQueue: empty')) throw new Error(twoTask.result)
for (const call of twoTask.calls.filter(call => call.type === null
  && !call.label.includes('complete and commit'))) {{
  if (call.model !== 'haiku') throw new Error('loop deterministic coordination did not use Haiku: ' + call.label)
}}

const noWork = await exercise('closed no-work', [closed], [], [], 'PASS', [], 'work')
if (noWork.calls.length !== 1 || noWork.calls[0].label !== 'zdev work: select next task') throw new Error(JSON.stringify(noWork.calls))
await exercise('closed no-work array args', [closed], [], [], 'PASS', [], ['work'])

const focused = await exercise(
  'fuzzy focus selection',
  [ready('work-002'), ready('work-002'), closed],
  [worker('work-002', 'implementer', 'ready'), worker('work-002', 'verifier', 'pass', 'none', passEvidence(head))],
  [completionPass('work-002')],
  'PASS',
  [],
  'work focus on database cleanup',
  [
    JSON.stringify({{ task_id: 'work-002', ready: ['work-001', 'work-002'], reason: 'best fit' }}),
    JSON.stringify({{ task_id: null, ready: [], reason: 'frontier empty' }}),
  ],
)
const selectorPrompt = focused.calls.find(call => call.label.includes('choose from ready frontier'))?.prompt ?? ''
if (!selectorPrompt.includes('zdev task show work <task-id>') || !focused.result.includes('Focus: focus on database cleanup')) throw new Error(focused.result)

const rework = await exercise(
  'rework',
  [ready('work-001'), ready('work-001'), ready('work-001'), ready('work-001'), closed],
  [
    worker('work-001', 'implementer', 'ready'),
    worker('work-001', 'verifier', 'rework', 'none', [], ['fix defect']),
    worker('work-001', 'implementer', 'ready'),
    worker('work-001', 'verifier', 'pass', 'none', passEvidence(head)),
  ],
  [completionPass('work-001')],
  'PASS',
)
if (rework.calls.filter(call => call.type === 'zdev:zdev-verifier').length !== 2) throw new Error(JSON.stringify(rework.calls))

const resumed = await exercise(
  'cached completion result',
  [ready('work-001'), ready('work-001'), empty(commit1)],
  [
    worker('work-001', 'implementer', 'ready'),
    worker('work-001', 'verifier', 'pass', 'none', passEvidence(head)),
  ],
  [completionPass('work-001')],
  'PASS',
)
const completionIndex = resumed.calls.findIndex(call => call.label.includes('complete and commit'))
if (resumed.calls[completionIndex + 1]?.label !== 'zdev work: select next task') throw new Error(JSON.stringify(resumed.calls))

const staleResumed = await exercise(
  'cached result without repository advance',
  [ready('work-001'), ready('work-001'), ready('work-001', 'standard', commit1)],
  [
    worker('work-001', 'implementer', 'ready'),
    worker('work-001', 'verifier', 'pass', 'none', passEvidence(head)),
  ],
  [completionPass('work-001')],
  'BLOCKER',
)
if (staleResumed.calls.at(-1)?.label !== 'zdev work: select next task') throw new Error(JSON.stringify(staleResumed.calls))

const failed = await exercise(
  'completion failure',
  [ready('work-001'), ready('work-001')],
  [
    worker('work-001', 'implementer', 'ready'),
    worker('work-001', 'verifier', 'pass', 'none', passEvidence(head)),
  ],
  ['BLOCKER zdev-implement work work-001\n\nArea: work\nTask: work-001\nFailed stage: commit\nReason: commit failed\nPreserved state: task completion and index remain'],
  'BLOCKER',
)
if (!failed.calls.at(-1)?.label.includes('complete and commit')) throw new Error(JSON.stringify(failed.calls))

const decision = await exercise(
  'user decision',
  [ready('work-001', 'advanced')],
  [worker('work-001', 'planner', 'blocker', 'none', [], ['public API choice belongs to the user'])],
  [],
  'BLOCKER',
)
if (decision.calls.at(-1)?.type !== 'zdev:zdev-planner') throw new Error(JSON.stringify(decision.calls))

const splitProposal = 'PROPOSE zdev-derived work work-001\n' + JSON.stringify({{
  schema_version: 1, proposal: 'implementation_split', area, source_task: 'work-001',
  source_result: {{ status: 'split', summary: 'split direct work', validation: [] }},
  tasks: [{{ key: 'child', title: 'Implement child', blocked_by: [], outcome: 'Child completes direct work.', done_when: ['Done.'], validation: ['Check.'] }}],
  split_ownership: {{ retained_parent_paths: [], child_future_paths: [{{ key: 'child', paths: ['src/child.rs'] }}] }},
}})
const splitLoop = await exercise(
  'derived split continuation',
  [ready('work-001'), ready('work-002', 'standard', commit1), ready('work-002', 'standard', commit1), closed],
  [
    worker('work-001', 'implementer', 'blocker', 'none', [splitProposal]),
    worker('work-002', 'implementer', 'ready'),
    worker('work-002', 'verifier', 'pass', 'none', passEvidence(commit1)),
  ],
  [completionPass('work-002')],
  'PASS',
  ['PASS zdev-implement work work-001\n\nArea: work\nTask: work-001\nDerived proposal: implementation_split\nSummary: split applied\nChanged files: task records\nValidation: derive apply\nVerifier evidence: source remains open\nCommit ID: ' + commit1],
)
if (!splitLoop.result.includes('Tasks completed: work-002')) throw new Error(splitLoop.result)
if (splitLoop.result.includes('Tasks completed: work-001')) throw new Error(splitLoop.result)
if (!splitLoop.result.includes('Commits: ' + commit1 + ', ' + commit2)) throw new Error(splitLoop.result)
"#
    );
    let output = Command::new("node")
        .args(["--input-type=module", "--eval", &probe])
        .output()
        .expect("run Claude area-loop workflow probe");
    assert!(
        output.status.success(),
        "Claude area-loop workflow probe failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn work_context_round_trip_counts_match_realized_routes() {
    let audit = include_str!("../docs/workflow-round-trips.md");
    let loop_contract = include_str!("../docs/area-loop.md");
    for exact_row in [
        "| Codex, OpenCode, Pi, Oh My Pi | 5 / 9 / 2 / 2 | 2 / 5 / 0 / 1 | 7 / 15 / 2 / 4 |",
        "| Claude | 5 / 10 / 2 / 6 | 2 / 4 / 0 / 3 | 7 / 17 / 2 / 11 |",
    ] {
        assert!(audit.contains(exact_row), "missing count row {exact_row}");
    }
    assert!(audit.contains("Closed K performs no\nstatus or Git inspection"));
    assert!(audit.contains("one-task command does\nnot run an unused post-commit `next` or K"));
    assert!(audit.contains("CS is coordinator snapshot store and\nshow"));
    assert!(audit.contains("returns exactly verdict, summary, findings, and\nescalation"));
    assert!(
        loop_contract
            .contains("After each exact PASS and commit, select again from fresh evidence")
    );
    assert!(
        loop_contract.contains("collect fresh work-context before deciding or dispatching again")
    );
}

#[test]
fn complexity_routing_uses_the_typed_escalation_vocabulary() {
    let guidance = include_str!("../docs/task-complexity-routing.md");
    let routing = guidance
        .split("## Coordinator routing")
        .nth(1)
        .expect("coordinator routing")
        .split("## Smallest implementation seam")
        .next()
        .expect("routing boundary");

    assert!(routing.contains("verifier verdict `rework`"));
    assert!(routing.contains("`advanced-implementer`"));
    assert!(!routing.contains("`REWORK`"));
    assert!(!routing.contains("`BLOCKER`"));
    assert!(!routing.contains("Escalation: strong-implementer"));
}

#[test]
fn all_harness_audit_entrypoints_are_discoverable_and_use_the_verifier_contract() {
    let repository = repository();
    let root = repository.path();
    let config_home = root.join("audit-worker-config");
    let environment = [("XDG_CONFIG_HOME", config_home.as_path())];

    for (harness, skill_root, adapter) in [
        ("codex", "zdev", None),
        ("claude", "skills/zdev", Some("workflows/zdev-audit.js")),
        (
            "opencode",
            "skills/zdev-opencode",
            Some("commands/zdev-audit.md"),
        ),
        ("pi", "skills/zdev-pi", Some("prompts/zdev-audit.md")),
        ("omp", "skills/zdev", Some("prompts/zdev-audit.md")),
    ] {
        let destination = root.join(format!("audit-{harness}"));
        json_output_with_env(
            root,
            &[
                "skill",
                "install",
                harness,
                "--to",
                destination.to_str().expect("audit destination"),
            ],
            &environment,
        );
        let skill_files = file_inventory(&destination)
            .into_iter()
            .filter(|path| path.ends_with("SKILL.md"))
            .collect::<Vec<_>>();
        assert_eq!(skill_files, [format!("{skill_root}/SKILL.md")], "{harness}");
        assert!(
            destination
                .join(skill_root)
                .join("references/audit.md")
                .is_file()
        );
        if let Some(adapter) = adapter {
            assert!(destination.join(adapter).is_file());
        }
        if harness == "claude" {
            let manifest: Value = serde_json::from_slice(
                &fs::read(destination.join(".claude-plugin/plugin.json"))
                    .expect("Claude audit manifest"),
            )
            .expect("Claude audit manifest JSON");
            assert_eq!(manifest["workflows"], "./workflows/");
        }
        assert_eq!(
            json_output_with_env(
                root,
                &[
                    "skill",
                    "check",
                    harness,
                    "--to",
                    destination.to_str().expect("audit destination"),
                ],
                &environment,
            )["status"],
            "ok"
        );
    }
}

#[test]
fn claude_audit_uses_one_default_verifier_and_bounds_explicit_lenses() {
    let source = include_str!("../templates/zdev/claude/workflows/zdev-audit.js")
        .replacen("export const meta =", "const meta =", 1)
        .replace(
            "{{audit_contract}}",
            &serde_json::to_string("audit contract").expect("audit contract JSON"),
        )
        .replace(
            "{{repository_guidance}}",
            &serde_json::to_string("repository guidance").expect("repository guidance JSON"),
        );
    let probe = format!(
        r#"
async function run(args, agent, pipeline) {{
{source}
}}
const publicResult = 'PASS zdev-audit\n\nBoundary: src\nInspected: src\nOmitted: none\nChecked evidence: cargo test'
const defaultCalls = []
const defaultResult = await run(
  'src',
  async (_prompt, options) => {{ defaultCalls.push(options.label); return publicResult }},
  async () => {{ throw new Error('default audit used pipeline') }},
)
if (defaultResult !== publicResult) throw new Error('default result changed')
if (JSON.stringify(defaultCalls) !== JSON.stringify(['audit checking verifier'])) throw new Error(`default calls: ${{JSON.stringify(defaultCalls)}}`)

const boundedCalls = []
const boundedPrompts = []
const boundedResult = await run(
  ['src', 'api', 'tests', 'safety', 'usability'],
  async (prompt, options) => {{
    boundedCalls.push(options.label)
    boundedPrompts.push(prompt)
    return options.label === 'audit evidence vetter' ? publicResult : `candidate from ${{options.label}}`
  }},
  async (scopes, dispatch) => Promise.all(scopes.map(dispatch)),
)
if (boundedResult !== publicResult) throw new Error('bounded result changed')
if (boundedCalls.length !== 5 || boundedCalls.filter(label => label === 'audit evidence vetter').length !== 1) throw new Error(`bounded calls: ${{JSON.stringify(boundedCalls)}}`)
const vetterPrompt = boundedPrompts.at(-1) ?? ''
for (const lens of ['api', 'tests', 'safety', 'usability']) {{
  if (!vetterPrompt.includes('Lens: ' + lens)) throw new Error('unlabeled lens: ' + lens)
}}
if (!vetterPrompt.includes('repository guidance')) throw new Error('repository guidance missing')

const incompleteResult = await run(
  {{ boundary: 'src', lenses: ['api', 'tests'] }},
  async () => 'candidate',
  async (scopes, dispatch) => [await dispatch(scopes[0])],
)
if (!incompleteResult.startsWith('BLOCKER zdev-audit\n') || !incompleteResult.includes('every requested lens must return a non-empty result')) throw new Error(incompleteResult)

let excessiveCalls = 0
const excessiveResult = await run(
  {{ boundary: 'src', lenses: ['one', 'two', 'three', 'four', 'five'] }},
  async () => {{ excessiveCalls += 1; throw new Error('excessive audit started agent') }},
  async () => {{ excessiveCalls += 1; throw new Error('excessive audit started pipeline') }},
)
if (excessiveCalls !== 0) throw new Error(`excessive calls: ${{excessiveCalls}}`)
if (!excessiveResult.startsWith('BLOCKER zdev-audit\n') || !excessiveResult.includes('5 lenses exceed the maximum of 4')) throw new Error(`excessive result: ${{excessiveResult}}`)
"#
    );
    let output = Command::new("node")
        .args(["--input-type=module", "--eval", &probe])
        .output()
        .expect("run Claude audit workflow probe");
    assert!(
        output.status.success(),
        "Claude audit workflow probe failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn all_harness_task_workflows_are_discoverable_and_keep_coordinator_boundaries() {
    let repository = repository();
    let root = repository.path();
    let config_home = root.join("workflow-worker-config");
    let environment = [("XDG_CONFIG_HOME", config_home.as_path())];

    for (harness, skill_root, adapters) in [
        ("codex", "zdev", None),
        (
            "claude",
            "skills/zdev",
            Some(("workflows/zdev-implement.js", "workflows/zdev-verify.js")),
        ),
        (
            "opencode",
            "skills/zdev-opencode",
            Some(("commands/zdev-implement.md", "commands/zdev-verify.md")),
        ),
        (
            "pi",
            "skills/zdev-pi",
            Some(("prompts/zdev-implement.md", "prompts/zdev-verify.md")),
        ),
        (
            "omp",
            "skills/zdev",
            Some(("prompts/zdev-implement.md", "prompts/zdev-verify.md")),
        ),
    ] {
        let destination = root.join(format!("workflows-{harness}"));
        json_output_with_env(
            root,
            &[
                "skill",
                "install",
                harness,
                "--to",
                destination.to_str().expect("workflow destination"),
            ],
            &environment,
        );
        if harness != "claude" {
            let mut coordinators = vec![format!("{skill_root}/SKILL.md")];
            if let Some((implement, verify)) = adapters {
                coordinators.extend([implement.to_owned(), verify.to_owned()]);
            }
            match harness {
                "opencode" => coordinators.push("commands/zdev-loop.md".to_owned()),
                "pi" | "omp" => coordinators.push("prompts/zdev-loop.md".to_owned()),
                _ => {}
            }
            assert_task_workflows_locator(
                &destination,
                skill_root,
                &coordinators.iter().map(String::as_str).collect::<Vec<_>>(),
            );
        }
        assert!(
            destination
                .join(skill_root)
                .join("references/task-workflows.md")
                .is_file()
        );
        assert!(
            destination
                .join(skill_root)
                .join("references/verify-workflow.md")
                .is_file()
        );
        let task_contract = fs::read_to_string(
            destination
                .join(skill_root)
                .join("references/task-workflows.md"),
        )
        .expect("task workflow contract");
        assert!(task_contract.contains("exactly those four unique keys"));
        assert!(
            task_contract.contains("Immediately before every verifier dispatch, coordination runs")
        );
        assert!(
            task_contract
                .contains("Coordination then constructs the compatible public verifier envelope")
        );
        assert!(
            task_contract.contains(
                "exact `validation_write: <normalized repository-relative path>` finding"
            )
        );
        let verify_contract = fs::read_to_string(
            destination
                .join(skill_root)
                .join("references/verify-workflow.md"),
        )
        .expect("verify workflow contract");
        assert!(
            verify_contract.contains("Legacy nine-key verifier objects are\ninvalid worker output")
        );
        assert!(verify_contract.contains("coordination runs `zdev work-context <area> --compare"));
        if let Some((implement, verify)) = adapters {
            assert!(destination.join(implement).is_file());
            assert!(destination.join(verify).is_file());
            for adapter in [implement, verify] {
                let rendered =
                    fs::read_to_string(destination.join(adapter)).expect("rendered task adapter");
                assert!(
                    rendered.contains("four semantic fields")
                        || rendered.contains("four-field")
                        || rendered.contains("verifier object")
                        || rendered.contains("exactly these four keys"),
                    "{harness} {adapter}"
                );
                assert!(rendered.contains("coordinator"), "{harness} {adapter}");
            }
        }

        assert_eq!(
            json_output_with_env(
                root,
                &[
                    "skill",
                    "check",
                    harness,
                    "--to",
                    destination.to_str().expect("workflow destination"),
                ],
                &environment,
            )["status"],
            "ok"
        );
    }
}

#[test]
fn omp_planner_constrains_one_result_without_formatting_retries() {
    let planner = include_str!("../templates/zdev/omp/agents/zdev-planner.md");
    let expected_schema = r#"output:
  type: object
  additionalProperties: false
  required: [verdict, summary, plan, findings]
  properties:
    verdict:
      type: string
      enum: [plan, blocker]
    summary:
      type: string
      minLength: 1
    plan:
      anyOf:
        - type: "null"
        - type: object
          additionalProperties: false
          required: [approach, paths, validation]
          properties:
            approach:
              type: string
              minLength: 1
            paths:
              type: array
              minItems: 1
              items:
                type: string
                minLength: 1
            validation:
              type: array
              minItems: 1
              items:
                type: string
                minLength: 1
    findings:
      type: array
      items:
        type: string
        minLength: 1
"#;
    assert!(planner.contains(expected_schema));

    for guidance in [
        include_str!("../templates/zdev/omp-skill.md"),
        include_str!("../templates/zdev/omp/prompts/zdev-implement.md"),
        include_str!("../templates/zdev/omp/prompts/zdev-loop.md"),
    ] {
        let normalized = guidance.split_whitespace().collect::<Vec<_>>().join(" ");
        assert!(normalized.contains("details.results[].structuredOutput.data"));
        assert!(normalized.contains("result's output"));
        assert!(normalized.contains("balanced JSON object"));
        assert!(normalized.contains("brief prose or a Markdown fence"));
        assert!(normalized.contains("formatting follow-up"));
        assert!(!normalized.contains("strictly validate the complete JSON string"));
    }

    let verifier = include_str!("../templates/zdev/omp/agents/zdev-verifier.md");
    let audit = include_str!("../templates/zdev/omp/prompts/zdev-audit.md");
    assert!(!verifier.contains("\noutput:"));
    assert!(!audit.contains("structuredOutput"));
}

#[test]
fn bounded_area_loop_aliases_share_one_stop_and_restart_contract() {
    let repository = repository();
    let root = repository.path();
    let config_home = root.join("bounded-loop-config");
    let environment = [("XDG_CONFIG_HOME", config_home.as_path())];

    for (harness, directory) in [("opencode", "commands"), ("pi", "prompts")] {
        let destination = root.join(format!("bounded-loop-{harness}"));
        json_output_with_env(
            root,
            &[
                "skill",
                "install",
                harness,
                "--to",
                destination.to_str().expect("loop destination"),
            ],
            &environment,
        );
        let canonical = fs::read_to_string(destination.join(directory).join("zdev-loop.md"))
            .expect("canonical bounded loop");
        let alias = fs::read_to_string(destination.join(directory).join("zdev-goal.md"))
            .expect("bounded loop alias");
        assert_eq!(canonical, alias, "{harness} aliases must be byte-identical");
        assert!(canonical.contains("remaining text as\noptional fuzzy focus"));
        assert!(canonical.contains("complete ready frontier"));
        assert!(canonical.contains("zdev task show <area>\n<task-id> --format json"));
        assert!(canonical.contains("--task <task-id> --store --format json"));
        assert!(canonical.contains("AFK, priority, then numeric order"));
        assert_eq!(
            json_output_with_env(
                root,
                &[
                    "skill",
                    "check",
                    harness,
                    "--to",
                    destination.to_str().expect("loop destination"),
                ],
                &environment,
            )["status"],
            "ok"
        );
    }
}

#[test]
fn native_loop_harnesses_install_one_skill_and_their_area_adapter() {
    let repository = repository();
    let root = repository.path();
    let config_home = root.join("native-loop-config");
    let environment = [("XDG_CONFIG_HOME", config_home.as_path())];

    for harness in ["codex", "omp"] {
        let destination = root.join(format!("native-loop-{harness}"));
        json_output_with_env(
            root,
            &[
                "skill",
                "install",
                harness,
                "--to",
                destination.to_str().expect("native loop destination"),
            ],
            &environment,
        );
        if harness == "codex" {
            assert_eq!(
                file_inventory(&destination)
                    .into_iter()
                    .filter(|path| path.ends_with("SKILL.md"))
                    .collect::<Vec<_>>(),
                ["zdev/SKILL.md"]
            );
            assert!(destination.join("zdev/references/area-loop.md").is_file());
            let loop_contract =
                fs::read_to_string(destination.join("zdev/references/area-loop.md"))
                    .expect("Codex native loop contract");
            assert!(loop_contract.contains("complete ready frontier"));
            assert!(loop_contract.contains("--task <task-id> --store --format json"));
        } else {
            let canonical =
                fs::read(destination.join("prompts/zdev-loop.md")).expect("native loop adapter");
            let alias =
                fs::read(destination.join("prompts/zdev-goal.md")).expect("native goal adapter");
            assert_eq!(canonical, alias, "OMP aliases must be byte-identical");
            let canonical = String::from_utf8(canonical).expect("OMP loop text");
            assert!(canonical.contains("complete ready frontier"));
            assert!(canonical.contains("--task <task-id> --store --format json"));
            assert!(
                destination
                    .join("skills/zdev/references/area-loop.md")
                    .is_file()
            );
        }
        assert_eq!(
            json_output_with_env(
                root,
                &[
                    "skill",
                    "check",
                    harness,
                    "--to",
                    destination.to_str().expect("native loop destination"),
                ],
                &environment,
            )["status"],
            "ok"
        );
    }
}

#[test]
fn non_claude_worker_handoffs_are_compact_and_wrapper_tolerant() {
    for prompt in [
        include_str!("../templates/zdev/opencode/commands/zdev-implement.md"),
        include_str!("../templates/zdev/pi/prompts/zdev-implement.md"),
        include_str!("../templates/zdev/omp/prompts/zdev-implement.md"),
        include_str!("../templates/zdev/omp/prompts/zdev-loop.md"),
    ] {
        assert!(prompt.contains("installed route-contract path"));
        assert!(prompt.contains("balanced"));
        assert!(prompt.contains("Markdown fence"));
        assert!(!prompt.contains("complete rendered contract"));
        assert!(!prompt.contains("strictly validate the complete JSON string"));
    }

    let pi = include_str!("../templates/zdev/pi/extensions/zdev-subagent.ts");
    assert!(pi.contains("Installed route-contract path plus compact file paths"));
    assert!(!pi.contains("Complete rendered task or audit contract"));
}

#[test]
fn worker_roles_state_their_semantic_result_contracts() {
    for prompt in [
        include_str!("../templates/zdev/claude/agents/zdev-routine-implementer.md"),
        include_str!("../templates/zdev/claude/agents/zdev-advanced-implementer.md"),
        include_str!("../templates/zdev/opencode/agents/zdev-routine-implementer.md"),
        include_str!("../templates/zdev/opencode/agents/zdev-advanced-implementer.md"),
        include_str!("../templates/zdev/omp/agents/zdev-routine-implementer.md"),
        include_str!("../templates/zdev/omp/agents/zdev-advanced-implementer.md"),
    ] {
        assert!(prompt.contains("schema_version: 1"));
        assert!(prompt.contains("kind: \"implementer\""));
        assert!(prompt.contains("`ready` or `blocker`"));
        assert!(prompt.contains("string arrays `evidence`"));
        assert!(prompt.contains("escalation: \"none\""));
        assert!(prompt.contains("Put changed files and validation in"));
    }

    for prompt in [
        include_str!("../templates/zdev/claude/agents/zdev-verifier.md"),
        include_str!("../templates/zdev/opencode/agents/zdev-verifier.md"),
        include_str!("../templates/zdev/omp/agents/zdev-verifier.md"),
    ] {
        let prompt = prompt.split_whitespace().collect::<Vec<_>>().join(" ");
        assert!(prompt.contains("For task verification only"));
        assert!(prompt.contains("`pass` with no findings"));
        assert!(prompt.contains("`rework` with at least one finding"));
        assert!(prompt.contains("`rework` may request `advanced-implementer`"));
        assert!(prompt.contains("For audit only, ignore the task-verification JSON contract"));
        assert!(prompt.contains("supplied textual audit envelope"));
    }

    let pi = include_str!("../templates/zdev/pi/extensions/zdev-subagent.ts");
    assert_eq!(
        pi.matches("schema_version: 1, kind: \\\"implementer\\\"")
            .count(),
        3
    );
    assert_eq!(pi.matches("escalation: \\\"none\\\"").count(), 3);
    assert!(pi.contains("For task verification only"));
    assert!(pi.contains("pass with no findings"));
    assert!(pi.contains("rework with at least one finding"));
    assert!(pi.contains("For audit only, ignore the task-verification JSON contract"));
}

#[test]
fn legacy_task_entrypoints_require_forced_allowlisted_migration() {
    use std::os::unix::fs::symlink;

    let repository = repository();
    let root = repository.path();
    for (harness, legacy_path, new_path, legacy_is_symlink) in [
        (
            "opencode",
            "command/zdev-task.md",
            "commands/zdev-implement.md",
            false,
        ),
        (
            "pi",
            "prompts/zdev-task.md",
            "prompts/zdev-implement.md",
            true,
        ),
    ] {
        let destination = root.join(format!("legacy-task-{harness}"));
        let legacy = destination.join(legacy_path);
        let unrelated = destination.join("unrelated/team-workflow.md");
        fs::create_dir_all(legacy.parent().expect("legacy parent")).expect("legacy parent");
        fs::create_dir_all(unrelated.parent().expect("unrelated parent"))
            .expect("unrelated parent");
        fs::write(&unrelated, "team workflow\n").expect("unrelated file");
        if legacy_is_symlink {
            symlink(&unrelated, &legacy).expect("legacy symlink");
        } else {
            fs::write(&legacy, "legacy task\n").expect("legacy file");
        }
        let before = file_inventory(&destination);

        let refused = run_zdev(
            root,
            &[
                "skill",
                "install",
                harness,
                "--to",
                destination.to_str().expect("migration destination"),
            ],
        );
        assert!(!refused.status.success());
        let error = String::from_utf8_lossy(&refused.stderr);
        assert!(error.contains("legacy zdev entrypoint"));
        assert!(error.contains("--force"));
        assert_eq!(file_inventory(&destination), before);
        assert!(!destination.join(new_path).exists());

        let check = run_zdev(
            root,
            &[
                "skill",
                "check",
                harness,
                "--to",
                destination.to_str().expect("migration destination"),
            ],
        );
        assert_eq!(check.status.code(), Some(1));
        assert!(String::from_utf8_lossy(&check.stdout).contains("--force"));

        let migrated = json_output(
            root,
            &[
                "skill",
                "install",
                harness,
                "--to",
                destination.to_str().expect("migration destination"),
                "--force",
            ],
        );
        assert_eq!(migrated["status"], "replaced");
        assert!(destination.join(new_path).is_file());
        assert!(fs::symlink_metadata(&legacy).is_err());
        assert_eq!(
            fs::read_to_string(&unrelated).expect("preserved unrelated"),
            "team workflow\n"
        );
        assert_eq!(
            json_output(
                root,
                &[
                    "skill",
                    "check",
                    harness,
                    "--to",
                    destination.to_str().expect("migration destination"),
                ],
            )["status"],
            "ok"
        );
    }
}

#[test]
fn audit_migration_is_allowlisted_and_invalid_profiles_fail_before_publication() {
    let repository = repository();
    let root = repository.path();
    let destination = root.join("audit-opencode-migration");
    let legacy = destination.join("command/zdev-audit.md");
    let unrelated = destination.join("commands/team-audit.md");
    fs::create_dir_all(legacy.parent().expect("legacy parent")).expect("legacy directory");
    fs::create_dir_all(unrelated.parent().expect("unrelated parent")).expect("unrelated directory");
    fs::write(&legacy, "legacy zdev audit\n").expect("legacy audit");
    fs::write(&unrelated, "team audit\n").expect("unrelated audit");
    let before = file_inventory(&destination);

    let config_home = root.join("invalid-audit-workers");
    let global_workers = config_home.join("zdev/workers.toml");
    fs::create_dir_all(global_workers.parent().expect("worker parent")).expect("worker directory");
    let environment = [("XDG_CONFIG_HOME", config_home.as_path())];
    let refused = run_zdev_with_env(
        root,
        &[
            "skill",
            "install",
            "opencode",
            "--to",
            destination.to_str().expect("migration destination"),
        ],
        &environment,
    );
    assert!(!refused.status.success());
    let refusal = String::from_utf8_lossy(&refused.stderr);
    assert!(refusal.contains("legacy zdev entrypoint"));
    assert!(refusal.contains("--force"));
    assert_eq!(file_inventory(&destination), before);
    assert!(!destination.join("commands/zdev-audit.md").exists());

    let checked = run_zdev_with_env(
        root,
        &[
            "skill",
            "check",
            "opencode",
            "--to",
            destination.to_str().expect("migration destination"),
        ],
        &environment,
    );
    assert_eq!(checked.status.code(), Some(1));
    let readiness = String::from_utf8_lossy(&checked.stdout);
    assert!(readiness.contains("differs from this version"));
    assert!(readiness.contains("--force"));
    assert_eq!(file_inventory(&destination), before);

    fs::write(
        &global_workers,
        "schema_version = 1\n\n[opencode.verifier]\nmodel = \"anthropic/custom\"\neffort = \"high\"\n",
    )
    .expect("unsupported audit verifier");
    let rejected = run_zdev_with_env(
        root,
        &[
            "skill",
            "install",
            "opencode",
            "--to",
            destination.to_str().expect("migration destination"),
            "--force",
        ],
        &environment,
    );
    assert!(!rejected.status.success());
    assert_eq!(file_inventory(&destination), before);
    assert_eq!(
        fs::read_to_string(&legacy).expect("preserved legacy"),
        "legacy zdev audit\n"
    );

    fs::remove_file(global_workers).expect("remove invalid worker profile");
    let migrated = json_output_with_env(
        root,
        &[
            "skill",
            "install",
            "opencode",
            "--to",
            destination.to_str().expect("migration destination"),
            "--force",
        ],
        &environment,
    );
    assert_eq!(migrated["status"], "replaced");
    assert!(!legacy.exists());
    assert!(destination.join("commands/zdev-audit.md").is_file());
    assert_eq!(
        fs::read_to_string(unrelated).expect("unrelated audit"),
        "team audit\n"
    );
    assert_eq!(
        json_output_with_env(
            root,
            &[
                "skill",
                "check",
                "opencode",
                "--to",
                destination.to_str().expect("migration destination"),
            ],
            &environment,
        )["status"],
        "ok"
    );

    let codex = root.join("audit-codex-shared-root");
    fs::create_dir_all(codex.join("team-skill")).expect("unrelated Codex skill directory");
    fs::write(codex.join("team-skill/SKILL.md"), "team skill\n").expect("unrelated Codex skill");
    json_output_with_env(
        root,
        &[
            "skill",
            "install",
            "codex",
            "--to",
            codex.to_str().expect("Codex shared root"),
            "--force",
        ],
        &environment,
    );
    assert_eq!(
        fs::read_to_string(codex.join("team-skill/SKILL.md")).expect("preserved Codex skill"),
        "team skill\n"
    );
}

#[test]
fn codex_force_install_migrates_split_skills_to_one_router() {
    let repository = repository();
    let root = repository.path();
    let destination = root.join("codex-split-skill-migration");
    let legacy_files = [
        "zdev-audit/SKILL.md",
        "zdev-audit/agents/openai.yaml",
        "zdev-implement/SKILL.md",
        "zdev-implement/agents/openai.yaml",
        "zdev-verify/SKILL.md",
        "zdev-verify/agents/openai.yaml",
        "zdev-loop/SKILL.md",
        "zdev-loop/agents/openai.yaml",
        "zdev-goal/SKILL.md",
        "zdev-goal/agents/openai.yaml",
    ];
    for path in legacy_files {
        let path = destination.join(path);
        fs::create_dir_all(path.parent().expect("legacy parent")).expect("legacy directory");
        fs::write(path, "old managed file\n").expect("legacy file");
    }
    fs::create_dir_all(destination.join("team-skill")).expect("team skill directory");
    fs::write(destination.join("team-skill/SKILL.md"), "team skill\n").expect("team skill");

    let refused = run_zdev(
        root,
        &[
            "skill",
            "install",
            "codex",
            "--to",
            destination.to_str().expect("Codex destination"),
        ],
    );
    assert!(!refused.status.success());

    json_output(
        root,
        &[
            "skill",
            "install",
            "codex",
            "--to",
            destination.to_str().expect("Codex destination"),
            "--force",
        ],
    );
    assert_eq!(
        file_inventory(&destination)
            .into_iter()
            .filter(|path| path.ends_with("SKILL.md"))
            .collect::<Vec<_>>(),
        ["team-skill/SKILL.md", "zdev/SKILL.md"]
    );
    for directory in [
        "zdev-audit",
        "zdev-implement",
        "zdev-verify",
        "zdev-loop",
        "zdev-goal",
    ] {
        assert!(!destination.join(directory).exists());
    }
}

#[test]
fn executable_templates_realize_deterministically_and_match_generated_fixtures() {
    let repository = repository();
    let root = repository.path();
    let source = Path::new(env!("CARGO_MANIFEST_DIR"));
    let config_home = root.join("empty-worker-config");
    let environment = [("XDG_CONFIG_HOME", config_home.as_path())];

    for (harness, checked_in_root) in [
        ("codex", Some("skills")),
        ("claude", Some(".claude/skills/zdev")),
        ("opencode", Some(".opencode")),
        ("pi", Some(".pi")),
        ("omp", Some(".omp")),
    ] {
        let destination = root.join(format!("checked-in-{harness}"));
        json_output_with_env(
            root,
            &[
                "skill",
                "install",
                harness,
                "--to",
                destination.to_str().expect("integration destination"),
            ],
            &environment,
        );
        let second = root.join(format!("deterministic-{harness}"));
        json_output_with_env(
            root,
            &[
                "skill",
                "install",
                harness,
                "--to",
                second.to_str().expect("second integration destination"),
            ],
            &environment,
        );
        let inventory = file_inventory(&destination);
        assert_eq!(inventory, file_inventory(&second));
        assert!(
            inventory.iter().all(|path| !path.contains("unslop")),
            "{harness} must not install a separate unslop artifact"
        );
        for path in inventory {
            let rendered = normalize_task_workflows_locator(
                fs::read(destination.join(&path)).expect("rendered integration file"),
            );
            if path.ends_with("SKILL.md") || path.contains("/references/") {
                continue;
            }
            assert_eq!(
                rendered,
                normalize_task_workflows_locator(
                    fs::read(second.join(&path)).expect("second rendered integration file"),
                ),
                "{harness} integration file {path} was not byte deterministic"
            );
            if let Some(checked_in_root) = checked_in_root {
                assert_eq!(
                    rendered,
                    normalize_task_workflows_locator(
                        fs::read(source.join(checked_in_root).join(&path))
                            .expect("checked-in integration file"),
                    ),
                    "checked-in {harness} integration file {path} drifted from its template"
                );
            }
        }
    }
}

#[test]
fn config_show_and_get_render_effective_and_scoped_layers() {
    let repository = repository();
    let root = repository.path();
    fs::create_dir(root.join(".zdev")).expect("zdev directory");
    fs::write(
        root.join(".zdev/config.toml"),
        "schema_version = 1\n\n[project]\nname = \"checkout\"\nrecord = \"project\"\ndefault_area = \"payments\"\n",
    )
    .expect("project config");
    fs::write(
        root.join(".zdev/workers.toml"),
        "schema_version = 1\n\n[codex.implementer]\nmodel = \"gpt-5.6-sol\"\neffort = \"high\"\n\n[claude.verifier]\ninherit = true\n",
    )
    .expect("local workers");
    let home = root.join("home");
    let global_path = home.join(".config/zdev/workers.toml");
    fs::create_dir_all(global_path.parent().expect("global worker parent"))
        .expect("global worker directory");
    fs::write(
        &global_path,
        "schema_version = 1\n\n[codex.implementer]\nmodel = \"gpt-5.5\"\neffort = \"xhigh\"\n\n[codex.verifier]\nmodel = \"gpt-5.5\"\neffort = \"high\"\n\n[claude.verifier]\nmodel = \"claude-opus-5\"\neffort = \"medium\"\n\n[pi.implementer]\nmodel = \"openai/gpt-5.5\"\neffort = \"high\"\n",
    )
    .expect("global workers");
    let relative_xdg = Path::new("relative-config-home");
    let environment = [("XDG_CONFIG_HOME", relative_xdg), ("HOME", home.as_path())];
    let global = global_path.to_string_lossy().replace('\\', "/");

    let shown = run_zdev_with_env(root, &["config", "show"], &environment);
    assert!(shown.status.success());
    let expected = format!(
        "project.name = \"checkout\"  [local .zdev/config.toml]\n\
project.record = \"project\"  [local .zdev/config.toml]\n\
project.trunk = null  [default]\n\
project.default-area = \"payments\"  [local .zdev/config.toml]\n\
  shadows null  [default]\n\
project.guidance = \"auto\"  [default]\n\
worker.codex.routine-implementer = {{ model = \"gpt-5.6-luna\", effort = \"low\" }}  [default]\n\
worker.codex.implementer = {{ model = \"gpt-5.6-sol\", effort = \"high\" }}  [local .zdev/workers.toml]\n\
  shadows {{ model = \"gpt-5.5\", effort = \"xhigh\" }}  [global {global}]\n\
  shadows {{ model = \"gpt-5.6-sol\", effort = \"low\" }}  [default]\n\
worker.codex.verifier = {{ model = \"gpt-5.5\", effort = \"high\" }}  [global {global}]\n\
  shadows {{ model = \"gpt-5.6-sol\", effort = \"low\" }}  [default]\n\
worker.codex.advanced-implementer = {{ model = \"gpt-5.6-sol\", effort = \"high\" }}  [default]\n\
worker.claude.routine-implementer = {{ model = \"haiku\", effort = \"low\" }}  [default]\n\
worker.claude.implementer = {{ model = \"claude-opus-5\", effort = \"low\" }}  [default]\n\
worker.claude.verifier = {{ inherit = true }}  [local .zdev/workers.toml]\n\
  shadows {{ model = \"claude-opus-5\", effort = \"medium\" }}  [global {global}]\n\
  shadows {{ model = \"claude-opus-5\", effort = \"low\" }}  [default]\n\
worker.claude.advanced-implementer = {{ model = \"claude-opus-5\", effort = \"high\" }}  [default]\n\
worker.opencode.routine-implementer = {{ model = \"openai/gpt-5.6-luna\", effort = \"low\" }}  [default]\n\
worker.opencode.implementer = {{ model = \"openai/gpt-5.6-sol\", effort = \"low\" }}  [default]\n\
worker.opencode.verifier = {{ model = \"anthropic/claude-opus-5\", effort = \"inherit\" }}  [default]\n\
worker.opencode.advanced-implementer = {{ model = \"openai/gpt-5.6-sol\", effort = \"high\" }}  [default]\n\
worker.pi.routine-implementer = {{ model = \"openai/gpt-5.6-luna\", effort = \"low\" }}  [default]\n\
worker.pi.implementer = {{ model = \"openai/gpt-5.5\", effort = \"high\" }}  [global {global}]\n\
  shadows {{ model = \"openai/gpt-5.6-sol\", effort = \"low\" }}  [default]\n\
worker.pi.verifier = {{ model = \"anthropic/claude-opus-5\", effort = \"low\" }}  [default]\n\
worker.pi.advanced-implementer = {{ model = \"openai/gpt-5.6-sol\", effort = \"high\" }}  [default]\n\
worker.omp.routine-implementer = {{ model = \"openai/gpt-5.6-luna\", effort = \"low\" }}  [default]\n\
worker.omp.implementer = {{ model = \"openai/gpt-5.6-sol\", effort = \"low\" }}  [default]\n\
worker.omp.verifier = {{ model = \"anthropic/claude-opus-5\", effort = \"low\" }}  [default]\n\
worker.omp.advanced-implementer = {{ model = \"openai/gpt-5.6-sol\", effort = \"high\" }}  [default]\n"
    )
    .replace("\nshadows", "\n  shadows");
    assert_eq!(
        String::from_utf8(shown.stdout).expect("human output"),
        expected
    );

    let effective = json_output_with_env(root, &["config", "show"], &environment);
    assert_eq!(effective["scope"], "effective");
    let values = effective["values"].as_array().expect("effective values");
    assert_eq!(values.len(), 25);
    assert_eq!(
        values
            .iter()
            .map(|entry| entry["key"].as_str().expect("configuration key"))
            .collect::<Vec<_>>(),
        [
            "project.name",
            "project.record",
            "project.trunk",
            "project.default-area",
            "project.guidance",
            "worker.codex.routine-implementer",
            "worker.codex.implementer",
            "worker.codex.verifier",
            "worker.codex.advanced-implementer",
            "worker.claude.routine-implementer",
            "worker.claude.implementer",
            "worker.claude.verifier",
            "worker.claude.advanced-implementer",
            "worker.opencode.routine-implementer",
            "worker.opencode.implementer",
            "worker.opencode.verifier",
            "worker.opencode.advanced-implementer",
            "worker.pi.routine-implementer",
            "worker.pi.implementer",
            "worker.pi.verifier",
            "worker.pi.advanced-implementer",
            "worker.omp.routine-implementer",
            "worker.omp.implementer",
            "worker.omp.verifier",
            "worker.omp.advanced-implementer",
        ]
    );
    assert_eq!(values[2]["value"], Value::Null);
    assert_eq!(values[3]["shadowed"][0]["value"], Value::Null);
    assert_eq!(values[6]["origin"]["scope"], "local");
    assert_eq!(values[6]["shadowed"][0]["origin"]["path"], global);
    assert_eq!(values[11]["value"], json!({"inherit": true}));
    assert_eq!(
        values[15]["value"],
        json!({"model": "anthropic/claude-opus-5", "effort": "inherit"})
    );

    let got = run_zdev_with_env(
        root,
        &["config", "get", "worker.codex.implementer"],
        &environment,
    );
    assert!(got.status.success());
    assert_eq!(
        String::from_utf8(got.stdout).expect("get output"),
        format!(
            "worker.codex.implementer = {{ model = \"gpt-5.6-sol\", effort = \"high\" }}  [local .zdev/workers.toml]\n  shadows {{ model = \"gpt-5.5\", effort = \"xhigh\" }}  [global {global}]\n  shadows {{ model = \"gpt-5.6-sol\", effort = \"low\" }}  [default]\n"
        )
    );
    let got_json = json_output_with_env(
        root,
        &["config", "get", "worker.codex.implementer"],
        &environment,
    );
    assert_eq!(got_json["key"], "worker.codex.implementer");
    assert_eq!(got_json["origin"]["scope"], "local");
    assert_eq!(got_json["shadowed"].as_array().expect("shadows").len(), 2);

    fs::write(
        &global_path,
        "schema_version = 1\n\n[codex.implementer]\nmodel = \"gpt-5.5\"\neffort = \"xhigh\"\n\n[codex.verifier]\ninherit = true\n",
    )
    .expect("scoped global workers");
    let scoped = run_zdev_with_env(root, &["config", "show", "--global"], &environment);
    assert!(scoped.status.success());
    assert_eq!(
        String::from_utf8(scoped.stdout).expect("scoped output"),
        format!(
            "worker.codex.implementer = {{ model = \"gpt-5.5\", effort = \"xhigh\" }}  [global {global}]\nworker.codex.verifier = {{ inherit = true }}  [global {global}]\n"
        )
    );
    let scoped_json = json_output_with_env(root, &["config", "show", "--global"], &environment);
    assert_eq!(scoped_json["scope"], "global");
    assert_eq!(scoped_json["values"].as_array().expect("values").len(), 2);
    assert_eq!(scoped_json["values"][1]["shadowed"], json!([]));
    assert_eq!(scoped_json["values"][1]["value"], json!({"inherit": true}));
}

#[test]
fn config_reads_reject_strict_or_absent_values_without_mutation() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    let local_path = root.join(".zdev/workers.toml");
    fs::write(
        &local_path,
        "schema_version = 1\n\n[codex.implementer]\ninherit = true\n",
    )
    .expect("local workers");
    let config_home = root.join("config-home/parent/../resolved");
    let global_path = root.join("config-home/resolved/zdev/workers.toml");
    fs::create_dir_all(global_path.parent().expect("global worker parent"))
        .expect("global worker directory");
    fs::write(
        &global_path,
        "schema_version = 1\n\n[codex.implementer]\nmodel = \"gpt\"\neffort = \"high\"\nunknown = true\n",
    )
    .expect("malformed global workers");
    let project_path = root.join(".zdev/config.toml");
    let before = [
        fs::read(&project_path).expect("project bytes"),
        fs::read(&local_path).expect("local bytes"),
        fs::read(&global_path).expect("global bytes"),
    ];
    let environment = [("XDG_CONFIG_HOME", config_home.as_path())];

    let failed = run_zdev_with_env(root, &["config", "show", "--format", "json"], &environment);
    assert_eq!(failed.status.code(), Some(2));
    assert!(failed.stdout.is_empty());
    let error: Value = serde_json::from_slice(&failed.stderr).expect("JSON error");
    assert!(
        error["error"]
            .as_str()
            .expect("error")
            .contains(global_path.to_string_lossy().as_ref())
    );
    assert!(
        error["error"]
            .as_str()
            .expect("error")
            .contains("unknown field")
    );

    let local = json_output_with_env(root, &["config", "show", "--local"], &environment);
    assert_eq!(local["scope"], "local");
    assert!(
        local["values"]
            .as_array()
            .expect("local values")
            .iter()
            .any(|entry| entry["key"] == "worker.codex.implementer")
    );

    for arguments in [
        vec!["config", "get", "worker.pi.verifier", "--local"],
        vec!["config", "get", "project.name", "--global"],
        vec!["config", "get", "unknown.key"],
    ] {
        let failed = run_zdev_with_env(root, &arguments, &environment);
        assert_eq!(failed.status.code(), Some(2));
        assert!(failed.stdout.is_empty());
    }
    assert_eq!(
        fs::read(&project_path).expect("project preserved"),
        before[0]
    );
    assert_eq!(fs::read(&local_path).expect("local preserved"), before[1]);
    assert_eq!(fs::read(&global_path).expect("global preserved"), before[2]);
}

#[test]
fn config_set_validates_typed_project_values_and_preserves_trunk_alias() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    json_output(
        root,
        &[
            "area",
            "create",
            "improvements",
            "--title",
            "Improvements",
            "--objective",
            "Improve configuration.",
            "--branch",
            "improvements",
        ],
    );

    let set = json_output(
        root,
        &["config", "set", "project.default-area", "improvements"],
    );
    assert_eq!(set["status"], "set");
    assert_eq!(set["key"], "project.default-area");
    assert_eq!(set["value"], "improvements");
    assert!(set.get("integration_refresh_required").is_none());
    assert!(set.get("integration_refresh_command").is_none());
    assert_eq!(
        set["origin"],
        json!({"path": ".zdev/config.toml", "scope": "local"})
    );
    let config_path = root.join(".zdev/config.toml");
    let valid = fs::read(&config_path).expect("valid project config");

    for arguments in [
        vec!["config", "set", "project.default-area", "missing"],
        vec!["config", "set", "project.guidance", "../outside.md"],
        vec!["config", "set", "project.trunk", "one", "two"],
        vec![
            "config",
            "set",
            "--allow-divergent",
            "project.guidance",
            "auto",
        ],
        vec!["config", "trunk", "main", "--allow-divergent"],
        vec!["config", "set", "project.name", "replacement"],
        vec!["config", "set", "--global", "project.trunk", "main"],
    ] {
        let failed = run_zdev(root, &arguments);
        assert_eq!(failed.status.code(), Some(2));
        assert!(failed.stdout.is_empty());
        assert_eq!(fs::read(&config_path).expect("preserved config"), valid);
    }

    json_output(root, &["config", "set", "project.trunk", "feature"]);
    assert!(
        fs::read_to_string(&config_path)
            .expect("generic trunk")
            .contains("trunk = \"feature\"")
    );
    let alias = json_output(root, &["config", "trunk", "main"]);
    assert_eq!(alias["trunk"], "main");
    assert!(
        fs::read_to_string(&config_path)
            .expect("trunk alias")
            .contains("trunk = \"main\"")
    );

    let unset = json_output(root, &["config", "unset", "project.default-area"]);
    assert_eq!(unset["status"], "unset");
    assert_eq!(unset["effective"]["value"], Value::Null);
    assert_eq!(unset["effective"]["origin"]["scope"], "default");
    assert!(unset.get("integration_refresh_required").is_none());
    assert!(unset.get("integration_refresh_command").is_none());
}

#[test]
fn config_worker_mutations_are_atomic_and_unset_exposes_the_next_layer() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    let config_home = root.join("global-config");
    let global_path = config_home.join("zdev/workers.toml");
    let environment = [("XDG_CONFIG_HOME", config_home.as_path())];

    let global = json_output_with_env(
        root,
        &[
            "config",
            "set",
            "--global",
            "worker.codex.implementer",
            "gpt-5.5",
            "xhigh",
        ],
        &environment,
    );
    assert_eq!(global["status"], "set");
    assert_eq!(global["integration_refresh_required"], true);
    assert_eq!(
        global["integration_refresh_command"],
        "zdev skill install codex --scope user --force"
    );
    assert_eq!(
        global["value"],
        json!({"model": "gpt-5.5", "effort": "xhigh"})
    );
    assert_eq!(global["origin"]["path"], json!(global_path));
    assert_eq!(
        fs::read_to_string(&global_path).expect("global workers"),
        "schema_version = 1\n\n[codex.implementer]\nmodel = \"gpt-5.5\"\neffort = \"xhigh\"\n"
    );

    let local_set = run_zdev_with_env(
        root,
        &[
            "config",
            "set",
            "worker.codex.implementer",
            "gpt-local",
            "high",
        ],
        &environment,
    );
    assert!(local_set.status.success());
    assert_eq!(
        String::from_utf8(local_set.stdout).expect("local set output"),
        "Set worker.codex.implementer in local .zdev/workers.toml.\nRefresh integration: zdev skill install codex --scope project --force\n"
    );
    let unset_local = json_output_with_env(
        root,
        &["config", "unset", "worker.codex.implementer"],
        &environment,
    );
    assert_eq!(unset_local["effective"]["origin"]["scope"], "global");
    assert_eq!(unset_local["integration_refresh_required"], true);
    assert_eq!(
        unset_local["integration_refresh_command"],
        "zdev skill install codex --scope project --force"
    );
    assert_eq!(
        unset_local["effective"]["value"],
        json!({"model": "gpt-5.5", "effort": "xhigh"})
    );
    assert_eq!(
        fs::read_to_string(root.join(".zdev/workers.toml")).expect("empty local workers"),
        "schema_version = 1\n"
    );

    let unset_global = json_output_with_env(
        root,
        &["config", "unset", "--global", "worker.codex.implementer"],
        &environment,
    );
    assert_eq!(unset_global["effective"]["origin"]["scope"], "default");
    assert_eq!(unset_global["integration_refresh_required"], true);
    assert_eq!(
        unset_global["integration_refresh_command"],
        "zdev skill install codex --scope user --force"
    );
    assert_eq!(
        fs::read_to_string(&global_path).expect("empty global workers"),
        "schema_version = 1\n"
    );
    json_output_with_env(
        root,
        &[
            "config",
            "set",
            "--global",
            "worker.codex.verifier",
            "inherit",
        ],
        &environment,
    );
    let human_unset = run_zdev_with_env(
        root,
        &["config", "unset", "--global", "worker.codex.verifier"],
        &environment,
    );
    assert!(human_unset.status.success());
    assert_eq!(
        String::from_utf8(human_unset.stdout).expect("global unset output"),
        format!(
            "Unset worker.codex.verifier from global {}.\nEffective value: {{ model = \"gpt-5.6-sol\", effort = \"low\" }}  [default]\nRefresh integration: zdev skill install codex --scope user --force\n",
            global_path.display()
        )
    );
    let before = fs::read(&global_path).expect("global bytes");
    for arguments in [
        vec![
            "config",
            "set",
            "--global",
            "worker.codex.implementer",
            "model-only",
        ],
        vec![
            "config",
            "set",
            "--global",
            "worker.opencode.verifier",
            "anthropic/custom",
            "high",
        ],
        vec!["config", "unset", "--global", "worker.codex.implementer"],
    ] {
        let failed = run_zdev_with_env(root, &arguments, &environment);
        assert_eq!(failed.status.code(), Some(2));
        assert!(failed.stdout.is_empty());
        assert_eq!(fs::read(&global_path).expect("preserved workers"), before);
    }
}

#[test]
fn config_global_lock_failure_preserves_worker_bytes() {
    let repository = tempfile::tempdir().expect("outside repository");
    let root = repository.path();
    let config_home = root.join("locked-global-config");
    let global_path = config_home.join("zdev/workers.toml");
    let lock_path = config_home.join("zdev/workers.lock");
    let environment = [("XDG_CONFIG_HOME", config_home.as_path())];
    let created = run_zdev_with_env(
        root,
        &[
            "config",
            "set",
            "--global",
            "worker.codex.implementer",
            "gpt-before",
            "high",
        ],
        &environment,
    );
    assert!(created.status.success());
    assert_eq!(
        String::from_utf8(created.stdout).expect("set output"),
        format!(
            "Set worker.codex.implementer in global {}.\nRefresh integration: zdev skill install codex --scope user --force\n",
            global_path.display()
        )
    );
    let before = fs::read(&global_path).expect("worker bytes");
    fs::remove_file(&lock_path).expect("remove unlocked lock file");
    fs::create_dir(&lock_path).expect("blocking lock directory");

    let failed = run_zdev_with_env(
        root,
        &[
            "config",
            "set",
            "--global",
            "worker.codex.verifier",
            "inherit",
            "--format",
            "json",
        ],
        &environment,
    );
    assert_eq!(failed.status.code(), Some(2));
    assert!(failed.stdout.is_empty());
    let error: Value = serde_json::from_slice(&failed.stderr).expect("JSON error");
    assert!(
        error["error"]
            .as_str()
            .expect("error")
            .contains("workers.lock")
    );
    assert_eq!(fs::read(&global_path).expect("preserved workers"), before);
}

#[test]
fn worker_profiles_use_whole_profile_layering_and_native_inheritance() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    let config_home = root.join("worker-config");
    let global_path = config_home.join("zdev/workers.toml");
    fs::create_dir_all(global_path.parent().expect("global worker parent"))
        .expect("global worker directory");
    fs::write(
        &global_path,
        "schema_version = 1\n\n[codex.routine-implementer]\nmodel = \"gpt-routine\"\neffort = \"medium\"\n\n[codex.implementer]\nmodel = \"gpt-global\"\neffort = \"medium\"\n\n[claude.verifier]\ninherit = true\n",
    )
    .expect("global workers");
    fs::write(
        root.join(".zdev/workers.toml"),
        "schema_version = 1\n\n[codex.implementer]\nmodel = \"gpt-local\"\neffort = \"xhigh\"\n\n[codex.advanced-implementer]\ninherit = true\n",
    )
    .expect("local workers");
    let environment = [("XDG_CONFIG_HOME", config_home.as_path())];

    let codex_destination = root.join("layered-codex");
    let installed = json_output_with_env(
        root,
        &[
            "skill",
            "install",
            "codex",
            "--scope",
            "project",
            "--to",
            codex_destination.to_str().expect("Codex destination"),
        ],
        &environment,
    );
    assert_eq!(
        installed["workers"]["implementer"]["origin"]["scope"],
        "local"
    );
    assert_eq!(
        installed["workers"]["verifier"]["origin"]["scope"],
        "default"
    );
    assert_eq!(
        installed["workers"]["routine-implementer"]["origin"]["scope"],
        "global"
    );
    assert_eq!(
        installed["workers"]["advanced-implementer"]["origin"]["scope"],
        "local"
    );
    assert_eq!(
        json_output_with_env(
            root,
            &[
                "skill",
                "check",
                "codex",
                "--scope",
                "project",
                "--to",
                codex_destination.to_str().expect("Codex destination"),
            ],
            &environment,
        )["status"],
        "ok"
    );

    let claude_destination = root.join("inherited-claude");
    let claude = json_output_with_env(
        root,
        &[
            "skill",
            "install",
            "claude",
            "--to",
            claude_destination.to_str().expect("Claude destination"),
        ],
        &environment,
    );
    assert_eq!(
        claude["workers"]["verifier"]["origin"]["path"],
        global_path.to_string_lossy().as_ref()
    );
    assert_eq!(
        claude["workers"]["verifier"]["value"],
        json!({"inherit": true})
    );
}

#[test]
fn worker_profile_files_reject_strict_schema_errors_with_the_source_path() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    let config_home = root.join("empty-worker-config");
    let environment = [("XDG_CONFIG_HOME", config_home.as_path())];
    let path = root.join(".zdev/workers.toml");

    for (source, expected) in [
        ("schema_version = 2\n", "schema_version 2"),
        (
            "schema_version = 1\n[other.implementer]\ninherit = true\n",
            "unknown field `other`",
        ),
        (
            "schema_version = 1\n[codex.reviewer]\ninherit = true\n",
            "unknown field `reviewer`",
        ),
        (
            "schema_version = 1\n[codex.implementer]\nmodel = \"gpt\"\neffort = \"high\"\ntemperature = 1\n",
            "unknown field `temperature`",
        ),
        (
            "schema_version = 1\n[codex.implementer]\nmodel = \"gpt\"\neffort = \"ultra\"\n",
            "unknown variant `ultra`",
        ),
        (
            "schema_version = 1\n[codex.implementer]\nmodel = \" \"\neffort = \"high\"\n",
            "model must not be empty",
        ),
        (
            "schema_version = 1\n[codex.implementer]\ninherit = true\nmodel = \"gpt\"\n",
            "inherit must be true and cannot be combined",
        ),
    ] {
        fs::write(&path, source).expect("invalid worker file");
        let output = run_zdev_with_env(
            root,
            &[
                "skill",
                "install",
                "codex",
                "--scope",
                "project",
                "--to",
                root.join("strict-worker-destination")
                    .to_str()
                    .expect("destination"),
            ],
            &environment,
        );
        assert!(!output.status.success());
        let error = String::from_utf8_lossy(&output.stderr);
        assert!(
            error.contains(".zdev/workers.toml"),
            "missing path: {error}"
        );
        assert!(error.contains(expected), "missing {expected:?}: {error}");
    }
}

#[test]
fn unsupported_worker_adapter_config_preserves_an_installed_destination() {
    let repository = repository();
    let root = repository.path();
    let config_home = root.join("worker-config");
    let environment = [("XDG_CONFIG_HOME", config_home.as_path())];
    let destination = root.join("preserved-opencode");
    json_output_with_env(
        root,
        &[
            "skill",
            "install",
            "opencode",
            "--to",
            destination.to_str().expect("destination"),
        ],
        &environment,
    );
    let managed = destination.join("agents/zdev-implementer.md");
    let before = fs::read(&managed).expect("installed implementer");
    let global_path = config_home.join("zdev/workers.toml");
    fs::create_dir_all(global_path.parent().expect("worker config parent"))
        .expect("worker config directory");
    fs::write(
        &global_path,
        "schema_version = 1\n\n[opencode.implementer]\nmodel = \"anthropic/custom\"\neffort = \"high\"\n",
    )
    .expect("unsupported worker config");

    for command in ["install", "check"] {
        let mut arguments = vec![
            "skill",
            command,
            "opencode",
            "--to",
            destination.to_str().expect("destination"),
        ];
        if command == "install" {
            arguments.push("--force");
        }
        let output = run_zdev_with_env(root, &arguments, &environment);
        assert!(!output.status.success());
        let error = String::from_utf8_lossy(&output.stderr);
        assert!(error.contains(global_path.to_string_lossy().as_ref()));
        assert!(error.contains("[opencode.implementer]"));
        assert!(error.contains("\"high\""));
        assert!(error.contains("\"anthropic/custom\""));
        assert_eq!(fs::read(&managed).expect("preserved implementer"), before);
    }
}

#[test]
fn codex_skill_check_and_force_install_manage_ui_metadata() {
    let repository = repository();
    let root = repository.path();
    let destination = root.join("codex-bundle");
    let destination_text = destination.to_str().expect("Codex destination");

    json_output(
        root,
        &["skill", "install", "codex", "--to", destination_text],
    );

    let metadata = destination.join("zdev/agents/openai.yaml");
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
    assert_eq!(
        fs::read_to_string(destination.join("opencode.json")).expect("preserved config"),
        "{\"theme\":\"system\"}\n"
    );
    for path in [
        "skills/zdev-opencode/SKILL.md",
        "agents/zdev-advanced-implementer.md",
        "agents/zdev-implementer.md",
        "agents/zdev-planner.md",
        "agents/zdev-routine-implementer.md",
        "agents/zdev-verifier.md",
        "commands/zdev-implement.md",
        "commands/zdev-loop.md",
        "commands/zdev-goal.md",
        "commands/zdev-verify.md",
        "commands/zdev-audit.md",
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
    assert_eq!(
        file_inventory(&destination),
        [
            "extensions/zdev-subagent.ts",
            "prompts/zdev-audit.md",
            "prompts/zdev-goal.md",
            "prompts/zdev-implement.md",
            "prompts/zdev-loop.md",
            "prompts/zdev-verify.md",
            "settings.json",
            "skills/zdev-pi/SKILL.md",
            "skills/zdev-pi/references/area-loop.md",
            "skills/zdev-pi/references/audit.md",
            "skills/zdev-pi/references/discuss.md",
            "skills/zdev-pi/references/implement.md",
            "skills/zdev-pi/references/improve.md",
            "skills/zdev-pi/references/investigate.md",
            "skills/zdev-pi/references/recovery.md",
            "skills/zdev-pi/references/setup.md",
            "skills/zdev-pi/references/shape-work.md",
            "skills/zdev-pi/references/task-format.md",
            "skills/zdev-pi/references/task-workflows.md",
            "skills/zdev-pi/references/to-tasks.md",
            "skills/zdev-pi/references/verify-workflow.md",
            "skills/zdev-pi/references/verify.md",
        ]
    );
    assert_eq!(
        fs::read_to_string(destination.join("settings.json")).expect("preserved config"),
        "{\"theme\":\"dark\"}\n"
    );
    for path in [
        "skills/zdev-pi/SKILL.md",
        "prompts/zdev-implement.md",
        "prompts/zdev-goal.md",
        "prompts/zdev-loop.md",
        "prompts/zdev-verify.md",
        "prompts/zdev-audit.md",
        "extensions/zdev-subagent.ts",
    ] {
        assert!(destination.join(path).is_file(), "missing {path}");
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
    assert_eq!(
        file_inventory(&destination),
        [
            "agents/zdev-advanced-implementer.md",
            "agents/zdev-implementer.md",
            "agents/zdev-planner.md",
            "agents/zdev-routine-implementer.md",
            "agents/zdev-verifier.md",
            "prompts/zdev-audit.md",
            "prompts/zdev-goal.md",
            "prompts/zdev-implement.md",
            "prompts/zdev-loop.md",
            "prompts/zdev-verify.md",
            "settings.json",
            "skills/zdev/SKILL.md",
            "skills/zdev/references/area-loop.md",
            "skills/zdev/references/audit.md",
            "skills/zdev/references/discuss.md",
            "skills/zdev/references/implement.md",
            "skills/zdev/references/improve.md",
            "skills/zdev/references/investigate.md",
            "skills/zdev/references/recovery.md",
            "skills/zdev/references/setup.md",
            "skills/zdev/references/shape-work.md",
            "skills/zdev/references/task-format.md",
            "skills/zdev/references/task-workflows.md",
            "skills/zdev/references/to-tasks.md",
            "skills/zdev/references/verify-workflow.md",
            "skills/zdev/references/verify.md",
        ]
    );
    assert_eq!(
        fs::read_to_string(destination.join("settings.json")).expect("preserved config"),
        "{\"theme\":\"dark\"}\n"
    );

    assert!(!destination.join("extensions/zdev-subagent.ts").exists());

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
    assert_eq!(codex["path"], json!(codex_home.join("skills")));
    assert_eq!(codex["scope"], "user");
    assert_task_workflows_locator(&codex_home.join("skills"), "zdev", &["zdev/SKILL.md"]);

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
    assert_task_workflows_locator(
        &xdg_home.join("opencode"),
        "skills/zdev-opencode",
        &[
            "skills/zdev-opencode/SKILL.md",
            "commands/zdev-implement.md",
            "commands/zdev-verify.md",
        ],
    );

    let pi = json_output_with_env(
        root,
        &["skill", "install", "pi"],
        &[("PI_CODING_AGENT_DIR", pi_home.as_path())],
    );
    assert_eq!(pi["path"], json!(pi_home));
    assert_eq!(pi["scope"], "user");
    assert_task_workflows_locator(
        &pi_home,
        "skills/zdev-pi",
        &[
            "skills/zdev-pi/SKILL.md",
            "prompts/zdev-implement.md",
            "prompts/zdev-verify.md",
        ],
    );

    let omp = json_output_with_env(
        root,
        &["skill", "install", "omp"],
        &[("PI_CODING_AGENT_DIR", omp_home.as_path())],
    );
    assert_eq!(omp["path"], json!(omp_home));
    assert_eq!(omp["scope"], "user");
    assert_task_workflows_locator(
        &omp_home,
        "skills/zdev",
        &[
            "skills/zdev/SKILL.md",
            "prompts/zdev-implement.md",
            "prompts/zdev-verify.md",
            "prompts/zdev-loop.md",
        ],
    );

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
    assert_task_workflows_locator(
        &fallback_home.join(".omp/agent"),
        "skills/zdev",
        &[
            "skills/zdev/SKILL.md",
            "prompts/zdev-implement.md",
            "prompts/zdev-verify.md",
            "prompts/zdev-loop.md",
        ],
    );

    let codex_project = json_output(root, &["skill", "install", "codex", "--scope", "project"]);
    let canonical_root = fs::canonicalize(root).expect("canonical repository root");
    assert_eq!(
        codex_project["path"],
        json!(canonical_root.join(".codex/skills"))
    );
    assert_eq!(codex_project["scope"], "project");
    assert_task_workflows_locator(
        &canonical_root.join(".codex/skills"),
        "zdev",
        &["zdev/SKILL.md"],
    );

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
    assert_task_workflows_locator(
        &canonical_root.join(".opencode"),
        "skills/zdev-opencode",
        &[
            "skills/zdev-opencode/SKILL.md",
            "commands/zdev-implement.md",
            "commands/zdev-verify.md",
        ],
    );

    let pi_project = json_output(root, &["skill", "install", "pi", "--scope", "project"]);
    assert_eq!(pi_project["path"], json!(canonical_root.join(".pi")));
    assert_eq!(pi_project["scope"], "project");
    assert_task_workflows_locator(
        &canonical_root.join(".pi"),
        "skills/zdev-pi",
        &[
            "skills/zdev-pi/SKILL.md",
            "prompts/zdev-implement.md",
            "prompts/zdev-verify.md",
        ],
    );

    let omp_project = json_output(root, &["skill", "install", "omp", "--scope", "project"]);
    assert_eq!(omp_project["path"], json!(canonical_root.join(".omp")));
    assert_eq!(omp_project["scope"], "project");
    assert_task_workflows_locator(
        &canonical_root.join(".omp"),
        "skills/zdev",
        &[
            "skills/zdev/SKILL.md",
            "prompts/zdev-implement.md",
            "prompts/zdev-verify.md",
            "prompts/zdev-loop.md",
        ],
    );
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
fn project_integration_check_requires_an_initialized_record() {
    let repository = repository();
    let root = repository.path();

    let output = run_zdev(
        root,
        &[
            "skill",
            "check",
            "codex",
            "--scope",
            "project",
            "--guidance",
            "auto",
            "--format",
            "json",
        ],
    );

    assert_eq!(output.status.code(), Some(2));
    let error: Value = serde_json::from_slice(&output.stderr).expect("JSON error");
    assert_eq!(error["command"], "skill");
    assert_eq!(error["ok"], false);
    assert!(
        error["error"]
            .as_str()
            .expect("error message")
            .contains("zdev init --record <personal|project|pull-request>")
    );
    assert!(
        error["error"]
            .as_str()
            .expect("error message")
            .contains(".zdev/config.toml")
    );
    assert!(!root.join(".zdev").exists());
    assert!(!root.join(".codex").exists());

    json_output(root, &["init", "--record", "project"]);
    let missing = json_output_with_exit_code(
        root,
        &[
            "skill",
            "check",
            "codex",
            "--scope",
            "project",
            "--guidance",
            "auto",
        ],
        1,
    );
    assert_eq!(missing["status"], "missing");
    assert_eq!(missing["bundle"]["status"], "missing");
}

#[test]
fn project_skill_install_always_inlines_guidance_while_user_install_does_not() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    let guidance = "# Repository instructions\n\nRun `just ci-project-only`. Keep `{{trusted_fragment}}` and \"quoted text\" literal.\n";
    fs::write(root.join("AGENTS.md"), guidance).expect("repository guidance");

    for harness in ["codex", "claude", "opencode", "pi", "omp"] {
        let project = json_output(root, &["skill", "install", harness, "--scope", "project"]);
        assert_eq!(project["guidance"]["source"], "AGENTS.md");
        assert_eq!(project["guidance"]["status"], "ok");

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
        assert_eq!(
            json_output(
                root,
                &[
                    "skill",
                    "check",
                    harness,
                    "--to",
                    user_destination.to_str().expect("user destination"),
                ],
            )["status"],
            "ok"
        );
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
    let claude = json_output(root, &["skill", "install", "claude", "--scope", "project"]);
    assert_eq!(claude["guidance"]["status"], "ok");
    assert_eq!(
        fs::read_to_string(root.join(".zdev/guidance.md")).expect("preserved guidance"),
        authored
    );
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
    fs::write(custom_bundle.join("zdev/SKILL.md"), "replace\n").expect("change bundle");
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
fn slice_briefs_create_list_show_and_leave_existing_areas_valid() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "feature", "main");

    let empty = json_output(root, &["slice", "list", "feature"]);
    assert_eq!(empty["slices"], json!([]));
    assert_eq!(json_output(root, &["check", "feature"])["status"], "ok");

    json_output(
        root,
        &[
            "slice",
            "create",
            "feature",
            "second",
            "--title",
            "Second slice",
            "--objective",
            "Deliver the second useful increment.",
            "--boundary",
            "Keep it small.",
        ],
    );
    let created = json_output(
        root,
        &[
            "slice",
            "create",
            "feature",
            "first",
            "--title",
            "First slice",
            "--objective",
            "Deliver the first useful increment.",
            "--boundary",
            "Preserve compatibility.",
            "--boundary",
            "Use existing tests.",
        ],
    );
    assert_eq!(created["status"], "created");
    assert_eq!(created["path"], ".zdev/feature/slices/first.md");

    let listed = json_output(root, &["slice", "list", "feature"]);
    assert_eq!(listed["slices"][0]["key"], "first");
    assert_eq!(listed["slices"][0]["title"], "First slice");
    assert_eq!(listed["slices"][1]["key"], "second");
    let listed_text = run_zdev(root, &["slice", "list", "feature"]);
    assert!(listed_text.status.success());
    assert_eq!(
        String::from_utf8_lossy(&listed_text.stdout),
        "first  First slice\nsecond  Second slice\n"
    );

    let shown = json_output(root, &["slice", "show", "feature", "first"]);
    assert_eq!(shown["title"], "First slice");
    assert_eq!(shown["objective"], "Deliver the first useful increment.");
    assert_eq!(
        shown["boundaries"],
        "- Preserve compatibility.\n- Use existing tests."
    );
    let path = root.join(".zdev/feature/slices/first.md");
    let text = run_zdev(root, &["slice", "show", "feature", "first"]);
    assert!(text.status.success());
    assert_eq!(
        String::from_utf8_lossy(&text.stdout).trim_end(),
        fs::read_to_string(path).expect("slice brief").trim_end()
    );
    assert_eq!(json_output(root, &["check", "feature"])["status"], "ok");
}

#[test]
fn slice_create_rejects_invalid_or_incomplete_input_without_replacing_a_slice() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "feature", "main");

    for arguments in [
        vec![
            "slice",
            "create",
            "feature",
            "Bad-Key",
            "--title",
            "Bad",
            "--objective",
            "Bad key.",
            "--boundary",
            "No publication.",
        ],
        vec![
            "slice",
            "create",
            "feature",
            "empty-boundary",
            "--title",
            "Bad",
            "--objective",
            "Empty boundary.",
            "--boundary",
            "",
        ],
    ] {
        assert!(!run_zdev(root, &arguments).status.success());
    }
    let missing_boundary = run_zdev(
        root,
        &[
            "slice",
            "create",
            "feature",
            "missing-boundary",
            "--title",
            "Bad",
            "--objective",
            "Missing boundary.",
        ],
    );
    assert!(!missing_boundary.status.success());
    assert!(!root.join(".zdev/feature/slices").exists());

    json_output(
        root,
        &[
            "slice",
            "create",
            "feature",
            "one",
            "--title",
            "One",
            "--objective",
            "Create one slice.",
            "--boundary",
            "Do not replace it.",
        ],
    );
    let path = root.join(".zdev/feature/slices/one.md");
    let original = fs::read(&path).expect("original slice");
    let duplicate = run_zdev(
        root,
        &[
            "slice",
            "create",
            "feature",
            "one",
            "--title",
            "Replacement",
            "--objective",
            "Replace one slice.",
            "--boundary",
            "This must fail.",
        ],
    );
    assert!(!duplicate.status.success());
    assert_eq!(fs::read(path).expect("preserved slice"), original);
}

#[test]
fn check_validates_slice_frontmatter_identity_and_required_sections() {
    let repository = repository();
    let root = repository.path();
    git(root, &["branch", "-m", "main"]);
    commit_file(root, "seed.txt", "seed\n", "seed");
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "feature", "main");
    json_output(
        root,
        &[
            "slice",
            "create",
            "feature",
            "one",
            "--title",
            "One",
            "--objective",
            "Create one slice.",
            "--boundary",
            "Keep validation focused.",
        ],
    );
    let slices = root.join(".zdev/feature/slices");
    let path = slices.join("one.md");
    let valid = fs::read_to_string(&path).expect("valid slice");

    fs::write(
        &path,
        valid.replace("title = \"One\"", "title = \"One\"\nstatus = \"open\""),
    )
    .expect("unknown field slice");
    let unknown = run_zdev(root, &["check", "feature"]);
    assert!(!unknown.status.success());
    assert!(String::from_utf8_lossy(&unknown.stderr).contains("unknown field"));

    fs::write(&path, &valid).expect("restore valid slice");
    fs::rename(&path, slices.join("moved.md")).expect("move slice");
    let moved = run_zdev(root, &["check", "feature"]);
    assert!(!moved.status.success());
    assert!(String::from_utf8_lossy(&moved.stderr).contains("does not match its filename"));

    fs::rename(slices.join("moved.md"), &path).expect("restore slice name");
    fs::write(
        &path,
        valid.replace("## Objective\n\nCreate one slice.", "## Objective\n\n"),
    )
    .expect("empty objective slice");
    let empty = run_zdev(root, &["check", "feature"]);
    assert!(!empty.status.success());
    assert!(String::from_utf8_lossy(&empty.stderr).contains("empty ## Objective"));

    fs::write(
        &path,
        valid.replace("## Boundaries\n\n- Keep validation focused.\n", ""),
    )
    .expect("missing boundaries slice");
    let missing = run_zdev(root, &["check", "feature"]);
    assert!(!missing.status.success());
    assert!(String::from_utf8_lossy(&missing.stderr).contains("lacks ## Boundaries"));
}

#[test]
fn project_check_says_current_bundle_is_not_ready_when_guidance_is_missing() {
    let repository = repository();
    let root = repository.path();
    let destination = root.join(".codex/skills");
    json_output(root, &["init", "--record", "project"]);
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

#[test]
fn task_slice_membership_flows_through_selection_and_derived_progress() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "feature", "main");
    create_slice(root, "feature", "alpha", "Alpha slice");
    create_slice(root, "feature", "empty", "Empty slice");

    let bundle = serde_json::to_vec(&json!({
        "schema_version": 1,
        "area": "feature",
        "tasks": [
            {
                "key": "alpha-first",
                "title": "Build alpha foundation",
                "slice": "alpha",
                "blocked_by": [],
                "outcome": "The slice has a ready task.",
                "done_when": ["The foundation is complete."],
                "validation": ["Exercise slice task behavior."]
            },
            {
                "key": "alpha-second",
                "title": "Build alpha follow-up",
                "slice": "alpha",
                "blocked_by": ["alpha-first"],
                "outcome": "The slice has a blocked task.",
                "done_when": ["The follow-up is complete."],
                "validation": ["Exercise derived slice progress."]
            },
            {
                "key": "ordinary",
                "title": "Keep one ordinary task",
                "blocked_by": [],
                "outcome": "Unsliced tasks remain valid.",
                "done_when": ["The ordinary task remains selectable."],
                "validation": ["Exercise area totals."]
            }
        ]
    }))
    .expect("task bundle");
    let reviewed = json_output_with_stdin(
        root,
        &["tasks", "review", "feature", "--from", "-"],
        &bundle,
    );
    let shown = json_output(root, &["tasks", "review", "feature", "--show"]);
    let markdown = shown["markdown"].as_str().expect("approval markdown");
    assert!(markdown.contains("### Slice\nalpha"));
    assert!(markdown.contains("### Slice\nNone"));
    let review = reviewed["review"].as_str().expect("review identity");
    json_output(root, &["tasks", "import", "feature", "--reviewed", review]);

    let first_path = root.join(".zdev/feature/tasks/001-build-alpha-foundation.md");
    assert!(
        fs::read_to_string(&first_path)
            .expect("first task")
            .contains("slice = \"alpha\"")
    );
    let ordinary_path = root.join(".zdev/feature/tasks/003-keep-one-ordinary-task.md");
    assert!(
        !fs::read_to_string(ordinary_path)
            .expect("ordinary task")
            .contains("slice =")
    );

    let listed = json_output(root, &["tasks", "list", "feature"]);
    assert_eq!(listed["tasks"][0]["slice"], "alpha");
    assert_eq!(listed["tasks"][2]["slice"], Value::Null);
    let listed_text = run_zdev(root, &["tasks", "list", "feature"]);
    assert!(String::from_utf8_lossy(&listed_text.stdout).contains("slice:alpha"));

    let shown = json_output(root, &["task", "show", "feature", "feature-001"]);
    assert_eq!(shown["slice"], "alpha");
    assert_eq!(shown["slice_brief"], ".zdev/feature/slices/alpha.md");
    let shown_text = run_zdev(root, &["task", "show", "feature", "feature-001"]);
    assert!(
        String::from_utf8_lossy(&shown_text.stdout)
            .contains("Slice brief: .zdev/feature/slices/alpha.md")
    );

    let next = json_output(root, &["next", "feature"]);
    assert_eq!(next["task"]["id"], "feature-001");
    assert_eq!(next["task"]["slice"], "alpha");
    assert_eq!(next["task"]["slice_brief"], ".zdev/feature/slices/alpha.md");
    let next_text = run_zdev(root, &["next", "feature"]);
    assert!(
        String::from_utf8_lossy(&next_text.stdout)
            .contains("Slice brief: .zdev/feature/slices/alpha.md")
    );

    let index = fs::read_to_string(root.join(".zdev/feature/TASKS.md")).expect("task index");
    assert!(index.contains("| ID | Task | Slice | State | Blocked by |"));
    assert!(index.contains("| alpha | ready |"));
    assert!(index.contains("| — | ready |"));

    let status = json_output(root, &["status", "feature"]);
    assert_eq!(
        status["counts"],
        json!({"total": 3, "ready": 2, "blocked": 1, "done": 0})
    );
    assert_eq!(
        status["slices"][0],
        json!({
            "key": "alpha",
            "title": "Alpha slice",
            "path": ".zdev/feature/slices/alpha.md",
            "ready": 1,
            "blocked": 1,
            "done": 0
        })
    );
    assert_eq!(status["slices"][1]["key"], "empty");
    assert_eq!(status["slices"][1]["ready"], 0);
    assert_eq!(status["slices"][1]["blocked"], 0);
    assert_eq!(status["slices"][1]["done"], 0);
    let status_text = run_zdev(root, &["status", "feature"]);
    assert!(
        String::from_utf8_lossy(&status_text.stdout)
            .contains("Slice empty: 0 ready, 0 blocked, 0 done")
    );

    json_output(
        root,
        &[
            "task",
            "done",
            "feature",
            "feature-001",
            "--summary",
            "Completed the first slice task.",
            "--validation",
            "Focused slice checks passed.",
        ],
    );
    let progressed = json_output(root, &["status", "feature"]);
    assert_eq!(progressed["slices"][0]["ready"], 1);
    assert_eq!(progressed["slices"][0]["blocked"], 0);
    assert_eq!(progressed["slices"][0]["done"], 1);
}

#[test]
fn legacy_unsliced_task_index_remains_valid() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "feature", "main");
    import_one_task(root, "feature");

    let index = fs::read_to_string(root.join(".zdev/feature/TASKS.md")).expect("task index");
    assert!(index.contains("| ID | Task | State | Blocked by |"));
    assert!(!index.contains("| ID | Task | Slice | State | Blocked by |"));
    assert_eq!(json_output(root, &["check", "feature"])["status"], "ok");
}

#[test]
fn missing_task_slice_references_fail_before_publication_and_during_check() {
    let repository = repository();
    let root = repository.path();
    json_output(root, &["init", "--record", "project"]);
    create_area(root, "feature", "main");
    create_slice(root, "feature", "known", "Known slice");
    let index_path = root.join(".zdev/feature/TASKS.md");
    let original_index = fs::read(&index_path).expect("original index");
    let invalid = serde_json::to_vec(&json!({
        "schema_version": 1,
        "area": "feature",
        "tasks": [{
            "key": "missing",
            "title": "Reference a missing slice",
            "slice": "missing",
            "blocked_by": [],
            "outcome": "Invalid membership is rejected.",
            "done_when": ["No task is published."],
            "validation": ["Exercise reference validation."]
        }]
    }))
    .expect("invalid bundle");
    for command in [
        ["tasks", "review", "feature", "--from", "-"],
        ["tasks", "import", "feature", "--from", "-"],
    ] {
        let output = json_output_with_stdin_status(root, &command, &invalid);
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("unknown slice missing"));
        assert_eq!(
            fs::read(&index_path).expect("unchanged index"),
            original_index
        );
        assert_eq!(
            fs::read_dir(root.join(".zdev/feature/tasks"))
                .expect("tasks directory")
                .count(),
            0
        );
    }

    let valid = serde_json::to_vec(&json!({
        "schema_version": 1,
        "area": "feature",
        "tasks": [{
            "key": "known",
            "title": "Reference the known slice",
            "slice": "known",
            "blocked_by": [],
            "outcome": "Manual references are checked.",
            "done_when": ["The task is valid."],
            "validation": ["Run zdev check."]
        }]
    }))
    .expect("valid bundle");
    json_output_with_stdin(root, &["tasks", "import", "feature", "--from", "-"], &valid);
    let task_path = root.join(".zdev/feature/tasks/001-reference-the-known-slice.md");
    let task = fs::read_to_string(&task_path)
        .expect("task")
        .replace("slice = \"known\"", "slice = \"missing\"");
    fs::write(task_path, task).expect("write invalid manual reference");
    let checked = run_zdev(root, &["check", "feature"]);
    assert!(!checked.status.success());
    assert!(String::from_utf8_lossy(&checked.stderr).contains("unknown slice missing"));
}

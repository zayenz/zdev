const README: &str = include_str!("../README.md");
const USER_GUIDE: &str = include_str!("../docs/user-guide.md");
const WORKFLOW: &str = include_str!("../docs/workflow.md");
const ADAPTED_METHODS: &str = include_str!("../docs/adapted-methods.md");
const SHARED_CONTRACT: &str = include_str!("../templates/zdev/shared-contract.md");
const TASK_WORKFLOWS: &str = include_str!("../templates/zdev/task-workflows.md");
const SHAPE_WORK: &str = include_str!("../templates/zdev/references/shape-work.md");
const TO_TASKS: &str = include_str!("../templates/zdev/references/to-tasks.md");

fn normalized(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn user_docs_describe_observable_actions_without_chat_roles() {
    let docs = [README, USER_GUIDE, WORKFLOW, ADAPTED_METHODS].join("\n");
    let lowercase = docs.to_lowercase();

    for internal_term in [
        "main conversation",
        "primary conversation",
        "side chat",
        "side-chat",
        "side conversation",
        "return control",
        "authority",
        "harness boundary",
        "full branch coverage",
        "one decision branch at a time",
    ] {
        assert!(
            !lowercase.contains(internal_term),
            "user documentation exposes internal workflow term: {internal_term}"
        );
    }

    let readme = normalized(README);
    assert!(readme.contains("zdev tasks review scheduling --from - --format json"));
    assert!(readme.contains("zdev tasks review scheduling --show"));
    assert!(readme.contains("--reviewed <review-id> --commit --format json"));
    assert!(readme.contains("user never handles it or the internal fingerprint"));
    assert!(readme.contains("does not interrupt the selected task"));
    assert!(readme.contains("zdev work-context scheduling --store --format json"));
    assert!(readme.contains("zdev work-context scheduling --show <snapshot-id> --format json"));
    assert!(readme.contains("zdev work-context scheduling --compare <snapshot-id> --format json"));
    assert!(readme.contains("immutable handoff evidence, not reusable current state"));

    let guide = normalized(USER_GUIDE);
    for public_contract in [
        "zdev tasks review scheduling --from - --format json",
        "zdev tasks review scheduling --show",
        "zdev tasks import scheduling --reviewed <review-id>",
        "you never read, copy, compare, or diagnose it or the internal fingerprint",
        "zdev tasks import scheduling --from -",
        "zdev tasks import scheduling --from - --commit --format json",
        "including for the initial task split",
        "task IDs, paths, the commit hash, and the stable change ID",
        "zdev status scheduling --format json",
        "zdev next scheduling --format json",
        "zdev work-context scheduling --store --format json",
        "zdev work-context scheduling --show <snapshot-id> --format json",
        "zdev work-context scheduling --compare <snapshot-id> --format json",
        "stored file is an immutable handoff, not permission to act on later",
        "zdev task done scheduling scheduling-001",
        "zdev area rebase scheduling --continue",
        "zdev area rebase scheduling --abort",
        "zdev change inspect HEAD",
        "zdev change lookup Z0123456789abcdef...",
        "branch_status.task_work.safe",
        "Codex",
        "Claude Code",
        "OpenCode",
        "Pi",
        "Oh My Pi",
        "OMP 17.2.15",
        "PI_CODING_AGENT_DIR",
    ] {
        assert!(
            guide.contains(public_contract),
            "user guide lost public contract: {public_contract}"
        );
    }
    assert!(guide.contains("New task-only commits do not interrupt the selected task"));

    let workflow = normalized(WORKFLOW);
    for public_contract in [
        "branch_matches",
        "anchor_valid",
        "zdev area rebase <area> --continue",
        "zdev area rebase <area> --abort",
        "Zdev-Change-Id",
        "New task-only commits are expected and do not interrupt the selected task",
        "fresh, read-only context",
        "separate Spec and Standards passes",
    ] {
        assert!(
            workflow.contains(public_contract),
            "workflow guide lost public contract: {public_contract}"
        );
    }

    let adapted_methods = normalized(ADAPTED_METHODS);
    for concept in [
        "challenge independent branches breadth first",
        "no unresolved choice",
        "task splitting",
        "validation",
    ] {
        assert!(adapted_methods.contains(concept));
    }

    for document in [README, WORKFLOW, USER_GUIDE] {
        let document = normalized(document);
        assert!(document.contains("same reviewer"));
        assert!(document.contains("complete revised document"));
        assert!(document.contains("fresh full challenge"));
        assert!(document.contains("approval"));
    }
}

#[test]
fn writing_guidance_preserves_semantics_and_reaches_prose_workers() {
    let readme = normalized(README);
    assert!(readme.contains("Cursor pstack's `unslop` skill"));
    assert!(readme.contains("`writing-for-agents`"));

    let shared = normalized(SHARED_CONTRACT);
    assert!(shared.contains("## Write human-facing prose plainly"));
    assert!(shared.contains("active voice"));
    assert!(shared.contains("If a sentence could describe any project unchanged, cut it"));
    assert!(shared.contains("JSON, TOML, YAML, frontmatter"));
    assert!(shared.contains("## Write durable records for agents"));
    assert!(shared.contains("Each completion condition must be checkable"));

    let workflows = normalized(TASK_WORKFLOWS);
    assert!(
        workflows.contains("shared prose guidance only when the task authors human-facing text")
    );
    assert!(workflows.contains("## Write transient instructions for the worker's path"));
    assert!(workflows.contains("Transient payloads include only"));

    let briefs = normalized(SHAPE_WORK);
    assert!(briefs.contains("Apply the shared durable-record guidance for agents"));
    assert!(briefs.contains("cannot recover cheaply from the repository"));

    let tasks = normalized(TO_TASKS);
    assert!(tasks.contains("Apply the shared durable-record guidance for agents"));
    assert!(tasks.contains("`Boundaries` only for real constraints"));
    assert!(tasks.contains("Use `Done when` for observable completion"));
    assert!(tasks.contains("`docs/report-format.md`"));
}

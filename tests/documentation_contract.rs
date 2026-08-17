const README: &str = include_str!("../README.md");
const USER_GUIDE: &str = include_str!("../docs/user-guide.md");
const WORKFLOW: &str = include_str!("../docs/workflow.md");
const ADAPTED_METHODS: &str = include_str!("../docs/adapted-methods.md");

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
    assert!(readme.contains("--approval <approval-id> --commit --format json"));
    assert!(readme.contains("does not interrupt the selected task"));

    let guide = normalized(USER_GUIDE);
    for public_contract in [
        "zdev tasks review scheduling --from - --format json",
        "zdev tasks import scheduling --from - --approval <approval-id>",
        "--approval <approval-id> --commit --format json",
        "Use ordinary import for the initial task split",
        "task IDs, paths, the commit hash, and the stable change ID",
        "zdev status scheduling --format json",
        "zdev next scheduling --format json",
        "zdev task done scheduling scheduling-001",
        "zdev area rebase scheduling --continue",
        "zdev area rebase scheduling --abort",
        "zdev change inspect HEAD",
        "zdev change lookup Z0123456789abcdef...",
        "matching, fresh, anchor-valid, and finalized",
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
    assert!(adapted_methods.contains("challenge independent branches breadth first"));
    assert!(adapted_methods.contains(
        "stops when no unresolved choice could materially change behavior, scope, task splitting, or validation"
    ));
}

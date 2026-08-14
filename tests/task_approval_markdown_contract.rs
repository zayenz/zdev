use std::fs;
use std::path::Path;

const CREATE_TASK_REFERENCES: [&str; 4] = [
    "skills/zdev/references/to-tasks.md",
    ".claude/skills/zdev/skills/zdev/references/to-tasks.md",
    ".opencode/skills/zdev-opencode/references/to-tasks.md",
    ".pi/skills/zdev-pi/references/to-tasks.md",
];

const APPROVAL_RENDERING_EXAMPLE: &str = r#"```markdown
# Task Bundle

## Area
scheduling

## Schema version
1

## Task 1

### Key
model

### Title
Add the scheduling model

### Outcome
The model represents the required scheduling decisions.

### Context
Add the model beside the existing scheduler types.

### Boundaries
1. Do not change unrelated APIs.
2. Use the vocabulary settled in the brief.

### Blocked by
None

### Done when / proof
1. The model represents every decision named in the brief.
2. Focused tests cover the model.

### Validation / Testing
1. Run the focused model tests.
2. Apply the area's focused testing level; no broader tests are required.
```"#;

#[test]
fn every_harness_uses_markdown_as_the_exact_task_import_approval_source() {
    let repository = Path::new(".");
    assert!(repository.join("Cargo.toml").is_file());

    let references = CREATE_TASK_REFERENCES.map(|path| {
        (
            path,
            fs::read_to_string(repository.join(path))
                .unwrap_or_else(|error| panic!("failed to read {path}: {error}")),
        )
    });

    for (path, reference) in &references {
        let reference_text = reference.split_whitespace().collect::<Vec<_>>().join(" ");
        assert!(
            reference.contains(APPROVAL_RENDERING_EXAMPLE),
            "{path} does not contain the canonical fenced approval rendering"
        );

        let example = APPROVAL_RENDERING_EXAMPLE;
        for heading in [
            "## Area",
            "## Schema version",
            "## Task 1",
            "### Key",
            "### Title",
            "### Outcome",
            "### Context",
            "### Boundaries",
            "### Blocked by",
            "### Done when / proof",
            "### Validation / Testing",
        ] {
            assert_eq!(
                example.matches(heading).count(),
                1,
                "canonical example must contain {heading} exactly once"
            );
        }
        assert!(
            example.contains("### Outcome\nThe model represents"),
            "canonical example must show a scalar on the line after its heading"
        );
        assert!(
            example.contains("### Boundaries\n1. Do not change unrelated APIs.\n2. Use"),
            "canonical example must show an ordered list"
        );
        assert!(
            example.contains("### Blocked by\nNone"),
            "canonical example must show an empty value as None"
        );

        for expected in [
            "one human-readable Markdown document inside a fenced block",
            "Use a fence longer than any run of backticks in the values",
            "Put each scalar value on the line after its heading",
            "Render each list value as a numbered item and preserve item order",
            "Render an empty optional scalar or empty list as the single value `None`",
            "Repeat the complete `## Task N` section for every task, preserving task order",
            "fenced Markdown document is the exact approval source",
            "carry every value losslessly",
            "serialize those retained values after approval",
            "do not reconstruct them by parsing or paraphrasing the display",
            "Do not present the exact Task Bundle JSON by default",
            "only when the user explicitly asks to inspect it",
            "explicit approval immediately before every import",
            "bundle changes after approval—meaning any task content, task order, or dependency changes",
            "re-render the complete Markdown list and obtain fresh approval",
            "Only after that explicit approval, serialize the exact approved Markdown",
            "without additions, omissions, or rewrites",
            "ask a fresh read-only reviewer to challenge the draft",
            "perform the same evidence-based challenge locally",
            "zd tasks import <area> --from - --commit --format json",
            "does not interrupt a task already being implemented",
            "consider the additions at the next `zd next` boundary",
        ] {
            assert!(
                reference_text.contains(expected),
                "{path} is missing approval contract: {expected}"
            );
        }

        assert!(
            !reference.contains("Present the exact Task Bundle JSON"),
            "{path} still makes raw JSON the default approval UI"
        );
        for obsolete in [
            "planning model and reasoning effort",
            "stronger planning configuration",
            "primary conversation",
            "Group only a small set of tasks",
        ] {
            assert!(
                !reference.contains(obsolete),
                "{path} still contains organizational policy: {obsolete}"
            );
        }
    }

    for (path, reference) in references.iter().skip(1) {
        assert_eq!(
            reference, &references[0].1,
            "{path} drifted from {}",
            references[0].0
        );
    }
}

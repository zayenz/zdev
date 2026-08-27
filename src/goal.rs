use std::path::Path;

use serde::Serialize;
use serde_json::{to_string_pretty, to_value};

use super::{CommandOutput, SCHEMA_VERSION, ZdevError, project, relative, tasks};

#[derive(Serialize)]
struct GoalArea {
    tag: String,
    title: String,
    objective: String,
    path: String,
}

#[derive(Serialize)]
struct GoalCounts {
    total: usize,
    open: usize,
    ready: usize,
    blocked: usize,
    done: usize,
}

#[derive(Serialize)]
struct GoalSlice {
    key: String,
    title: String,
    path: String,
    objective: String,
    boundaries: String,
}

#[derive(Serialize)]
struct GoalTask {
    id: String,
    key: String,
    title: String,
    complexity: tasks::TaskComplexity,
    path: String,
    outcome: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boundaries: Option<String>,
    done_when: String,
    validation: String,
    blocked_by: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slice: Option<GoalSlice>,
}

#[derive(Serialize)]
struct GoalProjection {
    schema_version: u64,
    area: GoalArea,
    lifecycle: &'static str,
    queue: &'static str,
    counts: GoalCounts,
    task: Option<GoalTask>,
    #[serde(skip_serializing_if = "Option::is_none")]
    native_goal: Option<String>,
}

pub(super) fn show(root: &Path, area: &str) -> Result<CommandOutput, ZdevError> {
    show_selected(root, area, None)
}

pub(super) fn show_selected(
    root: &Path,
    area: &str,
    selected: Option<&str>,
) -> Result<CommandOutput, ZdevError> {
    let (metadata, area_path) = project::load_area(root, area)?;
    let read = tasks::goal_tasks(root, area, selected)?;
    let summary = tasks::summary(root, area)?;
    tasks::validate_area_lifecycle(&metadata, &summary)?;
    let lifecycle = metadata.lifecycle.as_str();
    let queue = summary.queue().as_str();
    let counts = GoalCounts {
        total: read.total,
        open: read.open,
        ready: read.ready,
        blocked: read.blocked,
        done: read.done,
    };
    let mut text = format!(
        "Area: {} — {}\nLifecycle: {lifecycle}\nQueue: {queue}\nObjective:\n{}\nCounts: {} total; {} open; {} ready; {} blocked; {} done",
        metadata.tag,
        metadata.title,
        metadata.objective,
        counts.total,
        counts.open,
        counts.ready,
        counts.blocked,
        counts.done,
    );
    let (task, native_goal) = if metadata.lifecycle == project::AreaLifecycle::Open
        && let Some(task) = read.focus
    {
        let slice = task
            .slice
            .as_deref()
            .map(|key| project::goal_slice(root, area, key))
            .transpose()?
            .map(|slice| GoalSlice {
                key: slice.key,
                title: slice.title,
                path: relative(root, &slice.path),
                objective: slice.objective,
                boundaries: slice.boundaries,
            });
        let task_path = relative(root, &task.path);
        text.push_str(&format!(
            "\n\nTask: {} — {}\nComplexity: {}\nTask source: {task_path}\nOutcome:\n{}",
            task.id,
            task.title,
            task.complexity.as_str(),
            task.outcome
        ));
        if let Some(context) = &task.context {
            text.push_str(&format!("\n\nContext:\n{context}"));
        }
        if let Some(slice) = &slice {
            text.push_str(&format!(
                "\n\nSlice: {} — {}\nSlice source: {}\nSlice objective:\n{}\nSlice boundaries:\n{}",
                slice.key, slice.title, slice.path, slice.objective, slice.boundaries
            ));
        }
        if let Some(boundaries) = &task.boundaries {
            text.push_str(&format!("\n\nBoundaries:\n{boundaries}"));
        }
        text.push_str(&format!(
            "\n\nDone when:\n{}\n\nValidation:\n{}",
            task.done_when, task.validation
        ));
        let area_record = format!(".zdev/{area}/area.toml");
        let authoritative = match &slice {
            Some(slice) => format!("{area_record}, {}, and {task_path}", slice.path),
            None => format!("{area_record} and {task_path}"),
        };
        let native_goal = format!(
            "Complete zdev task {} in area {area}. Treat {authoritative} as authoritative. Meet the recorded outcome, boundaries, done-when conditions, and validation; preserve zdev approval, branch-safety, independent-verification, task-completion, and commit rules. Stop and report if the task is no longer ready or needs a product decision.",
            task.id
        );
        text.push_str(&format!("\n\nNative goal:\n{native_goal}"));
        (
            Some(GoalTask {
                id: task.id,
                key: task.key,
                title: task.title,
                complexity: task.complexity,
                path: task_path,
                outcome: task.outcome,
                context: task.context,
                boundaries: task.boundaries,
                done_when: task.done_when,
                validation: task.validation,
                blocked_by: task.blocked_by,
                slice,
            }),
            Some(native_goal),
        )
    } else {
        text.push_str(if lifecycle == "closed" {
            "\n\nThe area is closed. Reopen it before adding or selecting work."
        } else if queue == "empty" {
            "\n\nThe open area has no tasks. Create and approve a task, or close the area."
        } else {
            "\n\nThe open area's task queue is exhausted. Add approved work, reopen a task, or close the area."
        });
        (None, None)
    };
    let projection = GoalProjection {
        schema_version: SCHEMA_VERSION,
        area: GoalArea {
            tag: metadata.tag,
            title: metadata.title,
            objective: metadata.objective,
            path: relative(root, &area_path),
        },
        lifecycle,
        queue,
        counts,
        task,
        native_goal,
    };
    let json = to_string_pretty(&projection)
        .map_err(|error| ZdevError::new(format!("Cannot render zdev goal: {error}")))?;
    let value = to_value(projection)
        .map_err(|error| ZdevError::new(format!("Cannot render zdev goal: {error}")))?;
    Ok(CommandOutput::new(text, value).with_json(json))
}

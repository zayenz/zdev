use std::path::Path;

use serde::Serialize;
use serde_json::{to_string_pretty, to_value};

use super::{CommandOutput, SCHEMA_VERSION, ZdevError, project, relative, tasks};

#[derive(Debug, clap::Args, Default)]
pub struct GoalOptions {
    /// User-supplied guidance for choosing work across the area
    #[arg(long)]
    focus: Option<String>,
    /// Continue compatible tasks in independent worktree slots
    #[arg(long)]
    parallel: bool,
}

fn native_condition(area: &str, options: &GoalOptions) -> String {
    let execution = if options.parallel {
        "Use four independent task slots by default under the installed parallel contract, bounded by available worker capacity. Refill each free slot from fresh compatible ready work without waiting for the other slots. Integrate serially; independently verify and commit each task separately."
    } else {
        "Run the installed one-task implementation contract, independently verifying and committing each task before selecting again."
    };
    let selection = match options
        .focus
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        Some(focus) => format!(
            "Fuzzy focus: {}. Choose from the complete ready frontier, preferring AFK tasks; use focus to rank eligible work.",
            serde_json::to_string(focus).unwrap()
        ),
        None => "Let fresh work-context select the next task.".to_owned(),
    };
    format!(
        "Continue zdev area {area}. Read .zdev/{area}/area.toml and .zdev/{area}/brief.md for the current objective and boundaries. {execution} Use worktrees by default and retain the run's workspace and worker choices. {selection} Refresh repository state after each commit. Continue while the area is open, work is ready and safe, and the work remains authorized. Ordinary dispatch stops before hypotheses. Finish when fresh context is closed or the queue is empty or exhausted, or after reporting a confirmed blocker or required user decision with its reason and preserved state. Settle owned workers before finishing; preserve unfinished work and area lifecycle."
    )
}

#[derive(Serialize)]
struct GoalArea {
    tag: String,
    title: String,
    objective: String,
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    research: Option<super::research::Settings>,
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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    requires_code: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slice: Option<GoalSlice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hypothesis: Option<super::research::Hypothesis>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    research_results: Vec<super::research::ResultRecord>,
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

pub(super) fn show(
    root: &Path,
    area: &str,
    options: &GoalOptions,
) -> Result<CommandOutput, ZdevError> {
    render(root, area, None, options)
}

pub(super) fn show_selected(
    root: &Path,
    area: &str,
    selected: Option<&str>,
) -> Result<CommandOutput, ZdevError> {
    render(root, area, selected, &GoalOptions::default())
}

fn render(
    root: &Path,
    area: &str,
    selected: Option<&str>,
    options: &GoalOptions,
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
    if let Some(settings) = &metadata.research {
        text.push_str(&format!(
            "\nResearch settings:\n{}",
            to_string_pretty(settings).unwrap()
        ));
    }
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
        if let Some(hypothesis) = &task.hypothesis {
            text.push_str(&format!(
                "\nKind: hypothesis\nMechanism: {}\nContext tasks: {}\nResearch results:\n{}",
                hypothesis.mechanism,
                hypothesis.context_tasks.join(", "),
                to_string_pretty(&task.research_results).unwrap()
            ));
        }
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
        let native_goal = native_condition(area, options);
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
                requires_code: task.requires_code,
                slice,
                hypothesis: task.hypothesis,
                research_results: task.research_results,
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
            research: metadata.research,
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

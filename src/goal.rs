use std::path::Path;

use serde::Serialize;
use serde_json::{to_string_pretty, to_value};

use super::{CommandOutput, SCHEMA_VERSION, ZdevError, project, relative, tasks};

#[derive(Debug, clap::Args, Default)]
pub struct GoalOptions {
    /// User-supplied guidance for choosing work across the covered areas
    #[arg(long)]
    focus: Option<String>,
    /// Continue compatible tasks in independent worktree slots
    #[arg(long)]
    parallel: bool,
}

fn execution_clause(options: &GoalOptions) -> &'static str {
    if options.parallel {
        "Use four independent task slots by default under the installed parallel contract, bounded by available worker capacity. Refill each free slot from fresh compatible ready work without waiting for the other slots. Integrate serially; independently verify and commit each task separately."
    } else {
        "Run the installed one-task implementation contract, independently verifying and committing each task before selecting again."
    }
}

fn focus_text(options: &GoalOptions) -> Option<String> {
    options
        .focus
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|focus| serde_json::to_string(focus).unwrap())
}

fn native_condition(area: &str, options: &GoalOptions) -> String {
    let execution = execution_clause(options);
    let selection = match focus_text(options) {
        Some(focus) => format!(
            "Fuzzy focus: {focus}. Choose from the complete ready frontier, preferring AFK tasks; use focus to rank eligible work."
        ),
        None => "Let fresh work-context select the next task.".to_owned(),
    };
    format!(
        "Continue zdev area {area}. Read .zdev/{area}/area.toml and .zdev/{area}/brief.md for the current objective and boundaries. {execution} Retain the run's workspace and worker choices; with no workspace choice, follow workspace.branching (current checkout by default). {selection} Refresh repository state after each commit. Continue while the area is open, work is ready and safe, and the work remains authorized. Ordinary dispatch stops before hypotheses. Finish when fresh context is closed or the queue is empty, blocked or exhausted, or after reporting a confirmed blocker or required user decision with its reason and preserved state. Settle owned workers before finishing; preserve unfinished work and area lifecycle."
    )
}

/// Which areas a multi-area goal covers.
enum Coverage<'a> {
    All,
    Listed(&'a [String]),
}

fn multi_native_condition(coverage: &Coverage<'_>, options: &GoalOptions) -> String {
    let (scope, order) = match coverage {
        Coverage::All => (
            "Continue every open zdev area in this repository (the covered areas); `zdev goal --all --format json` lists them with their queues.".to_owned(),
            "in tag order",
        ),
        Coverage::Listed(areas) => (
            format!(
                "Continue zdev areas {} (the covered areas).",
                areas.join(", ")
            ),
            "in the listed order",
        ),
    };
    let execution = execution_clause(options);
    let selection = match focus_text(options) {
        Some(focus) => format!(
            "Fuzzy focus: {focus}. Choose from the complete ready frontiers of the covered areas, preferring AFK tasks; use focus to rank eligible work."
        ),
        None => format!(
            "Before each task, work in the first covered area {order} whose fresh work-context is open, ready, and safe, and let that work-context select the next task."
        ),
    };
    format!(
        "{scope} Before working in an area, read .zdev/<area>/area.toml and .zdev/<area>/brief.md for its current objective and boundaries. {execution} Retain the run's workspace and worker choices; with no workspace choice, follow workspace.branching (current checkout by default). {selection} Refresh repository state after each commit. Skip a covered area whose fresh context is closed, empty, blocked, exhausted, or unsafe for this checkout, and report why. Continue while a covered area is open, work is ready and safe, and the work remains authorized. Ordinary dispatch stops before hypotheses. Finish when no covered area has ready, safe work, or after reporting a confirmed blocker or required user decision with its reason and preserved state. Settle owned workers before finishing; preserve unfinished work and area lifecycles."
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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    cross_area_waits: Vec<tasks::CrossAreaWait>,
}

pub(super) fn show(
    root: &Path,
    areas: &[String],
    all: bool,
    options: &GoalOptions,
) -> Result<CommandOutput, ZdevError> {
    if all {
        let tags = project::list_areas(root)?
            .into_iter()
            .map(|area| area.tag)
            .collect::<Vec<_>>();
        return render_multi(root, &tags, &Coverage::All, options);
    }
    for (index, area) in areas.iter().enumerate() {
        if areas[..index].contains(area) {
            return Err(ZdevError::new(format!(
                "Area {area} is named more than once"
            )));
        }
    }
    match areas {
        [] => Err(ZdevError::new("Name at least one area, or pass --all")),
        [area] => render(root, area, None, options),
        _ => render_multi(root, areas, &Coverage::Listed(areas), options),
    }
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
        if lifecycle == "closed" {
            text.push_str("\n\nThe area is closed. Reopen it before adding or selecting work.");
        } else if queue == "empty" {
            text.push_str(
                "\n\nThe open area has no tasks. Create and approve a task, or close the area.",
            );
        } else if queue == "blocked" {
            text.push_str(&format!(
                "\n\nEvery open task waits on another area. Work here becomes ready when these blockers are done:{}",
                tasks::render_cross_area_waits(&summary.cross_area_waits)
            ));
        } else {
            text.push_str("\n\nThe open area's task queue is exhausted. Add approved work, reopen a task, or close the area.");
        }
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
        cross_area_waits: summary.cross_area_waits,
    };
    let json = to_string_pretty(&projection)
        .map_err(|error| ZdevError::new(format!("Cannot render zdev goal: {error}")))?;
    let value = to_value(projection)
        .map_err(|error| ZdevError::new(format!("Cannot render zdev goal: {error}")))?;
    Ok(CommandOutput::new(text, value).with_json(json))
}

#[derive(Serialize)]
struct MultiGoalTask {
    id: String,
    title: String,
    complexity: tasks::TaskComplexity,
    path: String,
}

#[derive(Serialize)]
struct MultiGoalArea {
    tag: String,
    title: String,
    path: String,
    lifecycle: &'static str,
    queue: &'static str,
    counts: GoalCounts,
    task: Option<MultiGoalTask>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    cross_area_waits: Vec<tasks::CrossAreaWait>,
}

#[derive(Serialize)]
struct MultiGoalProjection {
    schema_version: u64,
    scope: &'static str,
    areas: Vec<MultiGoalArea>,
    ready_areas: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    native_goal: Option<String>,
}

fn render_multi(
    root: &Path,
    tags: &[String],
    coverage: &Coverage<'_>,
    options: &GoalOptions,
) -> Result<CommandOutput, ZdevError> {
    let mut areas = Vec::new();
    for tag in tags {
        let (metadata, area_path) = project::load_area(root, tag)?;
        let read = tasks::goal_tasks(root, tag, None)?;
        let summary = tasks::summary(root, tag)?;
        tasks::validate_area_lifecycle(&metadata, &summary)?;
        let open = metadata.lifecycle == project::AreaLifecycle::Open;
        areas.push(MultiGoalArea {
            tag: metadata.tag,
            title: metadata.title,
            path: relative(root, &area_path),
            lifecycle: metadata.lifecycle.as_str(),
            queue: summary.queue().as_str(),
            counts: GoalCounts {
                total: read.total,
                open: read.open,
                ready: read.ready,
                blocked: read.blocked,
                done: read.done,
            },
            task: read.focus.filter(|_| open).map(|task| MultiGoalTask {
                path: relative(root, &task.path),
                id: task.id,
                title: task.title,
                complexity: task.complexity,
            }),
            cross_area_waits: summary.cross_area_waits,
        });
    }
    let ready_areas = areas
        .iter()
        .filter(|area| area.task.is_some())
        .map(|area| area.tag.clone())
        .collect::<Vec<_>>();
    let native_goal = (!ready_areas.is_empty()).then(|| multi_native_condition(coverage, options));
    let mut text = match coverage {
        Coverage::All => "Areas: all".to_owned(),
        Coverage::Listed(tags) => format!("Areas: {}", tags.join(", ")),
    };
    text.push_str(&format!(
        "\nReady areas: {}",
        if ready_areas.is_empty() {
            "none".to_owned()
        } else {
            ready_areas.join(", ")
        }
    ));
    for area in &areas {
        let counts = &area.counts;
        text.push_str(&format!(
            "\n\nArea: {} — {}\nLifecycle: {}\nQueue: {}\nCounts: {} total; {} open; {} ready; {} blocked; {} done",
            area.tag,
            area.title,
            area.lifecycle,
            area.queue,
            counts.total,
            counts.open,
            counts.ready,
            counts.blocked,
            counts.done,
        ));
        if let Some(task) = &area.task {
            text.push_str(&format!("\nNext task: {} — {}", task.id, task.title));
        } else if area.lifecycle == "open" && area.queue == "blocked" {
            text.push_str(&format!(
                "\nWaiting on other areas:{}",
                tasks::render_cross_area_waits(&area.cross_area_waits)
            ));
        }
    }
    match &native_goal {
        Some(goal) => text.push_str(&format!("\n\nNative goal:\n{goal}")),
        None if areas.is_empty() => text.push_str("\n\nNo zdev area exists."),
        None => text.push_str("\n\nNo covered area has ready work."),
    }
    let projection = MultiGoalProjection {
        schema_version: SCHEMA_VERSION,
        scope: match coverage {
            Coverage::All => "all",
            Coverage::Listed(_) => "areas",
        },
        areas,
        ready_areas,
        native_goal,
    };
    let json = to_string_pretty(&projection)
        .map_err(|error| ZdevError::new(format!("Cannot render zdev goal: {error}")))?;
    let value = to_value(projection)
        .map_err(|error| ZdevError::new(format!("Cannot render zdev goal: {error}")))?;
    Ok(CommandOutput::new(text, value).with_json(json))
}

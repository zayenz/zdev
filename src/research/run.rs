use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use clap::{Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::{Direction, Settings};
use crate::{
    CommandOutput, ZdevError, ZdevStateLock, git_output, git_output_bytes, tasks, write_atomic,
};

const DEFAULT_ATTEMPT_LIMIT: usize = 20;

#[derive(Debug, Subcommand)]
pub(crate) enum Action {
    /// Authorize mixed generated work within the area policy and explicit scope
    Start {
        #[arg(long)]
        scope: String,
        #[arg(long)]
        until_interrupted: bool,
        #[arg(long)]
        attempt_limit: Option<usize>,
        #[arg(long)]
        command_limit: Option<usize>,
        #[arg(long)]
        command_seconds: Option<f64>,
        #[arg(long)]
        time_limit_seconds: Option<f64>,
        #[arg(long)]
        target: Option<f64>,
        #[arg(long, default_value_t = 1)]
        worker_limit: usize,
        #[arg(long, default_value_t = 1)]
        measurement_limit: usize,
    },
    /// Inspect current policy, task results, unfinished checkpoints and live remaining limits
    Context,
    /// Read the same fresh projection as context
    Inspect,
    /// Admit an exact mixed bundle under the run's authorization; retry by request identity
    Admit {
        #[arg(long)]
        run: String,
        #[arg(long)]
        request: String,
        #[arg(long = "from")]
        source: PathBuf,
    },
    /// Charge one coordinator work dispatch before preparation, repair or other work
    Reserve {
        #[arg(long)]
        run: String,
        #[arg(long)]
        request: String,
        #[arg(long)]
        task: String,
        #[arg(long, value_enum)]
        kind: WorkKind,
        /// Claim native role capacity before launching a worker
        #[arg(long)]
        worker_slot: bool,
    },
    /// Settle completed supporting work; experiment costs settle through its checkpoint
    Settle {
        #[arg(long)]
        run: String,
        #[arg(long)]
        request: String,
    },
    /// Preserve authorization and allowance while interrupting further dispatch
    Stop {
        #[arg(long)]
        run: String,
    },
    /// Resume the same interrupted run without resetting its original time boundary
    Resume {
        #[arg(long)]
        run: String,
    },
    /// Add explicitly authorized allowance without resetting costs or unfinished work
    Extend {
        #[arg(long)]
        run: String,
        /// Stable identity: retrying the same extension never adds allowance twice
        #[arg(long)]
        request: String,
        #[arg(long)]
        add_attempts: Option<usize>,
        #[arg(long)]
        add_commands: Option<usize>,
        #[arg(long)]
        add_command_seconds: Option<f64>,
        /// Add seconds to the existing deadline
        #[arg(long)]
        add_time_seconds: Option<f64>,
    },
    /// Halt affected experimentation on a shared environment failure
    Block {
        #[arg(long)]
        run: String,
        #[arg(long)]
        reason: String,
    },
    /// Record resolution of the shared failure; an actual rebaseline is still required
    Resolve {
        #[arg(long)]
        run: String,
    },
    /// Measure current accepted code with actual correctness commands and evaluator
    Rebaseline {
        #[arg(long)]
        run: String,
        #[arg(long)]
        request: String,
        /// Explicit generated output path to preserve outside the clean-code check
        #[arg(long = "untracked-output", value_name = "PATH")]
        untracked_outputs: Vec<String>,
    },
    /// End this authorization once reservations and experiment checkpoints are settled
    End {
        #[arg(long)]
        run: String,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub(crate) enum WorkKind {
    Supporting,
    Preparation,
    Repair,
    Rerun,
    Confirmation,
    Reconciliation,
    Rebaseline,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Binding {
    run: String,
    pub(crate) request: String,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Run {
    schema_version: u64,
    area: String,
    id: String,
    scope: String,
    until_interrupted: bool,
    policy: Settings,
    started_at: f64,
    deadline: Option<f64>,
    attempt_limit: Option<usize>,
    command_limit: Option<usize>,
    command_seconds: Option<f64>,
    target: Option<f64>,
    #[serde(default = "serial_capacity")]
    worker_limit: usize,
    #[serde(default = "serial_capacity")]
    measurement_limit: usize,
    #[serde(default)]
    acceptance: Option<String>,
    stop: Option<Stop>,
    comparison_valid: bool,
    invalidation: Option<String>,
    ended: bool,
    admissions: BTreeMap<String, Admission>,
    reservations: BTreeMap<String, Reservation>,
    rebaseline: Option<Baseline>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    extensions: BTreeMap<String, Extension>,
}

#[derive(Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Extension {
    attempts: Option<usize>,
    commands: Option<usize>,
    command_seconds: Option<f64>,
    time_seconds: Option<f64>,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "reason", rename_all = "snake_case", deny_unknown_fields)]
enum Stop {
    Interruption,
    Blocker { description: String },
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Admission {
    fingerprint: String,
    tasks: Vec<String>,
    pending: BTreeMap<String, Publication>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Publication {
    before: Option<Vec<u8>>,
    after: Vec<u8>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Reservation {
    task: Option<String>,
    kind: WorkKind,
    bound: bool,
    settled: bool,
    commands: usize,
    elapsed_seconds: f64,
    pending_timeout: Option<f64>,
    #[serde(default)]
    worker: bool,
    #[serde(default)]
    requested_worker_slot: bool,
    #[serde(default)]
    measurement: bool,
}

// Baseline progress belongs here until measured samples can replace area settings.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Baseline {
    request: String,
    #[serde(default)]
    untracked_outputs: Vec<String>,
    head: String,
    settings: Settings,
    samples: Vec<f64>,
    owner_pid: u32,
    running_pid: Option<u32>,
    invocation_started: Option<f64>,
    complete: bool,
    commands_started: usize,
    published: bool,
}

pub(crate) fn run(root: &Path, area: &str, action: &Action) -> Result<CommandOutput, ZdevError> {
    let mut dispatch_authorized = false;
    if let Action::Rebaseline {
        run,
        request,
        untracked_outputs,
    } = action
    {
        rebaseline(root, area, run, request, untracked_outputs)?;
    } else if !matches!(action, Action::Context | Action::Inspect) {
        let _lock = ZdevStateLock::acquire(root)?;
        match action {
            Action::Start {
                scope,
                until_interrupted,
                attempt_limit,
                command_limit,
                command_seconds,
                time_limit_seconds,
                target,
                worker_limit,
                measurement_limit,
            } => {
                start(
                    root,
                    area,
                    scope,
                    *until_interrupted,
                    *attempt_limit,
                    *command_limit,
                    *command_seconds,
                    *time_limit_seconds,
                    *target,
                    *worker_limit,
                    *measurement_limit,
                )?;
            }
            Action::Admit {
                run,
                request,
                source,
            } => admit(root, area, run, request, source)?,
            Action::Reserve {
                run: identity,
                request,
                task,
                kind,
                worker_slot,
            } => {
                let mut run = authorized(root, area, identity)?;
                dispatch_authorized = reserve(
                    root,
                    &mut run,
                    request,
                    Some(task),
                    *kind,
                    false,
                    *worker_slot,
                )?;
            }
            Action::Settle {
                run: identity,
                request,
            } => {
                let mut run = authorized(root, area, identity)?;
                let bound_to_checkpoint = checkpoints(root, area)?.iter().any(|checkpoint| {
                    checkpoint["preparation"]["request"].as_str() == Some(request)
                        || checkpoint["isolated"]["reconciliation"]["request"].as_str()
                            == Some(request)
                        || checkpoint["attempts"].as_array().is_some_and(|attempts| {
                            attempts.iter().any(|attempt| {
                                attempt["reservation"]["request"].as_str() == Some(request)
                            })
                        })
                }) || run
                    .rebaseline
                    .as_ref()
                    .is_some_and(|baseline| baseline.request == *request);
                let reservation = run
                    .reservations
                    .get_mut(request)
                    .ok_or_else(|| ZdevError::new("Unknown work reservation"))?;
                if reservation.bound && bound_to_checkpoint {
                    return Err(ZdevError::new(
                        "Bound experiment costs settle through its checkpoint",
                    ));
                }
                reservation.settled = true;
                save(root, &run)?;
            }
            Action::Stop { run: identity } => {
                let mut run = authorized(root, area, identity)?;
                if !matches!(run.stop, Some(Stop::Blocker { .. })) {
                    run.stop = Some(Stop::Interruption);
                }
                save(root, &run)?;
            }
            Action::Resume { run: identity } => {
                let mut run = authorized(root, area, identity)?;
                if matches!(run.stop, Some(Stop::Blocker { .. })) {
                    return Err(ZdevError::new(
                        "Resolve the shared environment blocker before resuming",
                    ));
                }
                run.stop = None;
                save(root, &run)?;
            }
            Action::Extend {
                run: identity,
                request,
                add_attempts,
                add_commands,
                add_command_seconds,
                add_time_seconds,
            } => extend(
                root,
                area,
                identity,
                request,
                Extension {
                    attempts: *add_attempts,
                    commands: *add_commands,
                    command_seconds: *add_command_seconds,
                    time_seconds: *add_time_seconds,
                },
            )?,
            Action::Block {
                run: identity,
                reason,
            } => {
                crate::validate_nonempty_line(reason, "Shared environment failure")?;
                let mut run = authorized(root, area, identity)?;
                run.stop = Some(Stop::Blocker {
                    description: reason.clone(),
                });
                invalidate(
                    &mut run,
                    "Shared environment failure requires a fresh baseline",
                );
                save(root, &run)?;
            }
            Action::Resolve { run: identity } => {
                let mut run = authorized(root, area, identity)?;
                if !matches!(run.stop, Some(Stop::Blocker { .. })) {
                    return Err(ZdevError::new("Run has no shared environment blocker"));
                }
                run.stop = None;
                save(root, &run)?;
            }
            Action::End { run: identity } => {
                let mut run = authorized(root, area, identity)?;
                if run
                    .reservations
                    .values()
                    .any(|reservation| !reservation.settled)
                    || checkpoints(root, area)?
                        .iter()
                        .any(|checkpoint| checkpoint["index_updated"] != true)
                    || run
                        .admissions
                        .values()
                        .any(|admission| !admission.pending.is_empty())
                {
                    return Err(ZdevError::new(
                        "Recover and settle unfinished work before ending this run",
                    ));
                }
                run.ended = true;
                save(root, &run)?;
            }
            Action::Context | Action::Inspect | Action::Rebaseline { .. } => unreachable!(),
        }
    }
    let mut output = context(root, area)?;
    if matches!(action, Action::Reserve { .. }) {
        output.value["dispatch_authorized"] = json!(dispatch_authorized);
        output.text = serde_json::to_string_pretty(&output.value).unwrap();
    }
    Ok(output)
}

fn path(root: &Path, area: &str) -> Result<PathBuf, ZdevError> {
    crate::validate_segment(area, "Area tag")?;
    let path = PathBuf::from(git_output(
        root,
        &[
            "rev-parse",
            "--git-path",
            &format!("zdev/runs/{area}/current.json"),
        ],
    )?);
    Ok(if path.is_absolute() {
        path
    } else {
        root.join(path)
    })
}

fn load(root: &Path, area: &str) -> Result<Option<Run>, ZdevError> {
    let path = path(root, area)?;
    if !path
        .try_exists()
        .map_err(|error| ZdevError::io("Cannot inspect research run", error))?
    {
        return Ok(None);
    }
    let run: Run = super::read_json(&path)?;
    if run.schema_version != 1 || run.area != area {
        return Err(ZdevError::new(
            "Research run does not match the selected area",
        ));
    }
    run.policy.validate()?;
    crate::validate_segment(&run.id, "Run identity")?;
    crate::validate_nonempty_line(&run.scope, "Authorized research scope")?;
    if !run.started_at.is_finite()
        || run.started_at < 0.0
        || run
            .deadline
            .is_some_and(|value| !value.is_finite() || value < run.started_at)
        || (!run.until_interrupted && run.attempt_limit.is_none())
        || run.worker_limit == 0
        || run.measurement_limit == 0
        || run.attempt_limit == Some(0)
        || run.command_limit == Some(0)
        || [run.command_seconds]
            .into_iter()
            .flatten()
            .any(|value| !value.is_finite() || value <= 0.0)
        || run.target.is_some_and(|value| !value.is_finite())
    {
        return Err(ZdevError::new("Research run contains invalid limits"));
    }
    for (request, reservation) in &run.reservations {
        crate::validate_nonempty_line(request, "Work request identity")?;
        if !reservation.elapsed_seconds.is_finite()
            || reservation.elapsed_seconds < 0.0
            || reservation
                .pending_timeout
                .is_some_and(|value| !value.is_finite() || value <= 0.0)
        {
            return Err(ZdevError::new(
                "Research reservation contains invalid command costs",
            ));
        }
    }
    for admission in run.admissions.values() {
        for relative in admission.pending.keys() {
            if Path::new(relative)
                .components()
                .any(|component| !matches!(component, std::path::Component::Normal(_)))
                || !(relative == &format!(".zdev/{area}/TASKS.md")
                    || (relative.starts_with(&format!(".zdev/{area}/tasks/"))
                        && relative.ends_with(".md")))
            {
                return Err(ZdevError::new(
                    "Research admission contains an invalid publication path",
                ));
            }
        }
    }
    Ok(Some(run))
}

fn save(root: &Path, run: &Run) -> Result<(), ZdevError> {
    let path = path(root, &run.area)?;
    fs::create_dir_all(path.parent().unwrap())
        .map_err(|error| ZdevError::io("Cannot create research run directory", error))?;
    write_atomic(&path, &serde_json::to_vec_pretty(run).unwrap())
}

fn authorized(root: &Path, area: &str, identity: &str) -> Result<Run, ZdevError> {
    let run = load(root, area)?
        .ok_or_else(|| ZdevError::new("Start an explicitly authorized research run first"))?;
    if run.id != identity || run.ended {
        return Err(ZdevError::new(
            "Authorization does not match the current unfinished run",
        ));
    }
    Ok(run)
}

pub(crate) fn active(root: &Path, area: &str) -> Result<bool, ZdevError> {
    Ok(load(root, area)?.is_some_and(|run| !run.ended))
}

#[allow(clippy::too_many_arguments)]
fn start(
    root: &Path,
    area: &str,
    scope: &str,
    until_interrupted: bool,
    attempts: Option<usize>,
    commands: Option<usize>,
    command_seconds: Option<f64>,
    time: Option<f64>,
    target: Option<f64>,
    worker_limit: usize,
    measurement_limit: usize,
) -> Result<(), ZdevError> {
    require_area(root, area)?;
    crate::validate_nonempty_line(scope, "Authorized research scope")?;
    for value in [command_seconds, time].into_iter().flatten() {
        if !value.is_finite() || value <= 0.0 {
            return Err(ZdevError::new(
                "Research resource limits must be finite and positive",
            ));
        }
    }
    if worker_limit == 0
        || measurement_limit == 0
        || attempts == Some(0)
        || commands == Some(0)
        || target.is_some_and(|value| !value.is_finite())
    {
        return Err(ZdevError::new(
            "Count limits must be positive and target must be finite",
        ));
    }
    if let Some(previous) = load(root, area)? {
        if !previous.ended {
            return Err(ZdevError::new(
                "An unfinished run already exists; inspect or resume its original authorization",
            ));
        }
        let archive = path(root, area)?.with_file_name(format!("{}.json", previous.id));
        write_atomic(&archive, &serde_json::to_vec_pretty(&previous).unwrap())?;
    }
    if checkpoints(root, area)?
        .iter()
        .any(|checkpoint| checkpoint["index_updated"] != true)
    {
        return Err(ZdevError::new(
            "Recover unfinished experiments before authorizing another run",
        ));
    }
    let policy = super::settings(root, area)?;
    let valid = accepted_code_matches(root, &policy)?;
    let started_at = unix_seconds();
    let run = Run {
        schema_version: 1,
        area: area.to_owned(),
        id: crate::generate_change_id()?.replacen('Z', "run-", 1),
        scope: scope.to_owned(),
        until_interrupted,
        policy,
        started_at,
        deadline: time.map(|seconds| started_at + seconds),
        attempt_limit: attempts.or((!until_interrupted).then_some(DEFAULT_ATTEMPT_LIMIT)),
        command_limit: commands,
        command_seconds,
        target,
        worker_limit,
        measurement_limit,
        acceptance: None,
        stop: None,
        comparison_valid: valid,
        invalidation: (!valid).then(|| "Accepted code requires a measured baseline".to_owned()),
        ended: false,
        admissions: BTreeMap::new(),
        reservations: BTreeMap::new(),
        rebaseline: None,
        extensions: BTreeMap::new(),
    };
    save(root, &run)
}

fn extend(
    root: &Path,
    area: &str,
    identity: &str,
    request: &str,
    extra: Extension,
) -> Result<(), ZdevError> {
    let mut run = authorized(root, area, identity)?;
    crate::validate_nonempty_line(request, "Extension request identity")?;
    if let Some(previous) = run.extensions.get(request) {
        return if previous == &extra {
            Ok(())
        } else {
            Err(ZdevError::new(
                "Changed allowance under an existing extension request",
            ))
        };
    }
    if extra.attempts.is_none()
        && extra.commands.is_none()
        && extra.command_seconds.is_none()
        && extra.time_seconds.is_none()
    {
        return Err(ZdevError::new("Specify at least one allowance to add"));
    }
    let extend_count =
        |limit: Option<usize>, addition: Option<usize>| -> Result<Option<usize>, ZdevError> {
            match addition {
                None => Ok(limit),
                Some(added) if added > 0 => limit
                    .and_then(|value| value.checked_add(added))
                    .map(Some)
                    .ok_or_else(|| {
                        ZdevError::new("Cannot extend an unlimited or overflowing count limit")
                    }),
                _ => Err(ZdevError::new("Added allowances must be positive")),
            }
        };
    let extend_seconds =
        |limit: Option<f64>, addition: Option<f64>| -> Result<Option<f64>, ZdevError> {
            match addition {
                None => Ok(limit),
                Some(added) if added.is_finite() && added > 0.0 => limit
                    .map(|value| value + added)
                    .filter(|value| value.is_finite())
                    .map(Some)
                    .ok_or_else(|| {
                        ZdevError::new("Cannot extend an unlimited or overflowing time limit")
                    }),
                _ => Err(ZdevError::new(
                    "Added allowances must be finite and positive",
                )),
            }
        };
    run.attempt_limit = extend_count(run.attempt_limit, extra.attempts)?;
    run.command_limit = extend_count(run.command_limit, extra.commands)?;
    run.command_seconds = extend_seconds(run.command_seconds, extra.command_seconds)?;
    run.deadline = extend_seconds(run.deadline, extra.time_seconds)?;
    run.extensions.insert(request.to_owned(), extra);
    save(root, &run)
}

fn require_area(root: &Path, area: &str) -> Result<(), ZdevError> {
    crate::project::require_task_work_area_link(root, area)?;
    if crate::project::load_area(root, area)?.0.lifecycle != crate::project::AreaLifecycle::Open {
        return Err(ZdevError::new("Research dispatch requires an open area"));
    }
    Ok(())
}

fn accepted_code_matches(root: &Path, settings: &Settings) -> Result<bool, ZdevError> {
    Ok(git_output_bytes(
        root,
        &[
            "diff",
            "--name-only",
            "-z",
            &settings.comparison.baseline_revision,
            "HEAD",
            "--",
            ".",
            ":(exclude).zdev",
        ],
    )?
    .is_empty())
}

fn invalidate(run: &mut Run, reason: &str) {
    run.comparison_valid = false;
    run.invalidation = Some(reason.to_owned());
}

fn policy_matches(frozen: &Settings, current: &Settings) -> bool {
    let a = &frozen.comparison;
    let b = &current.comparison;
    a.metric == b.metric
        && a.direction == b.direction
        && a.repetitions == b.repetitions
        && a.aggregation == b.aggregation
        && a.minimum_gain == b.minimum_gain
        && a.required_checks == b.required_checks
        && a.significance == b.significance
        && frozen.correctness_commands == current.correctness_commands
        && frozen.permitted_changes == current.permitted_changes
}

/// Caller holds the state lock. Configuration cannot clear invalidation with supplied samples.
pub(crate) fn configure(
    root: &Path,
    area: &str,
    previous: &Settings,
    current: &Settings,
) -> Result<(), ZdevError> {
    let Some(mut run) = load(root, area)?.filter(|run| !run.ended) else {
        return Ok(());
    };
    if !policy_matches(&run.policy, current) {
        return Err(ZdevError::new(
            "Active run freezes its metric, direction, scope and correctness/acceptance requirements",
        ));
    }
    if previous.comparison.baseline_revision != current.comparison.baseline_revision
        || previous.comparison.baseline_samples != current.comparison.baseline_samples
    {
        return Err(ZdevError::new(
            "Supplied replacement samples cannot rebaseline an active run; use research run rebaseline",
        ));
    }
    if !previous.eq_ignoring_advisory(current) {
        invalidate(
            &mut run,
            "Evaluator or workload settings changed; actual rebaseline required",
        );
        save(root, &run)?;
    }
    Ok(())
}

/// Historical evidence may be recorded/restored after comparison conditions change.
pub(crate) fn record_policy_compatible(
    root: &Path,
    area: &str,
    original: &Settings,
) -> Result<bool, ZdevError> {
    let Some(run) = load(root, area)?.filter(|run| !run.ended) else {
        return Ok(false);
    };
    let current = super::settings(root, area)?;
    Ok(policy_matches(&run.policy, original) && policy_matches(&run.policy, &current))
}

fn require_policy(root: &Path, run: &mut Run, comparison: bool) -> Result<(), ZdevError> {
    require_area(root, &run.area)?;
    let settings = super::settings(root, &run.area)?;
    if !policy_matches(&run.policy, &settings) {
        return Err(ZdevError::new(
            "Current research policy differs from the run's authorization",
        ));
    }
    if !settings.eq_ignoring_advisory(&run.policy) || !accepted_code_matches(root, &settings)? {
        invalidate(
            run,
            "Evaluator/workload or accepted supporting code changed; actual rebaseline required",
        );
        save(root, run)?;
    }
    if comparison && !run.comparison_valid {
        return Err(ZdevError::new(
            "Comparisons are invalid; run an actual counted rebaseline first",
        ));
    }
    Ok(())
}

fn elapsed_cost(run: &Run) -> f64 {
    run.reservations
        .values()
        .map(|reservation| reservation.elapsed_seconds + reservation.pending_timeout.unwrap_or(0.0))
        .sum()
}

fn command_count(run: &Run) -> usize {
    run.reservations
        .values()
        .map(|reservation| reservation.commands)
        .sum()
}

fn target_reached(run: &Run, settings: &Settings) -> Result<bool, ZdevError> {
    let Some(target) = run.target.filter(|_| run.comparison_valid) else {
        return Ok(false);
    };
    let value = settings
        .comparison
        .aggregate(&settings.comparison.baseline_samples)?;
    Ok(match settings.comparison.direction {
        Direction::Minimize => value <= target,
        Direction::Maximize => value >= target,
    })
}

fn reason(
    run: &Run,
    settings: &Settings,
    new_work: bool,
) -> Result<Option<&'static str>, ZdevError> {
    if run.ended {
        return Ok(Some("ended"));
    }
    if let Some(stop) = &run.stop {
        return Ok(Some(match stop {
            Stop::Interruption => "interruption",
            Stop::Blocker { .. } => "blocker",
        }));
    }
    limits_reason(run, settings, new_work)
}

fn limits_reason(
    run: &Run,
    settings: &Settings,
    new_work: bool,
) -> Result<Option<&'static str>, ZdevError> {
    if target_reached(run, settings)? {
        return Ok(Some("target"));
    }
    if run
        .deadline
        .is_some_and(|deadline| unix_seconds() >= deadline)
        || run
            .command_limit
            .is_some_and(|limit| command_count(run) >= limit)
        || run
            .command_seconds
            .is_some_and(|limit| elapsed_cost(run) >= limit)
        || (new_work
            && run
                .attempt_limit
                .is_some_and(|limit| run.reservations.len() >= limit))
    {
        return Ok(Some("budget"));
    }
    Ok(None)
}

fn require_dispatch(
    root: &Path,
    run: &mut Run,
    kind: WorkKind,
    new_work: bool,
    task: Option<&str>,
    worker: bool,
) -> Result<(), ZdevError> {
    require_no_pending_withdrawal(root, &run.area)?;
    let acceptance_owner = worker
        && task.is_some()
        && run.acceptance.as_deref() == task
        && matches!(kind, WorkKind::Confirmation | WorkKind::Repair)
        && checkpoints(root, &run.area)?.iter().any(|checkpoint| {
            checkpoint["task"].as_str() == task
                && checkpoint["isolated"]["integrated"] == true
                && checkpoint["isolated"]["integrating"] == false
        });
    if run.acceptance.is_some()
        && !matches!(kind, WorkKind::Preparation | WorkKind::Supporting)
        && !acceptance_owner
    {
        return Err(ZdevError::new(
            "Finish the acceptance lane before starting measurements",
        ));
    }
    require_policy(
        root,
        run,
        !matches!(kind, WorkKind::Supporting | WorkKind::Rebaseline),
    )?;
    if run.reservations.iter().any(|(request, reservation)| {
        reservation.pending_timeout.is_some() && !live_isolated_measurement(root, run, request)
    }) {
        return Err(ZdevError::new(
            "Unresolved command cost; recover its checkpoint before dispatching more work",
        ));
    }
    let settings = super::settings(root, &run.area)?;
    let stop = if kind == WorkKind::Supporting && matches!(run.stop, Some(Stop::Blocker { .. })) {
        limits_reason(run, &settings, new_work)?
    } else {
        reason(run, &settings, new_work)?
    };
    if let Some(reason) = stop {
        return Err(ZdevError::new(format!(
            "Research run stopped: {reason}; consumed allowance is retained"
        )));
    }
    Ok(())
}

fn reserve(
    root: &Path,
    run: &mut Run,
    request: &str,
    task: Option<&str>,
    kind: WorkKind,
    bound: bool,
    worker_slot: bool,
) -> Result<bool, ZdevError> {
    crate::validate_nonempty_line(request, "Work request identity")?;
    if let Some(existing) = run.reservations.get(request) {
        if existing.task.as_deref() != task
            || existing.kind != kind
            || existing.requested_worker_slot != worker_slot
        {
            return Err(ZdevError::new(
                "Changed work under an existing reservation identity",
            ));
        }
        return Ok(false);
    }
    require_dispatch(root, run, kind, true, task, worker_slot)?;
    if worker_slot {
        if !matches!(
            kind,
            WorkKind::Supporting
                | WorkKind::Preparation
                | WorkKind::Repair
                | WorkKind::Confirmation
        ) {
            return Err(ZdevError::new(
                "This work kind does not authorize a native worker",
            ));
        }
        require_capacity(run, true)?;
    }
    if kind == WorkKind::Rebaseline {
        require_capacity(run, false)?;
        if run.acceptance.is_some() {
            return Err(ZdevError::new(
                "Finish the acceptance lane before rebaselining",
            ));
        }
    }
    if let Some(task) = task {
        let view = tasks::show(root, &run.area, task)?.value;
        if view["state"] != "ready" || view["status"] != "open" {
            return Err(ZdevError::new(
                "Research work dispatch requires a ready open task",
            ));
        }
        if kind == WorkKind::Supporting && !view["hypothesis"].is_null() {
            return Err(ZdevError::new(
                "Supporting reservations require ordinary tasks",
            ));
        }
    }
    run.reservations.insert(
        request.to_owned(),
        Reservation {
            task: task.map(str::to_owned),
            kind,
            bound,
            settled: false,
            commands: 0,
            elapsed_seconds: 0.0,
            pending_timeout: None,
            worker: worker_slot,
            requested_worker_slot: worker_slot,
            measurement: kind == WorkKind::Rebaseline,
        },
    );
    save(root, run)?;
    Ok(worker_slot)
}

fn admit(
    root: &Path,
    area: &str,
    identity: &str,
    request: &str,
    source: &Path,
) -> Result<(), ZdevError> {
    let mut run = authorized(root, area, identity)?;
    require_no_pending_withdrawal(root, area)?;
    crate::validate_nonempty_line(request, "Admission request identity")?;
    let bundle = tasks::read_task_bundle(source)?;
    let fingerprint = tasks::generated_bundle_fingerprint(&bundle)?;
    if let Some(admission) = run.admissions.get(request) {
        if admission.fingerprint != fingerprint {
            return Err(ZdevError::new(
                "Changed bundle under an existing admission identity",
            ));
        }
        require_area(root, area)?;
        return recover_admission(root, &mut run, request);
    }
    if run.acceptance.is_some() {
        return Err(ZdevError::new(
            "Finish the acceptance lane before publishing tasks",
        ));
    }
    require_dispatch(root, &mut run, WorkKind::Supporting, true, None, false)?;
    if run
        .admissions
        .values()
        .any(|admission| !admission.pending.is_empty())
    {
        return Err(ZdevError::new(
            "Recover the pending admission before admitting another bundle",
        ));
    }
    let generated = tasks::prepare_generated_bundle(root, area, &bundle)?;
    let mut pending = BTreeMap::new();
    for (path, after) in generated.files {
        let relative = crate::relative(root, &path);
        let before = match fs::read(&path) {
            Ok(bytes) if relative.ends_with("/TASKS.md") => Some(bytes),
            Ok(_) => return Err(ZdevError::new("Allocated task path already exists")),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => return Err(ZdevError::io("Cannot inspect admission publication", error)),
        };
        pending.insert(relative, Publication { before, after });
    }
    run.admissions.insert(
        request.to_owned(),
        Admission {
            fingerprint,
            tasks: generated.ids,
            pending,
        },
    );
    save(root, &run)?;
    recover_admission(root, &mut run, request)
}

fn recover_admission(root: &Path, run: &mut Run, request: &str) -> Result<(), ZdevError> {
    let pending = &run.admissions[request].pending;
    // Check every destination before publishing any, and only overwrite exactly admitted bytes.
    for (path, publication) in pending {
        if !git_output_bytes(
            root,
            &[
                "--literal-pathspecs",
                "diff",
                "--cached",
                "--name-only",
                "-z",
                "--",
                path,
            ],
        )?
        .is_empty()
        {
            return Err(ZdevError::new(format!(
                "Admission destination has staged work: {path}"
            )));
        }
        let bytes = match fs::read(root.join(path)) {
            Ok(bytes) => Some(bytes),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => return Err(ZdevError::io("Cannot read admission destination", error)),
        };
        if bytes.as_deref() != Some(publication.after.as_slice()) && bytes != publication.before {
            return Err(ZdevError::new(format!(
                "Admission destination changed; preserve and inspect {path}"
            )));
        }
    }
    for (path, publication) in pending {
        write_atomic(&root.join(path), &publication.after)?;
    }
    run.admissions.get_mut(request).unwrap().pending.clear();
    save(root, run)
}

/// Caller holds the state lock; a bound reservation is consumed by exactly one checkpoint operation.
pub(crate) fn bind_work(
    root: &Path,
    area: &str,
    task: &str,
    request: &str,
    kind: WorkKind,
    provided: Option<&str>,
) -> Result<Option<Binding>, ZdevError> {
    bind_work_with_capacity(root, area, task, request, kind, provided, false)
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn bind_work_with_capacity(
    root: &Path,
    area: &str,
    task: &str,
    request: &str,
    kind: WorkKind,
    provided: Option<&str>,
    worker: bool,
) -> Result<Option<Binding>, ZdevError> {
    let Some(mut run) = load(root, area)?.filter(|run| !run.ended) else {
        if provided.is_some() || worker {
            return Err(ZdevError::new("No active run owns this reservation"));
        }
        return Ok(None);
    };
    let measurement = !worker && !matches!(kind, WorkKind::Preparation | WorkKind::Reconciliation);
    let request = provided.unwrap_or(request);
    if let Some(reservation) = run.reservations.get(request)
        && reservation.task.as_deref() == Some(task)
        && reservation.kind == kind
        && reservation.bound
        && !reservation.settled
        && reservation.commands == 0
        && (worker || !reservation.requested_worker_slot)
        && !checkpoints(root, area)?.iter().any(|checkpoint| {
            checkpoint["preparation"]["request"].as_str() == Some(request)
                || checkpoint["isolated"]["reconciliation"]["request"].as_str() == Some(request)
                || checkpoint["attempts"].as_array().is_some_and(|attempts| {
                    attempts
                        .iter()
                        .any(|attempt| attempt["reservation"]["request"].as_str() == Some(request))
                })
        })
    {
        let claimed = if worker {
            reservation.worker
        } else {
            reservation.measurement
        };
        if !claimed && (worker || measurement) {
            require_capacity(&run, worker)?;
        }
        require_dispatch(root, &mut run, kind, false, Some(task), worker)?;
        let reservation = run.reservations.get_mut(request).unwrap();
        reservation.worker = worker;
        reservation.measurement = measurement;
        save(root, &run)?;
        return Ok(Some(Binding {
            run: run.id,
            request: request.to_owned(),
        }));
    }
    let own_claim = run.reservations.get(request).is_some_and(|reservation| {
        reservation.task.as_deref() == Some(task)
            && reservation.kind == kind
            && !reservation.bound
            && !reservation.settled
            && worker
            && reservation.worker
    });
    if (worker || measurement) && !own_claim {
        require_capacity(&run, worker)?;
    }
    let new_work = !run.reservations.contains_key(request);
    require_dispatch(root, &mut run, kind, new_work, Some(task), worker)?;
    if provided.is_some() {
        let reservation = run
            .reservations
            .get_mut(request)
            .ok_or_else(|| ZdevError::new("Reserve the supplied work identity first"))?;
        if reservation.task.as_deref() != Some(task)
            || reservation.bound
            || reservation.settled
            || reservation.kind == WorkKind::Supporting
            || (!worker && reservation.requested_worker_slot)
            || (worker && reservation.kind != kind)
        {
            return Err(ZdevError::new(
                "Reservation is already bound, settled or belongs to different work",
            ));
        }
        reservation.bound = true;
        save(root, &run)?;
    } else {
        if run.reservations.contains_key(request) {
            return Err(ZdevError::new(
                "Work was already reserved; recover its checkpoint without redispatch",
            ));
        }
        reserve(root, &mut run, request, Some(task), kind, true, false)?;
    }
    let reservation = run.reservations.get_mut(request).unwrap();
    reservation.worker = worker;
    reservation.measurement = measurement;
    save(root, &run)?;
    Ok(Some(Binding {
        run: run.id,
        request: request.to_owned(),
    }))
}

/// Reconciliation has already counted source refresh; measurement capacity starts with its actual attempt.
pub(crate) fn claim_measurement(
    root: &Path,
    area: &str,
    binding: &Binding,
) -> Result<(), ZdevError> {
    let mut run = authorized(root, area, &binding.run)?;
    let reservation = run
        .reservations
        .get(&binding.request)
        .ok_or_else(|| ZdevError::new("Missing reconciliation reservation"))?;
    if !reservation.bound
        || reservation.settled
        || reservation.kind != WorkKind::Reconciliation
        || reservation.commands != 0
    {
        return Err(ZdevError::new(
            "Reconciliation measurement is already consumed or unresolved",
        ));
    }
    if !reservation.measurement {
        require_capacity(&run, false)?;
    }
    require_dispatch(root, &mut run, WorkKind::Rerun, false, None, false)?;
    run.reservations
        .get_mut(&binding.request)
        .unwrap()
        .measurement = true;
    save(root, &run)
}

pub(crate) fn settle(
    root: &Path,
    area: &str,
    binding: &Binding,
    elapsed: f64,
    complete: bool,
    known_commands: usize,
    invocation_pending: bool,
) -> Result<(), ZdevError> {
    let mut run = authorized(root, area, &binding.run)?;
    let reservation = run
        .reservations
        .get_mut(&binding.request)
        .ok_or_else(|| ZdevError::new("Missing bound work reservation"))?;
    // The checkpoint owns cumulative elapsed cost; repeated recover/finalize never charges it twice.
    reservation.elapsed_seconds = reservation.elapsed_seconds.max(elapsed);
    if !invocation_pending {
        if known_commands >= reservation.commands {
            reservation.pending_timeout = None;
        } else if complete {
            // A crash before checkpoint publication leaves unknown cost; charge the reserved ceiling.
            reservation.elapsed_seconds += reservation.pending_timeout.take().unwrap_or(0.0);
        }
    }
    reservation.settled |= complete;
    if complete {
        reservation.worker = false;
        reservation.measurement = false;
    }
    save(root, &run)
}

pub(crate) fn reserve_command(
    root: &Path,
    area: &str,
    binding: &Binding,
    known_commands: usize,
    timeout: f64,
    comparison: bool,
) -> Result<f64, ZdevError> {
    let mut run = authorized(root, area, &binding.run)?;
    require_dispatch(
        root,
        &mut run,
        if comparison {
            WorkKind::Rerun
        } else {
            WorkKind::Rebaseline
        },
        false,
        None,
        false,
    )?;
    let reservation = run
        .reservations
        .get(&binding.request)
        .ok_or_else(|| ZdevError::new("Missing bound work reservation"))?;
    if reservation.settled || reservation.commands != known_commands {
        return Err(ZdevError::new(
            "Unresolved reserved invocation; recover without relaunching",
        ));
    }
    let mut timeout = timeout;
    if let Some(deadline) = run.deadline {
        timeout = timeout.min(deadline - unix_seconds());
    }
    if let Some(limit) = run.command_seconds {
        timeout = timeout.min(limit - elapsed_cost(&run));
    }
    if timeout <= 0.0 {
        return Err(ZdevError::new("Research run stopped: budget"));
    }
    let reservation = run.reservations.get_mut(&binding.request).unwrap();
    reservation.commands += 1;
    reservation.pending_timeout = Some(timeout);
    save(root, &run)?;
    Ok(timeout)
}

pub(crate) fn comparison_valid(root: &Path, area: &str) -> Result<bool, ZdevError> {
    let Some(run) = load(root, area)?.filter(|run| !run.ended) else {
        return Ok(true);
    };
    let current = super::settings(root, area)?;
    Ok(run.comparison_valid
        && current.eq_ignoring_advisory(&run.policy)
        && accepted_code_matches(root, &current)?)
}

pub(crate) fn accepted(root: &Path, area: &str, settings: &Settings) -> Result<(), ZdevError> {
    if let Some(mut run) = load(root, area)?.filter(|run| !run.ended) {
        if !policy_matches(&run.policy, settings) {
            return Err(ZdevError::new("Acceptance changed authorized policy"));
        }
        run.policy = settings.clone();
        save(root, &run)?;
    }
    Ok(())
}

fn checkpoints(root: &Path, area: &str) -> Result<Vec<serde_json::Value>, ZdevError> {
    let directory = path(root, area)?
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("experiments")
        .join(area);
    let mut values: Vec<serde_json::Value> = Vec::new();
    match fs::read_dir(directory) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry
                    .map_err(|error| ZdevError::io("Cannot read experiment directory", error))?;
                if entry
                    .path()
                    .extension()
                    .is_some_and(|extension| extension == "json")
                {
                    values.push(super::read_json(&entry.path())?);
                }
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(ZdevError::io("Cannot inspect experiments", error)),
    }
    values.sort_by(|a, b| a["task"].as_str().cmp(&b["task"].as_str()));
    Ok(values)
}

fn serial_capacity() -> usize {
    1
}

fn require_capacity(run: &Run, worker: bool) -> Result<(), ZdevError> {
    let used = run
        .reservations
        .values()
        .filter(|reservation| {
            !reservation.settled
                && if worker {
                    reservation.worker
                } else {
                    reservation.measurement
                }
        })
        .count();
    let limit = if worker {
        run.worker_limit
    } else {
        run.measurement_limit
    };
    if used >= limit {
        return Err(ZdevError::new(if worker {
            "Worker capacity is occupied; retain unresolved workers"
        } else {
            "Measurement capacity is occupied; no invocation was launched"
        }));
    }
    Ok(())
}

fn live_isolated_measurement(root: &Path, run: &Run, request: &str) -> bool {
    checkpoints(root, &run.area).is_ok_and(|checkpoints| {
        checkpoints.iter().any(|checkpoint| {
            !checkpoint["isolated"].is_null()
                && checkpoint["attempts"].as_array().is_some_and(|attempts| {
                    attempts.iter().any(|attempt| {
                        attempt["reservation"]["request"].as_str() == Some(request)
                            && attempt["measuring"] == true
                            && attempt["owner_pid"]
                                .as_u64()
                                .is_some_and(|pid| super::experiment::process_running(pid as u32))
                            && attempt["running_pid"].as_u64().is_some_and(|pid| {
                                super::experiment::process_group_running(pid as u32)
                            })
                    })
                })
        })
    })
}

pub(crate) fn require_no_pending_withdrawal(root: &Path, area: &str) -> Result<(), ZdevError> {
    if checkpoints(root, area)?
        .iter()
        .any(|checkpoint| checkpoint["isolated"]["withdrawing"] == true)
    {
        return Err(ZdevError::new(
            "Recover candidate withdrawal before dispatch or supporting completion",
        ));
    }
    Ok(())
}

pub(crate) fn require_acceptance_owner(
    root: &Path,
    area: &str,
    task: &str,
) -> Result<(), ZdevError> {
    let run = load(root, area)?
        .filter(|run| !run.ended)
        .ok_or_else(|| ZdevError::new("No active run owns this acceptance"))?;
    if run.acceptance.as_deref() != Some(task) {
        return Err(ZdevError::new("Candidate does not own the acceptance lane"));
    }
    Ok(())
}

/// The authoritative lock serializes the single destination acceptance lane.
pub(crate) fn acceptance(
    root: &Path,
    area: &str,
    task: &str,
    claim: bool,
) -> Result<(), ZdevError> {
    let mut run = load(root, area)?
        .filter(|run| !run.ended)
        .ok_or_else(|| ZdevError::new("Isolated candidates require an active run"))?;
    if run.acceptance.as_deref().is_some_and(|owner| owner != task) {
        return Err(ZdevError::new(
            "Another candidate owns the acceptance lane; recover it first",
        ));
    }
    if claim {
        require_no_pending_withdrawal(root, area)?;
        if run
            .reservations
            .values()
            .any(|reservation| !reservation.settled && reservation.measurement)
        {
            return Err(ZdevError::new(
                "Settle measurements before destination integration",
            ));
        }
        run.acceptance = Some(task.to_owned());
    } else if run.acceptance.as_deref() == Some(task) {
        run.acceptance = None;
    }
    save(root, &run)
}

fn context(root: &Path, area: &str) -> Result<CommandOutput, ZdevError> {
    let mut run = load(root, area)?.ok_or_else(|| ZdevError::new("Area has no research run"))?;
    let settings = super::settings(root, area)?;
    let valid = run.comparison_valid
        && settings.eq_ignoring_advisory(&run.policy)
        && accepted_code_matches(root, &settings)?;
    let unreachable_gain = settings.unreachable_gain()?;
    run.comparison_valid = valid;
    let projection = serde_json::to_value(&run).unwrap();
    let stop = reason(&run, &settings, true)?;
    let tasks = tasks::list(root, Some(area))?.value;
    let summary = tasks::summary(root, area)?;
    let next = if stop == Some("blocker") {
        "admit and reserve supporting work to resolve the shared environment; measurements remain halted"
    } else if stop.is_some() {
        "resolve stop reason or record/recover already authorized work"
    } else if !valid {
        "resolve comparison conditions and run a counted actual rebaseline"
    } else if unreachable_gain.is_some() {
        "ask the user to decide the study: the frozen minimum gain is unreachable from the accepted baseline; keep the frozen study, end it, or declare a new study with its own threshold and measured baseline"
    } else if matches!(summary.queue(), tasks::QueueState::Blocked) {
        "no work: every open task waits on another area; stop until the cross-area blockers are done there"
    } else if summary.ready == 0 {
        "generate and admit more in-scope supporting work or hypotheses"
    } else {
        "reserve and dispatch ready authorized work"
    };
    let now = unix_seconds();
    let mut value = json!({"schema_version": 1, "area": area, "run": projection, "stop_reason": stop,
        "accepted_baseline": settings.comparison, "tasks": tasks, "checkpoints": checkpoints(root, area)?,
        "consumed": {"work_attempts": run.reservations.len(), "commands": command_count(&run), "command_seconds": elapsed_cost(&run)},
        "remaining": {"work_attempts": run.attempt_limit.map(|limit| limit.saturating_sub(run.reservations.len())),
            "commands": run.command_limit.map(|limit| limit.saturating_sub(command_count(&run))),
            "command_seconds": run.command_seconds.map(|limit| (limit - elapsed_cost(&run)).max(0.0)),
            "wall_seconds": run.deadline.map(|deadline| (deadline - now).max(0.0)),
            "workers": run.worker_limit.saturating_sub(run.reservations.values().filter(|r| !r.settled && r.worker).count()),
            "measurements": run.measurement_limit.saturating_sub(run.reservations.values().filter(|r| !r.settled && r.measurement).count())},
        "elapsed_wall_seconds": (now - run.started_at).max(0.0),
        "unreachable_minimum_gain": unreachable_gain, "next_action": next});
    tasks::insert_cross_area_waits(&mut value, &summary.cross_area_waits);
    Ok(CommandOutput::new(
        serde_json::to_string_pretty(&value).unwrap(),
        value,
    ))
}

fn unix_seconds() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

fn rebaseline(
    root: &Path,
    area: &str,
    identity: &str,
    request: &str,
    untracked_outputs: &[String],
) -> Result<(), ZdevError> {
    let mut exclusions = Vec::new();
    for output in untracked_outputs {
        let path = Path::new(output);
        if output.is_empty()
            || !path
                .components()
                .all(|part| matches!(part, std::path::Component::Normal(_)))
            || path.components().next().is_some_and(|part| {
                part.as_os_str().eq_ignore_ascii_case(".git")
                    || part.as_os_str().eq_ignore_ascii_case(".zdev")
            })
        {
            return Err(ZdevError::new(
                "Untracked output must be a non-empty repository-relative path outside .git and .zdev",
            ));
        }
        exclusions.push(format!(":(top,literal,exclude){output}"));
    }
    let mut untracked_args = vec![
        "ls-files",
        "--others",
        "--exclude-standard",
        "-z",
        "--",
        ".",
        ":(exclude).zdev",
    ];
    untracked_args.extend(exclusions.iter().map(String::as_str));
    {
        let _lock = ZdevStateLock::acquire(root)?;
        let mut run = authorized(root, area, identity)?;
        require_area(root, area)?;
        if let Some(baseline) = &run.rebaseline {
            if baseline.request == request {
                if baseline.complete {
                    return publish_baseline(root, &mut run);
                }
                if run.reservations[request].settled {
                    return Err(ZdevError::new(
                        "This baseline request already failed; resolve its blocker and use a new counted request",
                    ));
                }
                if super::experiment::process_running(baseline.owner_pid)
                    || baseline
                        .running_pid
                        .is_some_and(super::experiment::process_group_running)
                {
                    return Err(ZdevError::new(
                        "Baseline process is unresolved or still running; no command was relaunched",
                    ));
                }
                let elapsed = baseline
                    .invocation_started
                    .map(|started| (unix_seconds() - started).max(0.0))
                    .unwrap_or(0.0);
                let reservation = run.reservations.get_mut(request).unwrap();
                reservation.elapsed_seconds += if baseline.commands_started < reservation.commands {
                    reservation.pending_timeout.unwrap_or(0.0)
                } else {
                    elapsed
                };
                reservation.pending_timeout = None;
                reservation.settled = true;
                run.rebaseline.as_mut().unwrap().invocation_started = None;
                run.rebaseline.as_mut().unwrap().running_pid = None;
                if reason(&run, &super::settings(root, area)?, true)?.is_none() {
                    run.stop = Some(Stop::Interruption);
                }
                invalidate(
                    &mut run,
                    "Baseline measurement interrupted; reserved costs retained. Resume and use a new counted request.",
                );
                save(root, &run)?;
                return Err(ZdevError::new(
                    "Interrupted baseline retained; resume the same run and use a new counted rebaseline",
                ));
            }
            if !baseline.complete && !run.reservations[&baseline.request].settled {
                return Err(ZdevError::new(
                    "Recover the previous baseline request before starting another",
                ));
            }
        }
        if run.reservations.contains_key(request) {
            return Err(ZdevError::new(
                "Rebaseline request already consumed; use a new work identity",
            ));
        }
        // The baseline measures accepted HEAD, never a dirty candidate or supplied replacement samples.
        if !git_output_bytes(
            root,
            &[
                "diff",
                "HEAD",
                "--name-only",
                "-z",
                "--",
                ".",
                ":(exclude).zdev",
            ],
        )?
        .is_empty()
            || !git_output_bytes(root, &untracked_args)?.is_empty()
        {
            return Err(ZdevError::new(
                "Actual rebaseline requires clean accepted code; retain or restore unfinished candidates first",
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
        reserve(
            root,
            &mut run,
            request,
            None,
            WorkKind::Rebaseline,
            true,
            false,
        )?;
        invalidate(&mut run, "Actual baseline measurement is unfinished");
        run.rebaseline = Some(Baseline {
            request: request.to_owned(),
            untracked_outputs: untracked_outputs.to_vec(),
            head: git_output(root, &["rev-parse", "HEAD"])?,
            settings,
            samples: Vec::new(),
            owner_pid: std::process::id(),
            running_pid: None,
            invocation_started: None,
            complete: false,
            commands_started: 0,
            published: false,
        });
        save(root, &run)?;
    }
    let settings = load(root, area)?.unwrap().rebaseline.unwrap().settings;
    let commands = settings
        .comparison
        .required_checks
        .iter()
        .map(|name| {
            (
                format!(
                    "( {} ) >/dev/null && printf 1",
                    settings.correctness_commands[name]
                ),
                false,
            )
        })
        .chain(
            (0..settings.comparison.repetitions)
                .map(|_| (settings.evaluator_command.clone(), true)),
        )
        .collect::<Vec<_>>();
    for (command, sample) in commands {
        let (binding, repetition, timeout) = {
            let _lock = ZdevStateLock::acquire(root)?;
            let mut run = authorized(root, area, identity)?;
            let binding = Binding {
                run: run.id.clone(),
                request: request.to_owned(),
            };
            let known = run.rebaseline.as_ref().unwrap().commands_started;
            let timeout =
                reserve_command(root, area, &binding, known, settings.timeout_seconds, false)?;
            run = authorized(root, area, identity)?;
            run.rebaseline.as_mut().unwrap().invocation_started = Some(unix_seconds());
            run.rebaseline.as_mut().unwrap().commands_started = known + 1;
            save(root, &run)?;
            let repetition = if sample {
                run.rebaseline.as_ref().unwrap().samples.len() + 1
            } else {
                known + 1
            };
            (binding, repetition, timeout)
        };
        let started = std::time::Instant::now();
        let result = super::evaluate_with_started(root, &command, timeout, repetition, |pid| {
            let _lock = ZdevStateLock::acquire(root)?;
            let mut run = authorized(root, area, identity)?;
            run.rebaseline.as_mut().unwrap().running_pid = Some(pid);
            save(root, &run)
        });
        let _lock = ZdevStateLock::acquire(root)?;
        let mut run = authorized(root, area, identity)?;
        run.reservations
            .get_mut(&binding.request)
            .unwrap()
            .elapsed_seconds += started.elapsed().as_secs_f64();
        run.reservations
            .get_mut(&binding.request)
            .unwrap()
            .pending_timeout = None;
        let baseline = run.rebaseline.as_mut().unwrap();
        baseline.running_pid = None;
        baseline.invocation_started = None;
        match result {
            Ok(value) if sample => baseline.samples.push(value),
            Ok(_) => {}
            Err((_, failure)) => {
                run.reservations.get_mut(request).unwrap().settled = true;
                run.stop = Some(Stop::Blocker {
                    description: format!("Accepted baseline command failed: {failure}"),
                });
                save(root, &run)?;
                return Err(ZdevError::new(
                    "Accepted baseline failed; shared environment blocker and consumed costs retained",
                ));
            }
        }
        save(root, &run)?;
    }
    let _lock = ZdevStateLock::acquire(root)?;
    let mut run = authorized(root, area, identity)?;
    let baseline = run.rebaseline.as_ref().unwrap();
    let current = super::settings(root, area)?;
    if baseline.head != git_output(root, &["rev-parse", "HEAD"])?
        || !current.eq_ignoring_advisory(&baseline.settings)
        || !git_output_bytes(
            root,
            &[
                "diff",
                "HEAD",
                "--name-only",
                "-z",
                "--",
                ".",
                ":(exclude).zdev",
            ],
        )?
        .is_empty()
        || !git_output_bytes(root, &untracked_args)?.is_empty()
    {
        run.reservations.get_mut(request).unwrap().settled = true;
        save(root, &run)?;
        return Err(ZdevError::new(
            "Accepted code or policy changed during rebaseline; comparison remains invalid",
        ));
    }
    let mut settings = baseline.settings.clone();
    settings.comparison.baseline_revision = baseline.head.clone();
    settings.comparison.baseline_samples = baseline.samples.clone();
    settings.metric_bound = current.metric_bound;
    settings.validate()?;
    // Persist expected settings first: retry can recover a crash before area publication.
    run.policy = settings.clone();
    run.rebaseline.as_mut().unwrap().complete = true;
    run.reservations.get_mut(request).unwrap().settled = true;
    run.comparison_valid = true;
    run.invalidation = None;
    save(root, &run)?;
    publish_baseline(root, &mut run)
}

fn publish_baseline(root: &Path, run: &mut Run) -> Result<(), ZdevError> {
    let baseline = run.rebaseline.as_ref().unwrap();
    if baseline.published {
        return Ok(());
    }
    if !accepted_code_matches(root, &run.policy)? {
        return Err(ZdevError::new(
            "Accepted code changed before baseline publication; measure its new baseline",
        ));
    }
    let (mut metadata, _) = crate::project::load_area(root, &run.area)?;
    let mut published = run.policy.clone();
    match &metadata.research {
        Some(current)
            if current.eq_ignoring_advisory(&baseline.settings)
                || current.eq_ignoring_advisory(&run.policy) =>
        {
            published.metric_bound = current.metric_bound;
        }
        _ => {
            return Err(ZdevError::new(
                "Research policy changed during baseline publication recovery",
            ));
        }
    }
    metadata.research = Some(published);
    crate::project::write_area_metadata(root, &metadata)?;
    run.rebaseline.as_mut().unwrap().published = true;
    save(root, run)
}

pub(crate) fn require_supporting_reservation(
    root: &Path,
    area: &str,
    task: &str,
) -> Result<(), ZdevError> {
    let Some(run) = load(root, area)?.filter(|run| !run.ended) else {
        return Ok(());
    };
    require_no_pending_withdrawal(root, area)?;
    if run.acceptance.is_some() {
        return Err(ZdevError::new(
            "Finish the acceptance lane before supporting completion",
        ));
    }
    if run
        .admissions
        .values()
        .any(|admission| admission.tasks.iter().any(|id| id == task))
        && !run.reservations.values().any(|reservation| {
            reservation.task.as_deref() == Some(task) && reservation.kind == WorkKind::Supporting
        })
    {
        return Err(ZdevError::new(
            "Generated supporting work requires a counted reservation before dispatch",
        ));
    }
    Ok(())
}

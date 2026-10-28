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
        target: Option<f64>,
        #[command(flatten)]
        limits: RequestedLimits,
    },
    /// Inspect current policy, task results, unfinished checkpoints and live remaining limits
    Context,
    /// Read a compact routine view of the run; `--full` reads the same projection as context
    Inspect {
        /// Return the complete context projection
        #[arg(long)]
        full: bool,
    },
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
        /// Generated output path to preserve outside the clean-code check, in addition to the
        /// declared `generated_outputs`
        #[arg(long = "untracked-output", value_name = "PATH")]
        untracked_outputs: Vec<String>,
    },
    /// End this authorization once reservations and experiment checkpoints are settled
    End {
        #[arg(long)]
        run: String,
    },
}

/// The limits `start` accepts; `--estimated` marks supplied values the coordinator chose itself.
#[derive(Debug, clap::Args)]
pub(crate) struct RequestedLimits {
    /// Coordinator work dispatches; defaults to 20 unless the run has a time limit or runs until
    /// interrupted
    #[arg(long)]
    attempt_limit: Option<usize>,
    #[arg(long)]
    command_limit: Option<usize>,
    #[arg(long)]
    command_seconds: Option<f64>,
    #[arg(long)]
    time_limit_seconds: Option<f64>,
    /// Native worker capacity; defaults to one
    #[arg(long)]
    worker_limit: Option<usize>,
    /// Concurrent measurement capacity; defaults to one
    #[arg(long)]
    measurement_limit: Option<usize>,
    /// Record a supplied limit as the coordinator's own estimate rather than the user's constraint
    #[arg(long = "estimated", value_enum, value_name = "LIMIT")]
    estimated: Vec<Limit>,
}

impl RequestedLimits {
    fn supplied(&self, limit: Limit) -> bool {
        match limit {
            Limit::AttemptLimit => self.attempt_limit.is_some(),
            Limit::CommandLimit => self.command_limit.is_some(),
            Limit::CommandSeconds => self.command_seconds.is_some(),
            Limit::TimeLimitSeconds => self.time_limit_seconds.is_some(),
            Limit::WorkerLimit => self.worker_limit.is_some(),
            Limit::MeasurementLimit => self.measurement_limit.is_some(),
        }
    }

    /// The origin of a limit the run has: a supplied value is the user's unless marked as the
    /// coordinator's estimate, and any other value is a product default.
    fn origin(&self, limit: Limit) -> LimitOrigin {
        if !self.supplied(limit) {
            LimitOrigin::Default
        } else if self.estimated.contains(&limit) {
            LimitOrigin::CoordinatorEstimate
        } else {
            LimitOrigin::User
        }
    }
}

/// A run limit, named after its `start` flag. The directional target is a goal, not a limit.
// Variant names spell the flags (`attempt-limit`) and stored keys (`attempt_limit`) exactly.
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
enum Limit {
    AttemptLimit,
    CommandLimit,
    CommandSeconds,
    TimeLimitSeconds,
    WorkerLimit,
    MeasurementLimit,
}

/// Who set a limit. Extensions keep the origin of the limit they raise.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum LimitOrigin {
    User,
    Default,
    CoordinatorEstimate,
    /// Reported for runs stored before origins were recorded; never stored.
    #[serde(skip_deserializing)]
    Unrecorded,
}

impl LimitOrigin {
    fn describe(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Default => "default",
            Self::CoordinatorEstimate => "coordinator estimate",
            Self::Unrecorded => "unrecorded",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize, ValueEnum)]
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
    /// One entry per limit the run has; runs stored before origins existed have none.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    limit_origins: BTreeMap<Limit, LimitOrigin>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    amendments: Vec<Amendment>,
}

impl Run {
    fn has_limit(&self, limit: Limit) -> bool {
        match limit {
            Limit::AttemptLimit => self.attempt_limit.is_some(),
            Limit::CommandLimit => self.command_limit.is_some(),
            Limit::CommandSeconds => self.command_seconds.is_some(),
            Limit::TimeLimitSeconds => self.deadline.is_some(),
            Limit::WorkerLimit | Limit::MeasurementLimit => true,
        }
    }

    fn origin(&self, limit: Limit) -> LimitOrigin {
        self.limit_origins
            .get(&limit)
            .copied()
            .unwrap_or(LimitOrigin::Unrecorded)
    }
}

/// A declared administrative change to existing correctness command text, in adoption order.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Amendment {
    reason: String,
    commands: BTreeMap<String, CommandChange>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CommandChange {
    before: String,
    after: String,
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
    /// Live measurement capacity claim; settlement releases it.
    #[serde(default)]
    measurement: bool,
    /// Whether this reservation was ever claimed as a measurement; never cleared.
    #[serde(default)]
    measured: bool,
}

impl Reservation {
    /// Sets the live measurement claim and remembers durably when one was made.
    fn set_measurement(&mut self, measurement: bool) {
        self.measurement = measurement;
        self.measured |= measurement;
    }

    /// Whether this dispatch measured a candidate. Reservations stored before `measured` existed
    /// fall back to having run a command; in newer runs only measurements run commands.
    fn is_experiment(&self) -> bool {
        self.kind != WorkKind::Rebaseline && (self.measured || self.commands > 0)
    }
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
    } else if !matches!(action, Action::Context | Action::Inspect { .. }) {
        let _lock = ZdevStateLock::acquire(root)?;
        match action {
            Action::Start {
                scope,
                until_interrupted,
                target,
                limits,
            } => start(root, area, scope, *until_interrupted, *target, limits)?,
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
                    || has_unfinished_checkpoint(root, area)?
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
            Action::Context | Action::Inspect { .. } | Action::Rebaseline { .. } => unreachable!(),
        }
    }
    if let Action::Inspect { full: false } = action {
        return inspection(root, area);
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
        || (!run.until_interrupted && run.attempt_limit.is_none() && run.deadline.is_none())
        || run.worker_limit == 0
        || run.measurement_limit == 0
        || run.attempt_limit == Some(0)
        || run.command_limit == Some(0)
        || [run.command_seconds]
            .into_iter()
            .flatten()
            .any(|value| !value.is_finite() || value <= 0.0)
        || run.target.is_some_and(|value| !value.is_finite())
        || run.limit_origins.keys().any(|limit| !run.has_limit(*limit))
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
    for amendment in &run.amendments {
        crate::validate_nonempty_line(&amendment.reason, "Administrative amendment reason")?;
        if amendment.commands.is_empty() {
            return Err(ZdevError::new(
                "Research run amendment records no command change",
            ));
        }
        for (name, change) in &amendment.commands {
            crate::validate_nonempty_line(name, "Correctness command name")?;
            crate::validate_nonempty_line(&change.before, "Correctness command")?;
            crate::validate_nonempty_line(&change.after, "Correctness command")?;
            if change.before == change.after {
                return Err(ZdevError::new(
                    "Research run amendment records an unchanged command",
                ));
            }
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

fn start(
    root: &Path,
    area: &str,
    scope: &str,
    until_interrupted: bool,
    target: Option<f64>,
    limits: &RequestedLimits,
) -> Result<(), ZdevError> {
    require_area(root, area)?;
    crate::validate_nonempty_line(scope, "Authorized research scope")?;
    for value in [limits.command_seconds, limits.time_limit_seconds]
        .into_iter()
        .flatten()
    {
        if !value.is_finite() || value <= 0.0 {
            return Err(ZdevError::new(
                "Research resource limits must be finite and positive",
            ));
        }
    }
    if [
        limits.attempt_limit,
        limits.command_limit,
        limits.worker_limit,
        limits.measurement_limit,
    ]
    .contains(&Some(0))
        || target.is_some_and(|value| !value.is_finite())
    {
        return Err(ZdevError::new(
            "Count limits must be positive and target must be finite",
        ));
    }
    if let Some(limit) = limits
        .estimated
        .iter()
        .find(|limit| !limits.supplied(**limit))
    {
        return Err(ZdevError::new(format!(
            "--estimated {} names a limit that was not supplied",
            limit.to_possible_value().unwrap().get_name()
        )));
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
    if has_unfinished_checkpoint(root, area)? {
        return Err(ZdevError::new(
            "Recover unfinished experiments before authorizing another run",
        ));
    }
    let policy = super::settings(root, area)?;
    let invalidation = input_invalidation(&changed_relevant_paths(root, &policy, "HEAD")?);
    let started_at = unix_seconds();
    // A time limit or an explicit until-interrupted request replaces the default dispatch cap.
    let attempt_limit = limits
        .attempt_limit
        .or((!until_interrupted && limits.time_limit_seconds.is_none())
            .then_some(DEFAULT_ATTEMPT_LIMIT));
    let mut run = Run {
        schema_version: 1,
        area: area.to_owned(),
        id: crate::generate_change_id()?.replacen('Z', "run-", 1),
        scope: scope.to_owned(),
        until_interrupted,
        policy,
        started_at,
        deadline: limits
            .time_limit_seconds
            .map(|seconds| started_at + seconds),
        attempt_limit,
        command_limit: limits.command_limit,
        command_seconds: limits.command_seconds,
        target,
        worker_limit: limits.worker_limit.unwrap_or_else(serial_capacity),
        measurement_limit: limits.measurement_limit.unwrap_or_else(serial_capacity),
        limit_origins: BTreeMap::new(),
        acceptance: None,
        stop: None,
        comparison_valid: invalidation.is_none(),
        invalidation,
        ended: false,
        admissions: BTreeMap::new(),
        reservations: BTreeMap::new(),
        rebaseline: None,
        extensions: BTreeMap::new(),
        amendments: Vec::new(),
    };
    run.limit_origins = Limit::value_variants()
        .iter()
        .copied()
        .filter(|limit| run.has_limit(*limit))
        .map(|limit| (limit, limits.origin(limit)))
        .collect();
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
    Ok(changed_relevant_paths(root, settings, "HEAD")?.is_empty())
}

/// Paths inside the settings' input boundary that differ between the accepted baseline and `head`.
pub(super) fn changed_relevant_paths(
    root: &Path,
    settings: &Settings,
    head: &str,
) -> Result<Vec<String>, ZdevError> {
    let pathspecs = settings.input_boundary().pathspecs();
    let mut arguments = vec![
        "diff",
        "--name-only",
        "-z",
        &settings.comparison.baseline_revision,
        head,
        "--",
    ];
    arguments.extend(pathspecs.iter().map(String::as_str));
    Ok(git_output_bytes(root, &arguments)?
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .map(|path| String::from_utf8_lossy(path).into_owned())
        .collect())
}

/// The invalidation reason naming the changed relevant inputs, or `None` when nothing changed.
fn input_invalidation(changed: &[String]) -> Option<String> {
    let (first, others) = changed.split_first()?;
    let more = if others.is_empty() {
        String::new()
    } else {
        format!(" (and {} more)", others.len())
    };
    Some(format!(
        "Accepted relevant input changed since the baseline: {first}{more}; actual rebaseline required"
    ))
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
        && frozen.relevant_inputs == current.relevant_inputs
}

/// Changed text of existing correctness commands, or `None` when anything else in the frozen
/// policy differs. Only the advisory metric bound may differ besides those commands.
fn command_changes(
    frozen: &Settings,
    current: &Settings,
) -> Option<BTreeMap<String, CommandChange>> {
    let with_frozen_commands = Settings {
        correctness_commands: frozen.correctness_commands.clone(),
        ..current.clone()
    };
    if !with_frozen_commands.eq_ignoring_advisory(frozen)
        || !frozen
            .correctness_commands
            .keys()
            .eq(current.correctness_commands.keys())
    {
        return None;
    }
    Some(
        frozen
            .correctness_commands
            .iter()
            .filter_map(|(name, before)| {
                let after = &current.correctness_commands[name];
                (before != after).then(|| {
                    let change = CommandChange {
                        before: before.clone(),
                        after: after.clone(),
                    };
                    (name.clone(), change)
                })
            })
            .collect(),
    )
}

/// Caller holds the state lock. Configuration cannot clear invalidation with supplied samples.
/// A declared administrative amendment changes only correctness command text and keeps the
/// run's comparison state; every other change follows the frozen-policy rules below.
pub(crate) fn configure(
    root: &Path,
    area: &str,
    previous: &Settings,
    current: &Settings,
    administrative: Option<&str>,
) -> Result<(), ZdevError> {
    let Some(mut run) = load(root, area)?.filter(|run| !run.ended) else {
        return Ok(());
    };
    if let Some(reason) = administrative {
        return amend(root, &mut run, current, reason);
    }
    if !policy_matches(&run.policy, current) {
        return Err(ZdevError::new(
            "Active run freezes its metric, direction, scope, relevant inputs and correctness/acceptance requirements",
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

/// Adopts changed correctness command text as the run's policy without invalidating it.
/// Nothing executes here; the next evaluate or rebaseline runs the amended commands.
fn amend(root: &Path, run: &mut Run, current: &Settings, reason: &str) -> Result<(), ZdevError> {
    let commands = command_changes(&run.policy, current).ok_or_else(|| {
        ZdevError::new(
            "An administrative amendment may change only the command text of existing correctness checks; a material change needs a new study and an actual measured baseline",
        )
    })?;
    // The caller saves the run before the area settings; a retry after a crash between the two
    // finds the commands already adopted and only completes the area write.
    if commands.is_empty() {
        return Ok(());
    }
    if has_unfinished_work(root, run)? {
        return Err(ZdevError::new(
            "Finish unfinished experiments, measurements and rebaselines before an administrative amendment",
        ));
    }
    run.amendments.push(Amendment {
        reason: reason.to_owned(),
        commands,
    });
    run.policy = current.clone();
    save(root, run)
}

/// Whether work that captured or measures the current policy is still open.
fn has_unfinished_work(root: &Path, run: &Run) -> Result<bool, ZdevError> {
    Ok(has_unfinished_checkpoint(root, &run.area)?
        || run.acceptance.is_some()
        || run.reservations.values().any(|reservation| {
            !reservation.settled
                && (reservation.measurement || reservation.kind == WorkKind::Rebaseline)
        })
        || run
            .rebaseline
            .as_ref()
            .is_some_and(|baseline| baseline.complete && !baseline.published))
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
    let invalidation = if settings.eq_ignoring_advisory(&run.policy) {
        input_invalidation(&changed_relevant_paths(root, &settings, "HEAD")?)
    } else {
        Some("Evaluator or workload settings changed; actual rebaseline required".to_owned())
    };
    if let Some(reason) = invalidation {
        invalidate(run, &reason);
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
    let measurement = kind == WorkKind::Rebaseline;
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
            measurement,
            measured: measurement,
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
        reservation.set_measurement(measurement);
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
    reservation.set_measurement(measurement);
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
        .set_measurement(true);
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

fn has_unfinished_checkpoint(root: &Path, area: &str) -> Result<bool, ZdevError> {
    Ok(checkpoints(root, area)?
        .iter()
        .any(|checkpoint| checkpoint["index_updated"] != true))
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

/// The run with its comparison validity checked against current settings and accepted code,
/// and the decisions that every run projection reports.
struct LiveRun {
    run: Run,
    settings: Settings,
    stop: Option<&'static str>,
    unreachable_gain: Option<super::UnreachableGain>,
    summary: tasks::AreaTaskSummary,
    next_action: &'static str,
    now: f64,
}

fn live_run(root: &Path, area: &str) -> Result<LiveRun, ZdevError> {
    let mut run = load(root, area)?.ok_or_else(|| ZdevError::new("Area has no research run"))?;
    let settings = super::settings(root, area)?;
    run.comparison_valid = run.comparison_valid
        && settings.eq_ignoring_advisory(&run.policy)
        && accepted_code_matches(root, &settings)?;
    let unreachable_gain = settings.unreachable_gain()?;
    let stop = reason(&run, &settings, true)?;
    let summary = tasks::summary(root, area)?;
    let next_action = next_action(
        stop,
        run.comparison_valid,
        unreachable_gain.is_some(),
        &summary,
    );
    Ok(LiveRun {
        run,
        settings,
        stop,
        unreachable_gain,
        summary,
        next_action,
        now: unix_seconds(),
    })
}

fn next_action(
    stop: Option<&str>,
    comparison_valid: bool,
    gain_unreachable: bool,
    summary: &tasks::AreaTaskSummary,
) -> &'static str {
    if stop == Some("blocker") {
        "admit and reserve supporting work to resolve the shared environment; measurements remain halted"
    } else if stop.is_some() {
        "resolve stop reason or record/recover already authorized work"
    } else if !comparison_valid {
        "resolve comparison conditions and run a counted actual rebaseline"
    } else if gain_unreachable {
        "ask the user to decide the study: the frozen minimum gain is unreachable from the accepted baseline; keep the frozen study, end it, or declare a new study with its own threshold and measured baseline"
    } else if matches!(summary.queue(), tasks::QueueState::Blocked) {
        "no work: every open task waits on another area; stop until the cross-area blockers are done there"
    } else if summary.ready == 0 {
        "generate and admit more in-scope supporting work or hypotheses"
    } else {
        "reserve and dispatch ready authorized work"
    }
}

/// Costs charged so far. Every dispatch is one work attempt; experiments are the dispatches that
/// measured a candidate, and the remaining dispatches support them.
#[derive(Serialize)]
struct Consumed {
    work_attempts: usize,
    /// No attempt limit applies, so the work-attempt count never stops dispatch.
    work_attempts_advisory: bool,
    dispatches_by_kind: BTreeMap<WorkKind, usize>,
    experiments: usize,
    commands: usize,
    command_seconds: f64,
}

impl Consumed {
    fn of(run: &Run) -> Self {
        let mut dispatches_by_kind: BTreeMap<WorkKind, usize> = WorkKind::value_variants()
            .iter()
            .map(|kind| (*kind, 0))
            .collect();
        for reservation in run.reservations.values() {
            *dispatches_by_kind.entry(reservation.kind).or_default() += 1;
        }
        Self {
            work_attempts: run.reservations.len(),
            work_attempts_advisory: run.attempt_limit.is_none(),
            dispatches_by_kind,
            experiments: run
                .reservations
                .values()
                .filter(|reservation| reservation.is_experiment())
                .count(),
            commands: command_count(run),
            command_seconds: elapsed_cost(run),
        }
    }
}

/// Each limit the run has, with its current value and origin; `None` means no such limit.
#[derive(Serialize)]
struct Limits {
    attempt_limit: Option<ReportedLimit<usize>>,
    command_limit: Option<ReportedLimit<usize>>,
    command_seconds: Option<ReportedLimit<f64>>,
    /// Seconds from the run's start to its current deadline, including extensions.
    time_limit_seconds: Option<ReportedLimit<f64>>,
    worker_limit: ReportedLimit<usize>,
    measurement_limit: ReportedLimit<usize>,
}

impl Limits {
    fn of(run: &Run) -> Self {
        Self {
            attempt_limit: run
                .attempt_limit
                .map(|value| ReportedLimit::of(run, Limit::AttemptLimit, value)),
            command_limit: run
                .command_limit
                .map(|value| ReportedLimit::of(run, Limit::CommandLimit, value)),
            command_seconds: run
                .command_seconds
                .map(|value| ReportedLimit::of(run, Limit::CommandSeconds, value)),
            time_limit_seconds: run.deadline.map(|deadline| {
                // Millisecond rounding removes the float error of storing an absolute deadline.
                let seconds = ((deadline - run.started_at) * 1000.0).round() / 1000.0;
                ReportedLimit::of(run, Limit::TimeLimitSeconds, seconds)
            }),
            worker_limit: ReportedLimit::of(run, Limit::WorkerLimit, run.worker_limit),
            measurement_limit: ReportedLimit::of(
                run,
                Limit::MeasurementLimit,
                run.measurement_limit,
            ),
        }
    }
}

#[derive(Serialize)]
struct ReportedLimit<T> {
    value: T,
    origin: LimitOrigin,
}

impl<T> ReportedLimit<T> {
    fn of(run: &Run, limit: Limit, value: T) -> Self {
        Self {
            value,
            origin: run.origin(limit),
        }
    }
}

impl<T: std::fmt::Display> ReportedLimit<T> {
    fn describe(&self, limit: Limit) -> String {
        format!(
            "{} {} ({})",
            limit.to_possible_value().unwrap().get_name(),
            self.value,
            self.origin.describe()
        )
    }
}

/// Allowance left under each limit; `None` means the run has no such limit.
#[derive(Serialize)]
struct Remaining {
    work_attempts: Option<usize>,
    commands: Option<usize>,
    command_seconds: Option<f64>,
    wall_seconds: Option<f64>,
    workers: usize,
    measurements: usize,
}

impl Remaining {
    fn of(run: &Run, now: f64) -> Self {
        let occupied = |claims: fn(&Reservation) -> bool| {
            run.reservations
                .values()
                .filter(|reservation| !reservation.settled && claims(reservation))
                .count()
        };
        Self {
            work_attempts: run
                .attempt_limit
                .map(|limit| limit.saturating_sub(run.reservations.len())),
            commands: run
                .command_limit
                .map(|limit| limit.saturating_sub(command_count(run))),
            command_seconds: run
                .command_seconds
                .map(|limit| (limit - elapsed_cost(run)).max(0.0)),
            wall_seconds: run.deadline.map(|deadline| (deadline - now).max(0.0)),
            workers: run
                .worker_limit
                .saturating_sub(occupied(|reservation| reservation.worker)),
            measurements: run
                .measurement_limit
                .saturating_sub(occupied(|reservation| reservation.measurement)),
        }
    }
}

/// The complete projection: the whole run, task list, every checkpoint and baseline samples.
fn context(root: &Path, area: &str) -> Result<CommandOutput, ZdevError> {
    let live = live_run(root, area)?;
    let run = &live.run;
    let tasks = tasks::list(root, Some(area))?.value;
    let mut value = json!({"schema_version": 1, "area": area, "run": run, "stop_reason": live.stop,
        "accepted_baseline": live.settings.comparison, "tasks": tasks, "checkpoints": checkpoints(root, area)?,
        "limits": Limits::of(run), "consumed": Consumed::of(run), "remaining": Remaining::of(run, live.now),
        "elapsed_wall_seconds": (live.now - run.started_at).max(0.0),
        "unreachable_minimum_gain": live.unreachable_gain, "next_action": live.next_action});
    tasks::insert_cross_area_waits(&mut value, &live.summary.cross_area_waits);
    Ok(CommandOutput::new(
        serde_json::to_string_pretty(&value).unwrap(),
        value,
    ))
}

/// The routine view: run state, limits, active work and the next decision. Its size depends
/// only on active work, never on recorded attempts, observations, samples or tasks.
#[derive(Serialize)]
struct Inspection<'a> {
    schema_version: u64,
    area: &'a str,
    run_id: &'a str,
    scope: &'a str,
    stop_reason: Option<&'static str>,
    blocker: Option<&'a str>,
    comparison_valid: bool,
    invalidation: Option<&'a str>,
    accepted_baseline: BaselineSummary<'a>,
    limits: Limits,
    consumed: Consumed,
    remaining: Remaining,
    elapsed_wall_seconds: f64,
    active_work: ActiveWork<'a>,
    ready_tasks: usize,
    blocked_tasks: usize,
    next_task: Option<&'a str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    cross_area_waits: Vec<tasks::CrossAreaWait>,
    unreachable_minimum_gain: Option<&'a super::UnreachableGain>,
    next_action: &'static str,
}

#[derive(Serialize)]
struct BaselineSummary<'a> {
    revision: &'a str,
    metric: &'a str,
    direction: Direction,
    aggregation: super::Aggregation,
    aggregate: f64,
}

/// Work that must be recovered or settled before the run can end.
#[derive(Serialize)]
struct ActiveWork<'a> {
    reservations: Vec<ActiveReservation<'a>>,
    checkpoints: Vec<String>,
    pending_admissions: Vec<&'a str>,
    acceptance: Option<&'a str>,
}

impl<'a> ActiveWork<'a> {
    fn of(run: &'a Run, unfinished_checkpoints: Vec<String>) -> Self {
        Self {
            reservations: run
                .reservations
                .iter()
                .filter(|(_, reservation)| !reservation.settled)
                .map(|(request, reservation)| ActiveReservation {
                    request,
                    task: reservation.task.as_deref(),
                    kind: reservation.kind,
                })
                .collect(),
            checkpoints: unfinished_checkpoints,
            pending_admissions: run
                .admissions
                .iter()
                .filter(|(_, admission)| !admission.pending.is_empty())
                .map(|(request, _)| request.as_str())
                .collect(),
            acceptance: run.acceptance.as_deref(),
        }
    }
}

#[derive(Serialize)]
struct ActiveReservation<'a> {
    request: &'a str,
    task: Option<&'a str>,
    kind: WorkKind,
}

fn inspection(root: &Path, area: &str) -> Result<CommandOutput, ZdevError> {
    let live = live_run(root, area)?;
    let run = &live.run;
    let comparison = &live.settings.comparison;
    let unfinished_checkpoints = checkpoints(root, area)?
        .into_iter()
        .filter(|checkpoint| checkpoint["index_updated"] != true)
        .filter_map(|checkpoint| checkpoint["task"].as_str().map(str::to_owned))
        .collect();
    let inspection = Inspection {
        schema_version: 1,
        area,
        run_id: &run.id,
        scope: &run.scope,
        stop_reason: live.stop,
        blocker: match &run.stop {
            Some(Stop::Blocker { description }) => Some(description),
            _ => None,
        },
        comparison_valid: run.comparison_valid,
        invalidation: run.invalidation.as_deref(),
        accepted_baseline: BaselineSummary {
            revision: &comparison.baseline_revision,
            metric: &comparison.metric,
            direction: comparison.direction,
            aggregation: comparison.aggregation,
            aggregate: comparison.aggregate(&comparison.baseline_samples)?,
        },
        limits: Limits::of(run),
        consumed: Consumed::of(run),
        remaining: Remaining::of(run, live.now),
        elapsed_wall_seconds: (live.now - run.started_at).max(0.0),
        active_work: ActiveWork::of(run, unfinished_checkpoints),
        ready_tasks: live.summary.ready,
        blocked_tasks: live.summary.blocked,
        next_task: live.summary.next.as_deref(),
        cross_area_waits: live.summary.cross_area_waits.clone(),
        unreachable_minimum_gain: live.unreachable_gain.as_ref(),
        next_action: live.next_action,
    };
    Ok(CommandOutput::new(
        render_inspection(&inspection),
        serde_json::to_value(&inspection).unwrap(),
    ))
}

fn render_inspection(inspection: &Inspection) -> String {
    let or_unlimited = |value: Option<String>| value.unwrap_or_else(|| "unlimited".to_owned());
    let consumed = &inspection.consumed;
    let remaining = &inspection.remaining;
    let baseline = &inspection.accepted_baseline;
    let active = &inspection.active_work;
    let mut lines = vec![
        format!("Run {}: {}", inspection.run_id, inspection.scope),
        format!(
            "Stop reason: {}{}",
            inspection.stop_reason.unwrap_or("none"),
            inspection
                .blocker
                .map(|blocker| format!(" ({blocker})"))
                .unwrap_or_default()
        ),
        match (inspection.comparison_valid, inspection.invalidation) {
            (true, _) => "Comparison: valid".to_owned(),
            (false, Some(reason)) => format!("Comparison: invalid ({reason})"),
            (false, None) => "Comparison: invalid".to_owned(),
        },
        format!(
            "Accepted baseline: {} {} at {}",
            baseline.metric, baseline.aggregate, baseline.revision
        ),
        render_limits(&inspection.limits),
        format!(
            "Consumed: {} work attempts{}, {} commands, {:.1} command seconds",
            consumed.work_attempts,
            if consumed.work_attempts_advisory {
                " (advisory: no attempt limit)"
            } else {
                ""
            },
            consumed.commands,
            consumed.command_seconds
        ),
        render_dispatches(consumed),
        format!(
            "Remaining: {} work attempts, {} commands, {} command seconds, {} wall seconds, {} workers, {} measurements",
            or_unlimited(remaining.work_attempts.map(|value| value.to_string())),
            or_unlimited(remaining.commands.map(|value| value.to_string())),
            or_unlimited(remaining.command_seconds.map(|value| format!("{value:.1}"))),
            or_unlimited(remaining.wall_seconds.map(|value| format!("{value:.0}"))),
            remaining.workers,
            remaining.measurements
        ),
    ];
    for reservation in &active.reservations {
        lines.push(format!(
            "Unsettled reservation: {} ({} for {})",
            reservation.request,
            reservation.kind.to_possible_value().unwrap().get_name(),
            reservation.task.unwrap_or("accepted code")
        ));
    }
    for task in &active.checkpoints {
        lines.push(format!("Unfinished checkpoint: {task}"));
    }
    for request in &active.pending_admissions {
        lines.push(format!("Pending admission: {request}"));
    }
    if let Some(task) = active.acceptance {
        lines.push(format!("Acceptance lane: {task}"));
    }
    lines.push(format!(
        "Tasks: {} ready, {} blocked; next ready: {}{}",
        inspection.ready_tasks,
        inspection.blocked_tasks,
        inspection.next_task.unwrap_or("none"),
        tasks::render_cross_area_waits(&inspection.cross_area_waits)
    ));
    if let Some(gain) = inspection.unreachable_minimum_gain {
        lines.push(format!(
            "Unreachable minimum gain: {} exceeds the largest possible gain {}",
            gain.minimum_gain, gain.largest_possible_gain
        ));
    }
    lines.push(format!("Next: {}", inspection.next_action));
    lines.join("\n")
}

fn render_limits(limits: &Limits) -> String {
    let described = [
        limits
            .attempt_limit
            .as_ref()
            .map(|limit| limit.describe(Limit::AttemptLimit)),
        limits
            .command_limit
            .as_ref()
            .map(|limit| limit.describe(Limit::CommandLimit)),
        limits
            .command_seconds
            .as_ref()
            .map(|limit| limit.describe(Limit::CommandSeconds)),
        limits
            .time_limit_seconds
            .as_ref()
            .map(|limit| limit.describe(Limit::TimeLimitSeconds)),
        Some(limits.worker_limit.describe(Limit::WorkerLimit)),
        Some(limits.measurement_limit.describe(Limit::MeasurementLimit)),
    ];
    format!(
        "Limits: {}",
        described
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn render_dispatches(consumed: &Consumed) -> String {
    let dispatched: Vec<String> = consumed
        .dispatches_by_kind
        .iter()
        .filter(|(_, count)| **count > 0)
        .map(|(kind, count)| format!("{count} {}", kind.to_possible_value().unwrap().get_name()))
        .collect();
    format!(
        "Dispatches: {}; {} experiments",
        if dispatched.is_empty() {
            "none".to_owned()
        } else {
            dispatched.join(", ")
        },
        consumed.experiments
    )
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
    }
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
        let settings = super::settings(root, area)?;
        // The baseline measures accepted HEAD, never a dirty candidate or supplied replacement samples.
        let outputs = settings.generated_outputs.iter().chain(untracked_outputs);
        if !accepted_worktree_clean(root, &super::untracked_exclusions(outputs))? {
            return Err(ZdevError::new(
                "Actual rebaseline requires clean accepted code; retain or restore unfinished candidates first",
            ));
        }
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
        let result = super::evaluate_recorded(root, &command, timeout, repetition, |pid| {
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
    let outputs = baseline
        .settings
        .generated_outputs
        .iter()
        .chain(&baseline.untracked_outputs);
    if baseline.head != git_output(root, &["rev-parse", "HEAD"])?
        || !current.eq_ignoring_advisory(&baseline.settings)
        || !accepted_worktree_clean(root, &super::untracked_exclusions(outputs))?
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

/// Whether accepted code outside `.zdev` has no tracked changes and no untracked files other
/// than those the exclusion pathspecs leave out.
fn accepted_worktree_clean(root: &Path, exclusions: &[String]) -> Result<bool, ZdevError> {
    let changed = [
        "diff",
        "HEAD",
        "--name-only",
        "-z",
        "--",
        ".",
        ":(exclude).zdev",
    ];
    let mut untracked = vec![
        "ls-files",
        "--others",
        "--exclude-standard",
        "-z",
        "--",
        ".",
        ":(exclude).zdev",
    ];
    untracked.extend(exclusions.iter().map(String::as_str));
    Ok(git_output_bytes(root, &changed)?.is_empty()
        && git_output_bytes(root, &untracked)?.is_empty())
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

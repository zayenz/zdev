use std::collections::{BTreeMap, BTreeSet};

pub(super) mod experiment;
pub(super) mod run;
use std::io::{self, Read};
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::{ZdevError, validate_nonempty_line};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub(super) enum Direction {
    Minimize,
    Maximize,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub(super) enum Aggregation {
    Mean,
    Median,
}

/// Comparison facts are copied into evidence; only area settings execute policy.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Comparison {
    pub metric: String,
    pub direction: Direction,
    pub evaluator_identity: String,
    pub conditions: String,
    pub baseline_revision: String,
    pub baseline_samples: Vec<f64>,
    pub repetitions: usize,
    pub aggregation: Aggregation,
    pub minimum_gain: f64,
    pub required_checks: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub significance: Option<Significance>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Settings {
    pub comparison: Comparison,
    pub evaluator_command: String,
    pub permitted_changes: Vec<String>,
    pub attempt_limit: usize,
    #[serde(default = "default_timeout")]
    pub timeout_seconds: f64,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub correctness_commands: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Hypothesis {
    pub mechanism: String,
    #[serde(default)]
    pub context_tasks: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum ObservationOutcome {
    Measured,
    Crashed,
    TimedOut,
    Invalid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Check {
    pub name: String,
    pub passed: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Observation {
    pub starting_revision: String,
    pub candidate_revision: String,
    pub outcome: ObservationOutcome,
    pub samples: Vec<f64>,
    pub checks: Vec<Check>,
    pub elapsed_seconds: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evaluation_runs: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correctness_runs: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum Decision {
    Observed,
    Interrupted,
    Accepted,
    Rejected,
    Inconclusive,
    FailedToRun,
}

impl Decision {
    pub fn is_terminal(self) -> bool {
        !matches!(self, Self::Observed | Self::Interrupted)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Verification {
    pub reviewer: String,
    pub passed: bool,
    pub summary: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ResultRecord {
    pub comparison: Comparison,
    pub attempt_limit: usize,
    pub observations: Vec<Observation>,
    pub decision: Decision,
    pub lesson: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<Verification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retained_candidate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restored_revision: Option<String>,
    /// Isolated rejection leaves the current accepted destination in place.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepted_revision: Option<String>,
}

pub(super) fn read_json<T: serde::de::DeserializeOwned>(source: &Path) -> Result<T, ZdevError> {
    let bytes = if source == Path::new("-") {
        let mut bytes = Vec::new();
        io::stdin()
            .read_to_end(&mut bytes)
            .map_err(|error| ZdevError::io("Cannot read research input", error))?;
        bytes
    } else {
        std::fs::read(source)
            .map_err(|error| ZdevError::io(format!("Cannot read {}", source.display()), error))?
    };
    serde_json::from_slice(&bytes)
        .map_err(|error| ZdevError::new(format!("Invalid research input: {error}")))
}

fn validate_revision(value: &str) -> Result<(), ZdevError> {
    if !matches!(value.len(), 40 | 64)
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(ZdevError::new(
            "Research revisions must be full lowercase Git object IDs",
        ));
    }
    Ok(())
}

fn validate_names(values: &[String], label: &str) -> Result<(), ZdevError> {
    let mut seen = BTreeSet::new();
    for value in values {
        validate_nonempty_line(value, label)?;
        if !seen.insert(value) {
            return Err(ZdevError::new(format!("Duplicate {label}: {value}")));
        }
    }
    Ok(())
}

fn validate_samples(values: &[f64]) -> Result<(), ZdevError> {
    if values.iter().any(|value| !value.is_finite()) {
        return Err(ZdevError::new("Research samples must be finite numbers"));
    }
    Ok(())
}

impl Comparison {
    fn validate(&self) -> Result<(), ZdevError> {
        for (value, label) in [
            (&self.metric, "Research metric"),
            (&self.evaluator_identity, "Evaluator identity"),
            (&self.conditions, "Comparison conditions"),
        ] {
            validate_nonempty_line(value, label)?;
        }
        validate_revision(&self.baseline_revision)?;
        validate_samples(&self.baseline_samples)?;
        if self.repetitions == 0 || self.baseline_samples.len() != self.repetitions {
            return Err(ZdevError::new(
                "Baseline samples must match positive repetitions",
            ));
        }
        if !self.minimum_gain.is_finite() || self.minimum_gain < 0.0 {
            return Err(ZdevError::new(
                "Minimum gain must be finite and nonnegative",
            ));
        }
        validate_names(&self.required_checks, "Required correctness check")?;
        if self.required_checks.is_empty() {
            return Err(ZdevError::new(
                "Research requires at least one correctness check",
            ));
        }
        // Finite individual samples can still overflow their aggregate.
        self.aggregate(&self.baseline_samples)?;
        if let Some(significance) = &self.significance {
            let Significance::PairedSign { alpha } = significance;
            if !alpha.is_finite()
                || *alpha <= 0.0
                || *alpha >= 1.0
                || !(2..=1024).contains(&self.repetitions)
            {
                return Err(ZdevError::new(
                    "Paired sign test requires 2..=1024 repetitions and 0 < alpha < 1",
                ));
            }
        }
        Ok(())
    }

    fn aggregate(&self, values: &[f64]) -> Result<f64, ZdevError> {
        let value = match self.aggregation {
            Aggregation::Mean => values.iter().map(|value| value / values.len() as f64).sum(),
            Aggregation::Median => {
                let mut ordered = values.to_vec();
                ordered.sort_by(f64::total_cmp);
                let middle = ordered.len() / 2;
                if ordered.len().is_multiple_of(2) {
                    ordered[middle - 1] / 2.0 + ordered[middle] / 2.0
                } else {
                    ordered[middle]
                }
            }
        };
        if !value.is_finite() {
            return Err(ZdevError::new("Research aggregate must be finite"));
        }
        Ok(value)
    }
}

impl Settings {
    pub fn validate(&self) -> Result<(), ZdevError> {
        self.comparison.validate()?;
        if !self.timeout_seconds.is_finite()
            || self.timeout_seconds <= 0.0
            || !std::time::Duration::try_from_secs_f64(self.timeout_seconds)
                .is_ok_and(|duration| !duration.is_zero())
        {
            return Err(ZdevError::new(
                "Evaluator timeout seconds must be finite, positive, and representable",
            ));
        }
        validate_nonempty_line(&self.evaluator_command, "Evaluator command")?;
        validate_names(&self.permitted_changes, "Permitted change")?;
        for (name, command) in &self.correctness_commands {
            validate_nonempty_line(name, "Correctness command name")?;
            validate_nonempty_line(command, "Correctness command")?;
            if !self.comparison.required_checks.contains(name) {
                return Err(ZdevError::new(
                    "Correctness commands must name required checks",
                ));
            }
        }
        if self.permitted_changes.is_empty() || self.attempt_limit == 0 {
            return Err(ZdevError::new(
                "Research requires permitted changes and a positive attempt limit",
            ));
        }
        Ok(())
    }
}

impl Hypothesis {
    pub fn validate(&self) -> Result<(), ZdevError> {
        validate_nonempty_line(&self.mechanism, "Hypothesis mechanism")?;
        validate_names(&self.context_tasks, "Hypothesis context task")
    }
}

impl Observation {
    fn validate(&self, comparison: &Comparison) -> Result<(), ZdevError> {
        validate_revision(&self.starting_revision)?;
        validate_revision(&self.candidate_revision)?;
        validate_samples(&self.samples)?;
        if !self.elapsed_seconds.is_finite() || self.elapsed_seconds < 0.0 {
            return Err(ZdevError::new(
                "Observation elapsed seconds must be finite and nonnegative",
            ));
        }
        validate_names(
            &self
                .checks
                .iter()
                .map(|check| check.name.clone())
                .collect::<Vec<_>>(),
            "Observation check",
        )?;
        if let Some(failure) = &self.failure {
            validate_nonempty_line(failure, "Observation failure")?;
        }
        if let Some(runs) = self.evaluation_runs
            && (runs < self.samples.len()
                || runs > comparison.repetitions
                || (self.outcome == ObservationOutcome::Measured && runs != comparison.repetitions))
        {
            return Err(ZdevError::new(
                "Evaluation run count must cover samples and match measured repetitions",
            ));
        }
        match self.outcome {
            ObservationOutcome::Measured => {
                if self.samples.len() != comparison.repetitions || self.failure.is_some() {
                    return Err(ZdevError::new(
                        "Measured observations require all samples and no execution failure",
                    ));
                }
                comparison.aggregate(&self.samples)?;
            }
            ObservationOutcome::Crashed
            | ObservationOutcome::TimedOut
            | ObservationOutcome::Invalid => {
                if self.failure.is_none() {
                    return Err(ZdevError::new(
                        "Failed observations require a failure description",
                    ));
                }
            }
        }
        Ok(())
    }

    fn is_usable(&self, comparison: &Comparison) -> bool {
        self.outcome == ObservationOutcome::Measured
            && self.starting_revision == comparison.baseline_revision
            && self.checks.iter().all(|check| check.passed)
            && comparison.required_checks.iter().all(|required| {
                self.checks
                    .iter()
                    .any(|check| &check.name == required && check.passed)
            })
    }
}

/// Validate each historical record against its own facts, then optionally gate
/// completion against the current area policy. Settings changes never erase history.
pub(super) fn validate_evidence(
    records: &[ResultRecord],
    completion: bool,
    current: Option<&Settings>,
) -> Result<(), ZdevError> {
    let mut attempts = 0usize;
    let mut elapsed_seconds = 0.0;
    let mut latest = None;
    let mut usable_seen = false;
    let mut terminal = false;
    for record in records {
        if terminal {
            return Err(ZdevError::new(
                "A terminal hypothesis result cannot receive more evidence; create a context-linked retry",
            ));
        }
        record.comparison.validate()?;
        if record.attempt_limit == 0 {
            return Err(ZdevError::new("Result attempt limit must be positive"));
        }
        validate_nonempty_line(&record.lesson, "Research lesson")?;
        for value in [&record.retained_candidate, &record.restored_revision]
            .into_iter()
            .flatten()
        {
            validate_nonempty_line(value, "Research completion evidence")?;
        }
        if let Some(revision) = &record.accepted_revision {
            validate_revision(revision)?;
            if !record.decision.is_terminal()
                || record.decision == Decision::Accepted
                || record.restored_revision.is_some()
            {
                return Err(ZdevError::new(
                    "Accepted destination evidence belongs to isolated rejection only",
                ));
            }
        }
        if let Some(verification) = &record.verification {
            validate_nonempty_line(&verification.reviewer, "Independent reviewer")?;
            validate_nonempty_line(&verification.summary, "Independent verification summary")?;
        }
        for observation in &record.observations {
            observation.validate(&record.comparison)?;
            elapsed_seconds += observation.elapsed_seconds;
            if !elapsed_seconds.is_finite() {
                return Err(ZdevError::new(
                    "Total research elapsed seconds must be finite",
                ));
            }
            usable_seen |= observation.is_usable(&record.comparison);
            latest = Some((observation, &record.comparison));
        }
        attempts = attempts
            .checked_add(record.observations.len())
            .ok_or_else(|| ZdevError::new("Research attempt count overflow"))?;
        if attempts > record.attempt_limit {
            return Err(ZdevError::new(
                "Research evidence exceeds the allotted attempt limit",
            ));
        }
        match record.decision {
            Decision::Accepted | Decision::Rejected => {
                let (observation, comparison) = latest
                    .ok_or_else(|| ZdevError::new("Terminal comparison requires observations"))?;
                if comparison != &record.comparison || !observation.is_usable(comparison) {
                    return Err(ZdevError::new(
                        "Accepted/rejected results require a usable comparable observation with passed correctness checks",
                    ));
                }
                if record.retained_candidate.is_none() {
                    return Err(ZdevError::new(
                        "Accepted/rejected results require retained candidate evidence",
                    ));
                }
                if record.decision == Decision::Accepted {
                    if !compare(comparison, observation)?.improved {
                        return Err(ZdevError::new(
                            "Accepted result does not meet the directional minimum gain or significance rule",
                        ));
                    }
                    if !record
                        .verification
                        .as_ref()
                        .is_some_and(|verification| verification.passed)
                    {
                        return Err(ZdevError::new(
                            "Accepted results require independent verification evidence",
                        ));
                    }
                } else if record.restored_revision.as_deref()
                    != Some(comparison.baseline_revision.as_str())
                    && record.accepted_revision.is_none()
                {
                    return Err(ZdevError::new(
                        "Rejected results require restoration to the compared baseline revision",
                    ));
                }
            }
            Decision::Inconclusive | Decision::FailedToRun => {
                if attempts != record.attempt_limit {
                    return Err(ZdevError::new(
                        "Inconclusive/failed-to-run results require exhausted allotted attempts",
                    ));
                }
                if record.decision == Decision::FailedToRun && usable_seen {
                    return Err(ZdevError::new(
                        "Failed-to-run cannot discard a usable observation; use inconclusive",
                    ));
                }
                if record.decision == Decision::Inconclusive && !usable_seen {
                    return Err(ZdevError::new(
                        "Inconclusive requires a usable observation; use failed-to-run when none ran successfully",
                    ));
                }
            }
            Decision::Observed => {
                if attempts == 0 {
                    return Err(ZdevError::new("Research evidence requires an observation"));
                }
            }
            Decision::Interrupted => {}
        }
        terminal = record.decision.is_terminal();
    }
    if completion {
        let last = records.last().filter(|record| record.decision.is_terminal())
            .ok_or_else(|| ZdevError::new("Hypothesis completion requires a terminal result; interrupted work remains open"))?;
        if let Some(settings) = current
            && (last.comparison != settings.comparison
                || last.attempt_limit != settings.attempt_limit)
        {
            return Err(ZdevError::new(
                "Hypothesis completion evidence differs from current research settings",
            ));
        }
    }
    Ok(())
}

/// Execute only the area's evaluator; callers supply revision and correctness evidence.
/// This utility collects evidence without changing task records or Git state.
pub(super) fn measure(
    root: &Path,
    area: &str,
    starting_revision: &str,
    candidate_revision: &str,
    checks_source: &Path,
) -> Result<super::CommandOutput, ZdevError> {
    let _lock = crate::ZdevStateLock::acquire(root)?;
    if run::active(root, area)? {
        return Err(ZdevError::new(
            "Standalone measure cannot bypass active run accounting; use the bound experiment lifecycle",
        ));
    }
    let settings = settings(root, area)?;
    validate_revision(starting_revision)?;
    validate_revision(candidate_revision)?;
    let checks: Vec<Check> = read_json(checks_source)?;
    let mut observation = Observation {
        starting_revision: starting_revision.to_owned(),
        candidate_revision: candidate_revision.to_owned(),
        outcome: ObservationOutcome::Measured,
        samples: Vec::new(),
        checks,
        elapsed_seconds: 0.0,
        evaluation_runs: Some(0),
        correctness_runs: None,
        failure: None,
    };
    validate_names(
        &observation
            .checks
            .iter()
            .map(|c| c.name.clone())
            .collect::<Vec<_>>(),
        "Observation check",
    )?;
    let started = std::time::Instant::now();
    for repetition in 0..settings.comparison.repetitions {
        observation.evaluation_runs = Some(repetition + 1);
        match evaluate(
            root,
            &settings.evaluator_command,
            settings.timeout_seconds,
            repetition + 1,
        ) {
            Ok(sample) => observation.samples.push(sample),
            Err((outcome, failure)) => {
                observation.outcome = outcome;
                observation.failure = Some(failure);
                break;
            }
        }
    }
    observation.elapsed_seconds = started.elapsed().as_secs_f64();
    let mut summary = if observation.outcome == ObservationOutcome::Measured {
        match summarize(&observation.samples) {
            Ok(summary) => Some(summary),
            Err(error) => {
                observation.outcome = ObservationOutcome::Invalid;
                observation.failure = Some(error.to_string());
                None
            }
        }
    } else {
        None
    };
    let comparison = match compare(&settings.comparison, &observation) {
        Ok(comparison) => comparison,
        Err(error) => {
            observation.outcome = ObservationOutcome::Invalid;
            observation.failure = Some(error.to_string());
            summary = None;
            compare(&settings.comparison, &observation)?
        }
    };
    let value = serde_json::json!({"schema_version": 1, "area": area, "comparison": settings.comparison, "observation": observation, "summary": summary, "assessment": comparison});
    Ok(super::CommandOutput::new(
        serde_json::to_string_pretty(&value).unwrap(),
        value,
    ))
}

pub(super) fn compare_observation(
    root: &Path,
    area: &str,
    source: &Path,
) -> Result<super::CommandOutput, ZdevError> {
    if !run::comparison_valid(root, area)? {
        return Err(ZdevError::new(
            "Comparisons are invalid until an actual counted rebaseline succeeds",
        ));
    }
    let settings = settings(root, area)?;
    let record: ResultRecord = read_json(source)?;
    if record.comparison != settings.comparison || record.attempt_limit != settings.attempt_limit {
        return Err(ZdevError::new(
            "Comparison evidence differs from current research settings",
        ));
    }
    validate_evidence(std::slice::from_ref(&record), false, None)?;
    let observation = record
        .observations
        .last()
        .ok_or_else(|| ZdevError::new("Comparison requires an observation"))?;
    let assessment = compare(&settings.comparison, observation)?;
    let summary = if observation.outcome == ObservationOutcome::Measured {
        Some(summarize(&observation.samples)?)
    } else {
        None
    };
    let value = serde_json::json!({"schema_version": 1, "area": area, "summary": summary, "assessment": assessment});
    Ok(super::CommandOutput::new(
        serde_json::to_string_pretty(&value).unwrap(),
        value,
    ))
}

fn default_timeout() -> f64 {
    60.0
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "method", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum Significance {
    PairedSign { alpha: f64 },
}

#[derive(Serialize)]
struct Summary {
    mean: f64,
    median: f64,
    minimum: f64,
    maximum: f64,
    spread: f64,
}

fn summarize(samples: &[f64]) -> Result<Summary, ZdevError> {
    validate_samples(samples)?;
    if samples.is_empty() {
        return Err(ZdevError::new("Cannot summarize missing measurements"));
    }
    let mut ordered = samples.to_vec();
    ordered.sort_by(f64::total_cmp);
    let middle = ordered.len() / 2;
    let summary = Summary {
        mean: samples.iter().map(|v| v / samples.len() as f64).sum(),
        median: if ordered.len().is_multiple_of(2) {
            ordered[middle - 1] / 2.0 + ordered[middle] / 2.0
        } else {
            ordered[middle]
        },
        minimum: ordered[0],
        maximum: ordered[ordered.len() - 1],
        spread: ordered[ordered.len() - 1] - ordered[0],
    };
    if !summary.mean.is_finite() || !summary.median.is_finite() || !summary.spread.is_finite() {
        return Err(ZdevError::new("Research summary must be finite"));
    }
    Ok(summary)
}

#[derive(Serialize)]
struct ComparisonResult {
    usable: bool,
    improved: bool,
    gain: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    significance: Option<SignTest>,
}

#[derive(Serialize)]
struct SignTest {
    method: &'static str,
    pairs: usize,
    ties: usize,
    wins: usize,
    p_value: f64,
    alpha: f64,
    passed: bool,
}

// Exact binomial upper tail under independent paired differences, P(sign = +) = 1/2.
// NIST: https://www.itl.nist.gov/div898/software/dataplot/refman1/auxillar/signtest.htm
fn paired_sign(
    comparison: &Comparison,
    samples: &[f64],
    alpha: f64,
) -> Result<SignTest, ZdevError> {
    let mut wins = 0;
    let mut ties = 0;
    for (baseline, candidate) in comparison.baseline_samples.iter().zip(samples) {
        let gain = match comparison.direction {
            Direction::Minimize => baseline - candidate,
            Direction::Maximize => candidate - baseline,
        };
        if !gain.is_finite() {
            return Err(ZdevError::new("Research paired gain must be finite"));
        }
        if gain > comparison.minimum_gain {
            wins += 1;
        } else if gain == comparison.minimum_gain {
            ties += 1;
        }
    }
    let pairs = samples.len() - ties;
    // Convolution avoids factorial overflow and the underflow of starting at 2^-n.
    let mut probabilities = vec![0.0; pairs + 1];
    probabilities[0] = 1.0;
    for n in 1..=pairs {
        for k in (1..=n).rev() {
            probabilities[k] = (probabilities[k] + probabilities[k - 1]) / 2.0;
        }
        probabilities[0] /= 2.0;
    }
    let p_value = probabilities[wins..].iter().sum::<f64>().min(1.0);
    Ok(SignTest {
        method: "paired_sign",
        pairs,
        ties,
        wins,
        p_value,
        alpha,
        passed: pairs > 0 && p_value <= alpha,
    })
}

fn compare(
    comparison: &Comparison,
    observation: &Observation,
) -> Result<ComparisonResult, ZdevError> {
    comparison.validate()?;
    observation.validate(comparison)?;
    if !observation.is_usable(comparison) {
        return Ok(ComparisonResult {
            usable: false,
            improved: false,
            gain: None,
            significance: None,
        });
    }
    let baseline = comparison.aggregate(&comparison.baseline_samples)?;
    let candidate = comparison.aggregate(&observation.samples)?;
    let gain = match comparison.direction {
        Direction::Minimize => baseline - candidate,
        Direction::Maximize => candidate - baseline,
    };
    if !gain.is_finite() {
        return Err(ZdevError::new("Research gain must be finite"));
    }
    let significance = match comparison.significance {
        Some(Significance::PairedSign { alpha }) => {
            Some(paired_sign(comparison, &observation.samples, alpha)?)
        }
        None => None,
    };
    Ok(ComparisonResult {
        usable: true,
        improved: gain > 0.0
            && gain >= comparison.minimum_gain
            && significance.as_ref().is_none_or(|test| test.passed),
        gain: Some(gain),
        significance,
    })
}

fn settings(root: &Path, area: &str) -> Result<Settings, ZdevError> {
    let settings = super::project::load_area(root, area)?
        .0
        .research
        .ok_or_else(|| ZdevError::new("Area has no research settings"))?;
    settings.validate()?;
    Ok(settings)
}

fn evaluate(
    root: &Path,
    command: &str,
    timeout_seconds: f64,
    repetition: usize,
) -> Result<f64, (ObservationOutcome, String)> {
    evaluate_with_started(root, command, timeout_seconds, repetition, |_| Ok(()))
}

fn evaluate_with_started(
    root: &Path,
    command: &str,
    timeout_seconds: f64,
    repetition: usize,
    started_process: impl FnOnce(u32) -> Result<(), ZdevError>,
) -> Result<f64, (ObservationOutcome, String)> {
    use std::process::{Command, Stdio};
    let crashed = |error: io::Error| (ObservationOutcome::Crashed, error.to_string());
    let output = tempfile::tempfile().map_err(crashed)?;
    let mut process = Command::new("sh");
    process
        .arg("-c")
        .arg(command)
        .current_dir(root)
        .env("ZDEV_REPETITION", repetition.to_string())
        .stdin(Stdio::null())
        .stdout(output.try_clone().map_err(crashed)?)
        .stderr(Stdio::null());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        process.process_group(0);
    }
    let mut child = process.spawn().map_err(crashed)?;
    if let Err(error) = started_process(child.id()) {
        #[cfg(unix)]
        kill_evaluator_group(child.id());
        let _ = child.kill();
        let _ = child.wait();
        return Err((ObservationOutcome::Crashed, error.to_string()));
    }
    let started = std::time::Instant::now();
    let deadline = std::time::Duration::from_secs_f64(timeout_seconds);
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if started.elapsed() < deadline => {
                std::thread::sleep(std::time::Duration::from_millis(5))
            }
            result => {
                #[cfg(unix)]
                kill_evaluator_group(child.id());
                let _ = child.kill();
                let _ = child.wait();
                return Err(match result {
                    Err(error) => crashed(error),
                    _ => (
                        ObservationOutcome::TimedOut,
                        format!(
                            "Evaluator repetition {repetition} exceeded {timeout_seconds} seconds"
                        ),
                    ),
                });
            }
        }
    };
    #[cfg(unix)]
    kill_evaluator_group(child.id());
    if !status.success() {
        return Err((
            ObservationOutcome::Crashed,
            format!("Evaluator repetition {repetition} exited with {status}"),
        ));
    }
    // Bound parsing; evaluator diagnostics belong on stderr, stdout is one scalar.
    use std::io::{Seek, SeekFrom};
    let mut output = output;
    output.seek(SeekFrom::Start(0)).map_err(crashed)?;
    let mut bytes = Vec::new();
    output.take(4097).read_to_end(&mut bytes).map_err(crashed)?;
    let sample = std::str::from_utf8(&bytes)
        .ok()
        .filter(|_| bytes.len() <= 4096)
        .and_then(|text| text.trim().parse::<f64>().ok())
        .filter(|value| value.is_finite());
    sample.ok_or_else(|| {
        (
            ObservationOutcome::Invalid,
            format!("Evaluator repetition {repetition} must output one finite number"),
        )
    })
}

#[cfg(unix)]
fn kill_evaluator_group(id: u32) {
    let _ = std::process::Command::new("sh")
        .args(["-c", &format!("kill -KILL -{id}")])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();
}

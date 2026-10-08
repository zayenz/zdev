use std::collections::BTreeSet;
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
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Settings {
    pub comparison: Comparison,
    pub evaluator_command: String,
    pub permitted_changes: Vec<String>,
    pub attempt_limit: usize,
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
        validate_nonempty_line(&self.evaluator_command, "Evaluator command")?;
        validate_names(&self.permitted_changes, "Permitted change")?;
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
        match self.outcome {
            ObservationOutcome::Measured => {
                if self.samples.len() != comparison.repetitions || self.failure.is_some() {
                    return Err(ZdevError::new(
                        "Measured observations require all samples and no execution failure",
                    ));
                }
                comparison.aggregate(&self.samples)?;
            }
            ObservationOutcome::Crashed | ObservationOutcome::TimedOut => {
                if self.failure.is_none() {
                    return Err(ZdevError::new(
                        "Crash and timeout observations require a failure description",
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
                    let baseline = comparison.aggregate(&comparison.baseline_samples)?;
                    let candidate = comparison.aggregate(&observation.samples)?;
                    let gain = match comparison.direction {
                        Direction::Minimize => baseline - candidate,
                        Direction::Maximize => candidate - baseline,
                    };
                    if !gain.is_finite() || gain <= 0.0 || gain < comparison.minimum_gain {
                        return Err(ZdevError::new(
                            "Accepted result does not meet the directional minimum gain",
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

use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};

use serde::Deserialize;
use serde_json::{Value, json};

use super::{SCHEMA_VERSION, ZdevError};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum WorkerHarness {
    Codex,
    Claude,
    Opencode,
    Pi,
    Omp,
}

impl WorkerHarness {
    fn as_str(self) -> &'static str {
        match self {
            Self::Codex => "codex",
            Self::Claude => "claude",
            Self::Opencode => "opencode",
            Self::Pi => "pi",
            Self::Omp => "omp",
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Effort {
    Inherit,
    Low,
    Medium,
    High,
    Xhigh,
    Max,
}

impl Effort {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Inherit => "inherit",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Xhigh => "xhigh",
            Self::Max => "max",
        }
    }
}

#[derive(Clone, Debug)]
struct WorkerProfile {
    model: Option<String>,
    effort: Option<Effort>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawWorkerProfile {
    inherit: Option<bool>,
    model: Option<String>,
    effort: Option<Effort>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct HarnessProfiles {
    implementer: Option<RawWorkerProfile>,
    verifier: Option<RawWorkerProfile>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct WorkerFile {
    schema_version: u64,
    #[serde(default)]
    codex: HarnessProfiles,
    #[serde(default)]
    claude: HarnessProfiles,
    #[serde(default)]
    opencode: HarnessProfiles,
    #[serde(default)]
    pi: HarnessProfiles,
    #[serde(default)]
    omp: HarnessProfiles,
}

#[derive(Default)]
struct WorkerLayer {
    codex: RoleProfiles,
    claude: RoleProfiles,
    opencode: RoleProfiles,
    pi: RoleProfiles,
    omp: RoleProfiles,
}

#[derive(Default)]
struct RoleProfiles {
    implementer: Option<WorkerProfile>,
    verifier: Option<WorkerProfile>,
}

impl WorkerLayer {
    fn roles(&self, harness: WorkerHarness) -> &RoleProfiles {
        match harness {
            WorkerHarness::Codex => &self.codex,
            WorkerHarness::Claude => &self.claude,
            WorkerHarness::Opencode => &self.opencode,
            WorkerHarness::Pi => &self.pi,
            WorkerHarness::Omp => &self.omp,
        }
    }
}

#[derive(Clone, Debug)]
struct WorkerOrigin {
    scope: &'static str,
    path: Option<String>,
}

#[derive(Clone, Debug)]
pub(super) struct ResolvedWorkerProfile {
    profile: WorkerProfile,
    origin: WorkerOrigin,
}

impl ResolvedWorkerProfile {
    pub(super) fn has_model(&self) -> bool {
        self.profile.model.is_some()
    }

    pub(super) fn has_effort(&self) -> bool {
        self.profile.effort.is_some()
    }

    pub(super) fn model_literal(&self) -> String {
        self.profile
            .model
            .as_ref()
            .map(|model| serde_json::to_string(model).expect("worker model is serializable"))
            .unwrap_or_else(|| "null".to_owned())
    }

    pub(super) fn effort_literal(&self) -> String {
        self.profile
            .effort
            .as_ref()
            .map(|effort| format!("\"{}\"", effort.as_str()))
            .unwrap_or_else(|| "null".to_owned())
    }

    fn value(&self) -> Value {
        let value = match (&self.profile.model, &self.profile.effort) {
            (None, None) => json!({"inherit": true}),
            (Some(model), Some(effort)) => {
                json!({"model": model, "effort": effort.as_str()})
            }
            (Some(model), None) => json!({"model": model, "effort": "inherit"}),
            (None, Some(_)) => unreachable!("validated profiles never contain effort alone"),
        };
        json!({
            "value": value,
            "origin": {
                "scope": self.origin.scope,
                "path": self.origin.path,
            }
        })
    }
}

#[derive(Clone, Debug)]
pub(super) struct ResolvedWorkers {
    pub(super) implementer: ResolvedWorkerProfile,
    pub(super) verifier: ResolvedWorkerProfile,
}

impl ResolvedWorkers {
    pub(super) fn value(&self) -> Value {
        json!({
            "implementer": self.implementer.value(),
            "verifier": self.verifier.value(),
        })
    }
}

pub(super) fn resolve_worker_profiles(
    project_root: Option<&Path>,
    harness: WorkerHarness,
) -> Result<ResolvedWorkers, ZdevError> {
    let global_path = global_worker_path()?;
    let global = read_worker_file(&global_path)?;
    let local_path = project_root.map(|root| root.join(".zdev/workers.toml"));
    let local = local_path
        .as_deref()
        .map(read_worker_file)
        .transpose()?
        .flatten();
    let built_in = built_in_profiles(harness);
    let local_roles = local.as_ref().map(|layer| layer.roles(harness));
    let global_roles = global.as_ref().map(|layer| layer.roles(harness));

    Ok(ResolvedWorkers {
        implementer: resolve_role(
            local_roles.and_then(|roles| roles.implementer.as_ref()),
            global_roles.and_then(|roles| roles.implementer.as_ref()),
            &built_in.0,
            local_path.as_deref(),
            &global_path,
        ),
        verifier: resolve_role(
            local_roles.and_then(|roles| roles.verifier.as_ref()),
            global_roles.and_then(|roles| roles.verifier.as_ref()),
            &built_in.1,
            local_path.as_deref(),
            &global_path,
        ),
    })
}

#[cfg(test)]
pub(super) fn built_in_worker_profiles(harness: WorkerHarness) -> ResolvedWorkers {
    let (implementer, verifier) = built_in_profiles(harness);
    let origin = WorkerOrigin {
        scope: "default",
        path: None,
    };
    ResolvedWorkers {
        implementer: ResolvedWorkerProfile {
            profile: implementer,
            origin: origin.clone(),
        },
        verifier: ResolvedWorkerProfile {
            profile: verifier,
            origin,
        },
    }
}

fn resolve_role(
    local: Option<&WorkerProfile>,
    global: Option<&WorkerProfile>,
    built_in: &WorkerProfile,
    local_path: Option<&Path>,
    global_path: &Path,
) -> ResolvedWorkerProfile {
    if let (Some(profile), Some(path)) = (local, local_path) {
        return ResolvedWorkerProfile {
            profile: profile.clone(),
            origin: WorkerOrigin {
                scope: "local",
                path: Some(local_origin(path)),
            },
        };
    }
    if let Some(profile) = global {
        return ResolvedWorkerProfile {
            profile: profile.clone(),
            origin: WorkerOrigin {
                scope: "global",
                path: Some(global_path.to_string_lossy().replace('\\', "/")),
            },
        };
    }
    ResolvedWorkerProfile {
        profile: built_in.clone(),
        origin: WorkerOrigin {
            scope: "default",
            path: None,
        },
    }
}

fn local_origin(_path: &Path) -> String {
    ".zdev/workers.toml".to_owned()
}

fn read_worker_file(path: &Path) -> Result<Option<WorkerLayer>, ZdevError> {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(ZdevError::io(
                format!("Cannot read worker configuration {}", path.display()),
                error,
            ));
        }
    };
    let source = std::str::from_utf8(&bytes).map_err(|error| {
        ZdevError::new(format!(
            "Invalid worker configuration {}: file is not UTF-8: {error}",
            path.display()
        ))
    })?;
    let file: WorkerFile = toml::from_str(source).map_err(|error| {
        ZdevError::new(format!(
            "Invalid worker configuration {}: {error}",
            path.display()
        ))
    })?;
    if file.schema_version != SCHEMA_VERSION {
        return Err(ZdevError::new(format!(
            "Invalid worker configuration {}: unsupported schema_version {}",
            path.display(),
            file.schema_version
        )));
    }

    Ok(Some(WorkerLayer {
        codex: validate_roles(path, WorkerHarness::Codex, file.codex)?,
        claude: validate_roles(path, WorkerHarness::Claude, file.claude)?,
        opencode: validate_roles(path, WorkerHarness::Opencode, file.opencode)?,
        pi: validate_roles(path, WorkerHarness::Pi, file.pi)?,
        omp: validate_roles(path, WorkerHarness::Omp, file.omp)?,
    }))
}

fn validate_roles(
    path: &Path,
    harness: WorkerHarness,
    profiles: HarnessProfiles,
) -> Result<RoleProfiles, ZdevError> {
    Ok(RoleProfiles {
        implementer: profiles
            .implementer
            .map(|profile| validate_profile(path, harness, "implementer", profile))
            .transpose()?,
        verifier: profiles
            .verifier
            .map(|profile| validate_profile(path, harness, "verifier", profile))
            .transpose()?,
    })
}

fn validate_profile(
    path: &Path,
    harness: WorkerHarness,
    role: &str,
    raw: RawWorkerProfile,
) -> Result<WorkerProfile, ZdevError> {
    let table = format!("{}.{}", harness.as_str(), role);
    if raw.inherit == Some(true) && raw.model.is_none() && raw.effort.is_none() {
        return Ok(WorkerProfile {
            model: None,
            effort: None,
        });
    }
    if raw.inherit.is_some() {
        return Err(profile_error(
            path,
            &table,
            "inherit must be true and cannot be combined with model or effort",
        ));
    }
    let model = raw
        .model
        .ok_or_else(|| profile_error(path, &table, "expected either inherit = true or a model"))?;
    if model.trim().is_empty() {
        return Err(profile_error(path, &table, "model must not be empty"));
    }
    let effort = raw.effort.ok_or_else(|| {
        profile_error(path, &table, "a model profile must include an effort value")
    })?;
    if harness == WorkerHarness::Opencode
        && !model.starts_with("openai/")
        && !matches!(effort, Effort::Inherit)
    {
        return Err(profile_error(
            path,
            &table,
            &format!(
                "effort value {:?} is unsupported for model {:?}; non-openai providers require effort = \"inherit\"",
                effort.as_str(),
                model
            ),
        ));
    }
    Ok(WorkerProfile {
        model: Some(model),
        effort: (!matches!(effort, Effort::Inherit)).then_some(effort),
    })
}

fn profile_error(path: &Path, table: &str, message: &str) -> ZdevError {
    ZdevError::new(format!(
        "Invalid worker configuration {} [{table}]: {message}",
        path.display()
    ))
}

fn built_in_profiles(harness: WorkerHarness) -> (WorkerProfile, WorkerProfile) {
    let profile = |model: &str, effort: Option<Effort>| WorkerProfile {
        model: Some(model.to_owned()),
        effort,
    };
    match harness {
        WorkerHarness::Codex => (
            profile("gpt-5.6-sol", Some(Effort::High)),
            profile("gpt-5.6-sol", Some(Effort::High)),
        ),
        WorkerHarness::Claude => (
            profile("claude-opus-5", Some(Effort::High)),
            profile("claude-opus-5", Some(Effort::High)),
        ),
        WorkerHarness::Opencode => (
            profile("openai/gpt-5.6-sol", Some(Effort::High)),
            profile("anthropic/claude-opus-5", None),
        ),
        WorkerHarness::Pi | WorkerHarness::Omp => (
            profile("openai/gpt-5.6-sol", Some(Effort::High)),
            profile("anthropic/claude-opus-5", Some(Effort::High)),
        ),
    }
}

fn global_worker_path() -> Result<PathBuf, ZdevError> {
    for (name, suffix) in [
        ("XDG_CONFIG_HOME", "zdev/workers.toml"),
        ("HOME", ".config/zdev/workers.toml"),
        ("USERPROFILE", ".config/zdev/workers.toml"),
    ] {
        let Some(base) = env::var_os(name).filter(|value| !value.is_empty()) else {
            continue;
        };
        let base = PathBuf::from(base);
        if base.is_absolute() {
            return Ok(normalize_path(&base.join(suffix)));
        }
    }
    Err(ZdevError::new(
        "Cannot locate global zdev configuration; set XDG_CONFIG_HOME, HOME, or USERPROFILE to an absolute path",
    ))
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            other => normalized.push(other.as_os_str()),
        }
    }
    normalized
}

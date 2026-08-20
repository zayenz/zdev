use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};

use serde::Deserialize;
use serde_json::{Value, json};

use super::project::{Config, read_config};
use super::{CommandOutput, SCHEMA_VERSION, ZdevError};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ConfigReadScope {
    Effective,
    Local,
    Global,
}

impl ConfigReadScope {
    fn as_str(self) -> &'static str {
        match self {
            Self::Effective => "effective",
            Self::Local => "local",
            Self::Global => "global",
        }
    }
}

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

impl WorkerProfile {
    fn value(&self) -> Value {
        match (&self.model, &self.effort) {
            (None, None) => json!({"inherit": true}),
            (Some(model), Some(effort)) => {
                json!({"model": model, "effort": effort.as_str()})
            }
            (Some(model), None) => json!({"model": model, "effort": "inherit"}),
            (None, Some(_)) => unreachable!("validated profiles never contain effort alone"),
        }
    }

    fn text(&self) -> String {
        match (&self.model, &self.effort) {
            (None, None) => "{ inherit = true }".to_owned(),
            (Some(model), Some(effort)) => format!(
                "{{ model = {}, effort = {} }}",
                quote(model),
                quote(effort.as_str())
            ),
            (Some(model), None) => {
                format!("{{ model = {}, effort = \"inherit\" }}", quote(model))
            }
            (None, Some(_)) => unreachable!("validated profiles never contain effort alone"),
        }
    }
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

    fn profile(&self, key: WorkerKey) -> Option<&WorkerProfile> {
        let roles = self.roles(key.harness);
        match key.role {
            WorkerRole::Implementer => roles.implementer.as_ref(),
            WorkerRole::Verifier => roles.verifier.as_ref(),
        }
    }
}

#[derive(Clone, Debug)]
struct Origin {
    scope: &'static str,
    path: Option<String>,
}

#[derive(Clone, Debug)]
pub(super) struct ResolvedWorkerProfile {
    profile: WorkerProfile,
    origin: Origin,
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
        json!({
            "value": self.profile.value(),
            "origin": {
                "scope": self.origin.scope,
                "path": self.origin.path,
            }
        })
    }
}

#[derive(Clone, Copy)]
enum WorkerRole {
    Implementer,
    Verifier,
}

#[derive(Clone, Copy)]
struct WorkerKey {
    name: &'static str,
    harness: WorkerHarness,
    role: WorkerRole,
}

const WORKER_KEYS: [WorkerKey; 10] = [
    WorkerKey {
        name: "worker.codex.implementer",
        harness: WorkerHarness::Codex,
        role: WorkerRole::Implementer,
    },
    WorkerKey {
        name: "worker.codex.verifier",
        harness: WorkerHarness::Codex,
        role: WorkerRole::Verifier,
    },
    WorkerKey {
        name: "worker.claude.implementer",
        harness: WorkerHarness::Claude,
        role: WorkerRole::Implementer,
    },
    WorkerKey {
        name: "worker.claude.verifier",
        harness: WorkerHarness::Claude,
        role: WorkerRole::Verifier,
    },
    WorkerKey {
        name: "worker.opencode.implementer",
        harness: WorkerHarness::Opencode,
        role: WorkerRole::Implementer,
    },
    WorkerKey {
        name: "worker.opencode.verifier",
        harness: WorkerHarness::Opencode,
        role: WorkerRole::Verifier,
    },
    WorkerKey {
        name: "worker.pi.implementer",
        harness: WorkerHarness::Pi,
        role: WorkerRole::Implementer,
    },
    WorkerKey {
        name: "worker.pi.verifier",
        harness: WorkerHarness::Pi,
        role: WorkerRole::Verifier,
    },
    WorkerKey {
        name: "worker.omp.implementer",
        harness: WorkerHarness::Omp,
        role: WorkerRole::Implementer,
    },
    WorkerKey {
        name: "worker.omp.verifier",
        harness: WorkerHarness::Omp,
        role: WorkerRole::Verifier,
    },
];

impl Origin {
    fn value(&self) -> Value {
        json!({"path": self.path, "scope": self.scope})
    }

    fn text(&self) -> String {
        match &self.path {
            Some(path) => format!("{} {path}", self.scope),
            None => self.scope.to_owned(),
        }
    }
}

struct Candidate {
    value: Value,
    text: String,
    origin: Origin,
}

impl Candidate {
    fn from_profile(profile: &WorkerProfile, origin: Origin) -> Self {
        Self {
            value: profile.value(),
            text: profile.text(),
            origin,
        }
    }

    fn value(&self) -> Value {
        json!({"origin": self.origin.value(), "value": self.value})
    }
}

struct ConfigValue {
    key: &'static str,
    value: Value,
    text: String,
    origin: Origin,
    shadowed: Vec<Candidate>,
}

impl ConfigValue {
    fn value(&self) -> Value {
        json!({
            "key": self.key,
            "origin": self.origin.value(),
            "shadowed": self.shadowed.iter().map(Candidate::value).collect::<Vec<_>>(),
            "value": self.value,
        })
    }

    fn text(&self) -> String {
        let mut text = format!("{} = {}  [{}]", self.key, self.text, self.origin.text());
        for shadowed in &self.shadowed {
            text.push_str(&format!(
                "\n  shadows {}  [{}]",
                shadowed.text,
                shadowed.origin.text()
            ));
        }
        text
    }
}

pub(super) fn show(
    root: Option<&Path>,
    scope: ConfigReadScope,
) -> Result<CommandOutput, ZdevError> {
    let values = read_values(root, scope)?;
    let text = values
        .iter()
        .map(ConfigValue::text)
        .collect::<Vec<_>>()
        .join("\n");
    Ok(CommandOutput::new(
        text,
        json!({
            "schema_version": SCHEMA_VERSION,
            "scope": scope.as_str(),
            "values": values.iter().map(ConfigValue::value).collect::<Vec<_>>(),
        }),
    ))
}

pub(super) fn get(
    root: Option<&Path>,
    scope: ConfigReadScope,
    key: &str,
) -> Result<CommandOutput, ZdevError> {
    require_known_key(key)?;
    if scope == ConfigReadScope::Global && key.starts_with("project.") {
        return Err(ZdevError::new(format!(
            "Configuration key {key} is not available in global scope"
        )));
    }
    let entry = read_values(root, scope)?
        .into_iter()
        .find(|entry| entry.key == key)
        .ok_or_else(|| {
            ZdevError::new(format!(
                "Configuration key {key} is not set in {} scope",
                scope.as_str()
            ))
        })?;
    let text = entry.text();
    let mut value = entry.value();
    value["schema_version"] = json!(SCHEMA_VERSION);
    Ok(CommandOutput::new(text, value))
}

fn require_known_key(key: &str) -> Result<(), ZdevError> {
    if matches!(
        key,
        "project.name"
            | "project.record"
            | "project.trunk"
            | "project.default-area"
            | "project.guidance"
    ) || WORKER_KEYS.iter().any(|worker| worker.name == key)
    {
        return Ok(());
    }
    Err(ZdevError::new(format!("Unknown configuration key {key}")))
}

fn read_values(root: Option<&Path>, scope: ConfigReadScope) -> Result<Vec<ConfigValue>, ZdevError> {
    let project = if scope == ConfigReadScope::Global {
        None
    } else {
        let root =
            root.ok_or_else(|| ZdevError::new("A local configuration read requires a project"))?;
        Some(read_config(root)?)
    };
    let local_path = root.map(|root| root.join(".zdev/workers.toml"));
    let local = if scope == ConfigReadScope::Global {
        None
    } else {
        local_path
            .as_deref()
            .map(read_worker_file)
            .transpose()?
            .flatten()
    };
    let global_path = if scope == ConfigReadScope::Local {
        None
    } else {
        Some(global_worker_path()?)
    };
    let global = global_path
        .as_deref()
        .map(read_worker_file)
        .transpose()?
        .flatten();

    let mut values = Vec::with_capacity(15);
    if let Some(project) = project.as_ref() {
        append_project_values(&mut values, project, scope);
    }
    for key in WORKER_KEYS {
        match scope {
            ConfigReadScope::Effective => values.push(effective_worker_value(
                key,
                local.as_ref(),
                global.as_ref(),
                global_path
                    .as_deref()
                    .expect("effective reads have a global path"),
            )),
            ConfigReadScope::Local => {
                if let Some(profile) = local.as_ref().and_then(|layer| layer.profile(key)) {
                    values.push(ConfigValue {
                        key: key.name,
                        value: profile.value(),
                        text: profile.text(),
                        origin: local_origin_value(),
                        shadowed: Vec::new(),
                    });
                }
            }
            ConfigReadScope::Global => {
                if let Some(profile) = global.as_ref().and_then(|layer| layer.profile(key)) {
                    values.push(ConfigValue {
                        key: key.name,
                        value: profile.value(),
                        text: profile.text(),
                        origin: global_origin_value(
                            global_path
                                .as_deref()
                                .expect("global reads have a global path"),
                        ),
                        shadowed: Vec::new(),
                    });
                }
            }
        }
    }
    Ok(values)
}

fn append_project_values(values: &mut Vec<ConfigValue>, config: &Config, scope: ConfigReadScope) {
    let local = Origin {
        scope: "local",
        path: Some(".zdev/config.toml".to_owned()),
    };
    values.push(scalar_value(
        "project.name",
        json!(config.project.name),
        local.clone(),
        Vec::new(),
    ));
    values.push(scalar_value(
        "project.record",
        json!(config.project.record.as_str()),
        local.clone(),
        Vec::new(),
    ));
    append_optional_project_value(
        values,
        "project.trunk",
        config.project.trunk.as_deref(),
        Value::Null,
        scope,
        &local,
    );
    append_optional_project_value(
        values,
        "project.default-area",
        config.project.default_area.as_deref(),
        Value::Null,
        scope,
        &local,
    );
    append_optional_project_value(
        values,
        "project.guidance",
        config.project.guidance.as_deref(),
        json!("auto"),
        scope,
        &local,
    );
}

fn append_optional_project_value(
    values: &mut Vec<ConfigValue>,
    key: &'static str,
    stored: Option<&str>,
    default: Value,
    scope: ConfigReadScope,
    local: &Origin,
) {
    match (stored, scope) {
        (Some(value), ConfigReadScope::Effective) => values.push(scalar_value(
            key,
            json!(value),
            local.clone(),
            vec![scalar_candidate(default, default_origin())],
        )),
        (Some(value), ConfigReadScope::Local) => {
            values.push(scalar_value(key, json!(value), local.clone(), Vec::new()))
        }
        (None, ConfigReadScope::Effective) => {
            values.push(scalar_value(key, default, default_origin(), Vec::new()))
        }
        (None, ConfigReadScope::Local) => {}
        (_, ConfigReadScope::Global) => unreachable!("global views omit project keys"),
    }
}

fn effective_worker_value(
    key: WorkerKey,
    local: Option<&WorkerLayer>,
    global: Option<&WorkerLayer>,
    global_path: &Path,
) -> ConfigValue {
    let local_profile = local.and_then(|layer| layer.profile(key));
    let global_profile = global.and_then(|layer| layer.profile(key));
    let built_in = built_in_profile(key);
    let (profile, origin) = if let Some(profile) = local_profile {
        (profile, local_origin_value())
    } else if let Some(profile) = global_profile {
        (profile, global_origin_value(global_path))
    } else {
        (&built_in, default_origin())
    };
    let mut shadowed = Vec::new();
    if local_profile.is_some() {
        if let Some(global) = global_profile {
            shadowed.push(Candidate::from_profile(
                global,
                global_origin_value(global_path),
            ));
        }
        shadowed.push(Candidate::from_profile(&built_in, default_origin()));
    } else if global_profile.is_some() {
        shadowed.push(Candidate::from_profile(&built_in, default_origin()));
    }
    ConfigValue {
        key: key.name,
        value: profile.value(),
        text: profile.text(),
        origin,
        shadowed,
    }
}

fn scalar_value(
    key: &'static str,
    value: Value,
    origin: Origin,
    shadowed: Vec<Candidate>,
) -> ConfigValue {
    ConfigValue {
        key,
        text: scalar_text(&value),
        value,
        origin,
        shadowed,
    }
}

fn scalar_candidate(value: Value, origin: Origin) -> Candidate {
    Candidate {
        text: scalar_text(&value),
        value,
        origin,
    }
}

fn scalar_text(value: &Value) -> String {
    match value {
        Value::Null => "null".to_owned(),
        Value::String(value) => quote(value),
        _ => unreachable!("configuration scalars are strings or null"),
    }
}

fn quote(value: &str) -> String {
    toml::Value::String(value.to_owned()).to_string()
}

fn default_origin() -> Origin {
    Origin {
        scope: "default",
        path: None,
    }
}

fn local_origin_value() -> Origin {
    Origin {
        scope: "local",
        path: Some(".zdev/workers.toml".to_owned()),
    }
}

fn global_origin_value(path: &Path) -> Origin {
    Origin {
        scope: "global",
        path: Some(path.to_string_lossy().replace('\\', "/")),
    }
}

fn built_in_profile(key: WorkerKey) -> WorkerProfile {
    let profiles = built_in_profiles(key.harness);
    match key.role {
        WorkerRole::Implementer => profiles.0,
        WorkerRole::Verifier => profiles.1,
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
    let origin = Origin {
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
            origin: Origin {
                scope: "local",
                path: Some(local_origin(path)),
            },
        };
    }
    if let Some(profile) = global {
        return ResolvedWorkerProfile {
            profile: profile.clone(),
            origin: Origin {
                scope: "global",
                path: Some(global_path.to_string_lossy().replace('\\', "/")),
            },
        };
    }
    ResolvedWorkerProfile {
        profile: built_in.clone(),
        origin: Origin {
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

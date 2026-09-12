use std::collections::BTreeMap;
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::project::{Config, load_area, read_config, write_config};
use super::{CommandOutput, SCHEMA_VERSION, ZdevError, ZdevStateLock, write_atomic};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ConfigReadScope {
    Effective,
    Local,
    Global,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ConfigWriteScope {
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
    Agy,
}

impl WorkerHarness {
    fn as_str(self) -> &'static str {
        match self {
            Self::Codex => "codex",
            Self::Claude => "claude",
            Self::Opencode => "opencode",
            Self::Pi => "pi",
            Self::Omp => "omp",
            Self::Agy => "agy",
        }
    }

    fn parse(value: &str) -> Result<Self, ZdevError> {
        match value {
            "codex" => Ok(Self::Codex),
            "claude" => Ok(Self::Claude),
            "opencode" => Ok(Self::Opencode),
            "pi" => Ok(Self::Pi),
            "omp" => Ok(Self::Omp),
            "agy" => Ok(Self::Agy),
            _ => Err(ZdevError::new(format!(
                "Unknown worker harness {value}; expected codex, claude, opencode, pi, omp, or agy"
            ))),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

    fn parse(value: &str) -> Result<Self, ZdevError> {
        match value {
            "inherit" => Ok(Self::Inherit),
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "xhigh" => Ok(Self::Xhigh),
            "max" => Ok(Self::Max),
            _ => Err(ZdevError::new(format!(
                "Unknown worker effort {value}; expected inherit, low, medium, high, xhigh, or max"
            ))),
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct RawWorkerProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    inherit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effort: Option<Effort>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct HarnessProfiles {
    #[serde(
        rename = "routine-implementer",
        skip_serializing_if = "Option::is_none"
    )]
    routine_implementer: Option<RawWorkerProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    implementer: Option<RawWorkerProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verifier: Option<RawWorkerProfile>,
    #[serde(
        rename = "advanced-implementer",
        skip_serializing_if = "Option::is_none"
    )]
    advanced_implementer: Option<RawWorkerProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    planner: Option<RawWorkerProfile>,
}

impl HarnessProfiles {
    fn is_empty(&self) -> bool {
        self.routine_implementer.is_none()
            && self.implementer.is_none()
            && self.verifier.is_none()
            && self.advanced_implementer.is_none()
            && self.planner.is_none()
    }

    fn get(&self, role: WorkerRole) -> Option<&RawWorkerProfile> {
        match role {
            WorkerRole::RoutineImplementer => self.routine_implementer.as_ref(),
            WorkerRole::Implementer => self.implementer.as_ref(),
            WorkerRole::Verifier => self.verifier.as_ref(),
            WorkerRole::AdvancedImplementer => self.advanced_implementer.as_ref(),
            WorkerRole::Planner => self.planner.as_ref(),
        }
    }
    fn set_role(&mut self, role: WorkerRole, value: Option<RawWorkerProfile>) -> bool {
        let target = match role {
            WorkerRole::RoutineImplementer => &mut self.routine_implementer,
            WorkerRole::Implementer => &mut self.implementer,
            WorkerRole::Verifier => &mut self.verifier,
            WorkerRole::AdvancedImplementer => &mut self.advanced_implementer,
            WorkerRole::Planner => &mut self.planner,
        };
        let existed = target.is_some();
        *target = value;
        existed
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct WorkerFile {
    schema_version: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_profile: Option<String>,
    #[serde(default, skip_serializing_if = "HarnessProfiles::is_empty")]
    codex: HarnessProfiles,
    #[serde(default, skip_serializing_if = "HarnessProfiles::is_empty")]
    claude: HarnessProfiles,
    #[serde(default, skip_serializing_if = "HarnessProfiles::is_empty")]
    opencode: HarnessProfiles,
    #[serde(default, skip_serializing_if = "HarnessProfiles::is_empty")]
    pi: HarnessProfiles,
    #[serde(default, skip_serializing_if = "HarnessProfiles::is_empty")]
    omp: HarnessProfiles,
    #[serde(default, skip_serializing_if = "HarnessProfiles::is_empty")]
    agy: HarnessProfiles,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    profiles: BTreeMap<String, NamedProfile>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct NamedProfile {
    #[serde(default, skip_serializing_if = "HarnessProfiles::is_empty")]
    codex: HarnessProfiles,
    #[serde(default, skip_serializing_if = "HarnessProfiles::is_empty")]
    claude: HarnessProfiles,
    #[serde(default, skip_serializing_if = "HarnessProfiles::is_empty")]
    opencode: HarnessProfiles,
    #[serde(default, skip_serializing_if = "HarnessProfiles::is_empty")]
    pi: HarnessProfiles,
    #[serde(default, skip_serializing_if = "HarnessProfiles::is_empty")]
    omp: HarnessProfiles,
    #[serde(default, skip_serializing_if = "HarnessProfiles::is_empty")]
    agy: HarnessProfiles,
}

impl NamedProfile {
    fn harness(&self, harness: WorkerHarness) -> &HarnessProfiles {
        match harness {
            WorkerHarness::Codex => &self.codex,
            WorkerHarness::Claude => &self.claude,
            WorkerHarness::Opencode => &self.opencode,
            WorkerHarness::Pi => &self.pi,
            WorkerHarness::Omp => &self.omp,
            WorkerHarness::Agy => &self.agy,
        }
    }
    fn harness_mut(&mut self, harness: WorkerHarness) -> &mut HarnessProfiles {
        match harness {
            WorkerHarness::Codex => &mut self.codex,
            WorkerHarness::Claude => &mut self.claude,
            WorkerHarness::Opencode => &mut self.opencode,
            WorkerHarness::Pi => &mut self.pi,
            WorkerHarness::Omp => &mut self.omp,
            WorkerHarness::Agy => &mut self.agy,
        }
    }
    fn is_empty(&self) -> bool {
        self.codex.is_empty()
            && self.claude.is_empty()
            && self.opencode.is_empty()
            && self.pi.is_empty()
            && self.omp.is_empty()
            && self.agy.is_empty()
    }
}

impl Default for WorkerFile {
    fn default() -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            default_profile: None,
            codex: HarnessProfiles::default(),
            claude: HarnessProfiles::default(),
            opencode: HarnessProfiles::default(),
            pi: HarnessProfiles::default(),
            omp: HarnessProfiles::default(),
            agy: HarnessProfiles::default(),
            profiles: BTreeMap::new(),
        }
    }
}

impl WorkerFile {
    fn profiles_mut(&mut self, harness: WorkerHarness) -> &mut HarnessProfiles {
        match harness {
            WorkerHarness::Codex => &mut self.codex,
            WorkerHarness::Claude => &mut self.claude,
            WorkerHarness::Opencode => &mut self.opencode,
            WorkerHarness::Pi => &mut self.pi,
            WorkerHarness::Omp => &mut self.omp,
            WorkerHarness::Agy => &mut self.agy,
        }
    }

    fn set(&mut self, key: WorkerKey, value: Option<RawWorkerProfile>) -> bool {
        let profiles = self.profiles_mut(key.harness);
        let target = match key.role {
            WorkerRole::RoutineImplementer => &mut profiles.routine_implementer,
            WorkerRole::Implementer => &mut profiles.implementer,
            WorkerRole::Verifier => &mut profiles.verifier,
            WorkerRole::AdvancedImplementer => &mut profiles.advanced_implementer,
            WorkerRole::Planner => &mut profiles.planner,
        };
        let existed = target.is_some();
        *target = value;
        existed
    }
}

#[derive(Default)]
struct WorkerLayer {
    codex: RoleProfiles,
    claude: RoleProfiles,
    opencode: RoleProfiles,
    pi: RoleProfiles,
    omp: RoleProfiles,
    agy: RoleProfiles,
}

#[derive(Default)]
struct RoleProfiles {
    routine_implementer: Option<WorkerProfile>,
    implementer: Option<WorkerProfile>,
    verifier: Option<WorkerProfile>,
    advanced_implementer: Option<WorkerProfile>,
    planner: Option<WorkerProfile>,
}

impl RoleProfiles {
    fn role(&self, role: WorkerRole) -> Option<&WorkerProfile> {
        match role {
            WorkerRole::RoutineImplementer => self.routine_implementer.as_ref(),
            WorkerRole::Implementer => self.implementer.as_ref(),
            WorkerRole::Verifier => self.verifier.as_ref(),
            WorkerRole::AdvancedImplementer => self.advanced_implementer.as_ref(),
            WorkerRole::Planner => self.planner.as_ref(),
        }
    }
}

impl WorkerLayer {
    fn roles(&self, harness: WorkerHarness) -> &RoleProfiles {
        match harness {
            WorkerHarness::Codex => &self.codex,
            WorkerHarness::Claude => &self.claude,
            WorkerHarness::Opencode => &self.opencode,
            WorkerHarness::Pi => &self.pi,
            WorkerHarness::Omp => &self.omp,
            WorkerHarness::Agy => &self.agy,
        }
    }

    fn profile(&self, key: WorkerKey) -> Option<&WorkerProfile> {
        let roles = self.roles(key.harness);
        match key.role {
            WorkerRole::RoutineImplementer => roles.routine_implementer.as_ref(),
            WorkerRole::Implementer => roles.implementer.as_ref(),
            WorkerRole::Verifier => roles.verifier.as_ref(),
            WorkerRole::AdvancedImplementer => roles.advanced_implementer.as_ref(),
            WorkerRole::Planner => roles.planner.as_ref(),
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

#[derive(Clone, Copy, Eq, PartialEq)]
enum WorkerRole {
    RoutineImplementer,
    Implementer,
    Verifier,
    AdvancedImplementer,
    Planner,
}

impl WorkerRole {
    fn parse(value: &str) -> Result<Self, ZdevError> {
        match value {
            "routine-implementer" => Ok(Self::RoutineImplementer),
            "implementer" => Ok(Self::Implementer),
            "verifier" => Ok(Self::Verifier),
            "advanced-implementer" => Ok(Self::AdvancedImplementer),
            "planner" => Ok(Self::Planner),
            _ => Err(ZdevError::new(format!(
                "Unknown worker role {value}; expected routine-implementer, implementer, verifier, advanced-implementer, or planner"
            ))),
        }
    }
}

#[derive(Clone, Copy)]
struct WorkerKey {
    name: &'static str,
    harness: WorkerHarness,
    role: WorkerRole,
}

const WORKER_KEYS: [WorkerKey; 30] = [
    WorkerKey {
        name: "worker.codex.routine-implementer",
        harness: WorkerHarness::Codex,
        role: WorkerRole::RoutineImplementer,
    },
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
        name: "worker.codex.advanced-implementer",
        harness: WorkerHarness::Codex,
        role: WorkerRole::AdvancedImplementer,
    },
    WorkerKey {
        name: "worker.codex.planner",
        harness: WorkerHarness::Codex,
        role: WorkerRole::Planner,
    },
    WorkerKey {
        name: "worker.claude.routine-implementer",
        harness: WorkerHarness::Claude,
        role: WorkerRole::RoutineImplementer,
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
        name: "worker.claude.advanced-implementer",
        harness: WorkerHarness::Claude,
        role: WorkerRole::AdvancedImplementer,
    },
    WorkerKey {
        name: "worker.claude.planner",
        harness: WorkerHarness::Claude,
        role: WorkerRole::Planner,
    },
    WorkerKey {
        name: "worker.opencode.routine-implementer",
        harness: WorkerHarness::Opencode,
        role: WorkerRole::RoutineImplementer,
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
        name: "worker.opencode.advanced-implementer",
        harness: WorkerHarness::Opencode,
        role: WorkerRole::AdvancedImplementer,
    },
    WorkerKey {
        name: "worker.opencode.planner",
        harness: WorkerHarness::Opencode,
        role: WorkerRole::Planner,
    },
    WorkerKey {
        name: "worker.pi.routine-implementer",
        harness: WorkerHarness::Pi,
        role: WorkerRole::RoutineImplementer,
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
        name: "worker.pi.advanced-implementer",
        harness: WorkerHarness::Pi,
        role: WorkerRole::AdvancedImplementer,
    },
    WorkerKey {
        name: "worker.pi.planner",
        harness: WorkerHarness::Pi,
        role: WorkerRole::Planner,
    },
    WorkerKey {
        name: "worker.omp.routine-implementer",
        harness: WorkerHarness::Omp,
        role: WorkerRole::RoutineImplementer,
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
    WorkerKey {
        name: "worker.omp.advanced-implementer",
        harness: WorkerHarness::Omp,
        role: WorkerRole::AdvancedImplementer,
    },
    WorkerKey {
        name: "worker.omp.planner",
        harness: WorkerHarness::Omp,
        role: WorkerRole::Planner,
    },
    WorkerKey {
        name: "worker.agy.routine-implementer",
        harness: WorkerHarness::Agy,
        role: WorkerRole::RoutineImplementer,
    },
    WorkerKey {
        name: "worker.agy.implementer",
        harness: WorkerHarness::Agy,
        role: WorkerRole::Implementer,
    },
    WorkerKey {
        name: "worker.agy.verifier",
        harness: WorkerHarness::Agy,
        role: WorkerRole::Verifier,
    },
    WorkerKey {
        name: "worker.agy.advanced-implementer",
        harness: WorkerHarness::Agy,
        role: WorkerRole::AdvancedImplementer,
    },
    WorkerKey {
        name: "worker.agy.planner",
        harness: WorkerHarness::Agy,
        role: WorkerRole::Planner,
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
    let kind = config_key(key)?;
    if scope == ConfigReadScope::Global && matches!(kind, ConfigKey::Project(_)) {
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

#[derive(Clone, Copy)]
enum ProjectKey {
    Name,
    Record,
    Trunk,
    DefaultArea,
    Guidance,
}

#[derive(Clone, Copy)]
enum ConfigKey {
    Project(ProjectKey),
    Worker(WorkerKey),
}

fn config_key(key: &str) -> Result<ConfigKey, ZdevError> {
    let project = match key {
        "project.name" => Some(ProjectKey::Name),
        "project.record" => Some(ProjectKey::Record),
        "project.trunk" => Some(ProjectKey::Trunk),
        "project.default-area" => Some(ProjectKey::DefaultArea),
        "project.guidance" => Some(ProjectKey::Guidance),
        _ => None,
    };
    if let Some(project) = project {
        return Ok(ConfigKey::Project(project));
    }
    if let Some(worker) = WORKER_KEYS.iter().find(|worker| worker.name == key) {
        return Ok(ConfigKey::Worker(*worker));
    }
    Err(ZdevError::new(format!("Unknown configuration key {key}")))
}

pub(super) fn set(
    root: Option<&Path>,
    scope: ConfigWriteScope,
    key: &str,
    values: &[String],
    allow_divergent: bool,
) -> Result<CommandOutput, ZdevError> {
    let kind = writable_key(scope, key)?;
    if allow_divergent
        && (scope != ConfigWriteScope::Local
            || !matches!(kind, ConfigKey::Project(ProjectKey::Trunk)))
    {
        return Err(ZdevError::new(
            "--allow-divergent is available only for a local project.trunk write",
        ));
    }
    match kind {
        ConfigKey::Project(ProjectKey::Trunk) => {
            let root = root
                .ok_or_else(|| ZdevError::new("A local configuration write requires a project"))?;
            let requested = require_one_value(key, values)?;
            super::project::configure_trunk(root, Some(requested), allow_divergent)
        }
        ConfigKey::Project(project) => set_project(
            root.ok_or_else(|| ZdevError::new("A local configuration write requires a project"))?,
            project,
            key,
            values,
        ),
        ConfigKey::Worker(worker) => set_worker(root, scope, worker, values),
    }
}

pub(super) fn unset(
    root: Option<&Path>,
    scope: ConfigWriteScope,
    key: &str,
) -> Result<CommandOutput, ZdevError> {
    let kind = writable_key(scope, key)?;
    match kind {
        ConfigKey::Project(project) => unset_project(
            root.ok_or_else(|| ZdevError::new("A local configuration write requires a project"))?,
            project,
            key,
        ),
        ConfigKey::Worker(worker) => unset_worker(root, scope, worker),
    }
}

fn writable_key(scope: ConfigWriteScope, key: &str) -> Result<ConfigKey, ZdevError> {
    let kind = config_key(key)?;
    if matches!(
        kind,
        ConfigKey::Project(ProjectKey::Name | ProjectKey::Record)
    ) {
        return Err(ZdevError::new(format!(
            "Configuration key {key} is read-only; choose project identity and record policy with `zdev init --record <personal|project|pull-request>` (see docs/user-guide.md)"
        )));
    }
    if scope == ConfigWriteScope::Global && matches!(kind, ConfigKey::Project(_)) {
        return Err(ZdevError::new(format!(
            "Configuration key {key} is not available in global scope"
        )));
    }
    Ok(kind)
}

fn require_one_value<'a>(key: &str, values: &'a [String]) -> Result<&'a str, ZdevError> {
    match values {
        [value] => Ok(value),
        _ => Err(ZdevError::new(format!(
            "Configuration key {key} requires exactly one value"
        ))),
    }
}

fn set_project(
    root: &Path,
    project: ProjectKey,
    key: &str,
    values: &[String],
) -> Result<CommandOutput, ZdevError> {
    let requested = require_one_value(key, values)?;
    let value = match project {
        ProjectKey::Trunk => super::project::canonical_branch(root, requested)?,
        ProjectKey::DefaultArea => {
            super::project::validate_default_area(root, requested)?;
            requested.to_owned()
        }
        ProjectKey::Guidance => {
            super::integrations::validate_guidance_selection(root, requested)?;
            requested.to_owned()
        }
        ProjectKey::Name | ProjectKey::Record => unreachable!("read-only keys were rejected"),
    };
    let _lock = ZdevStateLock::acquire(root)?;
    let mut config = read_config(root)?;
    validate_local_workers(root)?;
    match project {
        ProjectKey::Trunk => config.project.trunk = Some(value.clone()),
        ProjectKey::DefaultArea => config.project.default_area = Some(value.clone()),
        ProjectKey::Guidance => config.project.guidance = Some(value.clone()),
        ProjectKey::Name | ProjectKey::Record => unreachable!("read-only keys were rejected"),
    }
    write_config(root, &config)?;
    Ok(set_output(key, json!(value), local_project_origin()))
}

fn unset_project(root: &Path, project: ProjectKey, key: &str) -> Result<CommandOutput, ZdevError> {
    let _lock = ZdevStateLock::acquire(root)?;
    let mut config = read_config(root)?;
    validate_local_workers(root)?;
    if matches!(project, ProjectKey::Trunk) {
        super::project::validate_trunk_unset(root)?;
    }
    let removed = match project {
        ProjectKey::Trunk => config.project.trunk.take(),
        ProjectKey::DefaultArea => config.project.default_area.take(),
        ProjectKey::Guidance => config.project.guidance.take(),
        ProjectKey::Name | ProjectKey::Record => unreachable!("read-only keys were rejected"),
    };
    if removed.is_none() {
        return Err(not_set_error(key, "local"));
    }
    let effective = match project {
        ProjectKey::Trunk | ProjectKey::DefaultArea => {
            scalar_candidate(Value::Null, default_origin())
        }
        ProjectKey::Guidance => scalar_candidate(json!("auto"), default_origin()),
        ProjectKey::Name | ProjectKey::Record => unreachable!("read-only keys were rejected"),
    };
    write_config(root, &config)?;
    Ok(unset_output(key, local_project_origin(), effective))
}

fn set_worker(
    root: Option<&Path>,
    scope: ConfigWriteScope,
    key: WorkerKey,
    values: &[String],
) -> Result<CommandOutput, ZdevError> {
    let path = worker_target(root, scope)?;
    let raw = parse_worker_value(key.name, values)?;
    let profile = validate_profile(&path, key.harness, worker_role_name(key.role), &raw)?;
    let origin = worker_target_origin(scope, &path);
    match scope {
        ConfigWriteScope::Local => {
            let root = root.expect("local worker writes have a project");
            let _lock = ZdevStateLock::acquire(root)?;
            read_config(root)?;
            write_worker_value(&path, key, Some(raw), false, "local")?;
        }
        ConfigWriteScope::Global => {
            let _lock = GlobalWorkerLock::acquire(&path)?;
            write_worker_value(&path, key, Some(raw), false, "global")?;
        }
    }
    Ok(with_integration_refresh(
        set_output(key.name, profile.value(), origin),
        scope,
        key.harness,
    ))
}

fn unset_worker(
    root: Option<&Path>,
    scope: ConfigWriteScope,
    key: WorkerKey,
) -> Result<CommandOutput, ZdevError> {
    let path = worker_target(root, scope)?;
    let fallback = match scope {
        ConfigWriteScope::Local => {
            let global_path = global_worker_path()?;
            let global = read_worker_file(&global_path)?;
            global
                .as_ref()
                .and_then(|layer| layer.profile(key))
                .map(|profile| Candidate::from_profile(profile, global_origin_value(&global_path)))
                .unwrap_or_else(|| {
                    Candidate::from_profile(&built_in_profile(key), default_origin())
                })
        }
        ConfigWriteScope::Global => {
            Candidate::from_profile(&built_in_profile(key), default_origin())
        }
    };
    let origin = worker_target_origin(scope, &path);
    match scope {
        ConfigWriteScope::Local => {
            let root = root.expect("local worker writes have a project");
            let _lock = ZdevStateLock::acquire(root)?;
            read_config(root)?;
            write_worker_value(&path, key, None, true, "local")?;
        }
        ConfigWriteScope::Global => {
            let _lock = GlobalWorkerLock::acquire(&path)?;
            write_worker_value(&path, key, None, true, "global")?;
        }
    }
    Ok(with_integration_refresh(
        unset_output(key.name, origin, fallback),
        scope,
        key.harness,
    ))
}

fn worker_target(root: Option<&Path>, scope: ConfigWriteScope) -> Result<PathBuf, ZdevError> {
    match scope {
        ConfigWriteScope::Local => root
            .map(|root| root.join(".zdev/workers.toml"))
            .ok_or_else(|| ZdevError::new("A local configuration write requires a project")),
        ConfigWriteScope::Global => global_worker_path(),
    }
}

fn parse_worker_value(key: &str, values: &[String]) -> Result<RawWorkerProfile, ZdevError> {
    match values {
        [inherit] if inherit == "inherit" => Ok(RawWorkerProfile {
            inherit: Some(true),
            model: None,
            effort: None,
        }),
        [model, effort] => Ok(RawWorkerProfile {
            inherit: None,
            model: Some(model.clone()),
            effort: Some(Effort::parse(effort)?),
        }),
        _ => Err(ZdevError::new(format!(
            "Configuration key {key} requires `inherit` or exactly two values: model and effort"
        ))),
    }
}

fn worker_role_name(role: WorkerRole) -> &'static str {
    match role {
        WorkerRole::RoutineImplementer => "routine-implementer",
        WorkerRole::Implementer => "implementer",
        WorkerRole::Verifier => "verifier",
        WorkerRole::AdvancedImplementer => "advanced-implementer",
        WorkerRole::Planner => "planner",
    }
}

pub(super) fn validate_local_workers(root: &Path) -> Result<(), ZdevError> {
    read_worker_file(&root.join(".zdev/workers.toml")).map(|_| ())
}

fn write_worker_value(
    path: &Path,
    key: WorkerKey,
    value: Option<RawWorkerProfile>,
    require_existing: bool,
    scope: &str,
) -> Result<(), ZdevError> {
    let mut document = read_worker_document(path)?.unwrap_or_default();
    validate_worker_file(path, &document)?;
    let existed = document.set(key, value);
    if require_existing && !existed {
        return Err(not_set_error(key.name, scope));
    }
    validate_worker_file(path, &document)?;
    let rendered = toml::to_string_pretty(&document)
        .map_err(|error| ZdevError::new(format!("Cannot render {}: {error}", path.display())))?;
    write_atomic(path, rendered.as_bytes())
}

fn with_integration_refresh(
    mut output: CommandOutput,
    scope: ConfigWriteScope,
    harness: WorkerHarness,
) -> CommandOutput {
    let command = format!(
        "zdev skill install {} --scope {} --force",
        harness.as_str(),
        match scope {
            ConfigWriteScope::Local => "project",
            ConfigWriteScope::Global => "user",
        }
    );
    output
        .text
        .push_str(&format!("\nRefresh integration: {command}"));
    output.value["integration_refresh_command"] = json!(command);
    output.value["integration_refresh_required"] = json!(true);
    output
}

fn with_all_integration_refresh(
    mut output: CommandOutput,
    scope: ConfigWriteScope,
) -> CommandOutput {
    let command = format!(
        "zdev skill install <codex|claude|opencode|pi|omp|agy> --scope {} --force",
        match scope {
            ConfigWriteScope::Local => "project",
            ConfigWriteScope::Global => "user",
        }
    );
    output
        .text
        .push_str(&format!("\nRefresh each installed integration: {command}"));
    output.value["integration_refresh_command"] = json!(command);
    output.value["integration_refresh_required"] = json!(true);
    output
}

fn set_output(key: &str, value: Value, origin: Origin) -> CommandOutput {
    CommandOutput::new(
        format!("Set {key} in {}.", origin.text()),
        json!({
            "key": key,
            "origin": origin.value(),
            "schema_version": SCHEMA_VERSION,
            "status": "set",
            "value": value,
        }),
    )
}

fn unset_output(key: &str, origin: Origin, effective: Candidate) -> CommandOutput {
    CommandOutput::new(
        format!(
            "Unset {key} from {}.\nEffective value: {}  [{}]",
            origin.text(),
            effective.text,
            effective.origin.text()
        ),
        json!({
            "effective": effective.value(),
            "key": key,
            "origin": origin.value(),
            "schema_version": SCHEMA_VERSION,
            "status": "unset",
        }),
    )
}

fn not_set_error(key: &str, scope: &str) -> ZdevError {
    ZdevError::new(format!(
        "Configuration key {key} is not set in {scope} scope"
    ))
}

fn local_project_origin() -> Origin {
    Origin {
        scope: "local",
        path: Some(".zdev/config.toml".to_owned()),
    }
}

fn worker_target_origin(scope: ConfigWriteScope, path: &Path) -> Origin {
    match scope {
        ConfigWriteScope::Local => local_origin_value(),
        ConfigWriteScope::Global => global_origin_value(path),
    }
}

struct GlobalWorkerLock {
    _file: File,
}

impl GlobalWorkerLock {
    fn acquire(worker_path: &Path) -> Result<Self, ZdevError> {
        let parent = worker_path.parent().ok_or_else(|| {
            ZdevError::new(format!("Path has no parent: {}", worker_path.display()))
        })?;
        fs::create_dir_all(parent)
            .map_err(|error| ZdevError::io(format!("Cannot create {}", parent.display()), error))?;
        let path = parent.join("workers.lock");
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)
            .map_err(|error| {
                ZdevError::io(format!("Cannot open worker lock {}", path.display()), error)
            })?;
        for _ in 0..100 {
            match file.try_lock() {
                Ok(()) => {
                    file.set_len(0)
                        .and_then(|()| writeln!(file, "{}", std::process::id()))
                        .map_err(|error| {
                            ZdevError::io(
                                format!("Cannot write worker lock {}", path.display()),
                                error,
                            )
                        })?;
                    return Ok(Self { _file: file });
                }
                Err(std::fs::TryLockError::WouldBlock) => {
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
                Err(std::fs::TryLockError::Error(error)) => {
                    return Err(ZdevError::io(
                        format!("Cannot acquire worker lock {}", path.display()),
                        error,
                    ));
                }
            }
        }
        Err(ZdevError::new(format!(
            "Another worker configuration update is running. Retry when it finishes. Lock: {}",
            path.display()
        )))
    }
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

    let mut values = Vec::with_capacity(30);
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
    let mut local_profile = local.and_then(|layer| layer.profile(key));
    let mut global_profile = global.and_then(|layer| layer.profile(key));
    if key.role == WorkerRole::Planner && local_profile.is_none() && global_profile.is_none() {
        let advanced = WorkerKey {
            name: key.name,
            harness: key.harness,
            role: WorkerRole::AdvancedImplementer,
        };
        local_profile = local.and_then(|layer| layer.profile(advanced));
        global_profile = global.and_then(|layer| layer.profile(advanced));
    }
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
        WorkerRole::RoutineImplementer => profiles
            .routine_implementer
            .expect("built-in routine implementer"),
        WorkerRole::Implementer => profiles.implementer.expect("built-in implementer"),
        WorkerRole::Verifier => profiles.verifier.expect("built-in verifier"),
        WorkerRole::AdvancedImplementer => profiles
            .advanced_implementer
            .expect("built-in advanced implementer"),
        WorkerRole::Planner => profiles
            .advanced_implementer
            .expect("built-in planner fallback"),
    }
}

#[derive(Clone, Debug)]
pub(super) struct ResolvedWorkers {
    pub(super) routine_implementer: ResolvedWorkerProfile,
    pub(super) implementer: ResolvedWorkerProfile,
    pub(super) verifier: ResolvedWorkerProfile,
    pub(super) advanced_implementer: ResolvedWorkerProfile,
    pub(super) planner: ResolvedWorkerProfile,
}

impl ResolvedWorkers {
    pub(super) fn value(&self) -> Value {
        json!({
            "routine-implementer": self.routine_implementer.value(),
            "implementer": self.implementer.value(),
            "verifier": self.verifier.value(),
            "advanced-implementer": self.advanced_implementer.value(),
            "planner": self.planner.value(),
        })
    }
}

fn resolve_normal_worker_profiles(
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

    let advanced_implementer = resolve_role(
        local_roles.and_then(|roles| roles.advanced_implementer.as_ref()),
        global_roles.and_then(|roles| roles.advanced_implementer.as_ref()),
        built_in
            .advanced_implementer
            .as_ref()
            .expect("built-in advanced implementer"),
        local_path.as_deref(),
        &global_path,
    );
    let planner = if local_roles
        .and_then(|roles| roles.planner.as_ref())
        .is_some()
        || global_roles
            .and_then(|roles| roles.planner.as_ref())
            .is_some()
    {
        resolve_role(
            local_roles.and_then(|roles| roles.planner.as_ref()),
            global_roles.and_then(|roles| roles.planner.as_ref()),
            built_in
                .advanced_implementer
                .as_ref()
                .expect("built-in advanced implementer"),
            local_path.as_deref(),
            &global_path,
        )
    } else {
        advanced_implementer.clone()
    };
    Ok(ResolvedWorkers {
        routine_implementer: resolve_role(
            local_roles.and_then(|roles| roles.routine_implementer.as_ref()),
            global_roles.and_then(|roles| roles.routine_implementer.as_ref()),
            built_in
                .routine_implementer
                .as_ref()
                .expect("built-in routine implementer"),
            local_path.as_deref(),
            &global_path,
        ),
        implementer: resolve_role(
            local_roles.and_then(|roles| roles.implementer.as_ref()),
            global_roles.and_then(|roles| roles.implementer.as_ref()),
            built_in.implementer.as_ref().expect("built-in implementer"),
            local_path.as_deref(),
            &global_path,
        ),
        verifier: resolve_role(
            local_roles.and_then(|roles| roles.verifier.as_ref()),
            global_roles.and_then(|roles| roles.verifier.as_ref()),
            built_in.verifier.as_ref().expect("built-in verifier"),
            local_path.as_deref(),
            &global_path,
        ),
        advanced_implementer,
        planner,
    })
}

pub(super) fn resolve_worker_profiles(
    project_root: Option<&Path>,
    harness: WorkerHarness,
) -> Result<ResolvedWorkers, ZdevError> {
    let global_path = global_worker_path()?;
    let global = read_worker_document(&global_path)?;
    let local = project_root
        .map(|r| read_worker_document(&r.join(".zdev/workers.toml")))
        .transpose()?
        .flatten();
    let selected = choose_profile(local.as_ref(), global.as_ref(), None, None, None);
    if selected == "normal" {
        return resolve_normal_worker_profiles(project_root, harness);
    }
    let resolve = |role| {
        resolve_named_role(project_root, harness, role, Some(&selected), None, None)
            .map(|(_, value, _)| value)
    };
    Ok(ResolvedWorkers {
        routine_implementer: resolve(WorkerRole::RoutineImplementer)?,
        implementer: resolve(WorkerRole::Implementer)?,
        verifier: resolve(WorkerRole::Verifier)?,
        advanced_implementer: resolve(WorkerRole::AdvancedImplementer)?,
        planner: resolve(WorkerRole::Planner)?,
    })
}

pub(super) fn resolve_configured_worker_profiles(
    project_root: Option<&Path>,
    harness: WorkerHarness,
) -> Result<Vec<(String, ResolvedWorkers)>, ZdevError> {
    let global = read_worker_document(&global_worker_path()?)?;
    let local = project_root
        .map(|root| read_worker_document(&root.join(".zdev/workers.toml")))
        .transpose()?
        .flatten();
    let mut names = std::collections::BTreeSet::new();
    for file in [global.as_ref(), local.as_ref()].into_iter().flatten() {
        names.extend(file.profiles.iter().filter_map(|(name, profile)| {
            (!profile.harness(harness).is_empty()).then_some(name.clone())
        }));
    }
    names
        .into_iter()
        .map(|name| {
            let resolve = |role| {
                resolve_named_role(project_root, harness, role, Some(&name), None, None)
                    .map(|(_, value, _)| value)
            };
            Ok((
                name.clone(),
                ResolvedWorkers {
                    routine_implementer: resolve(WorkerRole::RoutineImplementer)?,
                    implementer: resolve(WorkerRole::Implementer)?,
                    verifier: resolve(WorkerRole::Verifier)?,
                    advanced_implementer: resolve(WorkerRole::AdvancedImplementer)?,
                    planner: resolve(WorkerRole::Planner)?,
                },
            ))
        })
        .collect()
}

#[cfg(test)]
pub(super) fn built_in_worker_profiles(harness: WorkerHarness) -> ResolvedWorkers {
    let profiles = built_in_profiles(harness);
    let origin = Origin {
        scope: "default",
        path: None,
    };
    ResolvedWorkers {
        routine_implementer: ResolvedWorkerProfile {
            profile: profiles
                .routine_implementer
                .expect("built-in routine implementer"),
            origin: origin.clone(),
        },
        implementer: ResolvedWorkerProfile {
            profile: profiles.implementer.expect("built-in implementer"),
            origin: origin.clone(),
        },
        verifier: ResolvedWorkerProfile {
            profile: profiles.verifier.expect("built-in verifier"),
            origin: origin.clone(),
        },
        advanced_implementer: ResolvedWorkerProfile {
            profile: profiles
                .advanced_implementer
                .clone()
                .expect("built-in advanced implementer"),
            origin,
        },
        planner: ResolvedWorkerProfile {
            profile: profiles
                .advanced_implementer
                .expect("built-in planner fallback"),
            origin: Origin {
                scope: "default",
                path: None,
            },
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
    read_worker_document(path)?
        .map(|file| validate_worker_file(path, &file))
        .transpose()
}

fn read_worker_document(path: &Path) -> Result<Option<WorkerFile>, ZdevError> {
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

    Ok(Some(file))
}

fn validate_worker_file(path: &Path, file: &WorkerFile) -> Result<WorkerLayer, ZdevError> {
    for (name, profile) in &file.profiles {
        validate_profile_name(name)?;
        for (harness, rows) in [
            (WorkerHarness::Codex, &profile.codex),
            (WorkerHarness::Claude, &profile.claude),
            (WorkerHarness::Opencode, &profile.opencode),
            (WorkerHarness::Pi, &profile.pi),
            (WorkerHarness::Omp, &profile.omp),
            (WorkerHarness::Agy, &profile.agy),
        ] {
            validate_roles(path, harness, rows)?;
        }
    }
    Ok(WorkerLayer {
        codex: validate_roles(path, WorkerHarness::Codex, &file.codex)?,
        claude: validate_roles(path, WorkerHarness::Claude, &file.claude)?,
        opencode: validate_roles(path, WorkerHarness::Opencode, &file.opencode)?,
        pi: validate_roles(path, WorkerHarness::Pi, &file.pi)?,
        omp: validate_roles(path, WorkerHarness::Omp, &file.omp)?,
        agy: validate_roles(path, WorkerHarness::Agy, &file.agy)?,
    })
}

fn validate_roles(
    path: &Path,
    harness: WorkerHarness,
    profiles: &HarnessProfiles,
) -> Result<RoleProfiles, ZdevError> {
    Ok(RoleProfiles {
        routine_implementer: profiles
            .routine_implementer
            .as_ref()
            .map(|profile| validate_profile(path, harness, "routine-implementer", profile))
            .transpose()?,
        implementer: profiles
            .implementer
            .as_ref()
            .map(|profile| validate_profile(path, harness, "implementer", profile))
            .transpose()?,
        verifier: profiles
            .verifier
            .as_ref()
            .map(|profile| validate_profile(path, harness, "verifier", profile))
            .transpose()?,
        advanced_implementer: profiles
            .advanced_implementer
            .as_ref()
            .map(|profile| validate_profile(path, harness, "advanced-implementer", profile))
            .transpose()?,
        planner: profiles
            .planner
            .as_ref()
            .map(|profile| validate_profile(path, harness, "planner", profile))
            .transpose()?,
    })
}

fn validate_profile(
    path: &Path,
    harness: WorkerHarness,
    role: &str,
    raw: &RawWorkerProfile,
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
        .as_ref()
        .ok_or_else(|| profile_error(path, &table, "expected either inherit = true or a model"))?;
    if model.trim().is_empty() {
        return Err(profile_error(path, &table, "model must not be empty"));
    }
    let effort = raw.effort.as_ref().ok_or_else(|| {
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
        model: Some(model.clone()),
        effort: (!matches!(effort, Effort::Inherit)).then_some(effort.clone()),
    })
}

fn profile_error(path: &Path, table: &str, message: &str) -> ZdevError {
    ZdevError::new(format!(
        "Invalid worker configuration {} [{table}]: {message}",
        path.display()
    ))
}

fn built_in_profiles(harness: WorkerHarness) -> RoleProfiles {
    let profile = |model: &str, effort: Option<Effort>| WorkerProfile {
        model: Some(model.to_owned()),
        effort,
    };
    match harness {
        WorkerHarness::Codex => RoleProfiles {
            routine_implementer: Some(profile("gpt-5.6-luna", Some(Effort::Low))),
            implementer: Some(profile("gpt-5.6-sol", Some(Effort::Low))),
            verifier: Some(profile("gpt-5.6-sol", Some(Effort::Low))),
            advanced_implementer: Some(profile("gpt-5.6-sol", Some(Effort::High))),
            planner: None,
        },
        WorkerHarness::Claude => RoleProfiles {
            routine_implementer: Some(profile("haiku", Some(Effort::Low))),
            implementer: Some(profile("claude-opus-5", Some(Effort::Low))),
            verifier: Some(profile("claude-opus-5", Some(Effort::Low))),
            advanced_implementer: Some(profile("claude-opus-5", Some(Effort::High))),
            planner: None,
        },
        WorkerHarness::Opencode => RoleProfiles {
            routine_implementer: Some(profile("openai/gpt-5.6-luna", Some(Effort::Low))),
            implementer: Some(profile("openai/gpt-5.6-sol", Some(Effort::Low))),
            verifier: Some(profile("anthropic/claude-opus-5", None)),
            advanced_implementer: Some(profile("openai/gpt-5.6-sol", Some(Effort::High))),
            planner: None,
        },
        WorkerHarness::Pi | WorkerHarness::Omp => RoleProfiles {
            routine_implementer: Some(profile("openai/gpt-5.6-luna", Some(Effort::Low))),
            implementer: Some(profile("openai/gpt-5.6-sol", Some(Effort::Low))),
            verifier: Some(profile("anthropic/claude-opus-5", Some(Effort::Low))),
            advanced_implementer: Some(profile("openai/gpt-5.6-sol", Some(Effort::High))),
            planner: None,
        },
        WorkerHarness::Agy => RoleProfiles {
            routine_implementer: Some(profile("gemini-3.8-flash", Some(Effort::Low))),
            implementer: Some(profile("gemini-3.8-flash", Some(Effort::Medium))),
            verifier: Some(profile("gemini-3.8-flash", Some(Effort::Medium))),
            advanced_implementer: Some(profile("gemini-3.8-flash", Some(Effort::High))),
            planner: Some(profile("gemini-3.8-flash", Some(Effort::High))),
        },
    }
}

fn built_in_named(name: &str, harness: WorkerHarness) -> Option<RoleProfiles> {
    let profile = |model: &str, effort| {
        Some(WorkerProfile {
            model: Some(model.to_owned()),
            effort: Some(effort),
        })
    };
    let inherit = || None;
    match (name, harness) {
        ("advanced", WorkerHarness::Codex) => Some(RoleProfiles {
            routine_implementer: inherit(),
            implementer: profile("gpt-6-astra", Effort::High),
            advanced_implementer: profile("gpt-6-astra", Effort::Xhigh),
            planner: profile("gpt-6-astra", Effort::Xhigh),
            verifier: profile("gpt-6-astra", Effort::High),
        }),
        ("simple", WorkerHarness::Codex) => Some(RoleProfiles {
            routine_implementer: profile("gpt-5.6-luna", Effort::Low),
            implementer: profile("gpt-5.6-luna", Effort::Low),
            advanced_implementer: profile("gpt-5.6-luna", Effort::High),
            planner: profile("gpt-5.6-luna", Effort::High),
            verifier: inherit(),
        }),
        ("advanced", WorkerHarness::Claude) => Some(RoleProfiles {
            routine_implementer: inherit(),
            implementer: profile("claude-fable-5-1", Effort::High),
            advanced_implementer: profile("claude-fable-5-1", Effort::Xhigh),
            planner: profile("claude-fable-5-1", Effort::Xhigh),
            verifier: profile("claude-fable-5-1", Effort::High),
        }),
        _ => None,
    }
}

fn named_role<'a>(
    file: Option<&'a WorkerFile>,
    name: &str,
    harness: WorkerHarness,
    role: WorkerRole,
) -> Option<&'a RawWorkerProfile> {
    file?.profiles.get(name)?.harness(harness).get(role)
}

fn validate_profile_name(name: &str) -> Result<(), ZdevError> {
    if name == "normal"
        || name.is_empty()
        || !name
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
    {
        return Err(ZdevError::new(
            "Profile names use lowercase letters, digits, and hyphens; normal is reserved",
        ));
    }
    Ok(())
}

fn raw_to_resolved(
    path: &Path,
    harness: WorkerHarness,
    role: WorkerRole,
    raw: &RawWorkerProfile,
    scope: &'static str,
) -> Result<ResolvedWorkerProfile, ZdevError> {
    Ok(ResolvedWorkerProfile {
        profile: validate_profile(path, harness, worker_role_name(role), raw)?,
        origin: Origin {
            scope,
            path: Some(if scope == "local" {
                ".zdev/workers.toml".to_owned()
            } else {
                path.to_string_lossy().replace('\\', "/")
            }),
        },
    })
}

fn choose_profile(
    local: Option<&WorkerFile>,
    global: Option<&WorkerFile>,
    explicit_role: Option<&str>,
    run: Option<&str>,
    area: Option<&str>,
) -> String {
    explicit_role
        .or(run)
        .or(area)
        .or_else(|| local.and_then(|f| f.default_profile.as_deref()))
        .or_else(|| global.and_then(|f| f.default_profile.as_deref()))
        .unwrap_or("normal")
        .to_owned()
}

fn resolve_named_role(
    project_root: Option<&Path>,
    harness: WorkerHarness,
    role: WorkerRole,
    explicit_role: Option<&str>,
    run: Option<&str>,
    area: Option<&str>,
) -> Result<(String, ResolvedWorkerProfile, Option<&'static str>), ZdevError> {
    let global_path = global_worker_path()?;
    let global = read_worker_document(&global_path)?;
    let local_path = project_root.map(|r| r.join(".zdev/workers.toml"));
    let local = local_path
        .as_deref()
        .map(read_worker_document)
        .transpose()?
        .flatten();
    let name = choose_profile(local.as_ref(), global.as_ref(), explicit_role, run, area);
    if name == "normal" {
        let workers = resolve_normal_worker_profiles(project_root, harness)?;
        let value = match role {
            WorkerRole::RoutineImplementer => workers.routine_implementer,
            WorkerRole::Implementer => workers.implementer,
            WorkerRole::Verifier => workers.verifier,
            WorkerRole::AdvancedImplementer => workers.advanced_implementer,
            WorkerRole::Planner => workers.planner,
        };
        return Ok((
            name,
            value,
            (role == WorkerRole::Planner).then_some("advanced-implementer"),
        ));
    }
    let builtin = built_in_named(&name, harness);
    let defined = local
        .as_ref()
        .and_then(|f| f.profiles.get(&name))
        .map(|p| !p.harness(harness).is_empty())
        .unwrap_or(false)
        || global
            .as_ref()
            .and_then(|f| f.profiles.get(&name))
            .map(|p| !p.harness(harness).is_empty())
            .unwrap_or(false)
        || builtin.is_some();
    if !defined {
        return Err(ZdevError::new(format!(
            "Execution profile {name} is undefined for harness {}",
            harness.as_str()
        )));
    }
    let requested = named_role(local.as_ref(), &name, harness, role)
        .map(|r| (r, local_path.as_deref().unwrap(), "local"))
        .or_else(|| {
            named_role(global.as_ref(), &name, harness, role)
                .map(|r| (r, global_path.as_path(), "global"))
        });
    if let Some((raw, path, scope)) = requested {
        return Ok((
            name,
            raw_to_resolved(path, harness, role, raw, scope)?,
            None,
        ));
    }
    if let Some(value) = builtin.as_ref().and_then(|p| p.role(role).cloned()) {
        return Ok((
            name,
            ResolvedWorkerProfile {
                profile: value,
                origin: Origin {
                    scope: "built-in-profile",
                    path: None,
                },
            },
            None,
        ));
    }
    if role == WorkerRole::Planner {
        let (_, value, _) = resolve_named_role(
            project_root,
            harness,
            WorkerRole::AdvancedImplementer,
            Some(&name),
            None,
            None,
        )?;
        return Ok((name, value, Some("advanced-implementer")));
    }
    let (_, value, _) =
        resolve_named_role(project_root, harness, role, Some("normal"), None, None)?;
    Ok((name, value, Some("normal")))
}

pub(super) fn profile_resolve(
    root: Option<&Path>,
    harness: &str,
    role: &str,
    role_profile: Option<&str>,
    run_profile: Option<&str>,
    area: Option<&str>,
) -> Result<CommandOutput, ZdevError> {
    let harness = WorkerHarness::parse(harness)?;
    let role = WorkerRole::parse(role)?;
    let area_profile = match (root, area) {
        (Some(root), Some(area)) => load_area(root, area)?
            .0
            .execution_profiles
            .get(harness.as_str())
            .cloned(),
        (None, Some(_)) => return Err(ZdevError::new("--area requires an initialized repository")),
        (_, None) => None,
    };
    let (selected, value, fallback) = resolve_named_role(
        root,
        harness,
        role,
        role_profile,
        run_profile,
        area_profile.as_deref(),
    )?;
    Ok(CommandOutput::new(
        format!(
            "{} {} = {}  [{}]",
            selected,
            worker_role_name(role),
            value.profile.text(),
            value.origin.text()
        ),
        json!({
        "schema_version": SCHEMA_VERSION, "profile": selected, "harness": harness.as_str(), "role": worker_role_name(role),
        "value": value.profile.value(), "origin": value.origin.value(), "fallback": fallback }),
    ))
}

pub(super) fn profile_list(root: Option<&Path>) -> Result<CommandOutput, ZdevError> {
    let global_path = global_worker_path()?;
    let global = read_worker_document(&global_path)?;
    let local = root
        .map(|r| read_worker_document(&r.join(".zdev/workers.toml")))
        .transpose()?
        .flatten();
    let mut names = std::collections::BTreeSet::from([
        "normal".to_owned(),
        "advanced".to_owned(),
        "simple".to_owned(),
    ]);
    for file in [global.as_ref(), local.as_ref()].into_iter().flatten() {
        names.extend(file.profiles.keys().cloned());
    }
    let names: Vec<_> = names.into_iter().collect();
    Ok(CommandOutput::new(
        names.join("\n"),
        json!({"schema_version": SCHEMA_VERSION, "profiles": names}),
    ))
}

pub(super) fn profile_show(
    root: Option<&Path>,
    name: &str,
    harness: &str,
) -> Result<CommandOutput, ZdevError> {
    let mut rows = serde_json::Map::new();
    for role in [
        WorkerRole::RoutineImplementer,
        WorkerRole::Implementer,
        WorkerRole::AdvancedImplementer,
        WorkerRole::Planner,
        WorkerRole::Verifier,
    ] {
        let out = profile_resolve(
            root,
            harness,
            worker_role_name(role),
            Some(name),
            None,
            None,
        )?;
        rows.insert(worker_role_name(role).to_owned(), out.value);
    }
    Ok(CommandOutput::new(
        format!("Profile {name} for {harness}"),
        json!({"schema_version": SCHEMA_VERSION, "profile": name, "harness": harness, "roles": rows}),
    ))
}

pub(super) fn validate_worker_harness(harness: &str) -> Result<(), ZdevError> {
    WorkerHarness::parse(harness).map(|_| ())
}

pub(super) fn validate_area_profile_reference(harness: &str, name: &str) -> Result<(), ZdevError> {
    WorkerHarness::parse(harness)?;
    if name == "normal" {
        Ok(())
    } else {
        validate_profile_name(name)
    }
}

pub(super) fn validate_named_profile_for_harness(
    root: Option<&Path>,
    harness: &str,
    name: &str,
) -> Result<(), ZdevError> {
    let harness = WorkerHarness::parse(harness)?;
    if name == "normal" || built_in_named(name, harness).is_some() {
        return Ok(());
    }
    validate_profile_name(name)?;
    let global = read_worker_document(&global_worker_path()?)?;
    let local = root
        .map(|root| read_worker_document(&root.join(".zdev/workers.toml")))
        .transpose()?
        .flatten();
    let defined = local
        .as_ref()
        .and_then(|file| file.profiles.get(name))
        .is_some_and(|profile| !profile.harness(harness).is_empty())
        || global
            .as_ref()
            .and_then(|file| file.profiles.get(name))
            .is_some_and(|profile| !profile.harness(harness).is_empty());
    if defined {
        Ok(())
    } else {
        Err(ZdevError::new(format!(
            "Execution profile {name} is undefined for harness {}",
            harness.as_str()
        )))
    }
}

fn mutate_profile(
    root: Option<&Path>,
    scope: ConfigWriteScope,
    mutator: impl FnOnce(&mut WorkerFile) -> Result<(), ZdevError>,
) -> Result<PathBuf, ZdevError> {
    let path = worker_target(root, scope)?;
    let _local_lock;
    let _global_lock;
    match scope {
        ConfigWriteScope::Local => {
            let project = root.expect("local profile write has project");
            _local_lock = Some(ZdevStateLock::acquire(project)?);
            _global_lock = None;
            read_config(project)?;
        }
        ConfigWriteScope::Global => {
            _global_lock = Some(GlobalWorkerLock::acquire(&path)?);
            _local_lock = None;
        }
    }
    let mut document = read_worker_document(&path)?.unwrap_or_default();
    validate_worker_file(&path, &document)?;
    mutator(&mut document)?;
    validate_worker_file(&path, &document)?;
    if let Some(name) = document
        .default_profile
        .as_deref()
        .filter(|name| *name != "normal" && *name != "advanced" && *name != "simple")
    {
        let inherited = if scope == ConfigWriteScope::Local {
            read_worker_document(&global_worker_path()?)?
                .is_some_and(|file| file.profiles.contains_key(name))
        } else {
            false
        };
        if !document.profiles.contains_key(name) && !inherited {
            return Err(ZdevError::new(format!(
                "default_profile refers to undefined execution profile {name}"
            )));
        }
    }
    let rendered = toml::to_string_pretty(&document)
        .map_err(|e| ZdevError::new(format!("Cannot render {}: {e}", path.display())))?;
    write_atomic(&path, rendered.as_bytes())?;
    Ok(path)
}

pub(super) fn profile_set(
    root: Option<&Path>,
    scope: ConfigWriteScope,
    name: &str,
    harness: &str,
    role: &str,
    values: &[String],
) -> Result<CommandOutput, ZdevError> {
    validate_profile_name(name)?;
    let harness = WorkerHarness::parse(harness)?;
    let role = WorkerRole::parse(role)?;
    let raw = parse_worker_value("profile role", values)?;
    let value = validate_profile(
        Path::new("workers.toml"),
        harness,
        worker_role_name(role),
        &raw,
    )?
    .value();
    let path = mutate_profile(root, scope, |f| {
        f.profiles
            .entry(name.to_owned())
            .or_default()
            .harness_mut(harness)
            .set_role(role, Some(raw));
        Ok(())
    })?;
    Ok(with_integration_refresh(
        set_output(
            &format!(
                "profiles.{name}.{}.{}",
                harness.as_str(),
                worker_role_name(role)
            ),
            value,
            worker_target_origin(scope, &path),
        ),
        scope,
        harness,
    ))
}

pub(super) fn profile_unset(
    root: Option<&Path>,
    scope: ConfigWriteScope,
    name: &str,
    harness: &str,
    role: &str,
) -> Result<CommandOutput, ZdevError> {
    validate_profile_name(name)?;
    let harness = WorkerHarness::parse(harness)?;
    let role = WorkerRole::parse(role)?;
    let path = mutate_profile(root, scope, |f| {
        let named = f
            .profiles
            .get_mut(name)
            .ok_or_else(|| not_set_error(name, "selected"))?;
        if !named.harness_mut(harness).set_role(role, None) {
            return Err(not_set_error(worker_role_name(role), "selected"));
        }
        if named.is_empty() {
            f.profiles.remove(name);
        }
        Ok(())
    })?;
    Ok(with_integration_refresh(
        CommandOutput::new(
            format!(
                "Unset profiles.{name}.{}.{} from {}.",
                harness.as_str(),
                worker_role_name(role),
                worker_target_origin(scope, &path).text()
            ),
            json!({"schema_version": SCHEMA_VERSION, "status":"unset", "profile":name, "harness":harness.as_str(), "role":worker_role_name(role)}),
        ),
        scope,
        harness,
    ))
}

pub(super) fn profile_set_default(
    root: Option<&Path>,
    scope: ConfigWriteScope,
    name: &str,
) -> Result<CommandOutput, ZdevError> {
    if name != "normal" {
        validate_profile_name(name)?;
    }
    if name != "normal" {
        let global_path = global_worker_path()?;
        let global = read_worker_document(&global_path)?;
        let local = root
            .map(|r| read_worker_document(&r.join(".zdev/workers.toml")))
            .transpose()?
            .flatten();
        let known = [local.as_ref(), global.as_ref()]
            .into_iter()
            .flatten()
            .any(|f| f.profiles.contains_key(name))
            || name == "advanced"
            || name == "simple";
        if !known {
            return Err(ZdevError::new(format!(
                "Cannot save undefined execution profile {name}"
            )));
        }
    }
    let path = mutate_profile(root, scope, |f| {
        f.default_profile = Some(name.to_owned());
        Ok(())
    })?;
    Ok(with_all_integration_refresh(
        set_output(
            "default_profile",
            json!(name),
            worker_target_origin(scope, &path),
        ),
        scope,
    ))
}

pub(super) fn profile_unset_default(
    root: Option<&Path>,
    scope: ConfigWriteScope,
) -> Result<CommandOutput, ZdevError> {
    let path = mutate_profile(root, scope, |f| {
        if f.default_profile.take().is_none() {
            return Err(not_set_error("default_profile", "selected"));
        }
        Ok(())
    })?;
    Ok(with_all_integration_refresh(
        CommandOutput::new(
            format!(
                "Unset default_profile from {}.",
                worker_target_origin(scope, &path).text()
            ),
            json!({"schema_version":SCHEMA_VERSION,"status":"unset","key":"default_profile"}),
        ),
        scope,
    ))
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

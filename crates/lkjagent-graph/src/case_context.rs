use crate::policy::ContextPressureLevel;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphContextState {
    pub selected_packages: Vec<String>,
    pub loaded_packages: Vec<String>,
    pub bindings: Vec<ContextBinding>,
    pub compression: Vec<PackageCompression>,
    pub stale: bool,
    pub pressure: ContextPressureLevel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextBinding {
    pub package: String,
    pub reason: String,
    pub priority: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageCompression {
    pub package: String,
    pub level: ContextPressureLevel,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WorkspaceState {
    pub candidate_paths: Vec<String>,
    pub touched_paths: Vec<String>,
    pub recent_paths: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaseBudgetState {
    pub max_observation_tokens: usize,
    pub max_graph_tokens: usize,
    pub max_batch_files: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaseHealthState {
    pub context_valid: bool,
    pub pressure: ContextPressureLevel,
    pub recent_faults: usize,
}

impl GraphContextState {
    pub fn new(selected_packages: Vec<String>) -> Self {
        Self {
            selected_packages,
            loaded_packages: Vec::new(),
            bindings: Vec::new(),
            compression: Vec::new(),
            stale: false,
            pressure: ContextPressureLevel::Green,
        }
    }
}

impl Default for CaseHealthState {
    fn default() -> Self {
        Self {
            context_valid: true,
            pressure: ContextPressureLevel::Green,
            recent_faults: 0,
        }
    }
}

impl Default for CaseBudgetState {
    fn default() -> Self {
        Self {
            max_observation_tokens: 2048,
            max_graph_tokens: 1024,
            max_batch_files: 20,
        }
    }
}

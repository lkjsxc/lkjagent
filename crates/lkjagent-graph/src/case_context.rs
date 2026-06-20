use crate::policy::ContextPressureLevel;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphContextState {
    pub selected_packages: Vec<String>,
    pub loaded_packages: Vec<String>,
    pub bindings: Vec<ContextBinding>,
    pub stale: bool,
    pub pressure: ContextPressureLevel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextBinding {
    pub package: String,
    pub reason: String,
    pub priority: String,
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

impl GraphContextState {
    pub fn new(selected_packages: Vec<String>) -> Self {
        Self {
            selected_packages,
            loaded_packages: Vec::new(),
            bindings: Vec::new(),
            stale: false,
            pressure: ContextPressureLevel::Green,
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

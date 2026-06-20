use crate::model::GraphNodeId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanState {
    pub objective: String,
    pub reason: String,
    pub steps: Vec<PlanStep>,
    pub checks: Vec<VerificationCheck>,
    pub ready: bool,
    pub active_step: Option<StepId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanStep {
    pub id: StepId,
    pub title: String,
    pub rationale: String,
    pub status: StepStatus,
    pub node: GraphNodeId,
    pub target_paths: Vec<String>,
    pub required_evidence: Vec<String>,
    pub verification: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationCheck {
    pub id: String,
    pub command: String,
    pub status: CheckStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepId(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepStatus {
    Pending,
    Active,
    Blocked,
    Done,
    Skipped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckStatus {
    Pending,
    Passed,
    Failed,
    NotRun,
}

impl PlanState {
    pub fn empty(objective: impl Into<String>) -> Self {
        Self {
            objective: objective.into(),
            reason: String::new(),
            steps: Vec::new(),
            checks: Vec::new(),
            ready: false,
            active_step: None,
        }
    }

    pub fn summary_text(&self) -> String {
        if self.steps.is_empty() {
            return "plan=missing".to_string();
        }
        let steps = self
            .steps
            .iter()
            .map(|step| format!("{}:{}", step.id.0, step.title))
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "objective={}\nreason={}\nsteps=\n{steps}",
            self.objective, self.reason
        )
    }

    pub fn active_step_title(&self) -> String {
        self.steps
            .iter()
            .find(|step| matches!(step.status, StepStatus::Active))
            .or_else(|| self.steps.first())
            .map_or_else(
                || "plan: create structured plan".to_string(),
                |step| step.title.clone(),
            )
    }
}

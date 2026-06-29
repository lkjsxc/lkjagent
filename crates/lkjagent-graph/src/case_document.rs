#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentState {
    pub root: String,
    pub kind: String,
    pub requested_count: Option<usize>,
    pub count_mode: CountModeState,
    pub topology_status: TopologyStatus,
    pub readme_index_status: TopologyStatus,
    pub coverage_map_status: TopologyStatus,
    pub sequence_ledger_status: TopologyStatus,
    pub audit_status: TopologyStatus,
    pub file_budget: Option<usize>,
    pub exact_title: Option<String>,
    pub profile: Option<String>,
    pub requested_scale: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountModeState {
    Exact,
    Approximate,
    Unspecified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyStatus {
    Missing,
    Planned,
    Present,
    Failed,
}

impl DocumentState {
    pub fn planned(root: impl Into<String>, kind: impl Into<String>) -> Self {
        Self {
            root: root.into(),
            kind: kind.into(),
            requested_count: None,
            count_mode: CountModeState::Unspecified,
            topology_status: TopologyStatus::Planned,
            readme_index_status: TopologyStatus::Missing,
            coverage_map_status: TopologyStatus::Missing,
            sequence_ledger_status: TopologyStatus::Missing,
            audit_status: TopologyStatus::Missing,
            file_budget: None,
            exact_title: None,
            profile: None,
            requested_scale: None,
        }
    }

    pub fn with_identity(
        mut self,
        title: Option<String>,
        profile: Option<String>,
        scale: Option<String>,
    ) -> Self {
        self.exact_title = title;
        self.profile = profile;
        self.requested_scale = scale;
        self
    }
}

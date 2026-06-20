#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultKind {
    Parse,
    Tool,
    Repeat,
    Endpoint,
    Context,
    Budget,
    Verification,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RecoveryState {
    pub parse_failures: u8,
    pub tool_failures: u8,
    pub repeat_failures: u8,
    pub endpoint_failures: u8,
    pub context_failures: u8,
    pub budget_failures: u8,
    pub verification_failures: u8,
    pub last_failed_action_fingerprint: Option<String>,
    pub strategy: Option<String>,
    pub ladder_position: u8,
    pub escalated: bool,
    pub history: Vec<RecoveryRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryRecord {
    pub kind: FaultKind,
    pub summary: String,
    pub action_fingerprint: Option<String>,
}

impl RecoveryState {
    pub fn count(&self, kind: FaultKind) -> u8 {
        match kind {
            FaultKind::Parse => self.parse_failures,
            FaultKind::Tool => self.tool_failures,
            FaultKind::Repeat => self.repeat_failures,
            FaultKind::Endpoint => self.endpoint_failures,
            FaultKind::Context => self.context_failures,
            FaultKind::Budget => self.budget_failures,
            FaultKind::Verification => self.verification_failures,
        }
    }
}

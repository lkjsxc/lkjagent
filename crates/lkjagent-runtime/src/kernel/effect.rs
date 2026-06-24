use crate::kernel::snapshot::ToolName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeEffectCommand {
    CompactNow,
    WaitClosedIdle,
    DeferMaintenance,
    RecordBlockedHandoff,
    RefreshStatus,
    DeterministicInspection { tool: ToolName },
}

impl RuntimeEffectCommand {
    pub fn requires_model_content(&self) -> bool {
        false
    }

    pub fn tool_name(&self) -> Option<&ToolName> {
        match self {
            Self::DeterministicInspection { tool } => Some(tool),
            Self::CompactNow
            | Self::WaitClosedIdle
            | Self::DeferMaintenance
            | Self::RecordBlockedHandoff
            | Self::RefreshStatus => None,
        }
    }
}

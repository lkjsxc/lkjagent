use crate::kernel::decision::{DecisionInvariantError, RuntimeDecision, RuntimeMission};
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

pub(crate) fn attach_runtime_effect(
    decision: RuntimeDecision,
    mission: RuntimeMission,
) -> Result<RuntimeDecision, DecisionInvariantError> {
    match mission {
        RuntimeMission::HardRuntimeCompaction => {
            decision.with_runtime_effect(RuntimeEffectCommand::CompactNow)
        }
        RuntimeMission::ClosedIdle => {
            decision.with_runtime_effect(RuntimeEffectCommand::WaitClosedIdle)
        }
        _ => Ok(decision),
    }
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

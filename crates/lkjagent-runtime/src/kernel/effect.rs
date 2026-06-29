use crate::kernel::decision::{
    ActionTemplate, DecisionInvariantError, RuntimeDecision, RuntimeDecisionKind, RuntimeMission,
};
use crate::kernel::snapshot::ToolName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeEffectCommand {
    CompactNow,
    WaitClosedIdle,
    DeferMaintenance,
    RecordMaintenanceCooldown,
    RecordBlockedHandoff,
    RefreshStatus,
    ExportModelLog,
    PauseProvider,
    CloseCase,
    DeterministicInspection { tool: ToolName },
}

pub(crate) fn attach_runtime_effect(
    mut decision: RuntimeDecision,
    mission: RuntimeMission,
) -> Result<RuntimeDecision, DecisionInvariantError> {
    match mission {
        RuntimeMission::HardRuntimeCompaction => {
            return decision.with_runtime_effect(RuntimeEffectCommand::CompactNow);
        }
        RuntimeMission::ClosedIdle => {
            return decision.with_runtime_effect(RuntimeEffectCommand::WaitClosedIdle);
        }
        _ => {}
    }
    if decision.kind != RuntimeDecisionKind::RuntimeEffect {
        return Ok(decision);
    }
    decision.runtime_effect = Some(runtime_effect_for_decision(&decision));
    Ok(decision)
}

fn runtime_effect_for_decision(decision: &RuntimeDecision) -> RuntimeEffectCommand {
    if decision.blocked_handoff_plan.is_some() {
        return RuntimeEffectCommand::RecordBlockedHandoff;
    }
    match decision.forced_next_action.as_ref() {
        Some(ActionTemplate::ExactTool { tool, .. }) => {
            RuntimeEffectCommand::DeterministicInspection { tool: tool.clone() }
        }
        _ => RuntimeEffectCommand::RefreshStatus,
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
            | Self::RecordMaintenanceCooldown
            | Self::RefreshStatus
            | Self::ExportModelLog
            | Self::PauseProvider
            | Self::CloseCase
            | Self::RecordBlockedHandoff => None,
        }
    }
}

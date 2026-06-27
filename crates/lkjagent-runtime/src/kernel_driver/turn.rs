use lkjagent_store::error::StoreError;
use rusqlite::Connection;

use crate::kernel::{
    build_snapshot, reduce_with_event_id, render_prompt_frame, DecisionInvariantError,
    PromptRenderError, RuntimeDecisionId, RuntimeDecisionKind, RuntimeEventId,
    SnapshotAdapterError,
};
use crate::kernel_driver::input::KernelTurnInput;
use crate::kernel_driver::persist::{
    persist_decision, persist_event, persist_prompt_frame, persist_runtime_effect, persist_snapshot,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KernelTurnStage {
    PromptFrame,
    RuntimeEffect,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelTurnRecord {
    pub snapshot_id: i64,
    pub event_id: i64,
    pub decision_id: i64,
    pub prompt_frame_id: Option<i64>,
    pub effect_id: Option<i64>,
    pub stage: KernelTurnStage,
    pub decision: crate::kernel::RuntimeDecision,
}

#[derive(Debug)]
pub enum KernelTurnError {
    Snapshot(SnapshotAdapterError),
    Decision(DecisionInvariantError),
    Prompt(PromptRenderError),
    Store(StoreError),
}

pub fn run_kernel_turn(
    conn: &Connection,
    input: KernelTurnInput,
) -> Result<KernelTurnRecord, KernelTurnError> {
    let snapshot = build_snapshot(input.snapshot.clone()).map_err(KernelTurnError::Snapshot)?;
    let snapshot_id = persist_snapshot(conn, &input, &snapshot).map_err(KernelTurnError::Store)?;
    let event_id = persist_event(conn, &input, snapshot_id).map_err(KernelTurnError::Store)?;
    let mut decision = reduce_with_event_id(
        &snapshot,
        RuntimeEventId(event_id as u64),
        input.event.clone(),
    )
    .map_err(KernelTurnError::Decision)?;
    let decision_id = persist_decision(conn, &input, snapshot_id, event_id, &snapshot, &decision)
        .map_err(KernelTurnError::Store)?;
    decision.decision_id = RuntimeDecisionId::Stored(decision_id as u64);
    match decision.kind {
        RuntimeDecisionKind::RuntimeEffect | RuntimeDecisionKind::ClosedIdle => {
            let effect_id = persist_runtime_effect(
                conn,
                decision_id,
                effect_summary(&decision),
                &input.created_at,
            )
            .map_err(KernelTurnError::Store)?;
            Ok(KernelTurnRecord {
                snapshot_id,
                event_id,
                decision_id,
                prompt_frame_id: None,
                effect_id: Some(effect_id),
                stage: KernelTurnStage::RuntimeEffect,
                decision,
            })
        }
        _ => {
            let rendered = render_prompt_frame(&decision).map_err(KernelTurnError::Prompt)?;
            let prompt_frame_id = persist_prompt_frame(conn, &input, decision_id, &rendered)
                .map_err(KernelTurnError::Store)?;
            decision.admission_view = decision
                .admission_view
                .with_current_ids(decision_id.to_string(), prompt_frame_id.to_string());
            Ok(KernelTurnRecord {
                snapshot_id,
                event_id,
                decision_id,
                prompt_frame_id: Some(prompt_frame_id),
                effect_id: None,
                stage: KernelTurnStage::PromptFrame,
                decision,
            })
        }
    }
}

fn effect_summary(decision: &crate::kernel::RuntimeDecision) -> &str {
    match decision.runtime_effect.as_ref() {
        Some(crate::kernel::RuntimeEffectCommand::CompactNow) => "hard_compaction",
        Some(crate::kernel::RuntimeEffectCommand::WaitClosedIdle) => "closed_idle_wait",
        Some(_) => "runtime_effect",
        None => "runtime_effect",
    }
}

impl From<StoreError> for KernelTurnError {
    fn from(error: StoreError) -> Self {
        Self::Store(error)
    }
}

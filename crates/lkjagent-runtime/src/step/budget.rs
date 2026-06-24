use lkjagent_context::model::NoticeKind;
use lkjagent_graph::TransitionDecision;
use lkjagent_store::events::EventKind;

use crate::maintenance::spend_cycle;
use crate::mode::{decide, ActiveMode, RuntimeEvent, RuntimeSnapshot};
use crate::prompt::token_estimate;
use crate::step::budget_render::{checkpoint_notice, decision_label};
use crate::step::frames::{append_notice, result};
use crate::step::{Effect, StepResult};
use crate::task::{spend_turn, RuntimeState, StopReason, TaskState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum BudgetExhaustion {
    Task,
    Maintenance,
}

impl BudgetExhaustion {
    pub(super) fn notice(self) -> &'static str {
        match self {
            Self::Task => "turn budget checkpoint reached; continuing autonomously",
            Self::Maintenance => "maintenance cycle turn budget exhausted",
        }
    }
}

pub(super) fn spend_active_budget(state: &mut RuntimeState) -> Option<BudgetExhaustion> {
    if matches!(state.task, TaskState::Open { .. }) {
        let (task, task_exhausted) = spend_turn(&state.task);
        state.task = task;
        if task_exhausted {
            state.maintenance = None;
            return Some(BudgetExhaustion::Task);
        }
        state.continuation_epoch.turns_used = state.continuation_epoch.turns_used.saturating_add(1);
        return None;
    }
    let (maintenance, maintenance_exhausted) = spend_cycle(&state.maintenance);
    state.maintenance = maintenance;
    if maintenance_exhausted {
        Some(BudgetExhaustion::Maintenance)
    } else {
        None
    }
}

pub(super) fn budget_exhausted_step(
    state: RuntimeState,
    exhausted: BudgetExhaustion,
) -> StepResult {
    if exhausted == BudgetExhaustion::Task {
        return task_checkpoint_step(state);
    }
    maintenance_budget_notice(state, exhausted.notice())
}

pub(super) fn task_checkpoint_step(mut state: RuntimeState) -> StepResult {
    let snapshot = checkpoint_snapshot(&state);
    let decision = decide(&snapshot, RuntimeEvent::TurnBudgetCheckpoint);
    let decision_label = decision_label(&decision);
    let notice = checkpoint_notice(&state, &snapshot, &decision, &decision_label);
    state = append_notice(state, NoticeKind::Budget, &notice);
    state
        .continuation_epoch
        .open_next("turn-budget-checkpoint", &decision_label);
    state.task = TaskState::Open {
        turns_remaining: state.continuation_epoch.checkpoint_turns,
    };
    state.maintenance = None;
    result(
        state,
        vec![Effect::RecordEvent {
            kind: EventKind::Notice,
            content: notice.clone(),
            tokens: token_estimate(&notice) as i64,
        }],
        Some(StopReason::BudgetNotice),
    )
}

fn maintenance_budget_notice(mut state: RuntimeState, notice: &str) -> StepResult {
    state = append_notice(state, NoticeKind::Budget, notice);
    result(
        state,
        vec![Effect::RecordEvent {
            kind: EventKind::Notice,
            content: notice.to_string(),
            tokens: token_estimate(notice) as i64,
        }],
        Some(StopReason::BudgetNotice),
    )
}

fn checkpoint_snapshot(state: &RuntimeState) -> RuntimeSnapshot {
    let (required, missing) = evidence_state(state);
    RuntimeSnapshot {
        active_mode: active_mode(state),
        case_id: state
            .graph
            .as_ref()
            .and_then(|graph| graph.case_id.map(|id| id.to_string())),
        graph_node: state
            .graph
            .as_ref()
            .map(|graph| graph.active_node.0.to_string()),
        graph_phase: state
            .graph
            .as_ref()
            .map(|graph| graph.phase.as_str().to_string()),
        owner_work_exists: matches!(state.task, TaskState::Open { .. }),
        recovery_ladder_active: recovery_active(state),
        context_pressure_active: state.compaction.is_some(),
        maintenance_eligible: false,
        required_evidence: required,
        missing_evidence: missing,
        active_artifact: state
            .graph
            .as_ref()
            .and_then(|graph| graph.document.as_ref().map(|doc| doc.root.clone())),
        last_tool_attempt: state
            .pending_action
            .as_ref()
            .map(|pending| pending.action.tool.clone()),
        repeated_action: state.repeat_faults > 0,
        external_owner_input_required: false,
    }
}

fn evidence_state(state: &RuntimeState) -> (Vec<String>, Vec<String>) {
    let Some(graph) = state.graph.as_ref() else {
        return (Vec::new(), vec!["observation".to_string()]);
    };
    let required = graph.evidence.requirement_ids();
    let missing = match lkjagent_graph::completion_decision(graph) {
        TransitionDecision::Admit { .. } => Vec::new(),
        TransitionDecision::Defer { missing } => missing,
        TransitionDecision::Recover { reason, .. } | TransitionDecision::Refuse { reason } => {
            vec![reason]
        }
    };
    (required, missing)
}

fn active_mode(state: &RuntimeState) -> ActiveMode {
    if recovery_active(state) {
        ActiveMode::Recovery
    } else {
        ActiveMode::OwnerTask
    }
}

fn recovery_active(state: &RuntimeState) -> bool {
    state.parse_faults > 0 || state.repeat_faults > 0 || state.tool_faults > 0
}

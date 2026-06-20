use lkjagent_graph::case_recovery::FaultKind;

use crate::step::fault_wait::RecoveryFault;

pub(super) fn set_graph_fault_count(
    graph: &mut lkjagent_graph::TaskGraphState,
    kind: FaultKind,
    count: u8,
) {
    match kind {
        FaultKind::Parse => graph.recovery.parse_failures = count,
        FaultKind::Params => graph.recovery.param_failures = count,
        FaultKind::Tool => graph.recovery.tool_failures = count,
        FaultKind::Repeat => graph.recovery.repeat_failures = count,
        FaultKind::Endpoint => graph.recovery.endpoint_failures = count,
        FaultKind::Context => graph.recovery.context_failures = count,
        FaultKind::Budget => graph.recovery.budget_failures = count,
        FaultKind::Verification => graph.recovery.verification_failures = count,
    }
}

pub(super) fn fault_kind(fault: RecoveryFault) -> FaultKind {
    match fault {
        RecoveryFault::Parse | RecoveryFault::Payload => FaultKind::Parse,
        RecoveryFault::Params => FaultKind::Params,
        RecoveryFault::Repeat => FaultKind::Repeat,
        RecoveryFault::Tool => FaultKind::Tool,
    }
}

pub(super) fn fault_name(kind: FaultKind) -> &'static str {
    match kind {
        FaultKind::Parse => "parse",
        FaultKind::Params => "params",
        FaultKind::Tool => "tool",
        FaultKind::Repeat => "repeat",
        FaultKind::Endpoint => "endpoint",
        FaultKind::Context => "context",
        FaultKind::Budget => "budget",
        FaultKind::Verification => "verification",
    }
}

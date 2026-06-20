use crate::state::TaskGraphState;

pub(crate) fn recovery_instruction(state: &TaskGraphState) -> &'static str {
    match state.recovery.ladder_position {
        0 => "correct format or parameters, then retry once",
        1 => "inspect graph.next or graph.audit and choose a different native tool",
        2 => "reduce scope and split the active step",
        3 => "route to recovery node and alternate path",
        4 => "use shell only if active node admits recover-by-shell-escape",
        _ => "block this step, replan, and continue independent work",
    }
}

pub(crate) fn compaction_instruction(state: &TaskGraphState) -> &'static str {
    match state.context.pressure {
        crate::policy::ContextPressureLevel::Green => "normal packages",
        crate::policy::ContextPressureLevel::Yellow => "narrow optional package text",
        crate::policy::ContextPressureLevel::Orange => "checkpoint at next phase boundary",
        crate::policy::ContextPressureLevel::Red => "compact before next endpoint call",
        crate::policy::ContextPressureLevel::BlackInvalid => {
            "pause only if context cannot be rebuilt"
        }
    }
}

pub(crate) fn completion_line(state: &TaskGraphState) -> String {
    if state.completion.ready {
        return "ready".to_string();
    }
    state
        .completion
        .refusal_reason
        .clone()
        .unwrap_or_else(|| "missing typed evidence".to_string())
}

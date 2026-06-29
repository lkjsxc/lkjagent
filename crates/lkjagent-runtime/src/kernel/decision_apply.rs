use crate::kernel::decision::{ActionTemplate, RuntimeDecision};
use crate::kernel::obligation_facts::root_identity_required;
use crate::kernel::snapshot::{RuntimeSnapshot, ToolName};
use crate::kernel::write_contract::content_contract_for;

pub(crate) fn apply_forced_action(snapshot: &RuntimeSnapshot, decision: &mut RuntimeDecision) {
    let Some(ActionTemplate::ExactTool { body, tool }) = &decision.forced_next_action else {
        return;
    };
    if exact_surface_tool(tool.as_str()) {
        decision.admission_view.admitted_tools = vec![tool.clone()];
    } else if !decision.admission_view.admits(tool) {
        decision.admission_view.admitted_tools.push(tool.clone());
    }
    decision
        .admission_view
        .blocked_tools
        .retain(|blocked| blocked != tool);
    if tool.as_str() == "fs.batch_write" {
        decision.content_write_contract = content_contract_for(snapshot);
        if root_identity_required(snapshot) {
            block_tool(decision, "doc.audit");
        }
    } else {
        decision.admission_view.exact_next_action = Some(body.clone());
    }
}

fn exact_surface_tool(tool: &str) -> bool {
    matches!(
        tool,
        "fs.batch_write"
            | "doc.audit"
            | "artifact.plan"
            | "artifact.next"
            | "artifact.audit"
            | "agent.done"
    )
}

fn block_tool(decision: &mut RuntimeDecision, name: &'static str) {
    decision
        .admission_view
        .admitted_tools
        .retain(|tool| tool.as_str() != name);
    if !decision
        .admission_view
        .blocked_tools
        .iter()
        .any(|tool| tool.as_str() == name)
    {
        decision
            .admission_view
            .blocked_tools
            .push(ToolName::from_static(name));
    }
}

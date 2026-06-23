use super::join::join_or_none;
use super::state::AuthorityAdmissionView;

pub fn authority_policy_refusal(tool: &str, view: &AuthorityAdmissionView) -> Option<String> {
    if admitted(tool, view) {
        return None;
    }
    Some(format!(
        "authority refused {tool}\ndecision_id={}\nmission={}\nnode={}\nreason={}\nadmitted_tools={}\nmissing_evidence={}\nvalid_example:\n{}",
        view.decision_id,
        view.active_mission,
        view.active_node,
        reason(tool, view),
        join_or_none(&view.admitted_tools),
        join_or_none(&view.missing_evidence),
        view.exact_valid_example
    ))
}

fn admitted(tool: &str, view: &AuthorityAdmissionView) -> bool {
    if tool == "agent.done" {
        return view.completion_allowed
            && !view.blocked_tools.iter().any(|blocked| blocked == tool);
    }
    if tool == "shell.run" && !view.shell_allowed {
        return false;
    }
    view.admitted_tools.iter().any(|allowed| allowed == tool)
        && !view.blocked_tools.iter().any(|blocked| blocked == tool)
}

fn reason(tool: &str, view: &AuthorityAdmissionView) -> String {
    if tool == "agent.done" && !view.completion_allowed {
        return "completion not admitted by authority decision".to_string();
    }
    if tool == "shell.run" && !view.shell_allowed {
        return "shell not admitted by authority decision".to_string();
    }
    if let Some(route) = view.recovery_route.as_ref() {
        return format!("tool is not admitted by authority decision; recovery_route={route}");
    }
    "tool is not admitted by authority decision".to_string()
}

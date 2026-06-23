mod support;

use lkjagent_tools::dispatch::{dispatch, AuthorityAdmissionView, EffectivePolicy};
use lkjagent_tools::observe::OutputKind;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn authority_view_blocks_tool_even_when_effective_policy_allows_it() -> TestResult<()> {
    let workspace = temp_workspace("authority-view-block")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    state.effective_policy = Some(owner_policy(vec!["graph.state"]));
    state.authority_view = Some(authority_view(
        vec!["workspace.summary"],
        vec!["graph.state"],
        false,
    ));

    let refused = dispatch(&action("graph.state", &[]), &runtime, &mut conn, &mut state);

    assert!(matches!(refused.kind, OutputKind::Notice { .. }));
    assert!(refused.content.contains("authority refused graph.state"));
    assert!(refused.content.contains("decision_id=decision-1"));
    assert!(!refused.content.contains("effective policy refused"));
    Ok(())
}

#[test]
fn authority_view_blocks_done_until_completion_is_allowed() -> TestResult<()> {
    let workspace = temp_workspace("authority-view-done")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    state.effective_policy = Some(owner_policy(vec!["agent.done"]));
    state.authority_view = Some(authority_view(
        vec!["artifact.next"],
        vec!["agent.done"],
        false,
    ));

    let refused = dispatch(
        &action("agent.done", &[("summary", "finished")]),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(matches!(refused.kind, OutputKind::Notice { .. }));
    assert!(refused.content.contains("completion not admitted"));
    assert!(refused.content.contains("artifact-readiness"));
    Ok(())
}

fn owner_policy(allowed: Vec<&str>) -> EffectivePolicy {
    EffectivePolicy {
        mode: "OwnerTask".to_string(),
        allowed_tools: allowed.into_iter().map(str::to_string).collect(),
        blocked_tools: Vec::new(),
        shell_allowed: false,
        completion_allowed: true,
        reason: "effective policy would allow the tool".to_string(),
        preferred_next_action: "graph.state".to_string(),
    }
}

fn authority_view(
    admitted: Vec<&str>,
    blocked: Vec<&str>,
    completion_allowed: bool,
) -> AuthorityAdmissionView {
    AuthorityAdmissionView {
        decision_id: "decision-1".to_string(),
        case_id: "case-1".to_string(),
        authority_fingerprint: "fp-1".to_string(),
        active_mission: "owner_execution".to_string(),
        active_node: "execute".to_string(),
        admitted_tools: admitted.into_iter().map(str::to_string).collect(),
        blocked_tools: blocked.into_iter().map(str::to_string).collect(),
        shell_allowed: false,
        completion_allowed,
        missing_evidence: vec!["artifact-readiness".to_string()],
        recovery_route: None,
        exact_valid_example: "<act>\n<tool>artifact.next</tool>\n</act>".to_string(),
    }
}

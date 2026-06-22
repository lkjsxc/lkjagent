mod support;

use lkjagent_tools::dispatch::{dispatch, EffectivePolicy};
use lkjagent_tools::observe::OutputKind;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn repeat_refusal_names_alternate_effective_policy_action() -> TestResult<()> {
    let workspace = temp_workspace("repeat-refusal-effective")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    state.effective_policy = Some(recovery_policy());
    let repeated = action("graph.recover", &[]);

    let first = dispatch(&repeated, &runtime, &mut conn, &mut state);
    let refused = dispatch(&repeated, &runtime, &mut conn, &mut state);

    assert!(matches!(first.kind, OutputKind::Observation { .. }));
    assert!(matches!(refused.kind, OutputKind::Notice { .. }));
    assert!(refused.content.contains("active_mode=Recovery"));
    assert!(refused
        .content
        .contains("next_action_must_change_shape=true"));
    assert!(refused.content.contains("forbidden_tool=graph.recover"));
    assert!(refused
        .content
        .contains("preferred_next_action=artifact.next"));
    assert!(refused.content.contains("<tool>artifact.next</tool>"));
    Ok(())
}

fn recovery_policy() -> EffectivePolicy {
    EffectivePolicy {
        mode: "Recovery".to_string(),
        allowed_tools: vec!["graph.recover".to_string(), "artifact.next".to_string()],
        blocked_tools: vec!["agent.done".to_string()],
        shell_allowed: false,
        completion_allowed: false,
        reason: "repeat recovery must change action shape".to_string(),
        preferred_next_action: "graph.recover".to_string(),
    }
}

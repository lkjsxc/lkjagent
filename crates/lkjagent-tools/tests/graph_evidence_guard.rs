mod support;

use lkjagent_tools::dispatch::{dispatch, EffectivePolicy};
use lkjagent_tools::observe::OutputKind;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn graph_evidence_rejects_claims_after_failed_tool_output() -> TestResult<()> {
    let workspace = temp_workspace("graph-evidence-failed-tool")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    state.effective_policy = Some(owner_policy(vec!["graph.evidence"]));

    let shell = dispatch(
        &action("shell.run", &[("command", "test -f out.txt")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(matches!(shell.kind, OutputKind::Notice { .. }));

    let evidence = dispatch(
        &action(
            "graph.evidence",
            &[("kind", "verification"), ("summary", "shell.run exited 0")],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(is_error(&evidence));
    assert!(evidence
        .content
        .contains("latest tool output cannot support"));
    assert!(evidence.content.contains("previous_tool=shell.run"));
    assert!(state.graph_evidence.is_empty());
    Ok(())
}

fn owner_policy(allowed: Vec<&str>) -> EffectivePolicy {
    EffectivePolicy {
        mode: "OwnerTask".to_string(),
        allowed_tools: allowed.into_iter().map(str::to_string).collect(),
        blocked_tools: vec!["shell.run".to_string()],
        shell_allowed: false,
        completion_allowed: false,
        reason: "runtime authority".to_string(),
        preferred_next_action: "record graph.plan".to_string(),
    }
}

fn is_error(output: &lkjagent_tools::dispatch::DispatchOutput) -> bool {
    matches!(
        &output.kind,
        OutputKind::Observation { status } if status == "error"
    )
}

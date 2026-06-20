mod support;

use lkjagent_protocol::{Action, Param};
use lkjagent_tools::dispatch::{dispatch, dispatch_with_text};
use lkjagent_tools::observe::OutputKind;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn graph_tools_report_state_and_record_evidence() -> TestResult<()> {
    let workspace = temp_workspace("graph")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    state.graph_state = Some("case=1\nphase=planning\nmissing=observation".to_string());

    let shown = dispatch(&action("graph.state", &[]), &runtime, &mut conn, &mut state);
    assert!(matches!(shown.kind, OutputKind::Observation { .. }));
    assert!(shown.content.contains("phase=planning"));

    let recorded = dispatch(
        &action(
            "graph.evidence",
            &[
                ("kind", "observation"),
                ("summary", "read README"),
                ("path", "README.md"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(recorded.content.contains("graph evidence recorded"));
    assert_eq!(state.graph_evidence.len(), 1);
    assert_eq!(state.graph_evidence[0].path.as_deref(), Some("README.md"));
    Ok(())
}

#[test]
fn agent_done_refusal_points_to_missing_graph_evidence() -> TestResult<()> {
    let workspace = temp_workspace("graph-done-refusal")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    state.graph_state = Some("case=1\nphase=execution".to_string());
    state.graph_completion_ready = false;
    state.graph_missing = vec!["document-structure".to_string()];

    let refused = dispatch(
        &action("agent.done", &[("summary", "finished")]),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(is_error(&refused));
    assert!(refused
        .content
        .contains("partial_handoff=blocked-with-evidence"));
    assert!(refused.content.contains("failed_gate=completion"));
    assert!(refused.content.contains("existing_graph=case=1"));
    assert!(refused.content.contains("<kind>document-structure</kind>"));
    Ok(())
}

#[test]
fn dispatcher_reports_validation_and_repeat_notices() -> TestResult<()> {
    let workspace = temp_workspace("dispatch")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    let bad = Action::new(
        "fs.read",
        vec![Param::new("bogus", "x"), Param::new("bogus", "y")],
    );
    let validation = dispatch(&bad, &runtime, &mut conn, &mut state);
    assert!(matches!(validation.kind, OutputKind::Notice { .. }));
    assert!(validation.content.contains("action params refused"));
    assert!(validation.content.contains("duplicate=bogus"));
    assert!(validation.content.contains("missing=path"));
    assert!(validation.content.contains("unknown=bogus"));
    assert!(validation.content.contains("valid_example:"));

    let unknown = dispatch(
        &Action::new("think", Vec::new()),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(matches!(unknown.kind, OutputKind::Notice { .. }));
    assert!(unknown.content.contains("valid tools"));

    let ask = action("agent.ask", &[("question", "Continue?")]);
    let first = dispatch_with_text(&ask, "same-act", &runtime, &mut conn, &mut state);
    let second = dispatch_with_text(&ask, "same-act", &runtime, &mut conn, &mut state);
    assert!(first.content.contains("waiting"));
    assert!(matches!(second.kind, OutputKind::Notice { .. }));
    assert!(second.content.contains("repeat action refused"));
    Ok(())
}

fn is_error(output: &lkjagent_tools::dispatch::DispatchOutput) -> bool {
    matches!(
        &output.kind,
        OutputKind::Observation { status } if status == "error"
    )
}

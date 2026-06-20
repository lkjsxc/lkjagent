mod support;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn agent_ask_refuses_internal_tool_schema_question() -> TestResult<()> {
    let output = ask("What are valid kinds for graph.note?")?;

    assert!(is_error(&output));
    assert!(output.content.contains("owner question refused"));
    assert!(output.content.contains("tool schema questions"));
    assert!(output.content.contains("<tool>graph.note</tool>"));
    Ok(())
}

#[test]
fn agent_ask_refuses_maintenance_transcript_question() -> TestResult<()> {
    let output = ask("What recent transcript spans should be distilled?")?;

    assert!(is_error(&output));
    assert!(output.content.contains("maintenance must inspect records"));
    assert!(output.content.contains("<tool>memory.find</tool>"));
    Ok(())
}

#[test]
fn agent_ask_refuses_recovery_how_to_use_tool_question() -> TestResult<()> {
    let output = ask("How should I create the story file given fs.write is blocked?")?;

    assert!(is_error(&output));
    assert!(output.content.contains("tool recovery must choose"));
    assert!(output.content.contains("<tool>doc.scaffold</tool>"));
    Ok(())
}

#[test]
fn agent_ask_admits_true_owner_scope_choice() -> TestResult<()> {
    let output = ask("Should the cookbook focus on sourdough or quick breads?")?;

    assert!(!is_error(&output));
    assert!(output.content.contains("waiting"));
    assert!(output.content.contains("sourdough or quick breads"));
    Ok(())
}

fn ask(question: &str) -> TestResult<lkjagent_tools::dispatch::DispatchOutput> {
    let workspace = temp_workspace("owner-question")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    Ok(dispatch(
        &action("agent.ask", &[("question", question)]),
        &runtime,
        &mut conn,
        &mut state,
    ))
}

fn is_error(output: &lkjagent_tools::dispatch::DispatchOutput) -> bool {
    matches!(
        &output.kind,
        lkjagent_tools::observe::OutputKind::Observation { status } if status == "error"
    )
}

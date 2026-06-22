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
fn agent_ask_refuses_stale_memory_row_question() -> TestResult<()> {
    let output = ask("Which memory rows are stale and should be pruned?")?;

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
fn agent_ask_refuses_doc_scaffold_usage_question() -> TestResult<()> {
    let output = ask("How do I use doc.scaffold when the recovery node blocks writes?")?;

    assert!(is_error(&output));
    assert!(output.content.contains("tool recovery must choose"));
    assert!(output.content.contains("<tool>doc.scaffold</tool>"));
    Ok(())
}

#[test]
fn agent_ask_refuses_internal_graph_recovery_question() -> TestResult<()> {
    let output = ask("Which internal graph recovery strategy should the tool use?")?;

    assert!(is_error(&output));
    assert!(output
        .content
        .contains("graph recovery strategy is runtime-owned"));
    assert!(output.content.contains("<tool>graph.recover</tool>"));
    Ok(())
}

#[test]
fn agent_ask_refuses_vague_need_input_question() -> TestResult<()> {
    let output = ask("Need input?")?;

    assert!(is_error(&output));
    assert!(output.content.contains("concrete external"));
    assert!(output.content.contains("<tool>workspace.summary</tool>"));
    Ok(())
}

#[test]
fn agent_ask_refuses_audit_and_repair_questions() -> TestResult<()> {
    let audit = ask("Should I run doc.audit before completion?")?;
    let repair = ask("Should I repair placeholder weak paths?")?;

    assert!(is_error(&audit));
    assert!(audit.content.contains("audits are runtime-owned"));
    assert!(audit.content.contains("<tool>doc.audit</tool>"));
    assert!(is_error(&repair));
    assert!(repair.content.contains("artifact repair is runtime-owned"));
    assert!(repair.content.contains("<tool>artifact.next</tool>"));
    Ok(())
}

#[test]
fn agent_ask_refuses_compaction_preemption_and_completion_questions() -> TestResult<()> {
    let compaction = ask("Should I perform compaction now?")?;
    let preempt = ask("Should I preempt maintenance for queued owner work?")?;
    let completion = ask("Should I refuse completion because evidence is missing?")?;

    assert!(is_error(&compaction));
    assert!(compaction.content.contains("compaction is runtime-owned"));
    assert!(is_error(&preempt));
    assert!(preempt
        .content
        .contains("maintenance preemption is runtime-owned"));
    assert!(is_error(&completion));
    assert!(completion
        .content
        .contains("completion refusal is runtime-owned"));
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

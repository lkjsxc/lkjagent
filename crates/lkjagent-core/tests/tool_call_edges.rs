use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision};
use lkjagent_core::runtime_tool_call::{parse_model_value, ModelValue, ToolCallError};
use lkjagent_core::runtime_tool_catalog::direct_tool_view;

#[test]
fn contract_tables_final_is_complete_text_not_json() -> Result<(), String> {
    let parsed = parse_model_value(
        "<final><message>{&quot;status&quot;:&quot;done&quot;}</message></final>",
        &decision(OutputEnvelope::Message),
    )
    .map_err(|error| format!("final failed: {error:?}"))?;
    assert_eq!(
        parsed,
        ModelValue::Final("{\"status\":\"done\"}".to_string())
    );
    Ok(())
}

#[test]
fn contract_tables_enforces_harness_grammar_phase() {
    let tool = "<tool_call><tool>read_file</tool><input><path>README.md</path></input></tool_call>";
    let final_value = "<final><message>done</message></final>";
    assert_eq!(
        parse_model_value(tool, &decision(OutputEnvelope::Message)),
        Err(ToolCallError::WrongGrammarPhase)
    );
    assert_eq!(
        parse_model_value(final_value, &decision(OutputEnvelope::Action)),
        Err(ToolCallError::WrongGrammarPhase)
    );
}

#[test]
fn contract_tables_bounds_raw_and_final_bytes() {
    assert_eq!(
        parse_model_value(&"x".repeat(16_385), &decision(OutputEnvelope::Action)),
        Err(ToolCallError::TooLarge)
    );
    let raw = format!("<final><message>{}</message></final>", "x".repeat(4097));
    assert_eq!(
        parse_model_value(&raw, &decision(OutputEnvelope::Message)),
        Err(ToolCallError::Bounds)
    );
}

fn decision(envelope: OutputEnvelope) -> RuntimeDecision {
    RuntimeDecision::new(
        "persisted-only",
        "case-1",
        OperationKey("direct".into()),
        direct_tool_view(),
        envelope,
    )
}

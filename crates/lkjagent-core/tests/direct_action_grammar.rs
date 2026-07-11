use lkjagent_core::runtime_decision::{
    OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView, ToolViewEntry,
};
use lkjagent_core::runtime_tool_call::{parse_tool_call, ToolCallError};

#[test]
fn rejects_name_value_argument_wrappers() -> Result<(), String> {
    let raw = "<lkjagent_action><decision_id>dec-1</decision_id><context_fingerprint>ctx-1</context_fingerprint><tool_name>fs.read</tool_name><argument><name>path</name><value>README.md</value></argument></lkjagent_action>";
    let error = match parse_tool_call(raw, &decision()) {
        Ok(_) => return Err("name/value wrapper accepted".to_string()),
        Err(error) => error,
    };
    assert_eq!(error, ToolCallError::UnknownTag("argument".to_string()));
    Ok(())
}

#[test]
fn requires_one_input_block() -> Result<(), String> {
    let raw = "<lkjagent_action><decision_id>dec-1</decision_id><context_fingerprint>ctx-1</context_fingerprint><tool_name>fs.read</tool_name></lkjagent_action>";
    let error = match parse_tool_call(raw, &decision()) {
        Ok(_) => return Err("missing input accepted".to_string()),
        Err(error) => error,
    };
    assert!(
        matches!(error, ToolCallError::ArgsSchemaViolation(message) if message == "missing input")
    );
    Ok(())
}

#[test]
fn rejects_oversized_action_bytes() -> Result<(), String> {
    let raw = "x".repeat(16_385);
    let error = match parse_tool_call(&raw, &decision()) {
        Ok(_) => return Err("oversized action accepted".to_string()),
        Err(error) => error,
    };
    assert!(
        matches!(error, ToolCallError::ArgsSchemaViolation(message) if message == "action too large")
    );
    Ok(())
}

#[test]
fn rejects_oversized_scalar_bytes() -> Result<(), String> {
    let identifier = "d".repeat(4097);
    let raw = format!(
        "<lkjagent_action><decision_id>{identifier}</decision_id><context_fingerprint>ctx-1</context_fingerprint><tool_name>fs.read</tool_name><input><path>README.md</path></input></lkjagent_action>"
    );
    let error = match parse_tool_call(&raw, &decision()) {
        Ok(_) => return Err("oversized scalar accepted".to_string()),
        Err(error) => error,
    };
    assert!(
        matches!(error, ToolCallError::ArgsSchemaViolation(message) if message == "scalar too large for decision_id")
    );
    Ok(())
}

fn decision() -> RuntimeDecision {
    let mut decision = RuntimeDecision::new(
        "dec-1",
        "case-1",
        OperationKey("read".to_string()),
        ToolSetView::new(vec![
            ToolViewEntry::new("fs.read", "read").with_params(vec!["path"], Vec::new())
        ]),
        OutputEnvelope::Action,
    );
    decision.context_frame_fingerprint = "ctx-1".to_string();
    decision
}

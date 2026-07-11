use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision};
use lkjagent_core::runtime_tool_call::{parse_tool_call, ToolCallError};
use lkjagent_core::runtime_tool_catalog::explore_tool_view;

#[test]
fn accepts_japanese_and_large_bounded_values() -> Result<(), String> {
    let japanese = parse_tool_call(
        &action(
            "plan.note",
            &[('s', "保存しました。次の確認を待っています。")],
        ),
        &decision(),
    )
    .map_err(|error| format!("japanese value failed: {error:?}"))?;
    assert_eq!(
        japanese.args["note"],
        "保存しました。次の確認を待っています。"
    );

    let large = "あ".repeat(1365);
    let parsed = parse_tool_call(&action("plan.note", &[('s', &large)]), &decision())
        .map_err(|error| format!("large value failed: {error:?}"))?;
    assert_eq!(parsed.args["note"], large);
    Ok(())
}

#[test]
fn count_values_are_canonical_and_within_persisted_bounds() -> Result<(), String> {
    for value in ["01", "0", "121", "+1", " 1"] {
        let error = match parse_tool_call(&read_count(value), &decision()) {
            Ok(_) => return Err(format!("invalid count accepted: {value}")),
            Err(error) => error,
        };
        assert!(
            matches!(error, ToolCallError::ArgsSchemaViolation(_)),
            "value={value}"
        );
    }
    let parsed = parse_tool_call(&read_count("120"), &decision())
        .map_err(|error| format!("bounded count failed: {error:?}"))?;
    assert_eq!(parsed.args["count"], 120);
    Ok(())
}

#[test]
fn rejects_unbounded_action_values() -> Result<(), String> {
    let too_large = "あ".repeat(1366);
    let error = match parse_tool_call(&action("plan.note", &[('s', &too_large)]), &decision()) {
        Ok(_) => return Err("oversize value should fail".to_string()),
        Err(error) => error,
    };
    assert!(
        matches!(error, ToolCallError::ArgsSchemaViolation(message) if message == "value too large for note")
    );
    Ok(())
}

fn decision() -> RuntimeDecision {
    let mut decision = RuntimeDecision::new(
        "dec-1",
        "case-1",
        OperationKey("explore".into()),
        explore_tool_view(),
        OutputEnvelope::Action,
    );
    decision.context_frame_fingerprint = "ctx-1".into();
    decision
}

fn action(tool: &str, args: &[(char, &str)]) -> String {
    let mut out = format!(
        "<lkjagent_action><decision_id>dec-1</decision_id><context_fingerprint>ctx-1</context_fingerprint><tool_name>{tool}</tool_name>"
    );
    out.push_str("<input>");
    for (name, value) in args {
        let field = field(*name);
        out.push_str(&format!("<{field}>{value}</{field}>"));
    }
    out.push_str("</input></lkjagent_action>");
    out
}

fn read_count(value: &str) -> String {
    format!("<lkjagent_action><decision_id>dec-1</decision_id><context_fingerprint>ctx-1</context_fingerprint><tool_name>fs.read</tool_name><input><path>README.md</path><count>{value}</count></input></lkjagent_action>")
}

fn field(name: char) -> &'static str {
    match name {
        's' => "note",
        _ => "content",
    }
}

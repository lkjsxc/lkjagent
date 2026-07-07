use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision};
use lkjagent_core::runtime_tool_call_v2::{parse_tool_call_v2, ToolCallV2Error};
use lkjagent_core::runtime_tool_catalog::explore_tool_view;

type TestResult<T> = Result<T, String>;

#[test]
fn accepts_valid_multiline_write_action() -> TestResult<()> {
    let parsed = parse_tool_call_v2(
        &action(r#""tool_name":"fs.write","args":{"path":"notes/today.md","content":"one\ntwo"}"#),
        &decision(),
    )
    .map_err(|error| format!("valid action failed: {error:?}"))?;

    assert_eq!(parsed.decision_id, "dec-1");
    assert_eq!(parsed.tool_name, "fs.write");
    assert_eq!(parsed.args["path"], "notes/today.md");
    assert_eq!(parsed.args["content"], "one\ntwo");
    assert_eq!(parsed.context_frame_fingerprint, "ctx-1");
    Ok(())
}

#[test]
fn rejects_duplicate_keys_with_pointer() -> TestResult<()> {
    let err = parse_err(
        &action(r#""tool_name":"fs.write","args":{"path":"notes/a.md","content":{"a":1,"a":2}}"#),
        "duplicate key must fail first",
    )?;

    assert_eq!(err, ToolCallV2Error::DuplicateKey("/args/content/a".into()));
    Ok(())
}

#[test]
fn rejects_prose_and_multiple_envelopes() -> TestResult<()> {
    assert_eq!(
        parse_err("plain response", "no action")?,
        ToolCallV2Error::NoActionFound
    );
    let one = action(r#""tool_name":"finish","args":{"summary":"done"}"#);
    let err = parse_err(&format!("{one}\n{one}"), "two actions")?;
    assert_eq!(err, ToolCallV2Error::MultipleActionsFound);
    Ok(())
}

#[test]
fn rejects_stale_decision_and_unknown_tool() -> TestResult<()> {
    let stale = action_with_decision(
        "dec-old",
        r#""tool_name":"finish","args":{"summary":"done"}"#,
    );
    assert_eq!(
        parse_err(&stale, "stale decision")?,
        ToolCallV2Error::DecisionMismatch
    );

    let err = parse_err(
        &action(r#""tool_name":"calendar.send","args":{}"#),
        "unknown tool",
    )?;
    assert_eq!(err, ToolCallV2Error::ToolUnknown);
    Ok(())
}

#[test]
fn rejects_unknown_top_level_and_args() -> TestResult<()> {
    let top = action(r#""tool_name":"finish","args":{"summary":"done"},"extra":1"#);
    assert_eq!(
        parse_err(&top, "unknown top level")?,
        ToolCallV2Error::UnknownTopLevel("extra".into())
    );

    let err = parse_err(
        &action(r#""tool_name":"finish","args":{"summary":"done","other":"x"}"#),
        "unknown arg",
    )?;
    assert!(matches!(err, ToolCallV2Error::ArgsSchemaViolation(_)));
    Ok(())
}

#[test]
fn rejects_missing_required_and_wrong_primitive() -> TestResult<()> {
    let err = parse_err(
        &action(r#""tool_name":"fs.write","args":{"path":"a.md"}"#),
        "missing content",
    )?;
    assert!(matches!(err, ToolCallV2Error::ArgsSchemaViolation(_)));

    let err = parse_err(
        &action(r#""tool_name":"fs.read","args":{"path":"a.md","count":"ten"}"#),
        "count must be numeric",
    )?;
    assert!(matches!(err, ToolCallV2Error::ArgsSchemaViolation(_)));
    Ok(())
}

fn parse_err(raw: &str, label: &str) -> TestResult<ToolCallV2Error> {
    match parse_tool_call_v2(raw, &decision()) {
        Ok(_) => Err(format!("{label}: parse unexpectedly succeeded")),
        Err(error) => Ok(error),
    }
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

fn action(fields: &str) -> String {
    action_with_decision("dec-1", fields)
}

fn action_with_decision(decision_id: &str, fields: &str) -> String {
    format!(
        "<lkjagent_action_v2>{{\n  \"schema_version\":\"lkjagent.tool_call.v2\",\n  \"decision_id\":\"{decision_id}\",\n  {fields},\n  \"context_frame_fingerprint\":\"ctx-1\"\n}}</lkjagent_action_v2>"
    )
}

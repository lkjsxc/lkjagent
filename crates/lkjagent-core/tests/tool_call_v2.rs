use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision};
use lkjagent_core::runtime_tool_call_v2::{parse_tool_call_v2, ToolCallV2Error};
use lkjagent_core::runtime_tool_catalog::explore_tool_view;

#[test]
fn accepts_valid_multiline_write_action() {
    let parsed = parse_tool_call_v2(
        &action(r#""tool_name":"fs.write","args":{"path":"notes/today.md","content":"one\ntwo"}"#),
        &decision(),
    )
    .expect("valid v2 action");

    assert_eq!(parsed.decision_id, "dec-1");
    assert_eq!(parsed.tool_name, "fs.write");
    assert_eq!(parsed.args["path"], "notes/today.md");
    assert_eq!(parsed.args["content"], "one\ntwo");
    assert_eq!(parsed.context_frame_fingerprint, "ctx-1");
}

#[test]
fn rejects_duplicate_keys_with_pointer() {
    let err = parse_tool_call_v2(
        &action(r#""tool_name":"fs.write","args":{"path":"notes/a.md","content":{"a":1,"a":2}}"#),
        &decision(),
    )
    .expect_err("duplicate key must fail first");

    assert_eq!(err, ToolCallV2Error::DuplicateKey("/args/content/a".into()));
}

#[test]
fn rejects_prose_and_multiple_envelopes() {
    assert_eq!(
        parse_tool_call_v2("plain response", &decision()).expect_err("no action"),
        ToolCallV2Error::NoActionFound
    );
    let one = action(r#""tool_name":"finish","args":{"summary":"done"}"#);
    let err = parse_tool_call_v2(&format!("{one}\n{one}"), &decision()).expect_err("two actions");
    assert_eq!(err, ToolCallV2Error::MultipleActionsFound);
}

#[test]
fn rejects_stale_decision_and_unknown_tool() {
    let stale = action_with_decision(
        "dec-old",
        r#""tool_name":"finish","args":{"summary":"done"}"#,
    );
    assert_eq!(
        parse_tool_call_v2(&stale, &decision()).expect_err("stale decision"),
        ToolCallV2Error::DecisionMismatch
    );

    let err = parse_tool_call_v2(
        &action(r#""tool_name":"calendar.send","args":{}"#),
        &decision(),
    )
    .expect_err("unknown tool");
    assert_eq!(err, ToolCallV2Error::ToolUnknown);
}

#[test]
fn rejects_unknown_top_level_and_args() {
    let top = action(r#""tool_name":"finish","args":{"summary":"done"},"extra":1"#);
    assert_eq!(
        parse_tool_call_v2(&top, &decision()).expect_err("unknown top level"),
        ToolCallV2Error::UnknownTopLevel("extra".into())
    );

    let err = parse_tool_call_v2(
        &action(r#""tool_name":"finish","args":{"summary":"done","other":"x"}"#),
        &decision(),
    )
    .expect_err("unknown arg");
    assert!(matches!(err, ToolCallV2Error::ArgsSchemaViolation(_)));
}

#[test]
fn rejects_missing_required_and_wrong_primitive() {
    let err = parse_tool_call_v2(
        &action(r#""tool_name":"fs.write","args":{"path":"a.md"}"#),
        &decision(),
    )
    .expect_err("missing content");
    assert!(matches!(err, ToolCallV2Error::ArgsSchemaViolation(_)));

    let err = parse_tool_call_v2(
        &action(r#""tool_name":"fs.read","args":{"path":"a.md","count":"ten"}"#),
        &decision(),
    )
    .expect_err("count must be numeric");
    assert!(matches!(err, ToolCallV2Error::ArgsSchemaViolation(_)));
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

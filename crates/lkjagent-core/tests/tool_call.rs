use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision};
use lkjagent_core::runtime_tool_call::{parse_tool_call, ToolCallError};
use lkjagent_core::runtime_tool_catalog::explore_tool_view;

type TestResult<T> = Result<T, String>;

#[test]
fn accepts_valid_multiline_write_action() -> TestResult<()> {
    let parsed = parse_tool_call(
        &action(
            "fs.write",
            &[("path", "notes/today.md"), ("content", "one\ntwo")],
            "dec-1",
            "ctx-1",
        ),
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
fn decodes_entities_and_count_values() -> TestResult<()> {
    let parsed = parse_tool_call(
        &action(
            "fs.read",
            &[("path", "notes/a&amp;b.md"), ("count", "10")],
            "dec-1",
            "ctx-1",
        ),
        &decision(),
    )
    .map_err(|error| format!("entity action failed: {error:?}"))?;
    assert_eq!(parsed.args["path"], "notes/a&b.md");
    assert_eq!(parsed.args["count"], 10);
    Ok(())
}

#[test]
fn rejects_attributes_json_and_duplicate_scalars() -> TestResult<()> {
    assert_eq!(
        parse_err("<lkjagent_action bad=\"1\"></lkjagent_action>", "attr")?,
        ToolCallError::Attribute("lkjagent_action".into())
    );
    assert_eq!(
        parse_err("<lkjagent_action>{}</lkjagent_action>", "json")?,
        ToolCallError::JsonLike
    );
    let duplicate = "<lkjagent_action><decision_id>dec-1</decision_id><decision_id>dec-1</decision_id></lkjagent_action>";
    assert_eq!(
        parse_err(duplicate, "duplicate")?,
        ToolCallError::DuplicateTag("decision_id".into())
    );
    Ok(())
}

#[test]
fn rejects_crossed_tags_unknown_tags_and_duplicate_args() -> TestResult<()> {
    let crossed = "<lkjagent_action><input><path>value</content></input></lkjagent_action>";
    assert_eq!(
        parse_err(crossed, "crossed")?,
        ToolCallError::CrossedTag("path".into())
    );
    let unknown = "<lkjagent_action><extra>x</extra></lkjagent_action>";
    assert_eq!(
        parse_err(unknown, "unknown")?,
        ToolCallError::UnknownTag("extra".into())
    );
    let duplicate = action(
        "fs.read",
        &[("path", "a.md"), ("path", "b.md")],
        "dec-1",
        "ctx-1",
    );
    assert_eq!(
        parse_err(&duplicate, "duplicate arg")?,
        ToolCallError::DuplicateTag("input/path".into())
    );
    Ok(())
}

#[test]
fn rejects_stale_decision_context_and_unknown_tool() -> TestResult<()> {
    assert_eq!(
        parse_err(
            &action("fs.read", &[("path", "a.md")], "dec-old", "ctx-1"),
            "stale"
        )?,
        ToolCallError::DecisionMismatch
    );
    assert_eq!(
        parse_err(
            &action("fs.read", &[("path", "a.md")], "dec-1", "ctx-old"),
            "ctx"
        )?,
        ToolCallError::ContextMismatch
    );
    assert_eq!(
        parse_err(
            &action("calendar.send", &[], "dec-1", "ctx-1"),
            "unknown tool"
        )?,
        ToolCallError::ToolUnknown
    );
    Ok(())
}

#[test]
fn rejects_unknown_missing_and_wrong_primitive_args() -> TestResult<()> {
    let err = parse_err(
        &action(
            "fs.read",
            &[("path", "a.md"), ("other", "x")],
            "dec-1",
            "ctx-1",
        ),
        "unknown arg",
    )?;
    assert!(matches!(err, ToolCallError::ArgsSchemaViolation(_)));
    let err = parse_err(
        &action("fs.write", &[("path", "a.md")], "dec-1", "ctx-1"),
        "missing content",
    )?;
    assert!(matches!(err, ToolCallError::ArgsSchemaViolation(_)));
    let err = parse_err(
        &action(
            "fs.read",
            &[("path", "a.md"), ("count", "ten")],
            "dec-1",
            "ctx-1",
        ),
        "count",
    )?;
    assert!(matches!(err, ToolCallError::ArgsSchemaViolation(_)));
    Ok(())
}

#[test]
fn property_round_trips_ascii_text_values() -> TestResult<()> {
    for idx in 0..64 {
        let content = format!("line {idx} &amp; value &lt;ok&gt;");
        let raw = action(
            "fs.write",
            &[("path", "notes/a.md"), ("content", &content)],
            "dec-1",
            "ctx-1",
        );
        let parsed = parse_tool_call(&raw, &decision()).map_err(|e| format!("{idx}: {e:?}"))?;
        assert_eq!(parsed.args["content"], format!("line {idx} & value <ok>"));
    }
    Ok(())
}

fn parse_err(raw: &str, label: &str) -> TestResult<ToolCallError> {
    match parse_tool_call(raw, &decision()) {
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

fn action(tool: &str, args: &[(&str, &str)], decision_id: &str, context: &str) -> String {
    let mut out = format!(
        "<lkjagent_action><decision_id>{decision_id}</decision_id><context_fingerprint>{context}</context_fingerprint><tool_name>{tool}</tool_name>"
    );
    out.push_str("<input>");
    for (name, value) in args {
        out.push_str(&format!("<{name}>{value}</{name}>"));
    }
    out.push_str("</input></lkjagent_action>");
    out
}

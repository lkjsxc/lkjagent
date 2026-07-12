use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision};
use lkjagent_core::runtime_tool_call::{parse_tool_call, ToolCallError};
use lkjagent_core::runtime_tool_catalog::direct_tool_view;

type TestResult = Result<(), String>;

#[test]
fn contract_tables_accept_descriptor_order_and_text() -> TestResult {
    let raw = call(
        "edit_file",
        &[
            ("path", "notes/today.md"),
            ("old_text", "old &amp; exact"),
            ("new_text", "{\"replacement\":true}"),
        ],
    );
    let parsed = parse_tool_call(&raw, &action_decision())
        .map_err(|error| format!("valid call failed: {error:?}"))?;
    assert_eq!(parsed.tool_name, "edit_file");
    assert_eq!(parsed.args[0].1, "notes/today.md");
    assert_eq!(parsed.args[1].1, "old & exact");
    assert_eq!(parsed.args[2].1, "{\"replacement\":true}");
    Ok(())
}

#[test]
fn contract_tables_validate_counts_paths_and_bounds() -> TestResult {
    let valid = call(
        "read_file",
        &[("path", "README.md"), ("offset", "0"), ("count", "120")],
    );
    assert!(parse_tool_call(&valid, &action_decision()).is_ok());
    assert_eq!(
        parse_error(&call("read_file", &[("path", "../secret")]))?,
        ToolCallError::UnsafePath
    );
    for value in ["01", "121"] {
        let raw = call("read_file", &[("path", "README.md"), ("count", value)]);
        assert_eq!(parse_error(&raw)?, ToolCallError::ValueClass);
    }
    let huge = "x".repeat(1025);
    assert_eq!(
        parse_error(&call("read_file", &[("path", &huge)]))?,
        ToolCallError::Bounds
    );
    Ok(())
}

#[test]
fn contract_tables_reject_hidden_missing_unknown_and_order() -> TestResult {
    assert_eq!(
        parse_error(&call("shell.run", &[("command", "pwd")]))?,
        ToolCallError::HiddenTool
    );
    assert_eq!(
        parse_error(&call("read_file", &[]))?,
        ToolCallError::MissingField
    );
    assert_eq!(
        parse_error(&call("read_file", &[("path", "a"), ("extra", "x")]))?,
        ToolCallError::UnknownTag
    );
    let wrong = call(
        "read_file",
        &[("path", "a"), ("count", "1"), ("offset", "0")],
    );
    assert_eq!(parse_error(&wrong)?, ToolCallError::FieldOrder);
    Ok(())
}

fn parse_error(raw: &str) -> Result<ToolCallError, String> {
    parse_tool_call(raw, &action_decision())
        .map_or_else(Ok, |_| Err("accepted invalid call".into()))
}

fn action_decision() -> RuntimeDecision {
    RuntimeDecision::new(
        "persisted-only",
        "case-1",
        OperationKey("direct.modify".into()),
        direct_tool_view(),
        OutputEnvelope::Action,
    )
}

fn call(tool: &str, fields: &[(&str, &str)]) -> String {
    let mut raw = format!("<tool_call><tool>{tool}</tool><input>");
    for (name, value) in fields {
        raw.push_str(&format!("<{name}>{value}</{name}>"));
    }
    raw.push_str("</input></tool_call>");
    raw
}

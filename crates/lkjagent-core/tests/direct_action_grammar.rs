use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision};
use lkjagent_core::runtime_tool_call::{parse_model_value, ToolCallError};
use lkjagent_core::runtime_tool_catalog::direct_tool_view;

#[test]
fn contract_tables_reject_forbidden_xml_forms() -> Result<(), String> {
    let cases = [
        ("<tool_call a='1'></tool_call>", ToolCallError::Attribute),
        ("<tool_call/>", ToolCallError::SelfClosing),
        (
            "<!--x--><tool_call></tool_call>",
            ToolCallError::ForbiddenSyntax,
        ),
        ("<![CDATA[x]]>", ToolCallError::ForbiddenSyntax),
        (
            "<?xml version='1.0'?><tool_call></tool_call>",
            ToolCallError::ForbiddenSyntax,
        ),
        (
            "<tool_call><tool>x</tool></final>",
            ToolCallError::CrossedTag,
        ),
        ("<tool_call><tool>x</tool>", ToolCallError::UnclosedTag),
    ];
    for (raw, expected) in cases {
        assert_eq!(error(raw)?, expected, "raw={raw}");
    }
    Ok(())
}

#[test]
fn contract_tables_reject_roots_prose_and_legacy_actions() -> Result<(), String> {
    let cases = [
        ("prose", ToolCallError::MissingRoot),
        ("prose <tool_call></tool_call>", ToolCallError::MissingRoot),
        (
            "<lkjagent_action></lkjagent_action>",
            ToolCallError::UnknownRoot,
        ),
        ("{\"tool\":\"read_file\"}", ToolCallError::MissingRoot),
        (
            "<tool_call></tool_call><tool_call></tool_call>",
            ToolCallError::MultipleRoots,
        ),
        (
            "<tool_call><decision_id>x</decision_id><input></input></tool_call>",
            ToolCallError::UnknownTag,
        ),
    ];
    for (raw, expected) in cases {
        assert_eq!(error(raw)?, expected, "raw={raw}");
    }
    Ok(())
}

#[test]
fn contract_tables_reject_nested_duplicate_and_invalid_entities() -> Result<(), String> {
    let cases = [
        (
            "<tool_call><tool><name>read_file</name></tool><input></input></tool_call>",
            ToolCallError::NestedTag,
        ),
        (
            "<tool_call><tool>read_file</tool><tool>read_file</tool></tool_call>",
            ToolCallError::DuplicateTag,
        ),
        (
            "<tool_call><tool>read_file</tool><input><path>a&bogus;</path></input></tool_call>",
            ToolCallError::BadEntity,
        ),
        (
            "<tool_call><tool>read_file</tool><input><path>a&amp;lt;b</path></input></tool_call>",
            ToolCallError::BadEntity,
        ),
    ];
    for (raw, expected) in cases {
        assert_eq!(error(raw)?, expected, "raw={raw}");
    }
    Ok(())
}

fn error(raw: &str) -> Result<ToolCallError, String> {
    parse_model_value(raw, &decision()).map_or_else(Ok, |_| Err("invalid form accepted".into()))
}

fn decision() -> RuntimeDecision {
    RuntimeDecision::new(
        "decision-is-not-model-data",
        "case-1",
        OperationKey("direct.modify".into()),
        direct_tool_view(),
        OutputEnvelope::Action,
    )
}

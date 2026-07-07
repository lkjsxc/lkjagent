use lkjagent_core::model::StepKind;
use lkjagent_core::parse::{parse_expected, parse_expected_for_decision, ParseFault, ParsedOutput};
use lkjagent_core::runtime_decision::{
    OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView, ToolViewEntry,
};
use lkjagent_core::runtime_tool_call_v2::ToolCallV2Error;

#[test]
fn parses_docs_plan_examples() {
    let raw = "<plan>\nwrite reports/summary.md | Weekly report | words=1000\nexplore | Find the file that names the target root | budget=5\nrespond | Report created paths and checks\n</plan>";
    let parsed = parse_expected(StepKind::Plan, raw);
    assert!(matches!(parsed, Ok(ParsedOutput::Plan(lines)) if lines.len() == 3));
}

#[test]
fn accepts_comma_separated_plan_actions() {
    let raw = "<plan>write notes/a.md | A | words=20, explore | gather facts | budget=3</plan>";
    let parsed = parse_expected(StepKind::Plan, raw);
    assert!(matches!(parsed, Ok(ParsedOutput::Plan(lines)) if lines.len() == 2));
}

#[test]
fn accepts_extended_respond_plan_action() {
    let raw = "<plan>respond | Documentation completed | SUMMARY</plan>";
    let parsed = parse_expected(StepKind::Plan, raw);
    assert!(matches!(parsed, Ok(ParsedOutput::Plan(lines)) if lines.len() == 1));
}

#[test]
fn reports_docs_fault_examples() {
    assert_eq!(
        parse_expected(StepKind::Write, "<message>x</message>"),
        Err(ParseFault::WrongBlock)
    );
    assert_eq!(
        parse_expected(StepKind::Write, "<content>x"),
        Err(ParseFault::Unclosed)
    );
    assert_eq!(
        parse_expected(StepKind::Write, "<content></content>"),
        Err(ParseFault::Empty)
    );
    let action = action("decision-1", "graph.state", "{}", "ctx-1");
    assert_eq!(
        parse_expected(StepKind::Explore, &action),
        Err(ParseFault::UnknownTool)
    );
    assert_eq!(
        parse_expected(StepKind::Write, "note <content>x</content>"),
        Err(ParseFault::WrongBlock)
    );
    assert_eq!(
        parse_expected(StepKind::Write, "<content>x</content> tail"),
        Err(ParseFault::WrongBlock)
    );
}

#[test]
fn decision_action_parser_uses_only_the_decision_tool_view() {
    let decision = RuntimeDecision::new(
        "decision-1",
        "case-1",
        OperationKey("model.call/1".to_string()),
        ToolSetView::new(vec![
            ToolViewEntry::new("fs.read", "read file").with_params(vec!["path"], Vec::new())
        ]),
        OutputEnvelope::Action,
    );
    let read = action("decision-1", "fs.read", r#"{"path":"README.md"}"#, "");
    let shell = action("decision-1", "shell.run", r#"{"command":"pwd"}"#, "");

    assert!(matches!(
        parse_expected_for_decision(&decision, &read),
        Ok(ParsedOutput::Action(action)) if action.tool == "fs.read"
    ));
    assert_eq!(
        parse_expected_for_decision(&decision, &shell),
        Err(ParseFault::UnknownTool)
    );
}

#[test]
fn explore_accepts_only_exact_v2_action_blocks() {
    let finish = action("decision-1", "finish", r#"{"summary":"done"}"#, "ctx-1");
    assert!(matches!(
        parse_expected(StepKind::Explore, &finish),
        Ok(ParsedOutput::Action(action)) if action.tool == "finish"
    ));
    assert_eq!(
        parse_expected(
            StepKind::Explore,
            "<action><tool_name>finish</tool_name></action>",
        ),
        Err(ParseFault::ActionV2(ToolCallV2Error::NoActionFound))
    );
    assert_eq!(
        parse_expected(StepKind::Explore, "<finish>done</finish>"),
        Err(ParseFault::ActionV2(ToolCallV2Error::NoActionFound))
    );
    assert_eq!(
        parse_expected(StepKind::Explore, "<ask>Which file?</ask>"),
        Err(ParseFault::ActionV2(ToolCallV2Error::NoActionFound))
    );
    let missing_tool = action("decision-1", "finish", "{}", "ctx-1");
    assert!(matches!(
        parse_expected(StepKind::Explore, &missing_tool),
        Err(ParseFault::ActionV2(ToolCallV2Error::ArgsSchemaViolation(
            _
        )))
    ));
    let duplicate = "<lkjagent_action_v2>{\"schema_version\":\"lkjagent.tool_call.v2\",\"decision_id\":\"decision-1\",\"tool_name\":\"fs.read\",\"args\":{\"path\":\"a\",\"path\":\"b\"},\"context_frame_fingerprint\":\"ctx-1\"}</lkjagent_action_v2>";
    assert_eq!(
        parse_expected(StepKind::Explore, duplicate),
        Err(ParseFault::ActionV2(ToolCallV2Error::DuplicateKey(
            "/args/path".into(),
        )))
    );
}

fn action(decision_id: &str, tool: &str, args: &str, context: &str) -> String {
    format!(
        "<lkjagent_action_v2>{{\"schema_version\":\"lkjagent.tool_call.v2\",\"decision_id\":\"{decision_id}\",\"tool_name\":\"{tool}\",\"args\":{args},\"context_frame_fingerprint\":\"{context}\"}}</lkjagent_action_v2>"
    )
}

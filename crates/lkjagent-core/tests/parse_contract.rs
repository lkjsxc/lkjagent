use lkjagent_core::model::StepKind;
use lkjagent_core::parse::{parse_expected, parse_expected_for_decision, ParseFault, ParsedOutput};
use lkjagent_core::runtime_decision::{
    OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView, ToolViewEntry,
};
use lkjagent_core::runtime_tool_call::ToolCallError;

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
    let action = action("decision-1", "graph.state", &[], "ctx-1");
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
    let mut decision = RuntimeDecision::new(
        "decision-1",
        "case-1",
        OperationKey("model.call/1".to_string()),
        ToolSetView::new(vec![
            ToolViewEntry::new("fs.read", "read file").with_params(vec!["path"], Vec::new())
        ]),
        OutputEnvelope::Action,
    );
    decision.context_frame_fingerprint = "ctx-1".into();
    let read = action("decision-1", "fs.read", &[("path", "README.md")], "ctx-1");
    let shell = action("decision-1", "shell.run", &[("command", "pwd")], "ctx-1");

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
fn explore_accepts_only_exact_action_blocks() {
    let read = action("decision-1", "fs.read", &[("path", "README.md")], "ctx-1");
    assert!(matches!(
        parse_expected(StepKind::Explore, &read),
        Ok(ParsedOutput::Action(action)) if action.tool == "fs.read"
    ));
    assert_eq!(
        parse_expected(
            StepKind::Explore,
            "<action><tool_name>fs.read</tool_name></action>",
        ),
        Err(ParseFault::Action(ToolCallError::NoActionFound))
    );
    assert_eq!(
        parse_expected(StepKind::Explore, "<report>done</report>"),
        Err(ParseFault::Action(ToolCallError::NoActionFound))
    );
    assert_eq!(
        parse_expected(StepKind::Explore, "<ask>Which file?</ask>"),
        Err(ParseFault::Action(ToolCallError::NoActionFound))
    );
    let missing_tool = action("decision-1", "fs.read", &[], "ctx-1");
    assert!(matches!(
        parse_expected(StepKind::Explore, &missing_tool),
        Err(ParseFault::Action(ToolCallError::ArgsSchemaViolation(_)))
    ));
    let duplicate = "<lkjagent_action><decision_id>decision-1</decision_id><context_fingerprint>ctx-1</context_fingerprint><tool_name>fs.read</tool_name><input><path>a</path><path>b</path></input></lkjagent_action>";
    assert_eq!(
        parse_expected(StepKind::Explore, duplicate),
        Err(ParseFault::Action(ToolCallError::DuplicateTag(
            "input/path".into(),
        )))
    );
}

fn action(decision_id: &str, tool: &str, args: &[(&str, &str)], context: &str) -> String {
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

use lkjagent_core::model::StepKind;
use lkjagent_core::parse::{parse_expected, parse_expected_for_decision, ParseFault, ParsedOutput};
use lkjagent_core::runtime_decision::{
    OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView, ToolViewEntry,
};

#[test]
fn parses_docs_plan_examples() {
    let raw = "<plan>\nwrite stories/aurora-ledger/manuscript/chapter-01.md | Opening Vault | words=1000\nexplore | Find the file that names the target root | budget=5\nrespond | Report created paths and measured word counts\n</plan>";
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
    let action = "<tool_call><tool_name>graph.state</tool_name></tool_call>";
    assert_eq!(
        parse_expected(StepKind::Explore, action),
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
    let read = "<tool_call><tool_name>fs.read</tool_name><path>README.md</path></tool_call>";
    let shell = "<tool_call><tool_name>shell.run</tool_name><command>pwd</command></tool_call>";

    assert!(matches!(
        parse_expected_for_decision(&decision, read),
        Ok(ParsedOutput::Action(action)) if action.tool == "fs.read"
    ));
    assert_eq!(
        parse_expected_for_decision(&decision, shell),
        Err(ParseFault::UnknownTool)
    );
}

#[test]
fn explore_accepts_only_exact_tool_call_blocks() {
    let finish = "<tool_call><tool_name>finish</tool_name><summary>done</summary></tool_call>";
    assert!(matches!(
        parse_expected(StepKind::Explore, finish),
        Ok(ParsedOutput::Action(action)) if action.tool == "finish"
    ));
    assert_eq!(
        parse_expected(
            StepKind::Explore,
            "<action><tool_name>finish</tool_name></action>"
        ),
        Err(ParseFault::WrongBlock)
    );
    assert_eq!(
        parse_expected(StepKind::Explore, "<finish>done</finish>"),
        Err(ParseFault::WrongBlock)
    );
    assert_eq!(
        parse_expected(StepKind::Explore, "<ask>Which file?</ask>"),
        Err(ParseFault::WrongBlock)
    );
    let missing_tool_name = "<tool_call><tool>finish</tool><summary>done</summary></tool_call>";
    assert_eq!(
        parse_expected(StepKind::Explore, missing_tool_name),
        Err(ParseFault::BadParams)
    );
    let attr = "<tool_call><tool_name kind=\"x\">finish</tool_name></tool_call>";
    assert_eq!(
        parse_expected(StepKind::Explore, attr),
        Err(ParseFault::BadParams)
    );
    let duplicate =
        "<tool_call><tool_name>fs.read</tool_name><path>a</path><path>b</path></tool_call>";
    assert_eq!(
        parse_expected(StepKind::Explore, duplicate),
        Err(ParseFault::BadParams)
    );
    let unknown =
        "<tool_call><tool_name>fs.read</tool_name><path>a</path><extra>x</extra></tool_call>";
    assert_eq!(
        parse_expected(StepKind::Explore, unknown),
        Err(ParseFault::BadParams)
    );
}

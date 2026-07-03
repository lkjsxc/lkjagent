use lkjagent_core::model::StepKind;
use lkjagent_core::parse::{parse_expected, ParseFault, ParsedOutput};

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
    let action = "<action><tool>graph.state</tool></action>";
    assert_eq!(
        parse_expected(StepKind::Explore, action),
        Err(ParseFault::UnknownTool)
    );
}

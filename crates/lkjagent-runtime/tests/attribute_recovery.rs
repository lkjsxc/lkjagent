use lkjagent_protocol::ParseFault;
use lkjagent_runtime::recovery::{parse_notice, parse_recovery_notice};

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn attribute_like_path_fault_renders_exact_graph_plan_repair() -> TestResult<()> {
    let notice = parse_notice(&ParseFault::AttributeLikeTag {
        tag_name: "path=stories/chronos-fracture".to_string(),
        value_hint: Some("stories/chronos-fracture".to_string()),
    });

    assert!(notice.contains("fault=attribute_like_tag"));
    assert!(notice.contains("repair_rule=tag names cannot contain values"));
    assert!(notice.contains("repair_tag=paths"));
    assert!(notice.contains("repair_value=stories/chronos-fracture"));
    assert!(notice.contains("<tool>graph.plan</tool>"));
    assert!(notice.contains("<paths>stories/chronos-fracture</paths>"));
    assert!(!notice.contains("<path=stories/chronos-fracture</path>"));
    Ok(())
}

#[test]
fn repeated_attribute_like_fault_changes_recovery_route() -> TestResult<()> {
    let fault = ParseFault::AttributeLikeTag {
        tag_name: "path=stories/chronos-fracture".to_string(),
        value_hint: Some("stories/chronos-fracture".to_string()),
    };
    let first = parse_recovery_notice(&fault, 1);
    let second = parse_recovery_notice(&fault, 2);
    let third = parse_recovery_notice(&fault, 3);

    assert!(first.contains("next_executable_action"));
    assert!(second.contains("graph.state"));
    assert!(third.contains("deterministic graph.state inspection"));
    Ok(())
}

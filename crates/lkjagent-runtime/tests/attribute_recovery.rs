use lkjagent_protocol::ParseFault;
use lkjagent_runtime::recovery::parse_notice;

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

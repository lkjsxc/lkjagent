use lkjagent_runtime::prompt::{build_prefix, PromptInputs};

#[test]
fn live_prefix_does_not_permit_thinking_tags() -> Result<(), Box<dyn std::error::Error>> {
    let prefix = build_prefix(&PromptInputs {
        graph_state: "case=1".to_string(),
        workspace_brief: "workspace".to_string(),
        memory_digest: "memory".to_string(),
    })?;
    let rendered = prefix
        .iter()
        .map(|frame| frame.content.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    assert!(!rendered.contains("<think>"));
    assert!(!rendered.contains("You may think"));
    assert!(rendered.contains("no prose outside tags"));
    Ok(())
}

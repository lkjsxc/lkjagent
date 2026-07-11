use lkjagent_core::runtime_tool_catalog::{default_explore_tool_view, effect_for_tool};

#[test]
fn default_explore_view_is_small_and_has_no_finish_or_shell() {
    let names = default_explore_tool_view().tool_names();
    assert!(names.len() <= 4, "default view is too broad: {names:?}");
    assert!(!names.iter().any(|name| name == "finish"));
    assert!(effect_for_tool("finish").is_none());
    assert!(!names.iter().any(|name| name == "shell.run"));
}

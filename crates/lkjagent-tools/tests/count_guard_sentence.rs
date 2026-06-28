use lkjagent_tools::count_guard::{count_target, CountKind};

#[test]
fn exact_sentence_count_does_not_become_markdown_file_guard() {
    let text = "create hello.md with exactly one short hello sentence";
    assert!(count_target(&text.to_ascii_lowercase(), text).is_none());
}

#[test]
fn exact_markdown_file_count_still_sets_guard() -> Result<(), String> {
    let text = "create exactly 12 markdown files";
    let Some(guard) = count_target(&text.to_ascii_lowercase(), text) else {
        return Err("missing guard".to_string());
    };
    assert_eq!(guard.kind, CountKind::Markdown);
    assert_eq!(guard.target, 12);
    Ok(())
}

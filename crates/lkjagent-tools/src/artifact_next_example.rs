pub fn batch_write_contract(root: &str, kind: &str, paths: &[String]) -> String {
    let joined = paths
        .iter()
        .map(|path| {
            format!(
                "- {}/{}",
                root.trim_end_matches('/'),
                path.trim_start_matches('/')
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "tool=fs.batch_write\nroot={root}\nkind={kind}\npaths:\n{joined}\nlimits:\n- max_files=20\n- max_file_bytes=1800\n- max_batch_bytes=6000\nrequired_sections:\n{}\nforbidden_weak_phrase_classes:\n- scaffold-only\n- placeholder\n- owner-terms-only\n- generic-example\nmodel_instruction=author the singular fs.batch_write action with line protocol; do not copy body prose from a tool",
        required_sections(kind)
    )
}

pub fn root_identity_contract(root: &str, kind: &str) -> String {
    let paths = if kind.eq_ignore_ascii_case("story") || story_root(root) {
        vec![
            "catalog.toml".to_string(),
            "README.md".to_string(),
            "objective.md".to_string(),
            "setting-overview.md".to_string(),
            "cast.md".to_string(),
        ]
    } else {
        vec![
            "catalog.toml".to_string(),
            "README.md".to_string(),
            "objective.md".to_string(),
            "overview.md".to_string(),
            "verification-notes.md".to_string(),
        ]
    };
    batch_write_contract(root, kind, &paths)
}

fn story_root(root: &str) -> bool {
    root.trim_start_matches("./").starts_with("stories/")
}

fn required_sections(kind: &str) -> &'static str {
    match kind.to_ascii_lowercase().as_str() {
        "cookbook" => "- title\n- purpose\n- ingredients or concept\n- method or procedure\n- timing, signals, and fixes\n- verification notes",
        "story" => "- title\n- purpose\n- scene content or reference detail\n- continuity notes\n- verification notes",
        _ => "- title\n- purpose\n- concrete content\n- verification notes",
    }
}

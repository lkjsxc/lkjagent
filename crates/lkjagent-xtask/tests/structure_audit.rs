use lkjagent_xtask::model::RepoFile;
use lkjagent_xtask::structure::audit;

fn messages(files: Vec<RepoFile>, root: &str) -> Vec<String> {
    audit(&files, root)
        .findings
        .into_iter()
        .map(|finding| finding.message())
        .collect()
}

#[test]
fn audit_reports_readme_missing_child_links() {
    let files = vec![
        RepoFile::new(
            "docs/README.md",
            "# Docs\n\n## Purpose\n\nmap\n\n## Table of Contents\n\n- [a.md](a.md): a.\n",
        ),
        RepoFile::new("docs/a.md", "# A\n\n## Purpose\n\na\n"),
        RepoFile::new("docs/b.md", "# B\n\n## Purpose\n\nb\n"),
    ];

    assert!(messages(files, "docs").contains(
        &"docs/README.md: structure readme-link: link child 'b.md' from README.md".to_string()
    ));
}

#[test]
fn audit_reports_stale_catalog_paths() {
    let files = vec![
        RepoFile::new("docs/README.md", "# Docs\n\n## Purpose\n\nmap\n"),
        RepoFile::new(
            "docs/_meta/catalog/root.toml",
            "\"docs/missing.md\" = { title = \"Missing\" }\n",
        ),
    ];

    assert!(messages(files, "docs").contains(&"docs/missing.md: structure catalog-stale: remove stale catalog entry from docs/_meta/catalog/root.toml:1".to_string()));
}

#[test]
fn audit_reports_over_cap_direct_children() {
    let mut files = vec![RepoFile::new(
        "docs/README.md",
        "# Docs\n\n## Purpose\n\nmap\n",
    )];
    for index in 0..9 {
        files.push(RepoFile::new(
            format!("docs/topic-{index}.md"),
            "# Topic\n\n## Purpose\n\nbody\n",
        ));
    }

    assert!(messages(files, "docs")
        .contains(&"docs: structure fanout: has 10 direct children; cap is 8".to_string()));
}

#[test]
fn audit_reports_scaffold_only_content() {
    let files = vec![
        RepoFile::new(
            "data/workspace/README.md",
            "# Workspace\n\n## Purpose\n\nmap\n\n## Table of Contents\n\n- [leaf.md](leaf.md): leaf.\n",
        ),
        RepoFile::new(
            "data/workspace/leaf.md",
            "# Leaf\n\n## Purpose\n\nrecord\n\n## Concrete Record\n\nConcrete record for a topic.\n",
        ),
    ];

    assert!(messages(files, "data/workspace").contains(&"data/workspace/leaf.md: structure scaffold-content: replace scaffold-only leaf content or mark it as failed evidence".to_string()));
}

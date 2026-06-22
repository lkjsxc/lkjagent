use lkjagent_xtask::doc_catalog::check_doc_catalog;
use lkjagent_xtask::model::{RepoFile, Violation};

fn messages(violations: Vec<Violation>) -> Vec<String> {
    violations
        .into_iter()
        .map(|violation| violation.message())
        .collect()
}

#[test]
fn catalog_requires_every_doc_once() {
    let files = vec![
        RepoFile::new("docs/README.md", "# Docs\n\n## Purpose\n\nroot\n"),
        RepoFile::new("docs/a.md", "# A\n\n## Purpose\n\na\n"),
        RepoFile::new(
            "docs/_meta/catalog/root.toml",
            "\"docs/README.md\" = { title = \"Docs\", parent = \"\", children = [\"docs/a.md\"], role = \"root\", sources = [], checks = [] }\n",
        ),
    ];

    assert_eq!(
        messages(check_doc_catalog(&files)),
        vec!["docs/a.md: doc catalog: add this path to docs/_meta/catalog"]
    );
}

#[test]
fn catalog_accepts_parent_and_children() {
    let files = vec![
        RepoFile::new("docs/README.md", "# Docs\n\n## Purpose\n\nroot\n"),
        RepoFile::new("docs/a.md", "# A\n\n## Purpose\n\na\n"),
        RepoFile::new(
            "docs/_meta/catalog/root.toml",
            "\"docs/README.md\" = { title = \"Docs\", parent = \"\", children = [\"docs/a.md\"], role = \"root\", sources = [], checks = [] }\n\"docs/a.md\" = { title = \"A\", parent = \"docs/README.md\", children = [], role = \"leaf\", sources = [], checks = [] }\n",
        ),
    ];

    assert!(check_doc_catalog(&files).is_empty());
}

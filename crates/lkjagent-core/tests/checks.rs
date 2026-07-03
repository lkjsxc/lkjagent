use lkjagent_core::checks::{evaluate, FileFact};
use lkjagent_core::model::CheckSpec;

#[test]
fn numeric_checks_store_measured_value_without_prose() {
    let files = vec![FileFact {
        path: "a.md".to_string(),
        content: "one two three".to_string(),
    }];
    let result = evaluate(
        &CheckSpec::MinWordsTotal {
            glob: "*.md".to_string(),
            n: 5,
        },
        &files,
        &[],
    );
    assert_eq!(result.name, "min_words_total");
    assert_eq!(result.measured, "3");
    assert!(!result.passed);
}

#[test]
fn readme_coverage_requires_links_to_children() {
    let files = vec![
        fact("docs/README.md", "# Docs\n"),
        fact("docs/page.md", "# Page\n"),
    ];
    let result = evaluate(
        &CheckSpec::ReadmeCoverage {
            root: "docs".to_string(),
        },
        &files,
        &[],
    );
    assert!(!result.passed);
}

#[test]
fn links_resolve_normalizes_relative_anchors_and_directory_readmes() {
    let files = vec![
        fact("docs/README.md", "# Docs\n\n- [Guide](./guide/)\n"),
        fact(
            "docs/guide/README.md",
            "# Guide\n\n[Back](../README.md#top)\n",
        ),
    ];
    let coverage = evaluate(
        &CheckSpec::ReadmeCoverage {
            root: "docs".to_string(),
        },
        &files,
        &[],
    );
    let links = evaluate(
        &CheckSpec::LinksResolve {
            root: "docs".to_string(),
        },
        &files,
        &[],
    );
    assert!(coverage.passed);
    assert!(links.passed);
}

fn fact(path: &str, content: &str) -> FileFact {
    FileFact {
        path: path.to_string(),
        content: content.to_string(),
    }
}

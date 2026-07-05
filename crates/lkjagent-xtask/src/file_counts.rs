use crate::model::{RepoFile, Violation};

const DOC_FILE_LIMIT: usize = 100;
const PRODUCT_SOURCE_LIMIT: usize = 170;
const PRODUCT_CRATES: &[&str] = &[
    "lkjagent-core",
    "lkjagent-store",
    "lkjagent-llm",
    "lkjagent-effects",
    "lkjagent-app",
    "lkjagent-xtask",
];

pub fn check_files(files: &[RepoFile]) -> Vec<Violation> {
    let mut violations = Vec::new();
    let docs = files
        .iter()
        .filter(|file| file.path.starts_with("docs/") && file.path.ends_with(".md"))
        .count();
    if docs > DOC_FILE_LIMIT {
        violations.push(Violation::new(
            "docs",
            "file budget",
            format!("has {docs} markdown files, limit is {DOC_FILE_LIMIT}"),
        ));
    }

    let source = files
        .iter()
        .filter(|file| product_source(&file.path))
        .count();
    if source > PRODUCT_SOURCE_LIMIT {
        violations.push(Violation::new(
            "crates",
            "file budget",
            format!("has {source} product source files, limit is {PRODUCT_SOURCE_LIMIT}"),
        ));
    }
    violations
}

fn product_source(path: &str) -> bool {
    PRODUCT_CRATES
        .iter()
        .any(|name| path.starts_with(&format!("crates/{name}/src/")))
        && path.ends_with(".rs")
}

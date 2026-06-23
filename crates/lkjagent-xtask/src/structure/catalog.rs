use std::collections::{BTreeMap, BTreeSet};

use crate::model::RepoFile;

use super::findings::StructureFinding;

pub fn catalog_findings(files: &[RepoFile], root: &str) -> Vec<StructureFinding> {
    if !root.trim_end_matches('/').starts_with("docs") {
        return Vec::new();
    }
    let docs = docs_under_root(files, root);
    let entries = catalog_entries(files);
    let mut findings = Vec::new();
    for doc in &docs {
        if !entries.contains_key(doc) {
            findings.push(StructureFinding::new(
                doc,
                "structure catalog-missing",
                "add this authored Markdown path to docs/_meta/catalog",
            ));
        }
    }
    for (path, sources) in entries {
        if path.starts_with(root) && !docs.contains(&path) {
            findings.push(StructureFinding::new(
                path,
                "structure catalog-stale",
                format!("remove stale catalog entry from {}", sources.join(", ")),
            ));
        }
    }
    findings
}

fn docs_under_root(files: &[RepoFile], root: &str) -> BTreeSet<String> {
    let prefix = format!("{}/", root.trim_end_matches('/'));
    files
        .iter()
        .filter(|file| file.path.ends_with(".md"))
        .filter(|file| file.path == root || file.path.starts_with(&prefix))
        .map(|file| file.path.clone())
        .collect()
}

fn catalog_entries(files: &[RepoFile]) -> BTreeMap<String, Vec<String>> {
    let mut entries: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for file in files
        .iter()
        .filter(|file| file.path.starts_with("docs/_meta/catalog/"))
        .filter(|file| file.path.ends_with(".toml"))
    {
        for (index, line) in file.text.lines().enumerate() {
            if let Some(path) = quoted_prefix(line.trim()) {
                entries
                    .entry(path)
                    .or_default()
                    .push(format!("{}:{}", file.path, index + 1));
            }
        }
    }
    entries
}

fn quoted_prefix(line: &str) -> Option<String> {
    let rest = line.strip_prefix('"')?;
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

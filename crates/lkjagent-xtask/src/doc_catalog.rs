use std::collections::{BTreeMap, BTreeSet};

use crate::model::{RepoFile, Violation};

pub fn check_doc_catalog(files: &[RepoFile]) -> Vec<Violation> {
    let docs = doc_paths(files);
    let entries = catalog_entries(files);
    let mut violations = Vec::new();
    if entries.is_empty() {
        violations.push(Violation::new(
            "docs/_meta/catalog",
            "doc catalog",
            "add catalog TOML entries for docs",
        ));
        return violations;
    }
    violations.extend(check_coverage(&docs, &entries));
    violations.extend(check_parents(&docs, &entries));
    violations.extend(check_children(&docs, &entries));
    violations
}

fn doc_paths(files: &[RepoFile]) -> BTreeSet<String> {
    files
        .iter()
        .filter(|file| file.path.starts_with("docs/") && file.path.ends_with(".md"))
        .map(|file| file.path.clone())
        .collect()
}

fn catalog_entries(files: &[RepoFile]) -> BTreeMap<String, Vec<CatalogEntry>> {
    let mut entries: BTreeMap<String, Vec<CatalogEntry>> = BTreeMap::new();
    for file in files.iter().filter(|file| is_catalog(&file.path)) {
        for (index, line) in file.text.lines().enumerate() {
            if let Some(entry) = parse_entry(&file.path, index + 1, line) {
                entries.entry(entry.path.clone()).or_default().push(entry);
            }
        }
    }
    entries
}

fn is_catalog(path: &str) -> bool {
    path.starts_with("docs/_meta/catalog/") && path.ends_with(".toml")
}

fn parse_entry(file: &str, line_number: usize, line: &str) -> Option<CatalogEntry> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }
    let path = quoted_prefix(trimmed)?;
    Some(CatalogEntry {
        path,
        parent: field_string(trimmed, "parent"),
        children: field_array(trimmed, "children"),
        valid_shape: required_fields_present(trimmed),
        source: format!("{file}:{line_number}"),
    })
}

fn quoted_prefix(line: &str) -> Option<String> {
    let rest = line.strip_prefix('"')?;
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

fn required_fields_present(line: &str) -> bool {
    ["title", "parent", "children", "role", "sources", "checks"]
        .iter()
        .all(|field| line.contains(&format!("{field} =")))
}

fn field_string(line: &str, name: &str) -> Option<String> {
    let needle = format!("{name} = \"");
    let after = line.split_once(&needle)?.1;
    let end = after.find('"')?;
    Some(after[..end].to_string())
}

fn field_array(line: &str, name: &str) -> Vec<String> {
    let needle = format!("{name} = [");
    let Some(after) = line.split_once(&needle).map(|parts| parts.1) else {
        return Vec::new();
    };
    let Some((body, _)) = after.split_once(']') else {
        return Vec::new();
    };
    quoted_values(body)
}

fn quoted_values(body: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut rest = body;
    while let Some(start) = rest.find('"') {
        let after = &rest[start + 1..];
        let Some(end) = after.find('"') else {
            break;
        };
        values.push(after[..end].to_string());
        rest = &after[end + 1..];
    }
    values
}

fn check_coverage(
    docs: &BTreeSet<String>,
    entries: &BTreeMap<String, Vec<CatalogEntry>>,
) -> Vec<Violation> {
    let mut violations = Vec::new();
    for doc in docs {
        match entries.get(doc).map(Vec::len) {
            Some(1) => {}
            Some(count) => violations.push(Violation::new(
                doc,
                "doc catalog",
                format!("catalog lists this path {count} times"),
            )),
            None => violations.push(Violation::new(
                doc,
                "doc catalog",
                "add this path to docs/_meta/catalog",
            )),
        }
    }
    for (path, entries_for_path) in entries {
        for entry in entries_for_path {
            if !entry.valid_shape {
                violations.push(Violation::new(
                    &entry.source,
                    "doc catalog",
                    "entry must include title, parent, children, role, sources, and checks",
                ));
            }
        }
        if !docs.contains(path) {
            violations.push(Violation::new(
                path,
                "doc catalog",
                "catalog entry does not point to an authored doc",
            ));
        }
    }
    violations
}

fn check_parents(
    docs: &BTreeSet<String>,
    entries: &BTreeMap<String, Vec<CatalogEntry>>,
) -> Vec<Violation> {
    entries
        .values()
        .filter_map(|items| items.first())
        .filter_map(|entry| match &entry.parent {
            Some(parent) if !parent.is_empty() && !docs.contains(parent) => Some(Violation::new(
                &entry.source,
                "doc catalog",
                format!("parent '{parent}' is not an authored doc"),
            )),
            None => Some(Violation::new(
                &entry.source,
                "doc catalog",
                "entry must name a parent field",
            )),
            _ => None,
        })
        .collect()
}

fn check_children(
    docs: &BTreeSet<String>,
    entries: &BTreeMap<String, Vec<CatalogEntry>>,
) -> Vec<Violation> {
    let mut violations = Vec::new();
    for entry in entries.values().filter_map(|items| items.first()) {
        for child in &entry.children {
            if !docs.contains(child) {
                violations.push(Violation::new(
                    &entry.source,
                    "doc catalog",
                    format!("child '{child}' is not an authored doc"),
                ));
            }
        }
    }
    violations
}

struct CatalogEntry {
    path: String,
    parent: Option<String>,
    children: Vec<String>,
    valid_shape: bool,
    source: String,
}

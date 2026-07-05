use std::collections::BTreeSet;

use crate::doc_reachability::check_reachability;
use crate::model::{RepoFile, Violation};

const DOC_FILE_LIMIT: usize = 100;

pub fn check_doc_topology(files: &[RepoFile]) -> Vec<Violation> {
    let mut violations = Vec::new();
    let dirs = docs_dirs(files);
    for dir in &dirs {
        violations.extend(check_dir(files, dir));
    }
    violations.extend(check_path_hygiene(files));
    violations.extend(check_doc_file_budget(files));
    violations.extend(check_reachability(files));
    violations
}

fn docs_dirs(files: &[RepoFile]) -> BTreeSet<String> {
    let mut dirs = BTreeSet::new();
    for file in files.iter().filter(|file| file.path.starts_with("docs/")) {
        let mut current = String::new();
        let mut parts = file.path.split('/').peekable();
        while let Some(part) = parts.next() {
            if current.is_empty() {
                current.push_str(part);
            } else {
                current.push('/');
                current.push_str(part);
            }
            if parts.peek().is_some() {
                dirs.insert(current.clone());
            }
        }
    }
    dirs
}

fn check_dir(files: &[RepoFile], dir: &str) -> Vec<Violation> {
    let mut violations = Vec::new();
    let readme_path = format!("{dir}/README.md");
    let readme = files.iter().find(|file| file.path == readme_path);
    if readme.is_none() {
        violations.push(Violation::new(
            dir,
            "readme topology",
            "directory must contain README.md",
        ));
        return violations;
    }

    let children = immediate_children(files, dir);
    let counted = children
        .iter()
        .filter(|child| child.as_str() != "README.md")
        .count();
    if counted < 2 {
        violations.push(Violation::new(
            dir,
            "readme topology",
            "directory must contain at least two children beside README.md",
        ));
    }
    if let Some(readme_file) = readme {
        for child in children
            .iter()
            .filter(|child| child.as_str() != "README.md")
        {
            if !readme_links_child(&readme_file.text, child) {
                violations.push(Violation::new(
                    &readme_file.path,
                    "readme topology",
                    format!("link child '{child}' from the table of contents"),
                ));
            }
        }
    }
    violations
}

fn immediate_children(files: &[RepoFile], dir: &str) -> BTreeSet<String> {
    let mut children = BTreeSet::new();
    let prefix = format!("{dir}/");
    for file in files.iter().filter(|file| file.path.starts_with(&prefix)) {
        let rest = file.path.trim_start_matches(&prefix);
        if let Some(child) = rest.split('/').next() {
            children.insert(child.to_string());
        }
    }
    children
}

fn readme_links_child(text: &str, child: &str) -> bool {
    if child.ends_with(".md") {
        text.contains(&format!("({child})"))
    } else {
        text.contains(&format!("({child}/)")) || text.contains(&format!("({child}/README.md)"))
    }
}

fn check_path_hygiene(files: &[RepoFile]) -> Vec<Violation> {
    let mut violations = Vec::new();
    for file in files.iter().filter(|file| file.path.starts_with("docs/")) {
        for segment in file.path.split('/').skip(1) {
            if segment == "README.md" {
                continue;
            }
            let stem = segment.strip_suffix(".md").unwrap_or(segment);
            if !is_kebab(stem) {
                violations.push(Violation::new(
                    &file.path,
                    "doc path",
                    format!("segment '{segment}' must be kebab-case"),
                ));
            }
        }
    }
    violations
}

fn is_kebab(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !value.contains("--")
        && !value.starts_with('-')
        && !value.ends_with('-')
}

fn check_doc_file_budget(files: &[RepoFile]) -> Vec<Violation> {
    let count = files
        .iter()
        .filter(|file| file.path.starts_with("docs/") && file.path.ends_with(".md"))
        .count();
    if count > DOC_FILE_LIMIT {
        vec![Violation::new(
            "docs",
            "doc file budget",
            format!("has {count} markdown files, limit is {DOC_FILE_LIMIT}"),
        )]
    } else {
        Vec::new()
    }
}

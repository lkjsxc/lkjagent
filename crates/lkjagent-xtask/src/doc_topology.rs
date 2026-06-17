use std::collections::BTreeSet;

use crate::model::{RepoFile, Violation};

pub fn check_doc_topology(files: &[RepoFile]) -> Vec<Violation> {
    let mut violations = Vec::new();
    let dirs = docs_dirs(files);
    for dir in &dirs {
        violations.extend(check_dir(files, dir));
    }
    violations.extend(check_all_files(files));
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
        text.contains(&format!("({child}/README.md)"))
    }
}

fn check_all_files(files: &[RepoFile]) -> Vec<Violation> {
    let readme = files.iter().find(|file| file.path == "docs/README.md");
    let Some(readme_file) = readme else {
        return vec![Violation::new(
            "docs/README.md",
            "all files",
            "add the documentation root README.md",
        )];
    };
    let mut docs_files: Vec<&RepoFile> = files
        .iter()
        .filter(|file| file.path.starts_with("docs/"))
        .filter(|file| file.path.ends_with(".md"))
        .filter(|file| file.path != "docs/README.md")
        .collect();
    docs_files.sort_by(|left, right| left.path.cmp(&right.path));
    docs_files
        .into_iter()
        .filter_map(|file| {
            let relative = file.path.trim_start_matches("docs/");
            if readme_file.text.contains(&format!("`{relative}`")) {
                None
            } else {
                Some(Violation::new(
                    &readme_file.path,
                    "all files",
                    format!("list '{relative}' in the All Files manifest"),
                ))
            }
        })
        .collect()
}

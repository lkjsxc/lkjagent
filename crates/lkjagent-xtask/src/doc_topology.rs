use std::collections::BTreeSet;

use crate::model::{RepoFile, Violation};

pub fn check_doc_topology(files: &[RepoFile]) -> Vec<Violation> {
    let mut violations = Vec::new();
    let dirs = docs_dirs(files);
    for dir in &dirs {
        violations.extend(check_markdown_suffix_dir(dir));
        violations.extend(check_dir(files, dir));
    }
    violations.extend(check_path_hygiene(files));
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

fn check_markdown_suffix_dir(dir: &str) -> Vec<Violation> {
    if dir
        .rsplit('/')
        .next()
        .is_some_and(|name| name.ends_with(".md"))
    {
        vec![Violation::new(
            dir,
            "readme topology",
            "directory name must not end with .md",
        )]
    } else {
        Vec::new()
    }
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
    if child.contains('.') {
        text.contains(&format!("({child})"))
    } else {
        text.contains(&format!("({child}/README.md)"))
    }
}

fn check_path_hygiene(files: &[RepoFile]) -> Vec<Violation> {
    let mut violations = Vec::new();
    for file in files.iter().filter(|file| file.path.starts_with("docs/")) {
        if file.path.contains(' ') {
            violations.push(Violation::new(
                &file.path,
                "doc path",
                "documentation paths must not contain spaces",
            ));
        }
        if file.path.contains("mincraft") {
            violations.push(Violation::new(
                &file.path,
                "doc path",
                "use 'minecraft' for the domain name",
            ));
        }
        if file.path.contains("-md-") {
            violations.push(Violation::new(
                &file.path,
                "doc path",
                "remove generated '-md-' path fragments",
            ));
        }
        if let Some(segment) = file.path.split('/').find(|segment| segment.len() > 80) {
            violations.push(Violation::new(
                &file.path,
                "doc path",
                format!("path segment '{segment}' is too long"),
            ));
        }
    }
    violations
}

use std::collections::BTreeSet;

use crate::model::{RepoFile, Violation};

pub fn check_doc_links(files: &[RepoFile]) -> Vec<Violation> {
    let paths = files
        .iter()
        .map(|file| file.path.as_str())
        .collect::<BTreeSet<_>>();
    let mut violations = Vec::new();
    for file in files.iter().filter(|file| is_doc(file)) {
        for target in markdown_links(&file.text) {
            if skip_target(&target) {
                continue;
            }
            let Some(resolved) = resolve_link(&file.path, &target) else {
                violations.push(Violation::new(
                    &file.path,
                    "doc link",
                    format!("invalid link target '{target}'"),
                ));
                continue;
            };
            if !link_exists(&paths, &resolved) {
                violations.push(Violation::new(
                    &file.path,
                    "doc link",
                    format!("broken link '{target}' resolves to '{resolved}'"),
                ));
            }
        }
    }
    violations
}

fn is_doc(file: &RepoFile) -> bool {
    file.path.starts_with("docs/") && file.path.ends_with(".md")
}

fn markdown_links(text: &str) -> Vec<String> {
    let mut links = Vec::new();
    for line in text.lines() {
        let mut rest = line;
        while let Some(start) = rest.find("](") {
            let after = &rest[start + 2..];
            let Some(end) = after.find(')') else {
                break;
            };
            links.push(after[..end].trim().to_string());
            rest = &after[end + 1..];
        }
    }
    links
}

fn skip_target(target: &str) -> bool {
    target.is_empty()
        || target.starts_with('#')
        || target.starts_with("http://")
        || target.starts_with("https://")
        || target.starts_with("mailto:")
}

fn resolve_link(path: &str, target: &str) -> Option<String> {
    let target = target.split('#').next().unwrap_or(target);
    if target.is_empty() {
        return Some(path.to_string());
    }
    let base = path.rsplit_once('/').map_or("", |(dir, _)| dir);
    let combined = if target.starts_with('/') {
        target.trim_start_matches('/').to_string()
    } else if base.is_empty() {
        target.to_string()
    } else {
        format!("{base}/{target}")
    };
    normalize(&combined)
}

fn normalize(path: &str) -> Option<String> {
    let mut stack = Vec::new();
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                stack.pop()?;
            }
            other => stack.push(other),
        }
    }
    Some(stack.join("/"))
}

fn link_exists(paths: &BTreeSet<&str>, resolved: &str) -> bool {
    paths.contains(resolved)
        || paths.contains(&format!("{resolved}/README.md").as_str())
        || resolved
            .strip_suffix('/')
            .is_some_and(|dir| paths.contains(&format!("{dir}/README.md").as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_relative_doc_links() {
        let files = vec![
            RepoFile::new("docs/a/one.md", "# One\n\n## Purpose\n\n[Two](../b/two.md)"),
            RepoFile::new("docs/b/two.md", "# Two\n\n## Purpose\n\nOk."),
        ];
        assert!(check_doc_links(&files).is_empty());
    }

    #[test]
    fn reports_broken_relative_doc_links() {
        let files = vec![RepoFile::new(
            "docs/a/one.md",
            "# One\n\n## Purpose\n\n[Missing](../b/missing.md)",
        )];
        let violations = check_doc_links(&files);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].fix.contains("missing.md"));
    }
}

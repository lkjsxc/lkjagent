use std::collections::{BTreeMap, BTreeSet};

use crate::model::{RepoFile, Violation};

pub fn check_crate_readmes(files: &[RepoFile]) -> Vec<Violation> {
    let mut violations = Vec::new();
    for crate_name in crate_names(files) {
        let root = format!("crates/{crate_name}");
        violations.extend(require_root_readme(files, &root));
        for source_dir in source_dirs(files, &root) {
            violations.extend(require_source_readme(files, &source_dir));
        }
    }
    violations
}

fn crate_names(files: &[RepoFile]) -> BTreeSet<String> {
    files
        .iter()
        .filter_map(|file| file.path.strip_prefix("crates/"))
        .filter_map(|rest| rest.split('/').next())
        .map(str::to_string)
        .collect()
}

fn source_dirs(files: &[RepoFile], root: &str) -> BTreeSet<String> {
    let mut dirs = BTreeSet::new();
    let prefix = format!("{root}/src/");
    for file in files.iter().filter(|file| file.path.starts_with(&prefix)) {
        let mut current = format!("{root}/src");
        dirs.insert(current.clone());
        let rest = file.path.trim_start_matches(&prefix);
        let mut parts = rest.split('/').peekable();
        while let Some(part) = parts.next() {
            if parts.peek().is_some() {
                current.push('/');
                current.push_str(part);
                dirs.insert(current.clone());
            }
        }
    }
    dirs
}

fn require_root_readme(files: &[RepoFile], root: &str) -> Vec<Violation> {
    let readme_path = format!("{root}/README.md");
    let Some(readme) = files.iter().find(|file| file.path == readme_path) else {
        return vec![Violation::new(
            root,
            "crate readme",
            "add README.md for this crate directory",
        )];
    };
    let mut violations = Vec::new();
    if !readme.text.contains("Doc contract:") {
        violations.push(Violation::new(
            &readme.path,
            "crate readme",
            "name the Doc contract",
        ));
    }
    if !readme.text.contains("## Table of Contents") {
        violations.push(Violation::new(
            &readme.path,
            "crate readme",
            "add a Table of Contents",
        ));
    }
    violations
}

fn require_source_readme(files: &[RepoFile], dir: &str) -> Vec<Violation> {
    let readme_path = format!("{dir}/README.md");
    let Some(readme) = files.iter().find(|file| file.path == readme_path) else {
        return vec![Violation::new(
            dir,
            "crate readme",
            "add README.md for this source directory",
        )];
    };
    let mut violations = Vec::new();
    if !readme.text.contains("## Table of Contents") {
        violations.push(Violation::new(
            &readme.path,
            "crate readme",
            "add a Table of Contents",
        ));
    }
    let links = resolved_markdown_links(dir, &readme.text);
    for (label, path) in direct_source_children(files, dir) {
        if !child_is_linked(&links, &path) {
            violations.push(Violation::new(
                &readme.path,
                "crate readme",
                format!("link direct source child {label}"),
            ));
        }
    }
    violations
}

fn direct_source_children(files: &[RepoFile], dir: &str) -> BTreeMap<String, String> {
    let prefix = format!("{dir}/");
    let mut children = BTreeMap::new();
    for file in files.iter().filter(|file| file.path.starts_with(&prefix)) {
        let rest = file.path.trim_start_matches(&prefix);
        if rest == "README.md" {
            continue;
        }
        if let Some((child, _)) = rest.split_once('/') {
            children.insert(format!("{child}/"), format!("{prefix}{child}"));
        } else if rest.ends_with(".rs") {
            children.insert(rest.to_string(), format!("{prefix}{rest}"));
        }
    }
    children
}

fn resolved_markdown_links(dir: &str, text: &str) -> BTreeSet<String> {
    let mut links = BTreeSet::new();
    for target in markdown_targets(text) {
        if let Some(resolved) = resolve_link(dir, &target) {
            links.insert(resolved);
        }
    }
    links
}

fn markdown_targets(text: &str) -> Vec<String> {
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

fn resolve_link(dir: &str, target: &str) -> Option<String> {
    if target.starts_with('#') || target.starts_with("http://") || target.starts_with("https://") {
        return None;
    }
    let target = target.split('#').next().unwrap_or(target);
    let combined = if target.starts_with('/') {
        target.trim_start_matches('/').to_string()
    } else {
        format!("{dir}/{target}")
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

fn child_is_linked(links: &BTreeSet<String>, path: &str) -> bool {
    links.contains(path) || links.contains(&format!("{path}/README.md"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_readme_must_link_direct_rs_child() {
        let files = vec![
            RepoFile::new("crates/demo/Cargo.toml", ""),
            RepoFile::new("crates/demo/README.md", "# Demo\n\nDoc contract:\n\n## Table of Contents\n"),
            RepoFile::new("crates/demo/src/README.md", "# Source\n\n## Table of Contents\n\n- [lib.rs](lib.rs): root."),
            RepoFile::new("crates/demo/src/lib.rs", ""),
            RepoFile::new("crates/demo/src/new.rs", ""),
        ];
        let violations = check_crate_readmes(&files);
        assert!(violations.iter().any(|item| item.fix.contains("new.rs")));
    }
}

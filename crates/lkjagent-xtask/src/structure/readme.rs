use std::collections::BTreeSet;

use crate::model::RepoFile;

use super::findings::StructureFinding;

pub fn readme_findings(files: &[RepoFile], root: &str, fanout_cap: usize) -> Vec<StructureFinding> {
    let scoped = scoped_files(files, root);
    let dirs = directories(&scoped, root);
    let mut findings = Vec::new();
    for dir in dirs {
        let readme_path = format!("{dir}/README.md");
        let readme = scoped.iter().find(|file| file.path == readme_path);
        let children = immediate_children(&scoped, &dir);
        if readme.is_none() {
            findings.push(StructureFinding::new(
                &dir,
                "structure readme",
                "create README.md that links immediate children",
            ));
            continue;
        }
        if children.len() > fanout_cap {
            findings.push(StructureFinding::new(
                &dir,
                "structure fanout",
                format!(
                    "has {} direct children; cap is {fanout_cap}",
                    children.len()
                ),
            ));
        }
        let meaningful = children
            .iter()
            .filter(|child| child.as_str() != "README.md")
            .count();
        if meaningful == 1 {
            findings.push(StructureFinding::new(
                &dir,
                "structure one-child",
                "collapse the directory or add a real sibling",
            ));
        }
        if let Some(readme_file) = readme {
            findings.extend(missing_child_links(readme_file, &children));
        }
    }
    findings.extend(path_findings(&scoped));
    findings.extend(weak_content_findings(&scoped));
    findings
}

fn scoped_files<'a>(files: &'a [RepoFile], root: &str) -> Vec<&'a RepoFile> {
    let prefix = format!("{}/", root.trim_end_matches('/'));
    files
        .iter()
        .filter(|file| file.path == root || file.path.starts_with(&prefix))
        .collect()
}

fn directories(files: &[&RepoFile], root: &str) -> BTreeSet<String> {
    let base = root.trim_end_matches('/').to_string();
    let prefix = format!("{base}/");
    let mut dirs = BTreeSet::from([base.clone()]);
    for file in files {
        let mut current = String::new();
        let mut parts = file.path.split('/').peekable();
        while let Some(part) = parts.next() {
            if current.is_empty() {
                current.push_str(part);
            } else {
                current.push('/');
                current.push_str(part);
            }
            if parts.peek().is_some() && (current == base || current.starts_with(&prefix)) {
                dirs.insert(current.clone());
            }
        }
    }
    dirs
}

fn immediate_children(files: &[&RepoFile], dir: &str) -> BTreeSet<String> {
    let prefix = format!("{dir}/");
    let mut children = BTreeSet::new();
    for file in files.iter().filter(|file| file.path.starts_with(&prefix)) {
        let rest = file.path.trim_start_matches(&prefix);
        if let Some(child) = rest.split('/').next() {
            children.insert(child.to_string());
        }
    }
    children
}

fn missing_child_links(readme: &RepoFile, children: &BTreeSet<String>) -> Vec<StructureFinding> {
    children
        .iter()
        .filter(|child| child.as_str() != "README.md")
        .filter(|child| !readme_links_child(&readme.text, child))
        .map(|child| {
            StructureFinding::new(
                &readme.path,
                "structure readme-link",
                format!("link child '{child}' from README.md"),
            )
        })
        .collect()
}

fn readme_links_child(text: &str, child: &str) -> bool {
    if child.contains('.') {
        text.contains(&format!("({child})"))
    } else {
        text.contains(&format!("({child}/README.md)"))
    }
}

fn path_findings(files: &[&RepoFile]) -> Vec<StructureFinding> {
    let mut findings = Vec::new();
    for file in files {
        if file.path.contains("/tmp/") || file.path.ends_with(".tmp") {
            findings.push(StructureFinding::new(
                &file.path,
                "structure runtime-tmp",
                "move runtime temporary files under repository tmp/",
            ));
        }
        if sequence_only_leaf(&file.path) {
            findings.push(StructureFinding::new(
                &file.path,
                "structure sequence-name",
                "rename sequence-only leaf to a semantic name",
            ));
        }
        if generic_bucket(&file.path) {
            findings.push(StructureFinding::new(
                &file.path,
                "structure generic-bucket",
                "replace generic topic bucket with objective-owned grouping",
            ));
        }
    }
    findings
}

fn weak_content_findings(files: &[&RepoFile]) -> Vec<StructureFinding> {
    files
        .iter()
        .filter(|file| file.path.ends_with(".md"))
        .filter(|file| scaffold_only(&file.text))
        .map(|file| {
            StructureFinding::new(
                &file.path,
                "structure scaffold-content",
                "replace scaffold-only leaf content or mark it as failed evidence",
            )
        })
        .collect()
}

fn sequence_only_leaf(path: &str) -> bool {
    let Some(name) = path.rsplit('/').next() else {
        return false;
    };
    let stem = name.trim_end_matches(".md");
    stem.chars().all(|char| char.is_ascii_digit())
        || stem
            .strip_prefix("part-")
            .is_some_and(|tail| tail.chars().all(|char| char.is_ascii_digit()))
}

fn generic_bucket(path: &str) -> bool {
    path.split('/').any(|part| {
        matches!(
            part,
            "misc" | "general" | "topics" | "content" | "stuff" | "bucket"
        )
    })
}

fn scaffold_only(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    let strong_phrases = [
        "## concrete record",
        "concrete record for",
        "this file is a scaffold",
        "todo: replace",
        "lorem ipsum",
    ];
    strong_phrases.iter().any(|phrase| lower.contains(phrase))
}

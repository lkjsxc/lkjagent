use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::model::{RepoFile, Violation};

pub fn check_docs(files: &[RepoFile]) -> Vec<Violation> {
    let mut violations = Vec::new();
    violations.extend(crate::doc_common::check_markdown_basics(files));
    violations.extend(crate::doc_topology::check_doc_topology(files));
    violations.extend(crate::doc_links::check_doc_links(files));
    violations.extend(check_special_docs(files));
    violations
}

pub fn check_special_docs(files: &[RepoFile]) -> Vec<Violation> {
    let mut violations = Vec::new();
    violations.extend(check_model_name_claims(files));
    violations.extend(check_generated_boilerplate(files));
    violations
}

fn check_model_name_claims(files: &[RepoFile]) -> Vec<Violation> {
    let mut violations = Vec::new();
    for file in files.iter().filter(|file| file.path.starts_with("docs/")) {
        for (index, line) in file.text.lines().enumerate() {
            if let Some(pattern) = model_name_pattern(line) {
                violations.push(Violation::new(
                    &file.path,
                    "model names",
                    format!(
                        "line {} contains '{pattern}'; use provider-neutral wording",
                        index + 1
                    ),
                ));
            }
        }
    }
    violations
}

fn model_name_pattern(line: &str) -> Option<&'static str> {
    for pattern in ["GPT-", "Qwen3.5", "Claude-", "Gemini-"] {
        if line.contains(pattern) {
            return Some(pattern);
        }
    }
    if line.to_ascii_lowercase().contains("latest model") {
        return Some("latest model");
    }
    None
}

fn check_generated_boilerplate(files: &[RepoFile]) -> Vec<Violation> {
    let mut violations = Vec::new();
    for file in files
        .iter()
        .filter(|file| !runtime_output(&file.path) && !boilerplate_allowlist(&file.path))
    {
        let repeated_leaf = [
            "Keep this file semantic and linked from its local README",
            "Record concrete facts, decisions, and verification evidence",
            "Implementation Hooks",
            "Failure Modes",
            "scaffolded",
        ]
        .iter()
        .all(|phrase| file.text.contains(phrase));
        let filler = [
            "defines the artifact role, the observed constraints",
            "Example one names a path, an invariant",
        ]
        .iter()
        .any(|phrase| file.text.contains(phrase));
        if repeated_leaf || filler {
            violations.push(Violation::new(
                &file.path,
                "generated boilerplate",
                "remove repeated generated leaf prose",
            ));
        }
    }
    violations
}

fn runtime_output(path: &str) -> bool {
    path.starts_with("data/logs/") || path.starts_with("data/workspace/")
}

fn boilerplate_allowlist(path: &str) -> bool {
    matches!(
        path,
        "crates/lkjagent-tools/tests/doc_boilerplate.rs"
            | "crates/lkjagent-tools/src/doc/content_signals.rs"
            | "crates/lkjagent-tools/src/doc/repeated_content.rs"
            | "crates/lkjagent-xtask/src/doc_special.rs"
    )
}

pub fn check_reachability(files: &[RepoFile]) -> Vec<Violation> {
    let docs = files
        .iter()
        .filter(|file| file.path.starts_with("docs/") && file.path.ends_with(".md"))
        .map(|file| file.path.clone())
        .collect::<BTreeSet<_>>();
    let graph = link_graph(files);
    let reached = reachable(&graph, "docs/README.md");
    docs.into_iter()
        .filter(|path| reached.get(path).is_none_or(|depth| *depth > 3))
        .map(|path| {
            Violation::new(
                path,
                "doc reachability",
                "link this page from docs/README.md within three links",
            )
        })
        .collect()
}

fn link_graph(files: &[RepoFile]) -> BTreeMap<String, Vec<String>> {
    let paths = files.iter().map(|file| file.path.as_str()).collect();
    let mut graph = BTreeMap::new();
    for file in files
        .iter()
        .filter(|file| file.path.starts_with("docs/") && file.path.ends_with(".md"))
    {
        let targets = markdown_links(&file.text)
            .into_iter()
            .filter_map(|target| resolve_link(&file.path, &target, &paths))
            .collect();
        graph.insert(file.path.clone(), targets);
    }
    graph
}

fn reachable(graph: &BTreeMap<String, Vec<String>>, start: &str) -> BTreeMap<String, usize> {
    let mut seen = BTreeMap::new();
    let mut queue = VecDeque::from([(start.to_string(), 0)]);
    while let Some((path, depth)) = queue.pop_front() {
        if seen.get(&path).is_some_and(|old| *old <= depth) {
            continue;
        }
        seen.insert(path.clone(), depth);
        for target in graph.get(&path).into_iter().flatten() {
            queue.push_back((target.clone(), depth + 1));
        }
    }
    seen
}

fn markdown_links(text: &str) -> Vec<String> {
    let mut links = Vec::new();
    for line in text.lines() {
        let mut rest = line;
        while let Some(start) = rest.find("](") {
            let after = &rest[start + 2..];
            let Some(end) = after.find(')') else { break };
            links.push(after[..end].trim().to_string());
            rest = &after[end + 1..];
        }
    }
    links
}

fn resolve_link(path: &str, target: &str, paths: &BTreeSet<&str>) -> Option<String> {
    if target.is_empty() || target.starts_with('#') || target.contains("://") {
        return None;
    }
    let target = target.split('#').next().unwrap_or(target);
    let base = path.rsplit_once('/').map_or("", |(dir, _)| dir);
    let combined = if base.is_empty() {
        target.to_string()
    } else {
        format!("{base}/{target}")
    };
    let normalized = normalize(&combined)?;
    if paths.contains(normalized.as_str()) {
        Some(normalized)
    } else {
        paths
            .contains(format!("{normalized}/README.md").as_str())
            .then(|| format!("{normalized}/README.md"))
    }
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

use std::fs;
use std::path::Path;

use crate::error::{ToolError, ToolResult};
use crate::structure::verify_recursive_tree;

const MIN_DIRECTORIES: usize = 8;
const MIN_MARKDOWN_FILES: usize = 15;
const MIN_LINKS: usize = 18;

const REQUIRED_FILES: &[&str] = &[
    "docs/README.md",
    "docs/maps/README.md",
    "docs/maps/concept-network.md",
    "docs/current-state.md",
    "docs/domains/README.md",
    "docs/domains/core/foundations/seed-topic.md",
    "docs/execution/README.md",
    "docs/execution/expansion-queue.md",
    "docs/execution/rebalance-plan.md",
    "docs/execution/current-blockers.md",
    "docs/reference/ontology.md",
    "docs/curation/workflow.md",
];

const TOP_LEVEL_DIRS: &[&str] = &["maps", "domains", "reference", "curation", "execution"];

const GROWTH_FILES: &[&str] = &[
    "docs/execution/expansion-queue.md",
    "docs/execution/rebalance-plan.md",
    "docs/curation/rebalance.md",
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct NetworkEvidence {
    directories: usize,
    markdown_files: usize,
    links: usize,
    contract_violations: Vec<String>,
}

pub fn verify_knowledge_network(workspace: &Path) -> ToolResult<()> {
    verify_recursive_tree(workspace)?;
    let root = workspace.join("docs");
    if !root.exists() {
        return Err(ToolError::invalid("knowledge network requires docs/"));
    }
    let evidence = collect(&root)?;
    let missing = missing_required(workspace);
    let missing_growth = missing_growth_markers(workspace)?;
    if evidence.directories >= MIN_DIRECTORIES
        && evidence.markdown_files >= MIN_MARKDOWN_FILES
        && evidence.links >= MIN_LINKS
        && evidence.contract_violations.is_empty()
        && missing.is_empty()
        && missing_growth.is_empty()
    {
        return Ok(());
    }
    Err(ToolError::invalid(format!(
        "knowledge nucleus incomplete: need at least {MIN_DIRECTORIES} docs directories, {MIN_MARKDOWN_FILES} markdown files, {MIN_LINKS} markdown links, docs contract shape, growth control sections, and required files {}; got directories={} markdown_files={} links={} missing={} missing_growth={} docs_contract={}",
        REQUIRED_FILES.join(","),
        evidence.directories,
        evidence.markdown_files,
        evidence.links,
        sample_list(&missing),
        sample_list(&missing_growth),
        sample_list(&evidence.contract_violations)
    )))
}

fn collect(root: &Path) -> ToolResult<NetworkEvidence> {
    let mut evidence = NetworkEvidence {
        directories: 0,
        markdown_files: 0,
        links: 0,
        contract_violations: Vec::new(),
    };
    collect_into(root, root, &mut evidence)?;
    Ok(evidence)
}

fn collect_into(root: &Path, path: &Path, evidence: &mut NetworkEvidence) -> ToolResult<()> {
    if hidden(path) {
        return Ok(());
    }
    if path.is_dir() {
        if let Some(violation) = directory_violation(root, path) {
            evidence.contract_violations.push(violation);
        }
        evidence.directories = evidence.directories.saturating_add(1);
        for entry in fs::read_dir(path)? {
            collect_into(root, &entry?.path(), evidence)?;
        }
    } else if path.extension().is_some_and(|extension| extension == "md") {
        let text = fs::read_to_string(path)?;
        evidence.markdown_files = evidence.markdown_files.saturating_add(1);
        evidence.links = evidence.links.saturating_add(text.matches("](").count());
        if let Some(violation) = contract_violation(root, path, &text) {
            evidence.contract_violations.push(violation);
        }
    }
    Ok(())
}

fn contract_violation(root: &Path, path: &Path, text: &str) -> Option<String> {
    let name = relative_name(root, path);
    if !text.is_ascii() {
        return Some(format!("{name} has non-ascii prose"));
    }
    let h1_count = text.lines().filter(|line| line.starts_with("# ")).count();
    if h1_count != 1 {
        return Some(format!("{name} must have exactly one H1"));
    }
    if !text.contains("\n## Purpose\n") {
        return Some(format!("{name} is missing ## Purpose"));
    }
    None
}

fn relative_name(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .map(|relative| format!("docs/{}", relative.to_string_lossy().replace('\\', "/")))
        .unwrap_or_else(|_| path.to_string_lossy().replace('\\', "/"))
}

fn directory_violation(root: &Path, path: &Path) -> Option<String> {
    let parts = relative_parts(root, path);
    if parts.is_empty() {
        return None;
    }
    if parts.iter().any(|part| part == "docs") {
        return Some(format!(
            "{} contains nested docs directory",
            relative_name(root, path)
        ));
    }
    if parts.len() == 1 && !TOP_LEVEL_DIRS.contains(&parts[0].as_str()) {
        return Some(format!(
            "{} is an unmanaged top-level directory",
            relative_name(root, path)
        ));
    }
    None
}

fn relative_parts(root: &Path, path: &Path) -> Vec<String> {
    path.strip_prefix(root)
        .ok()
        .into_iter()
        .flat_map(|relative| relative.components())
        .filter_map(|component| component.as_os_str().to_str())
        .map(str::to_string)
        .collect()
}

fn missing_required(workspace: &Path) -> Vec<String> {
    REQUIRED_FILES
        .iter()
        .filter(|relative| !workspace.join(relative).exists())
        .map(|relative| (*relative).to_string())
        .collect()
}

fn missing_growth_markers(workspace: &Path) -> ToolResult<Vec<String>> {
    let mut missing = Vec::new();
    for relative in GROWTH_FILES {
        let text = fs::read_to_string(workspace.join(relative)).unwrap_or_default();
        if !text.contains("## Growth Control") {
            missing.push((*relative).to_string());
        }
    }
    Ok(missing)
}

fn hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with('.'))
}

fn sample_list(paths: &[String]) -> String {
    if paths.is_empty() {
        "none".to_string()
    } else {
        paths.iter().take(6).cloned().collect::<Vec<_>>().join(",")
    }
}

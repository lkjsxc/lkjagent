use std::fs;
use std::path::{Path, PathBuf};

use crate::error::ToolResult;
use crate::fs::workspace_path;

use super::content_audit::content_checks;
use super::graph::parse_graph;
use super::model::ScaffoldMode;
use super::names::{banned_release_wording, forbidden_serial_name, link_target};

pub fn audit_root(
    workspace: &Path,
    root: &str,
    count: Option<usize>,
    mode: ScaffoldMode,
) -> ToolResult<String> {
    let full = workspace_path(workspace, root)?;
    let mut failures = Vec::new();
    if !full.exists() {
        failures.push(format!("missing_root: {root}"));
        return Ok(report(root, failures));
    }
    collect_dir_checks(&full, &full, &mut failures)?;
    graph_checks(&full, &mut failures);
    content_checks(&full, &mut failures)?;
    count_check(&full, count, mode, &mut failures)?;
    Ok(report(root, failures))
}

fn collect_dir_checks(root: &Path, dir: &Path, failures: &mut Vec<String>) -> ToolResult<()> {
    let readme = dir.join("README.md");
    if !readme.is_file() {
        failures.push(format!("missing_readme: {}", rel(root, dir)));
    } else {
        readme_checks(root, dir, &readme, failures)?;
    }
    let children = immediate_children(dir)?;
    let child_count = children
        .iter()
        .filter(|child| child.file_name().and_then(|name| name.to_str()) != Some("README.md"))
        .count();
    if child_count < 2 {
        failures.push(format!("too_few_children: {}", rel(root, dir)));
    }
    for child in children {
        if child.is_dir() {
            collect_dir_checks(root, &child, failures)?;
        } else if child.extension().is_some_and(|ext| ext == "md") {
            file_checks(root, &child, failures)?;
        }
    }
    Ok(())
}

fn readme_checks(
    root: &Path,
    dir: &Path,
    readme: &Path,
    failures: &mut Vec<String>,
) -> ToolResult<()> {
    let text = fs::read_to_string(readme)?;
    if !text.contains("## Purpose") {
        failures.push(format!("missing_purpose: {}", rel(root, readme)));
    }
    for child in immediate_children(dir)? {
        let Some(name) = child.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if name == "README.md" {
            continue;
        }
        let target = link_target(name, child.is_dir());
        if !text.contains(&format!("({target})")) {
            failures.push(format!("missing_readme_link: {}", rel(root, &child)));
        }
    }
    file_checks(root, readme, failures)
}

fn file_checks(root: &Path, file: &Path, failures: &mut Vec<String>) -> ToolResult<()> {
    let relative = rel(root, file);
    let text = fs::read_to_string(file)?;
    if text.lines().filter(|line| line.starts_with("# ")).count() != 1 {
        failures.push(format!("h1_count: {relative}"));
    }
    if text.lines().count() > 200 {
        failures.push(format!("line_limit: {relative}"));
    }
    if forbidden_serial_name(&relative) {
        failures.push(format!("serial_filename: {relative}"));
    }
    if let Some(token) = banned_release_wording(&format!("{relative}\n{text}")) {
        failures.push(format!("banned_release_wording: {relative} {token}"));
    }
    Ok(())
}

fn graph_checks(root: &Path, failures: &mut Vec<String>) {
    let Some(graph) = parse_graph(root) else {
        failures.push("missing_doc_graph: .lkj-doc-graph.md".to_string());
        return;
    };
    for path in &graph.paths {
        if !root.join(path).exists() {
            failures.push(format!("graph_missing_path: {path}"));
        }
    }
    for (from, to) in &graph.edges {
        if !graph.nodes.contains(from) || !graph.nodes.contains(to) {
            failures.push(format!("graph_bad_edge: {from}->{to}"));
        }
    }
}

fn count_check(
    root: &Path,
    count: Option<usize>,
    mode: ScaffoldMode,
    failures: &mut Vec<String>,
) -> ToolResult<()> {
    let Some(target) = count else {
        return Ok(());
    };
    let actual = markdown_count(root)?;
    let ok = match mode {
        ScaffoldMode::Exact => actual == target,
        ScaffoldMode::Approx => actual.abs_diff(target) <= usize::max(1, target / 10),
    };
    if !ok {
        failures.push(format!("count_mismatch: expected={target} actual={actual}"));
    }
    Ok(())
}

fn markdown_count(root: &Path) -> ToolResult<usize> {
    let mut count: usize = 0;
    for child in immediate_children(root)? {
        if child.is_dir() {
            count = count.saturating_add(markdown_count(&child)?);
        } else if child.extension().is_some_and(|ext| ext == "md") {
            count = count.saturating_add(1);
        }
    }
    Ok(count)
}

fn immediate_children(dir: &Path) -> ToolResult<Vec<PathBuf>> {
    let mut children = fs::read_dir(dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect::<Vec<_>>();
    children.sort();
    Ok(children)
}

fn report(root: &str, failures: Vec<String>) -> String {
    let failed = failures.len();
    let passed = 15usize.saturating_sub(failed.min(15));
    if failures.is_empty() {
        return format!(
            "document audit passed\nroot={root}\nchecks=15\npassed=15\nfailed=0\nnext_action=record document-structure evidence"
        );
    }
    format!(
        "document audit failed\nroot={root}\nchecks=15\npassed={passed}\nfailed={failed}\nfailures:\n- {}\nnext_action=doc.scaffold or fs.batch_write exact failed topology",
        failures.join("\n- ")
    )
}

fn rel(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .ok()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or(path)
        .to_string_lossy()
        .to_string()
}

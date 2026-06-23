use std::fs;
use std::path::{Path, PathBuf};

use crate::error::ToolResult;
use crate::fs::workspace_path;

use super::audit_report::report;
use super::content_audit::content_checks;
use super::model::ScaffoldMode;
use super::names::{
    banned_release_wording, forbidden_serial_name, link_target, path_hygiene_failures,
};

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
        return Ok(report(root, failures, false));
    }
    if full.is_file() {
        return Ok(file_root_report(root));
    }
    collect_dir_checks(&full, &full, &mut failures)?;
    let content_requested = full.join("catalog.toml").is_file();
    content_checks(&full, &mut failures)?;
    count_check(&full, count, mode, &mut failures)?;
    Ok(report(root, failures, content_requested))
}

fn file_root_report(root: &str) -> String {
    format!(
        "document audit failed\nroot={root}\npath_kind=file\naddress_status=root_is_file\nfailed=1\nfailures:\n- root_is_file: {root}\nnext_action=fs.read file\nvalid_example:\n<act>\n<tool>fs.read</tool>\n<path>{root}</path>\n</act>"
    )
}

fn collect_dir_checks(root: &Path, dir: &Path, failures: &mut Vec<String>) -> ToolResult<()> {
    if dir_name_ends_md(dir) {
        failures.push(format!("markdown_suffix_directory: {}", rel(root, dir)));
    }
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
    failures.extend(path_hygiene_failures(&relative));
    if let Some(token) = banned_release_wording(&format!("{relative}\n{text}")) {
        failures.push(format!("banned_release_wording: {relative} {token}"));
    }
    Ok(())
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

fn dir_name_ends_md(dir: &Path) -> bool {
    dir.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with(".md"))
}

fn immediate_children(dir: &Path) -> ToolResult<Vec<PathBuf>> {
    let mut children = fs::read_dir(dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect::<Vec<_>>();
    children.sort();
    Ok(children)
}

fn rel(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .ok()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or(path)
        .to_string_lossy()
        .to_string()
}

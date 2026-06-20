use std::fs;
use std::path::Path;

use crate::error::{ToolError, ToolResult};
use crate::fs::{workspace_path, write};

pub fn scaffold(
    workspace: &Path,
    root: &str,
    kind: &str,
    count: &str,
    mode: &str,
    title: &str,
    sections: &str,
) -> ToolResult<String> {
    let target = parse_count(count).unwrap_or(3);
    if target == 0 || target > 100 {
        return Err(ToolError::invalid("doc.scaffold count must be 1..100"));
    }
    workspace_path(workspace, root)?;
    let section_list = lines(sections);
    let children = child_paths(target.saturating_sub(1));
    write(
        workspace,
        &format!("{root}/README.md"),
        &root_readme(title, kind, mode, &children),
    )?;
    for (index, child) in children.iter().enumerate() {
        let heading = section_list
            .get(index)
            .cloned()
            .unwrap_or_else(|| format!("Section {}", index.saturating_add(1)));
        write(
            workspace,
            &format!("{root}/{child}"),
            &child_body(&heading, title),
        )?;
    }
    Ok(format!(
        "document scaffold created\nroot={root}\nkind={kind}\nmode={mode}\nfiles={target}\nreadme=present"
    ))
}

pub fn audit(workspace: &Path, root: &str, count: &str, mode: &str) -> ToolResult<String> {
    let full = workspace_path(workspace, root)?;
    if !full.join("README.md").is_file() {
        return Ok(format!(
            "document audit failed\nroot={root}\nmissing=README.md"
        ));
    }
    let mut files = markdown_files(&full)?;
    files.sort();
    let requested = parse_count(count);
    let count_ok = requested.is_none_or(|target| count_matches(files.len(), target, mode));
    let status = if count_ok { "passed" } else { "failed" };
    Ok(format!(
        "document audit {status}\nroot={root}\nmarkdown_files={}\nreadme=present\ncount_ok={count_ok}",
        files.len()
    ))
}

fn root_readme(title: &str, kind: &str, mode: &str, children: &[String]) -> String {
    let toc = children
        .iter()
        .map(|child| format!("- [{child}]({child})"))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "# {title}\n\n## Purpose\n\n{kind} scaffold generated under {mode} mode.\n\n## Table of Contents\n\n{toc}\n"
    )
}

fn child_body(heading: &str, title: &str) -> String {
    format!("# {heading}\n\n## Purpose\n\nPart of {title}.\n")
}

fn child_paths(count: usize) -> Vec<String> {
    (1..=count)
        .map(|index| format!("part-{index:03}.md"))
        .collect()
}

fn markdown_files(root: &Path) -> ToolResult<Vec<String>> {
    let mut out = Vec::new();
    collect_markdown(root, root, &mut out)?;
    Ok(out)
}

fn collect_markdown(root: &Path, path: &Path, out: &mut Vec<String>) -> ToolResult<()> {
    for entry in fs::read_dir(path)?.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            collect_markdown(root, &path, out)?;
        } else if path.extension().is_some_and(|ext| ext == "md") {
            let rel = path
                .strip_prefix(root)
                .map_err(|error| ToolError::Io(error.to_string()))?;
            out.push(rel.to_string_lossy().to_string());
        }
    }
    Ok(())
}

fn parse_count(value: &str) -> Option<usize> {
    value.trim().parse::<usize>().ok()
}

fn count_matches(actual: usize, target: usize, mode: &str) -> bool {
    if mode == "exact" {
        actual == target
    } else {
        let lower = target.saturating_sub(usize::max(1, target / 10));
        let upper = target.saturating_add(usize::max(1, target / 10));
        actual >= lower && actual <= upper
    }
}

fn lines(value: &str) -> Vec<String> {
    value
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}

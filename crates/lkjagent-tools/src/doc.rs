mod audit;
mod body;
mod content_audit;
mod fit;
mod graph;
mod model;
mod names;
mod profile;
mod roles;
mod shape_profiles;
mod shapes;
mod write;

use std::path::Path;

use crate::error::{ToolError, ToolResult};
pub use model::ScaffoldProfile;
use model::{ScaffoldInput, ScaffoldMode};

pub fn plan(
    root: &str,
    kind: &str,
    count: &str,
    mode: &str,
    title: &str,
    sections: &str,
) -> ToolResult<String> {
    let input = scaffold_input(root, kind, count, mode, title, sections)?;
    let plan = profile::semantic_doc_plan(&input)?;
    Ok(format!(
        "document plan created\nroot={}\nkind={}\nprofile={:?}\nmode={}\nfiles={}\nmanifest=.lkj-doc-graph.md\nwrites=0",
        input.root,
        input.kind,
        plan.profile,
        input.mode.as_str(),
        plan.files.len()
    ))
}

pub fn scaffold(
    workspace: &Path,
    root: &str,
    kind: &str,
    count: &str,
    mode: &str,
    title: &str,
    sections: &str,
) -> ToolResult<String> {
    let input = scaffold_input(root, kind, count, mode, title, sections)?;
    let plan = profile::semantic_doc_plan(&input)?;
    let files = plan.markdown_count();
    write::write_plan(workspace, &plan)?;
    Ok(format!(
        "document scaffold created\nroot={}\nkind={}\nprofile={:?}\nmode={}\nfiles={files}\nreadme=present\ngraph=.lkj-doc-graph.md",
        input.root,
        input.kind,
        plan.profile,
        input.mode.as_str()
    ))
}

fn scaffold_input(
    root: &str,
    kind: &str,
    count: &str,
    mode: &str,
    title: &str,
    sections: &str,
) -> ToolResult<ScaffoldInput> {
    let input = ScaffoldInput {
        root: root.trim().to_string(),
        kind: value_or(kind, "documentation"),
        count: parse_count(count)?,
        mode: ScaffoldMode::parse(mode),
        title: title.trim().to_string(),
        sections: lines(sections),
    };
    if input.root.is_empty() {
        return Err(ToolError::invalid("doc.scaffold root must not be empty"));
    }
    if input.title.is_empty() {
        return Err(ToolError::invalid("doc.scaffold title must not be empty"));
    }
    Ok(input)
}

pub fn audit(workspace: &Path, root: &str, count: &str, mode: &str) -> ToolResult<String> {
    audit::audit_root(
        workspace,
        root.trim(),
        parse_count(count)?,
        ScaffoldMode::parse(mode),
    )
}

fn parse_count(value: &str) -> ToolResult<Option<usize>> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    let count = trimmed
        .parse::<usize>()
        .map_err(|_| ToolError::invalid("doc count must be a positive integer"))?;
    if !(3..=100).contains(&count) {
        return Err(ToolError::invalid("doc count must be 3..100"));
    }
    Ok(Some(count))
}

fn value_or(value: &str, default: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        default.to_string()
    } else {
        trimmed.to_string()
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

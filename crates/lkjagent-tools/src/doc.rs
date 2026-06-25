mod audit;
mod audit_report;
mod body;
mod content_audit;
mod content_signals;
mod fit;
mod model;
mod names;
mod profile;
mod profile_builders;
mod repeated_content;
mod roles;
mod semantic_seed;
mod semantic_seed_body;
mod semantic_seed_domain;
mod semantic_seed_extra;
mod semantic_seed_select;
mod semantic_workspace;
mod semantic_workspace_body;
mod semantic_workspace_readme;
mod semantic_workspace_terms;
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
        "document plan created\nroot={}\nkind={}\nprofile={:?}\nmode={}\nfiles={}\ncatalog=catalog.toml\nwrites=0",
        input.root,
        input.kind,
        plan.profile,
        input.mode.as_str(),
        plan.markdown_count()
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
    scaffold_inner((workspace, root, kind, count, mode, title, sections), true)
}

pub fn scaffold_allow_existing(
    workspace: &Path,
    root: &str,
    kind: &str,
    count: &str,
    mode: &str,
    title: &str,
    sections: &str,
) -> ToolResult<String> {
    scaffold_inner((workspace, root, kind, count, mode, title, sections), false)
}

type ScaffoldArgs<'a> = (
    &'a Path,
    &'a str,
    &'a str,
    &'a str,
    &'a str,
    &'a str,
    &'a str,
);

fn scaffold_inner(args: ScaffoldArgs<'_>, refuse_existing: bool) -> ToolResult<String> {
    let (workspace, root, kind, count, mode, title, sections) = args;
    crate::artifact_address_support::ensure_document_root(workspace, "doc.scaffold", root)?;
    if refuse_existing {
        refuse_existing_catalog(workspace, root)?;
    }
    let input = scaffold_input(root, kind, count, mode, title, sections)?;
    let plan = profile::semantic_doc_plan(&input)?;
    let files = plan.markdown_count();
    write::write_plan(workspace, &plan)?;
    Ok(format!(
        "document scaffold created\nroot={}\nkind={}\nprofile={:?}\nmode={}\nfiles={files}\nreadme=present\ncatalog=catalog.toml",
        input.root,
        input.kind,
        plan.profile,
        input.mode.as_str()
    ))
}

fn refuse_existing_catalog(workspace: &Path, root: &str) -> ToolResult<()> {
    let full = crate::fs::workspace_path(workspace, root)?;
    if full.join("catalog.toml").is_file() {
        return Err(ToolError::invalid(
            "doc.scaffold refuses existing cataloged roots; use artifact.next or fs.batch_write",
        ));
    }
    Ok(())
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
    if crate::address::root_looks_like_markdown_file(&input.root) {
        return Err(ToolError::invalid(
            crate::address::render_markdown_root_refusal("doc.scaffold", &input.root),
        ));
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

pub fn weak_content_paths(root: &Path) -> ToolResult<Vec<String>> {
    content_audit::weak_content_paths(root)
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

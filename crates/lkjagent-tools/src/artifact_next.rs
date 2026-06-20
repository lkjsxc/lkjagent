use std::fs;
use std::path::Path;

use crate::error::ToolResult;
use crate::fs::workspace_path;

pub fn next(workspace: &Path, root: &str, kind: &str) -> ToolResult<String> {
    crate::artifact::reject_empty_root(root)?;
    let full = workspace_path(workspace, root)?;
    let kind = resolved_kind(kind, &full);
    if !full.exists() {
        return Ok(format!(
            "artifact next batch\nroot={root}\nkind={kind}\nmissing=root\nnext_action=artifact.apply\nvalid_example:\n{}",
            artifact_apply_example(root, &kind)
        ));
    }
    let weak = crate::doc::weak_content_paths(&full)?;
    if weak.is_empty() {
        return Ok(format!(
            "artifact next batch\nroot={root}\nkind={kind}\nmissing=0\nnext_action=artifact.audit\nvalid_example:\n{}",
            artifact_audit_example(root, &kind)
        ));
    }
    let selected = weak.into_iter().take(3).collect::<Vec<_>>();
    Ok(format!(
        "artifact next batch\nroot={root}\nkind={kind}\nmissing={}\nnext_paths:\n{}\nrequired_sections:\n{}\nnext_action=fs.batch_write\nvalid_example:\n{}",
        selected.len(),
        selected
            .iter()
            .map(|path| format!("- {path}"))
            .collect::<Vec<_>>()
            .join("\n"),
        required_sections(&kind),
        batch_write_example(root, &kind, &selected)
    ))
}

fn resolved_kind(kind: &str, root: &Path) -> String {
    let trimmed = kind.trim();
    if !trimmed.is_empty() {
        return trimmed.to_string();
    }
    let text = optional_manifest(root);
    if text.contains("Cookbook") {
        "cookbook".to_string()
    } else if text.contains("NarrativeManuscript") {
        "story".to_string()
    } else {
        "artifact".to_string()
    }
}

fn required_sections(kind: &str) -> &'static str {
    match kind.to_ascii_lowercase().as_str() {
        "cookbook" => "- title\n- purpose\n- ingredients or concept\n- method or procedure\n- timing, signals, and fixes\n- verification notes",
        "story" => "- title\n- purpose\n- scene content or reference detail\n- continuity notes\n- verification notes",
        _ => "- title\n- purpose\n- concrete content\n- verification notes",
    }
}

#[allow(clippy::manual_unwrap_or_default)]
fn optional_manifest(root: &Path) -> String {
    match fs::read_to_string(root.join(".lkj-doc-graph.md")) {
        Ok(text) => text,
        Err(_) => String::new(),
    }
}

fn artifact_apply_example(root: &str, kind: &str) -> String {
    format!("<act>\n<tool>artifact.apply</tool>\n<root>{root}</root>\n<kind>{kind}</kind>\n</act>")
}

fn artifact_audit_example(root: &str, kind: &str) -> String {
    format!("<act>\n<tool>artifact.audit</tool>\n<root>{root}</root>\n<kind>{kind}</kind>\n</act>")
}

fn batch_write_example(root: &str, kind: &str, paths: &[String]) -> String {
    let files = paths
        .iter()
        .map(|path| {
            format!(
                "path: {root}/{path}\ncontent:\n# {}\n\n## Purpose\n\nReplace this skeleton with real {kind} content before dispatch.\n\n## Concrete Content\n\nAdd the requested substance, details, and verification notes here.",
                title_from_path(path)
            )
        })
        .collect::<Vec<_>>()
        .join("\n-- lkjagent-next-file --\n");
    format!("<act>\n<tool>fs.batch_write</tool>\n<files>\n{files}\n</files>\n</act>")
}

fn title_from_path(path: &str) -> String {
    let stem = match path
        .rsplit('/')
        .next()
        .and_then(|name| name.strip_suffix(".md"))
    {
        Some(stem) => stem,
        None => path,
    };
    stem.split('-')
        .filter(|part| !part.is_empty())
        .map(capitalize)
        .collect::<Vec<_>>()
        .join(" ")
}

fn capitalize(part: &str) -> String {
    let mut chars = part.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    format!("{}{}", first.to_ascii_uppercase(), chars.as_str())
}

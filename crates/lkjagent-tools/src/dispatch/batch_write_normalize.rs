use std::collections::BTreeSet;
use std::path::{Component, Path};

use lkjagent_protocol::{Action, Param};

use super::normalize::NormalizationNote;

const MAX_TOTAL_BYTES: usize = 6_000;
const MAX_FILE_BYTES: usize = crate::fs::MAX_INLINE_FILE_BYTES;

pub fn normalize_batch_write_paths(action: &mut Action, notes: &mut Vec<NormalizationNote>) {
    if action.tool != "fs.batch_write" || has_param(action, "files") || action.params.is_empty() {
        return;
    }
    let Some(files) = path_shaped_files(&action.params) else {
        return;
    };
    action.params = vec![Param::new("files", files)];
    notes.push(NormalizationNote {
        tool: action.tool.clone(),
        message: "action params normalized\nrenamed=path-shaped-params->files\nreason=all unknown parameters were safe relative file paths".to_string(),
    });
}

fn has_param(action: &Action, name: &str) -> bool {
    action.params.iter().any(|param| param.name == name)
}

fn path_shaped_files(params: &[Param]) -> Option<String> {
    let mut seen = BTreeSet::new();
    let mut total = 0usize;
    let mut blocks = Vec::new();
    for param in params {
        if !safe_relative_path(&param.name) || !seen.insert(param.name.as_str()) {
            return None;
        }
        let content = param.value.trim_end_matches(['\r', '\n']);
        if !safe_content(content) {
            return None;
        }
        let bytes = content.len();
        if bytes > MAX_FILE_BYTES {
            return None;
        }
        total = total.saturating_add(bytes);
        if total > MAX_TOTAL_BYTES {
            return None;
        }
        blocks.push(format!("path: {}\ncontent:\n{}", param.name, content));
    }
    Some(blocks.join("\n-- lkjagent-next-file --\n"))
}

fn safe_relative_path(path: &str) -> bool {
    if path.trim().is_empty() || path.contains('\\') {
        return false;
    }
    let path = Path::new(path);
    if path.is_absolute() {
        return false;
    }
    path.components()
        .all(|component| matches!(component, Component::Normal(_)))
}

fn safe_content(content: &str) -> bool {
    let trimmed = content.trim();
    !trimmed.is_empty() && !trimmed.contains("<action>") && !trimmed.contains("</action>")
}

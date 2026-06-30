use std::fs;
use std::path::Path;

use crate::address::{status, AddressNextAction, ArtifactAddress};
use crate::error::ToolResult;

pub fn missing_root_response(address: &ArtifactAddress) -> String {
    let root = root(address);
    let kind = kind_from_action(&address.next_action)
        .map(|kind| kind_or_default_for_root(&kind, &root))
        .unwrap_or_else(|| kind_or_default_for_root("", &root));
    let selected = vec!["catalog.toml".to_string()];
    format!(
        "artifact_next_result=root_missing\nroot={root}\nkind={kind}\nmissing=root\nruntime_event=ArtifactRootMissing\nnext_decision_required=true\ncandidate_action=fs.batch_write\ncandidate_contract:\n{}",
        crate::artifact_next_example::batch_write_contract(&root, &kind, &selected)
    )
}

pub fn focused_response(address: &ArtifactAddress, kind: &str) -> String {
    let root = root(address);
    let kind = kind_from_action(&address.next_action)
        .map(|kind| kind_or_default_for_root(&kind, &root))
        .unwrap_or_else(|| kind_or_default_for_root(kind, &root));
    let path = address.weak_path.clone().unwrap_or_default();
    let selected = vec![path.clone()];
    let valid_example = crate::artifact_next_example::batch_write_contract(&root, &kind, &selected);
    format!(
        "artifact address normalized\nrequested_root={}\naddress_status={}\nnormalized_root={root}\nweak_path={path}\n{}",
        address.requested,
        status(address),
        batch_response(&root, &kind, &selected, &valid_example)
    )
}

pub fn can_repair_file(address: &ArtifactAddress) -> bool {
    matches!(address.next_action, AddressNextAction::RepairPath { .. })
}

pub fn path_param(path: &str) -> Option<&str> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

pub fn root(address: &ArtifactAddress) -> String {
    address
        .root
        .clone()
        .unwrap_or_else(|| address.requested.clone())
}

pub fn resolved_kind(kind: &str, root: &Path) -> String {
    let trimmed = kind.trim();
    let text = optional_catalog(root);
    if story_catalog(&text) || story_kind(trimmed) {
        "story".to_string()
    } else if text.to_ascii_lowercase().contains("cookbook") {
        "cookbook".to_string()
    } else if !trimmed.is_empty() && !trimmed.eq_ignore_ascii_case("artifact") {
        trimmed.to_string()
    } else {
        kind_or_default_for_root(trimmed, &root.to_string_lossy())
    }
}

fn story_kind(kind: &str) -> bool {
    matches!(
        kind.to_ascii_lowercase().as_str(),
        "story" | "novel" | "manuscript"
    )
}

fn story_catalog(catalog: &str) -> bool {
    let lower = catalog.to_ascii_lowercase();
    catalog.contains("NarrativeManuscript")
        || lower.contains("kind = \"story\"")
        || lower.contains("story bible")
}

pub fn batch_response(root: &str, kind: &str, selected: &[String], valid_example: &str) -> String {
    format!(
        "artifact_next_result=weak_path_batch_ready\nroot={root}\nkind={kind}\nmissing={}\nnext_paths:\n{}\nrequired_sections:\n{}\nforbidden_weak_phrase_classes:\n- scaffold-only\n- placeholder\n- owner-terms-only\n- generic-example\nruntime_event=ArtifactWeakPathFound\nnext_decision_required=true\ncandidate_action=fs.batch_write\ncandidate_contract:\n{}",
        selected.len(),
        selected.iter().map(|path| format!("- {path}")).collect::<Vec<_>>().join("\n"),
        required_sections(kind, selected),
        valid_example
    )
}

pub fn audit_response(root: &str, kind: &str, missing: &str) -> ToolResult<String> {
    Ok(format!(
        "artifact_next_result=ready_for_audit\nroot={root}\nkind={kind}\n{missing}\nruntime_event=ArtifactWeakPathsExhausted\nnext_decision_required=true\ncandidate_action=artifact.audit\ncandidate_contract:\n{}",
        audit_contract(root, kind)
    ))
}

pub fn cursor_key(root: &str) -> String {
    format!("artifact.next cursor {root}")
}

fn kind_from_action(action: &AddressNextAction) -> Option<String> {
    match action {
        AddressNextAction::ApplyRoot { kind, .. }
        | AddressNextAction::AuditRoot { kind, .. }
        | AddressNextAction::RepairPath { kind, .. } => Some(kind.clone()),
        _ => None,
    }
}

fn kind_or_default_for_root(kind: &str, root: &str) -> String {
    let trimmed = kind.trim();
    if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("artifact") {
        if story_root(root) {
            "story".to_string()
        } else {
            "artifact".to_string()
        }
    } else {
        trimmed.to_string()
    }
}

fn story_root(root: &str) -> bool {
    root.trim_start_matches("./").starts_with("stories/")
}

fn required_sections(kind: &str, selected: &[String]) -> &'static str {
    if selected.iter().any(|path| path.starts_with("manuscript/")) {
        return "- title\n- finished chapter prose\n- scene action and dialogue or interiority\n- continuity with prior facts";
    }
    match kind.to_ascii_lowercase().as_str() {
        "cookbook" => "- title\n- purpose\n- ingredients or concept\n- method or procedure\n- timing, signals, and fixes\n- verification notes",
        "story" => "- title\n- purpose\n- scene content or reference detail\n- continuity notes\n- verification notes",
        _ => "- title\n- purpose\n- concrete content\n- verification notes",
    }
}

#[allow(clippy::manual_unwrap_or_default)]
fn optional_catalog(root: &Path) -> String {
    match fs::read_to_string(root.join("catalog.toml")) {
        Ok(text) => text,
        Err(_) => String::new(),
    }
}

fn audit_contract(root: &str, kind: &str) -> String {
    format!(
        "tool=artifact.audit\nroot={root}\nkind={kind}\nwrite_required=false\ninstruction=run audit after contracted writes; do not author content here"
    )
}

use super::model::{AddressNextAction, ArtifactAddress, PathKind, RootPathProblem};

pub fn render_artifact_refusal(tool: &str, address: &ArtifactAddress, kind: &str) -> String {
    render_refusal("artifact address refused", tool, address, kind)
}

pub fn render_document_refusal(tool: &str, address: &ArtifactAddress) -> String {
    render_refusal("document address refused", tool, address, "documentation")
}

pub fn render_markdown_root_refusal(tool: &str, root: &str) -> String {
    format!(
        "{}\ntool={tool}\nrequested_root={root}\naddress_status=root_ends_with_markdown_suffix\ndetected_path_kind=missing\nnext_action=fs.write or fs.batch_write owns Markdown leaves\nvalid_example:\n<action>\n<tool>fs.batch_write</tool>\n<files>\npath: {root}\ncontent:\n# Markdown Leaf\n\n## Purpose\n\nRecord the requested leaf content under the byte limit.\n</files>\n</action>",
        if tool.starts_with("doc.") { "document address refused" } else { "artifact address refused" }
    )
}

fn render_refusal(prefix: &str, tool: &str, address: &ArtifactAddress, kind: &str) -> String {
    let mut lines = vec![
        prefix.to_string(),
        format!("tool={tool}"),
        format!("requested_root={}", address.requested),
        format!("address_status={}", status(address)),
        format!("detected_path_kind={}", path_kind(&address.detected)),
    ];
    if let Some(root) = &address.root {
        lines.push(format!("normalized_root={root}"));
    }
    if let Some(path) = &address.weak_path {
        lines.push(format!("weak_path={path}"));
    }
    append_invalid_root_repair(&mut lines, address);
    lines.push(format!(
        "next_action={}",
        action_label(&address.next_action, tool)
    ));
    lines.push("valid_example:".to_string());
    lines.push(render_action(&address.next_action, tool, kind));
    lines.join("\n")
}

pub fn status(address: &ArtifactAddress) -> &'static str {
    match address.problem.as_ref() {
        Some(RootPathProblem::RootIsFile) => "root_is_file",
        Some(RootPathProblem::RootEndsWithMarkdownSuffix) => "root_ends_with_markdown_suffix",
        Some(RootPathProblem::RootMissing) => "root_missing",
        Some(RootPathProblem::RootNotDirectory) => "root_not_directory",
        Some(RootPathProblem::RootOutsideWorkspace) => "root_outside_workspace",
        None => "ok",
    }
}

fn append_invalid_root_repair(lines: &mut Vec<String>, address: &ArtifactAddress) {
    if address.problem != Some(RootPathProblem::RootEndsWithMarkdownSuffix) {
        return;
    }
    if address.detected != PathKind::Directory {
        return;
    }
    lines.push("invalid_root_marker=required".to_string());
    lines.push("repair_outcome=choose_catalog_root_or_repair_markdown_directory".to_string());
}

fn path_kind(kind: &PathKind) -> &'static str {
    match kind {
        PathKind::Missing => "missing",
        PathKind::File => "file",
        PathKind::Directory => "directory",
        PathKind::Other => "other",
    }
}

fn action_label(action: &AddressNextAction, tool: &str) -> &'static str {
    match action {
        AddressNextAction::ApplyRoot { .. } => "artifact.apply root directory",
        AddressNextAction::AuditRoot { .. } => "artifact.audit owning root",
        AddressNextAction::RepairPath { .. } => "artifact.next focused weak path",
        AddressNextAction::InspectParent { .. } if tool.starts_with("doc.") => "fs.read or fs.list",
        AddressNextAction::InspectParent { .. } => "fs.list parent directory",
        AddressNextAction::Refuse { .. } => "workspace.summary",
    }
}

fn render_action(action: &AddressNextAction, tool: &str, kind: &str) -> String {
    match action {
        AddressNextAction::ApplyRoot { root, kind } => format!(
            "<action>\n<tool>artifact.apply</tool>\n<root>{root}</root>\n<kind>{kind}</kind>\n</action>"
        ),
        AddressNextAction::AuditRoot { root, kind } => format!(
            "<action>\n<tool>artifact.audit</tool>\n<root>{root}</root>\n<kind>{kind}</kind>\n</action>"
        ),
        AddressNextAction::RepairPath { root, path, kind } => format!(
            "<action>\n<tool>artifact.next</tool>\n<root>{root}</root>\n<path>{path}</path>\n<kind>{kind}</kind>\n</action>"
        ),
        AddressNextAction::InspectParent { path } if tool.starts_with("doc.") => format!(
            "<action>\n<tool>fs.list</tool>\n<path>{path}</path>\n<depth>1</depth>\n</action>"
        ),
        AddressNextAction::InspectParent { path } => format!(
            "<action>\n<tool>fs.list</tool>\n<path>{path}</path>\n<depth>1</depth>\n</action>"
        ),
        AddressNextAction::Refuse { reason } => format!(
            "<action>\n<tool>workspace.summary</tool>\n<path>.</path>\n</action>\nreason={reason}\nkind={kind}"
        ),
    }
}

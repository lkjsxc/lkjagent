use crate::kernel::event::RuntimeEvent;
use crate::kernel::obligation_facts::ArtifactRootStatus;
use crate::kernel::snapshot::RuntimeSnapshot;

pub(crate) fn audit_or_contract_text(text: &str) -> bool {
    text.contains("document audit")
        || text.contains("artifact_next_result=")
        || text.contains("address_status=root_is_file")
        || text.contains("missing_root")
        || text.contains("root_missing")
}

pub(crate) fn line_value(text: &str, key: &str) -> Option<String> {
    let prefix = format!("{key}=");
    text.lines()
        .find_map(|line| line.trim().strip_prefix(&prefix).map(str::to_string))
        .filter(|value| !value.trim().is_empty())
}

pub(crate) fn failure_lines(text: &str) -> Vec<String> {
    let mut in_failures = false;
    let mut failures = Vec::new();
    for line in text.lines().map(str::trim) {
        if line == "failures:" {
            in_failures = true;
            continue;
        }
        if in_failures && !line.starts_with("- ") {
            break;
        }
        if in_failures {
            failures.push(line.trim_start_matches("- ").to_string());
        }
    }
    if failures.is_empty() && text.contains("missing_root") {
        failures.push("missing_root".to_string());
    }
    failures
}

pub(crate) fn status_from_text(
    text: &str,
    topology: &str,
    content: &str,
    failures: &[String],
) -> ArtifactRootStatus {
    if text.contains("root_missing") || failures.iter().any(|item| item.contains("missing_root")) {
        return ArtifactRootStatus::Missing;
    }
    if text.contains("root_is_file") {
        return ArtifactRootStatus::RootIsFile;
    }
    if text.contains("root_needs_identity") {
        return ArtifactRootStatus::IdentityIncomplete;
    }
    if text.contains("document audit passed") && content == "passed" {
        return ArtifactRootStatus::Ready;
    }
    if text.contains("document audit passed") {
        return ArtifactRootStatus::StructurePassed;
    }
    if content == "failed" {
        return ArtifactRootStatus::ContentWeak;
    }
    if topology == "failed" || text.contains("document audit failed") {
        return ArtifactRootStatus::StructureFailed;
    }
    ArtifactRootStatus::Unknown
}

pub(crate) fn candidate_event(text: &str, status: ArtifactRootStatus) -> Option<RuntimeEvent> {
    if root_identity_status(status) || text.contains("runtime_event=ArtifactRootMissing") {
        return Some(RuntimeEvent::ArtifactRootMissing);
    }
    if text.contains("runtime_event=ArtifactWeakPathFound") {
        return Some(RuntimeEvent::ArtifactWeakPathFound);
    }
    if text.contains("document audit passed") {
        return Some(RuntimeEvent::ArtifactAudited);
    }
    None
}

pub(crate) fn status_from_snapshot(snapshot: &RuntimeSnapshot) -> ArtifactRootStatus {
    if !snapshot.artifact.weak_paths.is_empty() {
        return ArtifactRootStatus::ContentWeak;
    }
    match snapshot.artifact.audit_status.as_deref() {
        Some("ready" | "passed") => ArtifactRootStatus::Ready,
        Some("failed") => ArtifactRootStatus::StructureFailed,
        Some("missing") => ArtifactRootStatus::Missing,
        _ => ArtifactRootStatus::Unknown,
    }
}

pub(crate) fn recovery_event(event: &RuntimeEvent) -> bool {
    matches!(
        event,
        RuntimeEvent::ParseFault { .. }
            | RuntimeEvent::EndpointFault { .. }
            | RuntimeEvent::ProviderAnomaly { .. }
            | RuntimeEvent::AdmissionRefused { .. }
            | RuntimeEvent::StaleActionRefused { .. }
            | RuntimeEvent::RepeatedActionRefused { .. }
            | RuntimeEvent::RepeatActionDetected { .. }
            | RuntimeEvent::PayloadOverflowDetected { .. }
            | RuntimeEvent::ToolFailed { .. }
            | RuntimeEvent::TurnBudgetExhausted
    )
}

pub(crate) fn root_identity_status(status: ArtifactRootStatus) -> bool {
    matches!(
        status,
        ArtifactRootStatus::Missing
            | ArtifactRootStatus::EmptyDirectory
            | ArtifactRootStatus::IdentityIncomplete
    )
}

pub(crate) fn inferred_kind(root: &str) -> Option<String> {
    if root.trim_start_matches("./").starts_with("stories/") {
        Some("story".to_string())
    } else if root.contains("cookbook") || root.contains("cookbooks/") {
        Some("cookbook".to_string())
    } else {
        None
    }
}
